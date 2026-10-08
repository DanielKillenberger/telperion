//! fn-217 M0 (DESIGN.md 3.7, 7.4): the florets' visible coverage. The organ
//! mask draws the flowers white and every other surface black, unlit, in the
//! clay room, so a resolved still's mean is the share of the frame the
//! florets cover with the wood and the leaves in front of them as drawn.
mod common;
use telperion_core::foliage::BloomParams;
use telperion_core::presets::Preset;
use telperion_core::{mesh, Family};
use telperion_render::{
    hero_pose, render, Camera, Level, Renderer, Still, View, GROUND_REACH, STILL_FORMAT,
};

/// A beech at `age` whose outer ages flower at `share` with the cherry's
/// blossom (fn-216's overlay).
fn blossoming(age: f64, share: f64) -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    (f.age, f.skeleton.seed) = (age, 1);
    let species = f.space.species.as_mut().unwrap();
    for k in [12, 14] {
        species.states[k].flowering = share;
    }
    f.bloom = BloomParams {
        florets: 4.0,
        droop: 0.6,
        pedicel: 0.03,
        spread: 60.0,
        open: 75.0,
        petal_length: 0.014,
        petal_width: 0.012,
        petal_widest: 0.62,
        petal_cup: 0.35,
        scatter: 0.12,
        ..BloomParams::default()
    };
    f
}

/// The share of the frame the florets cover: the mean of the mask's
/// resolved, sRGB-encoded green, decoded to linear.
fn coverage(still: &Still) -> f64 {
    let linear = |v: u8| {
        let c = f64::from(v) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let pixels = still.rgba.chunks(4);
    pixels.clone().map(|p| linear(p[1])).sum::<f64>() / pixels.len() as f64
}

fn mask(renderer: &mut Renderer, camera: &Camera, w: u32, h: u32) -> Still {
    renderer.set_organ_mask(true);
    let still = render(renderer, camera, w, h).unwrap();
    renderer.set_organ_mask(false);
    still
}

/// The mask is the flowers alone: nothing on a tree that does not flower,
/// some share on one that does, and the clay room unchanged once it is off.
#[test]
fn the_mask_shows_the_flowers_and_nothing_else() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.set_view(View::Clay);
    let (w, h) = (320, 240);
    let mut shares = Vec::new();
    for share in [0.0, 0.6] {
        let f = blossoming(20.0, share);
        let tree = mesh::build(&f).unwrap();
        renderer.submit_at(&tree, Level::Chosen).unwrap();
        renderer.set_family(&f);
        let camera = hero_pose(tree.bounds, f64::from(w) / f64::from(h), GROUND_REACH);
        let clay = render(&mut renderer, &camera, w, h).unwrap();
        shares.push(coverage(&mask(&mut renderer, &camera, w, h)));
        let again = render(&mut renderer, &camera, w, h).unwrap();
        assert_eq!(clay.rgba, again.rgba, "the clay room after the mask");
    }
    assert_eq!(shares[0], 0.0);
    assert!(shares[1] > 1e-4, "flowering covers {}", shares[1]);
}

/// `still` box-filtered down by `k` each way: the reference resolved to the
/// target's size, in linear coverage.
fn resolved(still: &Still, k: u32) -> Vec<f64> {
    let (w, h) = (still.width / k, still.height / k);
    let linear = |v: u8| {
        let c = f64::from(v) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let mut out = vec![0.0; (w * h) as usize];
    for y in 0..still.height {
        for x in 0..still.width {
            let at = ((y * still.width + x) * 4 + 1) as usize;
            out[((y / k) * w + x / k) as usize] += linear(still.rgba[at]);
        }
    }
    let n = f64::from(k * k);
    out.iter_mut().for_each(|v| *v /= n);
    out
}

/// M0 (`STILLS=<dir> cargo test --profile ci -p telperion-render --test
/// organ_coverage -- --ignored --nocapture`): the florets' coverage at the
/// hero pose against a reference four times finer each way at the finest
/// geometry, resolved to the target's size, with the frame's selected
/// levels and with the finest. Writes the target and the resolved reference.
#[test]
#[ignore]
fn measure_the_florets_coverage_at_the_hero_pose() {
    let Some(gpu) = common::gpu() else { return };
    let dir = std::env::var("STILLS").expect("STILLS names the directory the masks go to");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.set_view(View::Clay);
    let (w, h, k) = (960, 720, 4);
    for age in [20.0, 40.0] {
        let f = blossoming(age, 1.0);
        let tree = mesh::build(&f).unwrap();
        let florets = tree.organ.as_ref().map_or(0, |o| o.instances.leaves.len());
        let camera = hero_pose(tree.bounds, f64::from(w) / f64::from(h), GROUND_REACH);
        let mut at = |level, scale: u32| {
            renderer.submit_at(&tree, level).unwrap();
            renderer.set_family(&f);
            mask(&mut renderer, &camera, w * scale, h * scale)
        };
        let chosen = at(Level::Chosen, 1);
        let finest = at(Level::Forced(0), 1);
        let reference = at(Level::Forced(0), k);
        let truth = resolved(&reference, k);
        let true_share = truth.iter().sum::<f64>() / truth.len() as f64;
        let error = |s: &Still| {
            let t = resolved(s, 1);
            let abs: f64 = t.iter().zip(&truth).map(|(a, b)| (a - b).abs()).sum();
            abs / truth.iter().sum::<f64>()
        };
        println!(
            "beech {age}: {florets} florets; coverage reference {:.5}, chosen {:.5} ({:.3} of it, \
             per-pixel error {:.3}), finest {:.5} ({:.3}, {:.3})",
            true_share,
            coverage(&chosen),
            coverage(&chosen) / true_share,
            error(&chosen),
            coverage(&finest),
            coverage(&finest) / true_share,
            error(&finest),
        );
        let grey = |v: &[f64]| -> Vec<u8> {
            v.iter()
                .flat_map(|c| {
                    let g = (c.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0) as u8;
                    [g, g, g, 255]
                })
                .collect()
        };
        let write = |name: &str, rgba: Vec<u8>| {
            let still = Still {
                width: w,
                height: h,
                rgba,
                stats: chosen.stats,
            };
            let path = format!("{dir}/m0-beech{age}-{name}.png");
            telperion_render::write_png(std::path::Path::new(&path), &still).unwrap();
        };
        write("chosen", chosen.rgba.clone());
        write("reference", grey(&truth));
    }
    // M1 by values: every outer age flowering, twice the florets, larger.
    for age in [20.0, 40.0] {
        let mut f = blossoming(age, 1.0);
        for k in 10..15 {
            f.space.species.as_mut().unwrap().states[k].flowering = 1.0;
        }
        (f.bloom.florets, f.bloom.petal_length, f.bloom.petal_width) = (12.0, 0.02, 0.018);
        let tree = mesh::build(&f).unwrap();
        let florets = tree.organ.as_ref().map_or(0, |o| o.instances.leaves.len());
        let camera = hero_pose(tree.bounds, f64::from(w) / f64::from(h), GROUND_REACH);
        renderer.submit_at(&tree, Level::Chosen).unwrap();
        renderer.set_family(&f);
        let share = coverage(&mask(&mut renderer, &camera, w, h));
        println!("beech {age}, denser: {florets} florets; coverage chosen {share:.5}");
    }
}
