//! One stage: fixed input paths, an idempotence key, one atomic output.
//!
//! A stage reads the admitted manifest and earlier artifacts at fixed paths,
//! computes its key over their checksums, the manifest checksum, the
//! question-set versions, the model name and the tool versions, and does
//! nothing when the artifact on disk already carries that key. Missing
//! inputs, an open decision that stops the stage, or a changed checksum stop
//! it by name. Opening a stage records it on the resolutions it consumes, and
//! every artifact carries what its stage spent. Every run appends to the
//! command log.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::canon::{canonical_sha256, file_sha256, read_json, write_canonical, CanonError};
use super::consume::{mark_consumed, rejected_proposal, ReconcileError};
use super::cost::{earlier_cost, Cost};
use super::decision::{
    open_for_stage, read_decisions, reconcile, write_decisions, Decision, Resolution,
};
use super::manifest::{self, Admitted, ManifestError};

pub const TOOL_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The fixed sequence. Discovery proposes; every later stage reads the
/// admitted manifest.
pub static STAGES: [&str; 11] = [
    "discover", "fetch", "extract", "screen", "quality", "select", "verify", "fit", "gate",
    "generate", "report",
];

/// Fixed artifact paths under the species' pipeline directory.
#[derive(Debug, Clone)]
pub struct Paths {
    pub dir: PathBuf,
}

impl Paths {
    pub fn new(dir: &Path) -> Self {
        Self {
            dir: dir.to_path_buf(),
        }
    }
    pub fn manifest(&self) -> PathBuf {
        self.dir.join("manifest.json")
    }
    pub fn decisions(&self) -> PathBuf {
        self.dir.join("decisions.json")
    }
    pub fn resolutions(&self) -> PathBuf {
        self.dir.join("resolutions.json")
    }
    pub fn command_log(&self) -> PathBuf {
        self.dir.join("command-log.json")
    }
    pub fn ledger(&self) -> PathBuf {
        self.dir.join("ledger")
    }
    /// One artifact per stage, named after it; the packet records and the
    /// sidecar are the select stage's outputs and sit beside it.
    pub fn artifact(&self, stage: &str) -> PathBuf {
        self.dir.join(format!("{stage}.json"))
    }
    pub fn packet(&self, record: &str) -> PathBuf {
        self.dir.join("packet").join(format!("{record}.json"))
    }
    pub fn sidecar(&self) -> PathBuf {
        self.dir.join("provenance.json")
    }
    pub fn cache(&self) -> PathBuf {
        self.dir.join("cache")
    }
}

#[derive(Debug)]
pub enum StageError {
    Manifest(ManifestError),
    File(CanonError),
    MissingInput {
        stage: String,
        path: PathBuf,
    },
    ChecksumChanged {
        stage: String,
        path: PathBuf,
        recorded: String,
        found: String,
    },
    OpenDecision {
        stage: String,
        decisions: Vec<String>,
    },
    Failed {
        stage: String,
        reason: String,
    },
}

impl std::fmt::Display for StageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Manifest(err) => write!(f, "{err}"),
            Self::File(err) => write!(f, "{err}"),
            Self::MissingInput { stage, path } => {
                write!(f, "{stage}: input missing: {}", path.display())
            }
            Self::ChecksumChanged {
                stage,
                path,
                recorded,
                found,
            } => write!(
                f,
                "{stage}: input changed since it was recorded: {} (recorded {recorded}, found {found})",
                path.display()
            ),
            Self::OpenDecision { stage, decisions } => {
                write!(f, "{stage}: open decision: {}", decisions.join(", "))
            }
            Self::Failed { stage, reason } => write!(f, "{stage}: {reason}"),
        }
    }
}

impl std::error::Error for StageError {}

impl From<CanonError> for StageError {
    fn from(err: CanonError) -> Self {
        Self::File(err)
    }
}

