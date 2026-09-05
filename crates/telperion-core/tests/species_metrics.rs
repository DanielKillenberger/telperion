#[path = "../examples/species_metrics/mod.rs"]
mod species_metrics;
use serde_json::json;
use species_metrics::{compare, measure};
use telperion_core::{
    foliage::{Element, Instances},
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
    };
    let instances = Instances {
        matrices: vec![[
            2., 0., 0., 0., 0., 3., 0., 0., 0., 0., 1., 0., 1., 5., 0., 1.,
        ]],
    };
    (tree, element, instances)
}
#[test]
fn analytic_units_dbh_axes_and_retained_area() {
    let (t, e, i) = fixture();
    let m = measure(&t, &[0., 0., 0., 0., 4.25, 0.], &e, 2, &i).unwrap();
    assert!((m["dbh_m"]["value"].as_f64().unwrap() - 0.74).abs() < 1e-12);
    assert_eq!(m["height_m"]["value"], 11.);
    assert_eq!(m["wood_height_m"]["value"], 4.25);
    assert_eq!(m["branch_count"]["value"], 1);
    assert_eq!(m["branch_lengths_m"]["value"], json!([4.]));
    assert_eq!(m["branch_order"]["value"], json!({"1":1}));
    assert_eq!(m["twig_count"]["value"], 1);
    assert_eq!(m["foliage_units"]["value"], 1);
    assert_eq!(m["discarded_units"]["value"], 1);
    assert_eq!(m["leaf_area_m2"]["value"], 6.);
    assert_eq!(m["foliage_length_m"]["max"], 6.);
    assert_eq!(m["foliage_width_m"]["max"], 2.);
    assert_eq!(
        m,
        measure(&t, &[0., 0., 0., 0., 4.25, 0.], &e, 2, &i).unwrap()
    );
}
#[test]
fn missing_ambiguous_truncated_and_nonfinite_are_distinct() {
    let (mut t, e, i) = fixture();
    let m = measure(&t, &[], &e, 0, &Instances::default()).unwrap();
    assert_eq!(m["height_m"]["status"], "unavailable");
    assert_eq!(m["crown_width_m"]["status"], "unavailable");
    t.nodes[3].parent = Some(0);
    t.nodes[3].kind = NodeKind::Structural;
    assert_eq!(
        measure(&t, &[0., 4., 0.], &e, 1, &i).unwrap()["dbh_m"]["status"],
        "ambiguous"
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
