//! The plan against hand-built trees: counts, systems, reach, refusals.
use super::*;
use crate::tree::Node;

/// A trunk with two limbs, each carrying one twig run of two segments.
fn two_limbs() -> Tree {
    let node = |parent: u32, x: f64, y: f64, r: f64, kind: NodeKind| Node {
        parent: Some(parent),
        position: Vec3::new(x, y, 0.0),
        radius: r,
        start_radius: r,
        kind,
        ..Node::root()
    };
    let mut root = Node::root();
    root.radius = 0.2;
    root.start_radius = 0.2;
    let nodes = vec![
        root,
        node(0, 0.0, 1.0, 0.2, NodeKind::Structural),
        node(1, -1.0, 2.0, 0.1, NodeKind::Structural),
        node(1, 1.0, 2.0, 0.1, NodeKind::Structural),
        node(2, -1.5, 2.5, 0.01, NodeKind::Twig),
        node(4, -2.0, 3.0, 0.005, NodeKind::Twig),
        node(3, 1.5, 2.5, 0.01, NodeKind::Twig),
        node(6, 2.0, 3.0, 0.005, NodeKind::Twig),
    ];
    Tree {
        crossover: 4,
        nodes,
        ..Tree::default()
    }
}

#[test]
fn descriptors_carry_counts_systems_and_reach() {
    let tree = two_limbs();
    let twig = Some(TwigPlacement {
        internode_length: 0.5,
        stations_per_internode: 2,
    });
    let element = crate::pipeline::foliage::build_element(Default::default()).unwrap();
    let p = CanopyParams {
        size: 2.0,
        size_variation: 0.5,
        ..Default::default()
    };
    let at = |order| {
        plan(
            &tree,
            Envelope::default(),
            p,
            twig,
            &SurfaceParams::default(),
            &element,
            order,
            1,
        )
        .unwrap()
        .unwrap()
    };
    let plan = at(None);
    // Two runs of two segments, each segment about 0.71 m: three
    // internodes of two stations, the first two internodes on the first
    // segment and the third on the second. The first limb carries the
    // stem's system on; the second opens its own.
    assert_eq!(plan.descriptors.len(), 4);
    assert_eq!(plan.total, 12);
    let counts: Vec<u32> = plan.descriptors.iter().map(|d| d.count).collect();
    assert_eq!(counts, [4, 2, 4, 2]);
    let systems = |plan: &Plan| {
        plan.descriptors
            .iter()
            .map(|d| d.system)
            .collect::<Vec<_>>()
    };
    assert_eq!(systems(&plan), [1, 1, 3, 3]);
    // No order is the family's order; order zero keeps the second limb on
    // the stem's system, order one and up opens its own.
    assert_eq!(plan, at(Some(p.clump_system_order)));
    assert_eq!(systems(&at(Some(0))), [1, 1, 1, 1]);
    assert_eq!(systems(&at(Some(1))), [1, 1, 3, 3]);
    let extent = element
        .positions
        .iter()
        .map(|v| v.length())
        .fold(0.0, f64::max);
    assert_eq!(plan.blade, extent * 3.0);
    assert_eq!(plan.seat, 1.0);
    assert_eq!(plan.reach(&plan.descriptors[0]), 0.01 + plan.blade);
    let seated = seating(
        &SurfaceParams::default(),
        CanopyParams {
            surface_contact: 0.5,
            ..p
        },
    );
    assert!(seated > 1.0);
}

#[test]
fn unsupported_families_have_no_plan_and_bad_rows_are_refused() {
    let tree = two_limbs();
    let element = crate::pipeline::foliage::build_element(Default::default()).unwrap();
    let twig = Some(TwigPlacement::default());
    let none = |p: CanopyParams, twig| {
        plan(
            &tree,
            Envelope::default(),
            p,
            twig,
            &SurfaceParams::default(),
            &element,
            None,
            1,
        )
    };
    assert!(none(CanopyParams::default(), None).unwrap().is_none());
    let spurs = CanopyParams {
        short_shoot_spacing: 0.03,
        ..Default::default()
    };
    assert!(none(spurs, twig).unwrap().is_none());
    let clumped = CanopyParams {
        limb_clumping: 0.25,
        ..Default::default()
    };
    assert!(none(clumped, twig).unwrap().is_none());
    let bad = CanopyParams {
        scatter: 91.0,
        ..Default::default()
    };
    assert!(none(bad, twig).is_err());
    let budget = CanopyParams {
        max_instances: 1,
        ..Default::default()
    };
    assert!(none(budget, twig).is_err());
    let bare = CanopyParams {
        size: 0.0,
        ..Default::default()
    };
    assert!(none(bare, twig).unwrap().unwrap().descriptors.is_empty());
}

