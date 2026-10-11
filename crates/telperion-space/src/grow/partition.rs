//! fn-211.4, probe (a)'s arithmetic (DESIGN-INTERLEAVE.md after Astra's
//! review): today's tree grown with every reduction that crosses the cut
//! taken as a partitioned run must take it, from the reference's own
//! values, and compared with the whole tree after every phase of every
//! cycle.
//!
//! A partitioned run keeps each axis's own work and every bearer-to-child
//! reduction (they read one axis and its children, in the tree's order),
//! so those are the whole tree's arithmetic wherever the work runs. Two
//! reductions run over the whole tree in its index order and cannot be
//! taken from per-root totals without regrouping their sums:
//!
//! - **sizes**: each PA's Σp and Σp(v/p)^ψ (decision 23), phase (iii);
//! - **leaves**: the lattice's deposits, phase (iv).
//!
//! Each is regrouped here as a partition would group it: the structure
//! above the cut in index order, then each fine root's own partial in its
//! index order, the roots taken in the plan's order, `chunk` partials
//! summed before they join the total. Every run is taken in one growth
//! pass (FRICTION.md, 2026-10-06): the tree grows on its own sums, and
//! each run's grouping of the same terms is compared with them where they
//! are taken. Where every grouping's sums are the tree's to the bit, every
//! partitioned run is the tree; the first sum that differs bounds from
//! below the first phase a separate run of it would differ in.
use super::oracle::{Oracle, Read};
use super::{run_beside, Beside, Grower, Options, Request};
use crate::bounded::Expected;
use crate::error::Result;
use crate::light::Field;
use crate::species::Species;
use crate::structure::{Structure, Vec3};
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

/// The order fine roots' partials are combined in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Forward,
    Reverse,
    Shuffled(u64),
}

/// How a partitioned run groups the whole-tree reductions, and which.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grouping {
    pub order: Order,
    pub chunk: usize,
    pub sizes: bool,
    pub leaves: bool,
}

/// One partitioned run: its cut and its grouping.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Run {
    pub cut: f64,
    pub grouping: Grouping,
}

/// The phases of a cycle (DESIGN-INTERLEAVE.md, Astra change 1): balance,
/// allocation, growth and waking, then layout and light.
pub const PHASES: [&str; 4] = ["balance", "allocation", "growth", "light"];

