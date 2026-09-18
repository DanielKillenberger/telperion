//! The render-and-measure step, behind a trait.
//!
//! A candidate value is a partial wire object laid over a shipped preset. The
//! species measurement example builds that tree and measures it; the headless
//! renderer draws it for the owner's eye. Code owns both: Jev never supplies a
//! number and never reaches this module. The value that ships is one code
//! proposed and a render measured, so every candidate on the described and the
//! reference route passes through `Measurer::measure` before it is chosen.

use std::{
    fmt, fs,
    path::{Path, PathBuf},
    process::Command,
};

use serde_json::{json, Value};

use crate::sha256_hex;

/// Why a render or a measurement produced no number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    /// The work directory, the family file or the receipt could not be used.
    Io(String),
    /// The measurement example refused the case, or completed none.
    Measure(String),
    /// The headless renderer refused the still, or is not configured.
    Render(String),
    /// A value was not the JSON the example documents.
    Json(String),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "io: {msg}"),
            Self::Measure(msg) => write!(f, "measure: {msg}"),
            Self::Render(msg) => write!(f, "render: {msg}"),
            Self::Json(msg) => write!(f, "receipt json: {msg}"),
        }
    }
}

impl std::error::Error for RenderError {}

/// One measured candidate: the metrics object and the receipt behind it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Measured {
    /// The `metrics` object of the species example's `completed` event.
    ///
    /// Each metric is one of three shapes, which [`metric`] reads: a scalar
    /// `{"status":"measured","value":<number>}` (height_m, dbh_m,
    /// crown_width_m, crown_base_m, crown_width_height_ratio, nodes and their
    /// kin, where dbh_m carries `"status":"measured_proxy"` and the extra keys
    /// `stems` and `diameters_m`); a distribution
    /// `{"status":"measured","min":n,"max":n,"median":n}` (foliage_length_m,
    /// foliage_width_m); or `{"status":"unavailable","reason":"..."}` when the
    /// tree offered nothing to measure. A few entries carry no number at all:
    /// `growth` is a status object, `foliage_unit` and `foliage_geometry_note`
    /// are plain strings, and `branch_order` is a scalar whose `value` is a
    /// histogram object.
    pub metrics: Value,
    /// The JSONL receipt the example wrote, one event per line.
    pub receipt_path: PathBuf,
}

/// The render-and-measure step. Implemented by [`SpeciesExample`] in the
/// pipeline and by a table in the tests, so no test needs a GPU or a binary.
pub trait Measurer {
    /// Measures one candidate: the partial wire object `family` laid over
    /// `preset` at `seed`. Returns the metrics object of the species example's
    /// `completed` event, whose shape is documented on [`Measured::metrics`].
    fn measure(&self, preset: &str, seed: u32, family: &Value) -> Result<Measured, RenderError>;
}

/// The measurement example and the headless renderer as the pipeline runs
/// them. Both are built binaries under `target/release/examples`.
#[derive(Debug, Clone)]
pub struct SpeciesExample {
    /// `target/release/examples/species_measure`.
    pub measure_binary: PathBuf,
    /// `target/release/examples/headless`, when stills are wanted.
    pub headless_binary: Option<PathBuf>,
    /// `.flow/evidence/fn9/profiles.json`, or a species profile manifest.
    pub profiles: PathBuf,
    /// The profile the case is checked against.
    pub profile_id: String,
    /// Scratch this run owns: family files and receipts land here.
    pub work_dir: PathBuf,
}

impl SpeciesExample {
    /// A stable name for this candidate's scratch files, from the preset, the
    /// seed and the family object, so a rerun reuses the same paths.
    fn stem(preset: &str, seed: u32, family: &Value) -> Result<String, RenderError> {
        let canonical =
            serde_json::to_string(family).map_err(|e| RenderError::Json(e.to_string()))?;
        let digest = sha256_hex(format!("{preset}\n{seed}\n{canonical}").as_bytes());
        Ok(format!("candidate-{}", &digest[..12]))
    }

    /// Writes `family` beside the receipts and returns its path.
    fn write_family(&self, stem: &str, family: &Value) -> Result<PathBuf, RenderError> {
        fs::create_dir_all(&self.work_dir).map_err(|e| RenderError::Io(e.to_string()))?;
        let path = self.work_dir.join(format!("{stem}.family.json"));
        let body = serde_json::to_vec(family).map_err(|e| RenderError::Json(e.to_string()))?;
        fs::write(&path, body).map_err(|e| RenderError::Io(e.to_string()))?;
        Ok(path)
    }

    /// The shipped preset's wire object, as `--print-family` prints it.
    pub fn print_family(&self, preset: &str) -> Result<Value, RenderError> {
        let output = Command::new(&self.measure_binary)
            .args(["--print-family", preset])
            .output()
            .map_err(|e| RenderError::Measure(format!("{}: {e}", self.measure_binary.display())))?;
        if !output.status.success() {
            return Err(RenderError::Measure(reason(
                &output.stderr,
                output.status.code(),
            )));
        }
        serde_json::from_slice(&output.stdout).map_err(|e| RenderError::Json(e.to_string()))
    }

