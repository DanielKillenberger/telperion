//! fn-246: leaves and flowers under two pixels on screen are thinned by
//! a share that follows their size, and the ones kept are enlarged so the
//! crown covers what it covered. Judged against the same crown drawn at four
//! times the resolution, where none of it is thinned.
mod common;

use telperion_core::mesh;
use telperion_render::{hero_pose, render, Renderer, Still, View, GROUND_REACH, STILL_FORMAT};

/// How far the far florets may cover from the same florets at four times
/// the resolution, as a share of the latter. The floret card below 4 px is
/// the owner's decision (fn-246 FPS.md, 2026-10-09, on the labelled
/// comparison https://claude.ai/artifact/8WwaynDsif2JekPvHDDtDi), accepted
/// past the crown bound on his look. Measured: 25.4% with the card, 14.8%
/// without it, on florets along the shoots. Seated at the shoots' tips
/// (fn-253) the florets cluster and overlap, and the beech at 20 read 31
/// to 47% as only the box its florets are packed against moved: its few
/// florets left the figure to which of them thinning kept. At 40 the same
/// three boxes read 30.1, 27.6 and 28.4%, a spread of 2.5 points; the bound
/// is the first, measured with the shipped box, plus 10 points (host,
/// 2026-10-10). Re-taken at the merge of F4 (fn-254's f32 growth among
/// it): 27.5, 27.8 and 28.8%, so 37.5%. A card per inflorescence was tried and measured 87 to 131%
/// (fn-253 SEATS.md), so the floret's card stays, and the owner's eye
/// against the 4x reference is the gate this number stands behind.
const BLOSSOM_BOUND: f64 = 0.375;

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
fn a_crown_of_small_leaves_draws_fewer_and_covers_the_same() {
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
    let (whole, bare, drawn) = pair(96);
    let (fine_whole, fine_bare, fine_drawn) = pair(384);
    let (small, large) = (
        covered(&whole, &bare, 1),
        covered(&fine_whole, &fine_bare, 4),
    );
    eprintln!("drawn {drawn} of {crown} at 96 px, {fine_drawn} at 384; covered {small:.4} against {large:.4}");
    assert!(
        f64::from(drawn) < 0.5 * f64::from(fine_drawn),
        "small leaves were not thinned: {drawn} drawn against {fine_drawn}"
    );
    assert!(
        (small - large).abs() <= 0.2 * large,
        // Unthinned, the 96 px crown read 0.0305 against 0.0280 (9% over):
        // the bound holds thinning to that resolution gap and a little more.
        "the thinned crown covers {small:.4} of the frame against {large:.4}"
    );
}

/// A beech whose outer ages flower with the cherry's blossom (fn-216's
/// overlay, as `organ_coverage.rs` draws it), at 40: enough florets at its
/// shoots' tips that the card's figure no longer turns on which of them
/// thinning keeps (fn-253).
fn blossoming() -> telperion_core::Family {
    use telperion_core::{foliage::BloomParams, presets::Preset};
    let mut f = Preset::EuropeanBeech.parameters();
    (f.age, f.skeleton.seed) = (40.0, 1);
    let species = f.space.species.as_mut().unwrap();
    for k in [12, 14] {
        species.states[k].flowering = 1.0;
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

/// The share of the frame the florets cover through the organ mask, in
/// linear light.
fn floret_share(still: &Still) -> f64 {
    let pixels = still.rgba.chunks(4);
    pixels.clone().map(|p| linear(p[1])).sum::<f64>() / pixels.len() as f64
}

#[test]
fn far_florets_drawn_as_cards_cover_what_the_florets_cover() {
    let Some(gpu) = common::gpu() else { return };
    let family = blossoming();
    let tree = mesh::build(&family).expect("the tree grew");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).expect("the tree fits the device");
    renderer.set_family(&family);
    renderer.set_view(View::Clay);
    renderer.set_organ_mask(true);
    let camera = hero_pose(renderer.bounds().unwrap(), 4.0 / 3.0, GROUND_REACH);
    let (w, h) = (320, 240);
    renderer.pin_curve_viewport(Some((w, h)));
    let near = floret_share(&render(&mut renderer, &camera, w * 4, h * 4).unwrap());
    let far = floret_share(&render(&mut renderer, &camera, w, h).unwrap());
    eprintln!("florets cover {far:.5} at {w} px against {near:.5} at four times it");
    assert!(
        (far - near).abs() <= BLOSSOM_BOUND * near,
        "the far florets cover {far:.5} against {near:.5}"
    );
}
