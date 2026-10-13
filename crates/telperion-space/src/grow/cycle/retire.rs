//! A shed subtree retired as the tree grows (fn-255, host decision 2 of
//! 2026-10-11). Once a lateral's subtree closes (nothing in it lives or
//! sleeps), what the settle reads of it is final: its units, sleep and
//! vigour, its nodes' scales, its woken buds' kept shares. It is then
//! evaluated once by the settle's own code, `presence::assign_living` on a
//! copy of it and `shed::subtree_living` with it as root, and where its
//! fade at the age grown is none the settle would drop it: its phytomers
//! and draws are let go, and its living stands for it in its bearer's fade
//! (`shed::shed_past`). Where any row keeps shed pipes or leaf girth, each
//! axis's girth is frozen as it closes (fn-270 R4, the host's ruling: dead
//! wood does not ripen once nothing in it lives): its base radius and its
//! two leaf-mean sums at its own base, carried up from its children's
//! (`girth::Frozen`); a retired root stands for its subtree by them.
//! A kept subtree's living is remembered, so an ancestor's evaluation
//! stops there. A subtree holding a bud that woke waits until that bud's
//! kept share is final (`Woken::kept`): its bearer's next unit, grown in
//! the cycle after the waking, and whether the bearer stopped then, which
//! the balance of the cycle after that may still decide by shedding it
//! (`apex_end` one cycle back); so from three cycles after the waking,
//! once that balance has run. Its upkeep stays frozen in its bearer's balance
//! (`working.rs`), as the balance charges it today. The subtrees closed in
//! a cycle are evaluated on every core, each alone, and applied in order.
//! The fade is read at the age grown so far, never at the tree's final age
//! (fn-270 R3), so a tree grows on from any age: a kept lateral that sheds
//! waits among the fading, its living final, and is evaluated again once
//! its fade there reaches none.
use super::{Grower, Working};
use crate::contract::presence::{q, real, times};
use crate::dormant::Woken;
use crate::girth::Frozen;
use crate::presence::{assign_living, Draws};
use crate::shed::{faded, subtree_living, Living};
use crate::species::Species;
use crate::structure::{Axis, Origin};
use std::borrow::Cow;

/// What retirement knows of each axis.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(in crate::grow) enum Fate {
    /// Not yet evaluated, or open.
    #[default]
    Open,
    /// Closed and kept: its subtree's living.
    Kept(Living),
    /// Retired: the root of a dropped subtree, and its living.
    Root(Living),
    /// Inside a retired subtree.
    Inside,
}

/// Each axis's fate and, closed where rows keep it, its frozen girth; the
/// woken bud each axis is, where it is one.
#[derive(Debug, Clone, Default)]
pub(in crate::grow) struct Retirement {
    pub fate: Vec<Fate>,
    pub frozen: Vec<Frozen>,
    woken: Vec<u32>,
    seen: usize,
    /// Closed subtrees waiting on a woken bud's kept share.
    pending: Vec<usize>,
    /// Kept laterals that shed, waiting for their fade to reach none at
    /// the age grown.
    fading: Vec<usize>,
    /// Every verdict in the order taken: the subtree's root, the cycle,
    /// waited (0), kept (1) or retired (2), and its living (none where it
    /// waited), for the device's retirement to be held to (fn-256).
    pub log: Vec<(usize, u32, u8, Living)>,
}

impl Retirement {
    /// Whether axis `i` was let go with a retired subtree.
    pub fn inside(&self, i: usize) -> bool {
        self.fate.get(i) == Some(&Fate::Inside)
    }

    /// The living a retired root stands for in its bearer's fade.
    pub fn root(&self, i: usize) -> Option<Living> {
        match self.fate.get(i) {
            Some(Fate::Root(l)) => Some(*l),
            _ => None,
        }
    }

    /// The subtrees retired.
    pub fn roots(&self) -> usize {
        self.fate.iter().filter(|f| matches!(f, Fate::Root(_))).count()
    }

