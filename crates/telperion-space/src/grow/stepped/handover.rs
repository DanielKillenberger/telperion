//! A fine root handed to its own grower at birth (PHASES.md's plan, step
//! 4). The coarse grower grows, frames and lays every axis made in a
//! cycle as the whole tree does; then each new lateral the cut classes
//! fine leaves it, with every axis made on it that cycle, its living
//! apexes and its sleeping buds, for an item of its own. The coarse
//! grower keeps a placeholder at the root's index (its origin, lineage
//! and PA, no wood) so its bearer's children keep their order; the
//! axes it keeps are renumbered past those it handed over.
use super::{Executor, Item};
use crate::grow::subtree::bare;
use crate::grow::{bud, Apex, Grower};
use crate::lineage::Key;
use crate::presence::Draws;
use crate::structure::{Axis, Origin};

/// Where an axis made this cycle goes: kept by the coarse grower at an
/// index, or an item's (its number among the new ones) at one.
#[derive(Clone, Copy)]
enum To {
    Coarse(usize),
    Root(usize, usize),
    Item(usize, usize),
}

/// Renames `origin`'s parent by `f`.
pub(super) fn reparent(origin: &mut Origin, f: impl Fn(usize) -> usize) {
    match origin {
        Origin::Seed => {}
        Origin::Lateral { parent, .. }
        | Origin::Continuation { parent }
        | Origin::Relay { parent, .. } => *parent = f(*parent),
    }
}

impl<'a> Executor<'a> {
    /// Hands every fine root the coarse grower made since axis
    /// `first_new` to a new item.
    pub(super) fn hand_over(&mut self, first_new: usize, next: u32) {
        let to = self.destinations(first_new);
        if to.iter().all(|t| matches!(t, To::Coarse(_))) {
            return;
        }
        let c = &mut self.coarse;
        let at = |i: usize| match i {
            i if i < first_new => i,
            i => match to[i - first_new] {
                To::Coarse(j) | To::Root(_, j) => j,
                To::Item(..) => unreachable!("the coarse grower holds nothing on a fine root"),
            },
        };
        let local = |i: usize| match to[i - first_new] {
            To::Root(k, _) => (k, 1),
            To::Item(k, j) => (k, j),
            To::Coarse(_) => unreachable!("a fine root holds nothing of the coarse grower"),
        };
        let in_item = |k: usize, i: usize| {
            let (owner, j) = local(i);
            assert_eq!(owner, k, "an item holds only its own root's axes");
            j
        };
        let tail = Tail::drained(c, first_new);
        let mut items: Vec<Grower<'a>> = Vec::new();
        for (n, (mut axis, rest)) in tail.into_iter().enumerate() {
            match to[n] {
                To::Coarse(_) => {
                    reparent(&mut axis.origin, at);
                    let successor = rest.successor.iter().map(|&s| at(s)).collect();
                    push(c, axis, Rest { successor, ..rest });
                }
                To::Root(k, _) => {
                    debug_assert_eq!(k, items.len());
                    let mut placeholder = Axis {
                        phytomers: Vec::new(),
                        units: Vec::new(),
                        ..axis.clone()
                    };
                    reparent(&mut placeholder.origin, at);
                    let held = Rest {
                        draws: Draws::default(),
                        units: 0.0,
                        successor: Vec::new(),
                        ..rest.clone()
                    };
                    push(c, placeholder, held);
                    items.push(self_item(c, &axis));
                    reparent(&mut axis.origin, |_| 0);
                    let successor = rest.successor.iter().map(|&s| in_item(k, s)).collect();
                    push(&mut items[k], axis, Rest { successor, ..rest });
                }
                To::Item(k, _) => {
                    reparent(&mut axis.origin, |p| in_item(k, p));
                    let successor = rest.successor.iter().map(|&s| in_item(k, s)).collect();
                    push(&mut items[k], axis, Rest { successor, ..rest });
                }
            }
        }
        // What refers to the axes made: the coarse grower's successors,
        // living apexes, sleeping buds and woken buds.
        for s in c.successor[..first_new].iter_mut().flatten() {
            *s = at(*s);
        }
        let live = std::mem::take(&mut c.live);
        for apex in live {
            match apex.axis {
                i if i < first_new => c.live.push(apex),
                i => match to[i - first_new] {
                    To::Coarse(j) => c.live.push(Apex { axis: j, ..apex }),
                    _ => {
                        let (k, j) = local(i);
                        items[k].live.push(Apex { axis: j, ..apex });
                    }
                },
            }
        }
        for (wakes, list) in c.asleep.iter_mut().enumerate() {
            let all = std::mem::take(list);
            for mut sleeper in all {
                match sleeper.parent {
                    i if i < first_new => list.push(sleeper),
                    i => match to[i - first_new] {
                        To::Coarse(j) => {
                            sleeper.parent = j;
                            list.push(sleeper);
                        }
                        _ => {
                            let (k, j) = local(i);
                            sleeper.parent = j;
                            items[k].asleep[wakes].push(sleeper);
                        }
                    },
                }
            }
        }
        for woken in &mut c.woken {
            woken.axis = at(woken.axis);
            woken.bearer = at(woken.bearer);
        }
        for (k, g) in items.into_iter().enumerate() {
            let root = (0..to.len())
                .find_map(|n| match to[n] {
                    To::Root(r, j) if r == k => Some(j),
                    _ => None,
                })
                .expect("its root");
            self.items.push(Item {
                at: root,
                g,
                next,
                died: None,
            });
        }
    }

