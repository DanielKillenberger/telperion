//! The Accept stage: the owner looks at the tuned tree in the harness, and
//! `--accept` records the tree they looked at as a value table, one wire
//! pointer to one value, in `accepted.json`, writes the tree into core as the
//! species' preset and refreshes its pins. An acceptance names the tree's
//! key, so a later revision's tree waits for a look of its own, and it is
//! refused while the species' catalogue folder fails
//! `scripts/catalogue-check.mjs`: the palm shipped without its entry.
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use super::preset::{self, Names};
use super::{folder, pins, start::flatten, tune, Done, Run, Stage, Stop};
use crate::pipeline::canon::{read_json, write_canonical};
use crate::pipeline::manifest;
use telperion_core::{params, presets::Preset, Family};

pub struct Accept;

impl Stage for Accept {
    fn name(&self) -> &'static str {
        "accept"
    }

    fn inputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        Ok(vec![tune::result(&run.out())])
    }

    fn outputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        Ok(vec![file(&run.out())])
    }

    fn run(&self, run: &Run) -> Result<Done, String> {
        if !run.accept {
            return Ok(Done::Current);
        }
        let out = run.out();
        let word = self::run(
            Path::new("."),
            &names(run)?,
            &run.folder(),
            &tune::result(&out),
            &out,
        )?;
        Ok(Done::Ran(word))
    }

    fn stop(&self, run: &Run) -> Result<Option<Stop>, String> {
        let out = run.out();
        Ok((!accepted(&tune::result(&out), &out)?).then_some(Stop::OwnerLook))
    }
}

/// What the species registers under, from its admitted manifest.
fn names(run: &Run) -> Result<Names, String> {
    let admitted = manifest::load(&run.paths.manifest()).map_err(|e| e.to_string())?;
    let taxon = &admitted.manifest.taxon;
    Ok(Names {
        id: run.species.clone(),
        common: taxon.common_name.clone(),
        scientific: taxon.scientific_name.clone(),
    })
}

pub fn file(out: &Path) -> PathBuf {
    out.join("accepted.json")
}

fn current(result: &Path) -> Result<Value, String> {
    let value = read_json(result).map_err(|e| e.to_string())?;
    let tree = value["outcome"]["current"].clone();
    match tree.is_null() {
        true => Err("the tuning result has no current tree to accept".into()),
        false => Ok(json!({"preset": value["outcome"]["preset"], "tree": tree})),
    }
}

/// True when the tree on disk is the one the owner accepted.
pub fn accepted(result: &Path, out: &Path) -> Result<bool, String> {
    let path = file(out);
    if !path.exists() || !result.exists() {
        return Ok(false);
    }
    let record = read_json(&path).map_err(|e| e.to_string())?;
    Ok(record["key"] == current(result)?["tree"]["key"])
}

/// The catalogue check's failures that name this species' `folder`, run from
/// the repository `root` over the folder's own catalogue. A check that did
/// not run is an error, never a pass.
pub fn catalogue_failures(root: &Path, folder: &Path) -> Result<Vec<String>, String> {
    let catalogue = folder.parent().unwrap_or(Path::new("."));
    let species = folder
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let output = folder::script(root, catalogue, "catalogue-check.mjs")
        .output()
        .map_err(|e| format!("node scripts/catalogue-check.mjs: {e}"))?;
    if output.status.success() {
        return Ok(vec![]);
    }
    let printed = String::from_utf8_lossy(&output.stderr);
    let counted = printed
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .is_some_and(|l| l.starts_with("catalogue: ") && l.contains("failure"));
    if !counted {
        return Err(format!(
            "the catalogue check did not run: {}",
            printed.trim()
        ));
    }
    // The check names a folder from the root, `<catalogue>/<species>`.
    let names = |l: &&str| {
        let at = l.split(':').next().unwrap_or_default();
        at.split('/').any(|part| part == species) && at.split('/').next_back() != Some("")
    };
    Ok(printed.lines().filter(names).map(str::to_string).collect())
}

/// The family the owner accepted: the tuning base with the tree's overlay.
pub fn family(result: &Path) -> Result<Family, String> {
    let now = current(result)?;
    let base = now["preset"]
        .as_str()
        .ok_or("the tuning result names no preset")?;
    let preset = Preset::from_id(base).ok_or(format!("unknown preset {base}"))?;
    params::overlay(&preset.parameters(), &now["tree"]["overrides"]).map_err(|e| format!("{e:?}"))
}

/// Accepts the current tree: its pins into the species' catalogue folder,
/// the folder through its check, the tree into core as the species' preset,
/// then the record. A refusal leaves the preset untouched.
pub fn run(
    root: &Path,
    names: &Names,
    folder: &Path,
    result: &Path,
    out: &Path,
) -> Result<String, String> {
    let accepted = family(result)?;
    pins::write(folder, &names.id, &accepted)?;
    let now = current(result)?;
    let shown: Vec<Value> = now["tree"]["stills"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|s| json!({"path": s["path"], "seed": s["seed"], "view": s["view"], "visual_status": "accepted"}))
        .collect();
    folder::stills(folder, &names.id, &names.id, &shown)?;
    let catalogue = folder.parent().unwrap_or(Path::new("."));
    folder::pages(root, catalogue)?;
    let failures = catalogue_failures(root, folder)?;
    if !failures.is_empty() {
        return Err(format!(
            "the catalogue entry fails its check: {}",
            failures.join("; ")
        ));
    }
    let tree = &now["tree"];
    let note = format!(
        "Accepted by the owner on {} as tuning tree {} (species runner, fn-149).",
        &crate::pipeline::stage::now()[..10],
        tree["key"].as_str().unwrap_or_default()
    );
    let written = preset::write(root, names, &accepted, &note)?;
    let table = flatten(&tree["overrides"]);
    let record = json!({
        "schema": "runner-accepted", "schema_version": 1,
        "preset": now["preset"], "key": tree["key"],
        "values": table, "stills": tree["stills"],
    });
    write_canonical(&file(out), &record).map_err(|e| e.to_string())?;
    Ok(format!(
        "accepted {}: {written}; pins refreshed",
        tree["key"]
    ))
}
