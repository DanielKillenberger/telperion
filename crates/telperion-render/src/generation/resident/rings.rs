//! The tree's compact rings on the device (fn-212 task 6): the ring
//! words `compact::read` writes, from the packed curve `sweep.wgsl`
//! swept, so the positions pass lays the vertices a seated leaf's
//! contact reads.
use super::super::io;
use super::sweep::Swept;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const RINGS: &str = include_str!("rings.wgsl");
/// A ring's words (`compact::CompactSurface::rings`).
const RINGW: u64 = 16;

pub(super) fn source() -> String {
    format!("{}\n{RINGS}", super::sweep::PACK)
}

pub(super) struct Ringer {
    pass: io::Pass,
}

/// The fine block's ring words, its rings and the vertices they lay.
pub(super) struct Rung {
    pub buffer: Owned,
    pub rings: u32,
    pub vertices: u32,
}

impl Ringer {
    pub(super) fn new(gpu: &Gpu) -> Self {
        Self {
            pass: io::Pass::new(gpu, source(), &[true, true, false, true], &["rings_of"]),
        }
    }

    /// The ring words of `swept`'s curve (its run table and points),
    /// `segments` vertices a ring, its twist's phase `twist` a metre along.
    pub(super) fn rings(
        &self,
        gpu: &Gpu,
        swept: &Swept,
        segments: u32,
        twist: f32,
    ) -> Result<Rung> {
        let n = swept.runs;
        let groups = n.div_ceil(64).max(1);
        let most = gpu.device.limits().max_compute_workgroups_per_dimension;
        let words = [n, groups.min(most) * 64, segments, twist.to_bits()];
        let usage = wgpu::BufferUsages::UNIFORM;
        let cfg = io::buffer(gpu, "rings cfg", 16, usage, bytemuck::cast_slice(&words))?;
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let bytes = u64::from(swept.points).max(1) * RINGW * 4;
        let buffer = io::buffer(gpu, "fine rings", bytes, usage, &[])?;
        let all: [&wgpu::Buffer; 5] = [
            &cfg,
            &swept.table,
            &swept.point_buffer,
            &buffer,
            &swept.cluster_buffer,
        ];
        let bind = self.pass.bind(gpu, &all);
        let mut e = gpu.encoder(&Default::default());
        self.pass.run(&mut e, &bind, 0, groups);
        gpu.submit([e.finish()]);
        Ok(Rung {
            buffer,
            rings: swept.points,
            vertices: swept.points * segments + 2 * n,
        })
    }
}
