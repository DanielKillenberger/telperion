//! One look over a batch of candidate photographs, through the
//! reference-first reviewer's adapter (`scripts/reference-first.py`, stage
//! `screen`): per photograph, whether it shows the species, a mature
//! open-grown tree, the whole tree, and which view. The look selects among
//! photographs code found and checked; it writes no value.
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::sha256_hex;
use crate::tuning::evaluation::Image;
use crate::tuning::vision::Adapter;

pub const PROTOCOL: &str = "reference-screen-v1";
pub const PROMPT: &str = "You are shown candidate photographs for one tree species, in the order the request lists them. For each, say whether it shows that species (yes, no or unsure), whether it shows a mature tree grown in the open rather than in a stand or a nursery, whether the whole tree is in frame from the ground to the top of the crown, and which view it is: leaf-on for a whole tree in leaf, bare for a whole tree without leaves, bark for a close view of the trunk's bark, or other for anything else. Judge only what the photograph shows; give no numbers.";

/// What the look said of one photograph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Verdict {
    pub id: String,
    pub species: String,
    pub mature_open_grown: bool,
    pub whole_tree: bool,
    pub view: String,
}

impl Verdict {
    /// The view a kept photograph serves, or None when it is not kept: the
    /// species, and a mature open-grown whole tree unless it is the bark.
    pub fn kept_view(&self) -> Option<&str> {
        let tree = self.mature_open_grown && self.whole_tree;
        match self.view.as_str() {
            _ if self.species != "yes" => None,
            "bark" => Some("bark"),
            "leaf-on" | "bare" if tree => Some(self.view.as_str()),
            _ => None,
        }
    }
}

/// One look over every photograph; the verdicts and the call's usage.
pub trait Screen {
    fn screen(&self, species: &str, images: &[Image]) -> Result<(Vec<Verdict>, Value), String>;
}

/// The reviewer's adapter, one attempt per batch, its record in the ledger.
pub struct Vision {
    pub adapter: Adapter,
}

impl Screen for Vision {
    fn screen(&self, species: &str, images: &[Image]) -> Result<(Vec<Verdict>, Value), String> {
        let candidates: Vec<Value> = images
            .iter()
            .enumerate()
            .map(|(i, image)| json!({"id": format!("candidate-{i}"), "image": image}))
            .collect();
        let request =
            json!({"protocol": PROTOCOL, "target_species": species, "candidates": candidates});
        let hash = |v: &[u8]| sha256_hex(v);
        let envelope = json!({"stage": "screen", "request": request,
            "request_sha256": hash(&serde_json::to_vec(&request).unwrap()),
            "prompt": PROMPT, "prompt_sha256": hash(PROMPT.as_bytes())});
        let a = &self.adapter;
        let mut child = Command::new("timeout")
            .arg(a.timeout_seconds.to_string())
            .arg(&a.program)
            .args(&a.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("{}: {e}", a.program.display()))?;
        child
            .stdin
            .take()
            .ok_or("missing adapter stdin")?
            .write_all(&serde_json::to_vec(&envelope).unwrap())
            .map_err(|e| e.to_string())?;
        let output = child.wait_with_output().map_err(|e| e.to_string())?;
        ledger(&a.ledger, &request, &output)?;
        let raw: Value = serde_json::from_slice(&output.stdout).unwrap_or(Value::Null);
        if raw["status"] != "ok" {
            let failed = "the photograph screen failed; the attempt is charged";
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::tuning::vision::refused(failed, &raw, &stderr));
        }
        let verdicts = serde_json::from_value(raw["answer"]["candidates"].clone())
            .map_err(|e| format!("screen answer: {e}"))?;
        Ok((verdicts, raw["usage"].clone()))
    }
}

fn ledger(dir: &Path, request: &Value, output: &std::process::Output) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let path = dir.join(format!("{}.json", crate::ledger::new_entry_id()));
    let record = json!({"stage": "screen", "request": request, "exit": output.status.code(),
        "stdout": String::from_utf8_lossy(&output.stdout),
        "stderr": String::from_utf8_lossy(&output.stderr)});
    std::fs::File::create(&path)
        .and_then(|mut f| f.write_all(&serde_json::to_vec_pretty(&record).unwrap()))
        .map_err(|e| format!("{}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_photograph_is_kept_for_the_species_as_a_mature_whole_tree_or_its_bark() {
        let v = |species: &str, grown: bool, whole: bool, view: &str| Verdict {
            id: "c".into(),
            species: species.into(),
            mature_open_grown: grown,
            whole_tree: whole,
            view: view.into(),
        };
        assert_eq!(v("yes", true, true, "leaf-on").kept_view(), Some("leaf-on"));
        assert_eq!(v("yes", true, true, "bare").kept_view(), Some("bare"));
        assert_eq!(v("yes", false, false, "bark").kept_view(), Some("bark"));
        assert_eq!(v("yes", true, false, "leaf-on").kept_view(), None);
        assert_eq!(v("yes", false, true, "leaf-on").kept_view(), None);
        assert_eq!(v("unsure", true, true, "leaf-on").kept_view(), None);
        assert_eq!(v("yes", true, true, "other").kept_view(), None);
    }
}
