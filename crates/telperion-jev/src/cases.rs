//! Live labelled-case runner. Workspace tests never call this against the network.

use std::path::Path;

use serde_json::json;

use crate::caller::{evaluate, CallerError, EvaluateRequest, Transport};
use crate::cite::{cite, ResearchClaim, SourceLoad};
use crate::questions::{
    screen_cases, screen_questions, selection_cases, severity_level, thresholds, triage_cases,
};
use crate::select::select;
use crate::triage::triage;

#[derive(Debug, Clone)]
pub struct CaseRow {
    pub set: String,
    pub id: String,
    pub expected: String,
    pub answered: String,
    pub top_probability: f64,
    pub confidence: f64,
    pub ledger: String,
    pub hit: bool,
}

#[derive(Debug, Clone)]
pub struct SetScore {
    pub name: String,
    pub hits: usize,
    pub total: usize,
    pub required: usize,
    pub ranking_ok: Option<bool>,
    pub confidences: Vec<f64>,
    pub rows: Vec<CaseRow>,
}

impl SetScore {
    pub fn meets_pilot(&self) -> bool {
        let score_ok = self.hits >= self.required;
        match self.ranking_ok {
            Some(ok) => score_ok && ok,
            None => score_ok,
        }
    }
}

pub fn run_labelled_cases(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<Vec<SetScore>, CallerError> {
    Ok(vec![
        run_screen(transport, key, ledger_dir)?,
        run_select(transport, key, ledger_dir)?,
        run_cite(transport, key, ledger_dir)?,
        run_routes(transport, key, ledger_dir)?,
        run_duplicates(transport, key, ledger_dir)?,
        run_severity(transport, key, ledger_dir)?,
    ])
}

pub fn format_scores(sets: &[SetScore]) -> String {
    let mut out = String::new();
    for set in sets {
        out.push_str(&format!("## {}\n", set.name));
        out.push_str("id\texpected\tanswered\ttop_p\tconf\tledger\thit\n");
        for row in &set.rows {
            out.push_str(&format!(
                "{id}\t{exp}\t{ans}\t{p:.2}\t{c:.2}\t{led}\t{hit}\n",
                id = row.id,
                exp = row.expected,
                ans = row.answered,
                p = row.top_probability,
                c = row.confidence,
                led = row.ledger,
                hit = row.hit
            ));
        }
        let (min, med, max) = spread(&set.confidences);
        out.push_str(&format!(
            "accuracy {}/{} (pilot {})  conf min={min:.2} median={med:.2} max={max:.2}\n",
            set.hits, set.total, set.required
        ));
        if let Some(ok) = set.ranking_ok {
            out.push_str(&format!("true pairs above every negative: {ok}\n"));
        }
        out.push('\n');
    }
    out
}

fn run_screen(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<SetScore, CallerError> {
    let questions = screen_questions();
    let mut rows = Vec::new();
    for case in screen_cases() {
        let state = json!({
            "species": case.source_id,
            "candidate": {
                "sentence": case.sentence,
                "context": case.sentence,
            }
        });
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: "screen",
                source: None,
                state: &state,
                questions: &questions,
                ledger_dir,
            },
        )?;
        let answered = entry.choice("kind").unwrap_or_else(|| "none".into());
        rows.push(CaseRow {
            set: "screen".into(),
            id: case.id,
            expected: case.expect_kind.clone(),
            hit: answered == case.expect_kind,
            answered,
            top_probability: entry.top_probability("kind"),
            confidence: entry.confidence("kind").unwrap_or(0.0),
            ledger: entry.reference(),
        });
    }
    Ok(score_set("screen kind", 12, rows, None))
}

