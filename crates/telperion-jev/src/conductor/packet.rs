//! The ready-for-review packet (R6): the preset, the matched views, the
//! checklist results, the sources and article, the unresolved limitations
//! and the actual costs. It is marked ready only when every automated
//! prerequisite holds; a missing view, an outstanding defect, an unavailable
//! visual judge or unchecked documentation keeps the species unready, and
//! the packet names which. Only the owner accepts.
use serde_json::{json, Value};

use super::gapcheck::{self, PASSING};
use super::state::Run;
use super::{now, Config, Result};
use crate::pipeline::canon::{read_json, write_canonical};
use crate::pipeline::decision::{read_decisions, Status};

pub fn read(config: &Config) -> Result<Option<Value>> {
    let path = config.packet_file();
    if !path.exists() {
        return Ok(None);
    }
    Ok(Some(read_json(&path)?))
}

fn sources(config: &Config) -> Vec<String> {
    let dir = config.dir.join("sources");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .flatten()
        .map(|e| e.path().display().to_string())
        .filter(|p| p.ends_with(".md"))
        .collect();
    names.sort();
    names
}

/// Assembles the packet from the artifacts and the latest tuning result and
/// writes it. Returns one line saying ready, or what keeps it unready.
pub fn assemble(config: &Config, run: &mut Run) -> Result<String> {
    let paths = config.paths();
    let mut unready: Vec<String> = Vec::new();
    let report = paths.artifact("report");
    let numeric = report.exists()
        && read_json(&report)
            .ok()
            .is_some_and(|r| r["body"]["status"] == "complete");
    if !numeric {
        unready.push("numeric validation: report.json is missing or not complete".into());
    }
    if !paths.dir.join("metrics.json").exists() {
        unready.push("numeric validation: metrics.json is not written".into());
    }
    let latest = run.latest_tuning().cloned();
    let (visual, views, checklist, identity, preset) = match &latest {
        Some(t) => match gapcheck::read_result(&t.out) {
            Ok(result) => {
                let views: Vec<Value> = result
                    .outcome
                    .current
                    .as_ref()
                    .map(|c| {
                        c.stills
                            .iter()
                            .map(|s| json!({"view": s.view, "seed": s.seed, "sha256": s.sha256,
                                            "path": s.path, "present": std::path::Path::new(&s.path).exists()}))
                            .collect()
                    })
                    .unwrap_or_default();
                let checklist: Vec<Value> = result
                    .gaps
                    .iter()
                    .map(|g| json!({"id": g.id, "priority": g.priority, "status": g.status}))
                    .collect();
                let visual = result.outcome.machine_ready && !result.outcome.bootstrap;
                if !visual {
                    unready.push(if result.outcome.bootstrap {
                        "visual readiness: the run was a bootstrap; the reviewer has not been shown to pass an owner-accepted tree".into()
                    } else {
                        format!("visual readiness: the automated result is not ready ({})", result.outcome.stopped)
                    });
                }
                for g in result.gaps.iter().filter(|g| g.status != PASSING) {
                    unready.push(format!(
                        "checklist defect outstanding: {} ({})",
                        g.id, g.status
                    ));
                }
                (
                    visual,
                    views,
                    checklist,
                    result.outcome.run_identity.clone(),
                    result.outcome.preset.clone(),
                )
            }
            Err(err) => {
                unready.push(format!(
                    "visual readiness: the tuning result is unreadable ({err})"
                ));
                (false, Vec::new(), Vec::new(), String::new(), String::new())
            }
        },
        None => {
            unready.push("visual readiness: no tuning revision has run; the visual judge never assessed the species".into());
            (false, Vec::new(), Vec::new(), String::new(), String::new())
        }
    };
    if views.is_empty() {
        unready.push("matched views: none recorded".into());
    } else if let Some(missing) = views.iter().find(|v| v["present"] == false) {
        unready.push(format!(
            "matched views: still missing on disk: {}",
            missing["path"]
        ));
    }
    let article = config.dir.join("ARTICLE.md");
    if !article.exists() || !paths.artifact("document").exists() {
        unready.push("documentation: ARTICLE.md or document.json is missing".into());
    }
    if paths.decisions().exists() {
        for d in read_decisions(&paths.decisions())?
            .iter()
            .filter(|d| d.status == Status::Open)
        {
            unready.push(format!("open decision: {} ({})", d.id, d.kind));
        }
    }
    let ready = unready.is_empty() && numeric && visual;
    let packet = json!({
        "schema": "conductor-packet",
        "schema_version": super::SCHEMA_VERSION,
        "species": config.species,
        "spec": config.spec,
        "preset": preset,
        "tuning_identity": identity,
        "matched_views": views,
        "checklist": checklist,
        "sources": sources(config),
        "article": article.display().to_string(),
        "unresolved_limitations": unready,
        "costs": run.budget,
        "dispatches": run.dispatches.len(),
        "ready_for_owner_review": ready,
        "owner_acceptance": "pending: only the owner's verdict accepts",
        "at": now(),
    });
    write_canonical(&config.packet_file(), &packet)?;
    Ok(if ready {
        format!(
            "packet ready for the owner's review at {}",
            config.packet_file().display()
        )
    } else {
        format!(
            "packet withheld: {}",
            packet["unresolved_limitations"]
                .as_array()
                .map(|l| l
                    .iter()
                    .filter_map(Value::as_str)
                    .collect::<Vec<_>>()
                    .join("; "))
                .unwrap_or_default()
        )
    })
}
