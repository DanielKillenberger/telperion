//! The records a compaction appends retired rows to (DESIGN.md section
//! 6), grown as they fill (geometrically, `build_size::GROWTH`): where a PA keeps shed pipes or leaf girth,
//! each retired phytomer's grown columns.
use super::build::Build;
use super::retire_device::zeroed_dst;
use super::state::GROWN;
use crate::{Gpu, Result};

impl Build {
    /// The records grown to hold `rows`, those held kept.
    pub(super) fn grow_records(&mut self, gpu: &Gpu, rows: u32) -> Result<()> {
        let r = &mut self.retire;
        if rows <= r.rec_cap {
            r.rec_n = rows;
            return Ok(());
        }
        let grown = (f64::from(r.rec_cap) * super::build_size::GROWTH).min(f64::from(u32::MAX)) as u32;
        let cap = rows.next_multiple_of(64).max(grown);
        r.rec_moves += 1;
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
