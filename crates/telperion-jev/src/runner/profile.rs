//! The Profile stage: read labels every span of every fetched document,
//! aggregate composes each field's value from them, and fit writes the
//! growth curves a manifest names; together they write
//! `packet/profile.json` (fn-157). It then finds the reference photographs
//! (`pipeline::photos`) and builds the reviewer's inventory of them.
use std::path::PathBuf;

use super::literature::{self as lit, e, said, Jev};
use super::{gaps, inventory, Done, Run, Stage, Stop};
use crate::pipeline::canon::read_json;
use crate::pipeline::judge::Judge;
use crate::pipeline::photos;
use crate::pipeline::stages::{aggregate, fit, read};

pub struct Profile;

const INNER: [&str; 3] = ["read", "aggregate", "fit"];

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
        let ran = !matches!(read::run(paths, &judge).map_err(e)?, read::Outcome::Current);
        said(&mut words, "read", ran);
        let ran = !matches!(
            aggregate::run(paths, &judge).map_err(e)?,
            aggregate::Outcome::Current
        );
        said(&mut words, "aggregate", ran);
        let ran = !matches!(fit::run(paths).map_err(e)?, fit::Outcome::Current);
        said(&mut words, "fit", ran);
        words.push(photographs(run, &judge)?);
        inventory::record(paths, &run.out())?;
        words.push(inventory::build(&run.tuning, &run.out())?);
        gaps::note_references(&run.tuning, &run.out())?;
        lit::logged(run, words.join(", "))
    }

    fn stop(&self, _: &Run) -> Result<Option<Stop>, String> {
        Ok(None)
    }
}

/// The reference photographs the run finds for itself, looked at through
/// the tuning config's reviewer adapter.
fn photographs(run: &Run, judge: &Judge<'_>) -> Result<String, String> {
    let tuning = read_json(&run.tuning).map_err(|e| format!("{}: {e}", run.tuning.display()))?;
    let mut vision = tuning["vision"].clone();
    crate::tape::adapters(&mut vision);
    let adapter =
        serde_json::from_value(vision).map_err(|e| format!("tuning config vision: {e}"))?;
    let web = crate::tape::Photos {
        inner: &photos::Http,
        tape: crate::tape::Tape::from_env(),
    };
    photos::find(&run.paths, &web, judge, &photos::Vision { adapter })
}
