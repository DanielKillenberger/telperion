//! fn-229: the florets of a tree built resident, as fn-212's handoff
//! takes its leaves (host decisions D4, D5). Both blocks are packed against
//! one box, the tree's combined leaf box padded by the farthest a floret
//! stands from its node. The CPU block's florets go first in one organ
//! buffer. The device places its own after them, in the submission that
//! places its leaves, from the blooms its stations counted and recorded.
//! One mass grid covers both blocks.
use super::super::compute::{self, Placement};
use super::super::{data, io, Generator, Metrics, Resident};
use super::handoff::union;
use super::stations::Stationed;
use crate::buffer::{Held, Owned};
use crate::Result;
use telperion_core::foliage::{Instances, Reference};
use telperion_core::math::Vec3;
use telperion_core::mesh::Foliage;
use telperion_core::pipeline::executor::Expansion;
use telperion_core::surface::Bounds;

/// The organ of a flowering tree: the CPU block's florets packed against
/// the shared box, and the rows the device draws its own by.
pub(super) struct Organ {
    cpu: Foliage,
    rows: [[f32; 4]; 3],
}

/// The organ buffer with both blocks written into it (the device's still
/// placing), and the device block's placement summary.
pub(super) struct Placed {
    buffer: Owned,
    count: u32,
    summary: Option<Owned>,
}

impl Organ {
    /// The organ of `x`'s part, packed against the whole tree's leaf box
    /// padded by a floret's reach.
    pub(super) fn of(x: &Expansion) -> Result<Self> {
        let (bloom, now, reach) = x.bloom();
        let leaf = x.reference();
        let pad = Vec3::new(reach, reach, reach);
        let within = Reference::spanning(leaf.min - pad, leaf.min + leaf.extent + pad);
        Ok(Self {
            cpu: x.organ_in(within)?,
            rows: data::flower(&bloom, now),
        })
    }

    /// Both blocks into one organ buffer: the CPU's written, the device's
    /// `stationed` florets recorded into `encoder` after them. None where
    /// the tree keeps no floret.
    pub(super) fn place(
        &self,
        gen: &Generator,
        encoder: &mut wgpu::CommandEncoder,
        stationed: &Stationed,
    ) -> Result<Option<Placed>> {
        let gpu = &gen.gpu;
        let cpu = self.cpu.instances.len() as u32;
        let count = cpu
            .checked_add(stationed.florets)
            .ok_or(telperion_core::Error::ResourceLimit("floret budget"))?;
        if count == 0 {
            return Ok(None);
        }
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let words = bytemuck::cast_slice(&self.cpu.instances.leaves);
        let buffer = io::buffer(
            gpu,
            "generated florets",
            u64::from(count) * 12,
            usage,
            words,
        )?;
        let mut summary = None;
        if let Some(records) = &stationed.records {
            let counts = (stationed.florets, stationed.blooms);
            let config = data::florets(self.rows, counts, &self.cpu.element, self.reference());
            // No ring is read: the bindings take the records again.
            let records = (&**records, &**records);
            let at = (stationed.florets, &*buffer);
            let config = config.at(cpu);
            let e = &self.cpu.element;
            let placed = gen.record_place(encoder, Placement::Florets, config, records, e, at)?;
            summary = Some(placed.0);
        }
        Ok(Some(Placed {
            buffer,
            count,
            summary,
        }))
    }

    /// The box both blocks are packed against.
    pub(super) fn reference(&self) -> Reference {
        self.cpu.instances.reference
    }

    /// The organ the mesh carries: its element and box, its florets on
    /// the device.
    pub(super) fn foliage(&self) -> Foliage {
        Foliage {
            element: self.cpu.element.clone(),
            instances: Instances::new(self.reference()),
        }
    }

    /// The rows the device counts and places its florets by.
    pub(super) fn rows(&self) -> [[f32; 4]; 3] {
        self.rows
    }

    /// The placed organ, from its device block's `summary` (read with the
    /// leaves' in one readback): its bounds, the florets' origins and every
    /// vertex, and one mass grid over both blocks, recorded into `encoder`.
    pub(super) fn resident(
        &self,
        gen: &Generator,
        encoder: &mut wgpu::CommandEncoder,
        (placed, summary): (Placed, Option<&[u32]>),
        m: &mut Metrics,
    ) -> Result<Resident> {
        let mut crown = crate::submit::crown_of(&self.cpu);
        let mut full = (self.cpu.instances.bounds(&self.cpu.element)?).map(Bounds::from);
        if let Some(s) = summary {
            if s[1] != 0 {
                let refused = telperion_core::Error::InvalidInput("GPU floret transform/reference");
                return Err(refused.into());
            }
            crown = union(crown, Some(compute::bounds(s, 2)));
            full = union(full, Some(compute::bounds(s, 8)));
        }
        let (buffer, count) = (placed.buffer, placed.count);
        let at = (&*buffer, count, crown, self.reference());
        let masses = gen.record_mass(encoder, at, 0, m)?;
        Ok(Resident {
            leaves: Held::resident(buffer, u64::from(count) * 12, "generated florets"),
            masses: Held::resident(masses.0, masses.1, "generated floret masses"),
            count,
            crown,
            full,
        })
    }
}

impl Placed {
    /// The device block's placement summary, none where it placed none.
    pub(super) fn summary(&self) -> Option<&Owned> {
        self.summary.as_ref()
    }
}
