//! The Catalogue stage: generate draws the specimens, the gate audits their
//! seeds, the runner writes the folder records no pipeline stage writes
//! (`folder`), and document checks the article's citations and writes the
//! species record. The add-species agent writes `ARTICLE.md` when Accept
//! names it; the cite check verifies its claims.
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use super::literature::{self as lit, e, said, Jev};
use super::{folder, Done, Run, Stage, Stop};
use crate::pipeline::canon::read_json;
use crate::pipeline::manifest;
use crate::pipeline::stages::{document, gate, generate};

pub struct Catalogue;

/// The folder files the stage leaves in `<catalogue>/<species>`.
const FOLDER: [&str; 6] = [
    "ARTICLE.md",
    "sources.json",
    "stills.json",
    "NOTES.md",
    "README.md",
    "pins.json",
];

impl Stage for Catalogue {
    fn name(&self) -> &'static str {
        "catalogue"
    }

    fn inputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        let p = &run.paths;
        let mut files = vec![p.manifest(), p.resolutions()];
        files.extend(["select", "fit", "gate"].map(|n| p.artifact(n)));
        // A written article reruns the cite check.
        files.push(run.folder().join("ARTICLE.md"));
        files.extend(run.tools()?.files());
        Ok(files)
    }

    fn outputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        let p = &run.paths;
        let mut files = vec![p.artifact("generate"), p.artifact("document")];
        files.extend([p.packet("species"), p.packet("specimens")]);
        files.extend(FOLDER.map(|n| run.folder().join(n)));
        Ok(files)
    }

    fn run(&self, run: &Run) -> Result<Done, String> {
        lit::refresh(run, &self.outputs(run)?, &["generate", "document"])?;
        let jev = Jev::load()?;
        let (paths, judge) = (&run.paths, jev.judge(run));
        let example = lit::example(run)?;
        let mut words = Vec::new();
        let ran = generate::run(paths, &judge, &example, Some(&example)).map_err(e)?;
        said(
            &mut words,
            "generate",
            !matches!(ran, generate::Outcome::Current),
        );
        // The gate audits the specimen seeds generate just wrote.
        lit::forget(paths.artifact("gate"))?;
        gate::run(paths, &lit::checks(run)?).map_err(e)?;
        said(&mut words, "gate", true);
        records(run)?;
        let ran = document::run(paths, &judge).map_err(e)?;
        said(
            &mut words,
            "document",
            !matches!(ran, document::Outcome::Current),
        );
        folder::pages(Path::new("."))?;
        lit::logged(run, words.join(", "))
    }

    fn stop(&self, run: &Run) -> Result<Option<Stop>, String> {
        lit::claims(run)
    }
}

/// Writes the folder records no pipeline stage writes, before the document
/// stage's scripts read them.
fn records(run: &Run) -> Result<(), String> {
    let (paths, dir) = (&run.paths, run.folder());
    let read = |p: PathBuf| read_json(&p).map_err(|e| e.to_string());
    let admitted = manifest::load(&paths.manifest()).map_err(|e| e.to_string())?;
    let m = &admitted.manifest;
    let references = read(paths.packet("references"))?;
    folder::sources(&dir, m, &read(paths.artifact("fetch"))?, &references)?;
    folder::reference_copies(Path::new("."), &dir, &m.species, &references)?;
    let generated = read(paths.artifact("generate"))?;
    let drawn: Vec<Value> = generated["body"]["stills"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|s| json!({"path": s["path"], "seed": m.seed, "view": "whole"}))
        .collect();
    folder::stills(&dir, &m.species, &m.preset, &drawn)?;
    folder::notes(&dir, m)?;
    folder::pins_stub(&dir, &m.species)
}
