//! A push, checked: the three deterministic guards block, the reviewer
//! advises. Guards read the pushed revisions, never the working tree, except
//! entry coverage, which runs its tests only when the checkout is the pushed
//! head and reports incomplete otherwise.

use std::path::Path;
use std::process::Command;
use std::time::Instant;

use serde::Serialize;

use super::boundary::{added, Caller, Scan};
use super::budget::{change_defects, parse, Budgets};
use super::candidates::Extraction;
use super::policy::Policy;
use super::review::{Finding, Mode};
use super::run::Run;
use super::source::git;

const BUDGETS_PATH: &str = "crates/telperion-jev/data/principles/budgets.json";
pub const SHOWN: usize = 3;

#[derive(Debug, Default, Serialize)]
pub struct Report {
    pub base: String,
    pub head: String,
    /// Guard failures: each blocks the push.
    pub blocking: Vec<String>,
    /// Guards that could not run here, and why.
    pub incomplete: Vec<String>,
    pub notes: Vec<String>,
    pub findings: Vec<Finding>,
    pub calls: u32,
    pub input_tokens: u64,
    pub elapsed_ms: u64,
}

/// Every path of the change a trigger list names, by prefix.
pub fn triggered<'a>(paths: impl IntoIterator<Item = &'a String>, prefixes: &[String]) -> Vec<String> {
    paths
        .into_iter()
        .filter(|p| prefixes.iter().any(|t| p.starts_with(t.as_str())))
        .cloned()
        .collect()
}

/// `run_entries` false leaves entry coverage to the workspace tests, as CI's
/// crate jobs run them.
pub fn guards(repo: &Path, base: &str, head: &str, policy: &Policy, x: &Extraction, run_entries: bool, report: &mut Report) -> Result<(), String> {
    for c in boundary_added(x) {
        report.blocking.push(format!(
            "boundary: {}:{} `{}` calls the build stage `{}` outside the pipeline, with no exception covering it. Route it through `pipeline::build`, or cite an exception id from crates/telperion-jev/data/principles/exceptions.json.",
            c.file, c.line, c.symbol, c.stage
        ));
    }
    let before = git(repo, &["show", &format!("{base}:{BUDGETS_PATH}")]).ok().and_then(|t| parse(&t).ok());
    match git(repo, &["show", &format!("{head}:{BUDGETS_PATH}")]) {
        Ok(text) => {
            let after: Budgets = parse(&text)?;
            for d in change_defects(before.as_ref(), &after) {
                report.blocking.push(format!("budget: {d}"));
            }
        }
        Err(_) => report.notes.push("budgets: the head carries no budget file".into()),
    }
    let entry = triggered(&x.paths, &policy.entry_triggers);
    if !entry.is_empty() && run_entries {
        entry_coverage(repo, head, policy, &entry, report)?;
    } else if !entry.is_empty() {
        report.notes.push(format!("entry coverage: triggered by {}; the crate jobs run it", entry[0]));
    }
    if !triggered(&x.paths, &policy.artifact_triggers).is_empty() {
        report.notes.push("artifact sizes: triggered; the fixed recipe (npm run build) is measured by CI's package job with `jev principles budget`".into());
    }
    Ok(())
}

/// Callers the head adds over the base, and every unresolvable import at
/// the head, from the extraction's frozen scans.
pub fn boundary_added(x: &Extraction) -> Vec<Caller> {
    let scan_of = |v: &Vec<Caller>| Scan { violations: v.clone(), ..Scan::default() };
    let mut out = added(&scan_of(&x.boundary_base), &scan_of(&x.boundary_head));
    for u in &x.boundary_unresolved {
        out.push(Caller { file: u.clone(), line: 0, symbol: "unresolvable import".into(), stage: "(unresolvable)".into() });
    }
    out
}

