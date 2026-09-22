//! Explicit replay of the eight recorded fn-32 poses, four images per run.
use serde_json::{json, Value};
use std::path::PathBuf;
use telperion_core::{
    math::Vec3,
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{
    render, write_png, Camera, Gpu, Level, Renderer, SceneRow, View, STILL_FORMAT,
};

fn vector(value: &Value) -> Vec3 {
    Vec3::new(
        value["x"].as_f64().unwrap(),
        value["y"].as_f64().unwrap(),
        value["z"].as_f64().unwrap(),
    )
}

#[test]
#[ignore = "explicit GPU evidence; BARK_EVIDENCE_SPECIES=oak|spruce and BARK_CAPTURE_DIR required"]
fn replay_recorded_bark_views() {
    let species = std::env::var("BARK_EVIDENCE_SPECIES").unwrap();
    assert!(matches!(species.as_str(), "oak" | "spruce"));
    let directory = PathBuf::from(std::env::var_os("BARK_CAPTURE_DIR").unwrap());
    std::fs::create_dir_all(&directory).unwrap();
    let source =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.flow/evidence/fn32/stills.json");
    let manifest: Value = serde_json::from_str(&std::fs::read_to_string(source).unwrap()).unwrap();
    let poses: Vec<_> = manifest["stills"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| {
            v["name"]
                .as_str()
                .unwrap()
                .starts_with(&format!("{species}-"))
        })
        .collect();
    assert_eq!(poses.len(), 4);
    let mut family = Preset::from_id(poses[0]["preset"].as_str().unwrap())
        .unwrap()
        .parameters();
    family.skeleton.seed = 7;
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let gpu = pollster::block_on(Gpu::request(None)).expect("hardware GPU required for evidence");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit_at(&tree, Level::Chosen).unwrap();
    renderer.set_material(family.material);
    let mut records = Vec::new();
    for pose in poses {
        let c = &pose["camera"];
        let camera = Camera {
            position: vector(&c["position"]),
            target: vector(&c["target"]),
            field_of_view: c["field_of_view"].as_f64().unwrap(),
            near: c["near"].as_f64().unwrap(),
            far: c["far"].as_f64().unwrap(),
        };
        let view = View::from_id(&pose["view"].as_str().unwrap().to_lowercase()).unwrap();
        renderer.set_view(view);
        renderer.set_scene(SceneRow::parse(&pose["scene"].to_string()).unwrap());
        let frame = render(&mut renderer, &camera, 1600, 1000).unwrap();
        assert!(frame.has_subject());
        let name = pose["name"].as_str().unwrap();
        write_png(&directory.join(format!("{name}.png")), &frame).unwrap();
        records.push(json!({"name": name, "preset": pose["preset"], "seed": 7,
            "camera": c, "scene": pose["scene"], "view": pose["view"],
            "width":1600,"height":1000,"material_debug":format!("{:?}",family.material)}));
    }
    std::fs::write(directory.join(format!("{species}-views.json")),
        serde_json::to_string_pretty(&json!({"source":"fn32/stills.json", "generator":"direct mature build", "stills":records})).unwrap()+"\n").unwrap();
}