/// A tree grown under probe (a): its state hashed after each phase of
/// each cycle, and for each run the first cycle and phase whose sums its
/// grouping takes otherwise (none: every one the tree's).
pub struct Partitioned {
    pub structure: Structure,
    pub phases: Vec<[u64; 4]>,
    pub first: Vec<Option<(u32, &'static str)>>,
}

/// Grows `species` as `grow` does, hashing each phase, with every run's
/// grouping of the whole-tree reductions compared where they are taken.
pub fn grow_partitioned(
    species: &Species,
    request: Request,
    expected: &Expected,
    runs: &[Run],
) -> Result<Partitioned> {
    let mut oracle = Oracle::new(expected, f64::NEG_INFINITY, Read::Exact);
    oracle.runs = runs.to_vec();
    oracle.first = vec![None; runs.len()];
    let beside = Beside {
        oracle: Some(oracle),
        ..Beside::default()
    };
    let quiet = &mut |_| {};
    let (grown, besides) = run_beside(species, request, (quiet, true), Options::default(), beside)?;
    let oracle = besides.oracle.expect("an oracle");
    Ok(Partitioned {
        structure: grown.structure,
        phases: oracle.phases,
        first: oracle
            .first
            .iter()
            .map(|f| f.map(|(c, k)| (c, PHASES[k])))
            .collect(),
    })
}

/// The roots in `order`.
pub(crate) fn ordered(mut roots: Vec<usize>, order: Order) -> Vec<usize> {
    roots.sort_unstable();
    roots.dedup();
    match order {
        Order::Forward => {}
        Order::Reverse => roots.reverse(),
        Order::Shuffled(seed) => {
            let key = crate::lineage::Key::root(seed);
            roots.sort_by_key(|&r| key.child(r as u64).0);
        }
    }
    roots
}

/// Merges per-root partials into `total` as the grouping takes them:
/// the roots in its order, `chunk` partials at a time.
pub(crate) fn merge(total: &mut [f32], partial: &HashMap<usize, Vec<f32>>, g: &Grouping) {
    let roots = ordered(partial.keys().copied().collect(), g.order);
    for chunk in roots.chunks(g.chunk.max(1)) {
        let mut sum = vec![0.0f32; total.len()];
        for r in chunk {
            for (s, &x) in sum.iter_mut().zip(&partial[r]) {
                *s += x;
            }
        }
        for (t, s) in total.iter_mut().zip(sum) {
            *t += s;
        }
    }
}

/// Sums `n`-wide values, each owned by the structure above the cut (none)
/// or a fine root, as the grouping takes them (none: in the items' order,
/// whole): the structure's in order, then each root's partial.
pub(crate) fn grouped(
    items: &[(Option<usize>, Vec<f32>)],
    n: usize,
    g: Option<&Grouping>,
) -> Vec<f32> {
    let mut total = vec![0.0f32; n];
    let mut partial: HashMap<usize, Vec<f32>> = HashMap::new();
    for (root, v) in items {
        let into = match (root, g) {
            (Some(r), Some(_)) => partial.entry(*r).or_insert_with(|| vec![0.0; n]),
            _ => &mut total,
        };
        for (t, &x) in into.iter_mut().zip(v) {
            *t += x;
        }
    }
    if let Some(g) = g {
        merge(&mut total, &partial, g);
    }
    total
}

/// Leaves regrouped: the structure's in their order, then each root's.
pub(crate) fn regroup(
    leaves: &[(Vec3, f64)],
    owner: &[Option<usize>],
    g: &Grouping,
) -> Vec<(Vec3, f64)> {
    let mut out: Vec<(Vec3, f64)> = Vec::with_capacity(leaves.len());
    let mut by_root: HashMap<usize, Vec<(Vec3, f64)>> = HashMap::new();
    for (&leaf, owner) in leaves.iter().zip(owner) {
        match owner {
            None => out.push(leaf),
            Some(r) => by_root.entry(*r).or_default().push(leaf),
        }
    }
    for r in ordered(by_root.keys().copied().collect(), g.order) {
        out.extend(&by_root[&r]);
    }
    out
}

/// A phase's state hashed to the bit.
pub(crate) fn hash(values: impl Iterator<Item = f64>) -> u64 {
    let mut h = DefaultHasher::new();
    for v in values {
        v.to_bits().hash(&mut h);
    }
    h.finish()
}

/// Which of the two sums a run regroups.
#[derive(Clone, Copy)]
enum Which {
    Sizes,
    Leaves,
}

impl Which {
    fn of(self, g: &Grouping) -> bool {
        match self {
            Which::Sizes => g.sizes,
            Which::Leaves => g.leaves,
        }
    }
}

impl Grower<'_> {
    /// The runs that regroup `which`, each with the index of its cut.
    fn runs(&self, which: Which) -> Vec<(usize, Run)> {
        self.oracle.as_ref().map_or_else(Vec::new, |o| {
            let runs = o.runs.iter().copied().enumerate();
            runs.filter(|(_, r)| which.of(&r.grouping)).collect()
        })
    }

    /// Notes run `k`'s first difference at `cycle`, phase `phase`.
    fn differs(&mut self, k: usize, cycle: u32, phase: usize) {
        let oracle = self.oracle.as_mut().expect("an oracle");
        oracle.first[k].get_or_insert((cycle, phase));
    }