/// A rosette is planned frond by frond, living then dead, each frond's
/// leaflets counted along its chords; a family whose placements are leaflets
/// along a rachis is planned by runs, each station counted once per leaflet
/// and the reach grown by the rachis.
#[test]
fn a_rosette_is_planned_by_fronds_and_leaflets_multiply_the_counts() {
    let mut tree = two_limbs();
    tree.nodes[1].stem = true;
    let element = crate::pipeline::foliage::build_element(Default::default()).unwrap();
    let twig = Some(TwigPlacement {
        internode_length: 0.5,
        stations_per_internode: 2,
    });
    let at = |p: CanopyParams| {
        plan(
            &tree,
            Envelope::default(),
            p,
            twig,
            &SurfaceParams::default(),
            &element,
            None,
            1,
        )
        .unwrap()
    };
    let crown = CanopyParams {
        rosette_fronds: 12,
        skirt_fronds: 4,
        skirt_length: 0.5,
        leaflet_count: 7,
        rachis_length: 0.3,
        rachis_arch: -0.5,
        ..Default::default()
    };
    assert!(supports(crown, twig));
    assert!(bearing_runs(&tree, crown).is_empty());
    let fronds = at(crown).unwrap();
    // One oriented box a leaflet, seven to each of the sixteen fronds.
    assert_eq!(fronds.descriptors.len(), 16 * 7);
    assert_eq!(fronds.total, 16 * 7);
    assert!(fronds.descriptors.iter().all(|d| d.count == 1));
    assert!(fronds.descriptors.iter().all(|d| d.side != Vec3::ZERO));
    assert_eq!((fronds.blade, fronds.rachis, fronds.seat), (0.0, 0.0, 1.0));
    // A dead frond is drawn at its share of a living one's size, so its
    // leaflets' boxes are narrower on the whole.
    let width = |range: std::ops::Range<usize>| {
        let n = range.len() as f64;
        range
            .map(|i| fronds.descriptors[i].side.length())
            .sum::<f64>()
            / n
    };
    assert!(width(12 * 7..16 * 7) < width(0..12 * 7));
    let single = CanopyParams {
        leaflet_count: 1,
        ..crown
    };
    let blades = at(single).unwrap();
    assert_eq!(blades.descriptors.len(), 16);
    assert_eq!(blades.total, 16);
    assert!(blades.descriptors.iter().all(|d| d.side != Vec3::ZERO));
    let budget = CanopyParams {
        max_instances: 16 * 7 - 1,
        ..crown
    };
    let refused = plan(
        &tree,
        Envelope::default(),
        budget,
        twig,
        &SurfaceParams::default(),
        &element,
        None,
        1,
    );
    assert!(refused.is_err());
    let single = at(CanopyParams::default()).unwrap();
    let pinnate = CanopyParams {
        leaflet_count: 7,
        rachis_length: 0.3,
        rachis_arch: -0.5,
        ..Default::default()
    };
    let compound = at(pinnate).unwrap();
    assert_eq!(compound.total, single.total * 7);
    assert_eq!(compound.descriptors.len(), single.descriptors.len());
    for (c, s) in compound.descriptors.iter().zip(&single.descriptors) {
        assert_eq!(c.count, s.count * 7);
        assert_eq!(c.endpoints, s.endpoints);
    }
    assert_eq!(single.rachis, 0.0);
    assert_eq!(compound.rachis, 0.3 * 1.5);
    let d = &compound.descriptors[0];
    assert!((compound.reach(d) - single.reach(d) - 0.3 * 1.5).abs() < 1e-12);
    // A count without a rachis, or a rachis without a count, is one blade.
    let bare = CanopyParams {
        leaflet_count: 7,
        ..Default::default()
    };
    assert_eq!(at(bare).unwrap(), single);
    let budget = CanopyParams {
        max_instances: single.total as usize,
        ..pinnate
    };
    assert!(runs(&tree, Envelope::default(), budget, twig).is_err());
}
