//! The proof the whole path holds on real hardware: a tree from the core, up
//! through the wood buffers, out as pixels. Skips with a reason where there is
//! no GPU to ask; it never fails for lack of one.
use telperion_core::{
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{hero_pose, render, Renderer, GROUND_REACH, STILL_FORMAT};

mod common;
use common::gpu;

#[test]
fn a_tree_reaches_the_pixels() {
    let Some(gpu) = gpu() else { return };
    let tree = mesh::build(&Preset::Ordinary.parameters(), Detail::Full).expect("the core built");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let submitted = renderer.submit(&tree).expect("the tree fits the device");

    assert_eq!(submitted.wood_vertices, tree.wood_vertices());
    assert_eq!(submitted.wood_triangles, tree.wood_triangles());
    let (positions, normals, indices) = renderer.wood_regions().expect("the wood was uploaded");
    for region in [positions, normals, indices] {
        assert!(
            region.capacity() > region.used(),
            "no headroom above {} bytes",
            region.used()
        );
    }
    assert_eq!(positions.used(), (tree.wood.positions.len() * 4) as u64);
    assert_eq!(indices.used(), (tree.wood.indices.len() * 4) as u64);

    let camera = hero_pose(tree.bounds, 1.0, GROUND_REACH);
    let still = render(&mut renderer, &camera, 256, 256).expect("the frame was drawn");
    assert_eq!(still.rgba.len(), 256 * 256 * 4);
    assert!(
        still.has_subject(),
        "the still is one flat colour: nothing was drawn"
    );
    // The room's own triangles ride along with the tree's.
    assert!(still.stats.triangles > tree.wood_triangles() as u32);
}

#[test]
fn a_second_smaller_tree_reuses_the_wood_buffers() {
    let Some(gpu) = gpu() else { return };
    let large = mesh::build(&Preset::Ordinary.parameters(), Detail::Full).expect("the core built");
    let mut small = Preset::Ordinary.parameters();
    small.skeleton.envelope.height *= 0.5;
    let small = mesh::build(&small, Detail::Full).expect("the core built the smaller tree");
    assert!(
        small.wood_vertices() < large.wood_vertices(),
        "the second tree was not smaller"
    );

    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&large).expect("the large tree fits");
    let (before, ..) = renderer.wood_regions().expect("the wood was uploaded");
    renderer.submit(&small).expect("the small tree fits");
    let (after, ..) = renderer
        .wood_regions()
        .expect("the second tree was uploaded");

    assert_eq!(
        after.capacity(),
        before.capacity(),
        "a smaller tree forced a re-layout"
    );
    assert!(
        after.used() < before.used(),
        "the live range did not shrink"
    );

    let camera = hero_pose(small.bounds, 1.0, GROUND_REACH);
    let still = render(&mut renderer, &camera, 256, 256).expect("the second frame was drawn");
    assert!(still.has_subject(), "the reused buffers drew nothing");
}
