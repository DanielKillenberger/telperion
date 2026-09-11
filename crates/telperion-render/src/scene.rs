//! The clay room: neutral ground, neutral background, one hemisphere of light
//! and a 1.8 m figure at the roots. Nothing here colours the tree; the warm and
//! cool split is between the subject and everything that is not the subject.
use bytemuck::{Pod, Zeroable};
use telperion_core::{material::MaterialParams, math::Vec3, surface::Bounds};
use wgpu::util::DeviceExt;

use crate::{
    device::Gpu,
    pass::{lit_shader, Depth},
    shadow::Light,
    view::View,
};

/// The scene row - the sun, the sky and the ground as numbers - and the room's
/// own geometry, each beside this file rather than in it so none of the three
/// outgrows the project's line rule.
mod room;
mod row;

pub use room::{FIGURE_HEIGHT, GROUND_REACH};
pub use row::SceneRow;

use room::{
    disc, figure, figure_indices, fixture, floor, linear, Vertex, ATTRIBUTES, BACKGROUND, CLAY,
    FIGURE, GROUND, GROUND_LIGHT, SKY_LIGHT,
};

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

/// What every pipeline in a frame is drawn under. The camera and the room's
/// light came first and keep their places; the sun and its map follow them, so
/// a shader that never grew a sun term reads the block it always read.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct Uniforms {
    view_projection: [f32; 16],
    sky: [f32; 4],
    ground: [f32; 4],
    /// The clay value, and in its fourth slot whether this frame is the clay
    /// room at all: the one thing every shader branches on.
    clay: [f32; 4],
    light_view_projection: [f32; 16],
    sun: [f32; 4],
    /// The direction towards the sun, and a fourth slot the block wants.
    sun_direction: [f32; 4],
    eye: [f32; 4],
    /// The picture's axes, so a pixel knows which way it looks: the forward
    /// axis plus each edge scaled by the pixel's place across the frame.
    ray_right: [f32; 4],
    ray_up: [f32; 4],
    ray_forward: [f32; 4],
    sky_zenith: [f32; 4],
    sky_horizon: [f32; 4],
    ground_colour: [f32; 4],
    /// The material row: bark with its roughness, the leaf's two faces with
    /// the interior darkening amount, and the offsets one leaf may take.
    bark: [f32; 4],
    leaf_front: [f32; 4],
    leaf_back: [f32; 4],
    leaf_variation: [f32; 4],
    /// The ellipsoid the crown's placements fill, and whether there is one.
    crown_centre: [f32; 4],
    crown_radii: [f32; 4],
}

/// The room and the light every pipeline draws under. Owns the one uniform
/// block and the static geometry; the tree is submitted separately.
pub struct Scene {
    /// Where the sun stands and what the sky and ground are. Stored the way
    /// the view is stored - the frame reads it, nothing about a tree states
    /// it - and set through the renderer's own setter.
    row: SceneRow,
    /// What the subject is made of, as the family stated it. It arrives with
    /// the tree and outlives nothing: a second tree brings its own row.
    material: MaterialParams,
    /// The ellipsoid the submitted crown's placements fill, which a leaf's
    /// depth into the crown is measured against. None before a tree is up.
    crown: Option<Bounds>,
    layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    uniforms: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    sky: wgpu::RenderPipeline,
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
        let (ground_colour, figure_colour) = (floor(GROUND), fixture(FIGURE));
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

        let shader = lit_shader(gpu, "scene", include_str!("shaders/scene.wgsl"));
        // The room takes no selection, so the group between the light and the
        // shadow map is a hole in its layout rather than a group it binds.
        let groups = [Some(&layout), None, Some(shadow)];
        let pipeline = crate::pipeline(
            gpu,
            &groups,
            &shader,
            colour_format,
            &[Some(wgpu::VertexBufferLayout {
                array_stride: size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &ATTRIBUTES,
            })],
            Depth::Surface,
            "scene",
        );
        // The sky has no geometry and no depth of its own: one triangle over
        // the frame, drawn before anything else stands in front of it.
        let sky = crate::pipeline(
            gpu,
            &groups,
            &lit_shader(gpu, "sky", include_str!("shaders/sky.wgsl")),
            colour_format,
            &[],
            Depth::Behind,
            "sky",
        );

