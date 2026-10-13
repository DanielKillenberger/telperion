//! Where each axis a cycle made goes (PHASES.md's plan, step 4): the
//! coarse grower grows, frames and lays every axis made in a cycle as the
//! whole tree does; then each new lateral the cut classes fine leaves it
//! to stand (`stands.rs`), with every axis made on it that cycle. The
//! coarse grower keeps a placeholder at the root's index (its origin,
//! lineage and PA, no wood) so its bearer's children keep their order;
//! the axes it keeps are renumbered past those it handed over.
use super::Executor;
use crate::grow::Grower;
use crate::presence::Draws;
use crate::structure::{Axis, Origin};

/// Where an axis made this cycle goes: kept by the coarse grower at an
/// index, a fine root (its number among the new ones) standing at one,
/// or an axis on that root.
#[derive(Clone, Copy)]
pub(super) enum To {
    Coarse(usize),
    Root(usize, usize),
    Item(usize),
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
    /// Where each axis made since `first_new` goes: a lateral the cut
    /// classes fine roots a stand, and every axis on it stands in it.
    pub(super) fn destinations(&self, first_new: usize) -> Vec<To> {
        let c = &self.coarse;
        let mut to: Vec<To> = Vec::with_capacity(c.axes.len() - first_new);
        let (mut next, mut roots) = (first_new, 0);
        for axis in &c.axes[first_new..] {
            let inherited = match axis.origin.parent() {
                Some(p) if p >= first_new => match to[p - first_new] {
                    To::Root(k, _) | To::Item(k) => Some(k),
                    To::Coarse(_) => None,
                },
                _ => None,
            };
            let fine = matches!(axis.origin, Origin::Lateral { .. })
                && (self.counts)(axis.pa, axis.birth) <= self.cut;
            to.push(match (inherited, fine) {
                (Some(k), _) => To::Item(k),
                (None, true) => {
                    (roots, next) = (roots + 1, next + 1);
                    To::Root(roots - 1, next - 1)
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
pub(super) struct Rest {
    pub draws: Draws,
    pub units: f64,
    pub successor: Vec<usize>,
    pub root: u64,
    pub horizon: f64,
    pub pencil: Option<crate::grow::sketch::Pencil>,
}

/// The axes made since an index, drained from a grower with their lines.
pub(super) struct Tail(Vec<(Axis, Rest)>);

impl Tail {
    pub(super) fn drained(g: &mut Grower, from: usize) -> Self {
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

    pub(super) fn into_iter(self) -> std::vec::IntoIter<(Axis, Rest)> {
        self.0.into_iter()
    }
}

/// Adds an axis and its lines to a grower.
pub(super) fn push(g: &mut Grower, axis: Axis, rest: Rest) {
    g.push_axis(axis, rest.draws, (rest.units, rest.root, rest.horizon));
    *g.successor.last_mut().expect("pushed") = rest.successor;
    if let (Some(s), Some(p)) = (g.sketch.as_mut(), rest.pencil) {
        s.pencils.push(p);
    }
}
