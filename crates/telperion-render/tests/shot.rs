//! A still that imitates a photograph: an authored shot poses the same tree,
//! and the room keeps its floor and leaves its figure out. The fixed views are
//! not touched by either flag being off.
mod common;
use telperion_core::{
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{
    hero_pose, render, shot_pose, Renderer, Shot, View, GROUND_REACH, STILL_FORMAT,
};

/// The figure is a capsule of 2 * 8 + 1 rings of 16 segments, two triangles each.
const FIGURE_TRIANGLES: u32 = (2 * 8 + 1) * 16 * 2;

#[test]
fn a_shot_poses_the_frame_and_the_figure_leaves_when_asked() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::Ordinary.parameters();
    family.skeleton.growth.max_nodes = Some(400);
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    renderer.set_material(family.material);
    renderer.set_view(View::Bare);
    let bounds = renderer.bounds().unwrap();
    let (width, height) = (160u32, 200u32);
    let aspect = f64::from(width) / f64::from(height);

    let hero = render(
        &mut renderer,
        &hero_pose(bounds, aspect, GROUND_REACH),
        width,
        height,
    )
    .unwrap();
    let shot = Shot {
        azimuth: 200.0,
        elevation: -5.0,
        fill: 0.95,
        ..Default::default()
    };
    let posed = render(
        &mut renderer,
        &shot_pose(bounds, aspect, GROUND_REACH, &shot),
        width,
        height,
    )
    .unwrap();
    assert!(posed.stats.triangles > 0, "the shot drew nothing");
    assert_ne!(
        hero.rgba, posed.rgba,
        "a different shot is a different picture"
    );
    assert_eq!(
        hero.stats.triangles, posed.stats.triangles,
        "the pose changes no geometry"
    );

    renderer.set_figure(false);
    let unfurnished = render(
        &mut renderer,
        &shot_pose(bounds, aspect, GROUND_REACH, &shot),
        width,
        height,
    )
    .unwrap();
    assert_eq!(
        posed.stats.triangles - unfurnished.stats.triangles,
        FIGURE_TRIANGLES,
        "leaving the figure out removes exactly the figure"
    );
    assert_eq!(
        posed.stats.draw_calls, unfurnished.stats.draw_calls,
        "the floor still draws"
    );

    renderer.set_figure(true);
    let again = render(
        &mut renderer,
        &hero_pose(bounds, aspect, GROUND_REACH),
        width,
        height,
    )
    .unwrap();
    assert_eq!(
        hero.rgba, again.rgba,
        "the fixed view is byte-identical after a shot"
    );
}
