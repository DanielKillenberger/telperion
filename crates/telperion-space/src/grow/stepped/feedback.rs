//! Contract A's fine roots on a second executor (fn-211 task 10): inside
//! the stepped executor, the device takes the items' D16, their leaves'
//! deposits and their buds' sampling each cycle, and the CPU the rest:
//! the coarse grower's, the lattice's box, the one sweep, the re-lay.
//! Every value crossing is an integer of contract A, so the tree is the
//! all-CPU run's to the word (CONTRACT-A.md 2.9), which `states` shows
//! after every phase of every cycle.
use super::lay::{Laid, LayJob};
use super::{links, Executor};
use crate::fixed::feedback::lattice::Grid;
use crate::fixed::feedback::shed::{walk, Lateral, Walked};
use crate::fixed::feedback::Fault;
use crate::fixed::{presence, ONE32};
use crate::grow::Grower;
use crate::light::field::q;
use crate::light::Field;
use crate::species::Species;
use crate::structure::Vec3;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Write;
use std::hash::Hasher;

/// What a second executor runs for the fine roots each cycle: the items'
/// D16 walks, their leaves' deposits and their buds' light. Each is one
/// result of contract A, a fault included.
pub trait Feedback {
    /// Each item's walk (`fixed::feedback::shed::walk`): its axes and its
    /// root's decision, given by its bearer.
    fn shed(&mut self, items: &[(Vec<Lateral>, (bool, u64))]) -> Result<Vec<Vec<Walked>>, Fault>;
    /// `leaves` (Q32 point, Q32 area) deposited on `grid`, in W.
    fn deposit(&mut self, grid: &Grid, leaves: &[([i64; 3], i64)]) -> Result<Vec<u128>, Fault>;
    /// The light, Q32, at each point of `at` on `grid`'s swept `light`.
    fn sample(&mut self, grid: &Grid, light: &[i64], at: &[[i64; 3]]) -> Vec<i64>;
    /// Each job's units laid (`LayPa::table(species)` its constants).
    fn lay(&mut self, species: &Species, jobs: &[LayJob]) -> Vec<Laid>;
}

/// The device again, for one more call.
pub(super) fn again<'s>(d: &'s mut Option<&mut dyn Feedback>) -> Option<&'s mut dyn Feedback> {
    match d {
        Some(d) => Some(&mut **d),
        None => None,
    }
}

/// The cycle's leaves and buds' tips, the coarse grower's and the items'.
type Gathered<'g> = (&'g [Vec<(Vec3, f64)>; 2], &'g [Vec<Vec3>; 2]);

/// The CPU's own: the reference a device is held to word for word.
pub struct Cpu;

impl Feedback for Cpu {
    fn shed(&mut self, items: &[(Vec<Lateral>, (bool, u64))]) -> Result<Vec<Vec<Walked>>, Fault> {
        items.iter().map(|(a, root)| walk(a, Some(*root))).collect()
    }

    fn deposit(&mut self, grid: &Grid, leaves: &[([i64; 3], i64)]) -> Result<Vec<u128>, Fault> {
        grid.deposit_all(leaves)
    }

    fn sample(&mut self, grid: &Grid, light: &[i64], at: &[[i64; 3]]) -> Vec<i64> {
        at.iter().map(|&p| grid.sample(light, p)).collect()
    }

    fn lay(&mut self, species: &Species, jobs: &[LayJob]) -> Vec<Laid> {
        super::lay::lay_cpu(species, jobs)
    }
}

impl<'a> Executor<'a> {
    /// Phase (iv)'s light: every grower's leaves of the cycle into one
    /// lattice, swept once, each grower reading its own buds; the items'
    /// deposits and reads on `device` where one is given.
    pub(super) fn shade(&mut self, device: Option<&mut dyn Feedback>) {
        let order = self.visit();
        let threads = self.coarse.threads;
        let light = self.coarse.sketch.as_ref().expect("sketching").light;
        let growers: Vec<usize> = std::iter::once(usize::MAX).chain(order).collect();
        let (mut leaves, mut tips) = ([Vec::new(), Vec::new()], [Vec::new(), Vec::new()]);
        for &k in &growers {
            let g = self.grower(k);
            let fine = usize::from(k != usize::MAX);
            leaves[fine].extend_from_slice(&g.leaves);
            tips[fine].extend(g.live.iter().map(|a| g.tip(a.axis)));
        }
        let (field, read) = match device {
            Some(device) => self.shade_on(device, (&leaves, &tips)),
            None => {
                let [mut leaves, fine] = leaves;
                leaves.extend(fine);
                let tips = tips.concat();
                let field = Field::on(&light, &leaves, &tips, threads);
                let mut read = vec![0.0; tips.len()];
                crate::cores::fill(&mut read, 1 << 10, threads, &|k| field.at(tips[k]));
                (field, read)
            }
        };
        if self.complete {
            self.lattices.push(lattice_hash(&field));
        }
        let mut from = 0;
        for &k in &growers {
            let g = self.grower_mut(k);
            let n = g.live.len();
            let sketch = g.sketch.as_mut().expect("sketching");
            for (apex, &light) in g.live.iter().zip(&read[from..from + n]) {
                sketch.pencils[apex.axis].light = light;
            }
            from += n;
        }
    }

