//! Trees resolved chain by chain against committed boundaries (fn-209,
//! option 4: Astra's "boundary-constrained chain geometry").
//!
//! A chain is a seed or lateral axis and every continuation and relay
//! that carries it on. Every lateral bud a chain makes exposes one
//! committed boundary (`boundary.rs`): the closed form's expectation of
//! what its subtree keeps at the tree's age, by its PA and birth, times
//! powers of its base scale, which its bearer's scale, its vigour as made
//! and its lineage's dominance draw set. The boundary is a value of the
//! bud alone, so it is the same at every zoom and never changes once a
//! bearer has read it.
//!
//! A chain is resolved from its own wood, its parent's laid frame and the
//! boundaries of its own bud and of the buds it bears, and nothing else:
//! its girth by the pipe model over its own phytomers and its borne buds'
//! committed sections, fitted to its own committed base (`girth.rs`); its
//! sag under its own wood and its borne buds' committed mass and moment
//! (`work.rs`). No chain reads a descendant's resolved state, so every
//! order of resolution gives the same tree, and the one-pass build is
//! resolving every chain.
//!
//! Not built here (the competition's cut): a chain's topology is not
//! grown alone. The grower grows the whole tree and sheds it as today,
//! and the chains are cut from what it grew; only girth and lay are
//! resolved per chain.
mod girth;
#[cfg(test)]
mod tests;
mod work;

use crate::closed_form::boundary::Rows;
use crate::error::Result;
use crate::geometry::dominance;
use crate::grow::{run, Options, Record, Request};
use crate::species::Species;
use crate::structure::{Origin, Structure, Vec3};
use std::sync::Arc;
use work::Done;

/// A species' committed boundaries per unit base scale at one age: read
/// once, shared by every tree of that species and age.
pub struct Expected {
    rows: Arc<Rows>,
    age: u32,
    seconds: f64,
}

impl Expected {
    /// The boundaries of `species` at `age`, read on up to `threads`
    /// cores (0: every core).
    pub fn new(species: &Species, age: u32, threads: usize) -> Self {
        let threads = if threads == 0 {
            crate::cores::available()
        } else {
            threads
        };
        let started = std::time::Instant::now();
        let rows = Arc::new(Rows::new(species, age, threads));
        Self {
            rows,
            age,
            seconds: started.elapsed().as_secs_f64(),
        }
    }

    pub(crate) fn rows(&self) -> Arc<Rows> {
        self.rows.clone()
    }

    /// The seconds reading them took, and the rows held.
    pub fn cost(&self) -> (f64, usize) {
        (self.seconds, self.rows.len())
    }
}

/// A piece of one order's chains, resolved.
type Worked = Vec<(usize, Result<Done>)>;

/// A tree whose chains resolve on demand.
pub struct Bounded<'s> {
    species: &'s Species,
    rows: Arc<Rows>,
    leaf_mean: Option<f64>,
    /// The tree before it bends, and as laid; unresolved axes unlaid.
    unbent: Structure,
    bent: Structure,
    records: Vec<Record>,
    chains: Vec<Chain>,
    chain_of: Vec<usize>,
    resolved: Vec<bool>,
    timings: (f64, f64),
}

/// One chain: its axes in index order, the chain that bears it, its bud
/// record (none: the seed's), its order and the records it bears.
struct Chain {
    axes: Vec<usize>,
    parent: Option<usize>,
    record: Option<usize>,
    order: usize,
    borne: Vec<usize>,
}

impl<'s> Bounded<'s> {
    /// Grows and settles the tree, and reads its closed form's rows.
    pub fn new(species: &'s Species, request: Request) -> Result<Self> {
        Self::with(species, request, &Expected::new(species, request.age, 0))
    }

    /// Grows and settles the tree, its boundaries read from `expected`,
    /// which must be the species' at the tree's age.
    pub fn with(species: &'s Species, request: Request, expected: &Expected) -> Result<Self> {
        assert_eq!(
            expected.age, request.age,
            "the boundaries of the tree's age"
        );
        let options = Options {
            settle: true,
            ..Options::default()
        };
        let started = std::time::Instant::now();
        let grown = run(species, request, &mut |_| {}, true, options)?;
        let rows = expected.rows.clone();
        let mut tree = Self::settled(
            species,
            grown.structure,
            grown.records,
            (rows, grown.leaf_mean),
        );
        tree.timings = (started.elapsed().as_secs_f64(), expected.seconds);
        Ok(tree)
    }

    pub(crate) fn settled(
        species: &'s Species,
        structure: Structure,
        records: Vec<Record>,
        (rows, leaf_mean): (Arc<Rows>, Option<f64>),
    ) -> Self {
        let n = structure.axes.len();
        let mut chain_of = vec![usize::MAX; n];
        let mut chains: Vec<Chain> = Vec::new();
        let mut by_axis = vec![None; n];
        for (r, record) in records.iter().enumerate() {
            if let Some(i) = record.axis {
                by_axis[i] = Some(r);
            }
        }
        for (i, axis) in structure.axes.iter().enumerate() {
            let c = match axis.origin {
                Origin::Continuation { parent } | Origin::Relay { parent, .. } => chain_of[parent],
                Origin::Seed | Origin::Lateral { .. } => {
                    let parent = axis.origin.parent().map(|p| chain_of[p]);
                    chains.push(Chain {
                        axes: Vec::new(),
                        parent,
                        record: by_axis[i],
                        order: parent.map_or(0, |p| chains[p].order + 1),
                        borne: Vec::new(),
                    });
                    chains.len() - 1
                }
            };
            chain_of[i] = c;
            chains[c].axes.push(i);
        }
        for (r, record) in records.iter().enumerate() {
            if let Some(p) = record.origin.parent() {
                chains[chain_of[p]].borne.push(r);
            }
        }
        let resolved = vec![false; chains.len()];
        Self {
            species,
            rows,
            leaf_mean,
            unbent: structure.clone(),
            bent: structure,
            records,
            chains,
            chain_of,
            resolved,
            timings: (0.0, 0.0),
        }
    }

