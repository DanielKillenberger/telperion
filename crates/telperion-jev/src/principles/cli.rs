//! `jev principles <push|spec|audit|boundary|budget|extract>`.
//!
//! push     the pre-push check: guards block, the reviewer advises
//!          (--no-jev, --no-entry for CI, --json for the full record)
//! spec     spec mode: judges a spec's current proposals, adds no step
//! audit    existing debt: every production stage call and exception
//! boundary the production-boundary guard on a checkout or a revision pair
//! budget   the artifact budgets on a built package
//! extract  the candidates a revision pair hands the reviewer, as JSON
//! eval     the live evaluation: records Jev's answers on the corpus

use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::caller::load_key;

use super::budget::{budgets, check, measure};
use super::candidates::extract;
use super::policy::{exceptions, policy};
use super::push::{absorb, format, guards, Report};
use super::run::{review, Deadline, Settings, DEADLINE};
use super::source::{git, wanted, Snapshot};

fn flag(args: &[String], name: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].clone())
}

fn has(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn repo() -> Result<PathBuf, String> {
    let top = git(Path::new("."), &["rev-parse", "--show-toplevel"])?;
    Ok(PathBuf::from(top.trim()))
}

/// Exit status: 0 clean or advisory, 1 a guard blocked, 2 usage.
pub fn run(args: &[String]) -> Result<i32, String> {
    let Some(cmd) = args.first() else {
        return Err("usage: jev principles <push|spec|audit|boundary|budget|extract>".into());
    };
    let args = &args[1..];
    let repo = repo()?;
    let policy = policy();
    match cmd.as_str() {
        "push" => {
            let started = Instant::now();
            let base = flag(args, "--base").ok_or("missing --base")?;
            let head = flag(args, "--head").unwrap_or_else(|| "HEAD".into());
            let x = cached_extract(&repo, &base, &head, &policy)?;
            let mut report = Report { base: base.clone(), head: head.clone(), ..Report::default() };
            // The reviewer runs first, inside the deadline; the guards'
            // triggered entry tests run after it and are timed on their own.
            if has(args, "--no-jev") {
                report.incomplete.push("reviewer: skipped by --no-jev".into());
            } else if !x.candidates.is_empty() {
                let reviewer = reviewer(&repo, &x, &policy, started);
                absorb(&mut report, &reviewer);
            }
            report.elapsed_ms = started.elapsed().as_millis() as u64;
            guards(&repo, &base, &head, &policy, &x, !has(args, "--no-entry"), &mut report)?;
            log(&repo, &report);
            if has(args, "--json") {
                println!("{}", serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?);
            } else {
                print!("{}", format(&report));
                let total = started.elapsed().as_secs_f64();
                println!("  {} candidate(s); review {:.2} s, all {total:.2} s", x.candidates.len(), report.elapsed_ms as f64 / 1000.0);
            }
            let blocks = !report.blocking.is_empty() || report.findings.iter().any(|f| f.mode == super::review::Mode::Block);
            Ok(i32::from(blocks))
        }
        "spec" => {
            let path = args.first().ok_or("usage: jev principles spec <spec.md>")?;
            let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
            let (x, abstained) = super::spec::extract(path, &text);
            for (at, why) in &abstained {
                println!("  abstain {at}: {why}");
            }
            let reviewer = reviewer(&repo, &x, &policy, Instant::now());
            if let Some(out) = flag(args, "--record") {
                let shapes: Vec<&str> = x.candidates.iter().map(|c| c.shape.as_str()).collect();
                let answers = serde_json::json!({"candidates": shapes, "phase_one": reviewer.phase_one, "phase_two": reviewer.phase_two, "ledger": reviewer.ledger, "calls": reviewer.calls, "input_tokens": reviewer.input_tokens});
                std::fs::write(&out, serde_json::to_string_pretty(&answers).map_err(|e| e.to_string())?).map_err(|e| format!("{out}: {e}"))?;
            }
            let mut report = Report { head: path.clone(), ..Report::default() };
            absorb(&mut report, &reviewer);
            print!("{}", format(&report));
            for f in &report.findings {
                println!("  {:?} [{}] {} {} ({:.2}, {:.2})", f.mode, f.principle, f.location, f.mechanism, f.select, f.confirm);
            }
            Ok(0)
        }
        "audit" => {
            let snap = Snapshot::from_dir(&repo, wanted)?;
            let scan = super::boundary::scan(&snap, &policy.boundary, &exceptions());
            for (c, id) in &scan.excepted {
                println!("excepted {id}: {}:{} {} -> {}", c.file, c.line, c.symbol, c.stage);
            }
            for c in &scan.violations {
                println!("debt: {}:{} {} -> {}", c.file, c.line, c.symbol, c.stage);
            }
            let empty = git(&repo, &["hash-object", "-t", "tree", "/dev/null"])?;
            let x = extract(&repo, empty.trim(), "HEAD", &policy)?;
            for c in &x.candidates {
                println!("candidate {:?} {} {}", c.class, c.location, c.shape);
            }
            println!("{} more candidate(s) past the cap; audit reports, it never blocks", x.dropped);
            Ok(0)
        }
        "boundary" => {
            let (label, scan) = match (flag(args, "--base"), flag(args, "--head")) {
                (Some(base), Some(head)) => {
                    let x = extract(&repo, &base, &head, &policy)?;
                    let added = super::push::boundary_added(&x);
                    for c in &added {
                        println!("added: {}:{} {} -> {}", c.file, c.line, c.symbol, c.stage);
                    }
                    println!("boundary {base}..{head}: {}", if added.is_empty() { "pass" } else { "FAIL" });
                    return Ok(i32::from(!added.is_empty()));
                }
                _ => ("checkout", super::boundary::scan(&Snapshot::from_dir(&repo, wanted)?, &policy.boundary, &exceptions())),
            };
            for c in &scan.violations {
                println!("violation: {}:{} {} -> {}", c.file, c.line, c.symbol, c.stage);
            }
            for u in &scan.unresolved {
                println!("unresolvable: {}:{} {}", u.file, u.line, u.what);
            }
            println!("boundary {label}: {}", if scan.passes() { "pass" } else { "FAIL" });
            Ok(i32::from(!scan.passes()))
        }
        "budget" => {
            let root = flag(args, "--root").map(PathBuf::from).unwrap_or(repo);
            let b = budgets();
            let sizes = measure(&b, &root);
            for (path, size) in &sizes {
                println!("{path}: {size} bytes");
            }
            let fails = check(&b, &sizes);
            for f in &fails {
                println!("FAIL {f}");
            }
            Ok(i32::from(!fails.is_empty()))
        }
        "extract" => {
            let base = flag(args, "--base").ok_or("missing --base")?;
            let head = flag(args, "--head").ok_or("missing --head")?;
            let x = extract(&repo, &base, &head, &policy)?;
            let json = serde_json::to_string_pretty(&x).map_err(|e| e.to_string())?;
            match flag(args, "--out") {
                Some(out) => std::fs::write(&out, json).map_err(|e| format!("{out}: {e}"))?,
                None => println!("{json}"),
            }
            Ok(0)
        }
        "eval" => eval(&repo, args, &policy),
        "report" => {
            use super::replay::{metrics, read, replay_all, Corpus};
            let dir = repo.join("crates/telperion-jev/data/principles");
            let corpus: Corpus = read(&dir.join("corpus.json"))?;
            let results = replay_all(&dir, &policy)?;
            for r in &results {
                let found: Vec<String> = r.findings.iter().map(|f| format!("{} {}", f.principle, f.location)).collect();
                println!("{} {} {} {:?} findings={found:?}{}", r.id, r.group, r.label, r.mechanisms, if r.incomplete { " incomplete" } else { "" });
            }
            for group in ["calibration", "holdout"] {
                let m = metrics(&results, &corpus, group);
                println!("{group}: recall {} (unsupported {}), precision {}, clean pushes flagged {}", m.recall, m.unsupported, m.precision, m.clean_flagged);
            }
            Ok(0)
        }
        other => Err(format!("unknown principles command {other}")),
    }
}

/// The advisory reviewer under the deadline; a missing key or a failed
/// call is an incomplete run, never a clean one.
fn reviewer(repo: &Path, x: &super::candidates::Extraction, policy: &super::policy::Policy, started: Instant) -> super::run::Run {
    if x.candidates.is_empty() {
        return super::run::Run::default();
    }
    let once = std::cell::OnceCell::new();
    let key = || once.get_or_init(|| load_key().ok()).clone();
    let deadline = started + DEADLINE;
    let settings = Settings {
        key: &key,
        ledger: &repo.join(".flow/ledger/jev"),
        cache: Some(&repo.join(".flow/ledger/principles/cache")),
        record_all: false,
        deadline,
        phase_one: None,
    };
    review(&Deadline { until: deadline }, &settings, x, policy, &exceptions())
}

/// Every run's full record, shadow findings included, for measuring
/// precision before a principle leaves shadow mode.
fn log(repo: &Path, report: &Report) {
    let dir = repo.join(".flow/ledger/principles");
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let line = serde_json::to_string(report).unwrap_or_default();
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(dir.join("runs.jsonl")) {
        let _ = writeln!(f, "{line}");
    }
}

/// Calls the live evaluation may spend before it stops and reports.
const EVAL_CALL_CAP: u32 = 200;

/// Asks Jev about every frozen extraction of the corpus in record mode and
/// writes the answers beside it for the offline replay. Stops before a case
/// whose two calls would pass the cap.
fn eval(repo: &Path, args: &[String], policy: &super::policy::Policy) -> Result<i32, String> {
    use super::replay::{read, Corpus};
    let dir = repo.join("crates/telperion-jev/data/principles");
    let corpus: Corpus = read(&dir.join("corpus.json"))?;
    let only = flag(args, "--only");
    let loaded = load_key().map_err(|e| e.to_string())?;
    let key = || Some(loaded.clone());
    let ledger = repo.join(".flow/ledger/jev");
    let (mut calls, mut input, mut output) = (0u32, 0u64, 0u64);
    for case in corpus.cases.iter().filter(|c| only.as_ref().is_none_or(|o| &c.id == o)) {
        let x: super::candidates::Extraction = read(&dir.join(format!("frozen/{}.extraction.json", case.id)))?;
        let out = dir.join(format!("frozen/{}.answers.json", case.id));
        if x.candidates.is_empty() {
            std::fs::write(&out, "{\"phase_one\": {}, \"phase_two\": {}, \"ledger\": [], \"calls\": 0}\n")
                .map_err(|e| format!("{}: {e}", out.display()))?;
            println!("{}: no candidates, no call", case.id);
            continue;
        }
        if calls + 2 > EVAL_CALL_CAP {
            println!("stopped before {}: {calls} calls spent, the cap is {EVAL_CALL_CAP}", case.id);
            break;
        }
        let kept = has(args, "--keep-phase-one")
            .then(|| read::<serde_json::Value>(&out).ok())
            .flatten()
            .map(|a| a["phase_one"].clone());
        let settings = Settings {
            key: &key,
            ledger: &ledger,
            cache: None,
            record_all: true,
            deadline: Instant::now() + std::time::Duration::from_secs(120),
            phase_one: kept,
        };
        let run = review(&super::run::Deadline { until: settings.deadline }, &settings, &x, policy, &exceptions());
        calls += run.calls;
        input += run.input_tokens;
        output += run.output_tokens;
        if let Some(why) = &run.outcome.incomplete {
            println!("{}: incomplete: {why}", case.id);
        }
        let answers = serde_json::json!({
            "phase_one": run.phase_one,
            "phase_two": run.phase_two,
            "ledger": run.ledger,
            "calls": run.calls,
            "input_tokens": run.input_tokens,
            "output_tokens": run.output_tokens,
            "elapsed_ms": run.elapsed_ms,
        });
        std::fs::write(&out, serde_json::to_string_pretty(&answers).map_err(|e| e.to_string())?)
            .map_err(|e| format!("{}: {e}", out.display()))?;
        println!("{}: {} call(s), {} in / {} out tokens, {} ms", case.id, run.calls, run.input_tokens, run.output_tokens, run.elapsed_ms);
    }
    println!("total: {calls} Jev call(s), {input} input and {output} output tokens");
    Ok(0)
}

/// The extraction for a revision pair, cached under both trees and every
/// version it depends on, so a repeated push extracts nothing.
fn cached_extract(repo: &Path, base: &str, head: &str, policy: &super::policy::Policy) -> Result<super::candidates::Extraction, String> {
    let tree = |rev: &str| git(repo, &["rev-parse", &format!("{rev}^{{tree}}")]).map(|t| t.trim().to_string());
    let identity = format!(
        "{}\n{}\n{}\n{}\n{}",
        super::candidates::EXTRACTOR_VERSION,
        policy.version,
        crate::sha256_hex(super::policy::EXCEPTIONS_JSON.as_bytes()),
        tree(base)?,
        tree(head)?
    );
    let path = repo.join(format!(".flow/ledger/principles/cache/x-{}.json", crate::sha256_hex(identity.as_bytes())));
    if let Ok(mut x) = super::replay::read::<super::candidates::Extraction>(&path) {
        x.base = base.into();
        x.head = head.into();
        return Ok(x);
    }
    let x = extract(repo, base, head, policy)?;
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(&path, serde_json::to_vec(&x).unwrap_or_default());
    Ok(x)
}
