#[path = "../examples/geometry_benchmark/metrics.rs"]
mod metrics;
use serde_json::{json, Value};
use telperion_core::{
    foliage::{self, ElementParams, Instances},
    math::Vec3,
    params,
    tree::{Node, NodeKind, Tree},
};
fn tree() -> Tree {
    let node = |x, y, p, r, s| Node {
        position: Vec3::new(x, y, 0.),
        parent: p,
        radius: r,
        start_radius: s,
        base_radius: r,
        branch: 0,
        kind: NodeKind::Structural,
    };
    Tree {
        nodes: vec![
            node(0., 0., None, 1., 1.),
            node(0., 2., Some(0), 0.8, 1.),
            node(0., 4., Some(1), 0.6, 0.8),
            node(3., 2., Some(1), 0.1, 0.4),
        ],
        ..Tree::default()
    }
}
fn near(v: &Value, n: f64) {
    assert!((v.as_f64().unwrap() - n).abs() < 1e-10, "{v} != {n}");
}
#[test]
fn analytic_axes_taper_angles_and_degeneracy() {
    let mut t = tree();
    assert_eq!(
        metrics::distribution((1..=10).map(f64::from).collect()),
        json!({"count":10,"min":1.,"max":10.,"median":5.5,"p10":1.,"p90":9.})
    );
    let m = metrics::axes(&t).unwrap();
    let s = &m["value"]["samples"];
    assert_eq!(m["status"], "estimated");
    near(&s[0]["length_m"], 4.);
    near(&s[0]["mean_diameter_m"], 1.6);
    near(&s[0]["taper"]["proximal_m"], 1.92);
    near(&s[0]["taper"]["distal_m"], 1.28);
    near(&s[0]["taper"]["ratio"], 2. / 3.);
    near(&s[0]["taper"]["slope_m_per_m"], 0.2);
    assert_eq!(s[1]["order"], 1);
    near(&s[1]["length_m"], 3.);
    near(&s[1]["angle_deg"], 90.);
    assert!(s[0]["angle_deg"].is_null());
    t.nodes[3].start_radius = 0.8;
    let tie = metrics::axes(&t).unwrap();
    assert_eq!(tie["value"]["samples"][0]["edge_nodes"], json!([1, 2]));
    assert_eq!(tie["support"]["excluded"], 2);
    assert_eq!(tie["value"]["by_order"][0]["length_m"]["count"], 0);
    assert!(tie["value"]["by_order"][0]["length_m"]["median"].is_null());
    t.nodes.truncate(2);
    t.nodes[1].position = Vec3::ZERO;
    let zero = metrics::axes(&t).unwrap();
    assert_eq!(zero["support"]["excluded"], 1);
    assert!(zero["value"]["samples"][0]["taper"].is_null());
    t.nodes[1].position.x = f64::NAN;
    assert!(metrics::axes(&t).is_err());
}
#[test]
fn biological_centroid_bins_count_needles_and_exclude_connectors() {
    let t = tree();
    let matrix = |x, y| [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., x, y, 0., 1.];
    let kept = Instances {
        matrices: vec![matrix(1., 0.), matrix(4., 4.)],
    };
    let p = ElementParams {
        section_roundness: 1.0,
        cross_segments: 4,
        length: 0.02,
        width: 0.002,
        connector_length: 0.001,
        ..ElementParams::default()
    };
    let e = foliage::build_element(p).unwrap();
    let m = metrics::foliage_bins(&t, &e, &kept).unwrap();
    assert_eq!(m["value"]["total"], 2);
    assert_eq!(m["value"]["unit"], "needle");
    assert_eq!(m["value"]["counts"][0][1], 1);
    assert_eq!(m["value"]["counts"][3][3], 1);
    let other = foliage::build_element(ElementParams {
        connector_length: 0.009,
        ..p
    })
    .unwrap();
    let other = metrics::foliage_bins(&t, &other, &kept).unwrap();
    assert_eq!(m["value"]["counts"], other["value"]["counts"]);
    assert!(
        (other["value"]["normalization"]["ymin_m"].as_f64().unwrap()
            - m["value"]["normalization"]["ymin_m"].as_f64().unwrap()
            - 0.008)
            .abs()
            < 1e-8
    );
    assert_eq!(
        metrics::foliage_bins(&t, &e, &Instances::default()).unwrap()["status"],
        "unavailable"
    );
    let mut bad = kept;
    bad.matrices[0][0] = f32::NAN;
    assert!(metrics::foliage_bins(&t, &e, &bad).is_err());
}
#[test]
fn frozen_parameters_resolve_without_default_substitution() {
    let p: Value =
        serde_json::from_str(include_str!("../../../.flow/evidence/fn19/protocol.json")).unwrap();
    for s in p["species"].as_array().unwrap() {
        let mut given = s["parameters"].clone();
        // fn-24 retired the tagged habit, the tagged element anatomy and the
        // tagged canopy attachment for numeric trait tables, so the frozen
        // file speaks the old shape for those three objects and no other.
        given["skeleton"]
            .as_object_mut()
            .unwrap()
            .remove("habit")
            .expect("frozen parameters carry a habit");
        given["element"]
            .as_object_mut()
            .unwrap()
            .remove("anatomy")
            .expect("frozen parameters carry an anatomy");
        given["canopy"]
            .as_object_mut()
            .unwrap()
            .remove("attachment")
            .expect("frozen parameters carry an attachment");
        let f = params::parse(&given).unwrap();
        let mut emitted = params::metadata(&f);
        for trait_name in ["lobeCount", "lobeDepth", "sectionRoundness"] {
            emitted["element"]
                .as_object_mut()
                .unwrap()
                .remove(trait_name)
                .expect("the element publishes its outline traits");
        }
        for trait_name in ["forwardLean", "leanRise", "surfaceContact"] {
            emitted["canopy"]
                .as_object_mut()
                .unwrap()
                .remove(trait_name)
                .expect("the canopy publishes its lean and contact traits");
        }
        for key in ["element", "canopy", "radii", "surface"] {
            same_numbers(&emitted[key], &given[key]);
        }
        // Every parameter the wire publishes is stated, never defaulted in.
        stated(&emitted, &given, "");
    }
    assert!(params::parse(&json!({"unknown":3})).is_err());
    let mut bad = p["species"][0]["parameters"].clone();
    bad["skeleton"]["seed"] = json!(1.5);
    assert!(params::parse(&bad).is_err());
    assert!(params::by_identity("not-implemented").is_err());
}

