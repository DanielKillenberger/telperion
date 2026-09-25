//! The species runner (fn-149): one path from a name to an accepted tree.
//!
//! Eight stages run in order, each writing the artifacts the next reads. A
//! stage records the content hash of every file it read; it reruns when an
//! input's bytes change or an output is gone, and never otherwise, so a
//! second run with nothing changed reruns nothing.
//! A run stops for two things: an identity gap waiting on its spec, and the
//! owner's look. A claim the search could not settle is kept as the range
//! its sources span or left unsourced, unless the run was asked to stop for
//! claims (`--settle-claims`). Every other condition is logged.

pub mod accept;
pub mod capability;
pub mod catalogue;
pub mod derive;
pub mod folder;
pub mod gaps;
pub mod inventory;
pub mod literature;
pub mod pins;
pub mod preflight;
pub mod preset;
pub mod profile;
pub mod record;
pub mod sources;
pub mod start;
pub mod tools;
pub mod tune;

mod drive;

use std::cell::OnceCell;
use std::path::{Path, PathBuf};

use crate::pipeline::stage::Paths;
pub use drive::{run, status, Scope};
use tools::Tools;

/// One stage: what it reads and writes, its body, and whether the run
/// stops after it, read from what is on disk.
pub trait Stage {
    fn name(&self) -> &'static str;
    /// Every file whose bytes the stage's output depends on.
    fn inputs(&self, run: &Run) -> Result<Vec<PathBuf>, String>;
    /// Every file the stage writes.
    fn outputs(&self, run: &Run) -> Result<Vec<PathBuf>, String>;
    fn run(&self, run: &Run) -> Result<Done, String>;
    fn stop(&self, run: &Run) -> Result<Option<Stop>, String>;
}

/// The stages, in the order they run.
pub const STAGES: [&dyn Stage; 8] = [
    &sources::Sources,
    &profile::Profile,
    &capability::Capability,
    &catalogue::Catalogue,
    &start::Start,
    &tune::Tune,
    &gaps::Gaps,
    &accept::Accept,
];

/// Why a run stops.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stop {
    /// Claims left for a person in `resolutions.json` (`--settle-claims`);
    /// their ids.
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
    /// Claims the search could not settle stop the run for a person.
    pub settle_claims: bool,
    /// Whether the render tools are built when first read; a status run
    /// reads the ones on disk.
    pub build: bool,
    tools: OnceCell<Tools>,
}

impl Run {
    pub fn new(species: &str, paths: Paths, catalogue: PathBuf, tuning: PathBuf) -> Self {
        Self {
            species: species.into(),
            paths,
            catalogue,
            tuning,
            adapter: "firecrawl".into(),
            accept: false,
            settle_claims: false,
            build: true,
            tools: OnceCell::new(),
        }
    }

    /// The runner's own files: records, log, tuning revisions, gaps.
    pub fn out(&self) -> PathBuf {
        self.paths.run.join("runner")
    }

    pub fn folder(&self) -> PathBuf {
        self.catalogue.join(&self.species)
    }

    /// The render tools, built from this checkout at the first stage that
    /// reads them, so a run that never draws or measures never builds.
    pub fn tools(&self) -> Result<&Tools, String> {
        if let Some(tools) = self.tools.get() {
            return Ok(tools);
        }
        let root = Path::new(".");
        let tools = match self.build {
            true => Tools::build(root)?,
            false => Tools::at(root),
        };
        Ok(self.tools.get_or_init(|| tools))
    }
}

/// What one stage did.
pub enum Done {
    Current,
    Ran(String),
}

/// A path under the repository root, as written in a record.
pub fn shown(path: &Path) -> String {
    path.display().to_string()
}
