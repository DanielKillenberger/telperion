//! A tree drawn within its last year (fn-270 R5, R5a): grown to
//! `committed` whole years and drawn a fraction `f` into the last.
//!
//! - Every organ present in both years moves from its drawing in the year
//!   before to its drawing in this one: its place, frame, girth and
//!   scale, the sag in its place. It holds last year's drawing until
//!   budburst, moves along its PA's elongation course with the shoots and
//!   holds this year's from there, so summer, autumn and winter stand
//!   still but for the leaves. An organ is known by its axis's identity
//!   (`fine::identities`) and its place on the axis.
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

/// `course`'s integral from 0 to `x`, its share run: a half at the end
/// of the span (fn-275 R2a, a stem's rise waited).
pub fn integral(x: f64) -> f64 {
    let x = x.clamp(0.0, 1.0);
    let (lo, hi) = ends();
    let soft = |t: f64| (1.0 + (STEEP * (t - 0.5)).exp_fixed()).ln_fixed() / STEEP;
    (soft(x) - soft(0.0) - lo * x) / (hi - lo)
}

fn sigmoid(t: f64) -> f64 {
    1.0 / (1.0 + (-t).exp_fixed())
}

fn ends() -> (f64, f64) {
    (sigmoid(-STEEP / 2.0), sigmoid(STEEP / 2.0))
}

/// How far a course `days` long has run `t` days after it began: whole at
/// once where it takes no days.
pub fn run(t: f64, days: f64) -> f64 {
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

/// A PA's year on the presentation's clock (fn-270 R5c), which runs from
/// one whole age, at full leaf, to the next: its leaves past their
/// lifespan colour and fall in autumn, and at budburst the year's shoots
/// and every leaf of the year grow from nothing, to full leaf `YEAR`
/// days on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Season {
    /// Days from the whole age to budburst, to the start of colouring,
    /// and the days of colouring, of falling, of a shoot's elongation and
    /// of a leaf's expansion.
    pub budburst: f64,
    pub senescence: f64,
    pub colouring: f64,
    pub fall: f64,
    pub elongation: f64,
    pub expansion: f64,
    /// The years a leaf lives.
    pub life: f64,
}

impl Season {
    /// `state`'s year: a whole age stands at full leaf, its budburst plus
    /// its shoot's elongation and its leaf's expansion, which places the
    /// calendar's days on the clock. A colouring and fall that would run
    /// past the next budburst are compressed, by degree, to end at it
    /// (host, fn-270 R5c): a walk between calendars that disagree stays a
    /// tree, every falling leaf gone before the year's leaves grow.
    pub fn of(state: &crate::species::PaState) -> Self {
        let grow = state.elongation + state.expansion;
        let budburst = YEAR - grow;
        let senescence = (state.senescence - state.budburst - grow).rem_euclid(YEAR).min(budburst);
        let (span, room) = (state.colouring + state.leaf_fall, budburst - senescence);
        let k = if span > room { room / span } else { 1.0 };
        Self {
            budburst,
            senescence,
            colouring: state.colouring * k,
            fall: state.leaf_fall * k,
            elongation: state.elongation,
            expansion: state.expansion,
            life: state.leaf_life,
        }
    }

    /// Days since budburst `tau` days into the year: what a shoot and its
    /// leaves grow by.
    pub fn grown(&self, tau: f64) -> f64 {
        tau - self.budburst
    }

    /// The share of its falling leaves gone `tau` days into the year: none
    /// until they have coloured, then all over the fall, each leaf on its
    /// own abscission day (`tree::Unfolded::leaf`).
    pub fn fallen(&self, tau: f64) -> f64 {
        run(tau - self.senescence - self.colouring, self.fall)
    }

    /// How far a falling leaf has coloured.
    pub fn colour(&self, tau: f64) -> f64 {
        run(tau - self.senescence, self.colouring)
    }

    /// The share of its size a leaf growing again from its bud stands at.
    pub fn regrow(&self, tau: f64) -> f64 {
        run(self.grown(tau), self.expansion)
    }

    /// The share of the leaves on wood `age` years old at the year's
    /// start that pass their lifespan this year: the wood's age once its
    /// leaves have fallen (its full leaf a whole age after budburst, its
    /// fall this year's) against the lifespan, by degree over a year, so
    /// a lifespan under a year drops every leaf and a longer one its
    /// oldest cohort; none where leaves live on.
    pub fn passing(&self, age: f64) -> f64 {
        let fallen = age + (YEAR - self.budburst + self.senescence + self.colouring + self.fall) / YEAR;
        (fallen + 1.0 - self.life).clamp(0.0, 1.0)
    }
}

/// How far a tree of `species` drawn `f` into its year has coloured its
/// falling leaves (fn-270 R5c), which the renderer turns every withered
/// leaf by: the furthest of its PAs whose leaves fall, before budburst;
/// whole where none falls, as the dead fronds a rosette keeps are, and at
/// the whole year.
pub fn colouring(species: &Species, f: f64) -> f64 {
    let tau = f * YEAR;
    let falling = species.states.iter().map(Season::of).filter(|s| s.life.is_finite());
    let coloured = falling.map(|s| if tau < s.budburst { s.colour(tau) } else { 0.0 });
    match f < 1.0 {
        true => coloured.fold(None, |m: Option<f64>, c| Some(m.map_or(c, |m| m.max(c)))).unwrap_or(1.0),
        false => 1.0,
    }
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
    /// The first of the axes gone this year, which follow the ones grown.
    pub gone: usize,
}

