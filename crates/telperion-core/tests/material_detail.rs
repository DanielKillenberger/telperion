//! Added appearance rows leave older documents intact and walk like geometry rows.
use serde_json::json;
use telperion_core::{blend, params, presets::Preset, Error};

const FIELDS: [(&str, &str, f64); 11] = [
    ("furrowStrength", "bark furrow strength", 1.0),
    ("ridgeScale", "bark ridge scale", 1.0),
    ("plateScale", "bark plate scale", 1.0),
    ("roughnessDetail", "bark roughness detail", 1.0),
    ("veinScale", "leaf vein scale", 32.0),
    ("veinContrast", "leaf vein contrast", 1.0),
    ("transmissionStrength", "leaf transmission strength", 1.0),
    ("transmissionRed", "leaf transmission red", 1.0),
    ("transmissionGreen", "leaf transmission green", 1.0),
    ("transmissionBlue", "leaf transmission blue", 1.0),
    ("thickness", "leaf thickness", 8.0),
];

#[test]
fn detail_rows_refuse_both_bounds_by_name_and_roundtrip_the_endpoints() {
    for (key, name, high) in FIELDS {
        for value in [-0.01, high + 0.01] {
            assert_eq!(
                params::parse(&json!({"material": {key: value}})).err(),
                Some(Error::InvalidInput(name)),
                "{key} at {value}"
            );
        }
        for value in [0.0, high] {
            let family = params::parse(&json!({"material": {key: value}})).unwrap();
            assert_eq!(params::metadata(&family)["material"][key], json!(value));
        }
    }
}

#[test]
fn detail_rows_all_walk_at_an_interior_point() {
    for (key, _, high) in FIELDS {
        let from = params::parse(&json!({"material": {key: 0.0}})).unwrap();
        let to = params::parse(&json!({"material": {key: high}})).unwrap();
        let walked = blend::families(&from, &to, 0.25).unwrap();
        assert_eq!(
            params::metadata(&walked)["material"][key],
            json!(high * 0.25)
        );
    }
}

#[test]
fn older_material_documents_gain_only_inert_detail_defaults() {
    let mut old = params::metadata(&Preset::Ordinary.parameters());
    for (key, _, _) in FIELDS {
        old["material"].as_object_mut().unwrap().remove(key);
    }
    let emitted = params::metadata(&params::parse(&old).unwrap());
    for key in [
        "ridgeScale",
        "plateScale",
        "roughnessDetail",
        "veinContrast",
        "transmissionStrength",
    ] {
        assert_eq!(emitted["material"][key], json!(0.0), "{key}");
    }
    let mut stripped = emitted;
    for (key, _, _) in FIELDS {
        assert!(stripped["material"]
            .as_object_mut()
            .unwrap()
            .remove(key)
            .is_some());
    }
    assert_eq!(stripped, old);
}
