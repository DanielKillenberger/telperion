//! A found photograph's shot, chosen, never supplied (host, 2026-09-26).
//!
//! For each reference the Profile stage kept, code renders the tree it
//! starts from under six candidate cameras and the reviewer names the one
//! whose framing matches the photograph (stage `shot`). Code then measures
//! that render's tree box and draws candidate boxes and crown-base lines
//! over the photograph, and the reviewer names a box, a line and a light
//! preset (stage `outline`). Every number is code's (`candidates`); a look
//! names labels or none. No camera leaves the reference unused; no box or
//! no line leaves it out of the numeric targets; no light draws it under
//! the overcast preset. Each is recorded with the candidates and the
//! selection in `runner/shots.json`, and a rerun reuses it.
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value};

use crate::pipeline::canon::{read_json, write_canonical};
use crate::pipeline::photos::screen::ask;
use crate::tuning::matched::still;
use crate::tuning::vision::Adapter;

pub mod candidates;
use candidates::{boxes, cameras, crown_base, image_size, lights, lines, overcast};

pub const SHOT_PROTOCOL: &str = "reference-shot-v1";
pub const OUTLINE_PROTOCOL: &str = "reference-outline-v1";
pub const SHOT_PROMPT: &str = "The first image is a photograph of a tree of the species `target_species`; the others are renders of a generated tree of that species under candidate cameras, in the order `candidates` lists them. Choose the candidate whose framing best matches the photograph: where the tree stands in the frame, how much of the frame's height it fills, and how far below the crown the camera looks up from. Judge the framing only, not the tree's shape, colour or detail. Answer none when no candidate frames the tree the way the photograph does. Give no numbers.";
pub const OUTLINE_PROMPT: &str = "The first image is a photograph of a tree. The second, when present, draws candidate boxes over it, labelled as `boxes.labels` lists them: choose the box that most closely bounds the whole tree, from the tip of its highest twig to the ground at its trunk and from its outermost branch on one side to the other, or none. The third, when present, draws horizontal lines labelled as `lines.labels` lists them: choose the line nearest the crown base, the height where the clear trunk ends and the lowest branches of the crown begin, or none. Choose the entry of `lights` that best describes the photograph's light, or none. Judge only what the images show; give no numbers.";

/// The height candidate renders are drawn at, in pixels.
const HEIGHT: u32 = 480;

/// The references with their chosen shots, beside the run.
pub fn file(out: &Path) -> PathBuf {
    out.join("shots.json")
}

/// What renders a candidate: the tool, the tree and where the stills go.
struct Drawing<'a> {
    headless: &'a Path,
    frames: PathBuf,
    preset: String,
    seed: u32,
    family: PathBuf,
    dir: PathBuf,
}

