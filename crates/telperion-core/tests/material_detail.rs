//! Added appearance rows leave older documents intact and walk like geometry rows.
use serde_json::json;
use telperion_core::{blend, params, presets::Preset, Error};

const FIELDS: [(&str, &str, f64, f64); 45] = [
    ("furrowStrength", "bark furrow strength", 0.0, 1.0),
    ("ridgeScale", "bark ridge scale", 0.0, 1.0),
    ("plateScale", "bark plate scale", 0.0, 1.0),
    ("roughnessDetail", "bark roughness detail", 0.0, 1.0),
    ("veinScale", "leaf vein scale", 0.0, 32.0),
    ("veinContrast", "leaf vein contrast", 0.0, 1.0),
    (
        "transmissionStrength",
        "leaf transmission strength",
        0.0,
        1.0,
    ),
    ("transmissionRed", "leaf transmission red", 0.0, 1.0),
    ("transmissionGreen", "leaf transmission green", 0.0, 1.0),
    ("transmissionBlue", "leaf transmission blue", 0.0, 1.0),
    ("thickness", "leaf thickness", 0.0, 8.0),
    ("fissureRed", "bark fissure red", -1.0, 1.0),
    ("fissureGreen", "bark fissure green", -1.0, 1.0),
    ("fissureBlue", "bark fissure blue", -1.0, 1.0),
    ("fissureStrength", "bark fissure strength", 0.0, 1.0),
    ("crestRed", "bark crest red", -1.0, 1.0),
    ("crestGreen", "bark crest green", -1.0, 1.0),
    ("crestBlue", "bark crest blue", -1.0, 1.0),
    ("crestStrength", "bark crest strength", 0.0, 1.0),
    ("barkMottleScale", "bark mottle scale", 0.0, 8.0),
    ("barkMottleStrength", "bark mottle strength", 0.0, 1.0),
    ("cavityStrength", "bark cavity strength", 0.0, 1.0),
    ("bladeMottleScale", "leaf blade mottle scale", 0.0, 32.0),
    (
        "bladeMottleStrength",
        "leaf blade mottle strength",
        0.0,
        1.0,
    ),
    ("marginWidth", "leaf margin width", 0.0, 0.5),
    ("marginRed", "leaf margin red", -1.0, 1.0),
    ("marginGreen", "leaf margin green", -1.0, 1.0),
    ("marginBlue", "leaf margin blue", -1.0, 1.0),
    ("cuticleGloss", "leaf cuticle gloss", 0.0, 1.0),
    ("skyOcclusionStrength", "sky occlusion strength", 0.0, 1.0),
    ("plateCellScale", "bark plate cell scale", 0.0, 1.0),
    ("plateElongation", "bark plate elongation", 0.0, 16.0),
    ("plateDome", "bark plate dome", 0.0, 1.0),
    ("plateEdgeLift", "bark plate edge lift", 0.0, 1.0),
    ("plateIdentity", "bark plate identity", 0.0, 1.0),
    ("weatheringStrength", "bark weathering strength", 0.0, 1.0),
    ("weatheringRed", "bark weathering red", -1.0, 1.0),
    ("weatheringGreen", "bark weathering green", -1.0, 1.0),
    ("weatheringBlue", "bark weathering blue", -1.0, 1.0),
    ("orientationStrength", "bark orientation strength", 0.0, 1.0),
    ("orientationRed", "bark orientation red", -1.0, 1.0),
    ("orientationGreen", "bark orientation green", -1.0, 1.0),
    ("orientationBlue", "bark orientation blue", -1.0, 1.0),
    (
        "directionalOcclusion",
        "bark directional occlusion",
        0.0,
        1.0,
    ),
    ("depthStrength", "bark depth strength", 0.0, 1.0),
];

#[test]
fn detail_rows_refuse_both_bounds_by_name_and_roundtrip_the_endpoints() {
    for (key, name, low, high) in FIELDS {
        for value in [low - 0.01, high + 0.01] {
            assert_eq!(
                params::parse(&json!({"material": {key: value}})).err(),
                Some(Error::InvalidInput(name)),
                "{key} at {value}"
            );
        }
        for value in [low, high] {
            let family = params::parse(&json!({"material": {key: value}})).unwrap();
            assert_eq!(params::metadata(&family)["material"][key], json!(value));
        }
    }
}

#[test]
fn detail_rows_all_walk_at_an_interior_point() {
    for (key, _, low, high) in FIELDS {
        let from = params::parse(&json!({"material": {key: low}})).unwrap();
        let to = params::parse(&json!({"material": {key: high}})).unwrap();
        let walked = blend::families(&from, &to, 0.25).unwrap();
        assert_eq!(
            params::metadata(&walked)["material"][key],
            json!(low * 0.75 + high * 0.25)
        );
    }
}

#[test]
fn older_material_documents_gain_only_inert_detail_defaults() {
    let mut old = params::metadata(&Preset::Ordinary.parameters());
    for (key, _, _, _) in FIELDS {
        old["material"].as_object_mut().unwrap().remove(key);
    }
    let emitted = params::metadata(&params::parse(&old).unwrap());
    for key in [
        "fissureRed",
        "fissureGreen",
        "fissureBlue",
        "fissureStrength",
        "crestRed",
        "crestGreen",
        "crestBlue",
        "crestStrength",
        "barkMottleScale",
        "barkMottleStrength",
        "cavityStrength",
        "bladeMottleScale",
        "bladeMottleStrength",
        "marginWidth",
        "marginRed",
        "marginGreen",
        "marginBlue",
        "cuticleGloss",
        "skyOcclusionStrength",
        "plateCellScale",
        "plateElongation",
        "plateDome",
        "plateEdgeLift",
        "plateIdentity",
        "weatheringStrength",
        "weatheringRed",
        "weatheringGreen",
        "weatheringBlue",
        "orientationStrength",
        "orientationRed",
        "orientationGreen",
        "orientationBlue",
        "directionalOcclusion",
        "depthStrength",
        "ridgeScale",
        "plateScale",
        "roughnessDetail",
        "veinContrast",
        "transmissionStrength",
    ] {
        assert_eq!(emitted["material"][key], json!(0.0), "{key}");
    }
    let mut stripped = emitted;
    for (key, _, _, _) in FIELDS {
        assert!(stripped["material"]
            .as_object_mut()
            .unwrap()
            .remove(key)
            .is_some());
    }
    assert_eq!(stripped, old);
}
