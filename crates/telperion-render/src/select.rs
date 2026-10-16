//! Which level every leaf is drawn at, decided on the GPU once per frame.
//!
//! One compute thread per placement projects the element's deviation at that
//! leaf's depth and takes the coarsest level that stays under half a pixel; a
//! leaf whose sphere is outside the frustum takes the unseen bucket instead.
//! Each level's list is compacted in placement-index order, so ties in depth
//! resolve identically on every draw, and the levels lie end to end in one
//! list, each from its own start. Its count drives one indexed indirect draw.
//!
//! Nothing survives the frame. There is no hysteresis and no previous-frame
//! state: the switch is below a pixel by construction, so there is nothing to
//! smooth, and a camera cut or a fresh tree needs nothing forgotten.
mod bind;
pub(crate) mod frame;

use std::ops::Range;

use telperion_core::mesh;

use crate::{
    buffer::{self, Held, Region},
    device::Gpu,
    Camera,
};

/// Threads per workgroup, matching `select.wgsl`'s membership bitset.
const WORKGROUP: u32 = 256;

/// The most levels one element may be selected from. The per-workgroup tally
/// lives in workgroup memory, which is sized when the shader is compiled; the
/// core's ladder doubles, so sixteen levels would be an element of tens of
/// thousands of triangles and no leaf is that.
pub const MAX_LEVELS: usize = 16;

/// An indexed indirect draw is five words: index count, instance count, first
/// index, base vertex, first instance.
const ARGUMENT_WORDS: usize = 5;

/// Which level the crown is drawn at. `Chosen` is the per-frame decision this
/// module exists for; `Forced` holds every leaf the frame shows at one level,
/// which is how a measurement asks what a level costs. Forcing still runs the
/// pass, so what is measured includes deciding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Level {
    #[default]
    Chosen,
    Forced(u32),
}

/// What the selection buffers cost for a crown of this shape. Shared with the
/// fit check, so what is judged before the upload is what the upload takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sizes {
    /// Bytes between two levels' start words, so each can be bound at a
    /// dynamic offset.
    pub slot: u64,
    /// Every level's list end to end: each leaf is in at most one level, so
    /// the crown fills it once.
    pub lists: u64,
    pub starts: u64,
    pub counts: u64,
    pub arguments: u64,
    pub scratch: u64,
}

/// The bytes each selection buffer needs for this many placements at this many
/// levels. The unseen bucket is a counter like any other, so there is one more
/// counter than there are levels.
pub fn sizes(instances: usize, levels: usize, alignment: u64) -> Sizes {
    let slot = alignment.max(1);
    Sizes {
        slot,
        lists: ((instances * size_of::<u32>()) as u64).div_ceil(slot) * slot,
        starts: slot * levels as u64,
        counts: ((levels + 1) * size_of::<u32>()) as u64,
        arguments: (levels * ARGUMENT_WORDS * size_of::<u32>()) as u64,
        scratch: ((instances + (levels + 1) * instances.div_ceil(WORKGROUP as usize))
            * size_of::<u32>()) as u64,
    }
}

/// The compute pipeline, the buffers it fills, and the bind groups the foliage
/// pass reads them back through.
pub struct Select {
    pipelines: [wgpu::ComputePipeline; 3],
    compute_layout: wgpu::BindGroupLayout,
    draw_layout: wgpu::BindGroupLayout,
    uniforms: crate::buffer::Owned,
    /// The leaf view: one placement at identity and a list of one, bound
    /// through the same layout as the crown and never rewritten.
    leaf: wgpu::BindGroup,
    placements: Option<Held>,
    masses: Option<Held>,
    deviations: Option<Held>,
    lists: Option<Held>,
    starts: Option<Held>,
    counts: Option<Held>,
    arguments: Option<Held>,
    scratch: Option<Held>,
    compute: Option<wgpu::BindGroup>,
    draw: Option<wgpu::BindGroup>,
    /// The indirect blocks as each frame starts them: the level's own index
    /// range, and no instances until the pass counts them.
    reset: Vec<u32>,
    /// Each level's range in the element's shared index buffer, coarsest first.
    levels: Vec<Range<u32>>,
    instances: u32,
    /// Bytes between two levels' start words (`Sizes::slot`).
    slot: u32,
    forced: Level,
    /// The element's bounding sphere at the origin: centre, then radius.
    sphere: [f32; 4],
    /// The box the submitted crown's positions were quantised against.
    reference: telperion_core::foliage::Reference,
    /// The organ's rachis and pedicel lengths and the rachis's droop
    /// (`frame::Uniforms::stalk`); zero on leaves.
    stalk: [f32; 3],
    /// How many of the element's levels, coarsest first, are the rachis's.
    rachis_levels: u32,
}

