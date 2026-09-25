//! Spec mode: each current proposal in a spec, with its decision context,
//! becomes a candidate. Descriptions of what exists, measurements, unknowns,
//! quotations, rejected designs and the designs a `Superseded` entry names
//! are left out. A proposal with no decision context abstains. Spec mode
//! advises before ready; it adds no approval step.

use super::candidates::{bound, Candidate, Class, Evidence, Extraction};

const PROPOSAL_SECTIONS: &[&str] = &["Architecture", "API Contracts", "Edge Cases"];
const CONTEXT_SECTIONS: &[&str] = &["Decision Context", "Boundaries"];
const PROPOSAL_CHARS: usize = 1_600;
const CONTEXT_CHARS: usize = 700;
const STOPWORDS: &[&str] = &["the", "and", "after", "with", "from", "first", "second", "third", "fourth", "owner", "host", "decision", "verdict"];

#[derive(Debug, Clone, PartialEq)]
pub struct Proposal {
    pub line: usize,
    pub title: String,
    pub text: String,
    /// The bullet carries its own decision: a `[decision]` tag or a host or
    /// owner decision in its lead.
    pub decided: bool,
}

/// Candidates from a spec's markdown, and the proposals that abstained for
/// want of decision context.
pub fn extract(path: &str, markdown: &str, policy: &super::policy::Policy) -> (Extraction, Vec<(String, String)>) {
    let sections = sections(markdown);
    let context: String = sections
        .iter()
        .filter(|(h, ..)| CONTEXT_SECTIONS.iter().any(|c| h.starts_with(c)))
        .map(|(_, _, body)| body.trim().to_string())
        .collect::<Vec<_>>()
        .join("\n");
    let mut found = Vec::new();
    let mut abstained = Vec::new();
    for p in current(&sections) {
        if !p.decided && context.trim().is_empty() {
            abstained.push((format!("{path}:{}", p.line), "no decision context".into()));
            continue;
        }
        let mut evidence = vec![excerpt(path, "proposal", p.line, &p.text, PROPOSAL_CHARS)];
        if !context.is_empty() {
            evidence.push(excerpt(path, "decision context", 0, &context, CONTEXT_CHARS));
        }
        found.push(Candidate {
            id: String::new(),
            class: Class::Proposal,
            location: format!("{path}:{}", p.line),
            shape: format!("proposal: {}", p.title),
            weight: 1.0,
            evidence,
        });
    }
    let (candidates, dropped) = bound(found, policy);
    let x = Extraction { extractor: super::candidates::EXTRACTOR_VERSION.into(), candidates, dropped, ..Extraction::default() };
    (x, abstained)
}

fn excerpt(path: &str, side: &str, line: usize, text: &str, cap: usize) -> Evidence {
    let text: String = text.chars().take(cap).collect();
    Evidence { id: String::new(), side: side.into(), path: path.into(), lines: (line, line), text, added: true }
}

/// `(heading, first line, body)` for every `## ` section.
fn sections(markdown: &str) -> Vec<(String, usize, String)> {
    let mut out: Vec<(String, usize, String)> = Vec::new();
    for (i, line) in markdown.lines().enumerate() {
        if let Some(h) = line.strip_prefix("## ") {
            out.push((h.trim().to_string(), i + 1, String::new()));
        } else if let Some(last) = out.last_mut() {
            last.2.push_str(line);
            last.2.push('\n');
        }
    }
    out
}

/// The proposals a spec currently makes, in order.
pub fn current(sections: &[(String, usize, String)]) -> Vec<Proposal> {
    let mut all = Vec::new();
    for (heading, start, body) in sections {
        if !PROPOSAL_SECTIONS.iter().any(|s| heading.starts_with(s)) {
            continue;
        }
        let mut para: Option<Proposal> = None;
        for (i, line) in body.lines().enumerate() {
            let at = start + 1 + i;
            let t = line.trim_start();
            let starts = t.starts_with("- ") || t.starts_with("* ") || (!t.is_empty() && para.is_none());
            if t.is_empty() || t.starts_with("<!--") || t.starts_with('>') {
                all.extend(para.take());
                continue;
            }
            if starts && !line.starts_with("  ") {
                all.extend(para.take());
                para = Some(Proposal { line: at, title: title(t), text: String::new(), decided: false });
            }
            if let Some(p) = para.as_mut() {
                p.text.push_str(t);
                p.text.push(' ');
            }
        }
        all.extend(para);
    }
    let superseded: Vec<String> = all
        .iter()
        .filter(|p| lead(&p.text).starts_with("superseded"))
        .map(|p| p.text.to_lowercase())
        .collect();
    all.into_iter()
        .filter(|p| !historical(p) && !superseded.iter().any(|s| names(s, p)))
        .map(|mut p| {
            let l = lead(&p.text);
            p.decided = p.text.contains("[decision]") || l.contains("decision");
            p
        })
        .collect()
}

/// A bullet's text without its `- ` or `* ` marker.
fn bullet(text: &str) -> &str {
    let t = text.trim_start();
    t.strip_prefix("- ").or_else(|| t.strip_prefix("* ")).unwrap_or(t)
}

/// The bold lead of a bullet, lower-cased: `host decision (…): the ribbon`.
fn lead(text: &str) -> String {
    bullet(text)
        .strip_prefix("**")
        .and_then(|rest| rest.split_once("**"))
        .map(|(l, _)| l.to_lowercase())
        .unwrap_or_default()
}

fn title(line: &str) -> String {
    let l = lead(line);
    if l.is_empty() {
        bullet(line).chars().take(80).collect()
    } else {
        l.trim_end_matches('.').to_string()
    }
}

/// What exists, what was measured, what is unknown, what was rejected, and
/// the `Superseded` entry itself: none of it is a proposal.
fn historical(p: &Proposal) -> bool {
    let l = lead(&p.text);
    let t = bullet(&p.text).to_lowercase();
    ["what exists", "measured", "unknown", "superseded", "rejected", "checked"]
        .iter()
        .any(|w| l.starts_with(w))
        || t.starts_with("rejected")
        || (p.text.contains("[checked]") && !p.text.contains("[decision]"))
}

/// True when the `Superseded` text names this proposal: every word of the
/// part of its title after the colon appears in it.
fn names(superseded: &str, p: &Proposal) -> bool {
    let l = lead(&p.text);
    let Some((_, subject)) = l.split_once(':') else { return false };
    let words: Vec<&str> = subject
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 2 && !STOPWORDS.contains(w))
        .collect();
    let stems = |w: &str| w.trim_end_matches('s').to_string();
    !words.is_empty() && words.iter().all(|w| superseded.contains(&stems(w)))
}
