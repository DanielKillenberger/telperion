//! fn-229: the florets of a tree built resident, as the build takes its
//! leaves. They are packed against the tree's leaf box padded by the
//! farthest a floret stands from its node, and the device places them in
//! the submission that places its leaves, from the blooms its stations
//! counted and recorded.
use super::super::compute::{self, Placement};
use super::super::{data, io, Generator, Metrics, Resident};
use super::build::union;
use super::stations::Stationed;
use crate::buffer::{Held, Owned};
use crate::Result;
use telperion_core::foliage::{Element, Instances, Reference};
use telperion_core::mesh::Foliage;
use telperion_core::pipeline::executor::Expansion;

/// The organ of a flowering tree: its element, the box its florets are
/// packed against, and the rows the device draws them by.
pub(super) struct Organ {
    element: Element,
    within: Reference,
    rows: [[f32; 4]; 3],
    /// The pedicels' tube radius, the stamen's half width.
    tube: f64,
}

/// The organ buffer the device places into, and its placement summary.
pub(super) struct Placed {
    buffer: Owned,
    count: u32,
    summary: Option<Owned>,
}

impl Organ {
    /// The organ of `x`, packed against the tree's leaf box padded by a
    /// floret's reach.
    pub(super) fn of(x: &Expansion) -> Result<Self> {
        let (bloom, now, _) = x.bloom();
        let within = telperion_core::foliage::bloom_box(x.reference(), &bloom);
        Ok(Self {
            element: x.organ_element()?,
            within,
            rows: data::flower(&bloom, now),
            tube: 0.5 * bloom.stamen_width,
        })
    }

    /// The device's `stationed` florets recorded into `encoder`, into an
    /// organ buffer of their own. None where the tree keeps no floret.
    pub(super) fn place(
        &self,
        gen: &Generator,
        encoder: &mut wgpu::CommandEncoder,
        stationed: &Stationed,
    ) -> Result<Option<Placed>> {
        let gpu = &gen.gpu;
        let count = stationed.florets;
        if count == 0 {
            return Ok(None);
        }
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let buffer = io::buffer(gpu, "generated florets", u64::from(count) * 12, usage, &[])?;
        let mut summary = None;
        if let Some(records) = &stationed.records {
            let counts = (stationed.florets, stationed.blooms);
            let config = data::florets((self.rows, self.tube), counts, &self.element, self.within);
            // No ring is read: the bindings take the records again.
            let records = (&**records, &**records);
            let at = (stationed.florets, &*buffer);
            let e = &self.element;
            let placed = gen.record_place(encoder, Placement::Florets, config, records, e, at)?;
            summary = Some(placed.0);
        }
        Ok(Some(Placed {
            buffer,
            count,
            summary,
        }))
    }

    /// The organ the mesh carries: its element and box, its florets on
    /// the device.
    pub(super) fn foliage(&self) -> Foliage {
        Foliage {
            element: self.element.clone(),
            instances: Instances::new(self.within),
        }
    }

    /// The rows the device counts and places its florets by.
    pub(super) fn rows(&self) -> [[f32; 4]; 3] {
        self.rows
    }

    /// The placed organ, from its device block's `summary` (read with the
    /// leaves' in one readback): its bounds, the florets' origins and every
    /// vertex, and its mass grid, recorded into `encoder`.
    pub(super) fn resident(
        &self,
        gen: &Generator,
        encoder: &mut wgpu::CommandEncoder,
        (placed, summary): (Placed, Option<&[u32]>),
        m: &mut Metrics,
    ) -> Result<Resident> {
        let (mut crown, mut full) = (None, None);
        if let Some(s) = summary {
            if s[1] != 0 {
                let refused = telperion_core::Error::InvalidInput("GPU floret transform/reference");
                return Err(refused.into());
            }
            crown = union(crown, Some(compute::bounds(s, 2)));
            full = union(full, Some(compute::bounds(s, 8)));
        }
        let (buffer, count) = (placed.buffer, placed.count);
        let at = (&*buffer, count, crown, self.within);
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
