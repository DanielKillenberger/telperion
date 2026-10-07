//! One axis laid alone from its laid parent (fn-209, option 4): what
//! `place` does for a depth, for the one axis a chain's resolution lays,
//! so a chain is laid whenever it is resolved and the tree is the same in
//! any order.
use super::lay::{lay, Layer, Stop, GROUND_TOLERANCE};
use super::{bend, frame};
use crate::error::Result;
use crate::sag::Lever;
use crate::species::Species;
use crate::structure::{Axis, Origin, Vec3};

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
    let inherited = match axis.origin {
        Origin::Relay { parent, .. } => {
            let p = &axes[parent];
            let bent = bend(p, &species.states[p.pa], years(p));
            layers[parent]
                .as_ref()
                .expect("laid")
                .handed((bent, reaches[parent]), axis.blend)
        }
        _ => Vec::new(),
    };
    let state = &species.states[axis.pa];
    let own = (bend(axis, state, years(axis)), reaches[i]);
    let (pa, birth, base) = (axis.pa, axis.birth, framed.0.z);
    if base < -GROUND_TOLERANCE {
        return Err(Stop::Below(-base).error(i, pa, birth, base));
    }
    let start = (framed, curve, inherited);
    lay(&mut axes[i], start, state, own, levers, trunk).map_err(|s| s.error(i, pa, birth, base))
}

/// Where a lateral bud would stand and face on its laid bearer, as
/// `frame` places a lateral: its base and heading.
pub(crate) fn bud_frame(
    axes: &[Axis],
    species: &Species,
    bud: (Origin, usize, u64),
) -> (Vec3, Vec3) {
    let (base, heading, _) = super::lateral(axes, species, bud);
    (super::q::v3(base), super::q::v3(heading))
}
