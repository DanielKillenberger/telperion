//! The submission contract. What the renderer reports is what the core built,
//! a tree the device cannot hold is refused by name rather than truncated, and
//! each view draws what its name promises.
use telperion_core::{
    foliage::{build_element, Element, ElementParams, Instances, Level as Section},
    math::Vec3,
    mesh::{self, Detail, Foliage, TreeMesh},
    params,
    surface::{Bounds, SurfaceMesh},
};
use telperion_render::{
    fits, hero_pose, render, Level, Region, Renderer, View, GROUND_REACH, MAX_LEVELS, STILL_FORMAT,
};

mod common;
use common::gpu;

/// A mesh with no geometry to speak of, so a test can make one buffer of it
/// large and leave the others out of the way.
fn small() -> TreeMesh {
    TreeMesh {
        wood: SurfaceMesh {
            positions: vec![0.0; 12],
            normals: vec![0.0; 12],
            indices: vec![0; 6],
            bounds: None,
            runs: 1,
        },
        foliage: Foliage {
            element: Element::default(),
            instances: Instances {
                matrices: vec![[0.0; 16]; 2],
            },
        },
        bounds: Bounds {
            min: Vec3::ZERO,
            max: Vec3::Y,
        },
    }
}

/// A mesh with no wood at all and an element carrying this many levels over
/// this many indices: what the selection buffers are sized from, and nothing
/// else in the way of judging them.
fn crown(levels: usize, indices: usize, instances: usize) -> TreeMesh {
    let mut mesh = small();
    mesh.wood = SurfaceMesh {
        positions: Vec::new(),
        normals: Vec::new(),
        indices: Vec::new(),
        bounds: None,
        runs: 0,
    };
    mesh.foliage.element = Element {
        level_indices: vec![0; indices],
        levels: (0..levels)
            .map(|n| Section {
                indices: 0..3,
                deviation: 1.0 / (n + 1) as f64,
            })
            .collect(),
        ..Element::default()
    };
    mesh.foliage.instances.matrices = vec![[0.0; 16]; instances];
    mesh
}

fn limit(bytes: u64) -> wgpu::Limits {
    wgpu::Limits {
        max_buffer_size: bytes,
        max_storage_buffer_binding_size: bytes,
        ..Default::default()
    }
}

#[test]
fn a_tree_is_judged_on_the_allocation_it_needs_not_the_bytes_it_holds() {
    let mesh = small();
    fits(&limit(4_096), &mesh).expect("a mesh of a few hundred bytes fits four kilobytes");

    // The buffers are taken with headroom, and the headroom is what the device
    // has to grant: a payload that exactly fills the limit does not fit.
    let payload = (mesh.wood.positions.len() * size_of::<f32>()) as u64;
    assert!(
        Region::capacity_for(payload) > payload,
        "no headroom to judge"
    );
    let error = fits(&limit(payload), &mesh)
        .expect_err("a payload that fills the limit leaves nothing for its headroom");
    assert!(error.to_string().contains("wood positions"), "{error}");
}

#[test]
fn every_buffer_that_will_not_fit_is_refused_by_name_and_by_size() {
    let granted = 4_096;
    let over = (granted / size_of::<f32>() as u64) as usize;
    let cases: [(&str, TreeMesh); 4] = [
        ("wood positions", {
            let mut mesh = small();
            mesh.wood.positions = vec![0.0; over];
            mesh
        }),
        ("wood normals", {
            let mut mesh = small();
            mesh.wood.normals = vec![0.0; over];
            mesh
        }),
        ("wood indices", {
            let mut mesh = small();
            mesh.wood.indices = vec![0; over];
            mesh
        }),
        ("foliage instances", {
            let mut mesh = small();
            mesh.foliage.instances.matrices = vec![[0.0; 16]; 100];
            mesh
        }),
    ];
    for (buffer, mesh) in cases {
        let message = fits(&limit(granted), &mesh)
            .expect_err("a buffer over the limit went through")
            .to_string();
        assert!(
            message.contains(buffer),
            "the buffer is not named: {message}"
        );
        assert!(
            message.contains(&granted.to_string()),
            "the granted limit is not in the message: {message}"
        );
        let wanted = message
            .split_whitespace()
            .find_map(|word| word.parse::<u64>().ok())
            .expect("the message carries the size that was wanted");
        assert!(
            wanted > granted,
            "the size named is not the one that overflowed: {message}"
        );
    }
}

