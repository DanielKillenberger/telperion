//! The run's measurements (R7): elapsed time, interruptions, dispatches by
//! role, tier and effort with their tokens and cost where known, Jev calls
//! and usage, captures, retries, wrong routes, failed fixes and waiting.
//! Expensive reasoning is attributed to design, implementation, visual
//! assessment or review so the allocation can be judged against what was
//! verified. Unknown costs stay unknown; failed attempts stay in the totals;
//! no saving is claimed, because nothing here compares two runs.
use std::collections::BTreeMap;

use serde_json::{json, Value};

use super::dispatch::Outcome;
use super::state::Run;
use super::{now, Config, Result};
use crate::ledger::LedgerEntry;
use crate::pipeline::canon::write_canonical;

/// Seconds since the epoch of an RFC 3339 UTC timestamp, or none.
pub fn epoch_seconds(stamp: &str) -> Option<i64> {
    let (date, time) = stamp.split_once('T')?;
    let mut d = date.split('-').map(|p| p.parse::<i64>().ok());
    let (y, m, day) = (d.next()??, d.next()??, d.next()??);
    let time = time.trim_end_matches('Z');
    let time = time.split(['+', '-']).next()?;
    let mut t = time
        .split(':')
        .map(|p| p.split('.').next()?.parse::<i64>().ok());
    let (h, min, s) = (t.next()??, t.next()??, t.next()??);
    // Days from civil, Howard Hinnant's algorithm.
    let (y, m) = if m <= 2 { (y - 1, m + 9) } else { (y, m - 3) };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * m + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Some(days * 86_400 + h * 3_600 + min * 60 + s)
}

fn seconds_between(from: &str, to: &str) -> Option<i64> {
    Some(epoch_seconds(to)? - epoch_seconds(from)?)
}

fn ledger_entries(config: &Config) -> Vec<LedgerEntry> {
    let Ok(entries) = std::fs::read_dir(config.ledger_dir()) else {
        return Vec::new();
    };
    let mut out: Vec<LedgerEntry> = entries
        .flatten()
        .filter_map(|e| std::fs::read(e.path()).ok())
        .filter_map(|bytes| serde_json::from_slice(&bytes).ok())
        .collect();
    out.sort_by(|a, b| a.recorded_at.cmp(&b.recorded_at));
    out
}

fn jev(config: &Config) -> Value {
    let entries = ledger_entries(config);
    let mut by_tool: BTreeMap<String, (u64, u64, u64, u64)> = BTreeMap::new();
    for e in &entries {
        let slot = by_tool.entry(e.tool.clone()).or_default();
        slot.0 += 1;
        match &e.usage {
            Some(u) => {
                slot.1 += u.input_tokens;
                slot.2 += u.output_tokens;
            }
            None => slot.3 += 1,
        }
    }
    json!({
        "calls": entries.len(),
        "by_tool": by_tool.iter().map(|(tool, (calls, input, output, unknown))| json!({
            "tool": tool, "calls": calls, "input_tokens": input, "output_tokens": output,
            "usage_unknown_calls": unknown,
        })).collect::<Vec<_>>(),
        "credits": "unknown: the caller records tokens, not credits",
    })
}

