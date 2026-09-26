//! `species <id> --status` preflights before a paid run (owner, 2026-09-25):
//! the Jev key is visible, Firecrawl answers, and the vision adapter makes
//! its smallest call, and each stage that would run says what it is expected
//! to spend. It runs no stage. beech-proof-3 learned of a spent Claude quota
//! only when Stage A failed after the literature stages had been paid for.
use std::io::Write;
use std::process::{Command, Stdio};

use serde_json::{json, Value};

use super::Run;
use crate::caller::load_key;
use crate::pipeline::canon::read_json;
use crate::pipeline::manifest;
use crate::sha256_hex;
use crate::tape::{self, Tape};
use crate::tuning::vision::{refused, Adapter};

pub const PROBE: &str = "reference-probe-v1";
const PROMPT: &str = "Answer ok true.";

/// Each preflight check's name and what it found, or why it failed.
pub fn checks(run: &Run, tape: Option<&Tape>) -> Vec<(&'static str, Result<String, String>)> {
    if let Some(Tape::Replay(dir)) = tape {
        let offline = format!("replaying {}: no network, no key", dir.display());
        return vec![("replay", Ok(offline))];
    }
    vec![
        (
            "jev key",
            load_key()
                .map(|_| "visible".into())
                .map_err(|e| e.to_string()),
        ),
        ("firecrawl", firecrawl()),
        ("vision adapter", vision(run)),
    ]
}

fn firecrawl() -> Result<String, String> {
    let out = Command::new("firecrawl")
        .arg("--status")
        .output()
        .map_err(|e| format!("firecrawl: {e}"))?;
    let said = String::from_utf8_lossy(&out.stdout);
    let line = said
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join("; ");
    match out.status.success() {
        true => Ok(line),
        false => Err(format!(
            "{line} {}",
            String::from_utf8_lossy(&out.stderr).trim()
        )),
    }
}

/// The tuning config's reviewer adapter, asked its smallest call: no image
/// and a one-field answer.
fn vision(run: &Run) -> Result<String, String> {
    let tuning = read_json(&run.tuning).map_err(|e| format!("{}: {e}", run.tuning.display()))?;
    let mut value = tuning["vision"].clone();
    tape::adapters(&mut value);
    let adapter: Adapter = serde_json::from_value(value).map_err(|e| format!("vision: {e}"))?;
    let request = json!({"protocol": PROBE});
    let envelope = json!({"stage": "probe", "request": request,
        "request_sha256": sha256_hex(&serde_json::to_vec(&request).unwrap()),
        "prompt": PROMPT, "prompt_sha256": sha256_hex(PROMPT.as_bytes())});
    let mut child = Command::new("timeout")
        .arg(adapter.timeout_seconds.to_string())
        .arg(&adapter.program)
        .args(&adapter.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("{}: {e}", adapter.program.display()))?;
    child
        .stdin
        .take()
        .ok_or("missing adapter stdin")?
        .write_all(&serde_json::to_vec(&envelope).unwrap())
        .map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    let raw: Value = serde_json::from_slice(&out.stdout).unwrap_or(Value::Null);
    match raw["status"] == "ok" {
        true => Ok(format!("{} answered ({})", adapter.model, raw["usage"])),
        false => Err(refused(
            "no answer",
            &raw,
            &String::from_utf8_lossy(&out.stderr),
        )),
    }
}

/// What a run of `stage` is expected to spend, from the manifest's fields,
/// appearance traits and sources and the tuning config's required views.
/// An estimate, bounded by the caps the stages hold, never a meter.
pub fn expected(run: &Run, stage: &str) -> String {
    let admitted = manifest::load(&run.paths.manifest()).ok();
    let m = admitted.as_ref().map(|a| &a.manifest);
    let fields = m.map_or(0, |m| m.fields.len());
    let traits = m.map_or(0, |m| m.appearance.len() + m.described.len());
    let sources = m.map_or(0, |m| m.sources.len()).max(1);
    let views = read_json(&run.tuning)
        .ok()
        .and_then(|t| t["required"].as_array().map(Vec::len))
        .unwrap_or(1)
        .max(1);
    estimate(stage, fields, traits, sources, views)
}

pub fn estimate(stage: &str, fields: usize, traits: usize, sources: usize, views: usize) -> String {
    match stage {
        "sources" => format!(
            "about {} Firecrawl searches and up to {} scrapes (leads, rights), about {} Jev calls (ranking, rights)",
            2 * fields,
            8 * fields + sources,
            2 * fields
        ),
        "profile" => format!(
            "about {} Jev calls (screen, quality, select, verify) and up to {} more in search-again rounds; up to 12 Jev rights calls and 1 vision call for photographs; 1 vision call for the inventory",
            sources + 4 * fields + 2 * traits,
            8 * fields
        ),
        "catalogue" => format!("about {} Jev calls (generate, cite check)", traits + sources),
        "tune" => format!("per round, {views} contact-sheet look(s) and 1 reference-first comparison; rounds end when one keeps nothing"),
        _ => "no paid call".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::estimate;

    #[test]
    fn a_replay_preflights_nothing_it_would_have_to_reach() {
        use crate::pipeline::stage::Paths;
        use crate::tape::Tape;
        let dir = std::env::temp_dir().join("jev-preflight-replay");
        let run = super::Run::new("s", Paths::new(&dir), dir.clone(), dir.join("t.json"));
        let checks = super::checks(&run, Some(&Tape::Replay("/rec".into())));
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].0, "replay");
        assert!(checks[0].1.as_ref().unwrap().contains("no network"));
    }

    #[test]
    fn only_the_paid_stages_expect_a_spend() {
        assert!(estimate("sources", 6, 4, 1, 2).starts_with("about 12 Firecrawl searches"));
        assert!(estimate("tune", 6, 4, 1, 2).starts_with("per round, 2 contact-sheet"));
        for free in ["capability", "start", "gaps", "accept"] {
            assert_eq!(estimate(free, 6, 4, 1, 2), "no paid call");
        }
    }
}
