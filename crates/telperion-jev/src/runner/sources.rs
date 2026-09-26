//! The Sources stage: gather everything written about the species and
//! fetch it. A document that cannot be read is dropped and logged; nothing
//! is admitted or refused before it is read (fn-157).
use std::path::PathBuf;

use super::literature::{self as lit, e, said};
use super::{Done, Run, Stage, Stop};
use crate::pipeline::known::{flow_root, KnownSources};
use crate::pipeline::stages::{fetch, gather};

pub struct Sources;

const INNER: [&str; 2] = ["gather", "fetch"];

impl Stage for Sources {
    fn name(&self) -> &'static str {
        "sources"
    }

    fn inputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        Ok(vec![run.paths.manifest(), run.paths.resolutions()])
    }

    fn outputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        Ok(INNER.map(|n| run.paths.artifact(n)).to_vec())
    }

    fn run(&self, run: &Run) -> Result<Done, String> {
        lit::refresh(run, &self.outputs(run)?, &INNER)?;
        let (paths, adapter) = (&run.paths, lit::adapter(run));
        let known = KnownSources::scan(&run.catalogue, &flow_root(&paths.run), &paths.manifest());
        let mut words = Vec::new();
        let ran = gather::run(paths, adapter.as_ref(), &known).map_err(e)?;
        match ran {
            gather::Outcome::Ran { documents } => {
                words.push(format!("gather ran ({documents} documents)"))
            }
            gather::Outcome::Current => said(&mut words, "gather", false),
        }
        let ran = fetch::run(paths, adapter.as_ref()).map_err(e)?;
        said(&mut words, "fetch", !matches!(ran, fetch::Outcome::Current));
        words.extend(dropped(run)?);
        lit::logged(run, words.join(", "))
    }

    fn stop(&self, _: &Run) -> Result<Option<Stop>, String> {
        Ok(None)
    }
}

/// Each document the fetch could not read, named with its reason.
fn dropped(run: &Run) -> Result<Vec<String>, String> {
    let fetch = crate::pipeline::canon::read_json(&run.paths.artifact("fetch"))
        .map_err(|e| e.to_string())?;
    Ok(fetch["body"]["dropped"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(id, why)| {
            let reason = why["error"].as_str().or(why["option"].as_str());
            format!("dropped {id}: {}", reason.unwrap_or("unread"))
        })
        .collect())
}
