//! The room the subject stands in, as geometry: a wide ground disc, a 1.8 m
//! figure at the roots for scale, and the neutral values both carry when no
//! scene row repaints them. Nothing here is lit; the frame decides that.
use bytemuck::{Pod, Zeroable};
use telperion_core::math::Vec3;

/// The subject stays a warm neutral clay, the room around it is cool, so the
/// tree reads as a silhouette while keeping one flat value.
pub const CLAY: u32 = 0x9d_96_8c;
pub const BACKGROUND: u32 = 0xc6_ce_d5;
pub const GROUND: u32 = 0xa9_b1_b8;
pub const FIGURE: u32 = 0x6b_67_63;
/// The judging light is one neutral hemisphere and nothing else: form reads off
/// the surface normal, with no key for weak geometry to hide behind.
pub const SKY_LIGHT: u32 = 0xff_ff_ff;
pub const GROUND_LIGHT: u32 = 0x6a_69_66;

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

pub const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4];

/// The room's own vertices. The fourth channel of the colour is how much of
/// this vertex is the floor: the ground disc is repainted by the scene row
/// outdoors, the figure keeps the neutral value it was built with.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub colour: [f32; 4],
}

/// sRGB hex to linear, because the shader works in linear and the target
/// encodes on the way out.
pub fn linear(hex: u32) -> [f32; 4] {
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

/// A room colour as a vertex carries it. The floor is repainted by the scene
/// row when the frame is outdoors; a fixture - the figure - never is.
pub fn floor(hex: u32) -> [f32; 4] {
    let c = linear(hex);
    [c[0], c[1], c[2], 1.0]
}

pub fn fixture(hex: u32) -> [f32; 4] {
    let c = linear(hex);
    [c[0], c[1], c[2], 0.0]
}

pub fn disc(colour: [f32; 4], vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
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
pub fn figure(foot: Vec3, colour: [f32; 4]) -> Vec<Vertex> {
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

pub fn figure_indices(base: u32) -> Vec<u32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_figure_stands_one_point_eight_metres_tall_on_the_ground() {
        let vertices = figure(Vec3::new(3.0, 0.0, 1.5), [0.0; 4]);
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
    fn the_disc_is_wound_one_way_and_covers_its_whole_circle() {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        disc([0.0; 4], &mut vertices, &mut indices);
        assert_eq!(indices.len() as u32, DISC_SEGMENTS * 3);
        assert!(vertices
            .iter()
            .all(|v| v.position[1] == 0.0 && v.normal == [0.0, 1.0, 0.0]));
    }
}
