//! The fn-32 structure score, on stills and on references alike.
//!
//! `bark_score <png> [reference.png ...]` prints the still's structure vector
//! and, for each reference given, that reference's vector and the normalised
//! weighted distance between them. A reference photograph is converted to PNG
//! without being modified in any other way; nothing here writes an image and
//! nothing redistributes one.
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_render::{centroid, measure_png, Structure};

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

fn round(value: f64) -> f64 {
    (value * 10_000.0).round() / 10_000.0
}

fn run() -> Result<(), String> {
    let mut arguments = std::env::args_os().skip(1).map(PathBuf::from);
    let still = arguments
        .next()
        .ok_or("usage: bark_score <png> [reference.png ...]")?;
    let measured = measure_png(&still).map_err(|e| e.to_string())?;
    let mut references = Vec::new();
    let mut vectors = Vec::new();
    for path in arguments {
        let reference = measure_png(&path).map_err(|e| e.to_string())?;
        references.push(json!({
            "path": path, "vector": vector(&reference),
            "distance": round(measured.distance(&reference)),
        }));
        vectors.push(reference);
    }
    // The score of the round: the distance to the middle of this still's
    // catalogued references, because no single photograph is the species.
    let target = (!vectors.is_empty()).then(|| centroid(&vectors));
    println!(
        "{}",
        json!({
            "path": Path::new(&still), "vector": vector(&measured),
            "references": references,
            "target": target.map(|t| vector(&t)),
            "score": target.map(|t| round(measured.distance(&t))),
        })
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
