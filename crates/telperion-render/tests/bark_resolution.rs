//! A pixel-footprint regression at the owner's session-four trunk pose.
mod common;
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
    assert!(
        mean <= 3.0 && p95 <= 12.0,
        "bark aliases across resolution: mean {mean:.6}, p95 {p95:.2}"
    );
}

fn agreement(high: &Still, low: &Still) -> (f64, f64) {
    let mut errors = Vec::new();
    // Fixed, material-independent mask inside the trunk silhouette at this
    // pinned pose. Includes the shaded side and the foreshortened right side;
    // excludes the silhouette's geometry samples, sky, ground and frame edge.
    // 116160 trunk pixels: x=280..520, y=8..492 in the half-size frame.
    for y in 8..492 {
        for x in 280..520 {
            for c in 0..3 {
                let sum: u32 = (0..2)
                    .flat_map(|dy| (0..2).map(move |dx| (dx, dy)))
                    .map(|(dx, dy)| {
                        u32::from(high.rgba[((y * 2 + dy) * 1600 + x * 2 + dx) * 4 + c])
                    })
                    .sum();
                errors.push(
                    (f64::from(sum) / 4.0 - f64::from(low.rgba[(y * 800 + x) * 4 + c])).abs(),
                );
            }
        }
    }
    let mean = errors.iter().sum::<f64>() / errors.len() as f64;
    errors.sort_by(f64::total_cmp);
    (mean, errors[errors.len() * 95 / 100])
}
