//! Citation check of a spec research section.

use std::path::Path;

use regex::Regex;
use serde_json::json;

use crate::caller::{evaluate, CallerError, EvaluateRequest, HttpRequest, Transport};
use crate::extract::{key_terms, section_for_terms, visible_text};
use crate::ledger::SourceRef;
use crate::questions::{citation_questions, screen_questions, thresholds};
use crate::screen::accumulate;
use crate::sha256_hex;

#[derive(Debug, Clone)]
pub struct ResearchClaim {
    pub claim: String,
    pub url: String,
    pub source_id: String,
    pub unresolved: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SourceLoad {
    Bytes(Vec<u8>),
    Unreachable(String),
}

#[derive(Debug, Clone)]
pub struct CiteRow {
    pub claim: String,
    pub relation: String,
    pub confidence: f64,
    pub section: String,
    pub ledger: String,
    pub listed: bool,
    pub reason: String,
    pub kind: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CiteReport {
    pub rows: Vec<CiteRow>,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub elapsed_ms: u64,
}

/// Parse research-section bullets. A URL is fetched; "same paper" / "same
/// source" reuses the previous URL; any other bullet is kept and listed as
/// unchecked when its source cannot be fetched.
pub fn parse_research(markdown: &str) -> Vec<ResearchClaim> {
    let url_re = Regex::new(r"https?://\S+").expect("url regex");
    let mut claims = Vec::new();
    let mut last_url = String::new();
    let mut last_id = String::new();
    for raw in markdown.lines() {
        let line = raw.trim();
        if !line.starts_with('-') {
            continue;
        }
        let source_tail = line
            .rfind("Source:")
            .map(|idx| line[idx + 7..].trim().trim_end_matches(['.', ',', ';']))
            .unwrap_or("");
        let url_in_line = url_re.find(line).map(|m| {
            m.as_str()
                .trim_end_matches([')', ']', '.', ',', '"'])
                .to_string()
        });
        let (url, unresolved) = match url_in_line {
            Some(url) => {
                last_url = url.clone();
                last_id = url
                    .rsplit('/')
                    .find(|part| !part.is_empty())
                    .unwrap_or("source")
                    .to_string();
                (url, None)
            }
            None if is_same_source(source_tail) && !last_url.is_empty() => (last_url.clone(), None),
            None if !source_tail.is_empty() => (
                String::new(),
                Some(format!("no fetchable source: {source_tail}")),
            ),
            None => (String::new(), Some("no fetchable source".to_string())),
        };
        let mut claim = line.trim_start_matches('-').trim().to_string();
        if let Some(idx) = claim.rfind("Source:") {
            claim.truncate(idx);
        } else if let Some(url) = url_re.find(&claim) {
            claim.truncate(url.start());
        }
        let claim = claim
            .trim()
            .trim_end_matches(['.', '—', '-', ' '])
            .to_string();
        if claim.is_empty() {
            continue;
        }
        let source_id = if url.is_empty() {
            "unresolved".into()
        } else if !last_id.is_empty() {
            last_id.clone()
        } else {
            "source".into()
        };
        claims.push(ResearchClaim {
            claim,
            url,
            source_id,
            unresolved,
        });
    }
    claims
}

fn is_same_source(tail: &str) -> bool {
    let lower = tail.to_ascii_lowercase();
    lower.contains("same paper") || lower.contains("same source")
}

/// Fetch or read a parsed bullet. An empty URL is unreachable with the reason.
pub fn load_claim_source(transport: &dyn Transport, claim: &ResearchClaim) -> SourceLoad {
    if claim.url.is_empty() {
        return SourceLoad::Unreachable(
            claim
                .unresolved
                .clone()
                .unwrap_or_else(|| "no fetchable source".into()),
        );
    }
    load_source(transport, &claim.url)
}

pub fn load_source(transport: &dyn Transport, url: &str) -> SourceLoad {
    if let Some(path) = url.strip_prefix("file:") {
        return match std::fs::read(path) {
            Ok(bytes) => SourceLoad::Bytes(bytes),
            Err(err) => SourceLoad::Unreachable(err.to_string()),
        };
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return match std::fs::read(url) {
            Ok(bytes) => SourceLoad::Bytes(bytes),
            Err(err) => SourceLoad::Unreachable(err.to_string()),
        };
    }
    let request = HttpRequest {
        method: "GET",
        url: url.to_string(),
        headers: Vec::new(),
        body: None,
    };
    match transport.send(&request) {
        Ok(resp) if resp.status < 400 => SourceLoad::Bytes(resp.body),
        Ok(resp) => SourceLoad::Unreachable(format!("HTTP {}", resp.status)),
        Err(err) => SourceLoad::Unreachable(err),
    }
}

pub fn cite(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
    claims: &[ResearchClaim],
    loads: &[SourceLoad],
) -> Result<CiteReport, CallerError> {
    let questions = citation_questions();
    let screen_q = screen_questions();
    let cuts = thresholds();
    let mut rows = Vec::new();
    let mut input_tokens = 0;
    let mut output_tokens = 0;
    let mut elapsed_ms = 0;

    for (claim, load) in claims.iter().zip(loads.iter()) {
        match load {
            SourceLoad::Unreachable(err) => {
                rows.push(CiteRow {
                    claim: claim.claim.clone(),
                    relation: "unchecked".into(),
                    confidence: 0.0,
                    section: String::new(),
                    ledger: String::new(),
                    listed: true,
                    reason: format!("fetch error: {err}"),
                    kind: None,
                });
            }
            SourceLoad::Bytes(bytes) => {
                let text = visible_text(bytes);
                let terms = key_terms(&claim.claim);
                let Some(section) = section_for_terms(&text, &terms, 260) else {
                    rows.push(CiteRow {
                        claim: claim.claim.clone(),
                        relation: "says_nothing".into(),
                        confidence: 0.0,
                        section: String::new(),
                        ledger: String::new(),
                        listed: true,
                        reason: "key terms match no section".into(),
                        kind: None,
                    });
                    continue;
                };
                let source = SourceRef {
                    id: claim.source_id.clone(),
                    url: claim.url.clone(),
                    sha256: sha256_hex(bytes),
                    bytes: bytes.len() as u64,
                };
                let state = json!({
                    "source": {"id": source.id, "url": source.url},
                    "claim": claim.claim,
                    "section": section,
                });
                let entry = evaluate(
                    transport,
                    key,
                    EvaluateRequest {
                        tool: "cite",
                        source: Some(&source),
                        state: &state,
                        questions: &questions,
                        ledger_dir,
                    },
                )?;
                accumulate(
                    &entry,
                    &mut input_tokens,
                    &mut output_tokens,
                    &mut elapsed_ms,
                );
                let relation = entry
                    .choice("relation")
                    .unwrap_or_else(|| "says_nothing".into());
                let confidence = entry.confidence("relation").unwrap_or(0.0);

                let mut kind = None;
                if looks_like_height_at_age(&claim.claim) {
                    let screen_state = json!({
                        "species": "",
                        "source": {"id": source.id, "url": source.url},
                        "candidate": {"sentence": section, "context": section},
                    });
                    let screen_entry = evaluate(
                        transport,
                        key,
                        EvaluateRequest {
                            tool: "screen",
                            source: Some(&source),
                            state: &screen_state,
                            questions: &screen_q,
                            ledger_dir,
                        },
                    )?;
                    accumulate(
                        &screen_entry,
                        &mut input_tokens,
                        &mut output_tokens,
                        &mut elapsed_ms,
                    );
                    kind = screen_entry.choice("kind");
                }

                let (listed, reason) = list_reason(
                    &relation,
                    confidence,
                    kind.as_deref(),
                    looks_like_height_at_age(&claim.claim),
                    cuts.citation_auto_accept,
                );
                rows.push(CiteRow {
                    claim: claim.claim.clone(),
                    relation,
                    confidence,
                    section,
                    ledger: entry.reference(),
                    listed,
                    reason,
                    kind,
                });
            }
        }
    }

    Ok(CiteReport {
        rows,
        input_tokens,
        output_tokens,
        elapsed_ms,
    })
}

pub fn looks_like_height_at_age(claim: &str) -> bool {
    let lower = claim.to_ascii_lowercase();
    if lower.contains("per year")
        || lower.contains("/year")
        || lower.contains(" a year")
        || lower.contains("sprout")
        || lower.contains("cultivar")
        || lower.contains("nursery")
    {
        return false;
    }
    let has_length = [" m ", " m.", "ft", "foot", "feet", "metre", "meter"]
        .iter()
        .any(|u| lower.contains(u));
    let has_age = ["year", "age", " at ten", " at 10"]
        .iter()
        .any(|u| lower.contains(u));
    has_length && has_age
}

pub fn list_reason(
    relation: &str,
    confidence: f64,
    kind: Option<&str>,
    height_at_age: bool,
    threshold: f64,
) -> (bool, String) {
    if relation == "unchecked" {
        return (true, "unchecked".into());
    }
    if relation != "supports" {
        return (true, relation.to_string());
    }
    if confidence < threshold {
        return (
            true,
            format!("confidence {confidence:.2} below {threshold}"),
        );
    }
    if height_at_age && kind != Some("measured_size_at_age") {
        return (
            true,
            format!(
                "numeric claim but screen kind is {}",
                kind.unwrap_or("missing")
            ),
        );
    }
    (false, "pass".into())
}

pub fn format_report(report: &CiteReport) -> String {
    let mut out = String::new();
    for row in &report.rows {
        let mark = if row.listed { "OWNER" } else { "pass" };
        out.push_str(&format!(
            "{mark}\t{rel}\tconf={conf:.2}\t{ledger}\t{reason}\t{claim}\n",
            rel = row.relation,
            conf = row.confidence,
            ledger = row.ledger,
            reason = row.reason,
            claim = row.claim
        ));
        if !row.section.is_empty() {
            out.push_str(&format!("  section: {}\n", row.section));
        }
    }
    out.push_str(&format!(
        "tokens in={} out={} wall_ms={}\n",
        report.input_tokens, report.output_tokens, report.elapsed_ms
    ));
    out
}
