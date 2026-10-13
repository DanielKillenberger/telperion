//! A tree drawn within its last year (fn-270 R5, R5a): grown to
//! `committed` whole years and drawn a fraction `f` into the last.
//!
//! - Every organ present in both years moves from its drawing in the year
//!   before to its drawing in this one by the fraction: its place, frame,
//!   girth and scale, the sag in its place. An organ is known by its
//!   axis's identity (`fine::identities`) and its place on the axis.
//! - A subtree gone this year fades from its drawing to nothing across the
//!   fraction, its girth and scale by `1 - f`, carried with the wood it
//!   stood on.
//! - A shoot grown this year (a woken bud's among them) extends from its
//!   base along its PA's elongation course, each node unfolding as the
//!   shoot reaches it, and the leaf at that node expands along its PA's
//!   expansion course from then.
//!
//! The year runs `YEAR` days from bud burst. The whole year draws the tree
//! as grown; the presentation never feeds back into growth.
use crate::fine::identities;
use crate::lineage::internode_factor;
use crate::species::Species;
use crate::structure::{Axis, Origin, Phytomer, Structure, Vec3};
use crate::transcendental::Transcendental;
use std::collections::HashMap;

/// The days of a year, from bud burst to bud burst.
pub const YEAR: f64 = 365.0;
/// The logistic course's steepness over its span: 4% of the way at a
/// fifth of the span, 96% at four fifths.
const STEEP: f64 = 10.0;

/// The logistic course of growth over a span, `x` its share run: 0 at
/// the start and 1 at the end, exactly.
pub fn course(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let (lo, hi) = ends();
    (sigmoid(STEEP * (x - 0.5)) - lo) / (hi - lo)
}

/// The share of its span at which the course reaches `y`: `course`'s
/// inverse.
pub fn reached(y: f64) -> f64 {
    if y <= 0.0 {
        return 0.0;
    }
    if y >= 1.0 {
        return 1.0;
    }
    let (lo, hi) = ends();
    let z = lo + y * (hi - lo);
    (0.5 + (z / (1.0 - z)).ln_fixed() / STEEP).clamp(0.0, 1.0)
}

fn sigmoid(t: f64) -> f64 {
    1.0 / (1.0 + (-t).exp_fixed())
}

fn ends() -> (f64, f64) {
    (sigmoid(-STEEP / 2.0), sigmoid(STEEP / 2.0))
}

/// How far a course `days` long has run `t` days after it began: whole at
/// once where it takes no days.
fn run(t: f64, days: f64) -> f64 {
    match days > 0.0 {
        true => course(t / days),
        false => f64::from(u8::from(t > 0.0)),
    }
}

/// `a` moved a share `f` of the way to `b`: `a` at 0 and `b` at 1, to the
/// bit.
fn mix(a: f64, b: f64, f: f64) -> f64 {
    match f {
        f if f <= 0.0 => a,
        f if f >= 1.0 => b,
        f => a + (b - a) * f,
    }
}

fn mix3(a: Vec3, b: Vec3, f: f64) -> Vec3 {
    Vec3::new(mix(a.x, b.x, f), mix(a.y, b.y, f), mix(a.z, b.z, f))
}

/// A tree drawn within its year.
#[derive(Debug, Clone, PartialEq)]
pub struct Presented {
    pub structure: Structure,
    /// Each axis's leaves' share of their whole size, by phytomer, base to
    /// tip; empty where every leaf of the axis is whole.
    pub leaves: Vec<Vec<f32>>,
    /// How far into its last year the tree is drawn, in [0, 1]: what the
    /// leaves the wood's age keeps move by.
    pub fraction: f64,
}

impl Presented {
    /// A tree drawn at its whole year.
    pub fn whole(structure: Structure) -> Self {
        let leaves = vec![Vec::new(); structure.axes.len()];
        Self { structure, leaves, fraction: 1.0 }
    }
}

