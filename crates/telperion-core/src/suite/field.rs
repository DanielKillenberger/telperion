//! The field read out to a grid (fn-257 R8): the grid's one definition,
//! the wood's cell rule on it, and its independence of the wood surface.
use telperion_core::{
    field::{self, Frame},
    foliage::{plan::Plan, Bounds},
    math::Vec3,
    tree::{Node, Tree},
    Error,
};

/// A plan with no leaf-bearing record.
fn bare() -> Plan {
    Plan {
        descriptors: Vec::new(),
        total: 0,
        blade: 0.0,
        seat: 1.0,
        rachis: 0.0,
    }
}

/// The grid is the voxelizer's (`src/field/voxelize.ts` `grid`): `n - 2`
/// cells span the longest axis, centred in x and z, one empty cell below;
/// a side outside 3 to 256 and a box with no extent are refused by name.
#[test]
fn the_grid_is_the_one_the_voxelizer_draws() {
    let b = Bounds {
        min: Vec3::new(0., 0., 0.),
        max: Vec3::new(2., 1., 1.),
    };
    let f = Frame::over(b, 4).unwrap();
    assert_eq!((f.n, f.cell, f.origin), (4, 1., Vec3::new(-1., -1., -1.5)));
    assert_eq!(f.cells(), 64);
    assert_eq!(f.centre(1 + 4 * (2 + 4 * 3)), Vec3::new(0.5, 1.5, 2.));
    for n in [0, 2, 257] {
        let side = Error::InvalidValue {
            field: "field grid side",
            value: n.to_string(),
        };
        assert_eq!(Frame::over(b, n).err(), Some(side));
    }
    assert!(Frame::over(b, 3).is_ok() && Frame::over(b, 256).is_ok());
    let flat = Bounds {
        min: b.min,
        max: b.min,
    };
    assert_eq!(
        Frame::over(flat, 24).err(),
        Some(Error::InvalidInput("field bounds"))
    );
}

/// A cell is wood where its circumsphere meets a node's tapered sweep, and
/// carries the thickest end radius of the sweeps it meets; with no leaf
/// record no cell is foliage. An unsolved tree is refused.
#[test]
fn tapered_wood_fills_the_cells_its_sweep_reaches() {
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
    let b = field::bounds(&tree, &bare()).unwrap().unwrap();
    assert_eq!(b.min, Vec3::new(-1., -1., -1.));
    assert_eq!(b.max, Vec3::new(1., 4.5, 1.));
    let frame = Frame::over(b, 13).unwrap();
    let grid = field::read(&tree, &bare(), frame).unwrap();
    let at = |p: Vec3| {
        let i = |v: f64, o: f64| ((v - o) / frame.cell).floor() as usize;
        let n = frame.n as usize;
        let o = frame.origin;
        grid.cells[i(p.x, o.x) + n * (i(p.y, o.y) + n * i(p.z, o.z))]
    };
    let mid = at(Vec3::new(0., 2., 0.));
    assert!(mid.wood && mid.wood_radius == 1. && !mid.foliage && mid.limb.is_none());
    assert!(!at(Vec3::new(3., 4.8, 3.)).wood);
    assert!(grid.cells.iter().all(|c| !c.foliage && c.leaves == 0.));
    let unsolved = Tree {
        nodes: vec![Node::root()],
        ..Tree::default()
    };
    assert!(field::bounds(&unsolved, &bare()).is_err());
    assert!(field::read(&unsolved, &bare(), frame).is_err());
}

#[test]
fn the_field_reads_no_wood_surface() {
    // These modules' imports are the field's whole path. The field and
    // generation sources must stay independent of the wood surface builder.
    for source in [
        include_str!("../pipeline/field.rs"),
        include_str!("../pipeline/field/splat.rs"),
        include_str!("../pipeline/branching.rs"),
        include_str!("../pipeline/foliage.rs"),
    ] {
        assert!(!source.contains("surface::"));
    }
}

#[test]
fn species_geometry_bounds_and_culling_cover_transformed_connectors_and_units() {
    use telperion_core::{
        envelope::Envelope,
        foliage::{build_element, cull, transform_point, ElementParams, Instances, Reference},
    };
    // One uniform scale: a packed leaf carries a rotation, a scale and a
    // point, so the three columns are one basis at one length.
    let matrix = [
        0., 0., 3., 0., 3., 0., 0., 0., 0., 3., 0., 0., 0., 15., 0., 1.,
    ];
    let mut instances = Instances::new(Reference::spanning(
        Vec3::new(-0.1, 14.9, -0.1),
        Vec3::new(0.1, 15.1, 0.1),
    ));
    instances.push(&matrix);
    let stored = instances.matrix(0);
    for section_roundness in [0.0, 1.0] {
        let element = build_element(ElementParams {
            section_roundness,
            lobe_count: if section_roundness > 0.0 { 0 } else { 5 },
            lobe_depth: if section_roundness > 0.0 { 0.0 } else { 0.7 },
            axial_segments: 12,
            cross_segments: 4,
            length: 0.02,
            width: 0.002,
            connector_length: 0.001,
            ..ElementParams::default()
        })
        .unwrap();
        let kept = cull(instances.clone(), &element, Envelope::default(), 1.).unwrap();
        assert_eq!(kept, instances);
        let bounds = kept.bounds(&element).unwrap().unwrap();
        for v in &element.positions {
            assert!(bounds.contains(transform_point(&stored, *v)));
        }
        let mut degenerate = element.clone();
        degenerate.positions[1] = degenerate.positions[0];
        assert!(kept.bounds(&degenerate).is_err());
    }
}
