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
use super::super::{io, Clock};
use super::group::{unlaid, Group, CR};
use super::laid::{Wave, BENDS, GIRTHS, LAYS, LEVERS, SCALES};
use super::realized::{RootBud, Waves};
use super::roots::{Cost, Kernels, Tables};
use crate::Gpu;
use std::sync::Arc;
use telperion_space::fixed::presence::times;
use telperion_space::{
    bearer_frame, ChainTables, GeometryTables, Lived, Loaded, Result, RootSeed, Roots, Step,
    Structure, Vec3,
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
    /// The device's failure, by name, which the engine reports as words.
    failure: Option<crate::RenderError>,
}

impl<'a> Device<'a> {
    pub(super) fn new(gpu: &'a Gpu, k: &'a Kernels, tables: &'a Tables, batch: usize) -> Self {
        let mut device = Self::unbuilt(gpu, k, batch);
        device.given = Some(tables);
        device
    }

    #[cfg(test)]
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
            failure: None,
        }
    }

    /// The failure that stopped a phase, as the device named it.
    pub(super) fn failure(&mut self) -> Option<crate::RenderError> {
        self.failure.take()
    }

    /// `result`, its failure kept by name and handed to the engine.
    fn kept<T>(&mut self, result: crate::Result<T>) -> Result<T> {
        result.map_err(|e| {
            let words = telperion_space::Error::Executor(e.to_string());
            self.failure.get_or_insert(e);
            words
        })
    }

    fn tables(&self) -> &Tables {
        self.given
            .or(self.built.as_ref())
            .expect("the tables, built")
    }

    /// The geometry table on the device (`laid::table`).
    pub(super) fn table(&self) -> &wgpu::Buffer {
        &self.tables().table
    }

    /// The tables, uploaded once built.
    fn ready(&mut self) -> crate::Result<()> {
        if let Some(building) = self.building.take() {
            let (chain, geo) = building.join().expect("the tables");
            self.built = Some(Tables::new(self.gpu, Arc::new(chain), &geo)?);
        }
        Ok(())
    }

    #[cfg(test)]
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

    #[cfg(test)]
    /// Waits for the last phase's passes, which the driver's own landing
    /// overlaps (task 8).
    pub(super) fn wait(&self) {
        self.gpu
            .device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();
    }

    /// `seeds` grown as one group, taken in `order`.
    async fn group(&mut self, seeds: &[&RootSeed], order: Vec<usize>) -> crate::Result<()> {
        let start = Clock::now();
        self.ready()?;
        let roots: Vec<RootBud> = seeds
            .iter()
            .map(|s| (s.bud, [s.made.0 as f32, s.made.1 as f32], s.birth))
            .collect();
        let nodes = seeds.iter().map(|s| s.bud.node).collect();
        let (gpu, k, batch) = (self.gpu, self.k, self.batch);
        let mut cost = std::mem::take(&mut self.cost);
        let t = self.tables();
        let group = Group::grow((gpu, k), (&roots, nodes), (order, batch), (t, &mut cost)).await;
        self.cost = cost;
        self.cost.grow += start.elapsed_ms();
        self.groups.push(group?);
        Ok(())
    }

    /// Each group's records read back in one round trip, by root.
    async fn records(&mut self) -> crate::Result<Vec<[f32; CR as usize]>> {
        self.cost.trips += 1;
        let gpu = self.gpu;
        let roots: u64 = self.roots.iter().map(|r| r.len() as u64).sum();
        let total = roots * u64::from(CR) * 4;
        let usage = wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let into = io::buffer(gpu, "records gathered", total.max(16), usage, &[])?;
        let mut encoder = gpu.encoder(&Default::default());
        let mut at = Vec::with_capacity(self.groups.len());
        let mut bytes = 0;
        for g in &self.groups {
            at.push(bytes as usize / 4);
            bytes += g.gather(&mut encoder, &into, bytes);
        }
        gpu.submit([encoder.finish()]);
        let raw = io::read_async(gpu, &into, total.max(16)).await?;
        let floats: &[f32] = bytemuck::cast_slice(&raw);
        let by_group: Vec<_> = self
            .groups
            .iter()
            .zip(at)
            .map(|(g, at)| g.records(&floats[at..]))
            .collect();
        Ok(self.places.iter().map(|&(g, i)| by_group[g][i]).collect())
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
    fn passes(&self, entry: usize, down: bool) -> crate::Result<()> {
        let t = self.tables();
        for g in &self.groups {
            g.attach(self.gpu);
            g.pass((self.gpu, self.k), t, entry, down)?;
        }
        Ok(())
    }

    /// Phase 2 on the device: each root's axes kept from `live`, scaled
    /// by `scales`, thickened, and the records read back.
    async fn girths(
        &mut self,
        live: &[usize],
        scales: &[(f64, f64)],
    ) -> crate::Result<Vec<[f32; CR as usize]>> {
        let (gpu, k) = (self.gpu, self.k);
        for (g, on) in self.groups.iter().zip(self.on(live)) {
            g.shed((gpu, k), &on)?;
        }
        for (&r, &(base, drawn)) in live.iter().zip(scales) {
            let a = self.attached(r);
            a.unbent = unlaid(base * drawn);
            a.bent = a.unbent;
        }
        let t = self.tables();
        for g in &self.groups {
            g.attach(gpu);
            g.pass((gpu, k), t, SCALES, true)?;
            g.pass((gpu, k), t, GIRTHS, false)?;
        }
        self.records().await
    }

    /// Phase 3's passes, and the levers' where the wood sags.
    fn lays(&self) -> crate::Result<()> {
        let t = self.tables();
        for g in &self.groups {
            g.attach(self.gpu);
            g.pass((self.gpu, self.k), t, LAYS, true)?;
            if t.sags {
                g.pass((self.gpu, self.k), t, LEVERS, false)?;
            }
        }
        Ok(())
    }

    fn attached(&mut self, r: usize) -> &mut telperion_space::RootGeometry {
        let (g, i) = self.places[r];
        &mut self.groups[g].attached[i]
    }

    /// Root `r`'s bearer's node as laid under sag, which its positions
    /// are measured from (fn-212 task 4).
    pub(super) fn origin(&self, r: usize) -> telperion_space::Vec3 {
        let (g, i) = self.places[r];
        self.groups[g].attached[i].bent.tip
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
    /// Natively the device grows the roots made so far while the structure
    /// above the cut grows on. A browser cannot wait on its device there,
    /// so it sows none: every root grows in `grow`, the same seeds in the
    /// same order (host decision 1).
    #[cfg(not(target_arch = "wasm32"))]
    fn sow(&mut self, seeds: &[RootSeed]) -> Result<usize> {
        if !self.order.is_empty() || seeds.len() < self.least {
            return Ok(0);
        }
        let start = Clock::now();
        let all: Vec<&RootSeed> = seeds.iter().collect();
        let grown = pollster::block_on(self.group(&all, (0..all.len()).collect()));
        self.kept(grown)?;
        self.cost.sown += start.elapsed_ms();
        Ok(seeds.len())
    }

    fn grow<'a>(&'a mut self, seeds: &'a [RootSeed]) -> Step<'a, Result<(Vec<Lived>, u64)>> {
        Box::pin(async move {
            let start = Clock::now();
            // The roots sown are the first seeds, group by group; the
            // rest's after them.
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
                let grown = self.group(&group, order).await;
                self.kept(grown)?;
                roots.push(rest);
            }
            self.places = vec![(0, 0); seeds.len()];
            for (g, rs) in roots.iter().enumerate() {
                for (i, &r) in rs.iter().enumerate() {
                    self.places[r] = (g, i);
                }
            }
            self.roots = roots;
            // Each root's first axis's vigour as its bearer made it, a
            // woken root's as its bearer kept it, which phase 2 scales it
            // by.
            for (r, s) in seeds.iter().enumerate() {
                let (g, i) = self.places[r];
                self.groups[g].made[i] = times(s.birth[0], s.birth[1]);
            }
            let lived = (self.places.iter())
                .map(|&(g, i)| self.groups[g].lived[i])
                .collect();
            let grown = self.groups.iter().flat_map(|g| &g.waves.phyts);
            let grown = grown.map(|&p| u64::from(p)).sum();
            self.cost.last = start.elapsed_ms();
            Ok((lived, grown))
        })
    }

    fn girth<'a>(
        &'a mut self,
        live: &'a [usize],
        scales: &'a [(f64, f64)],
    ) -> Step<'a, Result<Vec<f64>>> {
        Box::pin(async move {
            let start = Clock::now();
            let read = self.girths(live, scales).await;
            let read = self.kept(read)?;
            self.cost.lay += start.elapsed_ms();
            Ok(live.iter().map(|&r| f64::from(read[r][0])).collect())
        })
    }

    fn lay<'a>(
        &'a mut self,
        tree: &'a Structure,
        live: &'a [usize],
        bearers: &'a [usize],
    ) -> Step<'a, Result<Vec<Loaded>>> {
        Box::pin(async move {
            let start = Clock::now();
            for (&r, &b) in live.iter().zip(bearers) {
                let frame = bearer_frame(tree, b, self.node(r));
                let a = self.attached(r);
                (a.bearer_pa, a.unbent, a.bent) = (tree.axes[b].pa, frame, frame);
            }
            let laid = self.lays();
            self.kept(laid)?;
            let loads = if self.tables().sags {
                let read = self.records().await;
                let read = self.kept(read)?;
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
            self.cost.lay += start.elapsed_ms();
            Ok(loads)
        })
    }

    fn bend<'a>(
        &'a mut self,
        tree: &'a Structure,
        live: &'a [usize],
        bearers: &'a [usize],
    ) -> Step<'a, Result<()>> {
        let start = Clock::now();
        for (&r, &b) in live.iter().zip(bearers) {
            let frame = bearer_frame(tree, b, self.node(r));
            self.attached(r).bent = frame;
        }
        // Not waited for: the CPU lands its roots meanwhile (`wait`).
        let bent = self.passes(BENDS, true);
        self.cost.lay += start.elapsed_ms();
        telperion_space::ready(self.kept(bent))
    }
}
