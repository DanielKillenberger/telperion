//! fn-256 task 2 (DESIGN.md section 6, compaction at a move): a moved
//! build's phytomer tables filled from the old ones with the retired
//! subtrees' rows left out, test-only. Each row is flagged kept or
//! recorded and placed by a stable scan; the kept rows move into the new
//! tables, the recorded rows' grown columns into the records (where a PA
//! keeps them), each unit record's first phytomer is remapped, and the
//! census's phytomers become the kept rows'. Where rows are recorded the
//! records' size is read back first: the one wait a compaction adds.
use super::build::{Build, Kit};
use super::census as c;
use super::hier_scan::{sums_words, uniform, widest, words as read, Scan};
use super::retire_device::{self as rd, zeroed_dst};
use super::state::GROWN;
use crate::{Gpu, Result};

impl Build {
    /// The stashed old tables compacted into the state's, `census` the
    /// plan's.
    pub(super) async fn compact(&mut self, gpu: &Gpu, kit: &Kit, census: &wgpu::Buffer) -> Result<()> {
        let (grown, laid, old) = self.stashed.take().expect("a move's old tables");
        let caps = self.state.caps;
        let keeps = self.retire.keeps();
        let ss = (old + 1).next_multiple_of(64);
        let scan = zeroed_dst(gpu, "compaction scan", 2 * u64::from(ss))?;
        let (sums, tally) = (zeroed_dst(gpu, "compaction sums", sums_words(ss))?, zeroed_dst(gpu, "compaction tally", 64)?);
        let row = widest(gpu).0;
        let base = self.retire.rec_n;
        let stride = self.retire.stride;
        let words = |rec_cap: u32| [old, caps.phyts, stride, ss, rec_cap, base, u32::from(keeps), c::PHYTS as u32, row, 0, 0, 0];
        let mut e = gpu.encoder(&Default::default());
        let flags = uniform(gpu, &words(1))?;
        let r = &self.retire;
        let bind = kit.retire.pack.bind(gpu, &[&flags, census, &grown, &laid, &r.ret, &self.state.grown, &self.state.laid, &r.recs, &scan]);
        kit.retire.pack.run(&mut e, &bind, rd::FLAG, (old + 1).div_ceil(64));
        let scans: Vec<Scan> = (0..2u64)
            .map(|k| Scan::new(gpu, &kit.hier.scan, (&scan, k * u64::from(ss), old + 1), &sums, (&tally, 0)))
            .collect::<Result<_>>()?;
        for s in &scans {
            s.record(&kit.hier.scan, &mut e);
        }
        gpu.submit([e.finish()]);
        if keeps {
            let dropped = read(gpu, &scan, u64::from(ss + old), 1).await?[0];
            self.grow_records(gpu, base + dropped)?;
        }
        let mut e = gpu.encoder(&Default::default());
        let r = &self.retire;
        let places = uniform(gpu, &words(r.rec_cap.max(1)))?;
        let bind = kit.retire.pack.bind(gpu, &[&places, census, &grown, &laid, &r.ret, &self.state.grown, &self.state.laid, &r.recs, &scan]);
        kit.retire.pack.run(&mut e, &bind, rd::SCATTER, old.div_ceil(64).max(1));
        let units_words = [caps.units, caps.axes, r.stride, ss, base, u32::from(keeps), c::UNITS as u32, row];
        let cfg = uniform(gpu, &units_words)?;
        let bind = kit.retire.units.bind(gpu, &[&cfg, census, &r.ret, &scan, &self.state.units, &self.state.axes]);
        kit.retire.units.run(&mut e, &bind, 0, caps.units.div_ceil(64).max(1));
        e.copy_buffer_to_buffer(&scan, 4 * u64::from(old), census, 4 * c::PHYTS as u64, 4);
        gpu.submit([e.finish()]);
        Ok(())
    }

    /// The records grown to hold `rows`, those held kept.
    fn grow_records(&mut self, gpu: &Gpu, rows: u32) -> Result<()> {
        let r = &mut self.retire;
        if rows <= r.rec_cap {
            r.rec_n = rows;
            return Ok(());
        }
        let cap = rows.next_multiple_of(64).max(r.rec_cap * 2);
        let new = zeroed_dst(gpu, "retire records", GROWN * u64::from(cap))?;
        if r.rec_n > 0 {
            let mut e = gpu.encoder(&Default::default());
            for k in 0..GROWN {
                let (a, b) = (4 * k * u64::from(r.rec_cap), 4 * k * u64::from(cap));
                e.copy_buffer_to_buffer(&r.recs, a, &new, b, 4 * u64::from(r.rec_n));
            }
            gpu.submit([e.finish()]);
        }
        (r.recs, r.rec_cap, r.rec_n) = (new, cap, rows);
        Ok(())
    }
}