    /// Each run's grouping of each PA's Σp and Σp(v/p)^ψ against the
    /// tree's own.
    pub(super) fn check_sizes(
        &mut self,
        cycle: u32,
        got: &[f64],
        (presence, psi, pa): (&[f64], &[f64], &[usize]),
    ) {
        let runs = self.runs(Which::Sizes);
        if runs.is_empty() {
            return;
        }
        let pas = self.species.states.len();
        let q = |x: f64| x as f32;
        let terms: Vec<(usize, Vec<f32>)> = (0..got.len())
            .filter(|&i| q(presence[i]) > 0.0)
            .map(|i| {
                let (p, d) = (
                    q(presence[i]),
                    crate::allocation::drawn(q(got[i]), q(presence[i]), q(psi[i])),
                );
                let mut v = vec![0.0; 2 * pas];
                v[2 * pa[i]] = p;
                v[2 * pa[i] + 1] = d * p;
                (i, v)
            })
            .collect();
        let whole: Vec<_> = terms.iter().map(|(_, v)| (None, v.clone())).collect();
        let reference = grouped(&whole, 2 * pas, None);
        for (k, run) in runs {
            let oracle = self.oracle.as_mut().expect("an oracle");
            let items: Vec<_> = terms
                .iter()
                .map(|(i, v)| (oracle.root_at(&self.axes, *i, run.cut), v.clone()))
                .collect();
            if grouped(&items, 2 * pas, Some(&run.grouping)) != reference {
                self.differs(k, cycle, 1);
            }
        }
    }

    /// Each run's lattice of the cycle's leaves against the tree's own.
    pub(super) fn check_leaves(&mut self, cycle: u32, tips: &[Vec3]) {
        let runs = self.runs(Which::Leaves);
        if runs.is_empty() {
            return;
        }
        let reference = Field::deposited(&self.leaves, tips);
        let axes = std::mem::take(&mut self.oracle.as_mut().expect("an oracle").leaf_axes);
        for (k, run) in runs {
            let oracle = self.oracle.as_mut().expect("an oracle");
            let owner: Vec<Option<usize>> = axes
                .iter()
                .map(|&i| oracle.root_at(&self.axes, i, run.cut))
                .collect();
            let leaves = regroup(&self.leaves, &owner, &run.grouping);
            if Field::deposited(&leaves, tips) != reference {
                self.differs(k, cycle, 3);
            }
        }
    }
}

/// One grower's part of a phase's digest: the order-free sum of its
/// living apexes' hashes, each keyed by its lineage (so a run that
/// numbers its axes otherwise, a stepped executor, compares by it), and
/// the growth phase's counts of axes, phytomers and living apexes.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Digest {
    pub sum: u64,
    pub counts: [usize; 3],
}

impl Digest {
    /// Two growers' parts together.
    pub fn and(self, other: Self) -> Self {
        let c = |k: usize| self.counts[k] + other.counts[k];
        Self {
            sum: self.sum.wrapping_add(other.sum),
            counts: [c(0), c(1), c(2)],
        }
    }

    /// Phase `k`'s hash: the counts' for growth, the sum's otherwise.
    pub fn finish(self, k: usize) -> u64 {
        match k {
            2 => hash(self.counts.into_iter().map(|x| x as f64)),
            _ => self.sum,
        }
    }
}

impl Grower<'_> {
    /// This grower's part of phase `k`'s digest: the living apexes'
    /// balance (kept, memory), sizes, or light and tips, where a rough
    /// layout is grown; the tree grown (axes, phytomers, living).
    pub(super) fn digest(&self, k: usize) -> Digest {
        if k == 2 {
            let phytomers: usize = self.axes.iter().map(|a| a.phytomers.len()).sum();
            let counts = [self.axes.len(), phytomers, self.live.len()];
            return Digest { sum: 0, counts };
        }
        let Some(sketch) = self.sketch.as_ref() else {
            return Digest::default();
        };
        let live = self.live.iter().filter(|a| a.axis < sketch.pencils.len());
        let keyed = |a: &super::Apex| {
            let p = &sketch.pencils[a.axis];
            let axis = &self.axes[a.axis];
            let tip = axis.phytomers.last().map_or(axis.base, |q| q.tip);
            let v = match k {
                0 => vec![p.kept, p.memory],
                1 => vec![p.size],
                _ => vec![p.light, tip.x, tip.y, tip.z],
            };
            let lineage = f64::from_bits(axis.lineage);
            hash(std::iter::once(lineage).chain(v))
        };
        Digest {
            sum: live.map(keyed).fold(0, u64::wrapping_add),
            counts: [0; 3],
        }
    }
}
