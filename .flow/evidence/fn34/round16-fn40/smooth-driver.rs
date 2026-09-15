//! Temporary fn-40 driver: one tree, one shot, many material rows.
//!
//! Archived evidence, not a shipped command: it ran as the example
//! `crates/telperion-render/examples/zz_smooth.rs` during the value trials of
//! round 16 and was removed from the crate at 8a69f3f7.
//!
//! `zz_smooth <preset> <size WxH> <camera json> <scene json> <variants.json> <out dir>`
//! renders the fn-34 close-up of the tree at seed 1 once per variant, where a
//! variant is `{"name": ..., "material": {<wire key>: value, ...}}` merged over
//! the preset's own material row. Nothing here is shipped.
use std::path::PathBuf;

use serde_json::Value;
use telperion_core::{
    mesh::{self, Detail},
    params,
    presets::Preset,
};
use telperion_render::{
    render, shot_pose, write_png, Gpu, Renderer, SceneRow, Shot, View, GROUND_REACH, STILL_FORMAT,
};

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [id, size, camera, scene, variants, out] = args.as_slice() else {
        return Err("usage: zz_smooth <preset> <WxH> <camera> <scene> <variants> <out>".into());
    };
    let (w, h) = size.split_once('x').ok_or("size")?;
    let (width, height): (u32, u32) = (w.parse().unwrap(), h.parse().unwrap());
    let mut family = Preset::from_id(id).ok_or("preset")?.parameters();
    family.skeleton.seed = 1;
    let wire = params::metadata(&family);
    let tree = mesh::build(&family, Detail::Full).map_err(|e| e.to_string())?;
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).map_err(|e| e.to_string())?;
    renderer.set_view(View::Bare);
    renderer.set_scene(SceneRow::parse(scene).map_err(|e| e.to_string())?);
    renderer.set_figure(false);
    let shot = Shot::parse(camera).map_err(|e| e.to_string())?;
    let bounds = renderer.bounds().ok_or("bounds")?;
    let pose = shot_pose(
        bounds,
        f64::from(width) / f64::from(height),
        GROUND_REACH,
        &shot,
    );
    let list: Vec<Value> =
        serde_json::from_str(&std::fs::read_to_string(variants).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    for variant in list {
        let mut row = wire.clone();
        for (key, value) in variant["material"].as_object().ok_or("material")? {
            row["material"][key] = value.clone();
        }
        let material = params::parse(&row).map_err(|e| e.to_string())?.material;
        renderer.set_material(material);
        let still = render(&mut renderer, &pose, width, height).map_err(|e| e.to_string())?;
        let path = PathBuf::from(out).join(format!("{}.png", variant["name"].as_str().unwrap()));
        write_png(&path, &still).map_err(|e| e.to_string())?;
        println!("{}", path.display());
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
