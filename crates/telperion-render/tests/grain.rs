//! The grain below the relief varies a close-up and converges with its
//! footprint, on the bark by the box integral of its own noise over the pixel
//! and on the blade by its fade, so a distant tree is the smooth one it was.
mod common;
use telperion_core::{
    material::MaterialParams,
    math::Vec3,
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{render, Camera, Level, Renderer, Still, View, STILL_FORMAT};

/// How many channels moved between two stills, the furthest one moved, and
/// the mean move over every channel.
fn moved(a: &Still, b: &Still) -> (usize, u8, f64) {
    let (count, worst, total) = a
        .rgba
        .iter()
        .zip(&b.rgba)
        .map(|(a, b)| a.abs_diff(*b))
        .fold((0, 0, 0u64), |(count, worst, total), d| {
            (
                count + usize::from(d > 0),
                worst.max(d),
                total + u64::from(d),
            )
        });
    (count, worst, total as f64 / a.rgba.len() as f64)
}

/// The row with its grain on and off drawn from one pose: near, the grain
/// has to show; far, it has to have converged by `converged` of the mean
/// move it made near, the way the pixel's box integral of its noise does.
fn near_and_far(
    renderer: &mut Renderer,
    off: MaterialParams,
    on: MaterialParams,
    camera: impl Fn(f64) -> Camera,
    (near, far): (f64, f64),
    size: (u32, u32),
    converged: f64,
) {
    let mut near_move = 0.0;
    for (distance, showing) in [(near, true), (far, false)] {
        let pose = camera(distance);
        renderer.set_material(off);
        let smooth = render(renderer, &pose, size.0, size.1).unwrap();
        renderer.set_material(on);
        let grained = render(renderer, &pose, size.0, size.1).unwrap();
        let (count, worst, mean) = moved(&smooth, &grained);
        eprintln!("distance {distance}: {count} channels moved, worst {worst}, mean {mean:.4}");
        if showing {
            assert!(count > smooth.rgba.len() / 100, "no grain near: {count}");
            near_move = mean;
        } else {
            assert!(
                mean <= near_move * converged,
                "the grain did not converge far: mean {mean:.4} against {near_move:.4} near"
            );
        }
    }
}

#[test]
fn bark_grain_shows_near_and_is_its_mean_far() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::OregonWhiteOak.parameters();
    family.skeleton.seed = 7;
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    renderer.set_view(View::Bare);
    let off = MaterialParams {
        bark_grain_scale: 0.0,
        bark_grain_strength: 0.0,
        ..family.material
    };
    let on = MaterialParams {
        bark_grain_scale: 0.003,
        bark_grain_strength: 0.5,
        ..off
    };
    let target = Vec3::new(0.0, 2.0, 0.0);
    // The trunk at two metres, as bark_distance frames it, and at eight times
    // that, where a pixel is sixteen millimetres of bark and its box spans
    // five cells of the grain (fn-71: the grain leaves by that box integral
    // alone, so what is left is the box's own residue, under a fifth).
    let camera = |distance: f64| Camera {
        target,
        position: target
            + Vec3::new(1.7307636095778745, 0.2967023330704928, -1.7307636095778745) * distance,
        field_of_view: 38.0,
        near: 0.01,
        far: 1000.0,
    };
    near_and_far(&mut renderer, off, on, camera, (0.5, 8.0), (800, 500), 0.2);
}

#[test]
fn blade_grain_shows_near_and_is_its_mean_far() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::Ordinary.parameters();
    family.skeleton.growth.max_nodes = Some(20);
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit_at(&tree, Level::Forced(0)).unwrap();
    renderer.set_view(View::Leaf);
    let off = MaterialParams::default();
    let on = MaterialParams {
        blade_grain_scale: 90.0,
        blade_grain_strength: 0.5,
        ..off
    };
    let camera = |distance: f64| Camera {
        position: Vec3::new(0.0, 0.055, distance),
        target: Vec3::new(0.0, 0.055, 0.0),
        field_of_view: 38.0,
        near: 0.001,
        far: 20.0,
    };
    // At twelve centimetres a cell of ninety a blade is three pixels across;
    // at six metres the whole leaf is a few.
    near_and_far(&mut renderer, off, on, camera, (0.12, 6.0), (256, 256), 0.0);
}
