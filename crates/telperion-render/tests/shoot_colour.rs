//! Young wood takes the row's shoot colour and gives it up to the bark's as it
//! thickens. A row with no shoot radius has no young wood and draws the frame
//! it always drew, whatever shoot colour it states.
mod common;
#[path = "common/ramp.rs"]
mod ramp;
use ramp::{columns, draw, pixel, ramp, side};
use telperion_core::{material::MaterialParams, mesh};
use telperion_render::{Renderer, View, STILL_FORMAT};

const SHOOT: f64 = 0.1;

/// A white bark and a dark red-brown shoot, far apart in every channel.
fn birchlike() -> MaterialParams {
    MaterialParams {
        bark_red: 0.8,
        bark_green: 0.8,
        bark_blue: 0.75,
        shoot_red: 0.1,
        shoot_green: 0.06,
        shoot_blue: 0.045,
        shoot_radius: SHOOT,
        ..Default::default()
    }
}

#[test]
fn a_row_with_no_shoot_radius_draws_the_bark_whatever_shoot_colour_it_states() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = common::ordinary();
    family.skeleton.growth.max_nodes = Some(400);
    let tree = mesh::build(&family).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    let camera = telperion_render::hero_pose(tree.bounds, 1.0, telperion_render::GROUND_REACH);
    for view in [View::Whole, View::Bare] {
        renderer.set_view(view);
        let neutral = draw(&mut renderer, &camera, family.material);
        let stated = MaterialParams {
            shoot_red: 1.0,
            shoot_green: 0.0,
            shoot_blue: 1.0,
            ..family.material
        };
        let drawn = draw(&mut renderer, &camera, stated);
        assert!(neutral.rgba == drawn.rgba, "{view:?}: an inert row moved");
    }
}

#[test]
fn young_wood_takes_the_shoot_colour_and_gives_it_up_continuously_by_twice_its_radius() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&ramp()).unwrap();
    renderer.set_view(View::Bare);
    renderer.set_figure(false);
    let camera = side();
    let row = birchlike();
    let blended = draw(&mut renderer, &camera, row);
    let bark = draw(
        &mut renderer,
        &camera,
        MaterialParams {
            shoot_radius: 0.0,
            ..row
        },
    );
    let young = draw(
        &mut renderer,
        &camera,
        MaterialParams {
            bark_red: row.shoot_red,
            bark_green: row.shoot_green,
            bark_blue: row.shoot_blue,
            shoot_radius: 0.0,
            ..row
        },
    );
    let (mut previous, mut crossed, mut read) = (None::<f64>, false, 0);
    for (column, radius) in columns() {
        let (b, n, y) = (
            pixel(&blended, column),
            pixel(&bark, column),
            pixel(&young, column),
        );
        let apart = (0..3).all(|c| (y[c] - n[c]).abs() > 1);
        if radius < SHOOT * 0.97 {
            assert!(
                (0..3).all(|c| (b[c] - y[c]).abs() <= 1),
                "r {radius}: {b:?} is not young {y:?}"
            );
        }
        if radius > 2.0 * SHOOT * 1.03 {
            assert!(
                (0..3).all(|c| (b[c] - n[c]).abs() <= 1),
                "r {radius}: {b:?} is not bark {n:?}"
            );
        }
        // How far toward the shoot colour this column is, on the channel with
        // the widest gap: 1 young, 0 bark, never a step between neighbours.
        let c = (0..3).max_by_key(|&c| (y[c] - n[c]).abs()).unwrap();
        if !apart {
            continue;
        }
        let youth = f64::from(b[c] - n[c]) / f64::from(y[c] - n[c]);
        crossed |= (0.3..0.7).contains(&youth);
        if let Some(before) = previous {
            assert!(
                youth <= before + 0.03,
                "r {radius}: the blend turned back ({before} to {youth})"
            );
            assert!(
                before - youth <= 0.06,
                "r {radius}: a step from {before} to {youth}"
            );
        }
        previous = Some(youth);
        read += 1;
    }
    assert!(read > 200, "only {read} columns were read");
    assert!(crossed, "no column is between the two colours");
}

#[test]
fn mottle_still_acts_on_young_wood() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&ramp()).unwrap();
    renderer.set_view(View::Bare);
    renderer.set_figure(false);
    let camera = side();
    let plain = draw(&mut renderer, &camera, birchlike());
    let mottled = draw(
        &mut renderer,
        &camera,
        MaterialParams {
            bark_mottle_scale: 0.05,
            bark_mottle_strength: 0.8,
            ..birchlike()
        },
    );
    let moved = columns()
        .filter(|&(_, radius)| radius < SHOOT * 0.97)
        .filter(|&(column, _)| pixel(&plain, column) != pixel(&mottled, column))
        .count();
    assert!(moved > 10, "mottle moved only {moved} young columns");
}
