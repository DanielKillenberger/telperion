//! The proof the whole path holds on real hardware: a tree from the core, up
//! through its curve, surfaced on the device and out as pixels. Skips with a
//! reason where there is no GPU to ask; it never fails for lack of one.
use telperion_core::{mesh, presets::Preset};
use telperion_render::{hero_pose, render, Renderer, GROUND_REACH, STILL_FORMAT};

mod common;
use common::gpu;

#[test]
fn a_tree_reaches_the_pixels() {
    let Some(gpu) = gpu() else { return };
    let tree = mesh::build(&Preset::Ordinary.parameters()).expect("the core built");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let submitted = renderer.submit(&tree).expect("the tree fits the device");

    assert_eq!(submitted.wood_vertices, tree.wood_vertices());
    assert_eq!(submitted.wood_triangles, tree.wood_triangles());
    assert!(
        renderer.curve_bytes().is_some_and(|b| b > 0),
        "the curve was not uploaded"
    );

    let camera = hero_pose(tree.bounds, 1.0, GROUND_REACH);
    let still = render(&mut renderer, &camera, 256, 256).expect("the frame was drawn");
    assert_eq!(still.rgba.len(), 256 * 256 * 4);
    assert!(
        still.has_subject(),
        "the still is one flat colour: nothing was drawn"
    );
    let report = renderer.curve_report().expect("the wood was surfaced");
    assert!(report.tube_triangles > 0, "no wood was surfaced");
    assert_eq!(report.overrun, 0);
}
