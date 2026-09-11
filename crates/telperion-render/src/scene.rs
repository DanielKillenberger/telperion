//! The clay room: neutral ground, neutral background, one hemisphere of light
//! and a 1.8 m figure at the roots. Nothing here colours the tree; the warm and
//! cool split is between the subject and everything that is not the subject.
use bytemuck::{Pod, Zeroable};
use telperion_core::math::Vec3;
use wgpu::util::DeviceExt;

use crate::{device::Gpu, shadow::Light};

/// The scene row - the sun, the sky and the ground as numbers - kept beside
/// this file rather than in it so neither outgrows the project's line rule.
mod row;

pub use row::SceneRow;

/// The subject stays a warm neutral clay, the room around it is cool, so the
/// tree reads as a silhouette while keeping one flat value.
const CLAY: u32 = 0x9d_96_8c;
const BACKGROUND: u32 = 0xc6_ce_d5;
const GROUND: u32 = 0xa9_b1_b8;
const FIGURE: u32 = 0x6b_67_63;
/// The judging light is one neutral hemisphere and nothing else: form reads off
/// the surface normal, with no key for weak geometry to hide behind.
const SKY_LIGHT: u32 = 0xff_ff_ff;
const GROUND_LIGHT: u32 = 0x6a_69_66;

/// 1.8 m: radius 0.28 twice, plus a 1.24 m body.
const FIGURE_RADIUS: f64 = 0.28;
const FIGURE_BODY: f64 = 1.24;
pub const FIGURE_HEIGHT: f64 = FIGURE_BODY + FIGURE_RADIUS * 2.0;

/// The ground disc's radius in metres. Large enough that every hero subject
/// stands on a floor and not on a plate; the camera's far plane is solved
/// against it.
pub const GROUND_REACH: f64 = 400.0;
const DISC_SEGMENTS: u32 = 96;
const FIGURE_SEGMENTS: u32 = 16;
const FIGURE_STACKS: u32 = 8;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x3];

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    colour: [f32; 3],
}

/// What every pipeline in a frame is drawn under. The camera and the room's
/// light came first and keep their places; the sun and its map follow them, so
/// a shader that never grew a sun term reads the block it always read.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct Uniforms {
    view_projection: [f32; 16],
    sky: [f32; 4],
    ground: [f32; 4],
    clay: [f32; 4],
    light_view_projection: [f32; 16],
    sun: [f32; 4],
    /// The direction towards the sun, and a fourth slot the block wants.
    sun_direction: [f32; 4],
}

/// sRGB hex to linear, because the shader works in linear and the target
/// encodes on the way out.
fn linear(hex: u32) -> [f32; 4] {
    let channel = |shift: u32| {
        let c = ((hex >> shift) & 0xff) as f32 / 255.0;
        if c <= 0.040_45 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    [channel(16), channel(8), channel(0), 1.0]
}

fn disc(colour: [f32; 3], vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
    let base = vertices.len() as u32;
    let up = [0.0, 1.0, 0.0];
    vertices.push(Vertex {
        position: [0.0, 0.0, 0.0],
        normal: up,
        colour,
    });
    for step in 0..=DISC_SEGMENTS {
        let angle = std::f64::consts::TAU * f64::from(step) / f64::from(DISC_SEGMENTS);
        vertices.push(Vertex {
            position: [
                (GROUND_REACH * angle.cos()) as f32,
                0.0,
                (GROUND_REACH * angle.sin()) as f32,
            ],
            normal: up,
            colour,
        });
    }
    for step in 0..DISC_SEGMENTS {
        indices.extend([base, base + step + 2, base + step + 1]);
    }
}

/// A capsule standing on the ground at `foot`: two hemispheres of
/// `FIGURE_RADIUS` around a `FIGURE_BODY` waist. Rings run bottom to top so the
/// seam between the hemispheres is the body itself.
fn figure(foot: Vec3, colour: [f32; 3]) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    let half = std::f64::consts::FRAC_PI_2;
    for ring in 0..2 * (FIGURE_STACKS + 1) {
        let upper = ring > FIGURE_STACKS;
        let stack = if upper {
            ring - FIGURE_STACKS - 1
        } else {
            ring
        };
        let latitude =
            half * (f64::from(stack) / f64::from(FIGURE_STACKS) - if upper { 0.0 } else { 1.0 });
        let centre = foot.y + FIGURE_RADIUS + if upper { FIGURE_BODY } else { 0.0 };
        for step in 0..=FIGURE_SEGMENTS {
            let angle = std::f64::consts::TAU * f64::from(step) / f64::from(FIGURE_SEGMENTS);
            let normal = Vec3::new(
                latitude.cos() * angle.cos(),
                latitude.sin(),
                latitude.cos() * angle.sin(),
            );
            vertices.push(Vertex {
                position: [
                    (foot.x + normal.x * FIGURE_RADIUS) as f32,
                    (centre + normal.y * FIGURE_RADIUS) as f32,
                    (foot.z + normal.z * FIGURE_RADIUS) as f32,
                ],
                normal: [normal.x as f32, normal.y as f32, normal.z as f32],
                colour,
            });
        }
    }
    vertices
}

fn figure_indices(base: u32) -> Vec<u32> {
    let mut indices = Vec::new();
    let stride = FIGURE_SEGMENTS + 1;
    for ring in 0..2 * FIGURE_STACKS + 1 {
        for step in 0..FIGURE_SEGMENTS {
            let a = base + ring * stride + step;
            let b = a + stride;
            indices.extend([a, b, a + 1, a + 1, b, b + 1]);
        }
    }
    indices
}