    /// A retired root's frozen girth, which stands for its subtree's.
    pub fn frozen_root(&self, i: usize) -> Option<Frozen> {
        matches!(self.fate.get(i), Some(Fate::Root(_))).then(|| self.frozen[i])
    }
}

/// What the evaluation reads of the tree.
struct Tree<'t> {
    axes: &'t [Axis],
    draws: &'t [Draws],
    woken: &'t [Woken],
    successor: &'t [Vec<usize>],
    species: &'t Species,
    /// The age a fade is read at: the age grown so far.
    age: u32,
    /// The cycle about to grow: what has grown is final before it.
    cycle: u32,
}

/// One closed subtree's verdict: not yet, kept, or retired.
enum Verdict {
    Wait,
    Kept(Living),
    Retired(Living),
}

impl Grower<'_> {
    /// Every lateral whose subtree has closed since the last cycle,
    /// beside an open axis, evaluated and retired where the settle would
    /// drop it.
    pub(in crate::grow) fn retire(&mut self, cycle: u32) {
        let mut w = self.working.take().expect("a working set");
        self.take_in(&mut w);
        if w.keep_all {
            self.working = Some(w);
            return;
        }
        if crate::grow::keeps(self.species) {
            self.freeze(&mut w, cycle - 1);
        }
        // The closed subtrees beside an open axis, met at their roots.
        let beside = |c: usize| w.links[c].parent.is_none_or(|p| w.open[p]);
        let waiting = std::mem::take(&mut w.retire.pending);
        let mut todo: Vec<usize> = (w.closed.iter().copied().chain(waiting))
            .filter(|&c| beside(c) && w.retire.fate[c] == Fate::Open)
            .collect();
        // The age grown so far: what has grown is final before `cycle`.
        let age = cycle - 1;
        todo.extend(self.faded_out(&mut w, age));
        todo.sort_unstable();
        todo.dedup();
        let tree = Tree {
            axes: &self.axes,
            draws: &self.draws,
            woken: &self.woken,
            successor: &self.successor,
            species: self.species,
            age,
            cycle,
        };
        let wr = &w;
        let verdicts = crate::cores::map(todo.clone(), 16, self.threads, &|_, c| evaluate(&tree, wr, c));
        for (c, verdict) in todo.into_iter().zip(verdicts) {
            // Let go already with a fading ancestor that faded out with it.
            if w.retire.fate[c] == Fate::Inside {
                continue;
            }
            let logged = match &verdict {
                Verdict::Wait => (0, Living::default()),
                Verdict::Kept(l) => (1, *l),
                Verdict::Retired(l) => (2, *l),
            };
            w.retire.log.push((c, cycle, logged.0, logged.1));
            match verdict {
                Verdict::Wait => w.retire.pending.push(c),
                Verdict::Kept(living) => {
                    w.retire.fate[c] = Fate::Kept(living);
                    if sheds(&self.axes[c], self.species) {
                        w.retire.fading.push(c);
                    }
                }
                Verdict::Retired(living) => self.let_go(&mut w, c, living),
            }
        }
        self.working = Some(w);
    }

    /// The fading subtrees whose fade at `age` has reached none, taken
    /// out of the fading; those let go with an ancestor dropped.
    fn faded_out(&self, w: &mut Working, age: u32) -> Vec<usize> {
        let mut out = Vec::new();
        let fading = std::mem::take(&mut w.retire.fading);
        for c in fading {
            let Fate::Kept(living) = w.retire.fate[c] else {
                continue;
            };
            let axis = &self.axes[c];
            let delay = self.species.states[axis.pa].shedding;
            match faded(delay, axis.birth, living, age) <= 0.0 {
                true => out.push(c),
                false => w.retire.fading.push(c),
            }
        }
        out
    }

    /// The new axes and woken buds taken in.
    fn take_in(&self, w: &mut Working) {
        let n = w.links.len();
        w.retire.fate.resize(n, Fate::Open);
        w.retire.frozen.resize(n, Frozen::default());
        w.retire.woken.resize(self.axes.len(), u32::MAX);
        for k in w.retire.seen..self.woken.len() {
            let axis = self.woken[k].axis;
            if axis >= w.retire.woken.len() {
                w.retire.woken.resize(axis + 1, u32::MAX);
            }
            w.retire.woken[axis] = k as u32;
        }
        w.retire.seen = self.woken.len();
    }

    /// Each axis closed since the last cycle frozen at `age`, the age it
    /// closed at, deepest first: its children closed before it or with it.
    fn freeze(&self, w: &mut Working, age: u32) {
        let mut closed = w.closed.clone();
        closed.sort_unstable_by(|a, b| b.cmp(a));
        for i in closed {
            w.retire.frozen[i] = self.frozen_at(w, i, age);
        }
    }

    /// Axis `i`'s girth at its own base, closed at `age`: its phytomers'
    /// pipes, lit and not by the tree's mean, and each child's frozen
    /// girth by its base against `i`'s, in order (`girth::Frozen`).
    fn frozen_at(&self, w: &Working, i: usize, age: u32) -> Frozen {
        let state = &self.species.states[self.axes[i].pa];
        let (form, chi) = (&state.form, state.leaf_girth);
        let (axis, end) = crate::presence::assigned(&self.axes[i], &self.draws[i]);
        let mut f = Frozen::default();
        let mut section = 0.0f32;
        for p in &axis.phytomers {
            let scale = f64::from(p.scale as f32 * 1.0 * p.size as f32);
            let (pipe, term) = crate::girth::terms(crate::girth::own_at(form, p.cycle, age, scale, 1.0), p.light, chi);
            f.sums[0] += pipe;
            f.sums[1] += term;
            section += crate::girth::section(f64::from(term), form.exponent);
        }
        for &c in w.children(i) {
            let c = c as usize;
            let ratio = self.ratio(w, (c, &axis), (end, age));
            let child = w.retire.frozen[c];
            section += crate::girth::section(f64::from(ratio * child.radius), form.exponent);
            f.sums[0] += ratio * child.sums[0];
            f.sums[1] += ratio * child.sums[1];
        }
        f.radius = crate::girth::radius(section, form.exponent) as f32;
        f
    }

    /// Child `c`'s base against its bearer's (`girth::bases`), its bearer
    /// assigned and ending at `end`, at `age`: a lateral's node's scale,
    /// its vigour and its dominance share; a carried axis's vigour.
    fn ratio(&self, w: &Working, (c, bearer): (usize, &Axis), (end, age): (f64, u32)) -> f32 {
        use crate::contract::presence::{q, real, times};
        let a = &self.axes[c];
        let d = &self.draws[c];
        let mut birth = d.birth[1];
        if let Some(&k) = w.retire.woken.get(c).filter(|&&k| k != u32::MAX) {
            let kept = self.woken[k as usize].kept(&self.axes, &self.draws, &self.successor, age);
            birth = real(times(q(birth), q(kept)));
        }
        let made = times(q(d.birth[0]), q(birth));
        match a.origin {
            Origin::Lateral { node, .. } => {
                let scale = bearer.phytomers.get(node).map_or(0.0, |p| p.scale as f32);
                let share = crate::geometry::dominance(self.species.states[a.pa].form.dominance, a.lineage);
                scale * 1.0 * real(made) as f32 * share as f32
            }
            _ => real(times(q(end), made)) as f32 * 1.0,
        }
    }

    /// `c`'s subtree let go: its phytomers and draws, its root standing
    /// for it by `living`.
    fn let_go(&mut self, w: &mut Working, c: usize, living: Living) {
        w.retire.fate[c] = Fate::Root(living);
        let mut gone = vec![c];
        while let Some(i) = gone.pop() {
            if i != c {
                w.retire.fate[i] = Fate::Inside;
            }
            self.axes[i].phytomers = Vec::new();
            let d = &mut self.draws[i];
            (d.nodes, d.units, d.shade) = (Vec::new(), Vec::new(), Vec::new());
            gone.extend(w.children(i).iter().map(|&x| x as usize));
        }
    }
}

