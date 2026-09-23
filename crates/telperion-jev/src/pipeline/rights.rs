//! A proposed source's rights, found by code and classified by Jev (fn-129).
//!
//! Code fetches the page and lays out every licence or copyright statement
//! it finds in the page's metadata and text, beside the open-access records
//! it looks up for it: the Europe PMC record for a PMC article, the DOAJ
//! record for a DOI. Jev chooses one class among `open-licence`,
//! `public-cite-only` and `restricted`, or the no-match answer. Only the
//! first two admit; the class is recorded on the source. The statements are
//! short licence lines, never the page's text, and the numbers a source
//! yields are cited, never its prose.

use std::sync::OnceLock;

use regex::Regex;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::caller::{evaluate, CallerError, EvaluateRequest, Transport};
use crate::cases::CaseRow;
use crate::extract::slice_at;
use crate::html::source_text;

use super::adapter::{is_pdf, FetchAdapter};
use super::judge::Judge;
use super::sets::cases::{split, Rows};

pub const RIGHTS_JSON: &str = include_str!("../../data/questions/rights.json");
pub const RIGHTS_CASES: &str = include_str!("../../data/cases/rights.json");

pub const OPEN_LICENCE: &str = "open-licence";
pub const PUBLIC_CITE_ONLY: &str = "public-cite-only";
pub const RESTRICTED: &str = "restricted";
/// The no-match answer: the statements do not settle the rights.
pub const RIGHTS_NONE: &str = "none";

/// Statements kept per page or record, and the characters kept either side
/// of the words that found one: a page's statement is a sentence, a
/// record's is a key and its value.
const MAX_LINES: usize = 10;
const PAGE_RADIUS: usize = 100;
const RECORD_RADIUS: usize = 60;

/// Whether a class admits a source: an open licence or a public page whose
/// numbers are cited. A restricted page and the no-match answer never do.
pub fn admits(class: &str) -> bool {
    class == OPEN_LICENCE || class == PUBLIC_CITE_ONLY
}

fn page_terms() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)creativecommons\.org/(?:licenses|publicdomain)/[a-z0-9./-]*|creative commons|\bcc[ -]by\b|\bcc0\b|open access|public domain|all rights reserved|copyright|©|&copy;|licen[cs]ed under|terms of use|subscribe to (?:read|continue)|purchase (?:this )?(?:article|pdf)|access through|get full-text access|(?:log|sign) ?in to (?:read|view|continue)",
        )
        .expect("licence terms regex")
    })
}

/// A record is a licence answer itself, so its bare `license` key counts,
/// and so does its hit count: a DOI DOAJ does not list is a finding.
fn record_terms() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?i)licen[cs]e|open ?access|"(?:total|hitCount)"\s*:\s*\d+"#)
            .expect("record terms regex")
    })
}

fn meta_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r#"(?i)<(?:link[^>]*rel="license"[^>]*href="([^"]+)"|meta[^>]*(?:name|property)="((?:dc|dcterms)[.:](?:rights|copyright|license)|prism\.copyright|citation_license)"[^>]*content="([^"]*)")"#,
        )
        .expect("licence metadata regex")
    })
}

/// The licence statements in one page: its licence metadata first, then a
/// window around every licence word in its visible text (the markdown when
/// the response is a PDF), overlapping windows merged, at most `MAX_LINES`.
pub fn licence_lines(raw: &[u8], markdown: &str, content_type: &str, url: &str) -> Vec<String> {
    let pdf = is_pdf(content_type, url);
    let mut lines = Vec::new();
    if !pdf {
        let html = String::from_utf8_lossy(raw);
        for cap in meta_re().captures_iter(&html) {
            let line = match (cap.get(1), cap.get(2), cap.get(3)) {
                (Some(href), _, _) => format!("metadata: license link {}", href.as_str()),
                (_, Some(name), Some(value)) => {
                    format!("metadata: {} = {}", name.as_str(), value.as_str())
                }
                _ => continue,
            };
            push_unique(&mut lines, line);
        }
    }
    let text = if pdf || raw.is_empty() {
        markdown.to_string()
    } else {
        source_text(raw)
    };
    for line in windows(&text, page_terms(), PAGE_RADIUS) {
        push_unique(&mut lines, line);
    }
    lines.truncate(MAX_LINES);
    lines
}

