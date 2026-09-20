#[path = "../examples/species_metrics/mod.rs"]
mod species_metrics;
use serde_json::json;
use species_metrics::{compare, measure};
use telperion_core::{
    foliage::{Element, Instances, Reference},
    math::Vec3,
    tree::{Node, NodeKind, Tree},
};
fn fixture() -> (Tree, Element, Instances) {
    let node = |p, parent, radius, start_radius, branch, kind| Node {
        position: p,
        parent,
        radius,
        start_radius,
        base_radius: radius,
        branch,
        kind,
        ..Node::root()
    };
    let tree = Tree {
        nodes: vec![
            node(Vec3::ZERO, None, 0.5, 0.5, 0, NodeKind::Structural),
            node(
                Vec3::new(0., 2., 0.),
                Some(0),
                0.3,
                0.5,
                1,
                NodeKind::Structural,
            ),
            node(
                Vec3::new(0., 4., 0.),
                Some(1),
                0.2,
                0.3,
                2,
                NodeKind::Structural,
            ),
            node(Vec3::new(3., 2., 0.), Some(1), 0.1, 0.1, 3, NodeKind::Twig),
            node(Vec3::new(4., 2., 0.), Some(3), 0.05, 0.1, 3, NodeKind::Twig),
        ],
        crossover: 3,
        ..Tree::default()
    };
    let element = Element {
        positions: vec![Vec3::ZERO, Vec3::new(1., 0., 0.), Vec3::new(0., 2., 0.)],
        indices: vec![0, 1, 2],
        anatomy: None,
        ..Element::default()
    };
    // One rotation, one uniform scale and a point: what a stored leaf carries.
    // The crown is the single station at (1, 5, 0), so the box is that point
    // and the position survives the round trip exactly.
    let mut instances = Instances::new(Reference::spanning(
        Vec3::new(1., 5., 0.),
        Vec3::new(1., 5., 0.),
    ));
    instances.push(&[
        2., 0., 0., 0., 0., 2., 0., 0., 0., 0., 2., 0., 1., 5., 0., 1.,
    ]);
    (tree, element, instances)
}
#[test]
fn analytic_units_dbh_axes_and_retained_area() {
    let (t, e, i) = fixture();
    let m = measure(&t, &[0., 0., 0., 0., 4.25, 0.], &e, 2, &i).unwrap();
    assert!((m["dbh_m"]["value"].as_f64().unwrap() - 0.74).abs() < 1e-12);
    // The rotation is read back through ten bits a component, so the leaf's
    // own tip stands a micron from where it was asked for.
    assert!((m["height_m"]["value"].as_f64().unwrap() - 9.).abs() < 1e-5);
    assert_eq!(m["wood_height_m"]["value"], 4.25);
    assert_eq!(m["branch_count"]["value"], 1);
    assert_eq!(m["branch_lengths_m"]["value"], json!([4.]));
    assert_eq!(m["branch_order"]["value"], json!({"1":1}));
    assert_eq!(m["twig_count"]["value"], 1);
    assert_eq!(m["foliage_units"]["value"], 1);
    assert_eq!(m["discarded_units"]["value"], 1);
    // A rotation and one scale keep an area and a length; what moves is the
    // last bits of the columns the decoder writes back as f32.
    assert!((m["leaf_area_m2"]["value"].as_f64().unwrap() - 4.).abs() < 1e-7);
    assert!((m["foliage_length_m"]["max"].as_f64().unwrap() - 4.).abs() < 1e-7);
    assert!((m["foliage_width_m"]["max"].as_f64().unwrap() - 2.).abs() < 1e-7);
    assert_eq!(
        m,
        measure(&t, &[0., 0., 0., 0., 4.25, 0.], &e, 2, &i).unwrap()
    );
}
#[test]
fn missing_multi_stemmed_truncated_and_nonfinite_are_distinct() {
    let (mut t, e, i) = fixture();
    let m = measure(&t, &[], &e, 0, &Instances::default()).unwrap();
    assert_eq!(m["height_m"]["status"], "unavailable");
    assert_eq!(m["crown_width_m"]["status"], "unavailable");
    // A second stem leaving the root crosses breast height on its own edge:
    // the proxy is the largest of them, and it says how many there were.
    t.nodes[3].parent = Some(0);
    t.nodes[3].kind = NodeKind::Structural;
    let clump = measure(&t, &[0., 4., 0.], &e, 1, &i).unwrap();
    assert_eq!(clump["dbh_m"]["status"], "measured_proxy");
    assert_eq!(clump["dbh_m"]["stems"], 2);
    let diameters = clump["dbh_m"]["diameters_m"].as_array().unwrap().clone();
    assert_eq!(
        clump["dbh_m"]["value"].as_f64().unwrap(),
        diameters
            .iter()
            .map(|v| v.as_f64().unwrap())
            .fold(0., f64::max)
    );
    t.diagnostics.node_capped = true;
    assert_eq!(
        measure(&t, &[0., 4., 0.], &e, 1, &i).unwrap()["growth"]["status"],
        "truncated"
    );
    t.nodes[1].position.x = f64::NAN;
    assert!(measure(&t, &[0., 4., 0.], &e, 1, &i)
        .unwrap_err()
        .starts_with("non_finite"));
}
#[test]
fn gates_do_not_pass_estimates_missing_values_or_outliers() {
    let profile = json!({"metrics":{"x":{"classification":"gating","range":[1.,2.],"unit":"m"}}});
    for status in ["estimated", "unavailable", "ambiguous"] {
        let (pass, _) = compare(&profile, &json!({"x":{"status":status,"value":1.5}})).unwrap();
        assert!(!pass);
    }
    assert!(
        !compare(
            &profile,
            &json!({"x":{"status":"measured","min":1.5,"max":3.}})
        )
        .unwrap()
        .0
    );
    assert!(
        compare(&profile, &json!({"x":{"status":"measured","value":1.5}}))
            .unwrap()
            .0
    );
    assert!(compare(&json!({"metrics":{}}), &json!({})).is_err());
}

