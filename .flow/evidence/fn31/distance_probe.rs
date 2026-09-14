use telperion_core::{math::Vec3, mesh::{self, Detail}, presets::Preset};
use telperion_render::{render, Camera, Renderer, View, Gpu, STILL_FORMAT};
#[path = "../tests/common/resolution.rs"] mod resolution;
fn main() { for variant in 0..4 {
let gpu = pollster::block_on(Gpu::request(None)).unwrap();
    let mut family = Preset::OregonWhiteOak.parameters();
    family.skeleton.seed = 7;
    match variant {
        1 => family.growth.juvenile_branching = 0.0,
        2 => family.growth.seedling_height = 0.0,
        3 => family.skeleton.habit.crookedness = 0.0,
        _ => {}
    }
    println!("VARIANT {variant}");
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    renderer.set_material(family.material);
    renderer.set_view(View::Bare);
    let mut agreement = Vec::new();
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
        // Fixed physical trunk strip about y=2, excluding silhouette and ground.
        let mask = (200..300)
            .flat_map(|y| (400 - 100 / distance..400 + 100 / distance).map(move |x| (x, y)))
            .collect::<Vec<_>>();
        let (mean, p95) = resolution::masked_agreement(&high, &low, &mask);
        eprintln!("distance {distance}x resolution: mean {mean:.6}/255, p95 {p95:.2}/255");
        agreement.push((mean, p95));
    }
}}
