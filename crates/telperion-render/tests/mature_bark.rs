//! Thick wood takes the row's old bark, its colour and its furrows, over the
//! radius range the row states, the way young wood takes the shoot's colour.
//! A row whose old bark starts at zero radius has none and draws the frame it
//! always drew, whatever old bark it states.
mod common;
#[path = "common/ramp.rs"]
mod ramp;
use ramp::{columns, draw, pixel, ramp, side};
use telperion_core::{material::MaterialParams, mesh};
use telperion_render::{Renderer, View, STILL_FORMAT};

/// The radius old bark starts at and is whole by, inside the ramp's 0.04 to 0.28.
const LOW: f64 = 0.12;
const HIGH: f64 = 0.2;

/// A white bark turning near-black, far apart in every channel.
fn birchlike() -> MaterialParams {
    MaterialParams {
        bark_red: 0.8,
        bark_green: 0.8,
        bark_blue: 0.75,
        mature_red: 0.06,
        mature_green: 0.055,
        mature_blue: 0.05,
        mature_radius: LOW,
        mature_span: HIGH - LOW,
        ..Default::default()
    }
}

fn near(a: [i32; 3], b: [i32; 3]) -> bool {
    (0..3).all(|c| (a[c] - b[c]).abs() <= 1)
}

fn ramp_renderer() -> Option<Renderer> {
    let gpu = common::gpu()?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&ramp()).unwrap();
    renderer.set_view(View::Bare);
    renderer.set_figure(false);
    Some(renderer)
}

#[test]
fn a_row_with_no_mature_radius_draws_the_bark_whatever_old_bark_it_states() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = common::ordinary();
    family.skeleton.growth.max_nodes = Some(400);
    family.material.ridge_scale = 0.02;
    family.material.furrow_strength = 0.5;
    let tree = mesh::build(&family).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    let camera = telperion_render::hero_pose(tree.bounds, 1.0, telperion_render::GROUND_REACH);
    for view in [View::Whole, View::Bare] {
        renderer.set_view(view);
        let neutral = draw(&mut renderer, &camera, family.material);
        let stated = MaterialParams {
            mature_red: 1.0,
            mature_green: 0.0,
            mature_blue: 1.0,
            mature_furrow_strength: 0.0,
            mature_span: 0.1,
            ..family.material
        };
        let drawn = draw(&mut renderer, &camera, stated);
        assert!(neutral.rgba == drawn.rgba, "{view:?}: an inert row moved");
    }
}

#[test]
fn thick_wood_takes_the_old_colour_continuously_over_its_radius_range() {
    let Some(mut renderer) = ramp_renderer() else {
        return;
    };
    let camera = side();
    let row = birchlike();
    let blended = draw(&mut renderer, &camera, row);
    let young = MaterialParams {
        mature_radius: 0.0,
        ..row
    };
    let bark = draw(&mut renderer, &camera, young);
    let old = MaterialParams {
        bark_red: row.mature_red,
        bark_green: row.mature_green,
        bark_blue: row.mature_blue,
        ..young
    };
    let old = draw(&mut renderer, &camera, old);
    let (mut previous, mut crossed, mut read) = (None::<f64>, false, 0);
    for (column, radius) in columns() {
        let (b, n, o) = (
            pixel(&blended, column),
            pixel(&bark, column),
            pixel(&old, column),
        );
        if radius < LOW * 0.97 {
            assert!(near(b, n), "r {radius}: {b:?} is not the bark {n:?}");
        }
        if radius > HIGH * 1.03 {
            assert!(near(b, o), "r {radius}: {b:?} is not old bark {o:?}");
        }
        // How old this column reads on the widest channel: 0 the bark, 1
        // old, rising with the radius and never in a step.
        let c = (0..3).max_by_key(|&c| (o[c] - n[c]).abs()).unwrap();
        if (o[c] - n[c]).abs() <= 1 {
            continue;
        }
        let age = f64::from(b[c] - n[c]) / f64::from(o[c] - n[c]);
        crossed |= (0.3..0.7).contains(&age);
        if let Some(before) = previous {
            assert!(age >= before - 0.03, "r {radius}: the blend turned back");
            assert!(
                age - before <= 0.08,
                "r {radius}: a step from {before} to {age}"
            );
        }
        previous = Some(age);
        read += 1;
    }
    assert!(read > 150, "only {read} columns were read");
    assert!(crossed, "no column is between the two colours");
}

#[test]
fn thick_wood_takes_the_old_furrows_and_thin_wood_keeps_its_own() {
    let Some(mut renderer) = ramp_renderer() else {
        return;
    };
    let camera = side();
    // One colour throughout, so only the relief can tell old from young.
    let shallow = MaterialParams {
        ridge_scale: 0.03,
        plate_scale: 0.06,
        furrow_strength: 0.1,
        fissure_red: -0.5,
        fissure_green: -0.5,
        fissure_blue: -0.5,
        fissure_strength: 1.0,
        ..Default::default()
    };
    let deep = MaterialParams {
        furrow_strength: 1.0,
        ..shallow
    };
    let blended = MaterialParams {
        mature_red: shallow.bark_red,
        mature_green: shallow.bark_green,
        mature_blue: shallow.bark_blue,
        mature_furrow_strength: deep.furrow_strength,
        mature_radius: LOW,
        mature_span: HIGH - LOW,
        ..shallow
    };
    let (b, s, d) = (
        draw(&mut renderer, &camera, blended),
        draw(&mut renderer, &camera, shallow),
        draw(&mut renderer, &camera, deep),
    );
    let mut fissured = 0;
    for (column, radius) in columns() {
        let (b, s, d) = (pixel(&b, column), pixel(&s, column), pixel(&d, column));
        if radius < LOW * 0.97 {
            assert!(
                near(b, s),
                "r {radius}: {b:?} is not the shallow bark {s:?}"
            );
        }
        if radius > HIGH * 1.03 {
            assert!(near(b, d), "r {radius}: {b:?} is not the deep bark {d:?}");
            fissured += usize::from(!near(s, d));
        }
    }
    assert!(
        fissured > 10,
        "old bark's furrows moved only {fissured} columns"
    );
}
