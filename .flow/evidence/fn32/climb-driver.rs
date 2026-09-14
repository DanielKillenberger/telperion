//! fn-32's hill climb: coordinate descent over the bark rows against the
//! structure score, with no model in the loop and no image inspected.
//!
//! `bark_climb <oak|spruce> <log.json> <reference.png>...` renders the
//! species' trunk still headless, measures it, and walks each row up and down
//! until the score stops improving. A step is kept only if the guards in
//! `bark_climb/constraints.rs` still hold at it, so nothing the climb accepts
//! can have widened a bound. Every trial is logged. Temporary evidence
//! machinery: deleted before the final checkpoint and archived beside the
//! report.
#[path = "bark_climb/constraints.rs"]
mod constraints;

use std::path::PathBuf;

use constraints::{Guard, Level, Reading};
use serde_json::{json, Value};
use telperion_core::{
    material::MaterialParams,
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{centroid, measure_png, Gpu, Renderer, Structure, STILL_FORMAT};

/// One row of the material, its search interval and how far the climb first
/// steps along it. The interval is the row's own validated range, except for
/// the plate size, which is held near the species' own: how big a plate is is
/// what the species is, and the score's period component is deliberately too
/// weak to be trusted with it.
struct Row {
    name: &'static str,
    at: fn(&mut MaterialParams) -> &mut f64,
    low: f64,
    high: f64,
    step: f64,
}

fn rows(start: &MaterialParams) -> Vec<Row> {
    let row = |name, at: fn(&mut MaterialParams) -> &mut f64, low, high, step| Row {
        name,
        at,
        low,
        high,
        step,
    };
    vec![
        row(
            "plateCellScale",
            |m| &mut m.plate_cell_scale,
            (start.plate_cell_scale * 0.4).max(0.004),
            (start.plate_cell_scale * 2.5).min(1.0),
            0.006,
        ),
        row(
            "plateElongation",
            |m| &mut m.plate_elongation,
            0.0,
            16.0,
            0.15,
        ),
        row("plateDome", |m| &mut m.plate_dome, 0.0, 1.0, 0.05),
        row("plateEdgeLift", |m| &mut m.plate_edge_lift, 0.0, 1.0, 0.05),
        row(
            "plateFurrowWidth",
            |m| &mut m.plate_furrow_width,
            0.0,
            1.0,
            0.05,
        ),
        row("plateIdentity", |m| &mut m.plate_identity, 0.0, 1.0, 0.05),
        row(
            "weatheringStrength",
            |m| &mut m.weathering_strength,
            0.0,
            1.0,
            0.05,
        ),
        row("weatheringRed", |m| &mut m.weathering_red, -1.0, 1.0, 0.01),
        row(
            "weatheringGreen",
            |m| &mut m.weathering_green,
            -1.0,
            1.0,
            0.01,
        ),
        row(
            "weatheringBlue",
            |m| &mut m.weathering_blue,
            -1.0,
            1.0,
            0.01,
        ),
        row(
            "orientationStrength",
            |m| &mut m.orientation_strength,
            0.0,
            1.0,
            0.05,
        ),
        row(
            "orientationRed",
            |m| &mut m.orientation_red,
            -1.0,
            1.0,
            0.01,
        ),
        row(
            "orientationGreen",
            |m| &mut m.orientation_green,
            -1.0,
            1.0,
            0.01,
        ),
        row(
            "orientationBlue",
            |m| &mut m.orientation_blue,
            -1.0,
            1.0,
            0.01,
        ),
        row(
            "directionalOcclusion",
            |m| &mut m.directional_occlusion,
            0.0,
            1.0,
            0.05,
        ),
        row("depthStrength", |m| &mut m.depth_strength, 0.0, 1.0, 0.05),
    ]
}

fn vector(structure: &Structure) -> Value {
    json!({
        "orientation_entropy": round(structure.orientation_entropy),
        "area_variation": round(structure.area_variation),
        "furrow_curvature": round(structure.furrow_curvature),
        "junction_arms": round(structure.junction_arms),
        "dark_fraction": round(structure.dark_fraction),
        "furrow_period": round(structure.furrow_period),
    })
}

fn values(material: &MaterialParams, rows: &[Row]) -> Value {
    let mut copy = *material;
    let mut out = serde_json::Map::new();
    for row in rows {
        out.insert(row.name.into(), json!(round(*(row.at)(&mut copy))));
    }
    Value::Object(out)
}

fn checks(reading: &Reading) -> Value {
    Value::Array(
        reading
            .checks
            .iter()
            .map(|c| {
                json!({"case": c.name, "mean": round(c.mean), "p95": round(c.p95),
                    "redrew": c.redrew})
            })
            .collect(),
    )
}

fn round(value: f64) -> f64 {
    (value * 100_000.0).round() / 100_000.0
}

const SWEEPS: usize = 60;
/// A plateau: five accepted steps in a row that together bought less than one
/// per cent of the score they started from - and only once every row has been
/// refined to its finest step, because a coarse step that stops paying is a
/// step to shorten, not a hill to stop climbing.
const PLATEAU: f64 = 0.01;
/// How much of the plateau score the level pass may spend: two per cent.
const LEVEL_SLACK: f64 = 1.02;

fn run() -> Result<(), String> {
    let mut arguments = std::env::args().skip(1);
    let species = arguments.next().ok_or("usage: bark_climb <species> ...")?;
    // `bark_climb <species> probe:<row>=<value>[,...] <references>` reads one
    // material and prints what it measures, without climbing anything: a
    // question about a row, answered in one render.
    let log = PathBuf::from(arguments.next().ok_or("no log path")?);
    let references: Vec<Structure> = arguments
        .map(|path| measure_png(&PathBuf::from(path)).map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;
    if references.is_empty() {
        return Err("no references".into());
    }
    let target = centroid(&references);
    // The reference crop mean each species' level is held against, and how far
    // from it a candidate may stand. The oak is held to thirty code values of
    // the owner's white oak, which its round-two crop already sits at the edge
    // of, so in practice the band forbids it getting any lighter. The spruce
    // is held to seventy of the owner's spruce, wide enough to contain the
    // colour fn-29 was accepted at: those two disagree by more than any band,
    // and which of them is right is a question the owner has not answered.
    // Neither band may be left, and neither may be walked away from.
    let (preset, reference, band) = match species.as_str() {
        "oak" => (Preset::OregonWhiteOak, [118.0, 119.0, 114.0], 30.0),
        "spruce" => (Preset::NorwaySpruce, [154.0, 144.0, 140.0], 80.0),
        other => return Err(format!("unknown species {other}")),
    };
    let mut family = preset.parameters();
    family.skeleton.seed = 7;
    let tree = mesh::build(&family, Detail::Full).map_err(|e| e.to_string())?;
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).map_err(|e| e.to_string())?;
    let cases = if species == "oak" {
        constraints::oak_cases(&tree)
    } else {
        constraints::spruce_cases(&tree)
    };
    let mut guard = Guard::new(renderer, cases);
    let table = rows(&family.material);

    let mut material = family.material;
    if let Some(list) = log.to_string_lossy().strip_prefix("probe:") {
        for pair in list.split(',').filter(|p| !p.is_empty()) {
            let (name, value) = pair.split_once('=').ok_or("probe wants row=value")?;
            let row = table
                .iter()
                .find(|row| row.name == name)
                .ok_or(format!("unknown row {name}"))?;
            *(row.at)(&mut material) = value.parse::<f64>().map_err(|e| e.to_string())?;
        }
        let read = guard.read(material, true);
        println!(
            "{}",
            json!({"values": values(&material, &table), "vector": vector(&read.structure),
                "score": round(read.structure.distance(&target)), "checks": checks(&read),
                "colour": read.colour.map(round), "holds": read.holds()})
        );
        return Ok(());
    }
    let first = guard.read(material, true);
    if !first.holds() {
        return Err(format!(
            "the shipped rows already break a guard: {}",
            checks(&first)
        ));
    }
    let start = first.structure;
    let level = Level {
        reference,
        band,
        start: first.colour,
    };
    if !first.holds() || !first.keeps_colour(level) {
        return Err(format!(
            "the shipped rows already break a guard: {} at {:?}",
            checks(&first),
            first.colour
        ));
    }
    let mut score = start.distance(&target);
    eprintln!("{species} start {score:.4} {:?}", start);
    let mut trials: Vec<Value> = Vec::new();
    let mut accepted: Vec<f64> = vec![score];
    let mut steps: Vec<f64> = table.iter().map(|r| r.step).collect();
    let mut sweeps = 0;
    let mut stop = "budget";
    let mut touched = false;
    'climb: for sweep in 0..SWEEPS {
        sweeps = sweep + 1;
        let mut moved = false;
        for (index, row) in table.iter().enumerate() {
            for direction in [1.0, -1.0] {
                // Keep walking the way that helped, and give up on a row by
                // halving its step rather than by abandoning it.
                for _ in 0..4 {
                    let from = *(row.at)(&mut material);
                    let to = (from + direction * steps[index]).clamp(row.low, row.high);
                    if (to - from).abs() < 1e-9 {
                        break;
                    }
                    let mut candidate = material;
                    *(row.at)(&mut candidate) = to;
                    let tried = guard.read(candidate, false);
                    let next = tried.structure.distance(&target);
                    let better = next < score - 1e-6;
                    // The guards cost twelve draws; only a candidate that is
                    // worth keeping is asked to pay for them.
                    let full = better.then(|| guard.read(candidate, true));
                    let holds = full.as_ref().is_some_and(|full| {
                        full.holds()
                            && full.keeps_colour(level)
                            && full.keeps_dark(start.dark_fraction)
                    });
                    let mut entry = json!({
                        "sweep": sweep, "row": row.name, "from": round(from), "to": round(to),
                        "score": round(next), "accepted": better && holds,
                    });
                    if let Some(full) = &full {
                        entry["checks"] = checks(full);
                        entry["colour"] = json!(full.colour.map(round));
                        entry["keeps_colour"] = json!(full.keeps_colour(level));
                        entry["keeps_dark"] = json!(full.keeps_dark(start.dark_fraction));
                        entry["vector"] = vector(&full.structure);
                    }
                    trials.push(entry);
                    if !(better && holds) {
                        break;
                    }
                    material = candidate;
                    score = next;
                    accepted.push(score);
                    moved = true;
                    touched = true;
                    eprintln!("{species} {} {from:.4}->{to:.4} score {score:.4}", row.name);
                    let refined = steps
                        .iter()
                        .zip(&table)
                        .all(|(step, row)| *step <= row.step / 8.0 + 1e-12);
                    if refined && accepted.len() > 5 {
                        let then = accepted[accepted.len() - 6];
                        if (then - score) / then.max(1e-9) < PLATEAU {
                            stop = "plateau";
                            break 'climb;
                        }
                    }
                }
                if moved {
                    break;
                }
            }
            if !moved {
                steps[index] = (steps[index] * 0.5).max(row.step / 8.0);
            }
            moved = false;
        }
        let refined = steps
            .iter()
            .zip(&table)
            .all(|(step, row)| *step <= row.step / 8.0 + 1e-12);
        if !touched && refined {
            // Nothing moved anywhere, at the finest step every row has: that
            // is the plateau, whatever the last five steps bought.
            stop = "plateau";
            break;
        }
        touched = false;
    }
    // The level pass. The score is taken on the grey of a crop after an
    // auto-level, so it cannot see how light the trunk stands; the owner's eye
    // can, and the round-two verdict named it. With the descent at its
    // plateau, each row is walked in whichever direction carries the crop's
    // mean towards the reference, for as long as the score stays within
    // LEVEL_SLACK of the plateau and every guard still holds. The score is
    // never traded for level beyond that slack, and the structure the descent
    // bought is what the slack is measured against.
    let plateau = score;
    let away = |colour: [f64; 3]| {
        (0..3)
            .map(|c| (colour[c] - reference[c]).abs())
            .sum::<f64>()
    };
    let mut level_away = away(level.start);
    let mut levelled = 0;
    for _ in 0..12 {
        let mut moved = false;
        for (index, row) in table.iter().enumerate() {
            for direction in [1.0, -1.0] {
                let from = *(row.at)(&mut material);
                let to =
                    (from + direction * steps[index].max(row.step / 4.0)).clamp(row.low, row.high);
                if (to - from).abs() < 1e-9 {
                    continue;
                }
                let mut candidate = material;
                *(row.at)(&mut candidate) = to;
                let read = guard.read(candidate, true);
                let next = read.structure.distance(&target);
                let nearer = away(read.colour) < level_away - 1e-6;
                let cheap = next <= plateau * LEVEL_SLACK;
                let holds = read.holds()
                    && read.keeps_colour(level)
                    && read.keeps_dark(start.dark_fraction);
                trials.push(json!({
                    "pass": "level", "row": row.name, "from": round(from), "to": round(to),
                    "score": round(next), "colour": read.colour.map(round),
                    "accepted": nearer && cheap && holds,
                }));
                if !(nearer && cheap && holds) {
                    continue;
                }
                material = candidate;
                score = next;
                level_away = away(read.colour);
                levelled += 1;
                moved = true;
                eprintln!(
                    "{species} level {} {from:.4}->{to:.4} colour {:?} score {score:.4}",
                    row.name,
                    read.colour.map(|c| c.round())
                );
                break;
            }
        }
        if !moved {
            break;
        }
    }
    let last = guard.read(material, true);
    let json = json!({
        "species": species,
        "level": {"reference": reference, "band": band,
            "rule": "every channel of the crop mean inside the band around the reference, never further from it than the round started, and the channel order unchanged",
            "dark_floor": round(start.dark_fraction)},
        "target": vector(&target),
        "references": references.iter().map(vector).collect::<Vec<_>>(),
        "start": {"values": values(&family.material, &table), "vector": vector(&start),
            "score": round(start.distance(&target)), "colour": level.start.map(round)},
        "final": {"values": values(&material, &table), "vector": vector(&last.structure),
            "score": round(last.structure.distance(&target)), "checks": checks(&last),
            "colour": last.colour.map(round)},
        "sweeps": sweeps, "stopped": stop, "levelled": levelled,
        "plateau_score": round(plateau), "trials": trials,
    });
    std::fs::write(&log, serde_json::to_string_pretty(&json).unwrap())
        .map_err(|e| e.to_string())?;
    println!(
        "{species}: {:.4} -> {:.4} in {} trials, {stop}",
        start.distance(&target),
        last.structure.distance(&target),
        trials.len()
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
