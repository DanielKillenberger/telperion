use telperion_core::{field::Field, math::Vec3, tree::Tree};
#[test]
fn empty_and_invalid_queries() {
    let field = Field::new(&Tree::default(), None).unwrap();
    assert_eq!(field.bounds(), None);
    let empty = field.query(Vec3::ZERO, 0.).unwrap();
    assert!(!empty.wood && !empty.foliage);
    for (p, h) in [
        (Vec3::new(f64::NAN, 0., 0.), 0.),
        (Vec3::ZERO, -1.),
        (Vec3::ZERO, f64::INFINITY),
    ] {
        assert!(field.query(p, h).is_err());
    }
}

use std::time::Instant;
use telperion_core::{
    branching,
    foliage::{self, Element, Instances, TwigPlacement},
    presets::Preset,
    tree::Node,
};

#[test]
fn tapered_wood_and_flat_leaf_cells() {
    let mut root = Node::root();
    root.radius = 1.;
    root.start_radius = 1.;
    let mut tip = root.clone();
    tip.position = Vec3::new(0., 4., 0.);
    tip.parent = Some(0);
    tip.radius = 0.5;
    let tree = Tree {
        nodes: vec![root, tip],
        crossover: 2,
        ..Tree::default()
    };
    let element = Element {
        positions: vec![Vec3::new(-0.5, 0., 0.), Vec3::new(0.5, 1., 0.)],
        indices: vec![],
        anatomy: None,
    };
    let instances = Instances {
        matrices: vec![[
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 2., 0., 1.,
        ]],
    };
    let field = Field::new(&tree, Some((&instances, &element))).unwrap();
    let both = field.query(Vec3::new(0., 2., 0.), 0.).unwrap();
    assert!(both.wood && both.foliage);
    assert!(field.query(Vec3::new(0.5, 3., 0.), 0.).unwrap().foliage);
    assert!(!field.query(Vec3::new(0.5, 3., 0.01), 0.).unwrap().foliage);
    assert!(field.query(Vec3::new(0.5, 3., 0.01), 0.01).unwrap().foliage);
    assert!(field.query(Vec3::new(1., 0., 0.), 0.).unwrap().wood);
    assert!(!field.query(Vec3::new(0.9, 4., 0.), 0.).unwrap().wood);
    assert!(field.query(Vec3::new(0.5, 4., 0.), 0.).unwrap().wood);
    for half in [0., 0.01, 0.1, 0.5] {
        let q = field.query(Vec3::new(0., 2., 0.), half).unwrap();
        assert!(q.wood && q.foliage);
        assert_eq!(q, field.query(Vec3::new(0., 2., 0.), half).unwrap());
    }
    assert!(field.query(Vec3::new(f64::MAX, 0., 0.), f64::MAX).is_err());
    assert!(Field::new(
        &Tree {
            nodes: vec![Node::root()],
            ..Tree::default()
        },
        None
    )
    .is_err());
    let mut bad = instances.clone();
    bad.matrices[0][0] = f32::NAN;
    assert!(Field::new(&tree, Some((&bad, &element))).is_err());
    let empty = Element {
        positions: vec![],
        indices: vec![],
        anatomy: None,
    };
    assert!(
        !Field::new(&tree, Some((&instances, &empty)))
            .unwrap()
            .query(Vec3::new(0., 2., 0.), 0.)
            .unwrap()
            .foliage
    );
}

