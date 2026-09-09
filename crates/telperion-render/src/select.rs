//! Which level every leaf is drawn at, decided on the GPU once per frame.
//!
//! One compute thread per placement projects the element's deviation at that
//! leaf's depth and takes the coarsest level that stays under half a pixel; a
//! leaf whose sphere is outside the frustum takes the unseen bucket instead.
//! Each thread appends its index to the chosen level's list, and the levels'
//! counters are also the instance counts of one indexed indirect draw each.
//!
//! Nothing survives the frame. There is no hysteresis and no previous-frame
//! state: the switch is below a pixel by construction, so there is nothing to
//! smooth, and a camera cut or a fresh tree needs nothing forgotten.
mod bind;
mod frame;

use std::ops::Range;

use telperion_core::mesh;

use crate::{
    buffer::{self, Held, Region},
    device::Gpu,
    Camera,
};

/// Threads per workgroup, matching `select.wgsl`. The tally that reserves each
/// level's run of its list is taken once per workgroup, not once per leaf.
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
    /// Bytes one level's index list is given, aligned so the list can be bound
    /// at a dynamic offset.
    pub stride: u64,
    pub lists: u64,
    pub counts: u64,
    pub arguments: u64,
}

/// The bytes each selection buffer needs for this many placements at this many
/// levels. The unseen bucket is a counter like any other, so there is one more
/// counter than there are levels.
pub fn sizes(instances: usize, levels: usize, alignment: u64) -> Sizes {
    let stride = ((instances * size_of::<u32>()) as u64).div_ceil(alignment.max(1)) * alignment;
    Sizes {
        stride,
        lists: stride * levels as u64,
        counts: ((levels + 1) * size_of::<u32>()) as u64,
        arguments: (levels * ARGUMENT_WORDS * size_of::<u32>()) as u64,
    }
}

/// The compute pipeline, the buffers it fills, and the bind groups the foliage
/// pass reads them back through.
pub struct Select {
    pipeline: wgpu::ComputePipeline,
    compute_layout: wgpu::BindGroupLayout,
    draw_layout: wgpu::BindGroupLayout,
    uniforms: wgpu::Buffer,
    /// The leaf view: one placement at identity and a list of one, bound
    /// through the same layout as the crown and never rewritten.
    leaf: wgpu::BindGroup,
    placements: Option<Held>,
    deviations: Option<Held>,
    lists: Option<Held>,
    counts: Option<Held>,
    arguments: Option<Held>,
    compute: Option<wgpu::BindGroup>,
    draw: Option<wgpu::BindGroup>,
    /// The indirect blocks as each frame starts them: the level's own index
    /// range, and no instances until the pass counts them.
    reset: Vec<u32>,
    /// Each level's range in the element's shared index buffer, coarsest first.
    levels: Vec<Range<u32>>,
    instances: u32,
    stride: u32,
    forced: Level,
    /// The element's bounding sphere at the origin: centre, then radius.
    sphere: [f32; 4],
}

