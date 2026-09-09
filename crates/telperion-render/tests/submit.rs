//! The submission contract. What the renderer reports is what the core built,
//! a tree the device cannot hold is refused by name rather than truncated, and
//! each view draws what its name promises.
use telperion_core::{
    foliage::{Element, Instances},
    math::Vec3,
    mesh::{self, Detail, Foliage, TreeMesh},
    params,
    surface::{Bounds, SurfaceMesh},
};
use telperion_render::{
    fits, hero_pose, render, Region, Renderer, View, GROUND_REACH, STILL_FORMAT,
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

fn limit(bytes: u64) -> wgpu::Limits {
    wgpu::Limits {
        max_buffer_size: bytes,
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
fn a_submitted_tree_is_the_tree_the_core_counted() {
    let Some(gpu) = gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    // The two species the spec judges: one broad crown of half a million
    // leaves, one conifer of nearly eight million needles.
    for id in ["oregon-white-oak", "norway-spruce"] {
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
    }

    // The leaf is framed on the element itself, which is centimetres across,
    // not on the tree it was taken from.
    renderer.set_view(View::Leaf);
    let leaf = renderer.bounds().expect("the element bounds a leaf view");
    assert!(
        leaf.max.y - leaf.min.y < (tree.bounds.max.y - tree.bounds.min.y) / 10.0,
        "the leaf view frames the whole tree"
    );
}
