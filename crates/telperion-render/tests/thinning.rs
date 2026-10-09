//! fn-246: leaves and flowers smaller than a pixel on screen are thinned by
//! a share that follows their size, and the ones kept are enlarged so the
//! crown covers what it covered. Judged against the same crown drawn at four
//! times the resolution, where none of it is thinned.
mod common;

use telperion_core::mesh;
use telperion_render::{hero_pose, render, Renderer, Still, View, GROUND_REACH, STILL_FORMAT};

/// A delivered sRGB value as linear light, the space a box average is
/// taken in.
fn linear(v: u8) -> f64 {
    let c = f64::from(v) / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// How much the crown changes the frame: the mean linear colour difference between
/// the whole and the bare view, each averaged in blocks of `block` pixels a
/// side first, so a large still is judged at a small one's resolution.
fn covered(whole: &Still, bare: &Still, block: u32) -> f64 {
    let (w, h) = (whole.width / block, whole.height / block);
    let mean = |still: &Still, x: u32, y: u32, c: usize| {
        let mut sum = 0.0;
        for dy in 0..block {
            for dx in 0..block {
                let i = (((y * block + dy) * still.width + x * block + dx) * 4) as usize;
                sum += linear(still.rgba[i + c]);
            }
        }
        sum / f64::from(block * block)
    };
    let mut changed = 0.0;
    for y in 0..h {
        for x in 0..w {
            for c in 0..3 {
                changed += (mean(whole, x, y, c) - mean(bare, x, y, c)).abs();
            }
        }
    }
    changed / f64::from(w * h)
}

#[test]
fn a_crown_of_sub_pixel_leaves_draws_fewer_and_covers_the_same() {
    let Some(gpu) = common::gpu() else { return };
    let tree = mesh::build(&common::ordinary()).expect("the tree grew");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).expect("the tree fits the device");
    let camera = hero_pose(renderer.bounds().unwrap(), 1.0, GROUND_REACH);
    let crown = tree.foliage_instances() as u32;
    let mut pair = |size: u32| {
        renderer.set_view(View::Whole);
        let whole = render(&mut renderer, &camera, size, size).unwrap();
        let counts = renderer.level_counts().unwrap();
        renderer.set_view(View::Bare);
        let bare = render(&mut renderer, &camera, size, size).unwrap();
        (whole, bare, crown - counts.last().unwrap())
    };
    let (whole, bare, drawn) = pair(48);
    let (fine_whole, fine_bare, fine_drawn) = pair(192);
    let (small, large) = (covered(&whole, &bare, 1), covered(&fine_whole, &fine_bare, 4));
    eprintln!("drawn {drawn} of {crown} at 48 px, {fine_drawn} at 192; covered {small:.4} against {large:.4}");
    assert!(
        f64::from(drawn) < 0.5 * f64::from(fine_drawn),
        "sub-pixel leaves were not thinned: {drawn} drawn against {fine_drawn}"
    );
    assert!(
        (small - large).abs() <= 0.2 * large,
        // Unthinned, the 48 px crown read 0.0295 against 0.0261 (13% over):
        // the bound holds thinning to that resolution gap and a little more.
        "the thinned crown covers {small:.4} of the frame against {large:.4}"
    );
}
