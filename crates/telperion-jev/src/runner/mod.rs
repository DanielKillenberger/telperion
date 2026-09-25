//! The species runner (fn-149): one path from a name to an accepted tree.
//!
//! Eight stages run in order, each writing the artifacts the next reads. A
//! stage records the content hash of every file it read; it reruns when an
//! input's bytes change or an output is gone, and never otherwise, so a
//! second run with nothing changed reruns nothing.
//! A run stops for three things only: a sourced claim a person settles, an
//! identity gap waiting on its spec, and the owner's look. Every other
//! condition is rerun or written to the log.

pub mod accept;
pub mod derive;
pub mod gaps;
pub mod live;
pub mod pins;
pub mod pipeline;
pub mod preset;
pub mod record;
pub mod start;
pub mod tools;
pub mod tune;

use std::path::{Path, PathBuf};

use crate::pipeline::stage::Paths;
use record::Records;

/// The stages, in the order they run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// Discover, fetch and self-admit sources.
    Sources,
    /// Extract, screen, select, verify and fit: `packet/profile.json`.
    Profile,
    /// The species' needs against the generator's vocabulary.
    Capability,
    /// The packet's species record and stills, the source copies and the
    /// article in the catalogue folder.
    Catalogue,
    /// The profile's values mapped onto dials: the starting overlay.
    Start,
    /// Rounds over the live dials until a round keeps nothing.
    Tune,
    /// Every failing trait classed reachable, identity or global.
    Gaps,
    /// The owner's look; accepting writes the tree and the pins.
    Accept,
}

pub const STAGES: [Stage; 8] = [
    Stage::Sources,
    Stage::Profile,
    Stage::Capability,
    Stage::Catalogue,
    Stage::Start,
    Stage::Tune,
    Stage::Gaps,
    Stage::Accept,
];

impl Stage {
    pub fn name(self) -> &'static str {
        match self {
            Self::Sources => "sources",
            Self::Profile => "profile",
            Self::Capability => "capability",
            Self::Catalogue => "catalogue",
            Self::Start => "start",
            Self::Tune => "tune",
            Self::Gaps => "gaps",
            Self::Accept => "accept",
        }
    }
}

/// The three reasons a run stops.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stop {
    /// Contradicted or unsupported claims a person settles in
    /// `resolutions.json`; their ids.
    Claims(Vec<String>),
    /// Identity gaps waiting on their spec; their trait ids.
    IdentityGaps(Vec<String>),
    /// The owner looks in the harness and accepts with `--accept`.
    OwnerLook,
}

impl std::fmt::Display for Stop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Claims(ids) => write!(
                f,
                "claims to settle in resolutions.json: {}",
                ids.join(", ")
            ),
            Self::IdentityGaps(ids) => write!(
                f,
                "identity gaps waiting on their spec (see gaps.md): {}",
                ids.join(", ")
            ),
            Self::OwnerLook => {
                write!(f, "the owner's look: accept with `species <id> --accept`")
            }
        }
    }
}

/// Everything a run reads that is not an artifact.
pub struct Run {
    pub species: String,
    /// Pipeline artifacts, the manifest and the decisions.
    pub paths: Paths,
    /// The catalogue root; the species folder is `<catalogue>/<species>`.
    pub catalogue: PathBuf,
    /// The tuning config the Tune stage fills: references, required cells,
    /// reviewer adapters and tracks, authored with the manifest.
    pub tuning: PathBuf,
    /// `firecrawl`, or `fixture:<dir>` for an offline run.
    pub adapter: String,
    /// The owner accepted the tree they looked at.
    pub accept: bool,
}

impl Run {
    /// The runner's own files: records, log, tuning revisions, gaps.
    pub fn out(&self) -> PathBuf {
        self.paths.run.join("runner")
    }
    pub fn folder(&self) -> PathBuf {
        self.catalogue.join(&self.species)
    }
}

/// What one stage did.
pub enum Done {
    Current,
    Ran(String),
}

/// Runs every stage in order until one stops. `Ok(None)` is a run that
/// reached the end with the tree accepted.
pub fn run(run: &Run, stages: &dyn Stages) -> Result<Option<Stop>, String> {
    std::fs::create_dir_all(run.out()).map_err(|e| e.to_string())?;
    let mut records = Records::load(&run.out())?;
    for stage in STAGES {
        let inputs = stages.inputs(run, stage)?;
        let outputs = stages.outputs(run, stage);
        let done = match records.current(stage.name(), &inputs, &outputs)? {
            true => Done::Current,
            false => {
                let done = stages.run(run, stage).map_err(|e| {
                    let _ = records.log(stage.name(), &format!("failed: {e}"));
                    format!("{}: {e}", stage.name())
                })?;
                // Inputs are read again: a stage may write a file it also reads.
                let inputs = stages.inputs(run, stage)?;
                records.set(stage.name(), &inputs)?;
                done
            }
        };
        let word = match &done {
            Done::Current => "current".to_string(),
            Done::Ran(word) => format!("ran: {word}"),
        };
        records.log(stage.name(), &word)?;
        println!("{}: {word}", stage.name());
        if let Some(stop) = stages.stop(run, stage)? {
            records.log(stage.name(), &format!("stopped: {stop}"))?;
            return Ok(Some(stop));
        }
    }
    Ok(None)
}

/// The eight stages' bodies, behind a trait so a test drives the runner
/// without the network, Jev or a renderer.
pub trait Stages {
    /// Every file whose bytes the stage's output depends on.
    fn inputs(&self, run: &Run, stage: Stage) -> Result<Vec<PathBuf>, String>;
    /// Every file the stage writes.
    fn outputs(&self, run: &Run, stage: Stage) -> Vec<PathBuf>;
    fn run(&self, run: &Run, stage: Stage) -> Result<Done, String>;
    /// Whether the run stops after `stage`, read from what is on disk.
    fn stop(&self, run: &Run, stage: Stage) -> Result<Option<Stop>, String>;
}

/// A path under the repository root, as written in a record.
pub fn shown(path: &Path) -> String {
    path.display().to_string()
}
