//! The predictor's count is the closed form's row `count` to the bit, for
//! every PA a bud grows in and every age a bud can have.
use super::Predictor;
use crate::closed_form::boundary::Rows;
use crate::presence::Windows;
use crate::species::Species;
use crate::{beech, oak, palm, spruce};

/// Every count `species` can be asked at `age`, against the rows'.
fn same(species: &Species, age: u32) -> usize {
    let rows = Rows::new(species, age, 1);
    let predictor = Predictor::new(species, &Windows::new(species, age), age);
    let mut compared = 0;
    for pa in 0..species.states.len() {
        for birth in 0..=age {
            let Some(row) = rows.row_at(pa, (age - birth) as usize) else {
                continue;
            };
            let count = predictor.count(pa, birth).expect("a count");
            assert_eq!(
                count.to_bits(),
                row.count.to_bits(),
                "PA {pa}, birth {birth}"
            );
            compared += 1;
        }
    }
    compared
}

#[test]
fn the_count_is_the_rows_to_the_bit() {
    // Relays, dormancy, sleeping buds and passed-through ages among them.
    let mut compared = 0;
    for species in [beech(), spruce(), oak(), palm()] {
        for age in [1, 2, 9, 30] {
            compared += same(&species, age);
        }
    }
    assert!(compared > 500, "{compared}");
}

#[test]
fn an_old_tree_needs_no_deep_stack() {
    // The rows recurse about a kilobyte a cycle (`cores::deep`); the
    // predictor holds an age of 300 on a thread of 128 kB. The windows
    // are the grower's, made before.
    let species = spruce();
    let windows = Windows::new(&species, 300);
    let most = std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(128 << 10)
            .spawn_scoped(scope, || {
                let predictor = Predictor::new(&species, &windows, 300);
                let counts = predictor.counts.into_iter().flatten();
                counts.fold(0.0, f64::max)
            })
            .expect("a thread")
            .join()
            .expect("no overflow")
    });
    assert!(most > 1e3, "{most}");
}

/// The same at the ages the trees are proved at (fn-209 decision 9):
/// `cargo test --profile ci -p telperion-space -- --ignored counts_at`.
#[test]
#[ignore = "the rows at age 80, for the proof"]
fn counts_at_40_and_80() {
    for (name, species) in [
        ("beech", beech()),
        ("spruce", spruce()),
        ("oak", oak()),
        ("palm", palm()),
    ] {
        for age in [40, 80] {
            eprintln!("{name} at {age}: {} counts equal", same(&species, age));
        }
    }
}

/// The whole tree's expected kept phytomers (fn-198 host decision 15) is
/// the closed form's seed row to the bit; a species whose buds read light
/// has none, and the palm, which sheds by no draw, keeps exactly it.
#[test]
fn the_trees_count_is_the_seed_row() {
    for species in [beech(), spruce(), palm()] {
        for age in [1, 9, 30] {
            let rows = Rows::new(&species, age, 1);
            let kept = crate::expected_kept(&species, age).unwrap().unwrap();
            assert_eq!(kept.to_bits(), rows.seed().count.to_bits(), "age {age}");
        }
    }
    assert_eq!(crate::expected_kept(&oak(), 9).unwrap(), None);
    let request = crate::Request {
        age: 30,
        seed: 3,
        budget: 1_000_000,
        light: crate::Light::default(),
    };
    let tree = crate::grow(&palm(), request).unwrap();
    let kept: usize = tree.axes.iter().map(|a| a.phytomers.len()).sum();
    let expected = crate::expected_kept(&palm(), 30).unwrap().unwrap();
    assert_eq!(kept as f64, expected);
}

