//! fn-256 task 2 (DESIGN.md sections 2.3 and 7): the rough layout's
//! columns as the device holds them (`capture_grown.rs`): each PA's form
//! row and each pencil's words. Reals are the f32s the layout reads, held
//! exactly.
use super::super::sketch::Pencil;
use crate::geometry::{amount, Layer};
use crate::species::Species;

/// A PA's form row (the device's `layout.wgsl` holds the same words).
const FORM: usize = 20;
/// The pulls a pencil holds inline.
const LIFTS: usize = 8;

/// Each PA's form as the layout reads it, every angle an amount.
pub(super) fn form(species: &Species) -> Vec<f32> {
    let mut out = Vec::new();
    for s in &species.states {
        let fm = &s.form;
        out.extend([
            s.internode as f32,
            fm.tropism as f32,
            amount(fm.elevation),
            fm.wander as f32,
            fm.bend_length as f32,
            fm.internode_spread as f32,
            fm.tropism_spread as f32,
            amount(s.insertion),
            amount(fm.insertion_spread),
            amount(fm.roll),
            amount(fm.plane),
            amount(s.divergence),
            s.epitony as f32,
            s.straightening as f32,
            s.erection as f32,
            s.relay_at as f32,
            s.relay_at.clamp(0.0, 1.0) as f32,
            fm.dominance as f32,
        ]);
        out.push(s.leaf_area as f32);
        out.push(s.takeover as f32);
    }
    assert_eq!(out.len(), FORM * species.states.len(), "a form row");
    out
}

/// The largest f32 below the ground's tolerance, `-1e-9` (`Layer::step`).
pub(super) fn ground() -> f32 {
    let t = -1e-9f64;
    let c = t as f32;
    if f64::from(c) < t {
        c
    } else {
        f32::from_bits(c.to_bits() + 1)
    }
}

/// A real the layout holds as an f32.
fn f(x: f64) -> f32 {
    assert_eq!(f64::from(x as f32), x, "a layout value that is no f32");
    x as f32
}

fn bits(x: f64) -> u32 {
    f(x).to_bits()
}

/// A pencil's words: its walk, its pulls inline, then its scalars.
pub(super) fn pencil_row(p: &Pencil, fresh: bool) -> Vec<u32> {
    let mut row: Vec<u32> = walk(&p.layer);
    row.extend([
        bits(p.base_scale),
        bits(p.running),
        bits(p.rank),
        p.units as u32,
        p.laid as u32,
        u32::from(p.trunk),
        bits(p.wood),
    ]);
    let constants = [p.light, p.size, p.memory, p.kept];
    row.extend(constants.map(|x| if fresh { bits(x) } else { 0 }));
    row.extend(p.layer.take_words().map(f32::to_bits));
    row
}

/// A walk's words with its pulls' count and its pulls, `LIFTS` inline.
fn walk(layer: &Layer) -> Vec<u32> {
    let words = layer.words();
    let lifts = (words.len() - 15) / 4;
    assert!(lifts <= LIFTS, "a walk carries {lifts} pulls, past {LIFTS}");
    let mut row: Vec<u32> = words[..15].iter().map(|x| x.to_bits()).collect();
    row.push(lifts as u32);
    row.extend(words[15..].iter().map(|x| x.to_bits()));
    row.resize(16 + 4 * LIFTS, 0);
    row
}
