//! The GPU executor under realized mechanics (fn-209 F1 task 6;
//! REALIZED.md, host decision 8, step 4): `telperion_space::Roots` on the
//! device. The structure above the cut settles between the phases on the
//! CPU (`telperion_space::settle`); each phase takes one record per root
//! down and gives one per root up, and nothing per chain or per node
//! crosses. Phase 1 grows every root whole and carries its living up;
//! phase 2 sheds, scales and thickens, its base radius up; phase 3 lays
//! and loads, its load up; phase 4 lays under the levers. The roots grow
//! in groups (task 9, `group.rs`): those the driver sows while the
//! structure above the cut grows, then the rest in `grow`.
use super::super::io;
use super::group::{unlaid, Group, CR};
use super::laid::{Wave, BENDS, GIRTHS, LAYS, LEVERS, SCALES};
use super::realized::{RootBud, Waves};
use super::roots::{Cost, Kernels, Tables};
use crate::Gpu;
use std::sync::Arc;
use std::time::Instant;
use telperion_space::fixed::presence::times;
use telperion_space::{
    bearer_frame, ChainTables, GeometryTables, Lived, Loaded, Result, RootSeed, Roots, Structure,
    Vec3,
};

/// A species point's tables built on another core while the structure
/// above the cut grows: the chain's and the geometry's (task 8).
pub(super) type Building = std::thread::JoinHandle<(ChainTables, GeometryTables)>;

/// The least roots the device takes to grow while the structure above
/// the cut grows on (task 9): fewer wait for the next cycle's. Each
/// group grown pays its waves in turn, so the beech, whose few long
/// chains take most of its build, gains nothing from more (2,048: beech
/// 80 seed 1 at 163 ms against 156 unsown; `sown_against_none`).
const LEAST: usize = 8192;

/// The GPU executor's roots, taken in `order`, `batch` chains a run.
pub(super) struct Device<'a> {
    gpu: &'a Gpu,
    k: &'a Kernels,
    given: Option<&'a Tables>,
    built: Option<Tables>,
    building: Option<Building>,
    order: Vec<usize>,
    batch: usize,
    least: usize,
    pub cost: Cost,
    /// The groups grown, those sown first, and each one's roots, by
    /// their place in it.
    groups: Vec<Group>,
    roots: Vec<Vec<usize>>,
    /// Each root's group and place in it.
    places: Vec<(usize, usize)>,
}

impl<'a> Device<'a> {
    pub(super) fn new(gpu: &'a Gpu, k: &'a Kernels, tables: &'a Tables, batch: usize) -> Self {
        let mut device = Self::unbuilt(gpu, k, batch);
        device.given = Some(tables);
        device
    }

    /// The executor with its tables still `building`, uploaded when phase
    /// 1 first needs them.
    pub(super) fn building(gpu: &'a Gpu, k: &'a Kernels, building: Building, batch: usize) -> Self {
        let mut device = Self::unbuilt(gpu, k, batch);
        device.building = Some(building);
        device
    }

    fn unbuilt(gpu: &'a Gpu, k: &'a Kernels, batch: usize) -> Self {
        Self {
            gpu,
            k,
            given: None,
            built: None,
            building: None,
            order: Vec::new(),
            batch,
            least: LEAST,
            cost: Cost::default(),
            groups: Vec::new(),
            roots: Vec::new(),
            places: Vec::new(),
        }
    }

    fn tables(&self) -> &Tables {
        self.given
            .or(self.built.as_ref())
            .expect("the tables, built")
    }

    /// The tables, uploaded once built.
    fn ready(&mut self) {
        if let Some(building) = self.building.take() {
            let (chain, geo) = building.join().expect("the tables");
            self.built = Some(Tables::new(self.gpu, Arc::new(chain), &geo));
        }
    }

    /// Takes every root in `order` (by default, forward), in one group:
    /// none is sown while the structure above the cut grows.
    pub(super) fn ordered(mut self, order: Vec<usize>) -> Self {
        self.order = order;
        self
    }

    /// Sows while the structure above the cut grows from `least` roots on.
    pub(super) fn sown_from(mut self, least: usize) -> Self {
        self.least = least;
        self
    }