#[test]
fn every_buffer_selection_needs_is_judged_and_named_before_the_upload() {
    // Each case is the smallest limit that leaves the buffer under test the
    // first one that will not fit, so the name in the message is that
    // buffer's and no earlier one's.
    let cases: [(&str, u64, TreeMesh); 4] = [
        ("foliage level indices", 4_096, crown(3, 2_000, 1)),
        // A list is aligned up to a bindable offset, so a crown of one leaf
        // still costs one alignment per level.
        ("foliage level lists", 800, crown(3, 10, 1)),
        ("foliage level counters", 50, crown(MAX_LEVELS, 4, 0)),
        ("foliage indirect arguments", 100, crown(MAX_LEVELS, 4, 0)),
    ];
    for (buffer, granted, mesh) in cases {
        let message = fits(&limit(granted), &mesh)
            .expect_err("a selection buffer over the limit went through")
            .to_string();
        assert!(
            message.contains(buffer),
            "{buffer} is not the buffer named: {message}"
        );
    }
    // The same crowns fit a device that grants enough for them.
    for mesh in [crown(3, 2_000, 1), crown(MAX_LEVELS, 4, 0)] {
        fits(&limit(1 << 20), &mesh).expect("a kilobyte-sized crown does not fit a megabyte");
    }
}

#[test]
fn a_ladder_longer_than_selection_can_tally_is_refused_by_its_length() {
    let mesh = crown(MAX_LEVELS + 1, 10, 1);
    let message = fits(&limit(1 << 20), &mesh)
        .expect_err("a ladder past the tally went through")
        .to_string();
    assert!(
        message.contains(&(MAX_LEVELS + 1).to_string())
            && message.contains(&MAX_LEVELS.to_string()),
        "the refusal names neither the ladder nor the limit: {message}"
    );
    fits(&limit(1 << 20), &crown(MAX_LEVELS, 10, 1)).expect("a ladder the tally holds was refused");
}

#[test]
fn a_crown_with_no_leaves_submits_draws_and_frames() {
    let Some(gpu) = gpu() else { return };
    let mut mesh = crown(0, 0, 0);
    mesh.foliage.element = build_element(ElementParams::default()).expect("the core built a leaf");
    mesh.wood = small().wood;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let submitted = renderer
        .submit(&mesh)
        .expect("an empty crown fits any device");
    assert_eq!(submitted.foliage_instances, 0);

    let camera = hero_pose(mesh.bounds, 1.0, GROUND_REACH);
    let still = render(&mut renderer, &camera, 64, 64).expect("the frame was drawn");
    assert_eq!(
        still.stats.instances, 0,
        "a crown with no leaves drew leaves"
    );
    assert!(
        still.has_subject(),
        "the room around the empty crown was not drawn either"
    );
}

