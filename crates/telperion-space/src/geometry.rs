//! Minimal geometry: axes of internodes of their PA's length times their
//! scale, each lateral turned from its parent by its PA's insertion angle,
//! around the parent at the azimuth its node's phyllotactic rank and its
//! slot in the whorl give. A lateral's base straightens towards the vertical by its
//! PA's straightening. The seed stands at the origin, growing up (+z); no
//! wood goes below z = 0.
use crate::error::{Error, Result};
use crate::species::Species;
use crate::structure::{Axis, Origin, Structure, Vec3};
use std::f64::consts::TAU;

/// Wood this far below the ground plane is rounding, not wood below it.
const GROUND_TOLERANCE: f64 = 1e-9;
const UP: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub(crate) fn place(structure: &mut Structure, species: &Species) -> Result<()> {
    // Each axis's scale at its base, from its already scaled parent.
    let mut base_scale = vec![1.0; structure.axes.len()];
    for i in 0..structure.axes.len() {
        let (base, heading, side) = frame(structure, species, i);
        let inherited = match structure.axes[i].origin {
            Origin::Seed => 1.0,
            Origin::Lateral { parent, node, .. } => structure.axes[parent].phytomers[node].scale,
            Origin::Continuation { parent } | Origin::Relay { parent } => base_scale[parent],
        };
        let axis = &mut structure.axes[i];
        base_scale[i] = inherited * axis.vigour;
        for phytomer in &mut axis.phytomers {
            phytomer.scale *= base_scale[i];
        }
        let state = &species.states[axis.pa];
        let bend = match axis.origin {
            Origin::Lateral { .. } | Origin::Relay { .. } => state.straightening,
            _ => 0.0,
        };
        lay(axis, base, heading, side, state.internode, bend)
            .map_err(|height| Error::BelowGround { axis: i, height })?;
    }
    Ok(())
}

/// Lays the axis's internodes from `base`. Each phytomer's direction is
/// its heading pulled towards the vertical by `bend` at the base, fading to
/// none at the tip along the axis's scaled length; the pull weakens as the
/// heading turns down and vanishes for a branch hanging straight down,
/// which has no side to curl up on.
fn lay(
    axis: &mut Axis,
    base: Vec3,
    heading: Vec3,
    side: Vec3,
    internode: f64,
    bend: f64,
) -> std::result::Result<(), f64> {
    let total: f64 = axis.phytomers.iter().map(|p| p.scale).sum();
    let pull = (1.0 + heading.z) / 2.0;
    let mut tip = base;
    let mut run = 0.0;
    for phytomer in &mut axis.phytomers {
        let along = if total > 0.0 {
            (run + phytomer.scale / 2.0) / total
        } else {
            1.0
        };
        run += phytomer.scale;
        let share = bend * (1.0 - along);
        let blend = heading * (1.0 - share) + UP * (share * pull);
        let direction = if bend > 0.0 && blend.length() > 1e-12 {
            blend * (1.0 / blend.length())
        } else {
            heading
        };
        tip = tip + direction * (internode * phytomer.scale);
        if tip.z < -GROUND_TOLERANCE {
            return Err(-tip.z);
        }
        phytomer.tip = tip;
    }
    axis.base = base;
    axis.heading = heading;
    axis.side = side;
    Ok(())
}

/// The base, heading and side of axis `i`, from its already placed parent.
fn frame(structure: &Structure, species: &Species, i: usize) -> (Vec3, Vec3, Vec3) {
    let axis = &structure.axes[i];
    match axis.origin {
        Origin::Seed => (Vec3::default(), UP, Vec3::new(1.0, 0.0, 0.0)),
        Origin::Continuation { parent } => {
            let p = &structure.axes[parent];
            let base = p.phytomers.last().map_or(p.base, |last| last.tip);
            // The phyllotaxis runs on across the change of PA.
            let turn = species.states[p.pa].divergence * p.rank;
            let side = p.side * turn.cos() + p.heading.cross(p.side) * turn.sin();
            (base, p.heading, side)
        }
        Origin::Lateral {
            parent,
            node,
            slot,
            whorl,
        } => {
            let p = &structure.axes[parent];
            let azimuth = species.states[p.pa].divergence * p.phytomers[node].rank
                + TAU * f64::from(slot) / f64::from(whorl);
            let (heading, side) = turned(p, azimuth, species.states[axis.pa].insertion);
            (p.phytomers[node].tip, heading, side)
        }
        Origin::Relay { parent } => {
            // At the last node, facing where the next node's bud would.
            let p = &structure.axes[parent];
            let azimuth = species.states[p.pa].divergence * p.rank;
            let (heading, side) = turned(p, azimuth, species.states[axis.pa].insertion);
            let base = p.phytomers.last().map_or(p.base, |last| last.tip);
            (base, heading, side)
        }
    }
}

/// The heading and side of a bud at `azimuth` around parent `p`, inserted at `angle`.
fn turned(p: &Axis, azimuth: f64, angle: f64) -> (Vec3, Vec3) {
    let toward = p.side * azimuth.cos() + p.heading.cross(p.side) * azimuth.sin();
    let heading = p.heading * angle.cos() + toward * angle.sin();
    let side = toward * angle.cos() - p.heading * angle.sin();
    (heading, side)
}
