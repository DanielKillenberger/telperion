//! The extent bound against grown trees (fn-209, host decision 16).
use super::super::Kept;
use super::{shedding_species, tree};
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::structure::{Origin, Structure};

/// No wood stands farther from a lateral's base than its extent bound
/// (host decision 16), on the stochastic species and on the beech and
/// spruce, bent as they are.
#[test]
fn no_wood_reaches_past_the_extent_bound() {
    let all = [
        (shedding_species(), 12u32, 30u64),
        (crate::beech(), 20, 2),
        (crate::spruce(), 20, 2),
    ];
    for (species, age, seeds) in all {
        let windows = Windows::new(&species, age);
        let mut kept = Kept::new(&species, &windows);
        for seed in 0..seeds {
            let Ok(grown) = tree(&species, age, seed) else {
                continue;
            };
            let reached = farthest(&grown);
            for (i, axis) in grown.axes.iter().enumerate() {
                let Origin::Lateral { parent, node, .. } = axis.origin else {
                    continue;
                };
                let base = grown.axes[parent].phytomers[node].scale * axis.vigour;
                let m = (age - axis.birth) as usize;
                let bound = kept.reach_bound(axis.pa, m, Carried::default(), None) * base;
                assert!(
                    // The layout is f32 (fn-254): a reach holds its bound to a
                    // few of f32's last places, summed over its walk, and to
                    // the f32 places of the positions it is read from (over
                    // 101,528 laterals on fn-248's beech: at most 3.0e-6 m
                    // past, 2.2e-6 relative on a 1.35 m bound and 2.1e-6 m
                    // on a 0.8 mm one).
                    reached[i] <= bound * (1.0 + 1e-5) + 4e-6,
                    "axis {i} (PA {}, born {}) reaches {} past its bound {bound}",
                    axis.pa,
                    axis.birth,
                    reached[i]
                );
            }
        }
    }
}

/// How far each lateral's subtree's wood stands from the lateral's base:
/// every phytomer's tip, measured from each lateral it stands on.
pub(super) fn farthest(tree: &Structure) -> Vec<f64> {
    let mut reached = vec![0.0f64; tree.axes.len()];
    for (j, axis) in tree.axes.iter().enumerate() {
        for p in &axis.phytomers {
            let mut at = Some(j);
            while let Some(i) = at {
                let a = &tree.axes[i];
                if matches!(a.origin, Origin::Lateral { .. }) {
                    reached[i] = reached[i].max((p.tip - a.base).length());
                }
                at = a.origin.parent();
            }
        }
    }
    reached
}