fn run_select(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<SetScore, CallerError> {
    let mut rows = Vec::new();
    for case in selection_cases() {
        let report = select(
            transport,
            key,
            ledger_dir,
            &case.document,
            &case.question,
            None,
        )?;
        let answered = if report.contract_failure {
            "<rejected>".into()
        } else {
            report.chosen.clone()
        };
        rows.push(CaseRow {
            set: "select".into(),
            id: case.id,
            expected: case.expect_span.clone(),
            hit: !report.contract_failure && answered == case.expect_span,
            answered,
            top_probability: report
                .probabilities
                .as_object()
                .and_then(|map| {
                    map.values()
                        .filter_map(|v| v.as_f64())
                        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                })
                .unwrap_or(0.0),
            confidence: report.confidence,
            ledger: report.ledger,
        });
    }
    Ok(score_set("select", 5, rows, None))
}

fn run_cite(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<SetScore, CallerError> {
    let cases = crate::questions::citation_cases();
    let claims: Vec<ResearchClaim> = cases
        .iter()
        .map(|case| ResearchClaim {
            claim: case.claim.clone(),
            url: format!("file:{}", case.id),
            source_id: case.id.clone(),
            unresolved: None,
        })
        .collect();
    let loads: Vec<SourceLoad> = cases
        .iter()
        .map(|case| SourceLoad::Bytes(case.section.as_bytes().to_vec()))
        .collect();
    let report = cite(transport, key, ledger_dir, &claims, &loads)?;
    let mut rows = Vec::new();
    let mut pilot_hits = 0;
    for (case, row) in cases.iter().zip(report.rows.iter()) {
        let expected = if case.true_claim { "pass" } else { "OWNER" };
        let answered = if row.listed { "OWNER" } else { "pass" };
        let hit = expected == answered;
        if case.true_claim && !row.listed {
            pilot_hits += 1;
        }
        if case.id == "o1-misuse" && row.listed {
            pilot_hits += 1;
        }
        rows.push(CaseRow {
            set: "cite".into(),
            id: case.id.clone(),
            expected: expected.into(),
            answered: format!("{answered} {} {}", row.relation, row.reason),
            top_probability: row.confidence,
            confidence: row.confidence,
            ledger: row.ledger.clone(),
            hit,
        });
    }
    let mut set = score_set("cite compose", 7, rows, None);
    set.hits = pilot_hits;
    set.required = 7;
    Ok(set)
}

fn run_routes(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<SetScore, CallerError> {
    let cases = triage_cases();
    let mut rows = Vec::new();
    for route in &cases.routes {
        let proposal = triage(
            transport,
            key,
            ledger_dir,
            &route.observation,
            &cases.open_specs,
            &[],
            "",
        )?;
        let top = proposal
            .spec_probabilities
            .get(&proposal.spec)
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        rows.push(CaseRow {
            set: "route".into(),
            id: route.id.clone(),
            expected: route.expect.join("|"),
            hit: route.expect.contains(&proposal.spec),
            answered: proposal.spec,
            top_probability: top,
            confidence: top,
            ledger: proposal.ledger.first().cloned().unwrap_or_default(),
        });
    }
    Ok(score_set("triage route", 8, rows, None))
}

fn run_duplicates(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<SetScore, CallerError> {
    let cases = triage_cases();
    let cut = thresholds().duplicate_same_defect;
    let mut rows = Vec::new();
    let mut true_ps = Vec::new();
    let mut false_ps = Vec::new();
    for pair in &cases.duplicates {
        let proposal = triage(
            transport,
            key,
            ledger_dir,
            &pair.new_observation,
            &cases.open_specs,
            std::slice::from_ref(&pair.prior_finding),
            "",
        )?;
        let p = proposal.same_defect.unwrap_or(0.0);
        if pair.true_pair {
            true_ps.push(p);
        } else {
            false_ps.push(p);
        }
        rows.push(CaseRow {
            set: "duplicate".into(),
            id: pair.id.clone(),
            expected: if pair.true_pair {
                "true".into()
            } else {
                "false".into()
            },
            answered: format!("{p:.2}"),
            top_probability: p,
            confidence: p,
            ledger: proposal.ledger.last().cloned().unwrap_or_default(),
            hit: if pair.true_pair { p >= cut } else { p < cut },
        });
    }
    let ranking_ok = !true_ps.is_empty()
        && !false_ps.is_empty()
        && true_ps.iter().all(|t| false_ps.iter().all(|f| t > f));
    let mut set = score_set("triage duplicate", 2, rows, Some(ranking_ok));
    set.hits = if ranking_ok { 2 } else { 0 };
    Ok(set)
}

fn run_severity(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<SetScore, CallerError> {
    let cases = triage_cases();
    let mut rows = Vec::new();
    for sev in &cases.severity {
        let proposal = triage(
            transport,
            key,
            ledger_dir,
            &sev.observation,
            &cases.open_specs,
            &[],
            &sev.standard,
        )?;
        let answered = proposal
            .severity_level
            .clone()
            .unwrap_or_else(|| "none".into());
        let score = proposal.severity.unwrap_or(0.0);
        rows.push(CaseRow {
            set: "severity".into(),
            id: sev.id.clone(),
            expected: sev.expect_level.clone(),
            hit: answered == sev.expect_level,
            answered: format!("{answered} {score:.2}"),
            top_probability: score,
            confidence: score,
            ledger: proposal.ledger.last().cloned().unwrap_or_default(),
        });
        let _ = severity_level(score);
    }
    Ok(score_set("triage severity", 4, rows, None))
}

fn score_set(
    name: &str,
    required: usize,
    rows: Vec<CaseRow>,
    ranking_ok: Option<bool>,
) -> SetScore {
    let hits = rows.iter().filter(|row| row.hit).count();
    let confidences = rows.iter().map(|row| row.confidence).collect();
    SetScore {
        name: name.into(),
        hits,
        total: rows.len(),
        required,
        ranking_ok,
        confidences,
        rows,
    }
}

fn spread(values: &[f64]) -> (f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];
    let mid = sorted.len() / 2;
    let med = if sorted.len().is_multiple_of(2) {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    };
    (min, med, max)
}
