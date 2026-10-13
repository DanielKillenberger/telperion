//! fn-270 task 1's premises on the CPU cycle (spec R1, R2a, R2a'), on the
//! engine's own species at small ages; every preset at its default age is
//! `telperion-core`'s `premises` test.
use super::*;
use super::super::{grow_census, grow_cycle};
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

/// R1 (b): under the horizon, the tree at a year of a run that goes on
/// is the tree of a run stopped there, and retired at the age grown, it
/// is the tree with nothing retired at every year. (Retired at the
/// horizon, as today, a year's settle reads subtrees let go while they
/// still fade: PREMISES.md.)
#[test]
fn the_tree_at_a_year_is_the_horizons_prefix() {
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
        let at_age = walk(Retiring::AtAge);
        assert_eq!(at_age, walk(Retiring::Never));
        for y in [5, 12, h - 1] {
            let mut p = Paused::begin(&species, r, Retiring::AtAge, false).unwrap();
            p.grow_to(y).unwrap();
            assert_eq!(tree_digest(&p.settle().unwrap()), at_age[y as usize - 1], "year {y}");
        }
    }
}

/// R1 (c) at a small age: retired at the age grown, the kept tree is
/// today's, and the live set's peak no more than 25% above today's (every
/// preset at seeds 1, 7 and 42 is `premises kept`).
#[test]
fn retirement_at_the_age_grown_keeps_todays_tree() {
    for (species, k) in species() {
        for seed in [7] {
            let r = request(30, seed, k);
            let today = grow_census(&species, r).unwrap();
            let mut p = Paused::begin(&species, r, Retiring::AtAge, true).unwrap();
            p.grow_to(r.age).unwrap();
            let held = |c: &Census| c.cycles.iter().map(|c| c.phytomers).max().unwrap();
            let rise = held(p.census().unwrap()) as f64 / held(today.census.as_ref().unwrap()) as f64;
            assert!(rise <= 1.25, "seed {seed}: the live set rose {rise}");
            assert!(p.finish(0.0).unwrap().structure == today.structure, "seed {seed}");
        }
    }
}

/// Past the horizon and backward, a request is refused by name, and the
/// grower is left as it was.
#[test]
fn past_the_horizon_and_backward_are_refused() {
    let species = crate::beech();
    let mut p = Paused::begin(&species, request(12, 3, 0.0), Retiring::AtAge, false).unwrap();
    p.grow_to(8).unwrap();
    let before = p.digest();
    for (age, name) in [(13, "age.horizon"), (7, "age.backward")] {
        match p.grow_to(age) {
            Err(Error::Refused { input, .. }) => assert_eq!(input, name),
            other => panic!("{age}: {other:?}"),
        }
    }
    assert_eq!(p.digest(), before);
}

/// R2a and R2a' with a stand-in presentation: this year's phytomers drawn
/// at the fraction of the year, every other one whole. The real curves
/// are task 4's.
struct Unfolding<'a>(Paused<'a>);

impl Unfolding<'_> {
    /// Commits the cycles up to `a`'s ceiling; a request within the last
    /// year grown is already met, and one a year or more back refused.
    fn grow_to(&mut self, a: f64) -> Result<()> {
        let committed = f64::from(self.0.committed());
        if !a.is_finite() {
            return refuse("age", "an age is finite");
        }
        if a <= committed - 1.0 {
            return refuse("age.backward", "a tree grows only forward");
        }
        if a <= committed {
            return Ok(());
        }
        self.0.grow_to(a.ceil() as u32)
    }

    /// The tree grown, drawn at `at` within its last year.
    fn settle(&self, at: f64) -> Structure {
        let committed = self.0.committed();
        let fraction = at - f64::from(committed - 1);
        assert!(fraction > 0.0 && fraction <= 1.0, "{at} within the year to {committed}");
        let mut tree = self.0.settle().unwrap();
        if fraction < 1.0 {
            for p in tree.axes.iter_mut().flat_map(|a| &mut a.phytomers) {
                if p.cycle == committed {
                    p.scale *= fraction;
                }
            }
        }
        tree
    }
}

/// R2a': 40, 40.25, 40.5, 41 commits cycle 41 once, at 40.25, and the
/// tree at 41 is the one grown to 41 in one call. R2a: each fractional
/// settle leaves the grower as it was, the whole year draws today's tree,
/// and just past 40 this year's phytomers start from nothing.
#[test]
fn a_tree_drawn_within_its_year_unfolds_from_the_last() {
    let species = crate::oak();
    let r = request(41, 7, 0.5);
    let mut u = Unfolding(Paused::begin(&species, r, Retiring::AtAge, false).unwrap());
    u.grow_to(40.0).unwrap();
    assert_eq!(u.0.committed(), 40);
    let mut committed = Vec::new();
    for at in [40.25, 40.5, 41.0] {
        u.grow_to(at).unwrap();
        committed.push(u.0.committed());
        let before = u.0.digest();
        let drawn = u.settle(at);
        assert_eq!(u.0.digest(), before, "a settle at {at} moves the grower");
        let new = drawn.axes.iter().flat_map(|a| &a.phytomers).filter(|p| p.cycle == 41);
        assert!(new.clone().count() > 0);
        let whole = u.0.settle().unwrap();
        let whole = whole.axes.iter().flat_map(|a| &a.phytomers).filter(|p| p.cycle == 41);
        for (p, w) in new.zip(whole) {
            assert_eq!(p.scale, w.scale * (at - 40.0));
        }
    }
    assert_eq!(committed, [41, 41, 41]);
    match u.grow_to(40.0) {
        Err(Error::Refused { input, .. }) => assert_eq!(input, "age.backward"),
        other => panic!("{other:?}"),
    }
    let today = grow_cycle(&species, r).unwrap().structure;
    assert_eq!(tree_digest(&u.settle(41.0)), tree_digest(&today));
    let largest = (today.axes.iter().flat_map(|a| &a.phytomers)).map(|p| p.scale).fold(0.0, f64::max);
    let just = u.settle(40.0 + 1e-9);
    let new = just.axes.iter().flat_map(|a| &a.phytomers).filter(|p| p.cycle == 41);
    assert!(new.clone().all(|p| p.scale <= 1e-8 * largest), "this year's start from nothing");
    // The join with year 40's tree: its organs drawn just past 40 are
    // year 41's, so they join it only where the year sheds or fades
    // nothing. The stand-in leaves that step whole (PREMISES.md, R2a).
    let mut forty = Paused::begin(&species, r, Retiring::AtAge, false).unwrap();
    forty.grow_to(40).unwrap();
    let forty = forty.settle().unwrap();
    let whole = u.settle(41.0);
    let (joined, step) = (organs(&just), organs(&whole));
    let old = |m: &HashMap<(u64, usize), f64>| m.iter().filter(|(_, s)| **s > 1e-8 * largest).count();
    let mut moved = 0;
    for (key, s) in organs(&forty) {
        let (at, next) = (joined.get(&key), step.get(&key));
        assert_eq!(at, next, "just past 40 an organ of 40 is drawn as at 41");
        moved += usize::from(at != Some(&s));
    }
    assert!(old(&joined) > 0);
    eprintln!("JOIN oak 40 -> 40+: {moved} of year 40's organs moved or went");
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
