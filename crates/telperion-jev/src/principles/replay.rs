//! The offline replay: frozen extractions and recorded answers for every
//! labelled case, run through the guards and the decision rule with no call.
//! A positive's mechanism counts as caught only when a guard owns it and
//! fires, or a finding cites its span under its principle. A mechanism no
//! candidate reached is unsupported coverage, never a pass.

use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::candidates::Extraction;
use super::policy::Policy;
use super::push::{boundary_added, triggered};
use super::review::{decide, select, Finding};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Corpus {
    pub version: String,
    pub cases: Vec<Case>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Case {
    pub id: String,
    pub pr: u32,
    /// `positive` or `clean`, as the owner confirmed on 2026-09-25.
    pub label: String,
    /// `calibration` or `holdout`, fixed by lineage.
    pub group: String,
    pub lineage: String,
    pub base: String,
    pub head: String,
    /// The healthy shape a clean case matches, when it was chosen for one.
    #[serde(default)]
    pub matched: Option<String>,
    #[serde(default)]
    pub mechanisms: Vec<Mechanism>,
    /// Guard results observed on the real revisions, e.g. `entry: red`.
    #[serde(default)]
    pub observed: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mechanism {
    pub id: String,
    pub principle: String,
    /// `jev`, `guard:boundary`, `guard:entry` or `guard:budget`.
    pub owner: String,
    pub path: String,
    pub lines: (usize, usize),
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Verdict {
    CaughtByGuard(String),
    CaughtByJev(String),
    /// A candidate reached the span, and no finding named it.
    Missed,
    /// No candidate reached the span: the extractor has no class for it.
    Unsupported,
}

#[derive(Debug, Clone, Serialize)]
pub struct CaseResult {
    pub id: String,
    pub label: String,
    pub group: String,
    pub mechanisms: Vec<(String, Verdict)>,
    pub findings: Vec<Finding>,
    pub incomplete: bool,
}

/// A span matches when the path agrees and the lines overlap within three.
fn overlaps(m: &Mechanism, path: &str, lines: (usize, usize)) -> bool {
    path == m.path && lines.0 <= m.lines.1 + 3 && m.lines.0 <= lines.1 + 3
}

fn location(loc: &str) -> (String, usize) {
    let (path, line) = loc.rsplit_once(':').unwrap_or((loc, "0"));
    (path.to_string(), line.parse().unwrap_or(0))
}

pub fn replay_case(case: &Case, x: &Extraction, answers: &Value, policy: &Policy) -> CaseResult {
    let (picked, _) = select(x, &answers["phase_one"], policy, true);
    let outcome = decide(x, &picked, &answers["phase_two"], policy);
    let boundary = boundary_added(x);
    let entry = !triggered(&x.paths, &policy.entry_triggers).is_empty();
    let mut verdicts = Vec::new();
    for m in &case.mechanisms {
        // Any guard that fires on the span catches it, whoever was expected
        // to: the push-mode boundary check on what the change added, the
        // cargo-test boundary check on the whole head, or an entry-coverage
        // or budget result observed on the real revision.
        let pushed = boundary.iter().find(|c| overlaps(m, &c.file, (c.line, c.line)));
        let tree = x.boundary_head.iter().find(|c| overlaps(m, &c.file, (c.line, c.line)));
        let verdict = if let Some(c) = pushed {
            Some(Verdict::CaughtByGuard(format!("boundary (push) {}:{}", c.file, c.line)))
        } else if let Some(c) = tree.filter(|_| m.owner == "guard:boundary") {
            Some(Verdict::CaughtByGuard(format!("boundary (cargo test, whole tree) {}:{}", c.file, c.line)))
        } else if m.owner == "guard:entry" && entry && case.observed.iter().any(|o| o == "entry: red") {
            Some(Verdict::CaughtByGuard("entry coverage".into()))
        } else if m.owner == "guard:budget" && case.observed.iter().any(|o| o == "budget: red") {
            Some(Verdict::CaughtByGuard("artifact budget".into()))
        } else {
            None
        };
        let verdict = verdict.or_else(|| {
            outcome
                .findings
                .iter()
                .find(|f| f.principle == m.principle && cites(f, x, m))
                .map(|f| Verdict::CaughtByJev(format!("{} {}", f.candidate, f.mechanism)))
        });
        let reached = x.candidates.iter().any(|c| c.evidence.iter().any(|e| e.added && overlaps(m, &e.path, e.lines)));
        verdicts.push((m.id.clone(), verdict.unwrap_or(if reached { Verdict::Missed } else { Verdict::Unsupported })));
    }
    CaseResult {
        id: case.id.clone(),
        label: case.label.clone(),
        group: case.group.clone(),
        mechanisms: verdicts,
        findings: outcome.findings,
        incomplete: x.overflowed(),
    }
}

/// The finding's candidate location or its cited excerpt covers the span.
fn cites(f: &Finding, x: &Extraction, m: &Mechanism) -> bool {
    let (path, line) = location(&f.location);
    if overlaps(m, &path, (line, line)) {
        return true;
    }
    let Some(c) = x.candidates.iter().find(|c| c.id == f.candidate) else { return false };
    c.evidence.iter().any(|e| f.evidence.iter().any(|cited| cited.contains(&format!("{}:{}-{}", e.path, e.lines.0, e.lines.1))) && overlaps(m, &e.path, e.lines))
}

/// Loads the corpus and replays every case that has both files.
pub fn replay_all(dir: &Path, policy: &Policy) -> Result<Vec<CaseResult>, String> {
    let corpus: Corpus = read(&dir.join("corpus.json"))?;
    let mut out = Vec::new();
    for case in &corpus.cases {
        let x: Extraction = read(&dir.join(format!("frozen/{}.extraction.json", case.id)))?;
        let answers: Value = read(&dir.join(format!("frozen/{}.answers.json", case.id)))?;
        out.push(replay_case(case, &x, &answers, policy));
    }
    Ok(out)
}

pub fn read<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

/// One rate with its count and a 95% Wilson interval.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Rate {
    pub hits: usize,
    pub of: usize,
    pub low: f64,
    pub high: f64,
}

impl Rate {
    pub fn of(hits: usize, of: usize) -> Self {
        if of == 0 {
            return Self { hits, of, low: 0.0, high: 1.0 };
        }
        let (n, p, z) = (of as f64, hits as f64 / of as f64, 1.96_f64);
        let centre = (p + z * z / (2.0 * n)) / (1.0 + z * z / n);
        let half = z * (p * (1.0 - p) / n + z * z / (4.0 * n * n)).sqrt() / (1.0 + z * z / n);
        Self { hits, of, low: (centre - half).max(0.0), high: (centre + half).min(1.0) }
    }
}

impl std::fmt::Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{} [{:.0}%, {:.0}%]", self.hits, self.of, 100.0 * self.low, 100.0 * self.high)
    }
}

