//! fn-246: the sun's side of a frame, its wood surfaced for the map and the
//! map itself, is drawn again only when the tree, the light, the view's
//! casters or the caster stride changed, never because the camera moved.
//! A frame that reuses them is the frame that would have drawn them.
mod common;

use telperion_core::mesh;
use telperion_render::{hero_pose, render, Renderer, SceneRow, View, GROUND_REACH, STILL_FORMAT};

#[test]
fn the_sun_is_drawn_again_only_when_what_it_sees_changed() {
    let Some(gpu) = common::gpu() else { return };
    let tree = mesh::build(&common::ordinary()).expect("the tree grew");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).expect("the tree fits the device");
    let hero = hero_pose(renderer.bounds().unwrap(), 1.0, GROUND_REACH);
    let mut moved = hero;
    moved.position.x += 1.0;
    let frame = |renderer: &mut Renderer, view: View, camera| {
        renderer.set_view(view);
        let still = render(renderer, camera, 96, 96).expect("the frame was drawn");
        (renderer.shadow_cached(), still.rgba, renderer.shadow_depths().unwrap())
    };

    let (cached, whole, map) = frame(&mut renderer, View::Whole, &hero);
    assert!(!cached, "the first frame had no map to reuse");
    assert!(frame(&mut renderer, View::Whole, &moved).0, "a camera move redrew the sun");
    let (cached, _, bare_map) = frame(&mut renderer, View::Bare, &hero);
    assert!(!cached, "the bare view reused the crown's casters");
    assert_ne!(bare_map, map, "the bare view's map kept the crown");
    assert!(frame(&mut renderer, View::Bare, &moved).0);
    let (cached, again, again_map) = frame(&mut renderer, View::Whole, &hero);
    assert!(!cached, "back to the whole view, the crown's casters were not redrawn");
    assert_eq!(again_map, map, "whole, bare, whole changed the map");
    let (cached, reused, _) = frame(&mut renderer, View::Whole, &hero);
    assert!(cached);
    assert_eq!(reused, whole, "a frame on the reused map is not the drawn one");
    assert_eq!(again, whole);

    let stride = SceneRow {
        caster_stride: 9.0,
        ..SceneRow::default()
    };
    renderer.set_scene(stride);
    let (cached, _, strided) = frame(&mut renderer, View::Whole, &hero);
    assert!(!cached, "a caster stride change reused the map");
    assert_ne!(strided, map, "the stride changed nothing in the map");
    let sun = SceneRow {
        sun_azimuth: 200.0,
        ..stride
    };
    renderer.set_scene(sun);
    assert!(!frame(&mut renderer, View::Whole, &hero).0, "a moved sun reused the map");
    let ground = SceneRow {
        ground_red: 0.5,
        ..sun
    };
    renderer.set_scene(ground);
    assert!(frame(&mut renderer, View::Whole, &hero).0, "the ground's colour redrew the sun");
    renderer.submit(&tree).unwrap();
    assert!(!frame(&mut renderer, View::Whole, &hero).0, "a new tree reused the map");
}
