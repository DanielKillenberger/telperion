//! `jev principles <push|boundary|budget>`: the deterministic guards.
//!
//! push     the pre-push check (--base B --head H; --no-entry for CI, --json)
//! boundary the production boundary on the checkout, excepted callers listed
//! budget   the artifact budgets on a built package (--root, default the repo)

use std::path::{Path, PathBuf};

use super::boundary::scan;
use super::budget::{budgets, check, measure};
use super::policy::{exceptions, policy};
use super::push::{format, guards};
use super::source::{git, wanted, Snapshot};

fn flag(args: &[String], name: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].clone())
}

fn has(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

/// Exit status: 0 pass, 1 a guard blocked, 2 usage.
pub fn run(args: &[String]) -> Result<i32, String> {
    let Some(cmd) = args.first() else {
        return Err("usage: jev principles <push|boundary|budget>".into());
    };
    let args = &args[1..];
    let repo = PathBuf::from(git(Path::new("."), &["rev-parse", "--show-toplevel"])?.trim());
    let policy = policy();
    match cmd.as_str() {
        "push" => {
            let base = flag(args, "--base").ok_or("missing --base")?;
            let head = flag(args, "--head").unwrap_or_else(|| "HEAD".into());
            let report = guards(&repo, &base, &head, &policy, !has(args, "--no-entry"))?;
            if has(args, "--json") {
                println!("{}", serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?);
            } else {
                print!("{}", format(&report));
            }
            Ok(i32::from(!report.blocking.is_empty()))
        }
        "boundary" => {
            let scan = scan(&Snapshot::from_dir(&repo, wanted)?, &policy.boundary, &exceptions());
            for (c, id) in &scan.excepted {
                println!("excepted {id}: {}:{} {} -> {}", c.file, c.line, c.symbol, c.stage);
            }
            for c in &scan.violations {
                println!("violation: {}:{} {} -> {}", c.file, c.line, c.symbol, c.stage);
            }
            for u in &scan.unresolved {
                println!("unresolvable: {}:{} {}", u.file, u.line, u.what);
            }
            println!("boundary: {}", if scan.passes() { "pass" } else { "FAIL" });
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
        other => Err(format!("unknown principles command {other}")),
    }
}
