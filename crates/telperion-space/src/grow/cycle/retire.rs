//! A shed subtree retired as the tree grows (fn-255, host decision 2 of
//! 2026-10-11). Once a lateral's subtree closes (nothing in it lives or
//! sleeps), what the settle reads of it is final: its units, sleep and
//! vigour, its nodes' scales, its woken buds' kept shares. It is then
//! evaluated once by the settle's own code, `presence::assign_living` on a
//! copy of it and `shed::subtree_living` with it as root, and where its
//! fade at the age grown is none the settle would drop it: its phytomers
//! and draws are let go, and its living stands for it in its bearer's fade
//! (`shed::shed_past`). A kept subtree's living is remembered, so an ancestor's evaluation
//! stops there. A subtree holding a bud that woke waits until that bud's
//! kept share is final (`Woken::kept`): its bearer's next unit, grown in
//! the cycle after the waking, and whether the bearer stopped then, which
//! the balance of the cycle after that may still decide by shedding it
//! (`apex_end` one cycle back); so from three cycles after the waking,
//! once that balance has run. Being dead wood, it charges its bearer's
//! balance nothing (`working.rs`). The subtrees closed in
//! a cycle are evaluated on every core, each alone, and applied in order.
//! The fade is read at the age grown so far, never at the tree's final age
//! (fn-270 R3), so a tree grows on from any age: a kept lateral that sheds
//! waits among the fading, its living final, and is retired at that living
//! once its fade there reaches none (fn-368).
//!
//! A retired root is read only through its bearer: its carry into the
//! bearer's living, at every settle and every later evaluation of a subtree
//! holding it, as long as that bearer is held (fn-357, READING.md). Once
//! records are let go (`compact.rs`) it keeps only that, a `Stand`.
use super::{Grower, Working};
use crate::contract::presence::{q, real, times};
use crate::dormant::Woken;
use crate::presence::{assign_living, Draws};
use crate::shed::{faded, subtree_living, Living, Stand};
use crate::species::Species;
use crate::structure::{Axis, Growing, Origin};
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

/// Each axis's fate; the woken bud each axis is, where it is one.
#[derive(Debug, Clone, Default)]
pub(in crate::grow) struct Retirement {
    pub fate: Vec<Fate>,
    pub(super) woken: Vec<u32>,
    pub(super) seen: usize,
    /// Closed subtrees waiting on a woken bud's kept share.
    pub(super) pending: Vec<usize>,
    /// Kept laterals that shed, waiting for their fade to reach none at
    /// the age grown.
    pub(super) fading: Vec<usize>,
    /// Every verdict in the order taken: the subtree's root, the cycle,
    /// waited (0), kept (1) or retired (2), and its living (none where it
    /// waited) (fn-256); kept
    /// only where the records are (`Working::keep_records`), for its
    /// roots are indices.
    pub log: Vec<(usize, u32, u8, Living)>,
    /// The axes let go with a retired subtree, its root included, whose
    /// records are still held (`compact.rs`).
    pub(super) loose: usize,
    /// The retired roots whose records are let go, each by its bearer, in
    /// the bearers' order (fn-357).
    pub(super) stands: Vec<Stand>,
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

    /// The subtrees retired: the roots held, and those let go whose
    /// bearer has not been retired since.
    pub fn roots(&self) -> usize {
        let held = |f: &Fate| matches!(f, Fate::Root(_));
        let bearing =
            |s: &&Stand| matches!(self.fate[s.bearer as usize], Fate::Open | Fate::Kept(_));
        self.fate.iter().filter(|f| held(f)).count() + self.stands.iter().filter(bearing).count()
    }

    /// The retired roots let go that axis `i` bears.
    pub(super) fn stands_of(&self, i: usize) -> &[Stand] {
        let from = self.stands.partition_point(|s| (s.bearer as usize) < i);
        let to = self.stands.partition_point(|s| (s.bearer as usize) <= i);
        &self.stands[from..to]
    }

