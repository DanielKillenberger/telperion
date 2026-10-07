//! The GPU executor under realized mechanics (fn-209 F1 task 6;
//! REALIZED.md, host decision 8, step 4): `telperion_space::Roots` on the
//! device. The structure above the cut settles between the phases on the
//! CPU (`telperion_space::settle`); each phase takes one record per root
//! down and gives one per root up, and nothing per chain or per node
//! crosses. Phase 1 grows every root whole and carries its living up;
//! phase 2 sheds, scales and thickens, its base radius up; phase 3 lays
//! and loads, its load up; phase 4 lays under the levers.
use super::super::io;
use super::laid::{stubs, Wave, BENDS, GIRTHS, LAYS, LEVERS, SCALES};
use super::realized::{grow_roots, RootBud, Waves};
use super::roots::{Cost, Kernels, Tables};
use super::shedding::{report, table};
use super::steps::{buffer, DESCRIBE};
use crate::Gpu;
use std::sync::Arc;
use std::time::Instant;
use telperion_space::{
    bearer_frame, BearerFrame, ChainTables, GeometryTables, Lived, Loaded, Result, RootGeometry,
    RootSeed, Roots, Structure, Vec3,
};

/// A chain's record (`settle.wgsl`'s `CR`).
const CR: u32 = 5;

/// What the device keeps between the phases.
struct Kept {
    waves: Waves,
    /// The stubs (the first wave's bearers), then each wave's laid buffer.
    laid: Vec<Wave>,
    /// Each root's origins' heights, then each wave's chains' records.
    records: wgpu::Buffer,
    at: Vec<u32>,
    /// Each root's attachment as the stubs take it, and its node.
    attached: Vec<RootGeometry>,
    nodes: Vec<usize>,
}

/// A species point's tables built on another core while the structure
/// above the cut grows: the chain's and the geometry's (task 8).
pub(super) type Building = std::thread::JoinHandle<(ChainTables, GeometryTables)>;

/// The GPU executor's roots, taken in `order`, `batch` chains a run.
pub(super) struct Device<'a> {
    gpu: &'a Gpu,
    k: &'a Kernels,
    given: Option<&'a Tables>,
    built: Option<Tables>,
    building: Option<Building>,
    order: Vec<usize>,
    batch: usize,
    pub cost: Cost,
    kept: Option<Kept>,
}

impl<'a> Device<'a> {
    pub(super) fn new(gpu: &'a Gpu, k: &'a Kernels, tables: &'a Tables, batch: usize) -> Self {
        Self {
            gpu,
            k,
            given: Some(tables),
            built: None,
            building: None,
            order: Vec::new(),
            batch,
            cost: Cost::default(),
            kept: None,
        }
    }

    /// The executor with its tables still `building`, uploaded when phase
    /// 1 first needs them.
    pub(super) fn building(gpu: &'a Gpu, k: &'a Kernels, building: Building, batch: usize) -> Self {
        Self {
            gpu,
            k,
            given: None,
            built: None,
            building: Some(building),
            order: Vec::new(),
            batch,
            cost: Cost::default(),
            kept: None,
        }
    }

    fn tables(&self) -> &Tables {
        self.given
            .or(self.built.as_ref())
            .expect("the tables, built")
    }

    /// Takes the roots in `order` (by default, forward).
    pub(super) fn ordered(mut self, order: Vec<usize>) -> Self {
        self.order = order;
        self
    }

    /// The waves as the device keeps them, and the laid buffers.
    pub(super) fn waves(&self) -> (&Waves, &[Wave]) {
        let kept = self.kept.as_ref().expect("grown");
        (&kept.waves, &kept.laid)
    }

    fn kept(&mut self) -> &mut Kept {
        self.kept.as_mut().expect("grown")
    }

    /// Writes each root's stub and origins from its attachment.
    fn attach(&self, kept: &Kept) {
        let (data, origins) = stubs(&kept.attached);
        let q = &self.gpu.queue;
        q.write_buffer(&kept.laid[0].buffer, 0, bytemuck::cast_slice(&data));
        q.write_buffer(&kept.records, 0, bytemuck::cast_slice(&origins));
    }