impl Select {
    pub(crate) fn allocated_bytes(&self) -> u64 {
        [
            &self.placements,
            &self.masses,
            &self.deviations,
            &self.lists,
            &self.starts,
            &self.counts,
            &self.arguments,
            &self.scratch,
        ]
        .into_iter()
        .flatten()
        .map(|b| b.region().capacity())
        .sum::<u64>()
    }

    pub fn new(gpu: &Gpu) -> Self {
        let compute_layout = bind::compute_layout(gpu);
        let draw_layout = bind::draw_layout(gpu);
        Self {
            pipelines: bind::pipelines(gpu, &compute_layout),
            uniforms: crate::buffer::Owned::create(
                gpu,
                &wgpu::BufferDescriptor {
                    label: Some("selection"),
                    size: size_of::<frame::Uniforms>() as u64,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                },
            ),
            leaf: bind::leaf_group(gpu, &draw_layout),
            compute_layout,
            draw_layout,
            placements: None,
            masses: None,
            deviations: None,
            lists: None,
            starts: None,
            counts: None,
            arguments: None,
            scratch: None,
            compute: None,
            draw: None,
            reset: Vec::new(),
            levels: Vec::new(),
            instances: 0,
            slot: 0,
            forced: Level::default(),
            sphere: [0.0; 4],
            stalk: [0.0; 3],
            rachis_levels: 0,
            reference: telperion_core::foliage::Reference::default(),
        }
    }

    /// The organ's rachis and pedicel lengths, which size its
    /// inflorescences, and the rachis's droop, which lays it (fn-253); zero
    /// on the leaves, where no instance is a rachis.
    pub(crate) fn set_stalk(&mut self, rachis: f32, pedicel: f32, droop: f32) {
        self.stalk = [rachis, pedicel, droop];
    }

    /// Lets the crown's buffers go, back to the state before any crown.
    pub(crate) fn release(&mut self) {
        for held in [
            &mut self.placements,
            &mut self.masses,
            &mut self.deviations,
            &mut self.lists,
            &mut self.starts,
            &mut self.counts,
            &mut self.arguments,
            &mut self.scratch,
        ] {
            *held = None;
        }
        (self.compute, self.draw) = (None, None);
        (self.reset, self.levels, self.instances) = (Vec::new(), Vec::new(), 0);
    }

    /// The layout the foliage pipeline reads its placements and its level's
    /// list through, which is the same one the bind groups here are built to.
    pub fn draw_layout(&self) -> &wgpu::BindGroupLayout {
        &self.draw_layout
    }

    /// Takes the crown: the placements as a storage buffer, the levels'
    /// deviations, and room for the lists, counters and indirect blocks one
    /// frame fills. A crown with no leaves, or an element with no levels,
    /// takes nothing at all and selects nothing.
    /// `element` is the one drawn, which is the foliage's own or the floret
    /// with its card in front (`card.rs`).
    pub fn submit(
        &mut self,
        gpu: &Gpu,
        foliage: &mesh::Foliage,
        element: &telperion_core::foliage::Element,
        level: Level,
    ) {
        if !foliage.instances.is_empty() && !element.levels.is_empty() {
            buffer::write(
                gpu,
                &mut self.placements,
                "foliage placements",
                wgpu::BufferUsages::STORAGE,
                bytemuck::cast_slice(&foliage.instances.leaves),
            );
            let masses = crate::mass::grid(&foliage.instances, crate::submit::crown_of(foliage));
            buffer::write(
                gpu,
                &mut self.masses,
                "foliage masses",
                wgpu::BufferUsages::STORAGE,
                bytemuck::cast_slice(&masses),
            );
        }
        self.setup(
            gpu,
            element,
            foliage.instances.reference,
            foliage.instances.len() as u32,
            level,
        );
    }

    pub(crate) fn submit_resident(
        &mut self,
        gpu: &Gpu,
        element: &telperion_core::foliage::Element,
        reference: telperion_core::foliage::Reference,
        count: u32,
        placements: crate::buffer::Held,
        masses: crate::buffer::Held,
        level: Level,
    ) {
        self.placements = Some(placements);
        self.masses = Some(masses);
        self.setup(gpu, element, reference, count, level);
    }