    /// The retired roots let go whose bearer is neither retired nor inside
    /// a retired subtree: those the settle carries up.
    pub(in crate::grow) fn standing(&self) -> Vec<Stand> {
        let bearing =
            |s: &&Stand| matches!(self.fate[s.bearer as usize], Fate::Open | Fate::Kept(_));
        self.stands.iter().filter(bearing).copied().collect()
    }
}

/// What the evaluation reads of the tree.
struct Tree<'t> {
    axes: &'t [Axis<Growing>],
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

impl Grower {
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
            species: &self.species,
            age,
            cycle,
        };
        let wr = &w;
        // A kept subtree that faded out is retired at the living it was
        // kept at, which is final; only a closed one is evaluated.
        let verdicts = crate::cores::map(todo.clone(), 16, self.threads, &|_, c| {
            match wr.retire.fate[c] {
                Fate::Kept(living) => Verdict::Retired(living),
                _ => evaluate(&tree, wr, c),
            }
        });
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
            if w.keep_records {
                w.retire.log.push((c, cycle, logged.0, logged.1));
            }
            match verdict {
                Verdict::Wait => w.retire.pending.push(c),
                Verdict::Kept(living) => {
                    w.retire.fate[c] = Fate::Kept(living);
                    if sheds(&self.axes[c], &self.species) {
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
        w.retire.loose += 1;
        let mut gone = vec![c];
        while let Some(i) = gone.pop() {
            if i != c && w.retire.fate[i] != Fate::Inside {
                // A root retired before is counted loose already.
                if !matches!(w.retire.fate[i], Fate::Root(_)) {
                    w.retire.loose += 1;
                }
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
    if part
        .iter()
        .filter_map(|&i| woke(i))
        .any(|k| t.woken[k as usize].cycle + 3 > t.cycle)
    {
        return Verdict::Wait;
    }
    let local = copied(t, w, &part, &stub);
    let given = |k: usize| match w.retire.fate[part[k]] {
        Fate::Kept(l) | Fate::Root(l) if part[k] != c => Some(l),
        _ => None,
    };
    // The roots let go on each axis it descends through, by local index.
    let stands: Vec<Stand> = (part.iter().enumerate())
        .filter(|&(_, &i)| i == c || !stub(i))
        .flat_map(|(k, &i)| {
            (w.retire.stands_of(i).iter()).map(move |s| Stand {
                bearer: k as u32,
                ..*s
            })
        })
        .collect();
    let living = subtree_living(&local, t.species, t.age, (&given, &stands));
    let axis = &t.axes[c];
    let delay = t.species.states[axis.pa].shedding;
    if !(sheds(axis, t.species) && faded(delay, axis.birth, living, t.age) <= 0.0) {
        return Verdict::Kept(living);
    }
    Verdict::Retired(living)
}

/// Whether `axis` is a lateral that sheds.
fn sheds<P>(axis: &Axis<P>, species: &Species) -> bool {
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
                Origin::Lateral {
                    parent,
                    node,
                    slot,
                    whorl,
                    woken,
                } => Origin::Lateral {
                    parent: place(parent),
                    node,
                    slot,
                    whorl,
                    woken,
                },
                Origin::Continuation { parent } => Origin::Continuation {
                    parent: place(parent),
                },
                Origin::Relay { parent, node } => Origin::Relay {
                    parent: place(parent),
                    node,
                },
            };
            let phytomers = if stub(i) {
                Vec::new()
            } else {
                a.phytomers.iter().map(Growing::settled).collect()
            };
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
                joined: a.joined.clone(),
            }
        })
        .collect();
    let draws: Vec<Cow<Draws>> = (part.iter())
        .map(
            |&i| match w.retire.woken.get(i).copied().filter(|&k| k != u32::MAX) {
                Some(k) => {
                    let kept = t.woken[k as usize].kept(t.axes, t.draws, t.successor, t.age);
                    let mut d = t.draws[i].clone();
                    d.birth[1] = real(times(q(d.birth[1]), q(kept)));
                    Cow::Owned(d)
                }
                None => Cow::Borrowed(&t.draws[i]),
            },
        )
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