fn dispatches(run: &Run) -> Value {
    let mut groups: BTreeMap<String, Value> = BTreeMap::new();
    for d in &run.dispatches {
        let key = format!("{}/{}/{}", d.role.key(), d.tier, d.effort);
        let g = groups.entry(key.clone()).or_insert_with(|| json!({
            "role": d.role.key(), "tier": d.tier, "effort": d.effort,
            "count": 0, "verified": 0, "failed": 0, "obsolete": 0, "interrupted": 0, "open": 0,
            "input_tokens": 0, "output_tokens": 0, "tokens_unknown": 0, "cost_usd": 0.0, "cost_unknown": 0,
            "wall_ms": 0, "wall_unknown": 0, "actual_models": [],
        }));
        let bump = |g: &mut Value, key: &str| {
            g[key] = json!(g[key].as_u64().unwrap_or(0) + 1);
        };
        bump(g, "count");
        match d.result.as_ref() {
            None => bump(g, "open"),
            Some(r) => {
                match r.outcome {
                    Some(Outcome::Verified) => bump(g, "verified"),
                    Some(Outcome::Failed) => bump(g, "failed"),
                    Some(Outcome::Obsolete) => bump(g, "obsolete"),
                    Some(Outcome::Interrupted) | None => bump(g, "interrupted"),
                }
                match &r.usage {
                    Some(u) => {
                        g["input_tokens"] =
                            json!(g["input_tokens"].as_u64().unwrap_or(0) + u.input_tokens);
                        g["output_tokens"] =
                            json!(g["output_tokens"].as_u64().unwrap_or(0) + u.output_tokens);
                    }
                    None => bump(g, "tokens_unknown"),
                }
                match r.cost_usd {
                    Some(c) => g["cost_usd"] = json!(g["cost_usd"].as_f64().unwrap_or(0.0) + c),
                    None => bump(g, "cost_unknown"),
                }
                match r.wall_ms {
                    Some(w) => g["wall_ms"] = json!(g["wall_ms"].as_u64().unwrap_or(0) + w),
                    None => bump(g, "wall_unknown"),
                }
                let model = format!("{} at {}", r.actual_model, r.actual_effort);
                if let Some(list) = g["actual_models"].as_array_mut() {
                    if !list.iter().any(|m| m == &json!(model)) {
                        list.push(json!(model));
                    }
                }
            }
        }
    }
    json!(groups.into_values().collect::<Vec<_>>())
}

/// Retries: a dispatch that repeats the route of a failed one on the same
/// dependency. Wrong routes: a route whose result failed or was obsolete.
fn retries_and_wrong_routes(run: &Run) -> (usize, usize, usize) {
    let mut retries = 0;
    let mut wrong = 0;
    let mut failed_fixes = 0;
    for (i, d) in run.dispatches.iter().enumerate() {
        if run.dispatches[..i].iter().any(|p| {
            p.dependency == d.dependency
                && p.route == d.route
                && p.outcome() == Some(Outcome::Failed)
        }) {
            retries += 1;
        }
        match d.outcome() {
            Some(Outcome::Failed) | Some(Outcome::Obsolete) => {
                wrong += 1;
                if d.role == super::dispatch::Role::Implement {
                    failed_fixes += 1;
                }
            }
            _ => {}
        }
    }
    (retries, wrong, failed_fixes)
}

pub fn compute(config: &Config, run: &Run) -> Value {
    let end = now();
    let elapsed = seconds_between(&run.started_at, &end);
    let waiting: i64 = run
        .waits
        .iter()
        .filter_map(|w| seconds_between(&w.from, w.to.as_deref().unwrap_or(&end)))
        .sum();
    let (retries, wrong_routes, failed_fixes) = retries_and_wrong_routes(run);
    let captures: usize = run
        .tuning
        .iter()
        .filter_map(|t| super::gapcheck::read_result(&t.out).ok())
        .map(|r| r.outcome.budget["images"].as_u64().unwrap_or(0) as usize)
        .sum();
    let escalations = run
        .waits
        .iter()
        .filter(|w| w.reason.starts_with("human:"))
        .count();
    json!({
        "schema": "conductor-report",
        "schema_version": super::SCHEMA_VERSION,
        "species": config.species,
        "spec": config.spec,
        "elapsed_seconds": elapsed,
        "elapsed_note": "wall time from the first start to this report; service durations below are not wall time",
        "waiting_seconds": waiting,
        "interruptions": run.resumes,
        "dispatches": dispatches(run),
        "attribution_note": "expensive reasoning is attributed by role: design, implement, visual, review; routine and investigate are the cheap conductor's",
        "jev": jev(config),
        "tuning_revisions": run.tuning.iter().map(|t| json!({
            "revision": t.revision, "tokens": t.tokens, "machine_ready": t.machine_ready, "gaps": t.gaps,
        })).collect::<Vec<_>>(),
        "captures": captures,
        "retries": retries,
        "wrong_routes": wrong_routes,
        "failed_fixes": failed_fixes,
        "human_escalations": escalations,
        "autonomous_escalations_note": "an escalation the conductor raised counts as autonomous; it is not an owner intervention",
        "tokens_total_known": run.budget.tokens,
        "usage_known": run.budget.usage_known,
        "costs_note": "cost in currency is summed only where a result reported it; unknown costs are counted, never estimated",
        "comparison": Value::Null,
        "comparison_note": "no saving is claimed: nothing here compares this run with another",
        "routes": run.routes,
        "at": end,
    })
}