#[test]
fn overflow_and_degenerate_geometry_fail_explicitly() {
    let (mut t, mut e, i) = fixture();
    t.nodes[4].position.x = 1e300;
    assert!(measure(&t, &[0., 4., 0.], &e, 1, &i)
        .unwrap_err()
        .starts_with("non_finite"));
    t.nodes[4].position.x = 4.;
    e.positions[2] = e.positions[1];
    assert!(measure(&t, &[0., 4., 0.], &e, 1, &i)
        .unwrap_err()
        .starts_with("degenerate"));
}

#[test]
fn measured_species_subsets_exclude_connectors_and_use_transformed_geometry() {
    use telperion_core::foliage::{build_element, ElementParams};
    let (t, _, fixed) = fixture();
    // Rotate the leaf's own axes into world Z/X/Y.
    let mut instances = Instances::new(fixed.reference);
    instances.push(&[
        0., 0., 3., 0., 3., 0., 0., 0., 0., 3., 0., 0., 1., 5., 0., 1.,
    ]);
    // A flat blade and a shaft rolled shut, each with a station on its widest
    // point so the authored width is the measured one.
    for section_roundness in [0.0, 1.0] {
        let p = ElementParams {
            section_roundness,
            cross_segments: if section_roundness > 0. { 4 } else { 2 },
            widest_at: if section_roundness > 0. { 0.2 } else { 0.4 },
            base_fullness: if section_roundness > 0. { 0.2 } else { 0.85 },
            tip_sharpness: if section_roundness > 0. { 0.2 } else { 1.6 },
            length: 0.02,
            width: 0.002,
            connector_length: 0.001,
            cup: 0.,
            curl: 0.01,
            ..ElementParams::default()
        };
        let e = build_element(p).unwrap();
        let m = measure(&t, &[0., 4., 0.], &e, 3, &instances).unwrap();
        assert_eq!(m["foliage_length_m"]["status"], "measured");
        assert!((m["foliage_length_m"]["max"].as_f64().unwrap() - 0.06).abs() < 1e-8);
        // The blade is measured across its face, the shaft across its section.
        let expected_width = 0.006;
        assert!((m["foliage_width_m"]["max"].as_f64().unwrap() - expected_width).abs() < 1e-8);
        let longer_connector = build_element(ElementParams {
            connector_length: 0.009,
            ..p
        })
        .unwrap();
        let other = measure(&t, &[0., 4., 0.], &longer_connector, 3, &instances).unwrap();
        assert!(
            (m["leaf_area_m2"]["value"].as_f64().unwrap()
                - other["leaf_area_m2"]["value"].as_f64().unwrap())
            .abs()
                < 1e-10
        );
        assert_eq!(m["units_per_instance"]["value"], 1);
        assert_eq!(m["discarded_units"]["value"], 2);
        if section_roundness > 0. {
            let surface = m["needle_surface_area_m2"]["value"].as_f64().unwrap();
            let projected = m["projected_area_m2"]["value"].as_f64().unwrap();
            assert!(surface > 2. * projected);
            assert!(
                // Five equal intervals; the outline profile at 0, .2, .4, .6,
                // .8, 1 is 0, 1, .98429, .933033, .825217, 0, a trapezoidal
                // mean of .748508 across (.002 * 3) by (.02 * 3).
                (projected - 0.000269462_866).abs() < 1e-9,
                "canonical projected polygon area, got {projected}"
            );
            assert_eq!(m["foliage_unit"], "needle");
        }
    }
}
/// A trunk that parts into two runs at `fork` metres, the second of them a
/// clump's later stem if `second` and a limb if not.
fn forked(fork: f64, second: bool) -> Tree {
    let node = |x: f64, y: f64, parent: Option<u32>, radius: f64| Node {
        position: Vec3::new(x, y, 0.),
        parent,
        radius,
        start_radius: radius,
        base_radius: radius,
        branch: parent.map_or(0, |p| p + 1),
        stem: true,
        ..Node::root()
    };
    let mut nodes = vec![
        node(0., 0., None, 0.3),
        node(0., fork, Some(0), 0.3),
        node(0., 4., Some(1), 0.2),
        node(1., 4., Some(1), 0.15),
    ];
    nodes[3].stem = second;
    Tree {
        crossover: nodes.len(),
        nodes,
        ..Tree::default()
    }
}
#[test]
fn a_fork_below_breast_height_is_two_stems_there_and_above_it_one() {
    let (_, e, _) = fixture();
    let stems =
        |t: &Tree| measure(t, &[], &e, 0, &Instances::default()).unwrap()["dbh_m"]["stems"].clone();
    // Parted under the plane, each stem crosses it on its own wood.
    assert_eq!(stems(&forked(1.0, true)), 2);
    // Parted over it, the plane cuts the one trunk below the fork.
    assert_eq!(stems(&forked(2.0, true)), 1);
    // And a limb leaving the trunk under the plane is that trunk's.
    assert_eq!(stems(&forked(1.0, false)), 1);
}
