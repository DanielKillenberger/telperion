//! fn-270's premises and contract on the CPU cycle (spec R1, R2, R2a,
//! R2a', R2b, R3), on the engine's own species at small ages; every preset
//! at its default age is `telperion-core`'s `premises` example.
use super::*;
use super::super::grow_cycle;
use crate::error::Error;
use crate::light::Light;
use crate::structure::Origin;
use std::collections::HashMap;

fn request(age: u32, seed: u64, extinction: f64) -> Request {
    let light = Light { extinction, sky: 0.5 };
    Request { age, seed, budget: 20_000_000, light }
}

/// The oak (lit, its retired records kept), the spruce and the beech.
fn species() -> [(Species, f64); 3] {
    [(crate::oak(), 0.5), (crate::spruce(), 0.0), (crate::beech(), 0.0)]
}

/// The years around each waking, shedding and change of age a run to
/// `r.age` holds: the year before the first one past the third, and its own.
fn eventful(species: &Species, r: Request) -> Vec<u32> {
    let mut p = Paused::begin(species, r, Retiring::AtAge, false).unwrap();
    p.grow_to(r.age).unwrap();
    let g = &p.g;
    let woke = g.woken.iter().map(|w| w.cycle).collect::<Vec<_>>();
    let shed = g.axes.iter().filter_map(|a| a.apex_end).collect::<Vec<_>>();
    let w = g.working.as_ref().unwrap();
    let retired = w.retire.log.iter().filter(|l| l.2 == 2).map(|l| l.1).collect::<Vec<_>>();
    let changed = (g.axes.iter())
        .filter(|a| matches!(a.origin, Origin::Continuation { .. }))
        .map(|a| a.birth)
        .collect::<Vec<_>>();
    let mut years = Vec::new();
    for events in [woke, shed, retired, changed] {
        let first = events.into_iter().filter(|&c| c > 3 && c < r.age).min();
        years.extend(first.into_iter().flat_map(|c| [c - 1, c]));
    }
    years.sort_unstable();
    years.dedup();
    years
}

/// R1 (d): grown on from a pause, with settles, repeated settles and
/// zero-year advances between, the tree is the one grown in one call, to
/// the bit, and no settle or empty advance moves the grower.
#[test]
fn a_tree_paused_and_grown_on_is_the_tree_grown_at_once() {
    let h = 24;
    for (species, k) in species() {
        let r = request(h, 7, k);
        let today = tree_digest(&grow_cycle(&species, r).unwrap().structure);
        let mut splits = vec![10, h - 1];
        splits.extend(eventful(&species, r));
        assert!(splits.len() >= 6, "{splits:?}");
        for a in splits {
            let mut p = Paused::begin(&species, r, Retiring::AtAge, false).unwrap();
            p.grow_to(a).unwrap();
            let before = p.digest();
            let first = tree_digest(&p.settle().unwrap());
            p.grow_to(a).unwrap();
            assert_eq!(tree_digest(&p.settle().unwrap()), first, "a settle at {a} repeats");
            assert_eq!(p.digest(), before, "a settle or none grown at {a} moves the grower");
            p.grow_to(h).unwrap();
            assert_eq!(tree_digest(&p.settle().unwrap()), today, "split at {a}");
        }
    }
}

/// R1 (b) and R3: under the horizon, the tree at a year of a run that
/// goes on is the tree of a run stopped there, and retired at the age
/// grown, it is the tree with nothing retired, at the years around the
/// end. The walk over every year is `every_year_is_the_horizons_prefix`.
#[test]
fn the_tree_at_a_year_is_the_horizons_prefix() {
    let h = 24;
    for (species, k) in species() {
        let r = request(h, 7, k);
        let at = |retiring: Retiring, y: u32| {
            let mut p = Paused::begin(&species, r, retiring, false).unwrap();
            p.grow_to(y).unwrap();
            tree_digest(&p.settle().unwrap())
        };
        for y in [12, h - 1] {
            assert_eq!(at(Retiring::AtAge, y), at(Retiring::Never, y), "year {y}");
        }
        let today = tree_digest(&grow_cycle(&species, r).unwrap().structure);
        assert_eq!(today, at(Retiring::Never, h), "the tree at the horizon");
    }
}

