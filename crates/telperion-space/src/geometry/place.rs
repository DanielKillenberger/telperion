//! Laying the tree (fn-210, lever 2): an axis is laid from its parent's
//! frame and nothing else, so the axes of one depth are laid at once, on
//! every core, after the depth that bears them. Where axes cannot be
//! laid, the error is the one of the first in index order, as laying them
//! in turn would give.
use super::lay::{lay, Layer, Lift, GROUND_TOLERANCE};
use super::{bend, frame, reaches};
use crate::cores::{picked, spread};
use crate::error::{Error, Result};
use crate::sag::Lever;
use crate::species::Species;
use crate::structure::{Origin, Structure, Vec3};
use std::sync::Mutex;

/// The axes laid in one piece of work.
const PIECE: usize = 64;

/// What an axis is laid from: its frame, its bearer's wander, the pulls
/// it carries on, its bend and reach, and whether it is the trunk.
type Start = ((Vec3, Vec3, Vec3), Vec3, Vec<Lift>, (f64, f64), bool);

/// Lays every axis from its parent's frame; with `levers` (`sag.rs`),
/// each phytomer also turns down under the load it carries; on up to
/// `threads` cores. Where each axis's walk ended, by index.
pub(crate) fn place(
    structure: &mut Structure,
    species: &Species,
    levers: Option<&[Vec<Lever>]>,
    threads: usize,
) -> Result<Vec<Layer>> {
    let n = structure.axes.len();
    let age = structure.age;
    let reaches = reaches(&structure.axes);
    let (mut layers, mut bends, mut trunk) = (vec![None; n], vec![0.0; n], vec![false; n]);
    // The first axis that cannot be laid, and why.
    let mut failed: Option<(usize, Error)> = None;
    for depth in depths(structure) {
        let mut starts: Vec<(usize, Start)> = Vec::with_capacity(depth.len());
        for i in depth {
            if failed.as_ref().is_some_and(|(f, _)| *f < i) {
                break;
            }
            let ends = |p: usize| layers[p].as_ref().map(Layer::ends).expect("laid");
            let framed = frame(&structure.axes, species, i, &ends);
            let axis = &structure.axes[i];
            // A continuation or relay carries its bearer's wander on.
            let curve = match axis.origin {
                Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                    layers[parent].as_ref().map(Layer::curve).expect("laid")
                }
                _ => Vec3::default(),
            };
            let state = &species.states[axis.pa];
            let years = f64::from(age.saturating_sub(axis.birth)) - axis.sleep;
            bends[i] = bend(axis, state, years);
            // A relay carries on the pulls of the axis it replaces, as far
            // as it is still that axis's continuation, so one that has
            // barely left the axis's line is laid as the axis would have
            // gone on (fn-206).
            let inherited = match axis.origin {
                Origin::Relay { parent, .. } => layers[parent]
                    .as_ref()
                    .expect("laid")
                    .handed((bends[parent], reaches[parent]), axis.blend),
                _ => Vec::new(),
            };
            trunk[i] = match axis.origin {
                Origin::Seed => true,
                Origin::Continuation { parent } | Origin::Relay { parent, .. } => trunk[parent],
                Origin::Lateral { .. } => false,
            };
            if framed.0.z < -GROUND_TOLERANCE {
                failed = Some((i, below(structure, i, framed.0.z, -framed.0.z)));
                break;
            }
            let start = (framed, curve, inherited, (bends[i], reaches[i]), trunk[i]);
            starts.push((i, start));
        }
        let laid = lay_all(structure, species, levers, starts, threads);
        for (i, result) in laid {
            if failed.as_ref().is_some_and(|(f, _)| *f < i) {
                break;
            }
            match result {
                Ok(layer) => layers[i] = Some(layer),
                Err((base, height)) => {
                    failed = Some((i, below(structure, i, base, height)));
                    break;
                }
            }
        }
    }
    if let Some((_, error)) = failed {
        return Err(error);
    }
    Ok(layers.into_iter().map(|l| l.expect("laid")).collect())
}

/// The axes by depth from the seed, each depth in index order.
fn depths(structure: &Structure) -> Vec<Vec<usize>> {
    let mut depth = vec![0usize; structure.axes.len()];
    let mut out: Vec<Vec<usize>> = Vec::new();
    for (i, axis) in structure.axes.iter().enumerate() {
        depth[i] = axis.origin.parent().map_or(0, |p| depth[p] + 1);
        if out.len() <= depth[i] {
            out.push(Vec::new());
        }
        out[depth[i]].push(i);
    }
    out
}

fn below(structure: &Structure, i: usize, base: f64, height: f64) -> Error {
    let axis = &structure.axes[i];
    Error::BelowGround {
        axis: i,
        pa: axis.pa,
        birth: axis.birth,
        base,
        height,
    }
}

/// Lays the axes of one depth from their starts, in pieces on up to
/// `threads` cores: each axis's walk, or its base's height and how far
/// below the ground it went, in index order.
#[allow(clippy::type_complexity)]
fn lay_all(
    structure: &mut Structure,
    species: &Species,
    levers: Option<&[Vec<Lever>]>,
    starts: Vec<(usize, Start)>,
    threads: usize,
) -> Vec<(usize, std::result::Result<Layer, (f64, f64)>)> {
    let at: Vec<usize> = starts.iter().map(|(i, _)| *i).collect();
    let axes = picked(&mut structure.axes, at.iter().copied());
    let mut jobs = axes.into_iter().zip(starts);
    let pieces: Vec<Mutex<Vec<_>>> = (0..at.len().div_ceil(PIECE))
        .map(|_| Mutex::new(jobs.by_ref().take(PIECE).collect()))
        .collect();
    let done: Vec<Mutex<Vec<_>>> = (0..pieces.len()).map(|_| Mutex::new(Vec::new())).collect();
    spread(pieces.len(), threads, &|k| {
        let piece = std::mem::take(&mut *pieces[k].lock().unwrap());
        *done[k].lock().unwrap() = piece
            .into_iter()
            .map(|(axis, (i, (framed, curve, inherited, bend, trunk)))| {
                let state = &species.states[axis.pa];
                let load = levers.map(|t| t[i].as_slice());
                let base = framed.0.z;
                let layer = lay(axis, (framed, curve, inherited), state, bend, load, trunk);
                (i, layer.map_err(|height| (base, height)))
            })
            .collect();
    });
    done.into_iter()
        .flat_map(|d| d.into_inner().unwrap())
        .collect()
}