/// Chooses a shot for every kept reference that has none, reusing each one
/// `runner/shots.json` recorded; the file and the stage's words.
pub fn select(config: &Value, headless: &Path, out: &Path) -> Result<(PathBuf, String), String> {
    let text = |v: &Value, what: &str| v.as_str().map(str::to_string).ok_or(format!("{what}?"));
    let packet = text(&config["matched"]["references"], "matched.references")?;
    let mut doc = read_json(Path::new(&packet)).map_err(|e| format!("{packet}: {e}"))?;
    let previous = read_json(&file(out)).unwrap_or(Value::Null);
    let found = read_json(&super::inventory::found(out)).unwrap_or(Value::Null);
    let photo = |sha: &str| {
        let listed = found.as_array().into_iter().flatten();
        listed
            .clone()
            .find(|i| i["sha256"] == sha)
            .and_then(|i| i["path"].as_str().map(PathBuf::from))
    };
    let overrides = &config["initial_overrides"];
    let height = overrides
        .pointer("/skeleton/envelope/height")
        .and_then(Value::as_f64);
    let (mut chosen, mut reused, mut calls, mut unused) = (0, 0, 0, 0);
    let mut tools: Option<(Drawing, Adapter)> = None;
    for record in doc["references"].as_array_mut().into_iter().flatten() {
        let view = record["view"].as_str().unwrap_or_default().to_string();
        let hand_matched = record["shot"].is_object() && record["shot_selection"].is_null();
        if hand_matched || !["leaf-on", "bare", "bark"].contains(&view.as_str()) {
            continue;
        }
        let sha = record["asset_sha256"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let path = photo(&sha).ok_or(format!("no copy of the photograph {sha}"))?;
        record["photo_path"] = json!(path);
        let recorded = previous["references"].as_array().into_iter().flatten();
        let recorded = recorded
            .clone()
            .find(|r| r["asset_sha256"] == sha && r["view"] == view);
        let (shot, selection) = match recorded {
            Some(r) => {
                reused += 1;
                (r["shot"].clone(), r["shot_selection"].clone())
            }
            None => {
                let id = record["id"].as_str().unwrap_or("photo").to_string();
                if tools.is_none() {
                    tools = Some(prepare(config, headless, out)?);
                }
                let (drawing, adapter) = tools.as_ref().unwrap();
                let made = choose(drawing, adapter, &id, &view, &path, height.unwrap_or(20.0))?;
                chosen += 1;
                calls += made.1["calls"].as_u64().unwrap_or(0);
                made
            }
        };
        if shot.is_null() {
            unused += 1;
        } else {
            record["shot"] = shot;
        }
        record["shot_selection"] = selection;
    }
    write_canonical(&file(out), &doc).map_err(|e| e.to_string())?;
    let word = format!(
        "shots: {chosen} chosen ({calls} looks), {reused} reused, {unused} matched no camera"
    );
    Ok((file(out), word))
}

/// What choosing a shot needs, read only when a reference needs one.
fn prepare<'a>(
    config: &Value,
    headless: &'a Path,
    out: &Path,
) -> Result<(Drawing<'a>, Adapter), String> {
    let text = |v: &Value, what: &str| v.as_str().map(str::to_string).ok_or(format!("{what}?"));
    let adapter: Adapter = serde_json::from_value(config["vision"].clone())
        .map_err(|e| format!("tuning config vision: {e}"))?;
    let dir = out.join("shots");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let family = dir.join("family.json");
    std::fs::write(&family, config["initial_overrides"].to_string()).map_err(|e| e.to_string())?;
    let compare = text(
        &config["matched"]["compare_script"],
        "matched.compare_script",
    )?;
    let drawing = Drawing {
        headless,
        frames: Path::new(&compare).with_file_name("shot-frames.py"),
        preset: text(&config["preset"], "preset")?,
        seed: config["seed"].as_u64().unwrap_or(1) as u32,
        family,
        dir,
    };
    Ok((drawing, adapter))
}