    /// Runs `entry` over every wave, down from the first (`down`) or up
    /// from the deepest.
    fn pass(&self, kept: &Kept, entry: usize, down: bool) {
        let (gpu, t) = (self.gpu, self.tables());
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        let n = kept.waves.grown.len();
        let waves: Vec<usize> = match down {
            true => (0..n).collect(),
            false => (0..n).rev().collect(),
        };
        for w in waves {
            let prior = kept.at.get(w + 1).copied().unwrap_or(0);
            for run in &kept.waves.grown[w].runs {
                let packed = (&run.axes, &run.phyts);
                let item = (&run.items, run.n, packed, &run.scratch);
                let at = (entry, prior, kept.at[w]);
                let laid = (&kept.laid[w], &kept.laid[w + 1]);
                let bufs = (&t.table, &kept.records);
                let how = (t.chain.age, t.sags);
                self.k
                    .resolver
                    .settle(gpu, &mut encoder, item, bufs, laid, how, at);
            }
        }
        gpu.queue.submit([encoder.finish()]);
    }

    /// The first wave's records read back: one record per root, by root.
    fn records(&mut self) -> Vec<[f32; CR as usize]> {
        self.cost.trips += 1;
        let kept = self.kept.as_ref().expect("grown");
        let (n, cr) = (self.order.len(), CR as usize);
        let from = kept.at[0] as usize;
        let bytes = ((from + n * cr) * 4) as u64;
        let raw = io::read(self.gpu, &kept.records, bytes.max(16)).unwrap();
        let floats: &[f32] = bytemuck::cast_slice(&raw);
        let mut out = vec![[0.0; CR as usize]; n];
        for (wi, &r) in self.order.iter().enumerate() {
            out[r].copy_from_slice(&floats[from + wi * cr..][..cr]);
        }
        out
    }
}

/// A root's attachment before its bearer is laid.
fn unlaid(scale: f64) -> BearerFrame {
    BearerFrame {
        below: Vec3::default(),
        tip: Vec3::default(),
        heading: Vec3::default(),
        side: Vec3::default(),
        rank: 0.0,
        scale,
        size: 1.0,
    }
}

