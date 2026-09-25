//! The eight stages as a real run executes them.
use std::path::{Path, PathBuf};

use super::pipeline::{self, Literature};
use super::tools::Tools;
use super::{accept, gaps, start, tune, Done, Run, Stage, Stages, Stop};
use crate::pipeline::canon::read_json;

pub struct Live {
    pub tools: Tools,
    /// The tuning config's profile manifest and profile id, which the
    /// measurement example reads too.
    pub profiles: PathBuf,
    pub profile_id: String,
}

impl Live {
    pub fn new(run: &Run, tools: Tools) -> Result<Self, String> {
        let tuning =
            read_json(&run.tuning).map_err(|e| format!("{}: {e}", run.tuning.display()))?;
        Ok(Self {
            tools,
            profiles: PathBuf::from(start::field(&tuning, "profiles")?),
            profile_id: start::field(&tuning, "profile_id")?,
        })
    }

    fn literature<'a>(&'a self, run: &'a Run) -> Literature<'a> {
        Literature {
            run,
            tools: &self.tools,
            profiles: self.profiles.clone(),
            profile_id: self.profile_id.clone(),
        }
    }
}

impl Stages for Live {
    fn inputs(&self, run: &Run, stage: Stage) -> Result<Vec<PathBuf>, String> {
        let out = run.out();
        let p = &run.paths;
        Ok(match stage {
            Stage::Sources | Stage::Profile | Stage::Capability => pipeline::files(run, stage).0,
            Stage::Catalogue => {
                let mut files = pipeline::files(run, stage).0;
                files.extend(self.tools.files());
                files
            }
            Stage::Start => vec![
                p.packet("profile"),
                run.tuning.clone(),
                self.profiles.clone(),
            ],
            Stage::Tune => {
                let mut files = vec![start::file(&out), run.tuning.clone()];
                files.extend(self.tools.files());
                files
            }
            Stage::Gaps => vec![
                tune::result(&out),
                p.artifact("gate"),
                p.packet("capability"),
            ],
            Stage::Accept => vec![tune::result(&out)],
        })
    }

    fn outputs(&self, run: &Run, stage: Stage) -> Vec<PathBuf> {
        let out = run.out();
        match stage {
            Stage::Start => vec![start::file(&out)],
            Stage::Tune => vec![tune::result(&out)],
            Stage::Gaps => {
                let (json, md) = gaps::files(&out);
                vec![json, md]
            }
            Stage::Accept => vec![accept::file(&out)],
            _ => pipeline::files(run, stage).1,
        }
    }

    fn run(&self, run: &Run, stage: Stage) -> Result<Done, String> {
        let out = run.out();
        let p = &run.paths;
        let word = match stage {
            Stage::Start => start::run(&p.packet("profile"), &run.tuning, &out)?,
            Stage::Tune => tune::run(&run.tuning, &out)?,
            Stage::Gaps => gaps::run(
                &p.artifact("gate"),
                &p.packet("capability"),
                &tune::result(&out),
                &out,
            )?,
            Stage::Accept if run.accept => {
                accept::run(Path::new("."), &run.species, &tune::result(&out), &out)?
            }
            Stage::Accept => return Ok(Done::Current),
            _ => {
                let word = self.literature(run).run(stage)?;
                let (_, logged) = pipeline::open(run)?;
                match logged.is_empty() {
                    true => word,
                    false => {
                        let ids: Vec<&str> = logged.iter().map(|d| d.id.as_str()).collect();
                        format!("{word}; logged, not waited on: {}", ids.join(", "))
                    }
                }
            }
        };
        Ok(Done::Ran(word))
    }

    fn stop(&self, run: &Run, stage: Stage) -> Result<Option<Stop>, String> {
        let out = run.out();
        Ok(match stage {
            Stage::Profile | Stage::Catalogue => {
                let (claims, _) = pipeline::open(run)?;
                (!claims.is_empty()).then_some(Stop::Claims(claims))
            }
            Stage::Gaps => {
                let identity = gaps::identity(&out)?;
                (!identity.is_empty()).then_some(Stop::IdentityGaps(identity))
            }
            Stage::Accept => {
                (!accept::accepted(&tune::result(&out), &out)?).then_some(Stop::OwnerLook)
            }
            _ => None,
        })
    }
}
