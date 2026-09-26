use telperion_core::{
    envelope::Envelope,
    foliage::{place, place_on_surface, CanopyParams, Reference, TwigPlacement},
    math::Vec3,
    surface::{build, SurfaceParams},
    tree::{Node, NodeKind, Tree},
};

#[test]
fn needle_origins_touch_rendered_facets_across_bends_and_sockets() {
    let mut tree = Tree::default();
    for (i, (p, parent, radius, start)) in [
        (Vec3::new(0., 1., 0.), None, 0.02, 0.02),
        (Vec3::new(0., 1.1, 0.), Some(0), 0.015, 0.02),
        (Vec3::new(0.01, 1.2, 0.), Some(1), 0.01, 0.015),
        (Vec3::new(0.08, 1.12, 0.03), Some(1), 0.001, 0.003),
        (Vec3::new(0.15, 1.09, 0.04), Some(3), 0.00025, 0.001),
    ]
    .into_iter()
    .enumerate()
    {
        tree.nodes.push(Node {
            position: p,
            parent,
            radius,
            start_radius: start,
            base_radius: start,
            branch: if i >= 3 { 3 } else { 0 },
            kind: if i >= 3 {
                NodeKind::Twig
            } else {
                NodeKind::Structural
            },
            ..Node::root()
        });
    }
    tree.crossover = 3;
    let env = Envelope::default();
    let canopy = CanopyParams {
        surface_contact: 1.0,
        shoot_radius: 0.0,
        ..CanopyParams::default()
    };
    let twig = Some(TwigPlacement {
        internode_length: 0.0025,
        stations_per_internode: 1,
    });
    // The twig run stands between these corners; a station's own code is
    // half this box's step from where it was computed, and that half step is
    // the slack every facet test below carries.
    let box_of = Reference::spanning(Vec3::new(-0.05, 0.95, -0.05), Vec3::new(0.20, 1.25, 0.10));
    let step = box_of.step();
    let slack = step.x.max(step.y).max(step.z) / 2.;
    for (segments, lobes) in [(3, 0), (7, 0), (20, 0), (20, 5)] {
        let params = SurfaceParams {
            radial_segments: segments,
            lobes,
            lobe_depth: if lobes == 0 { 0.0 } else { 0.16 },
            flare_depth: 0.0,
            ..SurfaceParams::default()
        };
        let mesh = build(&tree, env.height, &params).unwrap();
        let placed = place_on_surface(&tree, env, 1, canopy, twig, &params, box_of).unwrap();
        let circular = place(&tree, env, 1, canopy, twig, box_of).unwrap();
        assert_eq!(placed.len(), circular.len());
        assert_ne!(
            placed, circular,
            "polygonal/socket contacts must replace circular origins"
        );
        for i in 0..placed.len() {
            let p = placed.position(i);
            let point = |i: u32| {
                let k = i as usize * 3;
                Vec3::new(
                    mesh.positions[k] as f64,
                    mesh.positions[k + 1] as f64,
                    mesh.positions[k + 2] as f64,
                )
            };
            assert!(
                mesh.indices.as_chunks::<3>().0.iter().any(|t| {
                    let a = point(t[0]);
                    let b = point(t[1]);
                    let c = point(t[2]);
                    let n = (b - a).cross(c - a).normalized();
                    (p - a).dot(n).abs() < 2e-7 + slack
                        && [(a, b), (b, c), (c, a)]
                            .iter()
                            .all(|(u, v)| (*v - *u).cross(p - *u).dot(n) >= -2e-8 - slack)
                }),
                "unattached origin {p:?}, polygon sides {segments}"
            );
        }
    }
}