    fn setup(
        &mut self,
        gpu: &Gpu,
        element: &telperion_core::foliage::Element,
        reference: telperion_core::foliage::Reference,
        instances: u32,
        level: Level,
    ) {
        self.forced = level;
        self.instances = instances;
        self.reference = reference;
        self.levels = element.levels.iter().map(|l| l.indices.clone()).collect();
        self.sphere = frame::sphere(element);
        self.rachis_levels = crate::card::rachis_levels(element) as u32;
        let sizes = sizes(
            self.instances as usize,
            self.levels.len(),
            u64::from(gpu.device.limits().min_storage_buffer_offset_alignment),
        );
        self.slot = sizes.slot as u32;
        if self.instances == 0 || self.levels.is_empty() {
            (self.compute, self.draw) = (None, None);
            return;
        }
        self.reset = self
            .levels
            .iter()
            .flat_map(|range| [range.len() as u32, 0, range.start, 0, 0])
            .collect();
        let deviations: Vec<f32> = element.levels.iter().map(|l| l.deviation as f32).collect();
        let storage = wgpu::BufferUsages::STORAGE;
        buffer::write(
            gpu,
            &mut self.deviations,
            "foliage level deviations",
            storage,
            bytemuck::cast_slice(&deviations),
        );
        buffer::reserve(
            gpu,
            &mut self.lists,
            "foliage level lists",
            storage | wgpu::BufferUsages::COPY_SRC,
            sizes.lists,
        );
        buffer::reserve(
            gpu,
            &mut self.starts,
            "foliage level starts",
            storage | wgpu::BufferUsages::COPY_SRC,
            sizes.starts,
        );
        buffer::reserve(
            gpu,
            &mut self.counts,
            "foliage level counters",
            storage | wgpu::BufferUsages::COPY_SRC,
            sizes.counts,
        );
        buffer::reserve(
            gpu,
            &mut self.arguments,
            "foliage indirect arguments",
            storage | wgpu::BufferUsages::INDIRECT,
            sizes.arguments,
        );
        buffer::reserve(
            gpu,
            &mut self.scratch,
            "foliage selection scratch",
            storage,
            sizes.scratch,
        );
        self.rebind(gpu);
    }

    /// The bind groups the buffers were just allocated for. Every submit
    /// builds them again: a buffer that outgrew its headroom is a new buffer,
    /// and a group holding the old one would bind what is no longer there.
    fn rebind(&mut self, gpu: &Gpu) {
        let (
            Some(placements),
            Some(masses),
            Some(deviations),
            Some(lists),
            Some(starts),
            Some(counts),
            Some(arguments),
            Some(scratch),
        ) = (
            &self.placements,
            &self.masses,
            &self.deviations,
            &self.lists,
            &self.starts,
            &self.counts,
            &self.arguments,
            &self.scratch,
        )
        else {
            (self.compute, self.draw) = (None, None);
            return;
        };
        self.compute = Some(bind::compute_group(
            gpu,
            &self.compute_layout,
            &self.uniforms,
            [
                placements, deviations, lists, counts, arguments, scratch, starts,
            ],
        ));
        self.draw = Some(bind::draw_group(
            gpu,
            &self.draw_layout,
            [placements, lists, masses],
            starts,
        ));
    }