/// R1 (b), every year: one run settled at each year is the run that
/// retires nothing, year by year (60 s on one thread; out of the gate).
///
///   cargo test --profile ci -p telperion-space every_year_is_the_horizons_prefix -- --ignored
#[test]
#[ignore = "slow: every year's settle of three species, run by name"]
fn every_year_is_the_horizons_prefix() {
    let h = 24;
    for (species, k) in species() {
        let r = request(h, 7, k);
        let walk = |retiring: Retiring| -> Vec<u64> {
            let mut p = Paused::begin(&species, r, retiring, false).unwrap();
            (1..=h)
                .map(|y| {
                    p.grow_to(y).unwrap();
                    tree_digest(&p.settle().unwrap())
                })
                .collect()
        };
        assert_eq!(walk(Retiring::AtAge), walk(Retiring::Never));
    }
}

/// R2b: a non-finite, negative, backward or past-horizon age, and a draw
/// outside the last year grown, are refused by name, and the grower is
/// left as it was.
#[test]
fn a_request_the_grower_cannot_meet_is_refused() {
    let species = crate::beech();
    let mut p = Paused::begin(&species, request(12, 3, 0.0), Retiring::AtAge, false).unwrap();
    p.grow_to(8).unwrap();
    let before = p.digest();
    let refused = |r: Result<()>| match r {
        Err(Error::Refused { input, .. }) => input,
        other => panic!("{other:?}"),
    };
    assert_eq!(refused(p.grow_to(13)), "age.horizon");
    assert_eq!(refused(p.grow_to(7)), "age.backward");
    let ages = [(f64::NAN, "age"), (f64::INFINITY, "age"), (-0.5, "age"), (12.01, "age.horizon"), (7.0, "age.backward")];
    for (age, name) in ages {
        assert_eq!(refused(admitted(8, 12, age).map(drop)), name, "{age}");
    }
    for at in [7.0, 8.01, f64::NAN] {
        assert_eq!(refused(p.settle_at(at).map(drop)), "at", "{at}");
    }
    let unborn = Paused::begin(&species, request(12, 3, 0.0), Retiring::AtAge, false).unwrap();
    assert_eq!(refused(unborn.settle_at(0.0).map(drop)), "at", "nothing grown yet");
    assert_eq!(p.digest(), before);
    assert_eq!(admitted(8, 12, 7.25).unwrap(), 8);
    assert_eq!(admitted(8, 12, 8.0).unwrap(), 8);
    assert_eq!(admitted(8, 12, 8.25).unwrap(), 9);
}

/// R2b: a cycle that fails (here the phytomer budget) spends the grower;
/// every later growth and draw is refused by name, and a tree drawn before
/// stays whole.
#[test]
fn a_growth_that_failed_is_spent() {
    let species = crate::beech();
    let r = Request { budget: 2_000, ..request(24, 3, 0.0) };
    let mut p = Paused::begin(&species, r, Retiring::AtAge, false).unwrap();
    let mut last = None;
    let failed = (1..=r.age).find_map(|y| match p.grow_to(y) {
        Ok(()) => {
            last = Some(p.settle().unwrap());
            None
        }
        Err(e) => Some(e),
    });
    assert_eq!(failed, Some(Error::Budget { limit: 2_000 }));
    let drawn = last.expect("a tree drawn before the failure");
    assert!(!drawn.axes.is_empty());
    let spent = |r: Result<()>| matches!(r, Err(Error::Refused { input, .. }) if input == "state.failed");
    assert!(spent(p.grow_to(r.age)));
    assert!(spent(p.settle().map(drop)));
    assert!(spent(p.settle_at(f64::from(p.committed())).map(drop)));
    assert!(spent(p.settle_at(f64::NAN).map(drop)), "spent before the age is judged");
}

