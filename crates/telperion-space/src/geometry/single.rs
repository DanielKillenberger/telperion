//! One axis laid alone from its laid parent (fn-209, option 4): what
//! `place` does for a depth, for the one axis a chain's resolution lays,
//! so a chain is laid whenever it is resolved and the tree is the same in
//! any order.
use super::lay::{lay, Layer, GROUND_TOLERANCE};
use super::{bend, frame, turned};
use crate::error::{Error, Result};
use crate::lineage::{Key, ROLL};
use crate::sag::Lever;
use crate::species::Species;
use crate::structure::{Axis, Origin, Vec3};
use std::f64::consts::TAU;

/// Lays axis `i` from its parent's laid frame (`layers`, by index), its
/// relays pulled as far as the axis they replace (`reaches`), with
/// `levers` where it sags; `trunk` whether it carries the seed on.
pub(crate) fn lay_one(
    axes: &mut [Axis],
    species: &Species,
    (i, age, trunk): (usize, u32, bool),
    layers: &[Option<Layer>],
    reaches: &[f64],
    levers: Option<&[Lever]>,
) -> Result<Layer> {
    let ends = |p: usize| layers[p].as_ref().map(Layer::ends).expect("laid");
    let framed = frame(axes, species, i, &ends);
    let years = |a: &Axis| f64::from(age.saturating_sub(a.birth)) - a.sleep;
    let axis = &axes[i];
    let curve = match axis.origin {
        Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
            layers[parent].as_ref().map(Layer::curve).expect("laid")
        }
        _ => Vec3::default(),
    };
    let inherited = match axis.carried_on() {
        Some((parent, b)) => {
            let p = &axes[parent];
            let bent = bend(p, &species.states[p.pa], years(p));
            layers[parent]
                .as_ref()
                .expect("laid")
                .handed((bent, reaches[parent]), b)
        }
        None => Vec::new(),
    };
    let state = &species.states[axis.pa];
    let own = (bend(axis, state, years(axis)), reaches[i]);
    let (pa, birth, base) = (axis.pa, axis.birth, framed.0.z);
    let below = |height: f64| Error::BelowGround {
        axis: i,
        pa,
        birth,
        base,
        height,
    };
    if base < -GROUND_TOLERANCE {
        return Err(below(-base));
    }
    let start = (framed, curve, inherited);
    lay(&mut axes[i], start, state, own, levers, trunk).map_err(below)
}

/// Where a lateral bud would stand and face on its laid bearer, as
/// `frame` places a lateral: its base and heading.
pub(crate) fn bud_frame(
    axes: &[Axis],
    species: &Species,
    bud: (Origin, usize, u64),
) -> (Vec3, Vec3) {
    let (origin, pa, lineage) = bud;
    let Origin::Lateral {
        parent,
        node,
        slot,
        whorl,
        woken,
    } = origin
    else {
        unreachable!("a bud record is a lateral")
    };
    let p = &axes[parent];
    let at = &p.phytomers[node];
    let parent_state = &species.states[p.pa];
    let roll = species.states[pa].form.roll * (2.0 * Key(lineage).child(ROLL).unit() - 1.0);
    let azimuth = super::phase(axes, species, parent)
        + parent_state.divergence * at.rank
        + TAU * f64::from(slot) / whorl
        + roll;
    let (heading, _) = turned(
        (at.heading, at.side),
        azimuth,
        species.states[pa].insertion,
        parent_state.form.plane,
        0.0,
    );
    let base = if woken {
        let below = node.checked_sub(1).map_or(p.base, |n| p.phytomers[n].tip);
        (below + at.tip) * 0.5
    } else {
        at.tip
    };
    (base, heading)
}
