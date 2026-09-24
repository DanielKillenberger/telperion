//! Citation check of a spec research section.

use std::path::Path;

use regex::Regex;
use serde_json::json;

use crate::caller::{evaluate, CallerError, EvaluateRequest, HttpRequest, Transport};
use crate::extract::{candidate_spans, collapse_ws, key_terms, section_for_terms};
use crate::html::source_text;
use crate::ledger::SourceRef;
use crate::questions::{citation_questions, thresholds, Thresholds};
use crate::screen::{accumulate, compose_kind};
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
    pub identity: String,
    pub screen_ledger: String,
    pub listed: bool,
    pub reason: String,
    pub kind: Option<String>,
    pub kind_confidence: f64,
    pub anchor_usable: f64,
}

#[derive(Debug, Clone)]
pub struct CiteReport {
    pub rows: Vec<CiteRow>,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub elapsed_ms: u64,
}

/// From a whole spec, the body of `## Resolved via Research` up to the next
/// `## ` heading. A section-only file (no such heading) is returned as-is.
pub fn research_markdown(markdown: &str) -> &str {
    const HEADING: &str = "## Resolved via Research";
    let Some(at) = markdown.find(HEADING) else {
        return markdown;
    };
    let after = &markdown[at + HEADING.len()..];
    let body = after.strip_prefix('\n').unwrap_or(after);
    match body.find("\n## ") {
        Some(end) => &body[..end],
        None => body,
    }
}

fn listed_row(claim: &str, relation: &str, reason: String) -> CiteRow {
    CiteRow {
        claim: claim.to_string(),
        relation: relation.into(),
        confidence: 0.0,
        section: String::new(),
        ledger: String::new(),
        identity: String::new(),
        screen_ledger: String::new(),
        listed: true,
        reason,
        kind: None,
        kind_confidence: 0.0,
        anchor_usable: 0.0,
    }
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
    let cuts = thresholds();
    let mut rows = Vec::new();
    let mut input_tokens = 0;
    let mut output_tokens = 0;
    let mut elapsed_ms = 0;

    for (claim, load) in claims.iter().zip(loads.iter()) {
        match load {
            SourceLoad::Unreachable(err) => {
                rows.push(listed_row(
                    &claim.claim,
                    "unchecked",
                    format!("fetch error: {err}"),
                ));
            }
            SourceLoad::Bytes(bytes) => {
                let text = source_text(bytes);
                let terms = key_terms(&claim.claim);
                let Some(section) = section_for_terms(&text, &terms, 260) else {
                    rows.push(listed_row(
                        &claim.claim,
                        "says_nothing",
                        "key terms match no section".into(),
                    ));
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

                let composed = if carries_number(&claim.claim) {
                    let composed = compose_kind(transport, key, ledger_dir, &source, &section)?;
                    input_tokens += composed.input_tokens;
                    output_tokens += composed.output_tokens;
                    elapsed_ms += composed.elapsed_ms;
                    Some(composed)
                } else {
                    None
                };
                let kind = composed.as_ref().and_then(|c| c.kind.clone());
                let kind_confidence = composed.as_ref().map(|c| c.kind_confidence).unwrap_or(0.0);
                let anchor_usable = composed.as_ref().map(|c| c.anchor_usable).unwrap_or(0.0);
                let screen_ledger = composed
                    .as_ref()
                    .map(|c| c.ledger.clone())
                    .unwrap_or_default();

                let (listed, reason) = list_reason(
                    &relation,
                    confidence,
                    is_verbatim(&claim.claim, &section),
                    kind.as_deref(),
                    kind_confidence,
                    anchor_usable,
                    looks_like_height_at_age(&claim.claim),
                    &cuts,
                );
                rows.push(CiteRow {
                    claim: claim.claim.clone(),
                    relation,
                    confidence,
                    section,
                    ledger: entry.reference(),
                    identity: entry.identity.clone(),
                    screen_ledger,
                    listed,
                    reason,
                    kind,
                    kind_confidence,
                    anchor_usable,
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

/// A claim that carries a digit is numeric and gets the compose screen.
pub fn carries_number(claim: &str) -> bool {
    claim.chars().any(|ch| ch.is_ascii_digit())
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

/// A claim's span (the number and unit as selected) stands, byte for byte
/// after whitespace collapse, in the cited sentence - the claim quotes the
/// source rather than paraphrasing it (fn-137). Whitespace is the only
/// normalisation: no case-folding, no unit conversion.
pub fn is_verbatim(claim: &str, section: &str) -> bool {
    let section = collapse_ws(section);
    candidate_spans(claim)
        .iter()
        .any(|span| section.contains(span.as_str()))
}

pub fn list_reason(
    relation: &str,
    confidence: f64,
    verbatim: bool,
    kind: Option<&str>,
    kind_confidence: f64,
    anchor_usable: f64,
    height_at_age: bool,
    cuts: &Thresholds,
) -> (bool, String) {
    if relation == "unchecked" {
        return (true, "unchecked".into());
    }
    if relation != "supports" {
        return (true, relation.to_string());
    }
    let mut verbatim_saved = false;
    if confidence < cuts.citation_auto_accept {
        if verbatim {
            verbatim_saved = true;
        } else {
            return (
                true,
                format!(
                    "confidence {confidence:.2} below {}",
                    cuts.citation_auto_accept
                ),
            );
        }
    }
    if kind == Some("site_quality_criterion") {
        return (true, "source sentence is a site-quality criterion".into());
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
    if height_at_age && kind == Some("measured_size_at_age") {
        if kind_confidence < cuts.citation_auto_accept {
            return (
                true,
                format!(
                    "screen kind confidence {kind_confidence:.2} below {}",
                    cuts.citation_auto_accept
                ),
            );
        }
        if anchor_usable < cuts.anchor_usable {
            return (
                true,
                format!(
                    "screen anchor {anchor_usable:.2} below {}",
                    cuts.anchor_usable
                ),
            );
        }
    }
    if verbatim_saved {
        (false, "verbatim".into())
    } else {
        (false, "pass".into())
    }
}

pub fn format_report(report: &CiteReport) -> String {
    let mut out = String::new();
    for row in &report.rows {
        let mark = if row.listed { "OWNER" } else { "pass" };
        out.push_str(&format!(
            "{mark}\t{rel}\tconf={conf:.2}\tkind={kind}\tkind_conf={kc:.2}\tanchor={anchor:.2}\tcite={cite}\tscreen={screen}\t{reason}\t{claim}\n",
            rel = row.relation,
            conf = row.confidence,
            kind = row.kind.as_deref().unwrap_or("-"),
            kc = row.kind_confidence,
            anchor = row.anchor_usable,
            cite = row.ledger,
            screen = row.screen_ledger,
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