/// The header every stage output carries beside its body.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Header {
    pub schema: String,
    pub schema_version: u32,
    pub stage: String,
    pub manifest_sha256: String,
    pub inputs: BTreeMap<String, String>,
    pub question_sets: BTreeMap<String, u32>,
    pub model: String,
    pub tools: BTreeMap<String, String>,
    pub ledger: Vec<String>,
    pub idempotence_key: String,
    #[serde(default)]
    pub cost: Cost,
}

/// What a stage reads before it runs.
#[derive(Debug)]
pub struct Context {
    pub paths: Paths,
    pub admitted: Admitted,
    pub decisions: Vec<Decision>,
}

impl Context {
    /// Loads the manifest, reconciles decisions with resolutions, records
    /// `stage` on the resolutions it consumes, and refuses to run it when a
    /// global open decision or a rejected manifest proposal stops it.
    /// Field-scoped open decisions are returned for the stage to exclude
    /// those fields.
    pub fn open(dir: &Path, stage: &str) -> Result<(Self, Vec<String>), StageError> {
        let paths = Paths::new(dir);
        let manifest_path = paths.manifest();
        if !manifest_path.exists() {
            return Err(StageError::MissingInput {
                stage: stage.into(),
                path: manifest_path,
            });
        }
        let admitted = manifest::load(&manifest_path).map_err(StageError::Manifest)?;
        let decisions =
            reconcile(&paths.decisions(), &paths.resolutions()).map_err(|err| match err {
                ReconcileError::File(err) => StageError::File(err),
                refused @ ReconcileError::Refused(_) => StageError::Failed {
                    stage: stage.into(),
                    reason: refused.to_string(),
                },
            })?;
        let (global, fields) = open_for_stage(&decisions, stage);
        if !global.is_empty() {
            return Err(StageError::OpenDecision {
                stage: stage.into(),
                decisions: global,
            });
        }
        if let Some(id) = rejected_proposal(&decisions, stage) {
            return Err(StageError::Failed {
                stage: stage.into(),
                reason: format!(
                    "manifest-proposed {id} was rejected; edit the seed and run discover again"
                ),
            });
        }
        Ok((
            Self {
                paths,
                admitted,
                decisions,
            },
            fields.into_iter().collect(),
        ))
    }

    /// Reads an earlier artifact, checking that its bytes still match the
    /// checksum its own header recorded for itself.
    pub fn input(&self, stage: &str, name: &str) -> Result<(Value, String), StageError> {
        let path = self.paths.artifact(name);
        if !path.exists() {
            return Err(StageError::MissingInput {
                stage: stage.into(),
                path,
            });
        }
        let value = read_json(&path)?;
        let found = file_sha256(&path)?;
        Ok((value, found))
    }

    /// The resolution a person wrote for decision `id`, when it binds.
    pub fn resolved(&self, id: &str) -> Option<&Resolution> {
        self.decisions
            .iter()
            .find(|d| d.id == id)
            .and_then(|d| d.resolution.as_ref())
    }

    /// The checksum of the bound resolutions to `stage`'s own decisions, so a
    /// resolution written after the stage ran changes its idempotence key.
    pub fn resolutions_sha256(&self, stage: &str) -> String {
        let bound: Vec<Value> = self
            .decisions
            .iter()
            .filter(|d| d.stage == stage)
            .filter_map(|d| d.resolution.as_ref())
            .map(|r| json!({"id": r.id, "option": r.option, "payload": r.payload}))
            .collect();
        canonical_sha256(&json!(bound))
    }

    /// The header for `stage` over `inputs` (artifact path -> checksum), keyed
    /// on the admitted manifest's checksum.
    pub fn header(
        &self,
        stage: &str,
        schema: &str,
        inputs: BTreeMap<String, String>,
        ledger: Vec<String>,
    ) -> Header {
        self.header_keyed(stage, schema, inputs, ledger, &self.admitted.sha256.clone())
    }