    /// Draws one still of the same candidate, for the visual-unassessed
    /// decision the generate stage files. Needs a GPU, so no test runs it.
    pub fn still(
        &self,
        preset: &str,
        seed: u32,
        family: &Value,
        out_png: &Path,
        view: &str,
        size: &str,
    ) -> Result<PathBuf, RenderError> {
        let binary = self
            .headless_binary
            .as_ref()
            .ok_or_else(|| RenderError::Render("no headless binary is configured".into()))?;
        let stem = Self::stem(preset, seed, family)?;
        let family_path = self.write_family(&stem, family)?;
        let seed = seed.to_string();
        let output = Command::new(binary)
            .args(["--preset", preset, "--seed", &seed])
            .arg("--family")
            .arg(&family_path)
            .arg("--out")
            .arg(out_png)
            .args(["--view", view, "--size", size])
            .output()
            .map_err(|e| RenderError::Render(format!("{}: {e}", binary.display())))?;
        if !output.status.success() {
            return Err(RenderError::Render(reason(
                &output.stderr,
                output.status.code(),
            )));
        }
        Ok(out_png.to_path_buf())
    }
}

impl Measurer for SpeciesExample {
    fn measure(&self, preset: &str, seed: u32, family: &Value) -> Result<Measured, RenderError> {
        let stem = Self::stem(preset, seed, family)?;
        let family_path = self.write_family(&stem, family)?;
        let receipt_path = self.work_dir.join(format!("{stem}.measure.jsonl"));
        // The example refuses to overwrite evidence, and the work directory is
        // scratch this run owns, so a stale receipt of this candidate goes.
        if receipt_path.exists() {
            fs::remove_file(&receipt_path).map_err(|e| RenderError::Io(e.to_string()))?;
        }
        let case = format!("{stem}:{}:{preset}:{seed}", self.profile_id);
        // The child inherits this process's environment unchanged. Nothing is
        // added: the Jev key is never read, set or handed to a render.
        let output = Command::new(&self.measure_binary)
            .args(["--case", &case])
            .arg("--output")
            .arg(&receipt_path)
            .arg("--profiles")
            .arg(&self.profiles)
            .arg("--family")
            .arg(&family_path)
            .output()
            .map_err(|e| RenderError::Measure(format!("{}: {e}", self.measure_binary.display())))?;
        let receipt =
            fs::read_to_string(&receipt_path).map_err(|e| RenderError::Io(e.to_string()))?;
        let metrics = completed_metrics(&receipt, output.status.code())?;
        Ok(Measured {
            metrics,
            receipt_path,
        })
    }
}

/// The metrics of the completed case, or why there are none. A case that misses
/// its numeric gates still completes and still carries metrics, and the example
/// exits non-zero for it, so the receipt decides; the exit code only colours the
/// reason when no case completed.
fn completed_metrics(receipt: &str, code: Option<i32>) -> Result<Value, RenderError> {
    let mut failure = None;
    // An interrupted run leaves an unterminated final line. Every earlier event
    // was flushed whole, so an unparsable line is that tail: it is ignored
    // rather than read as a failure.
    let events = receipt
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok());
    for event in events {
        match event["event"].as_str() {
            Some("completed") => return Ok(event["metrics"].clone()),
            Some("failed") => {
                failure = Some(event["reason"].as_str().unwrap_or("unstated").to_owned());
            }
            _ => {}
        }
    }
    Err(RenderError::Measure(failure.unwrap_or_else(|| {
        format!("no completed case (exit {})", status_code(code))
    })))
}

/// The exit status and the last line the binary said, for a one-line reason.
fn reason(stderr: &[u8], code: Option<i32>) -> String {
    let printed = String::from_utf8_lossy(stderr);
    let tail = printed.trim().lines().next_back().unwrap_or("no output");
    format!("exit {}: {tail}", status_code(code))
}

fn status_code(code: Option<i32>) -> String {
    code.map_or_else(|| "signal".to_owned(), |code| code.to_string())
}

/// Reads a metric number out of the metrics object whatever its shape: a bare
/// number, a scalar's `value`, or a distribution's `median`. An `unavailable`
/// entry, an absent metric and a non-numeric one are all `None`.
pub fn metric(metrics: &Value, name: &str) -> Option<f64> {
    let entry = metrics.get(name)?;
    if let Some(number) = entry.as_f64() {
        return Some(number);
    }
    if entry.get("status").and_then(Value::as_str) == Some("unavailable") {
        return None;
    }
    entry
        .get("value")
        .and_then(Value::as_f64)
        .or_else(|| entry.get("median").and_then(Value::as_f64))
}

/// Turns a dotted dial path into the nested partial wire object the family
/// overlay takes: `skeleton.envelope.spread` at 0.4 becomes
/// `{"skeleton":{"envelope":{"spread":0.4}}}`. An empty path overlays nothing.
pub fn family_for(dial: &str, value: f64) -> Value {
    if dial.is_empty() {
        return json!({});
    }
    let mut node = json!(value);
    for key in dial.split('.').rev() {
        node = json!({ key: node });
    }
    node
}
