//! A push, checked by the three deterministic guards, each of which blocks.
//! The boundary and the budget file are read at the pushed revisions, never
//! the working tree; entry coverage runs its tests only when the checkout is
//! the pushed head, and reports incomplete otherwise.

use std::path::Path;
use std::process::Command;
use std::time::Instant;

use serde::Serialize;

use super::boundary::{added, scan, Caller, Scan};
use super::budget::{change_defects, parse, Budgets};
use super::policy::{exceptions, Policy};
use super::source::{git, wanted, Snapshot};

const BUDGETS_PATH: &str = "crates/telperion-jev/data/principles/budgets.json";

#[derive(Debug, Default, Serialize)]
pub struct Report {
    pub base: String,
    pub head: String,
    /// Guard failures: each blocks the push.
    pub blocking: Vec<String>,
    /// Guards that could not run here, and why.
    pub incomplete: Vec<String>,
    pub notes: Vec<String>,
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

/// Runs the guards on `base..head`. `run_entries` false leaves entry
/// coverage to the workspace tests, as CI's crate jobs run them.
pub fn guards(repo: &Path, base: &str, head: &str, policy: &Policy, run_entries: bool) -> Result<Report, String> {
    let started = Instant::now();
    let mut report = Report { base: base.into(), head: head.into(), ..Report::default() };
    for c in boundary_added(repo, base, head, policy)? {
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
    let paths: Vec<String> = git(repo, &["diff", "--name-only", "--no-renames", base, head])?
        .lines()
        .map(str::to_string)
        .collect();
    let entry = triggered(&paths, &policy.entry_triggers);
    if !entry.is_empty() && run_entries {
        entry_coverage(repo, head, policy, &entry, &mut report)?;
    } else if !entry.is_empty() {
        report.notes.push(format!("entry coverage: triggered by {}; the crate jobs run it", entry[0]));
    }
    if !triggered(&paths, &policy.artifact_triggers).is_empty() {
        report.notes.push("artifact sizes: triggered; CI's package job measures the fixed recipe (npm run build) with `jev principles budget`".into());
    }
    report.elapsed_ms = started.elapsed().as_millis() as u64;
    Ok(report)
}

/// Callers `head` adds over `base`, and every import at the head the guard
/// cannot follow. The two revisions are scanned on their own threads.
pub fn boundary_added(repo: &Path, base: &str, head: &str, policy: &Policy) -> Result<Vec<Caller>, String> {
    let ex = exceptions();
    let side = |rev: &str| -> Result<Scan, String> {
        Ok(scan(&Snapshot::from_git(repo, rev, wanted)?, &policy.boundary, &ex))
    };
    let (b, h) = std::thread::scope(|s| {
        let b = s.spawn(|| side(base));
        let h = side(head);
        (b.join().expect("base scan"), h)
    });
    let (b, h) = (b?, h?);
    let mut out = added(&b, &h);
    for u in &h.unresolved {
        out.push(Caller { file: u.file.clone(), line: u.line, symbol: u.what.clone(), stage: "(unresolvable)".into() });
    }
    for e in &h.parse_errors {
        out.push(Caller { file: e.path.clone(), line: 0, symbol: e.message.clone(), stage: "(does not parse)".into() });
    }
    Ok(out)
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
        let out = Command::new(program)
            .current_dir(repo)
            .args(args)
            .output()
            .map_err(|err| format!("{test}: {err}"))?;
        let secs = started.elapsed().as_secs_f64();
        if out.status.success() {
            report.notes.push(format!("entry coverage `{}`: pass in {secs:.1} s", e.export));
        } else {
            let tail: String = String::from_utf8_lossy(&out.stdout)
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

pub fn format(report: &Report) -> String {
    let short = |rev: &str| rev[..rev.len().min(8)].to_string();
    let mut out = format!("principles: {} against {}\n", short(&report.head), short(&report.base));
    for b in &report.blocking {
        out.push_str(&format!("  BLOCK {b}\n"));
    }
    for n in &report.notes {
        out.push_str(&format!("  {n}\n"));
    }
    for i in &report.incomplete {
        out.push_str(&format!("  INCOMPLETE {i}\n"));
    }
    if report.blocking.is_empty() {
        out.push_str("  guards: pass\n");
    }
    out
}