    /// Each group's waves as the device keeps them, its laid buffers, and
    /// its roots by their place in it.
    pub(super) fn groups(&self) -> impl Iterator<Item = (&Waves, &[Wave], &[usize])> {
        let roots = self.roots.iter().map(Vec::as_slice);
        self.groups
            .iter()
            .zip(roots)
            .map(|(g, r)| (&g.waves, &g.laid[..], r))
    }

    /// Waits for the last phase's passes, which the driver's own landing
    /// overlaps (task 8).
    pub(super) fn wait(&self) {
        self.gpu
            .device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();
    }

    /// `seeds` grown as one group, taken in `order`.
    fn group(&mut self, seeds: &[&RootSeed], order: Vec<usize>) {
        let start = Instant::now();
        self.ready();
        let roots: Vec<RootBud> = seeds
            .iter()
            .map(|s| (s.bud, [s.made.0 as f32, s.made.1 as f32], s.birth))
            .collect();
        let nodes = seeds.iter().map(|s| s.bud.node).collect();
        let (gpu, k, batch) = (self.gpu, self.k, self.batch);
        let mut cost = std::mem::take(&mut self.cost);
        let t = self.tables();
        let group = Group::grow((gpu, k), (&roots, nodes), (order, batch), (t, &mut cost));
        self.cost = cost;
        self.cost.grow += start.elapsed().as_secs_f64() * 1e3;
        self.groups.push(group);
    }

    /// Each group's records read back in one round trip, by root.
    fn records(&mut self) -> Vec<[f32; CR as usize]> {
        self.cost.trips += 1;
        let gpu = self.gpu;
        let roots: u64 = self.roots.iter().map(|r| r.len() as u64).sum();
        let total = roots * u64::from(CR) * 4;
        let usage = wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let into = io::buffer(gpu, "records gathered", total.max(16), usage, &[]).unwrap();
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        let mut at = Vec::with_capacity(self.groups.len());
        let mut bytes = 0;
        for g in &self.groups {
            at.push(bytes as usize / 4);
            bytes += g.gather(&mut encoder, &into, bytes);
        }
        gpu.queue.submit([encoder.finish()]);
        let raw = io::read(gpu, &into, total.max(16)).unwrap();
        let floats: &[f32] = bytemuck::cast_slice(&raw);
        let by_group: Vec<_> = self
            .groups
            .iter()
            .zip(at)
            .map(|(g, at)| g.records(&floats[at..]))
            .collect();
        self.places.iter().map(|&(g, i)| by_group[g][i]).collect()
    }

    /// Each group's roots among `live`, as a word each by their place.
    fn on(&self, live: &[usize]) -> Vec<Vec<u32>> {
        let mut on: Vec<Vec<u32>> = self.roots.iter().map(|r| vec![0; r.len()]).collect();
        for &r in live {
            let (g, i) = self.places[r];
            on[g][i] = 1;
        }
        on
    }

    /// Writes each group's stubs and runs `entry` over it.
    fn passes(&self, entry: usize, down: bool) {
        let t = self.tables();
        for g in &self.groups {
            g.attach(self.gpu);
            g.pass((self.gpu, self.k), t, entry, down);
        }
    }

    fn attached(&mut self, r: usize) -> &mut telperion_space::RootGeometry {
        let (g, i) = self.places[r];
        &mut self.groups[g].attached[i]
    }

    fn node(&self, r: usize) -> usize {
        let (g, i) = self.places[r];
        self.groups[g].nodes[i]
    }
}

/// The pooled buffers go back for the next build (`pool.rs`).
impl Drop for Device<'_> {
    fn drop(&mut self) {
        for g in std::mem::take(&mut self.groups) {
            self.k.pool.give(g.buffers());
        }
    }
}

