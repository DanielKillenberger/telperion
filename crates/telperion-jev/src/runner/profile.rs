//! The Profile stage: extract, screen, quality, select, verify and fit
//! write `packet/profile.json`. A requirement below its bar, or a flagged
//! value with a search round left, is searched for again and the profile
//! rerun; once the rounds are spent the runner settles the claim itself
//! (`literature::settle`). It then finds the reference photographs
//! (`pipeline::photos`) and builds the reviewer's inventory of them.
use std::path::PathBuf;

use super::literature::{self as lit, e, said, settle, Jev};
use super::{inventory, Done, Run, Stage, Stop};
use crate::pipeline::canon::read_json;
use crate::pipeline::judge::Judge;
use crate::pipeline::photos;
use crate::pipeline::search;
use crate::pipeline::stages::{extract, fetch, fit, quality, screen, select, verify};

pub struct Profile;

const INNER: [&str; 6] = ["extract", "screen", "quality", "select", "verify", "fit"];

impl Stage for Profile {
    fn name(&self) -> &'static str {
        "profile"
    }

    fn inputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        let p = &run.paths;
        Ok(vec![
            p.manifest(),
            p.resolutions(),
            p.artifact("fetch"),
            run.tuning.clone(),
        ])
    }

    fn outputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        let p = &run.paths;
        let mut files: Vec<PathBuf> = INNER.map(|n| p.artifact(n)).to_vec();
        files.extend([p.packet("profile"), p.packet("references")]);
        files.extend(inventory::files(&run.tuning, &run.out())?);
        Ok(files)
    }

    fn run(&self, run: &Run) -> Result<Done, String> {
        lit::refresh(run, &self.outputs(run)?, &INNER)?;
        let jev = Jev::load()?;
        let (paths, judge) = (&run.paths, jev.judge(run));
        let mut words = Vec::new();
        loop {
            profile(run, &judge, &mut words)?;
            let sent = settle(run)?;
            let adapter = lit::adapter(run);
            match search::run(paths, adapter.as_ref(), &judge).map_err(e)? {
                search::Outcome::Nothing if sent.is_empty() => break,
                search::Outcome::Nothing => {}
                search::Outcome::Ran { .. } => {
                    said(&mut words, "search-again", true);
                    // What the search admitted is fetched before the
                    // profile reads it again.
                    fetch::run(paths, adapter.as_ref()).map_err(e)?;
                    words.extend(settle(run)?);
                }
            }
            words.extend(sent);
        }
        words.push(photographs(run, &judge)?);
        inventory::record(paths, &run.out())?;
        words.push(inventory::build(&run.tuning, &run.out())?);
        lit::logged(run, words.join(", "))
    }

    fn stop(&self, run: &Run) -> Result<Option<Stop>, String> {
        lit::claims(run)
    }
}

/// The reference photographs the run finds for itself, looked at through
/// the tuning config's reviewer adapter.
fn photographs(run: &Run, judge: &Judge<'_>) -> Result<String, String> {
    let tuning = read_json(&run.tuning).map_err(|e| format!("{}: {e}", run.tuning.display()))?;
    let adapter = serde_json::from_value(tuning["vision"].clone())
        .map_err(|e| format!("tuning config vision: {e}"))?;
    photos::find(
        &run.paths,
        &photos::Http,
        judge,
        &photos::Vision { adapter },
    )
}

/// One pass of the six pipeline stages.
fn profile(run: &Run, judge: &Judge<'_>, words: &mut Vec<String>) -> Result<(), String> {
    let paths = &run.paths;
    let ran = !matches!(extract::run(paths).map_err(e)?, extract::Outcome::Current);
    said(words, "extract", ran);
    let ran = !matches!(
        screen::run(paths, judge).map_err(e)?,
        screen::Outcome::Current
    );
    said(words, "screen", ran);
    let ran = !matches!(
        quality::run(paths, judge).map_err(e)?,
        quality::Outcome::Current
    );
    said(words, "quality", ran);
    let ran = !matches!(
        select::run(paths, judge).map_err(e)?,
        select::Outcome::Current
    );
    said(words, "select", ran);
    let ran = !matches!(
        verify::run(paths, judge).map_err(e)?,
        verify::Outcome::Current
    );
    said(words, "verify", ran);
    let ran = !matches!(fit::run(paths).map_err(e)?, fit::Outcome::Current);
    said(words, "fit", ran);
    Ok(())
}
