//! A shed subtree retired as the tree grows (fn-255, host decision 2 of
//! 2026-10-11). Once a lateral's subtree closes (nothing in it lives or
//! sleeps), what the settle reads of it is final: its units, sleep and
//! vigour, its nodes' scales, its woken buds' kept shares. It is then
//! evaluated once by the settle's own code, `presence::assign_living` on a
//! copy of it and `shed::subtree_living` with it as root, and where its
//! fade at the tree's age is none the settle would drop it: its phytomers
//! and draws are let go, and its living stands for it in its bearer's fade
//! (`shed::shed_past`). Where any row keeps shed pipes or leaf girth, each
//! of its phytomers is kept as the four f32s the girth pass reads
//! (`girth::Grew`), with its axis's vigour: none where those rows are zero.
//! A kept subtree's living is remembered, so an ancestor's evaluation
//! stops there. A subtree holding a bud that woke waits until that bud's
//! kept share is final (`Woken::kept`): its bearer's next unit, grown in
//! the cycle after the waking, and whether the bearer stopped then, which
//! the balance of the cycle after that may still decide by shedding it
//! (`apex_end` one cycle back); so from three cycles after the waking,
//! once that balance has run. Its upkeep stays frozen in its bearer's balance
//! (`working.rs`), as the balance charges it today. The subtrees closed in
//! a cycle are evaluated on every core, each alone, and applied in order.
use super::{Grower, Working};
use crate::contract::presence::{q, real, times};
use crate::dormant::Woken;
use crate::girth::Grew;
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

/// A retired axis's vigour and phytomers as the girth pass reads them.
type Record = (f64, Box<[Grew]>);

/// Each axis's fate and, retired where rows keep it, its record; the
/// woken bud each axis is, where it is one.
#[derive(Default)]
pub(in crate::grow) struct Retirement {
    pub fate: Vec<Fate>,
    pub grew: Vec<Option<Record>>,
    woken: Vec<u32>,
    seen: usize,
    /// Closed subtrees waiting on a woken bud's kept share.
    pending: Vec<usize>,
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

    /// A retired axis's vigour and phytomers as the girth pass reads them.
    pub fn record(&self, i: usize) -> Option<(f64, &[Grew])> {
        self.grew.get(i)?.as_ref().map(|(v, g)| (*v, &g[..]))
    }
}

/// What the evaluation reads of the tree.
struct Tree<'t> {
    axes: &'t [Axis],
    draws: &'t [Draws],
    woken: &'t [Woken],
    successor: &'t [Vec<usize>],
    species: &'t Species,
    age: u32,
    keeps: bool,
    /// The cycle about to grow: what has grown is final before it.
    cycle: u32,
}

/// One closed subtree's verdict: not yet, kept, or retired with each
/// record.
enum Verdict {
    Wait,
    Kept(Living),
    Retired(Living, Vec<(usize, Record)>),
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
        // The closed subtrees beside an open axis, met at their roots.
        let beside = |c: usize| w.links[c].parent.is_none_or(|p| w.open[p]);
        let waiting = std::mem::take(&mut w.retire.pending);
        let mut todo: Vec<usize> = (w.closed.iter().copied().chain(waiting))
            .filter(|&c| beside(c) && w.retire.fate[c] == Fate::Open)
            .collect();
        todo.sort_unstable();
        todo.dedup();
        let tree = Tree {
            axes: &self.axes,
            draws: &self.draws,
            woken: &self.woken,
            successor: &self.successor,
            species: self.species,
            age: self.rules.age,
            keeps: crate::grow::keeps(self.species),
            cycle,
        };
        let wr = &w;
        let verdicts = crate::cores::map(todo.clone(), 16, self.threads, &|_, c| evaluate(&tree, wr, c));
        for (c, verdict) in todo.into_iter().zip(verdicts) {
            match verdict {
                Verdict::Wait => w.retire.pending.push(c),
                Verdict::Kept(living) => w.retire.fate[c] = Fate::Kept(living),
                Verdict::Retired(living, records) => {
                    for (i, record) in records {
                        w.retire.grew[i] = Some(record);
                    }
                    self.let_go(&mut w, c, living);
                }
            }
        }
        self.working = Some(w);
    }

    /// The new axes and woken buds taken in.
    fn take_in(&self, w: &mut Working) {
        let n = w.links.len();
        w.retire.fate.resize(n, Fate::Open);
        w.retire.grew.resize(n, None);
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

/// One closed subtree's living, and whether its fade at the tree's age is
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
    let lateral = matches!(axis.origin, Origin::Lateral { .. });
    if !(lateral && delay.is_finite() && faded(delay, axis.birth, living, t.age) <= 0.0) {
        return Verdict::Kept(living);
    }
    if !t.keeps {
        return Verdict::Retired(living, Vec::new());
    }
    // Every axis not yet recorded: a kept subtree's too, copied whole.
    let done = |i: usize| i != c && matches!(w.retire.fate[i], Fate::Root(_));
    let part = subtree(w, c, &done);
    let local = copied(t, w, &part, &done);
    let records = (part.iter().zip(&local))
        .filter(|(&i, _)| !done(i))
        .map(|(&i, a)| (i, (a.vigour, a.phytomers.iter().map(Grew::of).collect())))
        .collect();
    Verdict::Retired(living, records)
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