/// `now`, the tree grown to its age, drawn `f` into its last year, from
/// `before`, its drawing a year earlier (empty before the first year), by
/// `species`' courses (module doc).
pub fn present(now: Structure, before: &Structure, species: &Species, f: f64) -> Presented {
    if f >= 1.0 {
        return Presented::whole(now);
    }
    let ids = identities(&now);
    let was = index(before);
    let grown: Vec<Vec<Vec3>> = now.axes.iter().map(|a| a.phytomers.iter().map(|p| p.tip).collect()).collect();
    let bases = now.axes.iter().map(|a| a.base).collect();
    let mut p = Presenting { before, bases, grown, structure: now, leaves: Vec::new(), f };
    for i in 0..p.structure.axes.len() {
        let old = was.get(&ids[i]).map(|&j| &before.axes[j]);
        let leaves = p.axis(i, old, species);
        p.leaves.push(leaves);
    }
    p.fade();
    Presented { structure: p.structure, leaves: p.leaves, fraction: f }
}

/// The tree as it is drawn, axis by axis, parents first.
struct Presenting<'a> {
    before: &'a Structure,
    /// Each axis's base and its phytomers' tips as grown, base to tip.
    bases: Vec<Vec3>,
    grown: Vec<Vec<Vec3>>,
    structure: Structure,
    leaves: Vec<Vec<f32>>,
    f: f64,
}

