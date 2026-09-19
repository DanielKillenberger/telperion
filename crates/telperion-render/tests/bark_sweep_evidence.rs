//! Explicit fn-71 footprint sweep replay; images are numeric diagnostics only.
use serde_json::json;
use std::{path::PathBuf, process::Command};
use telperion_core::{
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{
    hero_pose, render, shot_pose, write_png, Gpu, Renderer, SceneRow, Shot, View, GROUND_REACH,
    STILL_FORMAT,
};

const FACTORS: [f64; 6] = [1.5, 2.0, 3.0, 4.0, 6.0, 8.0];
const HERO_FACTORS: [f64; 6] = [1.12, 1.25, 1.5, 2.0, 3.0, 4.0];
const BEECH_BASE: &str =
    r#"{"azimuth":30,"elevation":0,"fill":1,"targetHeight":0.04,"fov":45,"distance":1.2}"#;
const BIRCH_BARK: &str =
    r#"{"azimuth":30,"elevation":0,"fill":1,"targetHeight":0.06,"fov":45,"distance":1.5}"#;

fn git(root: &std::path::Path, arguments: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(root)
        .args(arguments)
        .output()
        .expect("git required for source provenance");
    assert!(output.status.success(), "git provenance command failed");
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

#[test]
#[ignore = "numeric GPU evidence; BARK_SWEEP_SPECIES=beech|birch|birchhero and BARK_CAPTURE_DIR required"]
fn replay_fn71_footprint_sweep() {
    let name = std::env::var("BARK_SWEEP_SPECIES").expect("BARK_SWEEP_SPECIES required");
    let (preset, shot, factors) = match name.as_str() {
        "beech" => (Preset::EuropeanBeech, Some(BEECH_BASE), FACTORS),
        "birch" => (Preset::SilverBirch, Some(BIRCH_BARK), FACTORS),
        "birchhero" => (Preset::SilverBirch, None, HERO_FACTORS),
        _ => panic!("BARK_SWEEP_SPECIES must be beech, birch or birchhero"),
    };
    let size = if shot.is_some() {
        (2160, 1440)
    } else {
        (1350, 900)
    };
    let directory =
        PathBuf::from(std::env::var_os("BARK_CAPTURE_DIR").expect("BARK_CAPTURE_DIR required"));
    std::fs::create_dir_all(&directory).unwrap();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let commit = git(&root, &["rev-parse", "HEAD"]);
    let source_status = git(&root, &["status", "--porcelain"]);
    let mut family = preset.parameters();
    family.skeleton.seed = 1;
    let mut tree = mesh::build(&family, Detail::Full).unwrap();
    let gpu = pollster::block_on(Gpu::request(None)).expect("hardware GPU required for evidence");
    let adapter = format!("{:?}", gpu.adapter);
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    renderer.set_material(family.material);
    renderer.set_figure(false);
    renderer.set_view(View::Whole);
    let scene = SceneRow::default();
    renderer.set_scene(scene);
    let bounds = renderer.bounds().unwrap();
    let camera = match shot {
        Some(shot) => shot_pose(
            bounds,
            f64::from(size.0) / f64::from(size.1),
            GROUND_REACH,
            &Shot::parse(shot).unwrap(),
        ),
        None => hero_pose(bounds, 1.5, GROUND_REACH),
    };
    // Clay colours foliage and wood alike. Remove foliage only for the mask,
    // keeping the camera derived from the original whole-tree bounds.
    let foliage = std::mem::take(&mut tree.foliage.instances);
    renderer.submit(&tree).unwrap();
    renderer.set_view(View::Clay);
    let clay = render(&mut renderer, &camera, size.0, size.1).unwrap();
    assert!(clay.has_subject());
    write_png(&directory.join(format!("{name}-clay.png")), &clay).unwrap();
    tree.foliage.instances = foliage;
    renderer.submit(&tree).unwrap();
    renderer.set_view(View::Bare);
    let near = render(&mut renderer, &camera, size.0, size.1).unwrap();
    assert!(near.has_subject());
    write_png(&directory.join(format!("{name}-x1.png")), &near).unwrap();
    let mut reductions = Vec::new();
    for factor in factors {
        let width = (f64::from(size.0) / factor).round() as u32;
        let height = (f64::from(size.1) / factor).round() as u32;
        let frame = render(&mut renderer, &camera, width, height).unwrap();
        assert_eq!((frame.width, frame.height), (width, height));
        let file = format!("{name}-r{factor}.png");
        write_png(&directory.join(&file), &frame).unwrap();
        reductions.push(json!({"factor":factor, "width":width, "height":height, "file":file}));
    }
    let record = json!({
        "protocol":".flow/evidence/fn71/sweep.py", "purpose":"numeric footprint diagnostics only",
        "species":name, "preset":format!("{preset:?}"), "seed":1,
        "generator":"direct mature mesh::build", "figure":false,
        "width":size.0, "height":size.1, "shot":shot.map(|s| serde_json::from_str::<serde_json::Value>(s).unwrap()),
        "camera_recipe":if shot.is_some() { "shot_pose" } else { "hero_pose(bounds, 1.5, GROUND_REACH)" },
        "factors":factors,
        "bounds_view":"whole", "clay_view":"clay, wood-only instances", "material_view":"bare",
        "camera":{"position":[camera.position.x,camera.position.y,camera.position.z],
            "target":[camera.target.x,camera.target.y,camera.target.z],
            "field_of_view":camera.field_of_view,"near":camera.near,"far":camera.far},
        "scene":serde_json::from_str::<serde_json::Value>(&scene.to_json()).unwrap(),
        "material_debug":format!("{:?}",family.material), "samples":renderer.samples(),
        "adapter":adapter, "source_commit":commit, "source_status":source_status,
        "reductions":reductions, "camera_moved":false,
        "metric_command":format!("uv run --with numpy --with pillow .flow/evidence/fn71/sweep.py {} {name}",directory.display())
    });
    std::fs::write(
        directory.join(format!("{name}-sweep.json")),
        serde_json::to_string_pretty(&record).unwrap() + "\n",
    )
    .unwrap();
}
