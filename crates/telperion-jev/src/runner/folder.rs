//! The records of a species' catalogue folder that no pipeline stage writes,
//! written from the run's own records so `scripts/catalogue-check.mjs` passes
//! a fresh run's folder: `sources.json` from the admitted manifest and the
//! fetch record, `stills.json` from the stills the run rendered, a `NOTES.md`
//! when there is none, the `pins.json` stub until Accept fills it, and the
//! pages `scripts/catalogue-pages.mjs` renders.
use std::path::Path;
use std::process::Command;

use serde_json::{json, Value};

use crate::pipeline::canon::read_json;
use crate::pipeline::manifest::Manifest;

/// Writes `value` unless the file already holds it: a record the catalogue
/// scripts wrote in their own key order is left byte for byte, so the
/// article that recorded its checksum stays current.
fn write(path: &Path, value: &Value) -> Result<(), String> {
    if read_json(path).ok().as_ref() == Some(value) {
        return Ok(());
    }
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    std::fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()))
}

fn today() -> String {
    crate::pipeline::stage::now()[..10].to_string()
}

/// One record per admitted source that was fetched and per source a kept
/// reference photograph cites. A record already written for the same id and
/// url is kept as it stands, the date it was verified on with it.
pub fn sources(
    folder: &Path,
    manifest: &Manifest,
    fetch: &Value,
    references: &Value,
) -> Result<(), String> {
    let path = folder.join("sources.json");
    let old = read_json(&path).unwrap_or_default();
    let kept = |id: &str, url: &str| {
        old["sources"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|s| s["id"] == id && s["url"] == url)
            .cloned()
    };
    let fetched = &fetch["body"]["sources"];
    let mut records: Vec<Value> = manifest
        .sources
        .iter()
        .filter(|s| fetched.get(&s.id).is_some())
        .map(|s| {
            kept(&s.id, &s.url).unwrap_or_else(|| {
                json!({"id": s.id, "url": s.url, "title": s.title, "attribution": s.title,
                    "rights": s.rights, "sha256": s.sha256, "use": "", "verified": today(),
                    "tables": s.tables})
            })
        })
        .collect();
    for (id, shots) in photographs(references) {
        if records.iter().any(|r| r["id"] == id.as_str()) {
            continue;
        }
        let first = shots[0];
        let url = first["url"].as_str().unwrap_or_default();
        let names: Vec<&str> = shots.iter().filter_map(|r| r["id"].as_str()).collect();
        records.push(kept(&id, url).unwrap_or_else(|| {
            json!({"id": id, "url": url, "title": first["attribution"],
                "attribution": first["attribution"], "rights": first["usage"], "sha256": null,
                "use": format!("Reference photograph the tuning reviewer compares renders against ({}).", names.join(", ")),
                "verified": first["accessed_at"].as_str().map_or_else(today, str::to_string),
                "tables": []})
        }));
    }
    let record = json!({"schema": "sources", "schema_version": 1,
        "species": manifest.species, "sources": records});
    write(&path, &record)
}

/// The references grouped by the source each cites, in first-cited order.
fn photographs(references: &Value) -> Vec<(String, Vec<&Value>)> {
    let mut out: Vec<(String, Vec<&Value>)> = Vec::new();
    for r in references["references"].as_array().into_iter().flatten() {
        let Some(id) = r["source_id"].as_str() else {
            continue;
        };
        match out.iter_mut().find(|(s, _)| s == id) {
            Some((_, list)) => list.push(r),
            None => out.push((id.to_string(), vec![r])),
        }
    }
    out
}

/// A copy of each source a reference photograph cites, where none exists: the
/// page is not fetched, so the copy records it as unavailable.
pub fn reference_copies(
    root: &Path,
    folder: &Path,
    species: &str,
    references: &Value,
) -> Result<(), String> {
    for (id, _) in photographs(references) {
        if folder.join("sources").join(format!("{id}.md")).exists() {
            continue;
        }
        let catalogue = folder.parent().unwrap_or(Path::new("."));
        let output = script(root, catalogue, "catalogue-sources.mjs")
            .args(["--species", species, "--source", &id])
            .args(["--fetched", &today(), "--unavailable"])
            .output()
            .map_err(|e| format!("node scripts/catalogue-sources.mjs: {e}"))?;
        if !output.status.success() {
            return Err(format!(
                "{id}: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
    }
    Ok(())
}

/// The stills the run rendered, by their bytes, not yet judged by anyone.
pub fn stills(folder: &Path, species: &str, preset: &str, drawn: &[Value]) -> Result<(), String> {
    let mut rows = Vec::new();
    for still in drawn {
        let Some(path) = still["path"].as_str() else {
            continue;
        };
        let bytes = std::fs::read(path).map_err(|e| format!("{path}: {e}"))?;
        rows.push(json!({"path": path, "sha256": crate::sha256_hex(&bytes), "preset": preset,
            "seed": still["seed"].as_u64().unwrap_or(0), "view": still["view"].as_str().unwrap_or("whole"),
            "visual_status": still["visual_status"].as_str().unwrap_or("unassessed")}));
    }
    let record =
        json!({"schema": "stills", "schema_version": 1, "species": species, "stills": rows});
    write(&folder.join("stills.json"), &record)
}

/// A note, when the folder holds none: where its records came from.
pub fn notes(folder: &Path, manifest: &Manifest) -> Result<(), String> {
    let path = folder.join("NOTES.md");
    if std::fs::read_to_string(&path).is_ok_and(|n| !n.trim().is_empty()) {
        return Ok(());
    }
    let text = format!(
        "# {} notes\n\nOnboarded by the species runner (`docs/species-runner.md`) from the admitted \
         manifest; every record in this folder is one of its stages' artifacts.\n",
        manifest.taxon.common_name
    );
    std::fs::write(&path, text).map_err(|e| e.to_string())
}

/// The empty `pins.json` Accept fills, bound to the current profile.
pub fn pins_stub(folder: &Path, species: &str) -> Result<(), String> {
    let path = folder.join("pins.json");
    let old = read_json(&path).ok();
    if old.as_ref().is_some_and(|o| o["empty"] != true) {
        return Ok(());
    }
    let profile = std::fs::read(folder.join("packet/profile.json")).map_err(|e| e.to_string())?;
    let record = json!({"schema": "pins", "schema_version": 1, "species": species, "empty": true,
        "profile_sha256": crate::sha256_hex(&profile)});
    write(&path, &record)
}

/// The variable that names the catalogue to the catalogue scripts.
pub const CATALOGUE_VAR: &str = "TELPERION_CATALOGUE";

/// A catalogue script run from the repository `root` over `catalogue`, the
/// run's own (`species --catalogue`), never the repository's by default.
pub fn script(root: &Path, catalogue: &Path, name: &str) -> Command {
    let mut command = Command::new("node");
    command
        .current_dir(root)
        .env(CATALOGUE_VAR, absolute(catalogue))
        .arg(format!("scripts/{name}"));
    command
}

fn absolute(path: &Path) -> std::path::PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

/// Renders the catalogue's pages from its records.
pub fn pages(root: &Path, catalogue: &Path) -> Result<(), String> {
    let output = script(root, catalogue, "catalogue-pages.mjs")
        .output()
        .map_err(|e| format!("node scripts/catalogue-pages.mjs: {e}"))?;
    match output.status.success() {
        true => Ok(()),
        false => Err(format!(
            "catalogue-pages: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )),
    }
}
