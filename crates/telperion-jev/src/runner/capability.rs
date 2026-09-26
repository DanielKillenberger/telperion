//! The Capability stage: the gate asks the generator's vocabulary and the
//! rebuilt tools what the species needs and cannot be drawn. A capability
//! the species needs and the generator cannot express stops the run here,
//! with its evidence in `gaps.md`, before generation refuses to run on it.
use std::path::PathBuf;

use super::literature::{self as lit, e};
use super::{gaps, Done, Run, Stage, Stop};
use crate::pipeline::canon::read_json;
use crate::pipeline::stages::gate;

pub struct Capability;

impl Stage for Capability {
    fn name(&self) -> &'static str {
        "capability"
    }

    fn inputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        let p = &run.paths;
        let mut files = vec![p.artifact("select"), p.packet("capability")];
        files.extend(run.tools()?.files());
        Ok(files)
    }

    fn outputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        Ok(vec![run.paths.artifact("gate")])
    }

    fn run(&self, run: &Run) -> Result<Done, String> {
        // The gate's own key covers neither the vocabulary nor the tools.
        lit::forget(run.paths.artifact("gate"))?;
        gate::run(&run.paths, &lit::checks(run)?).map_err(e)?;
        let found = needed(run)?;
        if found.iter().any(|g| g.kind == gaps::Kind::Identity) {
            gaps::write(&run.out(), &found)?;
        }
        lit::logged(run, "gate ran".into())
    }

    fn stop(&self, run: &Run) -> Result<Option<Stop>, String> {
        let ids: Vec<String> = needed(run)?
            .into_iter()
            .filter(|g| g.kind == gaps::Kind::Identity)
            .map(|g| g.trait_id)
            .collect();
        Ok((!ids.is_empty()).then_some(Stop::IdentityGaps(ids)))
    }
}

/// The capability half of the gap list, read off the gate record.
fn needed(run: &Run) -> Result<Vec<gaps::Gap>, String> {
    let gate = read_json(&run.paths.artifact("gate")).map_err(|e| e.to_string())?;
    gaps::capability(&gate, &run.paths.packet("capability"))
}
