//! A pixel-footprint regression at the owner's session-four trunk pose.
mod common;
#[path = "common/resolution.rs"]
mod resolution;
use resolution::masked_agreement;
use telperion_core::{
    math::Vec3,
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{render, Camera, Renderer, Still, View, STILL_FORMAT};

#[test]
fn trunk_agrees_with_a_box_reduction_at_half_resolution() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::OregonWhiteOak.parameters();
    family.skeleton.seed = 7;
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    renderer.set_material(family.material);
    renderer.set_view(View::Bare);
    let camera = Camera {
        position: Vec3::new(1.7307636095778745, 2.2967023330704928, -1.7307636095778745),
        target: Vec3::new(0.0, 2.0, 0.0),
        field_of_view: 38.0,
        near: 0.01,
        far: 1000.0,
    };
    let high = render(&mut renderer, &camera, 1600, 1000).unwrap();
    let low = render(&mut renderer, &camera, 800, 500).unwrap();
    let again = render(&mut renderer, &camera, 800, 500).unwrap();
    let worst = low
        .rgba
        .iter()
        .zip(&again.rgba)
        .map(|(a, b)| a.abs_diff(*b))
        .max()
        .unwrap();
    eprintln!("trunk redraw worst {worst}/255");
    assert_eq!(low.rgba, again.rgba, "the identical trunk draw changed");
    let (mean, p95) = agreement(&high, &low);
    eprintln!("trunk resolution agreement: mean {mean:.6}/255, p95 {p95:.2}/255");
    family.material.ridge_scale = 0.0;
    renderer.set_material(family.material);
    let plain_high = render(&mut renderer, &camera, 1600, 1000).unwrap();
    let plain_low = render(&mut renderer, &camera, 800, 500).unwrap();
    eprintln!(
        "plain trunk diagnostic (mean, p95): {:?}",
        agreement(&plain_high, &plain_low)
    );
    // RGB in the delivered sRGB still, averaged by an exact 2x2 box (no
    // sharpening or registration). Three code values mean, twelve at p95:
    // enough for quantization/lighting, not the rejected sparkling edges.
    resolution::record(
        "bark_resolution_trunk",
        &[("OregonWhiteOak 1x".into(), mean, p95, 3.0)],
    );
    assert!(
        mean <= 3.0 && p95 <= 12.0,
        "bark aliases across resolution: mean {mean:.6}, p95 {p95:.2}"
    );
}

fn agreement(high: &Still, low: &Still) -> (f64, f64) {
    masked_agreement(
        high,
        low,
        &(8..492)
            .flat_map(|y| (280..520).map(move |x| (x, y)))
            .collect::<Vec<_>>(),
    )
}

#[test]
fn grazing_trunks_agree_with_a_box_reduction_at_half_resolution() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let mut measurements = Vec::new();
    for (preset, height) in [(Preset::OregonWhiteOak, 2.0), (Preset::NorwaySpruce, 0.9)] {
        let mut family = preset.parameters();
        family.skeleton.seed = 7;
        let tree = mesh::build(&family, Detail::Full).unwrap();
        // Aim beside the centreline so the silhouette occupies the middle of
        // the frame. The geometric mask selects grazing wood, never colour.
        let camera = Camera {
            position: Vec3::new(
                1.7307636095778745,
                height + 0.2967023330704928,
                -1.7307636095778745,
            ),
            target: Vec3::new(-0.24, height, -0.24),
            field_of_view: 20.0,
            near: 0.01,
            far: 1000.0,
        };
        let mask = edge_mask(&tree.wood, &camera);
        assert!(
            mask.len() > 1000,
            "grazing fixture has too little wood: {}",
            mask.len()
        );
        renderer.submit(&tree).unwrap();
        renderer.set_material(family.material);
        renderer.set_view(View::Bare);
        let high = render(&mut renderer, &camera, 1600, 1000).unwrap();
        let low = render(&mut renderer, &camera, 800, 500).unwrap();
        let again = render(&mut renderer, &camera, 800, 500).unwrap();
        let worst = low
            .rgba
            .iter()
            .zip(&again.rgba)
            .map(|(a, b)| a.abs_diff(*b))
            .max()
            .unwrap();
        assert_eq!(low.rgba, again.rgba, "grazing redraw changed");
        let (mean, p95) = masked_agreement(&high, &low, &mask);
        eprintln!("{preset:?} grazing: {} pixels, mean {mean:.6}/255, p95 {p95:.2}/255, redraw worst {worst}/255", mask.len());
        family.material.ridge_scale = 0.0;
        renderer.set_material(family.material);
        let plain_high = render(&mut renderer, &camera, 1600, 1000).unwrap();
        let plain_low = render(&mut renderer, &camera, 800, 500).unwrap();
        eprintln!(
            "{preset:?} plain grazing: {:?}",
            masked_agreement(&plain_high, &plain_low, &mask)
        );
        measurements.push((format!("{preset:?} grazing"), mean, p95, 3.0));
    }
    resolution::record("bark_resolution_grazing", &measurements);
    assert!(
        measurements
            .iter()
            .all(|(_, mean, p95, bound)| mean <= bound && *p95 <= 12.0),
        "grazing bark aliases across resolution: {measurements:?}"
    );
}