        Self {
            row: SceneRow::default(),
            material: MaterialParams::default(),
            crown: None,
            layout,
            bind_group,
            uniforms,
            pipeline,
            sky,
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

    /// What the subject that is up is made of. It arrives with the tree, so a
    /// tree submitted without one is drawn in the row every family starts
    /// from rather than in the last tree's colours.
    pub fn set_material(&mut self, material: MaterialParams) {
        self.material = material;
    }

    /// The ellipsoid a leaf's depth into the crown is measured against.
    pub fn set_crown(&mut self, crown: Option<Bounds>) {
        self.crown = crown;
    }

    /// The background the frame is cleared to: the room's cool neutral, or the
    /// sky at the horizon outdoors, where the sky's own triangle covers it
    /// everywhere the frame draws one.
    pub fn background(&self, view: View) -> wgpu::Color {
        let c = if view == View::Clay {
            linear(BACKGROUND)
        } else {
            [
                self.row.sky_horizon_red as f32,
                self.row.sky_horizon_green as f32,
                self.row.sky_horizon_blue as f32,
                1.0,
            ]
        };
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
        let vertices = figure(foot, fixture(FIGURE));
        gpu.queue.write_buffer(
            &self.vertices,
            self.figure_offset,
            bytemuck::cast_slice(&vertices),
        );
    }

    /// Writes the frame's one uniform block: where the eye stands and what it
    /// looks along, where the sun stands with the map it threw, and the two
    /// rows the frame is drawn from - the scene's and the subject's.
    pub fn set_frame(
        &self,
        gpu: &Gpu,
        camera: &crate::Camera,
        aspect: f64,
        light: &Light,
        view: View,
    ) {
        let m = &self.material;
        let colour = |r: f64, g: f64, b: f64, w: f64| [r as f32, g as f32, b as f32, w as f32];
        let clay = linear(CLAY);
        let (right, up, forward) = axes(camera, aspect);
        // A leaf on its own has no crown to stand deep inside, and the clay
        // room takes no material term at all.
        let inside = self
            .crown
            .filter(|_| view == View::Whole || view == View::Bare);
        let centre = inside.map_or([0.0; 4], |b| {
            let c = (b.min + b.max) * 0.5;
            colour(c.x, c.y, c.z, 1.0)
        });
        let radii = inside.map_or([0.0; 4], |b| {
            let half = (b.max - b.min) * 0.5;
            colour(half.x, half.y, half.z, 0.0)
        });
        gpu.queue.write_buffer(
            &self.uniforms,
            0,
            bytemuck::bytes_of(&Uniforms {
                view_projection: camera.view_projection(aspect),
                sky: linear(SKY_LIGHT),
                ground: linear(GROUND_LIGHT),
                clay: [
                    clay[0],
                    clay[1],
                    clay[2],
                    f32::from(u8::from(view == View::Clay)),
                ],
                light_view_projection: light.view_projection,
                sun: colour(self.row.sun_red, self.row.sun_green, self.row.sun_blue, 1.0),
                sun_direction: light.direction,
                eye: colour(camera.position.x, camera.position.y, camera.position.z, 1.0),
                ray_right: colour(right.x, right.y, right.z, 0.0),
                ray_up: colour(up.x, up.y, up.z, 0.0),
                ray_forward: colour(forward.x, forward.y, forward.z, 0.0),
                sky_zenith: colour(
                    self.row.sky_zenith_red,
                    self.row.sky_zenith_green,
                    self.row.sky_zenith_blue,
                    1.0,
                ),
                sky_horizon: colour(
                    self.row.sky_horizon_red,
                    self.row.sky_horizon_green,
                    self.row.sky_horizon_blue,
                    1.0,
                ),
                ground_colour: colour(
                    self.row.ground_red,
                    self.row.ground_green,
                    self.row.ground_blue,
                    1.0,
                ),
                bark: colour(m.bark_red, m.bark_green, m.bark_blue, m.bark_roughness),
                leaf_front: colour(
                    m.leaf_front_red,
                    m.leaf_front_green,
                    m.leaf_front_blue,
                    m.interior_darkening,
                ),
                leaf_back: colour(m.leaf_back_red, m.leaf_back_green, m.leaf_back_blue, 1.0),
                leaf_variation: [
                    m.hue_range_low as f32,
                    m.hue_range_high as f32,
                    m.brightness_range_low as f32,
                    m.brightness_range_high as f32,
                ],
                crown_centre: centre,
                crown_radii: radii,
            }),
        );
    }

    /// Binds the camera and the light every pipeline in the pass draws under.
    /// It is bound once per pass and not per subject, because a view that
    /// leaves the room out still needs the light.
    pub fn bind(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(0, &self.bind_group, &[]);
    }

    /// Draws the room: the sky behind everything outdoors, then the ground and
    /// the figure, which share a buffer and one call.
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, view: View) -> crate::FrameStats {
        let mut sky = 0;
        if view != View::Clay {
            pass.set_pipeline(&self.sky);
            pass.draw(0..3, 0..1);
            sky = 1;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.index_count, 0, 0..1);
        crate::FrameStats {
            draw_calls: 1 + sky,
            triangles: self.index_count / 3 + sky,
            instances: 0,
        }
    }
}

/// The picture's axes from the pose it was taken at: where the eye looks, and
/// how far a pixel at the edge of the frame leans off that in each direction.
/// A ray through the frame is the forward axis plus the two, so the sky can be
/// read per pixel without the projection being inverted.
fn axes(camera: &crate::Camera, aspect: f64) -> (Vec3, Vec3, Vec3) {
    let forward = (camera.target - camera.position).normalized();
    let right = forward.cross(Vec3::Y).normalized();
    let up = right.cross(forward);
    let tangent = (camera.field_of_view.to_radians() / 2.0).tan();
    (right * (tangent * aspect), up * tangent, forward)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sky_and_ground_light_bracket_the_hemisphere_term() {
        // Up-facing surfaces take the sky, down-facing the ground, and the
        // ground is the darker of the two or there is no form to read.
        let (sky, ground) = (linear(SKY_LIGHT), linear(GROUND_LIGHT));
        assert!(sky[0] > ground[0] && sky[1] > ground[1] && sky[2] > ground[2]);
        assert!((sky[0] - 1.0).abs() < 1e-6);
    }
}
