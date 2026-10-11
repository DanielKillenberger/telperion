//! fn-256 task 2: the whole grower on the device from the seed, test-only
//! (DESIGN.md section 2). The state (`state.rs`) is seeded as
//! `Grower::seeded` seeds it, then each cycle runs fn-255's phases over it
//! in order: the balance and the allotment over the hierarchy, the
//! units, the wake, and, before the tree's age, the layout and the light,
//! which writes each living apex's light into its pencil. Retirement is
//! not run: it leaves the tree as it was (DESIGN.md section 6).
use super::build_light::Light;
use super::columns::{packed, Cap};
use super::hier_build::{Hierarchy, CAP};
use super::hier_device::Sums;
use super::hier_scan::{read_words, zeroed};
use super::lay_device::Layout;
use super::state::{Caps, Held, State};
use super::{device, glue, hier_build, lay_device, light_device, Arith};
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_space::{Capture, ChainTables};

/// Every kernel the build runs, built once under one arithmetic.
pub(super) struct Kit {
    pub units: device::Kernels,
    pub layout: lay_device::Kernels,
    pub hier: hier_build::Kernels,
    pub light: light_device::Kernels,
    pub glue: glue::Kernels,
}

impl Kit {
    pub(super) fn new(gpu: &Gpu, arith: Arith) -> Self {
        Self {
            units: device::Kernels::new(gpu, arith),
            layout: lay_device::Kernels::new(gpu, arith),
            hier: hier_build::Kernels::new(gpu, arith),
            light: light_device::Kernels::new(gpu, arith),
            glue: glue::Kernels::new(gpu),
        }
    }
}

/// The species' tables and the request, as `grow_capture_tables` takes
/// them: the units' tables and offsets, the layout's form, each PA's
/// balance constants and its whole stake's window per cycle.
pub(super) struct Tables {
    pub age: u32,
    pub seed: u64,
    pub lights: u32,
    pub rows: u32,
    pub whorls: u32,
    pub pas: u32,
    pub table: Owned,
    pub words: Owned,
    pub form: Vec<f32>,
    pa: Vec<f32>,
    windows: Vec<f32>,
}

impl Tables {
    pub(super) fn new(gpu: &Gpu, c: &Capture, chain: &ChainTables) -> Result<Self> {
        let cap = Cap(c);
        let [_, age, _] = cap.u32s("cycle")[..] else {
            panic!("cycle, age, budget")
        };
        let (table, words, rows, whorls) = packed(&cap, chain, age);
        let mut form = cap.f32s("lay_globals");
        form.extend(cap.f32s("lay_form"));
        let pa = cap.f32s("hier_pa");
        let pas = (pa.len() / 5) as u32;
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let upload = |label, bytes: &[u8]| super::super::io::buffer(gpu, label, bytes.len() as u64, usage, bytes);
        Ok(Self {
            age,
            seed: cap.u64s("seed")[0],
            lights: cap.u32s("lights")[0],
            rows,
            whorls,
            pas,
            table: upload("cycle table", bytemuck::cast_slice(&table))?,
            words: upload("cycle words", bytemuck::cast_slice(&words))?,
            form,
            pa,
            windows: cap.f32s("pa_windows"),
        })
    }

    /// Each PA's row the balance reads in `cycle` (`hier_cycle.wgsl`'s
    /// `P_`): upkeep, hazard, tolerance, the whole stake's window, apical
    /// control and ψ.
    pub(super) fn rows_at(&self, cycle: u32) -> Vec<f32> {
        let span = self.age as usize + 2;
        (0..self.pas as usize)
            .flat_map(|p| {
                let r = &self.pa[5 * p..5 * p + 5];
                [r[0], r[1], r[2], self.windows[p * span + cycle as usize], r[3], r[4], 0.0, 0.0]
            })
            .collect()
    }
}

/// The build on the device: the state, the hierarchy and the light's
/// buffers, and the counts each cycle leaves.
pub(super) struct Build {
    pub state: State,
    pub held: Held,
    /// The axes laid (with pencils), and the phytomers grown.
    pub laid: u32,
    pub grown: u64,
    pub hier: Hierarchy,
    pub sums: Sums,
    pub tree: Owned,
    pub light: Light,
    pub layout: Layout,
    /// The units' inputs, the glue's flags and a spare binding.
    pub inputs: Owned,
    pub flags: Owned,
    pub spare: Owned,
    pub scan_tally: Owned,
    pub scan_sums: Owned,
}

impl Build {
    /// The state seeded at `caps` (`Grower::seeded`).
    pub(super) fn new(gpu: &Gpu, kit: &Kit, t: &Tables, (c, caps): (&Capture, Caps)) -> Result<Self> {
        let state = State::new(gpu, caps)?;
        let s = caps.axes;
        let build = Self {
            hier: Hierarchy::new(gpu, s, CAP)?,
            sums: Sums::alloc(gpu, s, caps.next, t.pas)?,
            tree: zeroed(gpu, "cycle tree", u64::from(super::hier_columns::TREE as u32 * s))?,
            light: Light::new(gpu, &Cap(c))?,
            layout: Layout::new(gpu, &t.form, s)?,
            inputs: zeroed(gpu, "cycle inputs", 2 * u64::from(caps.next) + 6 * u64::from(s))?,
            flags: zeroed(gpu, "cycle flags", 2 * u64::from(s))?,
            spare: zeroed(gpu, "cycle spare", 4)?,
            scan_tally: zeroed(gpu, "cycle scan tally", 64)?,
            scan_sums: zeroed(gpu, "cycle scan sums", super::hier_scan::sums_words(s))?,
            state,
            held: Held { axes: 1, next: 1, ..Held::default() },
            laid: 0,
            grown: 0,
        };
        build.seed(gpu, kit, t)?;
        Ok(build)
    }

    /// The seed's axis and its apex, as the cycle before the first leaves
    /// them; refused where the seed grows in no age.
    fn seed(&self, gpu: &Gpu, kit: &Kit, t: &Tables) -> Result<()> {
        let mut e = gpu.encoder(&Default::default());
        let (bind, _held) = self.units_bind(gpu, kit, t, self.units_words((gpu, t), (0, 0), (1, 0, 0)))?;
        kit.units.cycle.run(&mut e, &bind, device::SEED, 1);
        let w = glue::Words { n: 1, first: 0, ..glue::Words::default() };
        let aux = (&self.spare, &self.flags);
        glue::run(gpu, &kit.glue, &mut e, (&self.state, &self.sums.live), aux, glue::BORN, w)?;
        gpu.submit([e.finish()]);
        let collapsed = read_words(gpu, &self.state.lists, 20, 1)?[0];
        if collapsed != 0 {
            return Err(super::super::io::error("the seed grows in no age (Error::Collapsed)".into()));
        }
        Ok(())
    }

    /// Every cycle of the tree's age.
    pub(super) fn run(&mut self, gpu: &Gpu, kit: &Kit, t: &Tables) -> Result<()> {
        for cycle in 1..=t.age {
            self.cycle(gpu, kit, t, cycle)?;
        }
        Ok(())
    }
}