// CPU raster of the unchanged wood, used only to choose a material-independent
// edge region. Perspective interpolation matches the production vertex stage.
// Erode by two pixels to keep MSAA silhouette coverage out of the comparison.
fn edge_mask(wood: &telperion_core::surface::SurfaceMesh, camera: &Camera) -> Vec<(usize, usize)> {
    let matrix = camera.view_projection(1.6);
    let vector = |values: &[f32], i: usize| {
        Vec3::new(
            f64::from(values[i * 3]),
            f64::from(values[i * 3 + 1]),
            f64::from(values[i * 3 + 2]),
        )
    };
    let mut depth = vec![f64::INFINITY; 800 * 500];
    let mut incidence = vec![1.0; 800 * 500];
    let run = &wood.run_table[0];
    for (triangle_index, triangle) in wood.indices.as_chunks::<3>().0.iter().enumerate() {
        let positions = triangle
            .iter()
            .map(|&i| vector(&wood.positions, i as usize))
            .collect::<Vec<_>>();
        let trunk = (run.first_index..run.first_index + run.index_count)
            .contains(&(triangle_index as u32 * 3))
            && positions.iter().all(|p| p.y >= 0.5 && p.y <= 3.5);
        let projected = positions
            .iter()
            .map(|p| {
                let v = [p.x, p.y, p.z, 1.0];
                let clip: [f64; 4] = std::array::from_fn(|r| {
                    (0..4).map(|c| f64::from(matrix[c * 4 + r]) * v[c]).sum()
                });
                [
                    (clip[0] / clip[3] + 1.0) * 400.0,
                    (1.0 - clip[1] / clip[3]) * 250.0,
                    clip[3],
                ]
            })
            .collect::<Vec<_>>();
        if projected.iter().any(|p| p[2] <= camera.near) {
            continue;
        }
        let min = |axis: usize, bound: f64| {
            projected
                .iter()
                .map(|p| p[axis])
                .fold(f64::INFINITY, f64::min)
                .floor()
                .clamp(0.0, bound) as usize
        };
        let max = |axis: usize, bound: f64| {
            projected
                .iter()
                .map(|p| p[axis])
                .fold(f64::NEG_INFINITY, f64::max)
                .ceil()
                .clamp(0.0, bound) as usize
        };
        let [a, b, c] = [projected[0], projected[1], projected[2]];
        let det = (b[1] - c[1]) * (a[0] - c[0]) + (c[0] - b[0]) * (a[1] - c[1]);
        if det.abs() < 1e-8 {
            continue;
        }
        for y in min(1, 500.0)..max(1, 500.0) {
            for x in min(0, 800.0)..max(0, 800.0) {
                let px = x as f64 + 0.5;
                let py = y as f64 + 0.5;
                let wa = ((b[1] - c[1]) * (px - c[0]) + (c[0] - b[0]) * (py - c[1])) / det;
                let wb = ((c[1] - a[1]) * (px - c[0]) + (a[0] - c[0]) * (py - c[1])) / det;
                let wc = 1.0 - wa - wb;
                if wa.min(wb).min(wc) < 0.0 {
                    continue;
                }
                let w = [wa / a[2], wb / b[2], wc / c[2]];
                let z = 1.0 / w.iter().sum::<f64>();
                let index = y * 800 + x;
                if z >= depth[index] {
                    continue;
                }
                depth[index] = z;
                let p = (positions[0] * w[0] + positions[1] * w[1] + positions[2] * w[2]) * z;
                let n = (vector(&wood.normals, triangle[0] as usize) * w[0]
                    + vector(&wood.normals, triangle[1] as usize) * w[1]
                    + vector(&wood.normals, triangle[2] as usize) * w[2])
                    .normalized();
                incidence[index] = if trunk {
                    n.dot((camera.position - p).normalized())
                } else {
                    1.0
                };
            }
        }
    }
    (8..492)
        .flat_map(|y| (8..792).map(move |x| (x, y)))
        .filter(|&(x, y)| {
            (0.12..0.35).contains(&incidence[y * 800 + x])
                && (y - 2..=y + 2).all(|sy| {
                    (x - 2..=x + 2).all(|sx| {
                        depth[sy * 800 + sx].is_finite() && incidence[sy * 800 + sx] < 1.0
                    })
                })
        })
        .collect()
}
