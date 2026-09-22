use std::{fs, path::Path};
use serde_json::{json, Value};
use telperion_core::{math::Vec3, mesh::{self, Detail}, presets::Preset, surface::Bounds};
use telperion_render::{shot_pose, Shot, GROUND_REACH};

fn extent(bounds: Bounds, shot: &Shot, aspect: f64) -> f64 {
    let m = shot_pose(bounds, aspect, GROUND_REACH, shot).view_projection(aspect);
    let mut max = 0.0f64;
    for x in [bounds.min.x, bounds.max.x] {
        for y in [bounds.min.y, bounds.max.y] {
            for z in [bounds.min.z, bounds.max.z] {
                let v = [x, y, z, 1.0];
                let mut p = [0.0; 4];
                for r in 0..4 {
                    p[r] = (0..4).map(|k| m[k * 4 + r] as f64 * v[k]).sum();
                }
                if p[3] <= 0.0 || p[2] / p[3] <= 0.0 || p[2] / p[3] >= 1.0 {
                    return f64::INFINITY;
                }
                max = max.max((p[0] / p[3]).abs()).max((p[1] / p[3]).abs());
            }
        }
    }
    max
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut p = Preset::from_id("silver-birch").unwrap().parameters();
    p.skeleton.seed = 1;
    let tree = mesh::build(&p, Detail::Full).unwrap();
    let bounds = tree.bounds;
    let refs: Value = serde_json::from_slice(&fs::read(&args[1]).unwrap()).unwrap();
    let record = refs["references"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == "S-WHOLE")
        .unwrap();
    let mut shot: Shot = Shot::parse(&record["shot"]["camera"].to_string()).unwrap();
    let width = (1440.0 * record["shot"]["aspect"][0].as_f64().unwrap()
        / record["shot"]["aspect"][1].as_f64().unwrap())
    .round() as u32;
    let aspect = width as f64 / 1440.0;
    let before = extent(bounds, &shot, aspect);
    let old = shot.fill;
    shot.fill = 0.8;
    if extent(bounds, &shot, aspect) > 0.95 {
        let (mut lo, mut hi) = (0.2, 0.8);
        shot.fill = lo;
        assert!(extent(bounds, &shot, aspect) <= 0.95);
        for _ in 0..48 {
            let mid = (lo + hi) * 0.5;
            shot.fill = mid;
            if extent(bounds, &shot, aspect) <= 0.95 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        shot.fill = lo;
    }
    let mut camera = record["shot"]["camera"].clone();
    camera["fill"] = json!(shot.fill);
    let xyz = |p: Vec3| vec![p.x, p.y, p.z];
    let result = json!({
        "preset":"silver-birch","seed":1,"revision":"0f2c2f6a",
        "bounds_min":xyz(bounds.min),"bounds_max":xyz(bounds.max),
        "method":"Historical silver-birch mesh::build; shot_pose 8 corners; maxabsXY<=.95; fill<=.8; no pixels",
        "shot":{"id":"S-WHOLE","camera":camera,"light":record["shot"]["light"],"size":[width,1440],
            "old_fill":old,"old_max_corner_ndc":before,"new_max_corner_ndc":extent(bounds,&shot,aspect)}
    });
    assert!(!Path::new(&args[2]).exists());
    fs::write(&args[2], serde_json::to_vec_pretty(&result).unwrap()).unwrap();
    println!("{result}");
}
