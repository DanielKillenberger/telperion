//! fn-270's premises and contract on the CPU cycle (spec R1, R2, R2a,
//! R2a', R2b, R3), on the engine's own species at small ages; every preset
//! at its default age is `telperion-core`'s `premises` example.
use super::super::grow_cycle;
use super::*;
use crate::error::Error;
use crate::light::Light;
use crate::structure::Origin;

fn request(age: u32, seed: u64, extinction: f64) -> Request {
    let light = Light {
        extinction,
        sky: 0.5,
    };
    Request { age, seed, light }
}

/// The oak (lit, its retired records kept), the spruce and the beech.
fn species() -> [(Species, f64); 3] {
    [
        (crate::oak(), 0.5),
        (crate::spruce(), 0.0),
        (crate::beech(), 0.0),
    ]
}

/// The years around each waking, shedding and change of age a run to
/// `r.age` holds: the year before the first one past the third, and its own.
fn eventful(species: &Species, r: Request) -> Vec<u32> {
    // Its verdicts are logged where its records are kept (fn-333).
    let mut p = Paused::with(
        std::sync::Arc::new(species.clone()),
        r,
        false,
        Working::indexed(),
    )
    .unwrap();
    p.grow_to(r.age).unwrap();
    let g = &p.g;
    let woke = g.woken.iter().map(|w| w.cycle).collect::<Vec<_>>();
    let shed = g.axes.iter().filter_map(|a| a.apex_end).collect::<Vec<_>>();
    let w = g.working.as_ref().unwrap();
    let retired = w
        .retire
        .log
        .iter()
        .filter(|l| l.2 == 2)
        .map(|l| l.1)
        .collect::<Vec<_>>();
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
/// the bit, and no settle or empty advance moves the grower. Both digests
/// hold every phytomer's section and sapwood, so a year's wood is laid the
/// same paused and resumed as straight through (fn-354).
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
            let mut p = Paused::begin(
                std::sync::Arc::new(species.clone()),
                r,
                Retiring::AtAge,
                false,
            )
            .unwrap();
            p.grow_to(a).unwrap();
            let before = p.digest();
            let first = tree_digest(&p.settle().unwrap());
            p.grow_to(a).unwrap();
            assert_eq!(
                tree_digest(&p.settle().unwrap()),
                first,
                "a settle at {a} repeats"
            );
            assert_eq!(
                p.digest(),
                before,
                "a settle or none grown at {a} moves the grower"
            );
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
            let mut p =
                Paused::begin(std::sync::Arc::new(species.clone()), r, retiring, false).unwrap();
            p.grow_to(y).unwrap();
            p.settle().unwrap()
        };
        for y in [12, h - 1] {
            // Retired at the age grown, the girth of what closed is frozen
            // (fn-270 R4); the tree is the one retiring nothing but for it.
            let apart =
                crate::grow::cycle::girth_apart(&at(Retiring::AtAge, y), &at(Retiring::Never, y));
            assert!(
                apart.is_some_and(|(_, most)| most <= 1e-4),
                "year {y}: girth apart {apart:?}"
            );
        }
        let today = tree_digest(&grow_cycle(&species, r).unwrap().structure);
        assert_eq!(
            today,
            tree_digest(&at(Retiring::AtAge, h)),
            "the tree at the horizon"
        );
    }
}

/// fn-355 R1: the grower reads no horizon. The tree at year y of a growth
/// begun for a later age is the tree of a growth to y, to the bit, for the
/// oak, the spruce and the beech at two pairs of ages each.
#[test]
fn a_year_is_a_state_not_a_horizon() {
    for (species, k) in species() {
        for (y, later) in [(14, 15), (20, 28)] {
            let asked = tree_digest(&grow_cycle(&species, request(y, 7, k)).unwrap().structure);
            let mut p = Paused::begin(
                std::sync::Arc::new(species.clone()),
                request(later, 7, k),
                Retiring::AtAge,
                false,
            )
            .unwrap();
            p.grow_to(y).unwrap();
            let on_the_way = tree_digest(&p.settle().unwrap());
            assert_eq!(on_the_way, asked, "year {y} of a growth to {later}");
        }
    }
}

