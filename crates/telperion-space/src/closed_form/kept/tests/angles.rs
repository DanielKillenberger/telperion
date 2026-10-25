//! fn-224 R3: the kept closed form under an insertion spread. Every
//! lateral's angle is its PA's offset by a keyed draw uniform on [−s, s];
//! the formula reads the angle's cosine and its square in expectation
//! (`lineage::turn`), which is exact, so the deterministic tree's kept
//! quantities, averaged over seeds, are the formula's. Read as a constant
//! angle instead, the along and parallel moments stand off by many
//! standard errors.
use super::super::{Kept, ALONG, PARALLEL, SQUARE};
use super::{deterministic, moments, tree};
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::Species;
use crate::structure::Structure;

/// The tree's wood about its seed's base along its heading: Σ l (x · h),
/// Σ l |x|² and Σ l (x · h)².
fn moments_of(tree: &Structure) -> [f64; 3] {
    let (o, h) = (tree.axes[0].base, tree.axes[0].heading);
    let mut out = [0.0; 3];
    for axis in &tree.axes {
        let mut from = axis.base;
        for p in &axis.phytomers {
            let l = (p.tip - from).length();
            let x = (from + p.tip) * 0.5 - o;
            out[0] += l * x.dot(h);
            out[1] += l * x.dot(x);
            out[2] += l * x.dot(h).powi(2);
            from = p.tip;
        }
    }
    out
}

/// The formula's [along, square, parallel] for the seed at `age`.
fn formula(species: &Species, age: u32) -> [f64; 3] {
    let windows = Windows::new(species, age);
    let mut kept = Kept::new(species, &windows);
    let table = kept.of(0, age as usize, Carried::default(), None);
    let (_, _, b) = table.at(age as i64);
    let b = b.expect("born");
    [b[ALONG], b[SQUARE], b[PARALLEL]]
}

#[test]
fn an_insertion_spread_keeps_what_the_formula_keeps_in_expectation() {
    const NAMES: [&str; 3] = ["along", "square", "parallel"];
    let mut species = deterministic(false);
    for state in &mut species.states[1..] {
        state.form.insertion_spread = 0.6;
    }
    let constant = deterministic(false);
    for age in [3, 5] {
        let grown: Vec<[f64; 3]> = (0..400)
            .map(|seed| moments_of(&tree(&species, age, seed).expect("grows")))
            .collect();
        let (ours, theirs) = (formula(&species, age), formula(&constant, age));
        for q in 0..3 {
            let values: Vec<f64> = grown.iter().map(|m| m[q]).collect();
            let (mean, error) = moments(&values);
            let (by, off) = ((mean - ours[q]) / error, (mean - theirs[q]) / error);
            println!(
                "age {age} {}: grown {mean:.5} ± {error:.5}, formula {:.5} ({by:+.2} se), \
                 as a constant {:.5} ({off:+.2} se)",
                NAMES[q], ours[q], theirs[q]
            );
            assert!(by.abs() < 4.0, "age {age} {}: {by} se", NAMES[q]);
        }
    }
}