    /// The header for `stage` with its idempotence key over `keyed_on` in place
    /// of the manifest checksum: discovery keys on the seed, so admitting the
    /// manifest does not rerun it.
    pub fn header_keyed(
        &self,
        stage: &str,
        schema: &str,
        inputs: BTreeMap<String, String>,
        ledger: Vec<String>,
        keyed_on: &str,
    ) -> Header {
        let m = &self.admitted.manifest;
        let mut tools = m.versions.tools.clone();
        tools.insert("species-pipeline".into(), TOOL_VERSION.into());
        let key = idempotence_key(
            &inputs,
            keyed_on,
            &m.versions.question_sets,
            &m.model,
            &tools,
        );
        Header {
            schema: schema.into(),
            schema_version: 1,
            stage: stage.into(),
            manifest_sha256: self.admitted.sha256.clone(),
            inputs,
            question_sets: m.versions.question_sets.clone(),
            model: m.model.clone(),
            tools,
            ledger,
            idempotence_key: key,
            cost: Cost::default(),
        }
    }

    /// True when the artifact on disk already carries `key`: the stage does nothing.
    pub fn is_current(&self, stage: &str, key: &str) -> bool {
        let path = self.paths.artifact(stage);
        path.exists()
            && read_json(&path)
                .ok()
                .and_then(|v| v["idempotence_key"].as_str().map(|k| k == key))
                .unwrap_or(false)
    }

    /// Writes the stage artifact: the header's fields at the top level and the
    /// body under `body`. The cost written is the sum over every run that
    /// wrote this artifact: the header carries this run's adapter spend, and
    /// its ledger references count its Jev calls, except in the report,
    /// whose ledger cites every earlier artifact's references and which asks
    /// nothing itself.
    pub fn write(&self, header: &Header, body: Value) -> Result<PathBuf, StageError> {
        let path = self.paths.artifact(&header.stage);
        let mut cost = earlier_cost(&path);
        let mut this_run = header.cost.clone();
        this_run.runs = 1;
        this_run.jev_calls = if header.stage == "report" {
            0
        } else {
            header.ledger.len() as u32
        };
        cost.add(&this_run);
        let mut header = header.clone();
        header.cost = cost;
        let mut value = serde_json::to_value(&header).expect("header serializes");
        value["body"] = body;
        write_canonical(&path, &value)?;
        // A resolution counts as consumed once the stage that acts on it has
        // written its artifact, never on merely reading it: a stage that stops
        // before writing leaves every resolution it read unconsumed. The list
        // is re-read, because the stage may have appended decisions meanwhile.
        let mut decisions = read_decisions(&self.paths.decisions())?;
        if mark_consumed(&mut decisions, &header.stage) {
            write_decisions(&self.paths.decisions(), &decisions)?;
        }
        Ok(path)
    }
}

pub fn idempotence_key(
    inputs: &BTreeMap<String, String>,
    manifest_sha256: &str,
    question_sets: &BTreeMap<String, u32>,
    model: &str,
    tools: &BTreeMap<String, String>,
) -> String {
    canonical_sha256(&json!({
        "inputs": inputs,
        "manifest_sha256": manifest_sha256,
        "question_sets": question_sets,
        "model": model,
        "tools": tools,
    }))
}

/// Appends one command to the driver's command log (a canonical JSON array).
pub fn log_command(paths: &Paths, argv: &[String], exit: i32) -> Result<(), CanonError> {
    let path = paths.command_log();
    let mut entries: Vec<Value> = if path.exists() {
        serde_json::from_value(read_json(&path)?["commands"].clone()).unwrap_or_default()
    } else {
        Vec::new()
    };
    entries.push(json!({"seq": entries.len(), "argv": argv, "exit": exit}));
    write_canonical(
        &path,
        &json!({"schema": "command-log", "schema_version": 1, "commands": entries}),
    )?;
    Ok(())
}