    /// Where each axis made since `first_new` goes: a lateral the cut
    /// classes fine roots an item, and every axis on it stands in it.
    fn destinations(&self, first_new: usize) -> Vec<To> {
        let c = &self.coarse;
        let mut to: Vec<To> = Vec::with_capacity(c.axes.len() - first_new);
        let (mut next, mut items) = (first_new, Vec::<usize>::new());
        for axis in &c.axes[first_new..] {
            let inherited = match axis.origin.parent() {
                Some(p) if p >= first_new => match to[p - first_new] {
                    To::Root(k, _) | To::Item(k, _) => Some(k),
                    To::Coarse(_) => None,
                },
                _ => None,
            };
            let fine = matches!(axis.origin, Origin::Lateral { .. })
                && (self.counts)(axis.pa, axis.birth) <= self.cut;
            to.push(match (inherited, fine) {
                (Some(k), _) => {
                    items[k] += 1;
                    To::Item(k, items[k] - 1)
                }
                (None, true) => {
                    items.push(2);
                    next += 1;
                    To::Root(items.len() - 1, next - 1)
                }
                (None, false) => {
                    next += 1;
                    To::Coarse(next - 1)
                }
            });
        }
        to
    }
}

/// An axis's lines beside it in a grower.
#[derive(Clone)]
struct Rest {
    draws: Draws,
    units: f64,
    successor: Vec<usize>,
    root: u64,
    horizon: f64,
    pencil: Option<crate::grow::sketch::Pencil>,
}

/// The axes made since an index, drained from a grower with their lines.
struct Tail(Vec<(Axis, Rest)>);

impl Tail {
    fn drained(g: &mut Grower, from: usize) -> Self {
        let pencils: Vec<Option<_>> = match g.sketch.as_mut() {
            Some(s) => {
                assert_eq!(s.pencils.len(), g.axes.len(), "every axis laid");
                s.pencils.drain(from..).map(Some).collect()
            }
            None => vec![None; g.axes.len() - from],
        };
        let lines = g
            .axes
            .drain(from..)
            .zip(g.draws.drain(from..))
            .zip(g.units.drain(from..))
            .zip(g.successor.drain(from..))
            .zip(g.roots.drain(from..))
            .zip(g.horizon.drain(from..))
            .zip(pencils);
        let made = lines.map(
            |((((((axis, draws), units), successor), root), horizon), pencil)| {
                let rest = Rest {
                    draws,
                    units,
                    successor,
                    root,
                    horizon,
                    pencil,
                };
                (axis, rest)
            },
        );
        Self(made.collect())
    }

    fn into_iter(self) -> std::vec::IntoIter<(Axis, Rest)> {
        self.0.into_iter()
    }
}

/// Adds an axis and its lines to a grower.
fn push(g: &mut Grower, axis: Axis, rest: Rest) {
    g.push_axis(axis, rest.draws, (rest.units, rest.root, rest.horizon));
    *g.successor.last_mut().expect("pushed") = rest.successor;
    if let (Some(s), Some(p)) = (g.sketch.as_mut(), rest.pencil) {
        s.pencils.push(p);
    }
}

/// A new item's grower for `root`: the coarse grower's rules from no
/// phytomers grown, on one core, its stub standing for the bearer.
fn self_item<'a>(c: &Grower<'a>, root: &Axis) -> Grower<'a> {
    let mut rules = c.rules.clone();
    rules.grown = 0;
    let mut g = Grower::bare(rules, 1);
    g.asleep = vec![Vec::new(); c.asleep.len()];
    g.sketch = c.sketch.as_ref().map(|s| crate::grow::sketch::Sketch {
        light: s.light,
        pencils: vec![bare(0.0)],
    });
    let stub = bud(Key(0), root.pa, 0, Origin::Seed);
    g.push_axis(stub, Draws::default(), (0.0, 0, f64::NEG_INFINITY));
    g
}