    /// Seconds spent growing and settling the tree, and reading its
    /// closed form's rows.
    pub fn timings(&self) -> (f64, f64) {
        self.timings
    }

    /// The chains, indexed so that a chain follows the one bearing it.
    pub fn chains(&self) -> usize {
        self.chains.len()
    }

    pub fn parent(&self, c: usize) -> Option<usize> {
        self.chains[c].parent
    }

    /// 0 for the trunk's chain, one more for each lateral it stands on.
    pub fn order(&self, c: usize) -> usize {
        self.chains[c].order
    }

    /// The axes of chain `c`, in index order.
    pub fn axes(&self, c: usize) -> &[usize] {
        &self.chains[c].axes
    }

    /// The chain axis `i` belongs to.
    pub fn chain_of(&self, i: usize) -> usize {
        self.chain_of[i]
    }

    pub fn is_resolved(&self, c: usize) -> bool {
        self.resolved[c]
    }

    /// The phytomers chain `c` holds.
    pub fn phytomers(&self, c: usize) -> usize {
        let axes = &self.bent.axes;
        self.chains[c]
            .axes
            .iter()
            .map(|&i| axes[i].phytomers.len())
            .sum()
    }

    /// Where chain `c`'s bud stands on its laid bearer and how far its
    /// wood can reach from there (the extent bound, decision 16): known
    /// once its bearer is resolved; the seed at the ground.
    pub fn bud(&self, c: usize) -> Option<(Vec3, f64)> {
        let chain = &self.chains[c];
        let Some(r) = chain.record else {
            return Some((Vec3::default(), self.rows.seed().reach));
        };
        if !self.resolved[chain.parent?] {
            return None;
        }
        let record = &self.records[r];
        let (base, _) =
            crate::geometry::bud_frame(&self.bent.axes, self.species, girth::bud(record));
        let reach = self.rows.lateral(record.pa, record.birth).reach * self.base_scale(record);
        Some((base, reach))
    }

    /// The tree as resolved so far: an unresolved chain's axes are unlaid.
    pub fn structure(&self) -> &Structure {
        &self.bent
    }

    /// The tree once every chain is resolved.
    pub fn into_structure(self) -> Structure {
        self.bent
    }

    /// The closed form's rows held.
    pub fn rows(&self) -> usize {
        self.rows.len()
    }

    /// Resolves chain `c`, whose bearer must be resolved; a chain already
    /// resolved is left as it is.
    pub fn resolve(&mut self, c: usize) -> Result<()> {
        if self.resolved[c] {
            return Ok(());
        }
        let done = self.work(c)?;
        self.apply(c, done);
        Ok(())
    }

    /// Resolves every chain, those of one order at once on up to
    /// `threads` cores: the one-pass build, which is the same resolution.
    /// `threads` 0 takes every core.
    pub fn resolve_all(&mut self, threads: usize) -> Result<()> {
        let threads = if threads == 0 {
            crate::cores::available()
        } else {
            threads
        };
        let mut by_order: Vec<Vec<usize>> = Vec::new();
        for (c, chain) in self.chains.iter().enumerate() {
            if by_order.len() <= chain.order {
                by_order.push(Vec::new());
            }
            by_order[chain.order].push(c);
        }
        for level in by_order {
            let level: Vec<usize> = level.into_iter().filter(|&c| !self.resolved[c]).collect();
            for (c, done) in self.work_all(&level, threads) {
                self.apply(c, done?);
            }
        }
        Ok(())
    }

    fn work_all(&self, chains: &[usize], threads: usize) -> Vec<(usize, Result<Done>)> {
        use std::sync::Mutex;
        const PIECE: usize = 16;
        let pieces = chains.len().div_ceil(PIECE);
        let done: Vec<Mutex<Worked>> =
            (0..pieces).map(|_| Mutex::new(Vec::new())).collect();
        crate::cores::spread(pieces, threads, &|k| {
            let piece = &chains[k * PIECE..((k + 1) * PIECE).min(chains.len())];
            *done[k].lock().unwrap() = piece.iter().map(|&c| (c, self.work(c))).collect();
        });
        done.into_iter()
            .flat_map(|d| d.into_inner().unwrap())
            .collect()
    }

    /// A record's base scale: its bearer's at its node, before the size
    /// the bearer's unit took, times its vigour as made and its share of
    /// it among its siblings.
    fn base_scale(&self, record: &Record) -> f64 {
        let Origin::Lateral { parent, node, .. } = record.origin else {
            return 1.0;
        };
        let at = &self.bent.axes[parent].phytomers[node];
        let inherited = if at.size > 0.0 {
            at.scale / at.size
        } else {
            0.0
        };
        let share = dominance(
            self.species.states[record.pa].form.dominance,
            record.lineage,
        );
        inherited * record.vigour * share
    }
}

/// Grows `species` resolving every chain against its committed boundary
/// (fn-209, option 4), on up to `threads` cores (none: all of them).
pub fn grow_bounded(
    species: &Species,
    request: Request,
    threads: Option<usize>,
) -> Result<Structure> {
    let mut tree = Bounded::new(species, request)?;
    tree.resolve_all(threads.unwrap_or_else(crate::cores::available))?;
    Ok(tree.into_structure())
}