#[test]
fn a_submitted_tree_is_the_tree_the_core_counted() {
    let Some(gpu) = gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    // The two species the spec judges: one broad crown of half a million
    // leaves, one evergreen of nearly eight million needles. The oak's
    // coarsest level deviates 13 mm from its own outline and the needle's
    // 0.7 mm, so eight times the pixels outgrows the oak's coarsest level and
    // still does not reach half a pixel of the needle's.
    for (id, outgrows) in [("oregon-white-oak", true), ("norway-spruce", false)] {
        let family = params::by_identity(id).expect("a shipped family");
        let tree = mesh::build(&family, Detail::Full).expect("the core built the tree");
        let submitted = renderer.submit(&tree).expect("the tree fits the device");

        assert_eq!(submitted.wood_vertices, tree.wood_vertices(), "{id} wood");
        assert_eq!(
            submitted.wood_triangles,
            tree.wood_triangles(),
            "{id} triangles"
        );
        assert_eq!(
            submitted.foliage_instances,
            tree.foliage_instances(),
            "{id} instances"
        );
        assert_eq!(submitted.bounds, tree.bounds, "{id} bounds");

        let instances = renderer
            .foliage_region()
            .expect("the placements were uploaded");
        assert_eq!(
            instances.used(),
            (tree.foliage_instances() * size_of::<[f32; 16]>()) as u64,
            "{id}: the live instance range is not the crown that was built"
        );

        // Every leaf leaves the pass in exactly one place: a level, or the
        // bucket for the ones the frame does not show.
        let levels = tree.foliage.element.levels.len();
        let camera = hero_pose(tree.bounds, 1.0, GROUND_REACH);
        render(&mut renderer, &camera, 256, 256).expect("the frame was drawn");
        let counted = renderer.level_counts().expect("the pass counted");
        assert_eq!(
            counted.len(),
            levels + 1,
            "{id}: a counter per level and one"
        );
        assert_eq!(
            counted.iter().sum::<u32>(),
            tree.foliage_instances() as u32,
            "{id}: the levels and the unseen bucket do not account for the crown: {counted:?}"
        );
        // The whole tree in 256 pixels puts every leaf's deviation far under
        // half a pixel, so the coarsest level is the one that can carry it.
        assert_eq!(
            counted[0] + counted[levels],
            tree.foliage_instances() as u32,
            "{id}: a leaf under half a pixel was drawn finer than the coarsest: {counted:?}"
        );

        // The same tree in eight times the pixels does not fit in the coarsest
        // level any more: the decision is the projected deviation, not a
        // constant.
        render(&mut renderer, &camera, 2_048, 2_048).expect("the larger frame was drawn");
        let near = renderer.level_counts().expect("the pass counted");
        assert!(
            near[0] <= counted[0],
            "{id}: more pixels chose a coarser level: {near:?}"
        );
        assert_eq!(
            near[0] < counted[0],
            outgrows,
            "{id}: eight times the pixels was expected to outgrow the coarsest level: \
             {outgrows}, and the counts went {counted:?} to {near:?}"
        );

        // Forcing a level puts every leaf the frame shows in it and nowhere
        // else, which is how a measurement asks what one level costs.
        let forced = levels - 1;
        renderer
            .submit_at(&tree, Level::Forced(forced as u32))
            .expect("the tree fits the device");
        render(&mut renderer, &camera, 256, 256).expect("the forced frame was drawn");
        let held = renderer.level_counts().expect("the pass counted");
        assert_eq!(
            held[forced] + held[levels],
            tree.foliage_instances() as u32,
            "{id}: forcing level {forced} left leaves elsewhere: {held:?}"
        );
    }
}

#[test]
fn each_view_draws_what_its_name_promises() {
    let Some(gpu) = gpu() else { return };
    let family = params::by_identity("ordinary").expect("a shipped family");
    let tree = mesh::build(&family, Detail::Full).expect("the core built the tree");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let submitted = renderer.submit(&tree).expect("the tree fits the device");
    assert!(
        submitted.foliage_instances > 0,
        "the tree has no crown to hide"
    );

    let mut selected = None;
    for (view, instances) in [
        (View::Whole, submitted.foliage_instances as u32),
        (View::Bare, 0),
        (View::Leaf, 1),
    ] {
        renderer.set_view(view);
        let bounds = renderer.bounds().expect("the view has something to frame");
        let camera = hero_pose(bounds, 1.0, GROUND_REACH);
        let still = render(&mut renderer, &camera, 256, 256).expect("the frame was drawn");
        assert!(
            still.has_subject(),
            "{view:?} is one flat colour: nothing was drawn"
        );
        assert_eq!(
            still.stats.instances, instances,
            "{view:?} drew the wrong crown"
        );
        // Only the whole view selects. The counters the whole view left behind
        // are still there after the other two, because neither ran the pass
        // that clears and fills them.
        let counted = renderer.level_counts().expect("the crown has counters");
        match view {
            View::Whole => selected = Some(counted),
            _ => assert_eq!(
                selected.as_ref(),
                Some(&counted),
                "{view:?} ran the selection pass"
            ),
        }
    }
    // The leaf view is one instance of the element whole, in one draw, with
    // no room and no wood around it.
    renderer.set_view(View::Leaf);
    let bounds = renderer.bounds().expect("the leaf frames itself");
    let leaf = render(
        &mut renderer,
        &hero_pose(bounds, 1.0, GROUND_REACH),
        256,
        256,
    )
    .expect("the frame was drawn");
    assert_eq!(leaf.stats.draw_calls, 1, "the leaf view is one draw");
    assert_eq!(
        leaf.stats.triangles,
        (tree.foliage.element.indices.len() / 3) as u32,
        "the leaf view drew something other than the element whole"
    );

    // The leaf is framed on the element itself, which is centimetres across,
    // not on the tree it was taken from.
    renderer.set_view(View::Leaf);
    let leaf = renderer.bounds().expect("the element bounds a leaf view");
    assert!(
        leaf.max.y - leaf.min.y < (tree.bounds.max.y - tree.bounds.min.y) / 10.0,
        "the leaf view frames the whole tree"
    );
}
