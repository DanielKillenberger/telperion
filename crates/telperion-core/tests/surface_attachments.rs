use telperion_core::{
    envelope::Envelope,
    foliage::{place, place_on_surface, CanopyParams, TwigPlacement},
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
    for (segments, lobes) in [(3, 0), (7, 0), (20, 0), (20, 5)] {
        let params = SurfaceParams {
            radial_segments: segments,
            lobes,
            lobe_depth: if lobes == 0 { 0.0 } else { 0.16 },
            flare_depth: 0.0,
            ..SurfaceParams::default()
        };
        let mesh = build(&tree, env.height, &params).unwrap();
        let placed = place_on_surface(&tree, env, 1, canopy, twig, &params).unwrap();
        let circular = place(&tree, env, 1, canopy, twig).unwrap();
        assert_eq!(placed.matrices.len(), circular.matrices.len());
        assert_ne!(
            placed, circular,
            "polygonal/socket contacts must replace circular origins"
        );
        for m in &placed.matrices {
            let p = Vec3::new(m[12] as f64, m[13] as f64, m[14] as f64);
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
                    (p - a).dot(n).abs() < 2e-7
                        && [(a, b), (b, c), (c, a)]
                            .iter()
                            .all(|(u, v)| (*v - *u).cross(p - *u).dot(n) >= -2e-8)
                }),
                "unattached origin {p:?}, polygon sides {segments}"
            );
        }
    }
}