/// Windows of whitespace-collapsed `text` around each match of `terms`,
/// overlapping ones merged into the first.
fn windows(text: &str, terms: &Regex, radius: usize) -> Vec<String> {
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut out = Vec::new();
    let mut covered = 0;
    for found in terms.find_iter(&text) {
        if found.start() < covered && !out.is_empty() {
            continue;
        }
        let from = found.start().saturating_sub(radius);
        let to = (found.end() + radius).min(text.len());
        covered = to;
        out.push(
            slice_at(&text, from, to)
                .replace("&copy;", "©")
                .trim()
                .to_string(),
        );
        if out.len() >= MAX_LINES {
            break;
        }
    }
    out
}

fn push_unique(lines: &mut Vec<String>, line: String) {
    if !line.is_empty() && !lines.contains(&line) {
        lines.push(line);
    }
}

/// The host of a URL, lowercased, without a port.
pub fn host(url: &str) -> String {
    url.split("://")
        .nth(1)
        .unwrap_or(url)
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default()
        .split(':')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase()
}

/// The open-access records code looks up for a URL: Europe PMC's record for
/// a PMC id (it carries PubMed Central's licence), the DOAJ article search
/// for a DOI.
pub fn records(url: &str) -> Vec<(&'static str, String)> {
    static PMC: OnceLock<Regex> = OnceLock::new();
    static DOI: OnceLock<Regex> = OnceLock::new();
    let pmc = PMC.get_or_init(|| Regex::new(r"PMC\d+").expect("pmc regex"));
    let doi = DOI.get_or_init(|| Regex::new(r"10\.\d{4,9}/[^\s?#]+").expect("doi regex"));
    let mut out = Vec::new();
    if let Some(id) = pmc.find(url) {
        out.push((
            "europe-pmc",
            format!(
                "https://www.ebi.ac.uk/europepmc/webservices/rest/search?query=PMCID:{}&resultType=core&format=json",
                id.as_str()
            ),
        ));
    } else if let Some(found) = doi.find(url) {
        let encoded = found.as_str().replace('/', "%2F");
        out.push((
            "doaj",
            format!("https://doaj.org/api/search/articles/doi%3A{encoded}"),
        ));
    }
    out
}

/// The state Jev classifies for one proposed source. A fetch that fails is
/// recorded with its error, never retried here: thin evidence is the
/// no-match answer's to take.
pub fn evidence(adapter: &dyn FetchAdapter, url: &str, title: &str) -> Value {
    let mut state = json!({
        "source": {"url": url, "host": host(url), "title": title},
        "lines": [],
        "records": [],
    });
    match adapter.scrape(url) {
        Ok(page) => {
            state["lines"] = json!(licence_lines(
                &page.raw,
                &page.markdown,
                &page.content_type,
                &page.final_url
            ))
        }
        Err(err) => state["error"] = json!(err.to_string()),
    }
    let looked_up: Vec<Value> = records(url)
        .into_iter()
        .map(|(kind, record_url)| match adapter.scrape(&record_url) {
            Ok(record) => {
                let text = String::from_utf8_lossy(&record.raw).into_owned();
                let text = if text.trim().is_empty() {
                    record.markdown
                } else {
                    text
                };
                json!({"kind": kind, "url": record_url, "lines": windows(&text, record_terms(), RECORD_RADIUS)})
            }
            Err(err) => json!({"kind": kind, "url": record_url, "error": err.to_string()}),
        })
        .collect();
    state["records"] = json!(looked_up);
    state
}

/// The rights Choice: the three classes and the no-match answer.
pub fn rights_questions() -> Value {
    let raw: Value = serde_json::from_str(RIGHTS_JSON).expect("rights.json");
    json!({"rights": raw["rights"]})
}

pub fn set_version() -> u32 {
    let raw: Value = serde_json::from_str(RIGHTS_JSON).expect("rights.json");
    raw["version"].as_u64().unwrap_or(0) as u32
}

/// One classification: the class Jev chose (the no-match answer when it
/// chose nothing), its confidence and the ledger reference.
#[derive(Debug, Clone, PartialEq)]
pub struct Classified {
    pub class: String,
    pub confidence: f64,
    pub ledger: String,
}

pub fn classify(judge: &Judge<'_>, state: &Value) -> Result<Classified, CallerError> {
    let judgment = judge.ask("rights", None, state, &rights_questions())?;
    Ok(Classified {
        class: known_class(judgment.entry.choice("rights")),
        confidence: judgment.entry.confidence("rights").unwrap_or(0.0),
        ledger: judgment.reference,
    })
}

