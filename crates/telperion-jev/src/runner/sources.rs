//! The Sources stage: discover, fetch and self-admit the manifest's
//! sources. A proposal nobody admitted is skipped and an unreadable source
//! dropped, each logged, and the fetch runs again without it.
use std::path::PathBuf;

use super::literature::{self as lit, e, said, settle, Jev};
use super::{Done, Run, Stage, Stop};
use crate::pipeline::known::{flow_root, KnownSources};
use crate::pipeline::stages::{discover, fetch};

pub struct Sources;

const INNER: [&str; 2] = ["discover", "fetch"];

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
        let jev = Jev::load()?;
        let (paths, judge, adapter) = (&run.paths, jev.judge(run), lit::adapter(run));
        let known = KnownSources::scan(&run.catalogue, &flow_root(&paths.run), &paths.manifest());
        let mut words = Vec::new();
        let ran = discover::run(paths, adapter.as_ref(), &judge, &known).map_err(e)?;
        said(
            &mut words,
            "discover",
            !matches!(ran, discover::Outcome::Current),
        );
        words.extend(settle(run)?);
        let ran = fetch::run(paths, adapter.as_ref()).map_err(e)?;
        said(&mut words, "fetch", !matches!(ran, fetch::Outcome::Current));
        let dropped = settle(run)?;
        if !dropped.is_empty() {
            fetch::run(paths, adapter.as_ref()).map_err(e)?;
            words.extend(dropped);
        }
        lit::logged(run, words.join(", "))
    }

    fn stop(&self, _: &Run) -> Result<Option<Stop>, String> {
        Ok(None)
    }
}
