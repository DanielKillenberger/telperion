//! fn-206: the shared reference axis. An age a species does not use is
//! passed through at lifespan 0: its apex moves on without growing in it,
//! so a species written with ages it skips grows the tree it grew without
//! them.
mod walk;
use telperion_space::{expected_counts, grow, Request, Species};

/// `species` with an age of no lifespan put before each of its own, every
/// PA index shifted to its new place: each passed age moves on to the age
/// it stands before.
fn spread(species: &Species) -> Species {
    let at = |pa: usize| 2 * pa + 1;
    let count = 2 * species.states.len();
    let widen = |table: &[f64]| {
        let mut wide = vec![0.0; count];
        for (pa, &p) in table.iter().enumerate() {
            wide[at(pa)] = p;
        }
        wide
    };
    let mut states = Vec::with_capacity(count);
    for state in &species.states {
        let mut lived = state.clone();
        for zone in &mut lived.zones {
            zone.lateral = widen(&zone.lateral);
            zone.dormant = widen(&zone.dormant);
        }
        let mut passed = lived.clone();
        passed.lifespan = 0.0;
        passed.continuation = 1.0;
        states.push(passed);
        states.push(lived);
    }
    Species { states }
}

/// An age passed through grows nothing and takes nothing from the ages
/// around it: the expected phytomers of every age are the ones the species
/// grew without it, and no axis grows in it. The draws of a continuation
/// are keyed by its age's place on the axis (`Key::onto`), so the grown
/// trees differ in their draws, not in their law.
#[test]
fn an_age_passed_through_grows_nothing() {
    let species = walk::species();
    let spread = spread(&species);
    let (a, b) = (
        expected_counts(&species, walk::AGE).unwrap(),
        expected_counts(&spread, walk::AGE).unwrap(),
    );
    for pa in 0..species.states.len() {
        for cycle in 1..=a.cycles {
            assert_eq!(a.get(pa, cycle), b.get(2 * pa + 1, cycle), "{pa} {cycle}");
            assert_eq!(b.get(2 * pa, cycle), 0.0, "{pa} {cycle}");
        }
    }
    for seed in [1, 2, 3] {
        let request = Request {
            age: walk::AGE,
            seed,
            budget: 1_000_000,
            light: telperion_space::Light::NEUTRAL,
        };
        let tree = grow(&spread, request).unwrap();
        assert!(
            tree.axes.iter().all(|x| x.pa % 2 == 1),
            "no axis grows in a passed age"
        );
    }
}

/// A chain that ends in ages passed through grows nothing past them.
#[test]
fn a_chain_of_passed_ages_collapses() {
    let mut species = walk::species();
    species.states[0].lifespan = 0.0;
    species.states[0].continuation = 0.0;
    let request = Request {
        age: 3,
        seed: 1,
        budget: 1_000_000,
        light: telperion_space::Light::NEUTRAL,
    };
    assert!(grow(&species, request).is_err());
}

/// The point a share 0 or 1 of the way between two species grows each
/// species' own tree: the walk between them starts and ends on them.
#[test]
fn a_blend_ends_on_its_species() {
    use telperion_space::{beech, blend, oak, palm, spruce};
    let request = Request {
        age: 10,
        seed: 1,
        budget: 1_000_000,
        light: telperion_space::Light::NEUTRAL,
    };
    let tips = |s: &Species| -> Vec<(f64, f64, f64)> {
        let tree = grow(s, request).unwrap();
        let mut tips: Vec<_> = tree
            .axes
            .iter()
            .flat_map(|a| a.phytomers.iter().map(|p| (p.tip.x, p.tip.y, p.tip.z)))
            .collect();
        tips.sort_by(|a, b| a.partial_cmp(b).unwrap());
        tips
    };
    let close = |a: &[(f64, f64, f64)], b: &[(f64, f64, f64)]| {
        a.len() == b.len()
            && a.iter()
                .zip(b)
                .all(|(p, q)| (p.0 - q.0).abs() + (p.1 - q.1).abs() + (p.2 - q.2).abs() < 1e-6)
    };
    let pairs = [(beech(), spruce()), (spruce(), oak()), (oak(), palm())];
    for (a, b) in &pairs {
        assert!(close(&tips(&blend(a, b, 0.0).unwrap()), &tips(a)), "t = 0");
        assert!(close(&tips(&blend(a, b, 1.0).unwrap()), &tips(b)), "t = 1");
    }
}

/// A species in canonical form carries, at every age it does not use, its
/// nearest used age's settings, passed through: it grows the tree it grew.
#[test]
fn a_canonical_species_grows_its_own_tree() {
    let species = walk::species();
    let spread = spread(&species);
    let canonical = spread.clone().canonical();
    for (k, state) in canonical.states.iter().enumerate() {
        if k % 2 == 0 {
            assert_eq!(state.lifespan, 0.0, "age {k} is passed through");
            // Its nearest used age: the earlier on a tie, the next before
            // the first.
            let near = if k == 0 { 1 } else { k - 1 };
            assert_eq!(state.internode, spread.states[near].internode, "age {k}");
            assert!(state
                .zones
                .iter()
                .all(|z| z.lateral.iter().all(|&p| p == 0.0)));
        }
    }
    let (a, b) = (
        expected_counts(&spread, walk::AGE).unwrap(),
        expected_counts(&canonical, walk::AGE).unwrap(),
    );
    assert_eq!(a, b);
}

/// A species that is not valid is refused by the walk between species,
/// never read: an age with no zones (fn-206).
#[test]
fn a_blend_refuses_an_invalid_species() {
    let mut broken = telperion_space::beech();
    broken.states[9].zones.clear();
    assert!(telperion_space::blend(&broken, &telperion_space::beech(), 0.5).is_err());
}

/// Host decision 12: a stop's three outcomes mix as shares. The beech's
/// trunk aborts half its years and always relays; the spruce's never
/// aborts: halfway, it aborts a quarter of its years and always relays.
#[test]
fn a_blend_mixes_a_stops_outcomes_as_shares() {
    let half =
        telperion_space::blend(&telperion_space::beech(), &telperion_space::spruce(), 0.5).unwrap();
    let trunk = &half.states[2];
    assert!((trunk.abortion - 0.25).abs() < 1e-12, "{}", trunk.abortion);
    assert_eq!(trunk.relay, 1.0);
}
