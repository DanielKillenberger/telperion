#[path = "../tests/common/mod.rs"]
mod common;
#[path = "../tests/common/resolution.rs"]
mod resolution;
use telperion_core::{
    math::Vec3,
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{render, Camera, Renderer, View, STILL_FORMAT};
fn main() {
    let gpu = common::gpu().unwrap();
    let mut family = Preset::OregonWhiteOak.parameters();
    family.skeleton.seed = 7;
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    renderer.set_view(View::Bare);
    for plain in [false, true] {
        let mut material = family.material;
        if plain {
            material.ridge_scale = 0.0;
            material.furrow_strength = 0.0;
        }
        renderer.set_material(material);
        for distance in [2, 4] {
            let target = Vec3::new(0.0, 2.0, 0.0);
            let camera = Camera {
                target,
                position: target
                    + Vec3::new(1.7307636095778745, 0.2967023330704928, -1.7307636095778745)
                        * distance as f64,
                field_of_view: 38.0,
                near: 0.01,
                far: 1000.0,
            };
            let high = render(&mut renderer, &camera, 1600, 1000).unwrap();
            let low = render(&mut renderer, &camera, 800, 500).unwrap();
            let mask = (200..300)
                .flat_map(|y| (400 - 100 / distance..400 + 100 / distance).map(move |x| (x, y)))
                .collect::<Vec<_>>();
            println!(
                "plain={plain} distance={distance} mean,p95={:?}",
                resolution::masked_agreement(&high, &low, &mask)
            );
        }
    }
}