/// fn-355 R2: a growth goes on past its first request, and the tree is
/// the one asked for that far, to the bit.
#[test]
fn a_growth_goes_on_past_its_first_request() {
    for (species, k) in species() {
        let (first, later) = (12, 21);
        let asked = grow_cycle(&species, request(later, 7, k)).unwrap();
        let mut p = Paused::begin(
            std::sync::Arc::new(species.clone()),
            request(first, 7, k),
            Retiring::AtAge,
            false,
        )
        .unwrap();
        p.grow_to(first).unwrap();
        p.settle().unwrap();
        p.grow_to(later).unwrap();
        let on = tree_digest(&p.settle().unwrap());
        assert_eq!(on, tree_digest(&asked.structure), "grown on to {later}");
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
            let mut p =
                Paused::begin(std::sync::Arc::new(species.clone()), r, retiring, false).unwrap();
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

/// R2b: a non-finite, negative or backward age, and a draw outside the
/// last year grown, are refused by name, and the grower is left as it
/// was. An age past the first one asked is admitted (fn-355 R2).
#[test]
fn a_request_the_grower_cannot_meet_is_refused() {
    let species = crate::beech();
    let mut p = Paused::begin(
        std::sync::Arc::new(species.clone()),
        request(12, 3, 0.0),
        Retiring::AtAge,
        false,
    )
    .unwrap();
    p.grow_to(8).unwrap();
    let before = p.digest();
    let refused = |r: Result<()>| match r {
        Err(Error::Refused { input, .. }) => input,
        other => panic!("{other:?}"),
    };
    assert_eq!(refused(p.grow_to(7)), "age.backward");
    let ages = [
        (f64::NAN, "age"),
        (f64::INFINITY, "age"),
        (-0.5, "age"),
        (7.0, "age.backward"),
    ];
    for (age, name) in ages {
        assert_eq!(refused(admitted(8, age).map(drop)), name, "{age}");
    }
    for at in [7.0, 8.01, f64::NAN] {
        assert_eq!(refused(p.settle_at(at).map(drop)), "at", "{at}");
    }
    let unborn = Paused::begin(
        std::sync::Arc::new(species.clone()),
        request(12, 3, 0.0),
        Retiring::AtAge,
        false,
    )
    .unwrap();
    assert_eq!(
        refused(unborn.settle_at(0.0).map(drop)),
        "at",
        "nothing grown yet"
    );
    assert_eq!(p.digest(), before);
    assert_eq!(admitted(8, 7.25).unwrap(), 8);
    assert_eq!(admitted(8, 8.0).unwrap(), 8);
    assert_eq!(admitted(8, 8.25).unwrap(), 9);
    assert_eq!(admitted(8, 12.01).unwrap(), 13);
}

/// R2b: a cycle that fails (here a lattice past its nodes, as
/// `a_lattice_past_its_nodes_is_refused_by_name` holds it) spends the
/// grower; every later growth and draw is refused by name, and a tree
/// drawn before stays whole.
#[test]
fn a_growth_that_failed_is_spent() {
    let species = crate::oak();
    let r = request(8, 1, 0.5);
    let small = crate::light::field::SMALL;
    let begun = Paused::begin(
        std::sync::Arc::new(species.clone()),
        r,
        Retiring::AtAge,
        false,
    )
    .unwrap();
    let mut p = begun.lattice_within(small);
    let mut last = None;
    let failed = (1..=r.age).find_map(|y| match p.grow_to(y) {
        Ok(()) => {
            last = Some(p.settle().unwrap());
            None
        }
        Err(e) => Some(e),
    });
    assert!(
        matches!(&failed, Some(Error::Refused { input, .. }) if input == "light.lattice"),
        "{failed:?}"
    );
    let drawn = last.expect("a tree drawn before the failure");
    assert!(!drawn.axes.is_empty());
    let spent =
        |r: Result<()>| matches!(r, Err(Error::Refused { input, .. }) if input == "state.failed");
    assert!(spent(p.grow_to(r.age)));
    assert!(spent(p.settle().map(drop)));
    assert!(spent(p.settle_at(f64::from(p.committed())).map(drop)));
    assert!(
        spent(p.settle_at(f64::NAN).map(drop)),
        "spent before the age is judged"
    );
}

/// R2a and R2a': 40, 40.25, 40.5, 41 commits cycle 41 once, at 40.25, and
/// the tree at 41 is the one grown to 41 in one call. Each fractional
/// settle leaves the grower as it was and the whole year draws today's
/// tree; a grower that keeps no year before refuses a draw within the
/// year. How the year unfolds is `unfold`'s.
#[test]
fn a_tree_drawn_within_its_year_unfolds_from_the_last() {
    let species = crate::oak();
    let r = request(41, 7, 0.5);
    let mut p = Paused::begin(
        std::sync::Arc::new(species.clone()),
        r,
        Retiring::AtAge,
        false,
    )
    .unwrap()
    .presenting();
    let grow = |p: &mut Paused, a: f64| {
        let to = admitted(p.committed(), a)?;
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
        assert_eq!(drawn.fraction, at - 40.0);
        assert_eq!(p.previous().map(|t| t.age), Some(40));
    }
    assert_eq!(committed, [41, 41, 41]);
    match grow(&mut p, 40.0) {
        Err(Error::Refused { input, .. }) => assert_eq!(input, "age.backward"),
        other => panic!("{other:?}"),
    }
    let today = grow_cycle(&species, r).unwrap().structure;
    assert_eq!(
        tree_digest(&p.settle_at(41.0).unwrap().structure),
        tree_digest(&today)
    );
    let mut plain = Paused::begin(
        std::sync::Arc::new(species.clone()),
        request(3, 7, 0.5),
        Retiring::AtAge,
        false,
    )
    .unwrap();
    plain.grow_to(3).unwrap();
    match plain.settle_at(2.5) {
        Err(Error::Refused { input, .. }) => assert_eq!(input, "at"),
        other => panic!("{other:?}"),
    }
}

/// Each laid living tip's path from the root collar, by its axis's
/// lineage.
fn paths(p: &Paused) -> Vec<(u64, u32)> {
    let pencils = &p.g.sketch.as_ref().expect("sketching").pencils;
    let path = |i: usize| (p.g.axes[i].lineage, pencils[i].path_end().to_bits());
    let laid = p.g.live.iter().filter(|a| a.axis < pencils.len());
    let mut out: Vec<(u64, u32)> = laid.map(|a| path(a.axis)).collect();
    out.sort_unstable();
    out
}

/// fn-349: the path a living tip is charged by survives the records let
/// go (`compact.rs`) and a grower paused and grown on: grown in three
/// steps, letting go of every retired record, each tip's path is the one
/// a grower keeping every record and grown at once holds, to the bit.
#[test]
fn a_tips_path_survives_letting_go_and_a_pause() {
    for (species, k) in species() {
        let species = std::sync::Arc::new(species);
        let r = request(40, 7, k);
        let mut eager = Working::default();
        eager.eager = true;
        let mut stepped = Paused::with(species.clone(), r, false, eager).unwrap();
        for age in [12, 25, 40] {
            stepped.grow_to(age).unwrap();
        }
        let mut once = Paused::with(species, r, false, Working::indexed()).unwrap();
        once.grow_to(40).unwrap();
        let (a, b) = (paths(&stepped), paths(&once));
        assert!(!a.is_empty() && a.iter().any(|&(_, l)| f32::from_bits(l) > 1.0));
        assert_eq!(a, b);
    }
}

/// Each closed subtree's living, to the bit: those kept by their root's
/// lineage and birth; and those retired whose bearer is held, by birth,
/// with the vigour the settle reads of the root (`Grower::mark`,
/// `presence::assign`), sorted.
type Livings = (Vec<(u64, u32, [u32; 2])>, Vec<(u32, u32, [u32; 2])>);

fn livings(p: &Paused) -> Livings {
    use super::super::retire::Fate;
    let (g, bits) = (&p.g, |l: crate::shed::Living| {
        [l.lived, l.alive].map(f32::to_bits)
    });
    let mut settled = g.settled_copy(p.committed);
    settled.mark();
    crate::presence::assign(&mut settled.axes, &settled.draws, &g.species, 1);
    let r = &g.working.as_ref().expect("a working set").retire;
    let (mut kept, mut roots) = (Vec::new(), Vec::new());
    for (i, f) in r.fate.iter().enumerate() {
        let a = &settled.axes[i];
        match *f {
            Fate::Kept(l) => kept.push((a.lineage, a.birth, bits(l))),
            Fate::Root(l) => roots.push((a.birth, (a.vigour as f32).to_bits(), bits(l))),
            _ => {}
        }
    }
    let stands = r.standing().into_iter();
    roots.extend(stands.map(|s| (s.birth, s.vigour.to_bits(), bits(s.living))));
    kept.sort_unstable();
    roots.sort_unstable();
    (kept, roots)
}

/// fn-357: a retired root let go keeps what is read of it. Grown letting
/// go of every retired record, its root included, the grower holds stands
/// in their place, and every closed subtree's living, each one evaluated
/// with the roots let go below it, is the one a grower keeping every
/// record holds, to the bit.
#[test]
fn a_retired_root_let_go_is_read_as_it_was() {
    for (species, k) in species() {
        let species = std::sync::Arc::new(species);
        let r = request(40, 7, k);
        let mut eager = Working::default();
        eager.eager = true;
        let mut let_go = Paused::with(species.clone(), r, false, eager).unwrap();
        let mut kept = Paused::with(species, r, false, Working::indexed()).unwrap();
        for age in [12, 25, 40] {
            let_go.grow_to(age).unwrap();
            kept.grow_to(age).unwrap();
            assert_eq!(livings(&let_go), livings(&kept), "at {age}");
        }
        let held = |p: &Paused| p.g.working.as_ref().map(|w| w.retire.stands.len());
        assert!(held(&let_go) > Some(100) && held(&kept) == Some(0));
    }
}

/// The light's deposit is the leaf every open phytomer bears that year
/// (fn-354 R5): with leaves kept three years, the oak's phytomers of the
/// two years before deposit beside the year's own, and the area is the
/// open wood's; closed wood bears none.
#[test]
fn the_deposit_is_every_held_phytomers_leaf() {
    let mut species = crate::oak();
    for s in &mut species.states {
        (s.leaf_years, s.shedding) = (3.0, f64::INFINITY);
    }
    let r = request(9, 7, 0.5);
    let mut p = Paused::with(
        std::sync::Arc::new(species.clone()),
        r,
        false,
        Working::default(),
    )
    .unwrap();
    p.grow_to(8).unwrap();
    let g = &p.g;
    let (mut held, mut year) = (0.0, 0.0);
    let w = g.working.as_deref().expect("a working set");
    let closed = g
        .axes
        .iter()
        .enumerate()
        .filter(|&(i, _)| w.closed(i))
        .count();
    assert!(closed > 0, "some wood has closed");
    for (i, axis) in g.axes.iter().enumerate() {
        if w.closed(i) {
            continue;
        }
        let state = &species.states[axis.pa];
        for q in &axis.phytomers {
            held += crate::girth::leaf(state, q, 8);
            year += f64::from(u8::from(q.cycle == 8)) * crate::girth::leaf(state, q, 8);
        }
    }
    let deposit: f64 = g.leaves.iter().map(|l| l.1).sum();
    assert!(held > 1.5 * year, "{held} against this year's {year}");
    assert!(
        (deposit - held).abs() <= 1e-9 * held,
        "{deposit} against {held}"
    );
}
