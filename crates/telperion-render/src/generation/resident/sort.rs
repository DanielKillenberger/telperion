//! A device sort of pairs of words (fn-261): what numbers a build's items
//! by a key where threads would otherwise number them in the order the
//! device runs them, which changes from run to run.
use super::super::io;
use crate::{Gpu, Result};

pub(super) const SORT: &str = include_str!("sort.wgsl");

pub(super) struct Sorter {
    pass: io::Pass,
}

impl Sorter {
    pub(super) fn new(gpu: &Gpu) -> Self {
        Self {
            pass: io::Pass::new(gpu, SORT.to_string(), &[false], &["step"]),
        }
    }

    /// Sorts the first `n` pairs of `keys` in `e`, ascending by their first
    /// word, then their second. `keys` holds `n` rounded up to a power of
    /// two pairs, those past `n` already set to `u32::MAX`.
    pub(super) fn sort(
        &self,
        gpu: &Gpu,
        e: &mut wgpu::CommandEncoder,
        keys: &wgpu::Buffer,
        n: u32,
    ) -> Result<()> {
        let wide = n.next_power_of_two();
        let groups = wide.div_ceil(64).max(1);
        let most = gpu.device.limits().max_compute_workgroups_per_dimension;
        let usage = wgpu::BufferUsages::UNIFORM;
        let mut k = 2;
        while k <= wide {
            let mut j = k / 2;
            while j > 0 {
                let words = [wide, groups.min(most) * 64, j, k];
                let cfg = io::buffer(gpu, "sort cfg", 16, usage, bytemuck::cast_slice(&words))?;
                let bind = self.pass.bind(gpu, &[&cfg, keys]);
                self.pass.run(e, &bind, 0, groups);
                j /= 2;
            }
            k *= 2;
        }
        Ok(())
    }
}