#[test]
fn generated_block_consumer_and_giant_samples_without_surface() {
    // These modules' imports are the complete generation path. The field and
    // generation sources must stay independent of the wood surface builder.
    for source in [
        include_str!("../src/field.rs"),
        include_str!("../src/branching.rs"),
        include_str!("../src/foliage.rs"),
    ] {
        assert!(!source.contains("surface::"));
    }
    for preset in [Preset::Ordinary, Preset::Telperion] {
        let family = preset.parameters();
        let tree = branching::generate(&family.skeleton, family.radii)
            .unwrap()
            .tree;
        let element = foliage::build_element(family.element).unwrap();
        let placed = foliage::place(
            &tree,
            family.skeleton.envelope,
            family.skeleton.seed,
            family.canopy,
            Some(TwigPlacement {
                internode_length: family.skeleton.twigs.twig.internode_length,
                stations_per_internode: family.skeleton.twigs.twig.stations_per_internode,
            }),
        )
        .unwrap();
        let retained = foliage::cull(
            &placed,
            &element,
            family.skeleton.envelope,
            family.shell_depth,
        )
        .unwrap();
        let start = Instant::now();
        let field = Field::new(&tree, Some((&retained, &element))).unwrap();
        let build = start.elapsed();
        let bounds = field.bounds().unwrap();
        let start = Instant::now();
        let mut wood = 0;
        let mut leaves = 0;
        // A native block consumer chooses its own grid resolution and packs two
        // material bits. No surface output, voxelizer or per-tree mesh is built.
        let cells = 32;
        let span = bounds.max - bounds.min;
        let step = span.x.max(span.y).max(span.z) / cells as f64;
        for x in 0..cells {
            for y in 0..cells {
                for z in 0..cells {
                    let center = bounds.min
                        + Vec3::new(x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5) * step;
                    let flags = field.query(center, step / 2.).unwrap();
                    wood += usize::from(flags.wood);
                    leaves += usize::from(flags.foliage);
                }
            }
        }
        let query = start.elapsed();
        assert!(wood > 0 && leaves > 0);
        for m in retained
            .matrices
            .iter()
            .step_by((retained.matrices.len() / 100).max(1))
        {
            let p = foliage::transform_point(m, element.positions[0]);
            assert!(field.query(p, 0.001).unwrap().foliage);
        }
        assert_eq!(
            field
                .query(bounds.max + Vec3::new(10., 10., 10.), 0.)
                .unwrap(),
            Default::default()
        );
        eprintln!("nodes={} retained={} build_ms={:.2} queries={} query_ms={:.2} owned_capacity_bytes={} wood_cells={} foliage_cells={}",tree.nodes.len(),retained.matrices.len(),build.as_secs_f64()*1000.,cells*cells*cells,query.as_secs_f64()*1000.,field.storage_bytes(),wood,leaves);
        if matches!(preset, Preset::Telperion) {
            assert!(retained.matrices.len() > 1_000_000);
        }
    }
}

#[test]
fn species_geometry_bounds_culling_and_field_cover_transformed_connectors_and_units() {
    use telperion_core::{
        envelope::Envelope,
        foliage::{build_element, cull, transform_point, ElementAnatomy, ElementParams},
    };
    let matrix = [
        0., 0., 2., 0., 3., 0., 0., 0., 0., 4., 0., 0., 0., 15., 0., 1.,
    ];
    let instances = Instances {
        matrices: vec![matrix],
    };
    for anatomy in [ElementAnatomy::LobedBlade, ElementAnatomy::FourSidedNeedle] {
        let element = build_element(ElementParams {
            anatomy,
            length: 0.02,
            width: 0.002,
            connector_length: 0.001,
            ..ElementParams::default()
        })
        .unwrap();
        let kept = cull(&instances, &element, Envelope::default(), 1.).unwrap();
        assert_eq!(kept, instances);
        let bounds = kept.bounds(&element).unwrap().unwrap();
        let field = Field::new(&Tree::default(), Some((&kept, &element))).unwrap();
        for v in &element.positions {
            let world = transform_point(&matrix, *v);
            assert!(bounds.contains(world));
            assert!(field.query(world, 0.).unwrap().foliage);
        }
        let mut malformed = element.clone();
        malformed.anatomy.as_mut().unwrap().vertices.end += element.positions.len();
        assert!(Field::new(&Tree::default(), Some((&kept, &malformed))).is_err());
        let mut degenerate = element.clone();
        degenerate.positions[1] = degenerate.positions[0];
        assert!(kept.bounds(&degenerate).is_err());
    }
}

#[test]
fn portable_snapshot_is_optional_owned_and_preserves_index() {
    let empty = Field::new(&Tree::default(), None)
        .unwrap()
        .snapshot()
        .unwrap();
    assert!(empty.wood.is_empty());
    assert_eq!(empty.wood_index.node_count, 0);
    assert!(empty.leaves.bounds.is_empty());
    let mut root = Node::root();
    root.radius = 1.;
    root.start_radius = 1.;
    let tree = Tree {
        nodes: vec![root],
        crossover: 1,
        ..Default::default()
    };
    let field = Field::new(&tree, None).unwrap();
    let before = field.storage_bytes();
    let mut snapshot = field.snapshot().unwrap();
    assert_eq!(field.storage_bytes(), before);
    assert_eq!(snapshot.wood, vec![0., 0., 0., 0., 0., 0., 1., 1.]);
    assert_eq!(snapshot.wood_index.node_count, 1);
    assert_eq!(
        snapshot.wood_index.topology,
        vec![0, 1, u32::MAX, u32::MAX, 0]
    );
    assert_eq!(
        snapshot.wood_index.bounds,
        vec![-1., -1., -1., 1., 1., 1., -1., -1., -1., 1., 1., 1.]
    );
    snapshot.wood[6] = 0.;
    assert!(field.query(Vec3::new(1., 0., 0.), 0.).unwrap().wood);
    drop(field);
    assert_eq!(snapshot.wood[7], 1.);
}
