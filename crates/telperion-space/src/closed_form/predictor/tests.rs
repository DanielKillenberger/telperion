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
            let count = predictor.count(pa, birth);
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
                let counts = predictor.counts.into_iter().filter(|c| !c.is_nan());
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
