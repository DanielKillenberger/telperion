//! The room the subject stands in, as geometry: a wide ground disc, and the
//! neutral values it and the person beside the subject (`person.rs`) carry
//! when no scene row repaints them. Nothing here is lit; the frame decides that.
use bytemuck::{Pod, Zeroable};

/// The subject stays a warm neutral clay, the room around it is cool, so the
/// tree reads as a silhouette while keeping one flat value.
pub const CLAY: u32 = 0x9d_96_8c;
pub const BACKGROUND: u32 = 0xc6_ce_d5;
pub const GROUND: u32 = 0xa9_b1_b8;
/// The person is a dark cool slate, which reads as clothing under the warm
/// sun and the cool shade alike, never as skin.
pub const FIGURE: u32 = 0x35_3d_4a;
/// The judging light is one neutral hemisphere and nothing else: form reads off
/// the surface normal, with no key for weak geometry to hide behind.
pub const SKY_LIGHT: u32 = 0xff_ff_ff;
pub const GROUND_LIGHT: u32 = 0x6a_69_66;

/// The person's height, which the hero camera also stands its eye at.
pub const FIGURE_HEIGHT: f64 = super::person::HEIGHT;

/// The ground disc's radius in metres. Large enough that every hero subject
/// stands on a floor and not on a plate; the camera's far plane is solved
/// against it.
pub const GROUND_REACH: f64 = 400.0;
const DISC_SEGMENTS: u32 = 96;

pub const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4];

/// The room's own vertices. The fourth channel of the colour is how much of
/// this vertex is the floor: the ground disc is repainted by the scene row
/// outdoors, the person keeps the value it was built with.
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
/// row when the frame is outdoors; a fixture - the person - never is.
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

#[cfg(test)]
mod tests {
    use super::*;

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