impl Presented {
    /// A tree drawn at its whole year.
    pub fn whole(structure: Structure) -> Self {
        let leaves = vec![Vec::new(); structure.axes.len()];
        let gone = structure.axes.len();
        Self { structure, leaves, fraction: 1.0, gone }
    }
}

/// `now`, the tree grown to its age, drawn `f` into its last year, from
/// `before`, its drawing a year earlier (empty before the first year), by
/// `species`' courses (module doc).
/// `finest` is the thinnest wood the drawing draws: a gone subtree's
/// leaves reach nothing as its fading wood reaches it.
pub fn present(now: Structure, before: &Structure, species: &Species, (f, finest): (f64, f64)) -> Presented {
    if f >= 1.0 {
        return Presented::whole(now);
    }
    let ids = identities(&now);
    let was = index(before);
    let grown: Vec<Vec<Vec3>> = now.axes.iter().map(|a| a.phytomers.iter().map(|p| p.tip).collect()).collect();
    let bases = now.axes.iter().map(|a| a.base).collect();
    let gone = now.axes.len();
    let moving = Vec::with_capacity(gone);
    let mut p = Presenting { before, bases, grown, structure: now, leaves: Vec::new(), moving, f, finest };
    for i in 0..p.structure.axes.len() {
        let old = was.get(&ids[i]).map(|&j| &before.axes[j]);
        let leaves = p.axis(i, old, species);
        p.leaves.push(leaves);
    }
    p.fade();
    Presented { structure: p.structure, leaves: p.leaves, fraction: f, gone }
}

/// The tree as it is drawn, axis by axis, parents first.
struct Presenting<'a> {
    before: &'a Structure,
    /// Each axis's base and its phytomers' tips as grown, base to tip.
    bases: Vec<Vec3>,
    grown: Vec<Vec<Vec3>>,
    structure: Structure,
    leaves: Vec<Vec<f32>>,
    /// How far each axis drawn so far has moved from its drawing a year
    /// before (`Presenting::moving`).
    moving: Vec<f64>,
    f: f64,
    finest: f64,
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

    /// How far an axis of `state` has moved from its drawing a year
    /// before to this year's: not at all until budburst, then along its
    /// shoot's elongation, and held from there, so the year's movement
    /// all comes with the shoots (fn-270 R5c).
    fn moving(&self, state: &crate::species::PaState) -> f64 {
        run(Season::of(state).grown(self.f * YEAR), state.elongation)
    }

    /// Axis `i` drawn, `old` its drawing a year before: its leaves' shares.
    fn axis(&mut self, i: usize, old: Option<&Axis>, species: &Species) -> Vec<f32> {
        let f = self.moving(&species.states[self.structure.axes[i].pa]);
        // Its base moves with the wood it stands on.
        let carried = self.structure.axes[i].origin.parent().map_or(f, |p| self.moving[p]);
        self.moving.push(f);
        let moved = self.stand(&self.structure.axes[i]);
        let bearer = self.bearer_radius(&self.structure.axes[i]);
        let a = &mut self.structure.axes[i];
        let state = &species.states[a.pa];
        // A short shoot by its draw (fn-277).
        let clustered = crate::lineage::cluster(state.cluster_leaves, a.lineage) > 0;
        let olds = old.map_or(0, |b| b.phytomers.len().min(a.phytomers.len()));
        match old {
            Some(b) => {
                a.base = mix3(b.base, a.base, carried);
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
            None => {
                a.base = a.base + moved;
                // A relay born this year joins from its bearer's own wood.
                if let (Some(j), Some(r)) = (a.joined.as_mut(), bearer) {
                    (j.below, j.above) = (mix(r, j.below, f), mix(r, j.above, f));
                }
            }
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
        let tau = Season::of(state).grown(self.f * YEAR);
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
            // A short shoot's cluster stays as it was where the shoot
            // stood a year before, at whichever tip it stands now.
            let cluster = clustered && olds > 0;
            leaves.push(if cluster { 1.0 } else { run(tau - unfolds, state.expansion) as f32 });
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
        let (before, f, finest) = (self.before, self.f, self.finest);
        // Its wood thins to nothing and its leaves with it, each reaching
        // nothing as its wood reaches the thinnest drawn.
        let fade = |p: &mut Phytomer, shift: Vec3| -> f32 {
            let r = p.radius;
            (p.tip, p.radius) = (p.tip + shift, mix(r, 0.0, f));
            match r > finest {
                true => ((p.radius - finest) / (r - finest)).clamp(0.0, 1.0) as f32,
                false => mix(1.0, 0.0, f) as f32,
            }
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
            let leaves = axis.phytomers.iter_mut().map(|p| fade(p, shift)).collect();
            // A gone relay's join fades into its bearer's own wood.
            let bearer = self.bearer_radius(&axis);
            if let (Some(j), Some(r)) = (axis.joined.as_mut(), bearer) {
                (j.below, j.above) = (mix(j.below, r, f), mix(j.above, r, f));
            }
            at.push((self.structure.axes.len(), shift));
            self.structure.axes.push(axis);
            self.leaves.push(leaves);
        }
    }

    /// The radius of the phytomer a relay `a` joins on its bearer, as
    /// drawn; none for any other axis.
    fn bearer_radius(&self, a: &Axis) -> Option<f64> {
        let (Origin::Relay { parent, .. }, Some(j)) = (a.origin, a.joined) else {
            return None;
        };
        self.structure.axes[parent].phytomers.get(j.node).map(|p| p.radius)
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