/// R2a and R2a': 40, 40.25, 40.5, 41 commits cycle 41 once, at 40.25, and
/// the tree at 41 is the one grown to 41 in one call. Each fractional
/// settle leaves the grower as it was, the whole year draws today's tree,
/// and just past 40 this year's phytomers start from nothing (the
/// stand-in presentation, `presented`; the curves are task 4's).
#[test]
fn a_tree_drawn_within_its_year_unfolds_from_the_last() {
    let species = crate::oak();
    let r = request(41, 7, 0.5);
    let mut p = Paused::begin(&species, r, Retiring::AtAge, false).unwrap();
    let grow = |p: &mut Paused, a: f64| {
        let to = admitted(p.committed(), p.horizon(), a)?;
        p.grow_to(to)
    };
    grow(&mut p, 40.0).unwrap();
    assert_eq!(p.committed(), 40);
    let mut committed = Vec::new();
    let whole = p.settle_at(41.0).err();
    assert!(whole.is_some(), "41 is not within the year grown to 40");
    for at in [40.25, 40.5, 41.0] {
        grow(&mut p, at).unwrap();
        committed.push(p.committed());
        let before = p.digest();
        let drawn = p.settle_at(at).unwrap();
        assert_eq!(p.digest(), before, "a settle at {at} moves the grower");
        let new = drawn.axes.iter().flat_map(|a| &a.phytomers).filter(|p| p.cycle == 41);
        assert!(new.clone().count() > 0);
        let whole = p.settle().unwrap();
        let whole = whole.axes.iter().flat_map(|a| &a.phytomers).filter(|p| p.cycle == 41);
        for (p, w) in new.zip(whole) {
            assert_eq!(p.scale, w.scale * (at - 40.0));
        }
    }
    assert_eq!(committed, [41, 41, 41]);
    match grow(&mut p, 40.0) {
        Err(Error::Refused { input, .. }) => assert_eq!(input, "age.backward"),
        other => panic!("{other:?}"),
    }
    let today = grow_cycle(&species, r).unwrap().structure;
    assert_eq!(tree_digest(&p.settle_at(41.0).unwrap()), tree_digest(&today));
    let largest = (today.axes.iter().flat_map(|a| &a.phytomers)).map(|p| p.scale).fold(0.0, f64::max);
    let just = p.settle_at(40.0 + 1e-9).unwrap();
    let new = just.axes.iter().flat_map(|a| &a.phytomers).filter(|p| p.cycle == 41);
    assert!(new.clone().all(|p| p.scale <= 1e-8 * largest), "this year's start from nothing");
    // The join with year 40's tree: its organs drawn just past 40 are
    // year 41's, so they join it only where the year sheds or fades
    // nothing. R5a's blend is task 4's (PREMISES.md, R2a).
    let mut forty = Paused::begin(&species, r, Retiring::AtAge, false).unwrap();
    forty.grow_to(40).unwrap();
    let forty = forty.settle().unwrap();
    let whole = p.settle_at(41.0).unwrap();
    let (joined, step) = (organs(&just), organs(&whole));
    let old = |m: &HashMap<(u64, usize), f64>| m.iter().filter(|(_, s)| **s > 1e-8 * largest).count();
    for (key, _) in organs(&forty) {
        assert_eq!(joined.get(&key), step.get(&key), "just past 40 an organ of 40 is drawn as at 41");
    }
    assert!(old(&joined) > 0);
}

/// Every phytomer's scale, by its axis's identity and its place on it.
fn organs(tree: &Structure) -> HashMap<(u64, usize), f64> {
    let ids = crate::fine::identities(tree);
    let mut m = HashMap::new();
    for (a, id) in tree.axes.iter().zip(ids) {
        for (k, p) in a.phytomers.iter().enumerate() {
            m.insert((id, k), p.scale);
        }
    }
    m
}