/// A PA no zone draws has no count (fn-198 task 6: it was a NaN, which a
/// debug build refused and the cut's `<=` read as "not fine"), and every
/// lateral a tree grows stands in a PA that has one, so the cut never
/// classifies a bud on a missing count.
#[test]
fn every_lateral_grown_has_a_count_and_an_undrawn_pa_has_none() {
    use crate::structure::Origin;
    let lit = crate::Light {
        extinction: 0.5,
        sky: 0.5,
    };
    for (species, light) in [
        (beech(), crate::Light::NEUTRAL),
        (spruce(), crate::Light::NEUTRAL),
        (oak(), lit),
        (palm(), crate::Light::NEUTRAL),
    ] {
        let age = 20;
        let predictor = Predictor::new(&species, &Windows::new(&species, age), age);
        let borne = crate::closed_form::boundary::borne(&species);
        for pa in 0..species.states.len() {
            assert_eq!(predictor.count(pa, 0).is_some(), borne[pa], "PA {pa}");
        }
        let request = crate::Request {
            age,
            seed: 1,
            budget: 20_000_000,
            light,
        };
        let tree = crate::grow(&species, request).expect("grows");
        for axis in &tree.axes {
            if let Origin::Lateral { woken: false, .. } = axis.origin {
                assert!(
                    predictor.count(axis.pa, axis.birth).is_some(),
                    "PA {}",
                    axis.pa
                );
            }
        }
    }
    let beech = beech();
    let predictor = Predictor::new(&beech, &Windows::new(&beech, 6), 6);
    assert_eq!(predictor.count(0, 1), None, "the beech's PA 0 is drawn");
}

/// Every count the cut reads and every expected length, to the bit, with
/// each species' every PA's form `set`; each row's reach without and with.
fn same_counts(set: fn(&mut crate::species::Form)) -> Vec<(f64, f64)> {
    use crate::closed_form::expected_log_lengths;
    let mut reaches = Vec::new();
    for species in [beech(), spruce(), oak(), palm()] {
        let mut varied = species.clone();
        for state in &mut varied.states {
            set(&mut state.form);
        }
        let age = 30;
        let windows = Windows::new(&species, age);
        let (a, b) = (
            Predictor::new(&species, &windows, age),
            Predictor::new(&varied, &Windows::new(&varied, age), age),
        );
        let bits = |p: &Predictor| -> Vec<Option<u64>> {
            p.counts.iter().map(|c| c.map(f64::to_bits)).collect()
        };
        assert_eq!(bits(&a), bits(&b));
        let bits = |l: Vec<Vec<f64>>| -> Vec<Vec<u64>> {
            l.iter()
                .map(|r| r.iter().map(|v| v.to_bits()).collect())
                .collect()
        };
        assert_eq!(
            bits(expected_log_lengths(&species, age)),
            bits(expected_log_lengths(&varied, age))
        );
        let (rows, wide) = (Rows::new(&species, age, 1), Rows::new(&varied, age, 1));
        for pa in 0..species.states.len() {
            for m in 0..=age as usize {
                let (Some(r), Some(w)) = (rows.row_at(pa, m), wide.row_at(pa, m)) else {
                    continue;
                };
                assert_eq!(r.count.to_bits(), w.count.to_bits(), "PA {pa}, {m}");
                reaches.push((r.reach, w.reach));
            }
        }
    }
    reaches
}

/// fn-222 R3: an internode spread changes no count the cut reads and no
/// expected length, to the bit (its factor's mean is one); only the reach
/// bound widens, each internode taken at its longest.
#[test]
fn an_internode_spread_moves_no_count_and_no_expected_length() {
    for (r, w) in same_counts(|f| f.internode_spread = 0.8) {
        assert!(w >= r, "{w} < {r}");
    }
}

/// fn-224 R3: insertion and tropism spreads change no count the cut
/// reads, no expected length and no reach bound, to the bit: the counts
/// and lengths read no angle, and the bound takes every path straight.
#[test]
fn angle_spreads_move_no_count_no_expected_length_and_no_reach() {
    let set = |f: &mut crate::species::Form| {
        (f.insertion_spread, f.tropism_spread) = (std::f64::consts::FRAC_PI_2, 1.0);
    };
    for (r, w) in same_counts(set) {
        assert_eq!(r.to_bits(), w.to_bits());
    }
}