impl Roots for Device<'_> {
    fn grow(&mut self, seeds: &[RootSeed]) -> Result<(Vec<Lived>, u64)> {
        let (gpu, k) = (self.gpu, self.k);
        let start = Instant::now();
        if let Some(building) = self.building.take() {
            let (chain, geo) = building.join().expect("the tables");
            self.built = Some(Tables::new(gpu, Arc::new(chain), &geo));
        }
        if self.order.len() != seeds.len() {
            self.order = (0..seeds.len()).collect();
        }
        let roots: Vec<RootBud> = seeds
            .iter()
            .map(|s| (s.bud, [s.made.0 as f32, s.made.1 as f32], s.birth))
            .collect();
        let t = self.tables();
        let (waves, cost) = grow_roots((gpu, k), (&roots, &self.order, self.batch), t);
        let fading = Instant::now();
        let fades = table(gpu, &t.chain);
        let living = k
            .fades
            .living(gpu, (&waves.grown, &waves.chains), (&fades, t.chain.age));
        self.cost = cost;
        let read = report(gpu, &living[0], seeds.len(), &mut self.cost);
        self.cost.fade = fading.elapsed().as_secs_f64() * 1e3;
        let mut lived = vec![(0, 0); seeds.len()];
        for (wi, &r) in self.order.iter().enumerate() {
            lived[r] = read[wi];
        }
        let grown = waves.phyts.iter().map(|&p| u64::from(p)).sum();
        // The laid buffers and the records, sized now: nothing more is
        // read back to size them.
        let n = seeds.len() as u32;
        let mut laid = vec![Wave::new(gpu, (n, n), &[])];
        let mut at = Vec::with_capacity(waves.chains.len());
        let mut floats = 2 * n;
        for (w, &chains) in waves.chains.iter().enumerate() {
            laid.push(Wave::new(gpu, (waves.own[w], waves.phyts[w]), &[]));
            at.push(floats);
            floats += CR * chains;
        }
        let records = buffer(gpu, "settle records", u64::from(floats));
        let attached = seeds
            .iter()
            .map(|s| RootGeometry {
                vigour: s.made.0,
                whorl: s.made.1,
                bearer_pa: 0,
                unbent: unlaid(0.0),
                bent: unlaid(0.0),
            })
            .collect();
        let nodes = seeds.iter().map(|s| s.bud.node).collect();
        self.kept = Some(Kept {
            waves,
            laid,
            records,
            at,
            attached,
            nodes,
        });
        self.cost.grow = start.elapsed().as_secs_f64() * 1e3;
        Ok((lived, grown))
    }

    fn girth(&mut self, live: &[usize], scales: &[(f64, f64)]) -> Vec<f64> {
        let (gpu, k) = (self.gpu, self.k);
        let start = Instant::now();
        let kept = self.kept.take().expect("grown");
        let mut on = vec![0u32; kept.attached.len()];
        for &r in live {
            on[r] = 1;
        }
        let usage = wgpu::BufferUsages::STORAGE;
        let bytes = (on.len().max(4) * 4) as u64;
        let on = io::buffer(gpu, "roots kept", bytes, usage, bytemuck::cast_slice(&on)).unwrap();
        k.fades.shed(gpu, (&kept.waves.grown, &kept.waves.own), &on);
        // Each chain described for geometry, in its wave.
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        for (w, g) in kept.waves.grown.iter().enumerate() {
            let none = buffer(gpu, "no buds", 4);
            for run in &g.runs {
                let lay = (&run.runoff, &g.places, &g.meta, &run.items);
                let hand = (&run.axes, &run.scratch, &none, &none);
                k.steps
                    .hand(gpu, &mut encoder, (run.n, w == 0, DESCRIBE), lay, hand);
            }
        }
        gpu.queue.submit([encoder.finish()]);
        let mut kept = kept;
        for (&r, &(base, drawn)) in live.iter().zip(scales) {
            kept.attached[r].unbent = unlaid(base * drawn);
            kept.attached[r].bent = kept.attached[r].unbent;
        }
        self.attach(&kept);
        self.pass(&kept, SCALES, true);
        self.pass(&kept, GIRTHS, false);
        self.kept = Some(kept);
        let read = self.records();
        self.cost.lay += start.elapsed().as_secs_f64() * 1e3;
        live.iter().map(|&r| f64::from(read[r][0])).collect()
    }

    fn lay(&mut self, tree: &Structure, live: &[usize], bearers: &[usize]) -> Result<Vec<Loaded>> {
        let start = Instant::now();
        let kept = self.kept();
        for (&r, &b) in live.iter().zip(bearers) {
            let frame = bearer_frame(tree, b, kept.nodes[r]);
            let a = &mut kept.attached[r];
            (a.bearer_pa, a.unbent, a.bent) = (tree.axes[b].pa, frame, frame);
        }
        let kept = self.kept.take().expect("grown");
        self.attach(&kept);
        self.pass(&kept, LAYS, true);
        let sags = self.tables().sags;
        if sags {
            self.pass(&kept, LEVERS, false);
        }
        let origins: Vec<Vec3> = live.iter().map(|&r| kept.attached[r].unbent.tip).collect();
        self.kept = Some(kept);
        let loads = if sags {
            let read = self.records();
            live.iter()
                .zip(&origins)
                .map(|(&r, &o)| {
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
        let kept = self.kept();
        for (&r, &b) in live.iter().zip(bearers) {
            kept.attached[r].bent = bearer_frame(tree, b, kept.nodes[r]);
        }
        let kept = self.kept.take().expect("grown");
        self.attach(&kept);
        self.pass(&kept, BENDS, true);
        self.gpu
            .device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();
        self.kept = Some(kept);
        self.cost.lay += start.elapsed().as_secs_f64() * 1e3;
        Ok(())
    }
}