/// The room and the light every pipeline draws under. Owns the one uniform
/// block and the static geometry; the tree is submitted separately.
pub struct Scene {
    /// Where the sun stands and what the sky and ground are. Stored the way
    /// the view is stored - the frame reads it, nothing about a tree states
    /// it - and set through the renderer's own setter.
    row: SceneRow,
    layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    uniforms: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    index_count: u32,
    figure_offset: u64,
}

impl Scene {
    pub fn new(
        gpu: &Gpu,
        colour_format: wgpu::TextureFormat,
        shadow: &wgpu::BindGroupLayout,
    ) -> Self {
        let ground_colour = {
            let c = linear(GROUND);
            [c[0], c[1], c[2]]
        };
        let figure_colour = {
            let c = linear(FIGURE);
            [c[0], c[1], c[2]]
        };
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        disc(ground_colour, &mut vertices, &mut indices);
        let figure_offset = (vertices.len() * size_of::<Vertex>()) as u64;
        indices.extend(figure_indices(vertices.len() as u32));
        vertices.extend(figure(Vec3::ZERO, figure_colour));

        let vertex_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("scene vertices"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        let index_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("scene indices"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        let uniforms = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("scene uniforms"),
            size: size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let layout = gpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("scene"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scene"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms.as_entire_binding(),
            }],
        });

        let shader = gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/scene.wgsl"));
        // The room takes no selection, so the group between the light and the
        // shadow map is a hole in its layout rather than a group it binds.
        let pipeline = crate::pipeline(
            gpu,
            &[Some(&layout), None, Some(shadow)],
            &shader,
            colour_format,
            &[Some(wgpu::VertexBufferLayout {
                array_stride: size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &ATTRIBUTES,
            })],
            "scene",
        );

        Self {
            row: SceneRow::default(),
            layout,
            bind_group,
            uniforms,
            pipeline,
            vertices: vertex_buffer,
            indices: index_buffer,
            index_count: indices.len() as u32,
            figure_offset,
        }
    }

    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    /// The sun, sky and ground the next frame is drawn under.
    pub fn row(&self) -> &SceneRow {
        &self.row
    }

    /// Stands the sun somewhere else. The row is held until the next one
    /// replaces it, exactly as the view is.
    pub fn set_row(&mut self, row: SceneRow) {
        self.row = row;
    }

    /// The background the frame is cleared to.
    pub fn background(&self) -> wgpu::Color {
        let c = linear(BACKGROUND);
        wgpu::Color {
            r: f64::from(c[0]),
            g: f64::from(c[1]),
            b: f64::from(c[2]),
            a: 1.0,
        }
    }

    /// Stands the figure clear of the root flare of a subject this tall.
    pub fn place_figure(&self, gpu: &Gpu, height: f64) {
        let foot = Vec3::new(height * 0.16 + 1.2, 0.0, height * 0.2);
        let c = linear(FIGURE);
        let vertices = figure(foot, [c[0], c[1], c[2]]);
        gpu.queue.write_buffer(
            &self.vertices,
            self.figure_offset,
            bytemuck::cast_slice(&vertices),
        );
    }

    /// Writes the frame's one uniform block: where the eye stands, and where
    /// the sun stands with the map it threw.
    pub fn set_frame(&self, gpu: &Gpu, camera: &crate::Camera, aspect: f64, light: &Light) {
        gpu.queue.write_buffer(
            &self.uniforms,
            0,
            bytemuck::bytes_of(&Uniforms {
                view_projection: camera.view_projection(aspect),
                sky: linear(SKY_LIGHT),
                ground: linear(GROUND_LIGHT),
                clay: linear(CLAY),
                light_view_projection: light.view_projection,
                sun: [
                    self.row.sun_red as f32,
                    self.row.sun_green as f32,
                    self.row.sun_blue as f32,
                    1.0,
                ],
                sun_direction: light.direction,
            }),
        );
    }

    /// Binds the camera and the light every pipeline in the pass draws under.
    /// It is bound once per pass and not per subject, because a view that
    /// leaves the room out still needs the light.
    pub fn bind(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(0, &self.bind_group, &[]);
    }

    /// Draws the room. One call: the ground and the figure share a buffer.
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> crate::FrameStats {
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.index_count, 0, 0..1);
        crate::FrameStats {
            draw_calls: 1,
            triangles: self.index_count / 3,
            instances: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_figure_stands_one_point_eight_metres_tall_on_the_ground() {
        let vertices = figure(Vec3::new(3.0, 0.0, 1.5), [0.0; 3]);
        let low = vertices
            .iter()
            .fold(f32::MAX, |low, v| low.min(v.position[1]));
        let high = vertices
            .iter()
            .fold(f32::MIN, |high, v| high.max(v.position[1]));
        assert!(low.abs() < 1e-5, "the figure floats or sinks: {low}");
        assert!(
            (f64::from(high) - FIGURE_HEIGHT).abs() < 1e-5,
            "the figure is {high} m, not {FIGURE_HEIGHT} m"
        );
    }

    #[test]
    fn sky_and_ground_light_bracket_the_hemisphere_term() {
        // Up-facing surfaces take the sky, down-facing the ground, and the
        // ground is the darker of the two or there is no form to read.
        let (sky, ground) = (linear(SKY_LIGHT), linear(GROUND_LIGHT));
        assert!(sky[0] > ground[0] && sky[1] > ground[1] && sky[2] > ground[2]);
        assert!((sky[0] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn the_disc_is_wound_one_way_and_covers_its_whole_circle() {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        disc([0.0; 3], &mut vertices, &mut indices);
        assert_eq!(indices.len() as u32, DISC_SEGMENTS * 3);
        assert!(vertices
            .iter()
            .all(|v| v.position[1] == 0.0 && v.normal == [0.0, 1.0, 0.0]));
    }
}
