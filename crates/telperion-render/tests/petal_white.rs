//! fn-234 R2: a white petal draws near white at the whole-tree view. The
//! petals transmit their own colour, not the leaves', so a crown whose
//! leaves pass a strong green leaves a white floret white. The petal's
//! drawn colour is unmixed per pixel from the whole still against the same
//! tree without florets and the organ mask: P = (C - (1 - c) B) / c.
mod common;
use telperion_core::foliage::BloomParams;
use telperion_core::presets::Preset;
use telperion_core::{mesh, Family};
use telperion_render::{
    hero_pose, render, Level, Renderer, SceneRow, Still, View, GROUND_REACH, STILL_FORMAT,
};

/// A young beech flowering on its outer ages with the cherry's corymb, its
/// petals the material's default white albedo, its leaves passing a
/// saturated green.
fn flowering() -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    (f.age, f.skeleton.seed) = (20.0, 1);
    for k in [12, 14] {
        f.space.species.as_mut().unwrap().states[k].flowering = 1.0;
    }
    f.bloom = BloomParams {
        florets: 6.0,
        droop: 0.6,
        pedicel: 0.03,
        spread: 60.0,
        open: 75.0,
        petal_length: 0.016,
        petal_width: 0.014,
        petal_widest: 0.62,
        petal_cup: 0.35,
        scatter: 0.12,
        ..BloomParams::default()
    };
    let m = &mut f.material;
    (
        m.transmission_red,
        m.transmission_green,
        m.transmission_blue,
    ) = (0.05, 0.9, 0.05);
    m.transmission_strength = 1.0;
    f
}

fn linear(v: u8) -> f64 {
    let c = f64::from(v) / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn srgb(c: f64) -> f64 {
    let c = c.clamp(0.0, 1.0);
    255.0
        * if c <= 0.0031308 {
            12.92 * c
        } else {
            1.055 * c.powf(1.0 / 2.4) - 0.055
        }
}

/// The petals' drawn sRGB where the mask covers at least half a pixel.
fn petals(with: &Still, without: &Still, mask: &Still) -> [f64; 3] {
    let (mut sum, mut weight) = ([0.0; 3], 0.0);
    for k in 0..mask.rgba.len() / 4 {
        let c = linear(mask.rgba[k * 4 + 1]);
        if c < 0.5 {
            continue;
        }
        for (ch, s) in sum.iter_mut().enumerate() {
            *s += linear(with.rgba[k * 4 + ch]) - (1.0 - c) * linear(without.rgba[k * 4 + ch]);
        }
        weight += c;
    }
    assert!(weight > 100.0, "the florets cover {weight} pixels");
    sum.map(|s| srgb(s / weight))
}

/// The petals' drawn sRGB at the hero camera with the sun at `azimuth`.
fn drawn(renderer: &mut Renderer, f: &Family, azimuth: f64) -> [f64; 3] {
    renderer.set_scene(SceneRow {
        sun_azimuth: azimuth,
        sun_elevation: 40.0,
        ..SceneRow::default()
    });
    let mut tree = mesh::build(f).unwrap();
    let (w, h) = (480, 360);
    let camera = hero_pose(tree.bounds, f64::from(w) / f64::from(h), GROUND_REACH);
    let mut draw = |tree: &mesh::TreeMesh, view: View, mask: bool| {
        renderer.submit_at(tree, Level::Chosen).unwrap();
        renderer.set_family(f);
        renderer.set_view(view);
        renderer.set_organ_mask(mask);
        let still = render(renderer, &camera, w, h).unwrap();
        renderer.set_organ_mask(false);
        still
    };
    let mask = draw(&tree, View::Clay, true);
    let with = draw(&tree, View::Whole, false);
    tree.organ = None;
    let without = draw(&tree, View::Whole, false);
    petals(&with, &without, &mask)
}

/// The hero camera's azimuth, as the sun's is measured (`shadow/fit.rs`).
fn camera_azimuth(f: &Family) -> f64 {
    let bounds = mesh::build(f).unwrap().bounds;
    let c = hero_pose(bounds, 4.0 / 3.0, GROUND_REACH);
    let d = c.position - (bounds.min + bounds.max) * 0.5;
    d.x.atan2(d.z).to_degrees().rem_euclid(360.0)
}

/// Lit from the hero sun, a white petal draws near white; lit from behind
/// the crown, it passes its own white, not the leaves' green.
#[test]
fn a_white_petal_draws_near_white_at_the_whole_tree_view() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let f = flowering();
    let behind = (camera_azimuth(&f) + 180.0).rem_euclid(360.0);
    for (sun, azimuth) in [("hero", 115.0), ("behind", behind)] {
        let p = drawn(&mut renderer, &f, azimuth);
        println!("{sun}: petals {p:.1?}");
        let high = p.iter().copied().fold(0.0, f64::max);
        let low = p.iter().copied().fold(255.0, f64::min);
        assert!(
            (high - low) / high <= 0.06,
            "sun {sun}: the petals draw {p:.0?}, tinted off their own white"
        );
        if sun == "hero" {
            assert!(
                low >= 180.0,
                "the petals draw {p:.0?}, darker than a near white"
            );
        }
    }
}
