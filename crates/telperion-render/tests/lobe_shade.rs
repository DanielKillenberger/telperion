//! The lobe shade on real frames: two lobes of leaves, one over the other
//! with a gap between them, under an overcast sky. Each lobe's face keeps its
//! light and what hangs under it falls into its shade; the lower lobe's face
//! is lit again, because the gap ends the lobe over it. At zero the row draws
//! the frame it always drew.
mod common;
use telperion_core::pipeline::executor;
use telperion_core::{
    foliage::{ElementParams, Instances},
    material::MaterialParams,
    math::Vec3,
    mesh::{Foliage, TreeMesh},
    surface::{Bounds, SurfaceMesh},
};
use telperion_render::{render, Camera, Renderer, SceneRow, Still, View, STILL_FORMAT};

const SIZE: u32 = 256;

/// A seeded value in 0..1, by xorshift, so the lobes are the same lobes.
fn next(state: &mut u32) -> f64 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    f64::from(*state) / f64::from(u32::MAX)
}

/// Two slabs of leaves two metres wide and one deep, the lower from 1 m to
/// 2 m and the upper from 2.5 m to 3.5 m, every leaf upright and facing the
/// eye, and no wood.
fn lobes() -> TreeMesh {
    let element = executor::element(ElementParams {
        cup: 0.0,
        curl: 0.0,
        ..Default::default()
    })
    .unwrap();
    let mut state = 0x2545_f491_u32;
    // The crown the fixture fills, as a box that holds it: the leaves stand
    // between minus one and one across and up to three and a half high.
    let mut instances = Instances::new(telperion_core::foliage::Reference::spanning(
        Vec3::new(-2.0, 0.0, -2.0),
        Vec3::new(2.0, 4.0, 2.0),
    ));
    let matrices: Vec<[f32; 16]> = (0..160_000)
        .map(|k| {
            let base = if k % 2 == 0 { 1.0 } else { 2.5 };
            let at = Vec3::new(
                next(&mut state) * 2.0 - 1.0,
                base + next(&mut state),
                next(&mut state) * 2.0 - 1.0,
            );
            // Side along x, axis up, face toward the eye; 1.6 of its size.
            [
                1.6, 0.0, 0.0, 0.0, 0.0, 1.6, 0.0, 0.0, 0.0, 0.0, 1.6, 0.0, at.x, at.y, at.z, 1.0,
            ]
            .map(|v: f64| v as f32)
        })
        .collect();
    for m in &matrices {
        instances.push(m);
    }
    TreeMesh {
        wood: SurfaceMesh::default(),
        foliage: Foliage { element, instances },
        bounds: Bounds {
            min: Vec3::new(-1.2, 0.8, -1.2),
            max: Vec3::new(1.2, 3.7, 1.2),
        },
    }
}

/// The picture's mean over a band of rows across its middle columns.
fn rows(still: &Still, rows: std::ops::Range<u32>) -> f64 {
    let (mut sum, mut count) = (0.0, 0.0);
    for y in rows {
        for x in SIZE * 3 / 8..SIZE * 5 / 8 {
            let at = ((y * SIZE + x) * 4) as usize;
            sum += still.rgba[at..at + 3]
                .iter()
                .map(|&c| f64::from(c))
                .sum::<f64>()
                / 3.0;
            count += 1.0;
        }
    }
    sum / count
}

/// The eye level with the lobes, far enough off to hold both.
fn scene() -> (Renderer, Camera) {
    let gpu = common::gpu().expect("checked by the caller");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&lobes()).unwrap();
    renderer.set_view(View::Whole);
    renderer.set_figure(false);
    // Overcast: the sky carries the lobes and the shade is the sky's.
    renderer.set_scene(SceneRow {
        sun_azimuth: 180.0,
        sun_elevation: 0.0,
        sun_red: 0.0,
        sun_green: 0.0,
        sun_blue: 0.0,
        ..Default::default()
    });
    let camera = Camera {
        position: Vec3::new(0.0, 2.25, 9.0),
        target: Vec3::new(0.0, 2.25, 0.0),
        field_of_view: 22.0,
        near: 0.1,
        far: 100.0,
    };
    (renderer, camera)
}

fn draw(renderer: &mut Renderer, camera: &Camera, lobe_shade: f64) -> Still {
    renderer.set_material(MaterialParams {
        lobe_shade,
        ..Default::default()
    });
    render(renderer, camera, SIZE, SIZE).unwrap()
}

/// The row of the picture a height stands at, from the eye's framing.
fn row_at(height: f64) -> u32 {
    let half = (22.0_f64 / 2.0).to_radians().tan() * 9.0;
    ((0.5 - (height - 2.25) / (2.0 * half)) * f64::from(SIZE)) as u32
}

#[test]
fn a_lobe_s_face_keeps_its_light_and_its_underside_falls_into_shade() {
    let Some(_) = common::gpu() else { return };
    let (mut renderer, camera) = scene();
    let open = draw(&mut renderer, &camera, 0.0);
    let shaded = draw(&mut renderer, &camera, 1.0);
    let kept = |from: f64, to: f64| {
        let band = row_at(to)..row_at(from);
        rows(&shaded, band.clone()) / rows(&open, band)
    };
    let (upper_face, upper_under) = (kept(3.42, 3.5), kept(2.55, 2.75));
    let (lower_face, lower_under) = (kept(1.92, 2.0), kept(1.05, 1.25));
    assert!(
        upper_under < 0.8,
        "under the upper lobe kept {upper_under:.2}"
    );
    assert!(
        lower_under < 0.8,
        "under the lower lobe kept {lower_under:.2}"
    );
    assert!(
        upper_face > upper_under + 0.15,
        "upper: face {upper_face:.2}, under {upper_under:.2}"
    );
    // The gap ends the upper lobe: the lower one's face is its own face.
    assert!(
        lower_face > lower_under + 0.15,
        "lower: face {lower_face:.2}, under {lower_under:.2}"
    );
}

#[test]
fn at_zero_the_row_draws_the_frame_it_always_drew() {
    let Some(_) = common::gpu() else { return };
    let (mut renderer, camera) = scene();
    let neutral = draw(&mut renderer, &camera, 0.0);
    // The same lobes with no grid to read: the leaf view's empty grid is the
    // one a crown of no depth binds, so the frame cannot depend on it.
    renderer.set_material(MaterialParams::default());
    let default = render(&mut renderer, &camera, SIZE, SIZE).unwrap();
    assert_eq!(neutral.rgba, default.rgba);
}
