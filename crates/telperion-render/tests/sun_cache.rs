//! fn-246: the sun's side of a frame, its wood surfaced for the map and the
//! map itself, is drawn again only when the tree, the light, the view's
//! casters, the caster stride or the person changed, never because the
//! camera moved.
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
        (
            renderer.shadow_cached(),
            still.rgba,
            renderer.shadow_depths().unwrap(),
        )
    };

    let (cached, whole, map) = frame(&mut renderer, View::Whole, &hero);
    assert!(!cached, "the first frame had no map to reuse");
    // The views' budgets settle on what the first frames asked (fn-258),
    // which draws the sun once more with its new budget.
    for _ in 0..3 {
        frame(&mut renderer, View::Whole, &hero);
    }
    assert!(
        frame(&mut renderer, View::Whole, &moved).0,
        "a camera move redrew the sun"
    );
    let (cached, _, bare_map) = frame(&mut renderer, View::Bare, &hero);
    assert!(!cached, "the bare view reused the crown's casters");
    assert_ne!(bare_map, map, "the bare view's map kept the crown");
    assert!(frame(&mut renderer, View::Bare, &moved).0);
    let (cached, again, again_map) = frame(&mut renderer, View::Whole, &hero);
    assert!(
        !cached,
        "back to the whole view, the crown's casters were not redrawn"
    );
    assert_eq!(again_map, map, "whole, bare, whole changed the map");
    let (cached, reused, _) = frame(&mut renderer, View::Whole, &hero);
    assert!(cached);
    assert_eq!(
        reused, whole,
        "a frame on the reused map is not the drawn one"
    );
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
    assert!(
        !frame(&mut renderer, View::Whole, &hero).0,
        "a moved sun reused the map"
    );
    let ground = SceneRow {
        ground_red: 0.5,
        ..sun
    };
    renderer.set_scene(ground);
    assert!(
        frame(&mut renderer, View::Whole, &hero).0,
        "the ground's colour redrew the sun"
    );
    // The person stands in the map while the room draws it (fn-310), and
    // moves only with a new tree, which forgets the map below.
    renderer.set_figure(false);
    assert!(
        !frame(&mut renderer, View::Whole, &hero).0,
        "the person left and the map kept it"
    );
    assert!(frame(&mut renderer, View::Whole, &hero).0);
    renderer.set_figure(true);
    assert!(
        !frame(&mut renderer, View::Whole, &hero).0,
        "the person came back and the map left it out"
    );
    renderer.submit(&tree).unwrap();
    assert!(
        !frame(&mut renderer, View::Whole, &hero).0,
        "a new tree reused the map"
    );
}

/// fn-258: the sun's wood shares its buffers with the camera's and its
/// budget settles on what it asked, so the sun's view is made again after
/// the first frames; it is surfaced again with its map, and the map's
/// casters stay counted, frame after frame.
#[test]
fn a_remade_sun_view_keeps_the_maps_casters_counted() {
    let Some(gpu) = common::gpu() else { return };
    let tree = mesh::build(&common::ordinary()).expect("the tree grew");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).expect("the tree fits the device");
    let hero = hero_pose(renderer.bounds().unwrap(), 1.5, GROUND_REACH);
    render(&mut renderer, &hero, 96, 64).expect("the frame was drawn");
    let casters = renderer.caster_triangles();
    assert!(casters > 0, "the sun drew no wood");
    for _ in 0..4 {
        render(&mut renderer, &hero, 96, 64).expect("the frame was drawn");
        assert_eq!(
            renderer.caster_triangles(),
            casters,
            "a remade sun view lost the map's casters"
        );
    }
}
