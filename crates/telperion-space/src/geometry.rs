//! Minimal geometry: straight axes of fixed internodes, each lateral turned
//! from its parent by its PA's insertion angle, around the parent at the
//! azimuth its node's divergence and its slot in the whorl give. The seed
//! stands at the origin, growing up (+z); no wood goes below z = 0.
use crate::error::{Error, Result};
use crate::species::Species;
use crate::structure::{Origin, Structure, Vec3};
use std::f64::consts::TAU;

/// Wood this far below the ground plane is rounding, not wood below it.
const GROUND_TOLERANCE: f64 = 1e-9;

pub(crate) fn place(structure: &mut Structure, species: &Species) -> Result<()> {
    for i in 0..structure.axes.len() {
        let (base, heading, side) = frame(structure, species, i);
        let axis = &mut structure.axes[i];
        let step = heading * species.states[axis.pa].internode;
        let mut tip = base;
        for phytomer in &mut axis.phytomers {
            tip = tip + step;
            if tip.z < -GROUND_TOLERANCE {
                return Err(Error::BelowGround {
                    axis: i,
                    height: -tip.z,
                });
            }
            phytomer.tip = tip;
        }
        axis.base = base;
        axis.heading = heading;
        axis.side = side;
    }
    Ok(())
}

/// The base, heading and side of axis `i`, from its already placed parent.
fn frame(structure: &Structure, species: &Species, i: usize) -> (Vec3, Vec3, Vec3) {
    let axis = &structure.axes[i];
    match axis.origin {
        Origin::Seed => (
            Vec3::default(),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        Origin::Continuation { parent } => {
            let p = &structure.axes[parent];
            let base = p.phytomers.last().map_or(p.base, |last| last.tip);
            (base, p.heading, p.side)
        }
        Origin::Lateral {
            parent,
            node,
            slot,
            whorl,
        } => {
            let p = &structure.axes[parent];
            let divergence = species.states[p.pa].divergence;
            let azimuth = divergence * node as f64 + TAU * f64::from(slot) / f64::from(whorl);
            let toward = p.side * azimuth.cos() + p.heading.cross(p.side) * azimuth.sin();
            let angle = species.states[axis.pa].insertion;
            let heading = p.heading * angle.cos() + toward * angle.sin();
            let side = toward * angle.cos() - p.heading * angle.sin();
            (p.phytomers[node].tip, heading, side)
        }
    }
}