impl Roots for Device<'_> {
    fn sow(&mut self, seeds: &[RootSeed]) -> Result<usize> {
        if !self.order.is_empty() || seeds.len() < self.least {
            return Ok(0);
        }
        let start = Instant::now();
        let all: Vec<&RootSeed> = seeds.iter().collect();
        self.group(&all, (0..all.len()).collect());
        self.cost.sown += start.elapsed().as_secs_f64() * 1e3;
        Ok(seeds.len())
    }

    fn grow(&mut self, seeds: &[RootSeed]) -> Result<(Vec<Lived>, u64)> {
        let start = Instant::now();
        // The roots sown are the first seeds, group by group; the rest's
        // after them.
        let mut roots: Vec<Vec<usize>> = Vec::with_capacity(self.groups.len() + 1);
        let mut from = 0;
        for g in &self.groups {
            roots.push((from..from + g.attached.len()).collect());
            from += g.attached.len();
        }
        let rest: Vec<usize> = (from..seeds.len()).collect();
        if !rest.is_empty() {
            let order = match self.order.len() == seeds.len() {
                // The order given, over every root: none was sown.
                true => self.order.clone(),
                false => (0..rest.len()).collect(),
            };
            let group: Vec<&RootSeed> = rest.iter().map(|&r| &seeds[r]).collect();
            self.group(&group, order);
            roots.push(rest);
        }
        self.places = vec![(0, 0); seeds.len()];
        for (g, rs) in roots.iter().enumerate() {
            for (i, &r) in rs.iter().enumerate() {
                self.places[r] = (g, i);
            }
        }
        self.roots = roots;
        // Each root's first axis's vigour as its bearer made it, a woken
        // root's as its bearer kept it, which phase 2 scales it by.
        for (r, s) in seeds.iter().enumerate() {
            let (g, i) = self.places[r];
            self.groups[g].made[i] = times(s.birth[0], s.birth[1]);
        }
        let lived = self
            .places
            .iter()
            .map(|&(g, i)| self.groups[g].lived[i])
            .collect();
        let grown = self.groups.iter().flat_map(|g| &g.waves.phyts);
        let grown = grown.map(|&p| u64::from(p)).sum();
        self.cost.last = start.elapsed().as_secs_f64() * 1e3;
        Ok((lived, grown))
    }

    fn girth(&mut self, live: &[usize], scales: &[(f64, f64)]) -> Vec<f64> {
        let start = Instant::now();
        let (gpu, k) = (self.gpu, self.k);
        for (g, on) in self.groups.iter().zip(self.on(live)) {
            g.shed((gpu, k), &on);
        }
        for (&r, &(base, drawn)) in live.iter().zip(scales) {
            let a = self.attached(r);
            a.unbent = unlaid(base * drawn);
            a.bent = a.unbent;
        }
        let t = self.tables();
        for g in &self.groups {
            g.attach(gpu);
            g.pass((gpu, k), t, SCALES, true);
            g.pass((gpu, k), t, GIRTHS, false);
        }
        let read = self.records();
        self.cost.lay += start.elapsed().as_secs_f64() * 1e3;
        live.iter().map(|&r| f64::from(read[r][0])).collect()
    }

    fn lay(&mut self, tree: &Structure, live: &[usize], bearers: &[usize]) -> Result<Vec<Loaded>> {
        let start = Instant::now();
        for (&r, &b) in live.iter().zip(bearers) {
            let frame = bearer_frame(tree, b, self.node(r));
            let a = self.attached(r);
            (a.bearer_pa, a.unbent, a.bent) = (tree.axes[b].pa, frame, frame);
        }
        let sags = self.tables().sags;
        let t = self.tables();
        for g in &self.groups {
            g.attach(self.gpu);
            g.pass((self.gpu, self.k), t, LAYS, true);
            if sags {
                g.pass((self.gpu, self.k), t, LEVERS, false);
            }
        }
        let loads = if sags {
            let read = self.records();
            live.iter()
                .map(|&r| {
                    let o = self.attached(r).unbent.tip;
                    let [_, m, x, y, z] = read[r].map(f64::from);
                    (m, Vec3::new(x, y, z) + o * m)
                })
                .collect()
        } else {
            vec![(0.0, Vec3::default()); live.len()]
        };
        self.cost.lay += start.elapsed().as_secs_f64() * 1e3;
        Ok(loads)
    }

    fn bend(&mut self, tree: &Structure, live: &[usize], bearers: &[usize]) -> Result<()> {
        let start = Instant::now();
        for (&r, &b) in live.iter().zip(bearers) {
            let frame = bearer_frame(tree, b, self.node(r));
            self.attached(r).bent = frame;
        }
        // Not waited for: the CPU lands its roots meanwhile (`wait`).
        self.passes(BENDS, true);
        self.cost.lay += start.elapsed().as_secs_f64() * 1e3;
        Ok(())
    }
}
