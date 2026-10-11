//! One cycle of the build in fn-255's order (`Grower::year`): the living
//! taken in, the balance and allotment over the hierarchy (once any axis
//! is laid), the units and the wake, the new axes' layout columns, then
//! before the tree's age the layout and the light.
use super::build::{Build, Kit, Tables};
use super::build_light::{refusals, Lit};
use super::glue::{self, Words};
use super::hier_device::Counts;
use super::hier_scan::{read_words, Scan};
use super::lay_device::{Laying, T_FAULT};
use crate::{Gpu, Result};

fn refused(what: String) -> crate::RenderError {
    super::super::io::error(what)
}

impl Build {
    fn glue(&self, gpu: &Gpu, kit: &Kit, e: &mut wgpu::CommandEncoder, (entry, aux, aux2): (usize, &crate::buffer::Owned, &crate::buffer::Owned), w: Words) -> Result<()> {
        let w = Words { live_stride: self.sums.live_stride, ..w };
        glue::run(gpu, &kit.glue, e, (&self.state, &self.sums.live), (aux, aux2), entry, w)
    }

    /// One cycle (`Grower::year`).
    pub(super) fn cycle(&mut self, gpu: &Gpu, kit: &Kit, t: &Tables, cycle: u32) -> Result<()> {
        let (live0, laid0, axes0, phyts0) = (self.held.next, self.laid, self.held.axes, self.held.phyts);
        let mut e = gpu.encoder(&Default::default());
        let w = Words { n: live0, ..Words::default() };
        self.glue(gpu, kit, &mut e, (glue::RELIVE, &self.spare, &self.flags), w)?;
        gpu.submit([e.finish()]);
        let live = match laid0 {
            0 => live0,
            _ => self.balance(gpu, kit, t, (cycle, live0))?,
        };
        let mut e = gpu.encoder(&Default::default());
        let w = Words { n: live.max(laid0), live, laid: laid0, from_in: laid0 == 0, ..Words::default() };
        self.glue(gpu, kit, &mut e, (glue::INPUTS, &self.inputs, &self.spare), w)?;
        gpu.submit([e.finish()]);
        let made = self.units(gpu, kit, t, (cycle, live))?;
        if made.faults != 0 {
            return Err(refused(format!("cycle {cycle}: the units faulted ({:#x})", made.faults)));
        }
        let caps = self.state.caps;
        let h = self.held;
        if h.axes > caps.axes || h.units > caps.units || h.phyts > caps.phyts || h.sleepers > caps.sleepers || h.next > caps.next || h.woken > caps.woken {
            return Err(refused(format!("cycle {cycle}: {h:?} past {caps:?}")));
        }
        let mut e = gpu.encoder(&Default::default());
        let w = Words { n: made.axes, first: axes0, ..Words::default() };
        self.glue(gpu, kit, &mut e, (glue::BORN, &self.spare, &self.flags), w)?;
        let w = Words { n: made.phyts, first: phyts0, ..Words::default() };
        self.glue(gpu, kit, &mut e, (glue::FRESH, &self.spare, &self.flags), w)?;
        gpu.submit([e.finish()]);
        if cycle < t.age {
            self.lay(gpu, kit, cycle, (live0, laid0))?;
            self.shade(gpu, kit, phyts0)?;
            self.laid = self.held.axes;
        }
        Ok(())
    }

    /// The balance and allotment over the laid axes (`balance_one`,
    /// `allot_one`): the hierarchy taken up to them, the sums, their
    /// words into the pencils; the living after, counted.
    fn balance(&mut self, gpu: &Gpu, kit: &Kit, t: &Tables, (cycle, live0): (u32, u32)) -> Result<u32> {
        let laid = self.laid;
        let mut e = gpu.encoder(&Default::default());
        let w = Words { n: laid, laid, ..Words::default() };
        self.glue(gpu, kit, &mut e, (glue::TREE, &self.tree, &self.spare), w)?;
        gpu.submit([e.finish()]);
        self.hier.grow(gpu, &kit.hier, &self.tree, laid)?;
        gpu.queue.write_buffer(&self.sums.rows, 0, bytemuck::cast_slice(&t.rows_at(cycle)));
        let counts = Counts { n: laid, cycle, pas: t.pas, live: live0 };
        self.sums.plan(gpu, &kit.hier, &self.hier, &self.tree, counts)?;
        let mut e = gpu.encoder(&Default::default());
        self.sums.record(&kit.hier, &self.tree, &mut e);
        let w = Words { n: laid, laid, cycle, ..Words::default() };
        self.glue(gpu, kit, &mut e, (glue::SCATTER, &self.sums.sums, &self.spare), w)?;
        gpu.submit([e.finish()]);
        Ok(read_words(gpu, &self.sums.tally, 0, 1)?[0])
    }