/// Jev's end-to-end recall on the mechanisms it owns (extractor misses and
/// abstentions count against it), its finding precision, and the share of
/// clean pushes it flags, for one group.
#[derive(Debug, Clone, Serialize)]
pub struct Metrics {
    pub group: String,
    pub recall: Rate,
    pub precision: Rate,
    pub clean_flagged: Rate,
    pub unsupported: usize,
}

pub fn metrics(results: &[CaseResult], corpus: &Corpus, group: &str) -> Metrics {
    let cases: Vec<&CaseResult> = results.iter().filter(|r| r.group == group).collect();
    let mut jev = Vec::new();
    for r in &cases {
        let case = corpus.cases.iter().find(|c| c.id == r.id).expect("case");
        for (id, v) in &r.mechanisms {
            if case.mechanisms.iter().any(|m| &m.id == id && m.owner == "jev") {
                jev.push(v.clone());
            }
        }
    }
    let caught = jev.iter().filter(|v| matches!(v, Verdict::CaughtByJev(_))).count();
    let unsupported = jev.iter().filter(|v| **v == Verdict::Unsupported).count();
    let findings: usize = cases.iter().map(|r| r.findings.len()).sum();
    let true_findings: usize = cases
        .iter()
        .map(|r| {
            let named = r.mechanisms.iter().filter(|(_, v)| matches!(v, Verdict::CaughtByJev(_))).count();
            named.min(r.findings.len())
        })
        .sum();
    let clean: Vec<&&CaseResult> = cases.iter().filter(|r| r.label == "clean").collect();
    let flagged = clean.iter().filter(|r| !r.findings.is_empty()).count();
    Metrics {
        group: group.into(),
        recall: Rate::of(caught, jev.len()),
        precision: Rate::of(true_findings, findings),
        clean_flagged: Rate::of(flagged, clean.len()),
        unsupported,
    }
}
