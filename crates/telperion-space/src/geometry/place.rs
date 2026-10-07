//! Laying the tree (fn-210, lever 2): an axis is laid from its parent's
//! frame and nothing else, so the axes of one depth are laid at once, on
//! every core, after the depth that bears them. Where axes cannot be
//! laid, the error is the one of the first in index order, as laying them
//! in turn would give.
use super::lay::{lay, Layer, Lift, GROUND_TOLERANCE};
use super::{bend, frame, reaches};
use crate::cores::{map, picked, spread, PIECE};
use crate::error::{Error, Result};
use crate::sag::Lever;
use crate::species::Species;
use crate::structure::{Axis, Origin, Structure, Vec3};
use std::sync::Mutex;

/// What an axis is laid from: its frame, its bearer's wander, the pulls
/// it carries on, its bend and reach, and whether it is the trunk.
type Start = ((Vec3, Vec3, Vec3), Vec3, Vec<Lift>, (f64, f64), bool);

/// What each laid axis left: where its walk ended, its bend and whether
/// it is the trunk, and each axis's reach.
struct Laid {
    layers: Vec<Option<Layer>>,
    bends: Vec<f64>,
    trunk: Vec<bool>,
    reaches: Vec<f64>,
}

/// Lays every axis from its parent's frame; with `levers` (`sag.rs`),
/// each phytomer also turns down under the load it carries; on up to
/// `threads` cores. Where each axis's walk ended, by index.
pub(crate) fn place(
    structure: &mut Structure,
    species: &Species,
    levers: Option<&[Vec<Lever>]>,
    threads: usize,
) -> Result<Vec<Layer>> {
    let layers = place_from(structure, species, levers, 0, threads)?;
    Ok(layers.into_iter().map(|l| l.expect("laid")).collect())
}

/// `place`, the first `fixed` axes left as they stand, laid already
/// (fn-209 F1, realized mechanics: a fine root's bearer, whose laterals
/// are framed from its phytomers alone). Where each laid axis's walk
/// ended, by index; none for the fixed.
pub(crate) fn place_from(
    structure: &mut Structure,
    species: &Species,
    levers: Option<&[Vec<Lever>]>,
    fixed: usize,
    threads: usize,
) -> Result<Vec<Option<Layer>>> {
    let n = structure.axes.len();
    let mut laid = Laid {
        layers: vec![None; n],
        bends: vec![0.0; n],
        trunk: vec![false; n],
        reaches: reaches(&structure.axes),
    };
    // The first axis that cannot be laid, and why.
    let mut failed: Option<(usize, Error)> = None;
    for depth in depths(structure) {
        let todo: Vec<usize> = depth
            .into_iter()
            .filter(|&i| i >= fixed)
            .take_while(|&i| !failed.as_ref().is_some_and(|(f, _)| *f < i))
            .collect();
        let mut pieces: Vec<Vec<usize>> = todo.chunks(PIECE).map(<[usize]>::to_vec).collect();
        // Each axis's start from its parent's frame alone, on every core,
        // in pieces kept apart so no start is moved twice.
        let mut framed = map(pieces.iter().collect(), 1, threads, &|_, piece| {
            piece
                .iter()
                .map(|&i| start(structure, species, &laid, i))
                .collect::<Vec<Start>>()
        });
        // An axis whose base is below the ground ends the lay at it.
        'scan: for (k, (piece, starts)) in pieces.iter().zip(&framed).enumerate() {
            for (j, (&i, start)) in piece.iter().zip(starts).enumerate() {
                (laid.bends[i], laid.trunk[i]) = ((start.3).0, start.4);
                let base = (start.0).0.z;
                if base < -GROUND_TOLERANCE {
                    failed = Some((i, below(structure, i, base, -base)));
                    pieces.truncate(k + 1);
                    pieces[k].truncate(j);
                    framed.truncate(k + 1);
                    framed[k].truncate(j);
                    break 'scan;
                }
            }
        }
        let first = lay_all(
            structure,
            (species, levers),
            &mut laid.layers,
            (pieces, framed),
            threads,
        );
        if let Some((i, (base, height))) = first {
            failed = Some((i, below(structure, i, base, height)));
        }
    }
    if let Some((_, error)) = failed {
        return Err(error);
    }
    Ok(laid.layers)
}