/// One closed subtree's living, and whether its fade at the age grown is
/// none; where it is and rows keep it, every axis's record not yet kept.
fn evaluate(t: &Tree, w: &Working, c: usize) -> Verdict {
    let stub = |i: usize| i != c && w.retire.fate[i] != Fate::Open;
    let part = subtree(w, c, &stub);
    let woke = |i: usize| w.retire.woken.get(i).copied().filter(|&k| k != u32::MAX);
    if part.iter().filter_map(|&i| woke(i)).any(|k| t.woken[k as usize].cycle + 3 > t.cycle) {
        return Verdict::Wait;
    }
    let local = copied(t, w, &part, &stub);
    let given = |k: usize| match w.retire.fate[part[k]] {
        Fate::Kept(l) | Fate::Root(l) if part[k] != c => Some(l),
        _ => None,
    };
    let living = subtree_living(&local, t.species, t.age, &given);
    let axis = &t.axes[c];
    let delay = t.species.states[axis.pa].shedding;
    if !(sheds(axis, t.species) && faded(delay, axis.birth, living, t.age) <= 0.0) {
        return Verdict::Kept(living);
    }
    Verdict::Retired(living)
}

/// Whether `axis` is a lateral that sheds.
fn sheds(axis: &Axis, species: &Species) -> bool {
    let lateral = matches!(axis.origin, Origin::Lateral { .. });
    lateral && species.states[axis.pa].shedding.is_finite()
}