    /// Phase (i) inside the items on `device`: each item's walk read from
    /// its totals and its root's decision, taken there, applied here.
    pub(super) fn shed_on(
        &mut self,
        cycle: u32,
        device: &mut dyn Feedback,
        roots: Vec<(super::Totals, (bool, f64, f64))>,
    ) {
        let read = self.each(roots, &|item, (t, (shed, kept, memory))| {
            let root = &mut item.g.sketch.as_mut().expect("sketching").pencils[1];
            root.memory = memory;
            let rows = links(&item.g);
            (item.g.laterals(cycle, &rows, &t), (shed, presence::q(kept)))
        });
        let walked = device.shed(&read).expect("a balance's light and upkeep");
        self.each(walked, &|item, w| {
            item.g.shed_walked(cycle, &w);
        });
    }

    /// Phase (iv)'s light with the items' leaves deposited and their buds
    /// read on `device`: the coarse grower's on the CPU, the box and the
    /// one sweep on the CPU. Each grower's reads, in `growers`' order.
    pub(super) fn shade_on(
        &mut self,
        device: &mut dyn Feedback,
        (leaves, tips): Gathered,
    ) -> (Field, Vec<f64>) {
        let light = self.coarse.sketch.as_ref().expect("sketching").light;
        let quantised = |l: &[(Vec3, f64)]| -> Vec<([i64; 3], i64)> {
            l.iter()
                .map(|&(p, a)| (q(p), crate::fixed::q32(a)))
                .collect()
        };
        let (coarse, fine) = (quantised(&leaves[0]), quantised(&leaves[1]));
        let points =
            (coarse.iter().chain(&fine).map(|l| l.0)).chain(tips.iter().flatten().map(|&t| q(t)));
        let grid = Grid::spanning(points).expect("positions inside the contract's domain");
        let mut lattice = grid
            .deposit_all(&coarse)
            .expect("a leaf inside the contract");
        let theirs = device
            .deposit(&grid, &fine)
            .expect("a leaf inside the contract");
        for (l, d) in lattice.iter_mut().zip(theirs) {
            *l = l.wrapping_add(d);
        }
        let field = Field::of_lattice(&light, grid, lattice, self.coarse.threads);
        let mut read: Vec<f64> = tips[0].iter().map(|&t| field.at(t)).collect();
        let at: Vec<[i64; 3]> = tips[1].iter().map(|&t| q(t)).collect();
        let (grid, _, swept) = field.parts();
        let real = |l: i64| l as f64 / ONE32 as f64;
        read.extend(device.sample(grid, swept, &at).into_iter().map(real));
        (field, read)
    }

    /// Every grower's whole state after phase `k` (`Stepped::states`):
    /// the coarse grower's, then each item's, in its own order.
    pub(super) fn state(&self) -> u64 {
        let mut h = Words(DefaultHasher::new());
        let growers = std::iter::once(&self.coarse).chain(self.items.iter().map(|i| &i.g));
        for g in growers {
            whole(g, &mut h);
        }
        h.0.finish()
    }
}

/// A grower's leaves of `cycle` as its axes stand.
pub(super) fn relaid(g: &mut Grower, cycle: u32) {
    let species = g.species;
    g.leaves.clear();
    for axis in &g.axes {
        let area = species.states[axis.pa].leaf_area;
        for p in axis.phytomers.iter().rev().take_while(|p| p.cycle == cycle) {
            g.leaves
                .push((p.tip, crate::light::leaf_area(p.scale, p.size, area)));
        }
    }
}

/// A lattice and its light, hashed.
pub(super) fn lattice_hash(field: &Field) -> u64 {
    let (grid, density, light) = field.parts();
    let mut h = DefaultHasher::new();
    for v in grid.low.iter().chain(light) {
        h.write_i64(*v);
    }
    for s in grid.size {
        h.write_u64(s);
    }
    for d in density {
        h.write_u128(*d);
    }
    h.finish()
}

/// Everything a grower carries from cycle to cycle, written out whole.
fn whole(g: &Grower, h: &mut Words) {
    let _ = write!(
        h,
        "{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}",
        g.horizon,
        g.marked,
        g.axes,
        g.draws,
        g.live,
        g.units,
        g.successor,
        g.roots,
        g.asleep,
        g.woken,
        g.leaves,
        g.rules.grown,
    );
    if let Some(s) = g.sketch.as_ref() {
        let _ = write!(h, "{:?}", s.pencils);
    }
}

/// A hasher written to as text.
struct Words(DefaultHasher);

impl Write for Words {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write(s.as_bytes());
        Ok(())
    }
}
