//! A build paused between calls (fn-270 R2): what the next cycle reads
//! stays on the device (the state, retirement's columns and records, the
//! hierarchy's shape, counts, lists and tally, the census); every buffer
//! a cycle writes before it reads is let go, as is the recorded cycle,
//! and both are made again, at the build's sizes, when it grows on: the
//! light's lattice and points, the units' job counts, the hierarchy's
//! buckets, sums and the lists its next step writes (it reads the others,
//! `plan_hier.rs`), the layout's lists, the compactor's scratch, and
//! retirement's verdicts where no test logs them. A buffer made again
//! starts zeroed, as one a move makes anew does (`build_move.rs`).
use super::build::{Build, Kit, Tables};
use super::build_grow::{Growing, Sizes};
use super::build_light::Light;
use super::census as c;
use super::columns::Cap;
use super::hier_build::{BUCKETS, LISTS};
use super::hier_device::Sums;
use super::hier_scan::{sums_words, zeroed};
use super::lay_device::Layout;
use super::retire_device::{zeroed_dst, Retire};
use super::{device, hier_columns};
use crate::{Gpu, Result};
use telperion_space::Capture;

/// The rows a let-go table keeps: none it is read by while paused.
const LEAST: u32 = 64;

impl Growing {
    /// The build paused: its counts read from the census as its last
    /// cycle and readying left it, its scratch and recorded cycle let go
    /// and the device waited on, so a settle's buffers are made in the
    /// room they held. A paused build is paused already.
    pub(super) async fn paused(
        mut self,
        gpu: &Gpu,
        (t, tables): (&Tables, &Capture),
    ) -> Result<Self> {
        let Some(run) = self.run.take() else {
            return Ok(self);
        };
        let words = super::super::io::read_async(gpu, &run.0.census, 4 * c::WORDS as u64).await?;
        self.b.counted(bytemuck::cast_slice(&words));
        // A paused build grows on: a settle keeps what its next cycle reads.
        self.b.goes_on = true;
        drop(run);
        self.b = self.b.scratch(gpu, (t, tables), None)?;
        super::super::io::complete(gpu).await?;
        Ok(self)
    }

    /// The build ready to grow on: its scratch made again at its sizes and
    /// its cycle recorded again over it, the census as the pause left it,
    /// in the room a draw's scratch let go held (fn-270 R4: the device
    /// waited on first, as a pause waits). The seconds this took are the
    /// build's `rebuilt`.
    pub(super) async fn resumed(
        mut self,
        gpu: &Gpu,
        kit: &Kit,
        t: &Tables,
        tables: &Capture,
    ) -> Result<Self> {
        if self.run.is_some() {
            return Ok(self);
        }
        super::super::io::complete(gpu).await?;
        let clock = super::super::Clock::now();
        self.b = self.b.scratch(gpu, (t, tables), Some(self.sizes))?;
        let run = self.b.plan(gpu, kit, t, self.sizes.bounds)?;
        gpu.queue
            .write_buffer(&run.0.census, 0, bytemuck::cast_slice(&self.b.census));
        self.run = Some(run);
        self.grew.rebuilt += clock.elapsed_ms() / 1e3;
        Ok(self)
    }
}

impl Build {
    /// Every buffer a cycle writes before it reads made for `sizes`, or
    /// the least where none: the tables it keeps between calls kept.
    fn scratch(
        mut self,
        gpu: &Gpu,
        (t, tables): (&Tables, &Capture),
        sizes: Option<Sizes>,
    ) -> Result<Build> {
        let caps = sizes.map_or(Sizes::least().caps, |s| s.caps);
        let (s, next) = match sizes {
            Some(z) => (z.caps.axes, z.caps.next),
            None => (LEAST, LEAST),
        };
        let w = u64::from(s);
        self.light = Light::new(gpu, &Cap(tables))?;
        if let Some(z) = sizes {
            let (lit, nodes) = z.lit();
            self.light.ensure(gpu, lit, nodes)?;
        }
        self.hier.bucket = zeroed(gpu, "hierarchy buckets", BUCKETS * w)?;
        // Cycle `c`'s step, of parity `c % 2`, wrote the other lists; the
        // next writes these.
        let written = self.age() as usize % 2;
        let label = ["hierarchy lists a", "hierarchy lists b"][written];
        self.hier.lists[written] = zeroed(gpu, label, sizes.map_or(4, |_| LISTS * (w + 64)))?;
        self.hier.sums = zeroed(gpu, "hierarchy scan sums", sums_words(s + 64, 1))?;
        self.sums = Sums::alloc(gpu, s, next, t.pas)?;
        self.tree = zeroed(gpu, "cycle tree", u64::from(hier_columns::TREE as u32 * s))?;
        self.layout = Layout::new(gpu, &t.form, s)?;
        self.compactor = super::compact::Compactor::new(gpu, caps, s)?;
        self.inputs = zeroed(gpu, "cycle inputs", 2 * u64::from(next) + 7 * w)?;
        self.counts = zeroed(
            gpu,
            "cycle counts",
            sizes.map_or(4, |_| device::counts(caps, t.zones).words),
        )?;
        self.flags = zeroed(gpu, "cycle flags", 2 * w)?;
        self.spare = zeroed(gpu, "cycle spare", 4)?;
        self.scan_tally = zeroed(gpu, "cycle scan tally", 64)?;
        self.scan_sums = zeroed(gpu, "cycle scan sums", sums_words(s, 1))?;
        let r = &mut self.retire;
        if r.log_cap == 0 {
            let s = sizes.map_or(0, |_| r.stride);
            r.verdicts = zeroed_dst(gpu, "retire verdicts", Retire::verdict_words(0, s))?;
        }
        Ok(self)
    }
}