fn entry_coverage(repo: &Path, head: &str, policy: &Policy, why: &[String], report: &mut Report) -> Result<(), String> {
    let at = git(repo, &["rev-parse", "HEAD"])?;
    let dirty = !git(repo, &["status", "--porcelain", "--untracked-files=no"])?.trim().is_empty();
    let head = git(repo, &["rev-parse", head])?;
    if at.trim() != head.trim() || dirty {
        report.incomplete.push(format!(
            "entry coverage: triggered by {}, but the checkout is not the pushed head; CI runs it",
            why[0]
        ));
        return Ok(());
    }
    for e in policy.entries.iter().filter(|e| e.role == "generation") {
        let Some(test) = &e.guard else { continue };
        let started = Instant::now();
        let mut args = test.split_whitespace();
        let program = args.next().unwrap_or("cargo");
        let status = Command::new(program)
            .current_dir(repo)
            .args(args)
            .output()
            .map_err(|err| format!("{test}: {err}"))?;
        let secs = started.elapsed().as_secs_f64();
        if status.status.success() {
            report.notes.push(format!("entry coverage `{}`: pass in {secs:.1} s", e.export));
        } else {
            let tail: String = String::from_utf8_lossy(&status.stdout)
                .lines()
                .filter(|l| l.contains("entry ") || l.contains("panicked"))
                .take(3)
                .collect::<Vec<_>>()
                .join(" | ");
            report.blocking.push(format!("entry coverage `{}`: {test} failed: {tail}", e.export));
        }
    }
    Ok(())
}

pub fn absorb(report: &mut Report, run: &Run) {
    report.findings = run.outcome.findings.clone();
    report.calls = run.calls;
    report.input_tokens = run.input_tokens;
    if let Some(why) = &run.outcome.incomplete {
        report.incomplete.push(format!("reviewer: {why}"));
    }
}

/// At most three shown findings, deduplicated by location and principle.
pub fn format(report: &Report) -> String {
    let mut out = format!("principles: {} against {}\n", short(&report.head), short(&report.base));
    for b in &report.blocking {
        out.push_str(&format!("  BLOCK {b}\n"));
    }
    for n in &report.notes {
        out.push_str(&format!("  {n}\n"));
    }
    let mut seen = std::collections::BTreeSet::new();
    let shown: Vec<&Finding> = report
        .findings
        .iter()
        .filter(|f| f.mode != Mode::Shadow)
        .filter(|f| seen.insert((f.location.clone(), f.principle.clone())))
        .collect();
    let shadow = report.findings.iter().filter(|f| f.mode == Mode::Shadow).count();
    out.push_str(&format!(
        "  reviewer: {} warning(s), {shadow} logged in shadow, {} Jev call(s), {} input tokens\n",
        shown.len(),
        report.calls,
        report.input_tokens
    ));
    for f in shown.iter().take(SHOWN) {
        out.push_str(&format!(
            "  WARN [{}] {} {}: {}. Evidence: {}. Consequence: {}. Repair: {}\n",
            f.principle,
            f.location,
            f.mechanism,
            super::ask::mechanisms(f.class).iter().find(|m| m.0 == f.mechanism).map_or("", |m| m.2),
            f.evidence.join(", "),
            consequence(&f.mechanism),
            "rework it inside the one pipeline, or cite a registered exception id; `jev principles push --json` prints the full record"
        ));
    }
    for i in &report.incomplete {
        out.push_str(&format!("  INCOMPLETE {i}\n"));
    }
    out
}

fn consequence(mechanism: &str) -> &'static str {
    match mechanism {
        "selects_builder" | "suppresses_structure" => "a small change in the setting makes a jump in the tree",
        "surviving_duplicate" => "two copies drift, and a fix lands in one of them",
        "fallback_path" => "an input the pipeline cannot draw ships on a path nobody judges",
        "redundant_stop" => "work halts for a defect another step already catches",
        _ => "the output costs build time and memory no consumer repays",
    }
}

fn short(rev: &str) -> &str {
    &rev[..rev.len().min(8)]
}
