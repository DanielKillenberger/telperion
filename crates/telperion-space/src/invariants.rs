//! A grown tree's invariants (fn-254 R4, check 3): what f32 growth must
//! keep on every shipped preset, whatever its last places. Each broken
//! one is named.
use crate::structure::{Origin, Structure};

/// How far below the ground wood may stand (m): the settle lands a tip
/// on the ground in f32.
pub const UNDER: f64 = 1e-4;

/// Every invariant `tree` breaks, by name: links that point forward or
/// nowhere, geometry that is not finite, headings that are not unit,
/// radii, scales or sizes below nothing, light outside nothing to one,
/// a node more than whole, and wood under the ground.
pub fn invariants(tree: &Structure) -> Vec<String> {
    let mut out = Vec::new();
    let mut fail = |what: &str, i: usize, k: usize| {
        if out.len() < 16 {
            out.push(format!("axis {i} node {k}: {what}"));
        }
    };
    for (i, axis) in tree.axes.iter().enumerate() {
        let linked = match axis.origin {
            Origin::Seed => i == 0,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => parent < i,
            Origin::Lateral { parent, node, .. } => {
                parent < i && node < tree.axes[parent].phytomers.len().max(1)
            }
        };
        if !linked {
            fail("a link forward or nowhere", i, 0);
        }
        for (k, p) in axis.phytomers.iter().enumerate() {
            let v = [p.tip, p.heading, p.side];
            if !v
                .iter()
                .all(|v| v.x.is_finite() && v.y.is_finite() && v.z.is_finite())
            {
                fail("geometry not finite", i, k);
            }
            if (p.heading.length() - 1.0).abs() > 1e-5 {
                fail("a heading not unit", i, k);
            }
            if !(p.radius >= 0.0 && p.scale >= 0.0 && p.size >= 0.0) {
                fail("a radius, scale or size below nothing", i, k);
            }
            if !(0.0..=1.0).contains(&p.light) {
                fail("light outside nothing to one", i, k);
            }
            if !(0.0..=1.0 + 1e-6).contains(&p.grown) {
                fail("a node more than whole", i, k);
            }
            if p.tip.z < -UNDER {
                fail("wood under the ground", i, k);
            }
        }
    }
    out
}