/// A choice outside the table reads as the no-match answer.
fn known_class(choice: Option<String>) -> String {
    match choice.as_deref() {
        Some(c @ (OPEN_LICENCE | PUBLIC_CITE_ONLY | RESTRICTED)) => c.to_string(),
        _ => RIGHTS_NONE.into(),
    }
}

/// A labelled case: the statements code laid out for one page, and the
/// class a person admitted it under.
#[derive(Debug, Clone, Deserialize)]
pub struct RightsCase {
    pub id: String,
    pub url: String,
    pub title: String,
    pub lines: Vec<String>,
    #[serde(default)]
    pub records: Vec<Value>,
    #[serde(default)]
    pub error: Option<String>,
    pub expect_class: String,
    pub holdout: bool,
    pub negative: bool,
}

pub fn rights_cases() -> Vec<RightsCase> {
    serde_json::from_str(RIGHTS_CASES).expect("rights cases")
}

/// The state `evidence` would have laid out for the case's page.
pub fn rights_state(case: &RightsCase) -> Value {
    let mut state = json!({
        "source": {"url": case.url, "host": host(&case.url), "title": case.title},
        "lines": case.lines,
        "records": case.records,
    });
    if let Some(error) = &case.error {
        state["error"] = json!(error);
    }
    state
}

/// Asks every labelled case and scores the class over the labelled and the
/// held-out cases, against the accuracy bar.
pub fn run_cases(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &std::path::Path,
) -> Result<Rows, CallerError> {
    let questions = rights_questions();
    let mut rows = Rows::new();
    for case in rights_cases() {
        let state = rights_state(&case);
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: "rights",
                source: None,
                state: &state,
                questions: &questions,
                ledger_dir,
            },
        )?;
        let answered = known_class(entry.choice("rights"));
        rows.push((
            CaseRow {
                set: "rights class".into(),
                id: case.id.clone(),
                expected: case.expect_class.clone(),
                hit: answered == case.expect_class,
                answered,
                top_probability: entry.top_probability("rights"),
                confidence: entry.confidence("rights").unwrap_or(0.0),
                ledger: entry.reference(),
            },
            case.holdout,
        ));
    }
    Ok(rows)
}

/// The rights set scored as two sets, labelled and held out.
pub fn scored(rows: Rows) -> Vec<crate::cases::SetScore> {
    split(
        "rights class",
        rows,
        crate::questions::thresholds().accuracy_bar,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_and_text_statements_are_found_and_page_prose_is_not() {
        let raw = br#"<html><head><link rel="license" href="https://creativecommons.org/licenses/by-sa/4.0/"><meta name="dc.rights" content="2022 The Author(s)"></head><body><p>Date palms reach 30 m.</p><footer>&copy; 2025 The Arizona Board of Regents</footer></body></html>"#;
        let lines = licence_lines(raw, "", "text/html", "https://example.test/p");
        assert_eq!(
            lines[..2],
            [
                "metadata: license link https://creativecommons.org/licenses/by-sa/4.0/",
                "metadata: dc.rights = 2022 The Author(s)",
            ]
        );
        assert!(
            lines[2].contains("© 2025 The Arizona Board of Regents"),
            "{lines:?}"
        );
        assert_eq!(lines.len(), 3, "{lines:?}");
    }

    #[test]
    fn a_pmc_article_and_a_doi_each_name_their_open_access_record() {
        let table = [
            (
                "https://pmc.ncbi.nlm.nih.gov/articles/PMC9511727/",
                Some("https://www.ebi.ac.uk/europepmc/webservices/rest/search?query=PMCID:PMC9511727&resultType=core&format=json"),
            ),
            (
                "https://doi.org/10.1186/s12870-022-03841-0",
                Some("https://doaj.org/api/search/articles/doi%3A10.1186%2Fs12870-022-03841-0"),
            ),
            ("https://ask.ifas.ufl.edu/publication/FR314", None),
        ];
        for (url, expect) in table {
            let found = records(url).into_iter().next().map(|(_, u)| u);
            assert_eq!(found.as_deref(), expect, "{url}");
        }
        assert_eq!(host("https://Ask.IFAS.ufl.edu:443/x"), "ask.ifas.ufl.edu");
    }

    #[test]
    fn only_an_open_licence_or_a_public_page_admits() {
        assert!(admits(OPEN_LICENCE) && admits(PUBLIC_CITE_ONLY));
        assert!(!admits(RESTRICTED) && !admits(RIGHTS_NONE));
        assert_eq!(known_class(Some("public domain".into())), RIGHTS_NONE);
        assert_eq!(known_class(None), RIGHTS_NONE);
    }
}