impl Presenting<'_> {
    /// How far axis `i`'s phytomer `k`, or its last where it holds fewer,
    /// has moved from where it grew; its base where it holds none.
    fn shift(&self, i: usize, k: usize) -> Vec3 {
        let a = &self.structure.axes[i];
        match a.phytomers.len() {
            0 => a.base - self.bases[i],
            n => a.phytomers[k.min(n - 1)].tip - self.grown[i][k.min(n - 1)],
        }
    }

    /// How far the place axis `a` stands on has moved: the node of its
    /// bearer it stands at, its bearer's end where it carries one on.
    fn stand(&self, a: &Axis) -> Vec3 {
        match a.origin {
            Origin::Seed => Vec3::default(),
            Origin::Lateral { parent, node, .. } | Origin::Relay { parent, node } => self.shift(parent, node),
            Origin::Continuation { parent } => self.shift(parent, usize::MAX),
        }
    }

    /// Axis `i` drawn, `old` its drawing a year before: its leaves' shares.
    fn axis(&mut self, i: usize, old: Option<&Axis>, species: &Species) -> Vec<f32> {
        let f = self.f;
        let moved = self.stand(&self.structure.axes[i]);
        let a = &mut self.structure.axes[i];
        let state = &species.states[a.pa];
        let olds = old.map_or(0, |b| b.phytomers.len().min(a.phytomers.len()));
        match old {
            Some(b) => {
                a.base = mix3(b.base, a.base, f);
                a.heading = mix3(b.heading, a.heading, f);
                a.side = mix3(b.side, a.side, f);
                a.vigour = mix(b.vigour, a.vigour, f);
                a.blend = mix(b.blend, a.blend, f);
                if let (Some(j), Some(was)) = (a.joined.as_mut(), b.joined) {
                    if j.node == was.node {
                        j.t = mix(f64::from(was.t), f64::from(j.t), f) as f32;
                        j.below = mix(was.below, j.below, f);
                        j.above = mix(was.above, j.above, f);
                    }
                }
                for (p, q) in a.phytomers.iter_mut().zip(&b.phytomers) {
                    blend(p, q, f);
                }
            }
            None => a.base = a.base + moved,
        }
        // This year's shoot, its phytomers past the ones drawn a year
        // before, measured by the wood each grows (its scale and its
        // internode's spread, as the drawing measures it).
        let (grown, base) = (&self.grown[i], self.bases[i]);
        let (mut from, mut drawn) = match olds {
            0 => (base, a.base),
            k => (grown[k - 1], a.phytomers[k - 1].tip),
        };
        let wood = |p: &Phytomer| p.scale * internode_factor(p.key, state.form.internode_spread);
        let total: f64 = a.phytomers[olds..].iter().map(wood).sum();
        let tau = f * YEAR;
        let reach = run(tau, state.elongation) * total;
        let mut leaves = vec![1.0f32; olds];
        let mut along = 0.0;
        for k in olds..a.phytomers.len() {
            let p = &mut a.phytomers[k];
            let piece = wood(p);
            let r = match piece > 0.0 {
                true => ((reach - along) / piece).clamp(0.0, 1.0),
                false => f64::from(u8::from(reach >= along && tau > 0.0)),
            };
            let span = grown[k] - from;
            (p.tip, p.scale) = (drawn + span * r, p.scale * r);
            along += piece;
            let unfolds = state.elongation * reached(if total > 0.0 { along / total } else { 0.0 });
            leaves.push(run(tau - unfolds, state.expansion) as f32);
            (from, drawn) = (grown[k], p.tip);
        }
        if leaves.iter().all(|&l| l == 1.0) {
            leaves.clear();
        }
        leaves
    }

    /// Every axis drawn a year before that is gone, after the ones grown:
    /// faded by the fraction, carried with what it stood on. An axis that
    /// stays holds every phytomer it held (`unfold::tests`).
    fn fade(&mut self) {
        let (before, f) = (self.before, self.f);
        let fade = |p: &mut Phytomer, shift: Vec3| {
            (p.tip, p.radius, p.scale) = (p.tip + shift, mix(p.radius, 0.0, f), mix(p.scale, 0.0, f));
        };
        let (ids, now) = (identities(before), index(&self.structure));
        // Each axis of the year before: where it is drawn now, and how far
        // its wood has moved.
        let mut at: Vec<(usize, Vec3)> = Vec::with_capacity(before.axes.len());
        for (j, b) in before.axes.iter().enumerate() {
            if let Some(&i) = now.get(&ids[j]) {
                at.push((i, Vec3::default()));
                continue;
            }
            let mut axis = b.clone();
            let (parent, shift) = match b.origin.parent() {
                None => (None, Vec3::default()),
                Some(p) => {
                    let (i, carried) = at[p];
                    let stood = match now.contains_key(&ids[p]) {
                        true => self.moved_from(i, &before.axes[p], &b.origin),
                        false => carried,
                    };
                    (Some(i), stood)
                }
            };
            if let Some(i) = parent {
                axis.origin = reparented(b.origin, i);
            }
            axis.base = axis.base + shift;
            axis.phytomers.iter_mut().for_each(|p| fade(p, shift));
            at.push((self.structure.axes.len(), shift));
            self.structure.axes.push(axis);
            self.leaves.push(Vec::new());
        }
    }

    /// How far the place a gone axis of `origin` stood on its bearer,
    /// drawn as `b` a year before and as axis `i` now, has moved.
    fn moved_from(&self, i: usize, b: &Axis, origin: &Origin) -> Vec3 {
        let a = &self.structure.axes[i];
        let k = match *origin {
            Origin::Lateral { node, .. } | Origin::Relay { node, .. } => Some(node),
            Origin::Continuation { .. } => b.phytomers.len().checked_sub(1),
            Origin::Seed => None,
        };
        match k.filter(|&k| k < b.phytomers.len() && k < a.phytomers.len()) {
            Some(k) => a.phytomers[k].tip - b.phytomers[k].tip,
            None => a.base - b.base,
        }
    }
}

/// Phytomer `p` drawn a share `f` of the way from `q`, its drawing a year
/// before.
fn blend(p: &mut Phytomer, q: &Phytomer, f: f64) {
    p.tip = mix3(q.tip, p.tip, f);
    p.heading = mix3(q.heading, p.heading, f);
    p.side = mix3(q.side, p.side, f);
    p.radius = mix(q.radius, p.radius, f);
    p.scale = mix(q.scale, p.scale, f);
    p.grown = mix(q.grown, p.grown, f);
}

/// Each axis of `tree` by its identity.
fn index(tree: &Structure) -> HashMap<u64, usize> {
    identities(tree).into_iter().enumerate().map(|(i, id)| (id, i)).collect()
}

/// `origin` with its bearer at `parent`.
fn reparented(origin: Origin, parent: usize) -> Origin {
    match origin {
        Origin::Seed => Origin::Seed,
        Origin::Lateral { node, slot, whorl, woken, .. } => Origin::Lateral { parent, node, slot, whorl, woken },
        Origin::Continuation { .. } => Origin::Continuation { parent },
        Origin::Relay { node, .. } => Origin::Relay { parent, node },
    }
}

#[cfg(test)]
mod tests;