impl Select {
    pub fn new(gpu: &Gpu) -> Self {
        let shader = gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/select.wgsl"));
        let compute_layout = bind::compute_layout(gpu);
        let draw_layout = bind::draw_layout(gpu);
        let layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("select"),
                bind_group_layouts: &[Some(&compute_layout)],
                immediate_size: 0,
            });
        Self {
            pipeline: gpu
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("select"),
                    layout: Some(&layout),
                    module: &shader,
                    entry_point: Some("select"),
                    compilation_options: Default::default(),
                    cache: None,
                }),
            uniforms: gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("selection"),
                size: size_of::<frame::Uniforms>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            leaf: bind::leaf_group(gpu, &draw_layout),
            compute_layout,
            draw_layout,
            placements: None,
            deviations: None,
            lists: None,
            counts: None,
            arguments: None,
            compute: None,
            draw: None,
            reset: Vec::new(),
            levels: Vec::new(),
            instances: 0,
            stride: 0,
            forced: Level::default(),
            sphere: [0.0; 4],
        }
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
    pub fn submit(&mut self, gpu: &Gpu, foliage: &mesh::Foliage, level: Level) {
        let element = &foliage.element;
        self.forced = level;
        self.instances = foliage.instances.matrices.len() as u32;
        self.levels = element.levels.iter().map(|l| l.indices.clone()).collect();
        self.sphere = frame::sphere(element);
        let sizes = sizes(
            self.instances as usize,
            self.levels.len(),
            u64::from(gpu.device.limits().min_storage_buffer_offset_alignment),
        );
        self.stride = sizes.stride as u32;
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
            &mut self.placements,
            "foliage placements",
            storage,
            bytemuck::cast_slice(&foliage.instances.matrices),
        );
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
            storage,
            sizes.lists,
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
        self.rebind(gpu, sizes.stride);
    }

    /// The bind groups the buffers were just allocated for. Every submit
    /// builds them again: a buffer that outgrew its headroom is a new buffer,
    /// and a group holding the old one would bind what is no longer there.
    fn rebind(&mut self, gpu: &Gpu, stride: u64) {
        let (Some(placements), Some(deviations), Some(lists), Some(counts), Some(arguments)) = (
            &self.placements,
            &self.deviations,
            &self.lists,
            &self.counts,
            &self.arguments,
        ) else {
            (self.compute, self.draw) = (None, None);
            return;
        };
        self.compute = Some(bind::compute_group(
            gpu,
            &self.compute_layout,
            &self.uniforms,
            [placements, deviations, lists, counts, arguments],
        ));
        self.draw = Some(bind::draw_group(
            gpu,
            &self.draw_layout,
            placements,
            lists,
            stride,
        ));
    }

    /// Starts the counters and the indirect blocks at nothing and runs the
    /// pass. A crown that selects nothing never reaches the device.
    pub fn dispatch(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        camera: &Camera,
        viewport: (u32, u32),
    ) {
        let (Some(compute), Some(counts), Some(arguments)) =
            (&self.compute, &self.counts, &self.arguments)
        else {
            return;
        };
        gpu.queue.write_buffer(
            &self.uniforms,
            0,
            bytemuck::bytes_of(&self.frame(camera, viewport)),
        );
        encoder.clear_buffer(counts.buffer(), 0, Some(counts.region().used()));
        gpu.queue
            .write_buffer(arguments.buffer(), 0, bytemuck::cast_slice(&self.reset));
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("select"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, compute, &[]);
        pass.dispatch_workgroups(self.instances.div_ceil(WORKGROUP), 1, 1);
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
            instances: self.instances,
            levels: self.levels.len() as u32,
            stride: self.stride / size_of::<u32>() as u32,
            forced: match self.forced {
                Level::Chosen => -1,
                Level::Forced(level) => level as i32,
            },
        }
    }

    /// Binds the crown's placements with one level's list, for the indirect
    /// draw of that level.
    pub fn bind_level(&self, pass: &mut wgpu::RenderPass<'_>, level: usize) -> bool {
        let Some(draw) = &self.draw else {
            return false;
        };
        pass.set_bind_group(1, draw, &[self.stride * level as u32]);
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

    /// What the last frame's pass counted: one entry per level, coarsest
    /// first, and the leaves no level drew. Reading it stalls on the device,
    /// so it is a check and a record, never a per-frame call.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn counted(&self, gpu: &Gpu) -> Option<Vec<u32>> {
        let bytes = buffer::read(gpu, self.counts.as_ref()?)?;
        Some(
            bytes
                .as_chunks::<4>()
                .0
                .iter()
                .map(|word| u32::from_ne_bytes(*word))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_level_is_given_a_list_the_whole_crown_could_fill() {
        let four = sizes(1_000, 4, 256);
        assert_eq!(four.stride % 256, 0, "a list cannot be bound at an offset");
        assert!(
            four.stride >= 4_000,
            "a list too small for the crown: {}",
            four.stride
        );
        assert_eq!(four.lists, four.stride * 4);
        // One counter per level and one for the leaves no level drew.
        assert_eq!(four.counts, 5 * 4);
        assert_eq!(four.arguments, 4 * 5 * 4);

        // A crown of nothing asks for nothing, at any number of levels.
        assert_eq!(sizes(0, 4, 256).lists, 0);
        assert_eq!(sizes(1_000, 0, 256).lists, 0);
    }
}