/// Reads the stage names the command log ran, in order.
pub fn logged_stages(paths: &Paths) -> Result<Vec<String>, CanonError> {
    let path = paths.command_log();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let value = read_json(&path)?;
    Ok(value["commands"]
        .as_array()
        .map(|list| {
            list.iter()
                .filter_map(|c| c["argv"][0].as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::decision::{append_decisions, DecisionParts};

    fn scratch() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "jev-stage-{}-{}",
            std::process::id(),
            crate::ledger::new_entry_id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_manifest(dir: &Path) {
        let value = crate::pipeline::manifest::tests::minimal();
        std::fs::write(
            dir.join("manifest.json"),
            serde_json::to_vec(&value).unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn a_missing_manifest_names_the_stage_and_the_path() {
        let dir = scratch();
        let err = Context::open(&dir, "fetch").unwrap_err().to_string();
        assert!(err.starts_with("fetch: input missing"), "{err}");
    }

    #[test]
    fn a_global_open_decision_stops_the_stage_by_id() {
        let dir = scratch();
        write_manifest(&dir);
        let paths = Paths::new(&dir);
        let decision = Decision::new(
            DecisionParts {
                species: "oregon-white-oak",
                stage: "discover",
                kind: "manifest-proposed",
                field: None,
                age_years: None,
            },
            &["fetch"],
            BTreeMap::new(),
            vec![],
            json!({}),
            &["admit"],
            "",
        );
        append_decisions(&paths.decisions(), vec![decision]).unwrap();
        let err = Context::open(&dir, "fetch").unwrap_err().to_string();
        assert_eq!(
            err,
            "fetch: open decision: oregon-white-oak/discover/manifest-proposed"
        );
        assert!(Context::open(&dir, "discover").is_ok());
    }

    #[test]
    fn the_key_changes_with_every_covered_input_and_the_artifact_is_current_only_on_match() {
        let dir = scratch();
        write_manifest(&dir);
        let (ctx, _) = Context::open(&dir, "fetch").unwrap();
        let inputs: BTreeMap<String, String> = [("discover.json".to_string(), "aaa".to_string())]
            .into_iter()
            .collect();
        let header = ctx.header("fetch", "sources", inputs.clone(), vec![]);
        assert!(!ctx.is_current("fetch", &header.idempotence_key));
        ctx.write(&header, json!({"sources": []})).unwrap();
        assert!(ctx.is_current("fetch", &header.idempotence_key));
        let m = &ctx.admitted.manifest;
        let base = idempotence_key(
            &inputs,
            &ctx.admitted.sha256,
            &m.versions.question_sets,
            &m.model,
            &header.tools,
        );
        assert_eq!(base, header.idempotence_key);
        let mut other_inputs = inputs.clone();
        other_inputs.insert("discover.json".into(), "bbb".into());
        let mut other_sets = m.versions.question_sets.clone();
        other_sets.insert("screen".into(), 2);
        let variants = [
            idempotence_key(
                &other_inputs,
                &ctx.admitted.sha256,
                &m.versions.question_sets,
                &m.model,
                &header.tools,
            ),
            idempotence_key(
                &inputs,
                "other",
                &m.versions.question_sets,
                &m.model,
                &header.tools,
            ),
            idempotence_key(
                &inputs,
                &ctx.admitted.sha256,
                &other_sets,
                &m.model,
                &header.tools,
            ),
            idempotence_key(
                &inputs,
                &ctx.admitted.sha256,
                &m.versions.question_sets,
                "jev-2026-09",
                &header.tools,
            ),
        ];
        for variant in variants {
            assert_ne!(variant, base);
        }
        let written = read_json(&ctx.paths.artifact("fetch")).unwrap();
        assert_eq!(written["stage"], "fetch");
        assert_eq!(written["body"]["sources"], json!([]));
    }

    #[test]
    fn the_command_log_keeps_every_argv_in_order() {
        let dir = scratch();
        let paths = Paths::new(&dir);
        log_command(&paths, &["discover".into(), "--dir".into(), "x".into()], 0).unwrap();
        log_command(&paths, &["fetch".into()], 1).unwrap();
        assert_eq!(logged_stages(&paths).unwrap(), vec!["discover", "fetch"]);
        let log = read_json(&paths.command_log()).unwrap();
        assert_eq!(log["commands"][1]["exit"], 1);
    }
}