/// The axes `part` (in order, its first the root) copied and assigned as
/// the settle assigns them, the root as a seed: each one `stub` names
/// without its phytomers, each woken bud by the share its bearer kept of it.
fn copied(t: &Tree, w: &Working, part: &[usize], stub: &dyn Fn(usize) -> bool) -> Vec<Axis> {
    let place = |i: usize| part.binary_search(&i).expect("in the subtree");
    let mut local: Vec<Axis> = (part.iter())
        .map(|&i| {
            let a = &t.axes[i];
            let origin = match a.origin {
                _ if i == part[0] => Origin::Seed,
                Origin::Seed => Origin::Seed,
                Origin::Lateral { parent, node, slot, whorl, woken } => {
                    Origin::Lateral { parent: place(parent), node, slot, whorl, woken }
                }
                Origin::Continuation { parent } => Origin::Continuation { parent: place(parent) },
                Origin::Relay { parent, node } => Origin::Relay { parent: place(parent), node },
            };
            let phytomers = if stub(i) { Vec::new() } else { a.phytomers.clone() };
            Axis {
                lineage: a.lineage,
                pa: a.pa,
                birth: a.birth,
                origin,
                vigour: a.vigour,
                blend: a.blend,
                apex_end: a.apex_end,
                base: a.base,
                heading: a.heading,
                side: a.side,
                phytomers,
                units: Vec::new(),
                alive: a.alive,
                rank: a.rank,
                sleep: a.sleep,
                phase: a.phase,
                joined: a.joined,
            }
        })
        .collect();
    let draws: Vec<Cow<Draws>> = (part.iter())
        .map(|&i| match w.retire.woken.get(i).copied().filter(|&k| k != u32::MAX) {
            Some(k) => {
                let kept = t.woken[k as usize].kept(t.axes, t.draws, t.successor, t.age);
                let mut d = t.draws[i].clone();
                d.birth[1] = real(times(q(d.birth[1]), q(kept)));
                Cow::Owned(d)
            }
            None => Cow::Borrowed(&t.draws[i]),
        })
        .collect();
    assign_living(&mut local, &draws, t.species, &|_, _| None, 1);
    local
}

/// `c` and every axis below it, in order, not below one `stops` names.
fn subtree(w: &Working, c: usize, stops: &dyn Fn(usize) -> bool) -> Vec<usize> {
    let mut part = vec![c];
    let mut k = 0;
    while k < part.len() {
        let i = part[k];
        if i == c || !stops(i) {
            part.extend(w.children(i).iter().map(|&x| x as usize));
        }
        k += 1;
    }
    part.sort_unstable();
    part
}