/// One reference's shot and the record of how it was chosen; a null shot
/// when no camera matched.
fn choose(
    d: &Drawing,
    a: &Adapter,
    id: &str,
    view: &str,
    photo: &Path,
    height_m: f64,
) -> Result<(Value, Value), String> {
    let bytes = std::fs::read(photo).map_err(|e| format!("{}: {e}", photo.display()))?;
    let (w, h) = image_size(&bytes).ok_or("the photograph's size is unreadable")?;
    let size = format!(
        "{}x{HEIGHT}",
        (HEIGHT as f64 * w as f64 / h as f64).round() as u32
    );
    let dir = d.dir.join(id);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let foliage = if view == "bare" { "hidden" } else { "leaf-on" };
    let draw = |camera: &Value, name: &str, twin: bool| -> Result<PathBuf, String> {
        let shot = json!({"camera": camera, "light": overcast(), "foliage": foliage});
        let path = dir.join(name);
        still(
            d.headless, &d.preset, d.seed, &shot, &size, &d.family, &path, twin,
        )?;
        Ok(path)
    };
    let cams = cameras(view, height_m);
    let mut renders = Vec::new();
    for (i, camera) in cams.iter().enumerate() {
        let path = draw(camera, &format!("camera-{i}.png"), false)?;
        renders.push(json!({"id": format!("camera-{i}"), "image": picture(&path, view)?}));
    }
    let photograph = picture(photo, view)?;
    let request = json!({"protocol": SHOT_PROTOCOL, "target_species": d.preset, "view": view,
        "photograph": photograph, "candidates": renders});
    let failed = "the shot look failed; the attempt is charged";
    let (answer, first) = ask(a, "shot", &request, SHOT_PROMPT, failed)?;
    let mut selection = json!({"candidates": {"camera": cams}, "calls": 1, "usage": [first],
        "selected": {"camera": answer["choice"]}});
    let Some(at) = label(&answer["choice"], "camera-") else {
        return Ok((Value::Null, selection));
    };
    let camera = cams[at].clone();
    let (framed, ruled) = match view {
        "bark" => (Vec::new(), Vec::new()),
        _ => {
            let render = dir.join(format!("camera-{at}.png"));
            let twin = draw(&camera, &format!("camera-{at}-twin.png"), true)?;
            let measured = frames(d, &["measure", "--still"], &[&render, &twin])?;
            let b = &measured["box"];
            let n = |i: usize| b[i].as_f64().ok_or("unreadable render box");
            let render = [n(0)?, n(1)?, n(2)?, n(3)?];
            (boxes(render), lines(render))
        }
    };
    let presets = lights(camera["azimuth"].as_f64().unwrap_or(candidates::AZIMUTH));
    let mut outline = json!({"protocol": OUTLINE_PROTOCOL, "view": view, "photograph": photograph,
        "boxes": null, "lines": null,
        "lights": presets.iter().map(|(id, said, _)| json!({"id": id, "description": said})).collect::<Vec<_>>()});
    if !framed.is_empty() {
        let (boxed, lined) = (dir.join("boxes.png"), dir.join("lines.png"));
        let args = [
            "overlay".to_string(),
            "--boxes".into(),
            json!(framed).to_string(),
            "--lines".into(),
            json!(ruled).to_string(),
        ];
        let mut command = frames_command(d);
        command.args(&args).arg("--photo").arg(photo);
        command
            .arg("--out-boxes")
            .arg(&boxed)
            .arg("--out-lines")
            .arg(&lined);
        crate::tuning::matched::run(&mut command)?;
        let labels = |l: Vec<&String>| json!(l);
        outline["boxes"] = json!({"image": picture(&boxed, view)?,
            "labels": labels(framed.iter().map(|(l, _)| l).collect())});
        outline["lines"] = json!({"image": picture(&lined, view)?,
            "labels": labels(ruled.iter().map(|(l, _)| l).collect())});
    }
    let failed = "the outline look failed; the attempt is charged";
    let (picked, second) = ask(a, "outline", &outline, OUTLINE_PROMPT, failed)?;
    let tree_box = framed
        .iter()
        .find(|(l, _)| picked["box"] == l.as_str())
        .map(|(_, b)| *b);
    let line = ruled
        .iter()
        .find(|(l, _)| picked["crown_base"] == l.as_str())
        .map(|(_, y)| *y);
    let light = presets.iter().find(|(l, _, _)| picked["light"] == *l);
    let tree = match (tree_box, line) {
        (Some(b), Some(y)) => json!({"box": b, "crownBase": crown_base(b, y)}),
        _ => Value::Null,
    };
    selection["candidates"]["box"] = json!(framed);
    selection["candidates"]["crown_base"] = json!(ruled);
    selection["candidates"]["light"] = json!(presets
        .iter()
        .map(|(l, _, v)| json!([l, v]))
        .collect::<Vec<_>>());
    selection["selected"]["box"] = picked["box"].clone();
    selection["selected"]["crown_base"] = picked["crown_base"].clone();
    selection["selected"]["light"] = picked["light"].clone();
    selection["calls"] = json!(2);
    selection["usage"] = json!([selection["usage"][0].clone(), second]);
    let shot = json!({"aspect": [w, h], "camera": camera, "crop": [0.0, 0.0, 1.0, 1.0],
        "foliage": foliage, "light": light.map_or_else(overcast, |(_, _, v)| v.clone()),
        "tree": tree, "confidence": "chosen",
        "derivation": format!("Chosen from code's candidates by the reference-first reviewer (fn-157): camera-{at}, box {}, crown-base line {}, light {}.",
            picked["box"], picked["crown_base"], picked["light"])});
    Ok((shot, selection))
}

/// The candidate index a look named, or None for none.
fn label(named: &Value, prefix: &str) -> Option<usize> {
    named.as_str()?.strip_prefix(prefix)?.parse().ok()
}

/// An image as the reviewer's request names it.
fn picture(path: &Path, view: &str) -> Result<Value, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(json!({"path": path, "sha256": crate::sha256_hex(&bytes), "view": view, "seed": 0}))
}

fn frames_command(d: &Drawing) -> Command {
    let mut command = Command::new("timeout");
    command.args(["300", "uv", "run"]).arg(&d.frames);
    command
}

/// Runs `shot-frames.py measure` over a still and its twin.
fn frames(d: &Drawing, args: &[&str], images: &[&Path; 2]) -> Result<Value, String> {
    let mut command = frames_command(d);
    command
        .args(args)
        .arg(images[0])
        .arg("--twin")
        .arg(images[1]);
    let output = command.output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "shot-frames: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| format!("shot-frames: {e}"))
}