pub fn write(config: &Config, run: &Run) -> Result<Value> {
    let report = compute(config, run);
    write_canonical(&config.report_file(), &report)?;
    std::fs::write(config.report_file().with_extension("md"), markdown(&report))
        .map_err(|err| super::ConductorError::Invalid(err.to_string()))?;
    Ok(report)
}

pub fn markdown(r: &Value) -> String {
    let mut s = format!(
        "# Conductor report: {}\n\n",
        r["species"].as_str().unwrap_or_default()
    );
    s.push_str("| | |\n| --- | --- |\n");
    for (label, key) in [
        ("Elapsed (s)", "elapsed_seconds"),
        ("Waiting (s)", "waiting_seconds"),
        ("Interruptions", "interruptions"),
        ("Captures", "captures"),
        ("Retries", "retries"),
        ("Wrong routes", "wrong_routes"),
        ("Failed fixes", "failed_fixes"),
        ("Human escalations", "human_escalations"),
        ("Tokens known", "tokens_total_known"),
        ("Usage known", "usage_known"),
    ] {
        s.push_str(&format!("| {label} | {} |\n", r[key]));
    }
    s.push_str("\n## Dispatches by role, tier and effort\n\n| role | tier | effort | n | verified | failed | in | out | unknown tokens | cost | models |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for d in r["dispatches"].as_array().into_iter().flatten() {
        s.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            d["role"].as_str().unwrap_or_default(),
            d["tier"].as_str().unwrap_or_default(),
            d["effort"].as_str().unwrap_or_default(),
            d["count"],
            d["verified"],
            d["failed"],
            d["input_tokens"],
            d["output_tokens"],
            d["tokens_unknown"],
            if d["cost_unknown"].as_u64().unwrap_or(0) > 0 {
                "partly unknown".to_string()
            } else {
                d["cost_usd"].to_string()
            },
            d["actual_models"]
                .as_array()
                .map(|l| l
                    .iter()
                    .filter_map(Value::as_str)
                    .collect::<Vec<_>>()
                    .join(", "))
                .unwrap_or_default(),
        ));
    }
    s.push_str(&format!(
        "\n## Jev\n\n{} calls; credits {}.\n",
        r["jev"]["calls"],
        r["jev"]["credits"].as_str().unwrap_or_default()
    ));
    for t in r["jev"]["by_tool"].as_array().into_iter().flatten() {
        s.push_str(&format!(
            "- {}: {} calls, {} in, {} out, {} with unknown usage\n",
            t["tool"].as_str().unwrap_or_default(),
            t["calls"],
            t["input_tokens"],
            t["output_tokens"],
            t["usage_unknown_calls"]
        ));
    }
    s.push_str(&format!(
        "\n{}\n\n{}\n",
        r["costs_note"].as_str().unwrap_or_default(),
        r["comparison_note"].as_str().unwrap_or_default()
    ));
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc3339_stamps_subtract_to_seconds() {
        assert_eq!(epoch_seconds("1970-01-01T00:00:00Z"), Some(0));
        assert_eq!(epoch_seconds("2026-09-22T10:00:00Z"), Some(1_790_071_200));
        assert_eq!(
            seconds_between("2026-09-22T10:00:00Z", "2026-09-22T10:01:30.5Z"),
            Some(90)
        );
        assert_eq!(epoch_seconds("not a stamp"), None);
    }
}
