//! fn-217 DESIGN.md 8.2: the margin band is a share of the leaf, set in
//! leaf space, so a leaf smaller than a pixel takes the area-weighted mix
//! of blade and rim, not the rim alone.
mod common;
use telperion_core::material::MaterialParams;
use telperion_core::presets::Preset;
use telperion_core::{math::Vec3, mesh, Family};
use telperion_render::{
    hero_pose, render, Camera, Level, Renderer, Still, View, GROUND_REACH, STILL_FORMAT,
};

/// The step both colour changes take in the green channel, as a linear
/// reflectance: small, so the tone curve is near linear over it.
const STEP: f64 = 0.03;

/// A young beech whose leaf colour is the face's alone: no veins, mottle,
/// grain or per-leaf variation, so the blade and the band each add to the
/// colour linearly.
fn beech() -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    (f.age, f.skeleton.seed) = (10.0, 1);
    let m = &mut f.material;
    (
        m.vein_contrast,
        m.blade_mottle_strength,
        m.blade_grain_strength,
    ) = (0.0, 0.0, 0.0);
    (m.hue_range_low, m.hue_range_high) = (0.0, 0.0);
    (m.brightness_range_low, m.brightness_range_high) = (0.0, 0.0);
    (m.margin_red, m.margin_green, m.margin_blue) = (0.0, 0.0, 0.0);
    m.margin_width = 0.1;
    f
}

/// The green the picture gains, summed over every pixel, from `a` to `b`.
fn gained(a: &Still, b: &Still) -> f64 {
    let green = |s: &Still| s.rgba.chunks(4).map(|p| f64::from(p[1])).sum::<f64>();
    green(b) - green(a)
}

/// The band's share of what the leaves show: the green a step on the band
/// adds over the green the same step on the whole face adds, so the light
/// and the coverage cancel and what is left is the band's area share.
fn band_share(renderer: &mut Renderer, m: MaterialParams, camera: &Camera, w: u32) -> f64 {
    let mut shot = |m: MaterialParams| {
        renderer.set_material(m);
        render(renderer, camera, w, w).unwrap()
    };
    let plain = shot(m);
    let band = shot(MaterialParams {
        margin_green: STEP,
        ..m
    });
    let face = shot(MaterialParams {
        leaf_front_green: m.leaf_front_green + STEP,
        leaf_back_green: m.leaf_back_green + STEP,
        ..m
    });
    gained(&plain, &band) / gained(&plain, &face)
}

/// Past every element's finest level: selection holds each at its finest.
const FINEST: Level = Level::Forced(1_000);

/// The far distances, as multiples of the hero distance, whose band shares
/// are averaged.
const FAR: [f64; 5] = [7.6, 7.8, 8.0, 8.2, 8.4];

#[test]
fn a_leaf_under_a_pixel_takes_the_band_s_area_share_not_the_band() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    for width in [0.1, 0.5] {
        let mut family = beech();
        family.material.margin_width = width;
        let tree = mesh::build(&family).unwrap();
        // Near: one leaf, a couple of hundred pixels long, at its finest level.
        renderer.set_view(View::Leaf);
        renderer.submit_at(&tree, FINEST).unwrap();
        let close = Camera {
            position: Vec3::new(0.0, 0.055, 0.3),
            target: Vec3::new(0.0, 0.055, 0.0),
            field_of_view: 38.0,
            near: 0.001,
            far: 2.0,
        };
        let near = band_share(&mut renderer, family.material, &close, 256);
        // Far: the whole crown, about eight times the hero distance, every
        // leaf a fraction of a pixel. One far picture gains only about 110
        // green steps from the band and 840 from the face, so its share
        // carries about +-0.01 of rounding noise, and any change to the light
        // (the sun's map refitted for the person) moves it that much: from
        // 7.8 to 8.2 times the distance it spans 0.120 to 0.137 (fn-310 task
        // 9), the width of the bound itself. The mean of five distances
        // resolves it (host ruling on task 9).
        renderer.set_view(View::Whole);
        renderer.submit_at(&tree, Level::Chosen).unwrap();
        let hero = hero_pose(tree.bounds, 1.0, GROUND_REACH);
        let shares = FAR.map(|k| {
            let far_camera = Camera {
                position: hero.target + (hero.position - hero.target) * k,
                far: hero.far * k,
                ..hero
            };
            band_share(&mut renderer, family.material, &far_camera, 512)
        });
        let far = shares.iter().sum::<f64>() / shares.len() as f64;
        println!("width {width}: band share near {near:.3}, far {far:.3} (mean of {shares:.3?})");
        assert!(near > width, "width {width}, near {near}");
        assert!(
            (far / near - 1.0).abs() < 0.2,
            "width {width}: near {near}, far {far}"
        );
    }
}