/// Every leaf of the published schema is present in the frozen parameters.
fn stated(schema: &Value, given: &Value, path: &str) {
    for (key, value) in schema.as_object().unwrap() {
        if value.is_null() {
            // An override the family leaves to the envelope's own derivation.
            continue;
        }
        if path.is_empty() && key == "skeleton" {
            // The habit is the one object the frozen file no longer speaks.
            let mut narrowed = value.clone();
            narrowed.as_object_mut().unwrap().remove("habit");
            stated(&narrowed, &given[key], "skeleton");
            continue;
        }
        let stated_here = given
            .get(key)
            .unwrap_or_else(|| panic!("frozen parameters omit {path}/{key}"));
        if value.is_object() {
            stated(value, stated_here, &format!("{path}/{key}"));
        }
    }
}

fn same_numbers(a: &Value, b: &Value) {
    match (a, b) {
        (Value::Number(_), Value::Number(_)) => assert_eq!(a.as_f64(), b.as_f64()),
        (Value::Object(a), Value::Object(b)) => {
            assert_eq!(a.len(), b.len());
            for (k, v) in a {
                same_numbers(v, &b[k]);
            }
        }
        _ => assert_eq!(a, b),
    }
}

#[test]
fn receipt_manifest_and_replay_controls() {
    let status = std::process::Command::new("python3")
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/geometry_benchmark/test_controls.py"
        ))
        .status()
        .expect("Python 3 receipt controls");
    assert!(status.success());
}