    /// Starts the counters and the indirect blocks at nothing and runs the
    /// pass. `crown` is false for a view with no crown to select from, and a
    /// crown of no leaves has nothing to run; a timed frame opens the pass
    /// anyway, empty, so its timestamp pair is written and no resolve is left
    /// waiting on the device for a query that no pass wrote.
    pub fn dispatch(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        camera: &Camera,
        viewport: (u32, u32),
        crown: bool,
        timestamps: Option<wgpu::ComputePassTimestampWrites<'_>>,
    ) {
        let ready = match (crown, &self.compute, &self.counts, &self.arguments) {
            (true, Some(compute), Some(counts), Some(args)) => Some((compute, counts, args)),
            _ => None,
        };
        if ready.is_none() && timestamps.is_none() {
            return;
        }
        if let Some((_, counts, arguments)) = ready {
            gpu.queue.write_buffer(
                &self.uniforms,
                0,
                bytemuck::bytes_of(&self.frame(camera, viewport)),
            );
            encoder.clear_buffer(counts.buffer(), 0, Some(counts.region().used()));
            gpu.queue
                .write_buffer(arguments.buffer(), 0, bytemuck::cast_slice(&self.reset));
        }
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("select"),
            timestamp_writes: timestamps,
        });
        if let Some((compute, _, _)) = ready {
            pass.set_bind_group(0, compute, &[]);
            let maximum = gpu.device.limits().max_compute_workgroups_per_dimension;
            let leaves = grid(self.instances.div_ceil(WORKGROUP), maximum);
            let levels = (self.levels.len() as u32 + 1, 1);
            for (pipeline, (x, y)) in self.pipelines.iter().zip([leaves, levels, leaves]) {
                pass.set_pipeline(pipeline);
                pass.dispatch_workgroups(x, y, 1);
            }
        }
    }

    /// What this frame decides against: the frustum a leaf must fall inside,
    /// the eye it is measured from, and the pixels a metre spans at it.
    fn frame(&self, camera: &Camera, viewport: (u32, u32)) -> frame::Uniforms {
        let (width, height) = (f64::from(viewport.0.max(1)), f64::from(viewport.1.max(1)));
        let forward = (camera.target - camera.position).normalized();
        frame::Uniforms {
            planes: frame::planes(&camera.view_projection(width / height)),
            eye: frame::point(
                camera.position,
                frame::pixels_per_metre(height, camera.field_of_view),
            ),
            forward: frame::point(forward, camera.near),
            sphere: self.sphere,
            box_min: frame::point(self.reference.min, 0.0),
            box_extent: frame::point(self.reference.extent, 0.0),
            instances: self.instances,
            levels: self.levels.len() as u32,
            slot: self.slot / size_of::<u32>() as u32,
            forced: match self.forced {
                Level::Chosen => -1,
                Level::Forced(level) => level as i32,
            },
            stalk: [
                self.stalk[0],
                self.stalk[1],
                self.rachis_levels as f32,
                self.stalk[2],
            ],
        }
    }

    /// Binds the crown's placements and the lists with one level's start,
    /// for the indirect draw of that level.
    pub fn bind_level(&self, pass: &mut wgpu::RenderPass<'_>, level: usize) -> bool {
        let Some(draw) = &self.draw else {
            return false;
        };
        pass.set_bind_group(1, draw, &[self.slot * level as u32]);
        true
    }

    /// Binds the leaf view's single placement at identity.
    pub fn bind_leaf(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(1, &self.leaf, &[0]);
    }

    /// The indirect block one level's draw reads its instance count from.
    pub fn arguments(&self, level: usize) -> Option<(&wgpu::Buffer, u64)> {
        let arguments = self.arguments.as_ref()?;
        let words = (level * ARGUMENT_WORDS * size_of::<u32>()) as u64;
        (words < arguments.region().used()).then(|| (arguments.buffer(), words))
    }

    /// Each level's range in the element's shared index buffer, coarsest
    /// first; the last is the element itself.
    pub fn levels(&self) -> &[Range<u32>] {
        &self.levels
    }

    /// The placements the pass classifies, which is the crown as it was
    /// submitted and not what any one frame drew of it.
    pub fn instances(&self) -> u32 {
        self.instances
    }

    /// The live placement range, for a caller that wants to see what went up.
    pub fn region(&self) -> Option<Region> {
        Some(self.placements.as_ref()?.region())
    }

    /// Each instance's level as the last dispatch chose it, or `None` for
    /// one it gave to the unseen bucket: the lists read back (fn-253).
    #[cfg(test)]
    pub(crate) fn chosen(&self, gpu: &Gpu) -> Vec<Option<u32>> {
        let mut out = vec![None; self.instances as usize];
        for (level, ids) in self.level_lists(gpu).iter().enumerate() {
            for &id in ids {
                out[id as usize] = Some(level as u32);
            }
        }
        out
    }

    /// Each level's list as the last dispatch left it, in the order its draw
    /// reads it: level *k* is `counts[k]` words from the level's start word.
    #[cfg(test)]
    pub(crate) fn level_lists(&self, gpu: &Gpu) -> Vec<Vec<u32>> {
        let (Some(lists), Some(starts), Some(counts)) =
            (&self.lists, self.starts(gpu), self.counted(gpu))
        else {
            return Vec::new();
        };
        let words = words(&buffer::read(gpu, lists).expect("lists read back"));
        (counts.iter().zip(starts))
            .map(|(&count, start)| words[start as usize..(start + count) as usize].to_vec())
            .collect()
    }

    /// Each level's start word as the last dispatch wrote it, coarsest first.
    #[cfg(test)]
    pub(crate) fn starts(&self, gpu: &Gpu) -> Option<Vec<u32>> {
        let words = words(&buffer::read(gpu, self.starts.as_ref()?)?);
        let slot = self.slot as usize / size_of::<u32>();
        Some(
            (0..self.levels.len())
                .map(|level| words[level * slot])
                .collect(),
        )
    }

    /// What the last frame's pass counted: one entry per level, coarsest
    /// first, and the leaves no level drew. Reading it stalls on the device,
    /// so it is a check and a record, never a per-frame call.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn counted(&self, gpu: &Gpu) -> Option<Vec<u32>> {
        Some(words(&buffer::read(gpu, self.counts.as_ref()?)?))
    }
}

/// A buffer read back as the words it holds.
#[cfg(not(target_arch = "wasm32"))]
fn words(bytes: &[u8]) -> Vec<u32> {
    (bytes.as_chunks::<4>().0.iter())
        .map(|word| u32::from_ne_bytes(*word))
        .collect()
}

/// The groups a per-leaf pass dispatches, folded into a second dimension once
/// the first runs out, as generation does: a crown of more than 65,535 groups
/// of 256 leaves would otherwise be refused (fn-194). The shader rebuilds the
/// flat index and the last row's overhang returns past the last leaf.
fn grid(groups: u32, maximum: u32) -> (u32, u32) {
    (groups.min(maximum), groups.div_ceil(maximum).max(1))
}

#[cfg(test)]
mod tests;