    /// The cycle's rough layout (`Grower::sketch`): the axes that grew
    /// listed by index and drawn, then the new axes by generation.
    fn lay(&mut self, gpu: &Gpu, kit: &Kit, cycle: u32, (live0, laid0): (u32, u32)) -> Result<()> {
        self.layout.ensure(gpu, self.state.caps.axes)?;
        let s = u64::from(self.state.caps.axes);
        let mut grown = 0;
        if laid0 > 0 {
            let mut e = gpu.encoder(&Default::default());
            e.clear_buffer(&self.flags, 0, None);
            let w = Words { n: live0, live: live0, laid: laid0, ..Words::default() };
            self.glue(gpu, kit, &mut e, (glue::GREW_MARK, &self.flags, &self.spare), w)?;
            let scan = Scan::new(gpu, &kit.hier.scan, (&self.flags, s, laid0), &self.scan_sums, (&self.scan_tally, 0))?;
            scan.record(&kit.hier.scan, &mut e);
            let w = Words { n: laid0, laid: laid0, ..Words::default() };
            self.glue(gpu, kit, &mut e, (glue::GREW_GATHER, &self.flags, &self.layout.lists), w)?;
            gpu.submit([e.finish()]);
            grown = read_words(gpu, &self.scan_tally, 0, 1)?[0];
        }
        let l = Laying { cycle, first_new: laid0, grown, new_axes: self.held.axes - laid0 };
        let mut e = gpu.encoder(&Default::default());
        self.layout.record_first(gpu, &kit.layout, &mut e, &self.state, l)?;
        gpu.submit([e.finish()]);
        if l.new_axes > 0 {
            let at = 16 + u64::from(grown);
            let gens = read_words(gpu, &self.layout.lists, at, u64::from(l.new_axes))?;
            let count = gens.iter().max().map_or(0, |g| g + 1);
            let mut e = gpu.encoder(&Default::default());
            for g in 0..count {
                self.layout.record_generation(gpu, &kit.layout, &mut e, &self.state, (l, g))?;
            }
            gpu.submit([e.finish()]);
        }
        let fault = read_words(gpu, &self.layout.lists, T_FAULT, 1)?[0];
        if fault != 0 {
            return Err(refused(format!("cycle {cycle}: the layout faulted ({fault:#x})")));
        }
        Ok(())
    }

    /// The cycle's light (`Grower::shade`): its leaves and the living
    /// apexes' tips gathered, the box read to size the sweep, the light
    /// swept and read into each living apex's pencil.
    fn shade(&mut self, gpu: &Gpu, kit: &Kit, phyts0: u32) -> Result<()> {
        let lit = Lit { leaves: self.held.phyts - phyts0, tips: self.held.next };
        self.light.ensure(gpu, lit, 0)?;
        let mut e = gpu.encoder(&Default::default());
        let w = Words { n: lit.leaves + lit.tips, first: phyts0, leaves: lit.leaves, ..Words::default() };
        self.glue(gpu, kit, &mut e, (glue::POINTS, &self.light.points, &self.spare), w)?;
        self.light.record_box(gpu, &kit.light, &mut e, lit)?;
        gpu.submit([e.finish()]);
        let (mut grid, mut errors) = self.light.grid(gpu)?;
        if grid[6] > self.light.nodes_cap {
            self.light.ensure(gpu, lit, grid[6])?;
            let mut e = gpu.encoder(&Default::default());
            self.light.record_box(gpu, &kit.light, &mut e, lit)?;
            gpu.submit([e.finish()]);
            (grid, errors) = self.light.grid(gpu)?;
        }
        if errors != 0 {
            return Err(refused(format!("the light refused: {:?}", refusals(errors))));
        }
        let mut e = gpu.encoder(&Default::default());
        self.light.record_rest(gpu, &kit.light, &mut e, lit, [grid[3], grid[4], grid[5]])?;
        let w = Words { n: lit.tips, ..Words::default() };
        self.glue(gpu, kit, &mut e, (glue::LIT, &self.light.reads, &self.spare), w)?;
        gpu.submit([e.finish()]);
        let (_, errors) = self.light.grid(gpu)?;
        if errors != 0 {
            return Err(refused(format!("the light refused: {:?}", refusals(errors))));
        }
        Ok(())
    }
}