/// Axis `i`'s start, from its parent's frame and walk.
fn start(structure: &Structure, species: &Species, laid: &Laid, i: usize) -> Start {
    let layer = |p: usize| laid.layers[p].as_ref().expect("laid");
    let framed = frame(&structure.axes, species, i, &|p| layer(p).ends());
    let axis = &structure.axes[i];
    // A continuation or relay carries its bearer's wander on.
    let curve = match axis.origin {
        Origin::Continuation { parent } | Origin::Relay { parent, .. } => layer(parent).curve(),
        _ => Vec3::default(),
    };
    let state = &species.states[axis.pa];
    let years = f64::from(structure.age.saturating_sub(axis.birth)) - axis.sleep;
    let bent = bend(axis, state, years);
    // A relay carries on the pulls of the axis it replaces, as far as it
    // is still that axis's continuation, so one that has barely left the
    // axis's line is laid as the axis would have gone on (fn-206).
    let inherited = match axis.carried_on() {
        Some((parent, b)) => layer(parent).handed((laid.bends[parent], laid.reaches[parent]), b),
        None => Vec::new(),
    };
    let trunk = match axis.origin {
        Origin::Seed => true,
        Origin::Continuation { parent } | Origin::Relay { parent, .. } => laid.trunk[parent],
        Origin::Lateral { .. } => false,
    };
    (framed, curve, inherited, (bent, laid.reaches[i]), trunk)
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

/// One piece of a depth's lay: its axes' indices and starts, the axes,
/// and where each walk's end goes.
type Piece<'s> = (
    Vec<usize>,
    Vec<Start>,
    Vec<&'s mut Axis>,
    Vec<&'s mut Option<Layer>>,
);

/// Lays the axes of one depth from their starts, piece by piece on up to
/// `threads` cores, each walk's end into `layers`: the first axis in
/// index order that cannot be laid, its base's height and how far below
/// the ground it went.
fn lay_all(
    structure: &mut Structure,
    (species, levers): (&Species, Option<&[Vec<Lever>]>),
    layers: &mut [Option<Layer>],
    (pieces, framed): (Vec<Vec<usize>>, Vec<Vec<Start>>),
    threads: usize,
) -> Option<(usize, (f64, f64))> {
    let at = || pieces.iter().flatten().copied();
    let (mut axes, mut ends) = (
        picked(&mut structure.axes, at()).into_iter(),
        picked(layers, at()).into_iter(),
    );
    let work: Vec<Mutex<Option<Piece>>> = pieces
        .iter()
        .zip(framed)
        .map(|(piece, starts)| {
            let n = piece.len();
            let mine = (
                axes.by_ref().take(n).collect(),
                ends.by_ref().take(n).collect(),
            );
            Mutex::new(Some((piece.clone(), starts, mine.0, mine.1)))
        })
        .collect();
    let done: Vec<Mutex<Option<(usize, (f64, f64))>>> =
        (0..work.len()).map(|_| Mutex::new(None)).collect();
    spread(work.len(), threads, &|k| {
        let (at, starts, axes, ends) = work[k].lock().unwrap().take().expect("a piece once");
        let jobs = at.into_iter().zip(starts).zip(axes.into_iter().zip(ends));
        for ((i, (framed, curve, inherited, bend, trunk)), (axis, end)) in jobs {
            let state = &species.states[axis.pa];
            let load = levers.map(|t| t[i].as_slice());
            let base = framed.0.z;
            match lay(axis, (framed, curve, inherited), state, bend, load, trunk) {
                Ok(layer) => *end = Some(layer),
                Err(height) => {
                    *done[k].lock().unwrap() = Some((i, (base, height)));
                    break;
                }
            }
        }
    });
    done.into_iter().find_map(|d| d.into_inner().unwrap())
}
