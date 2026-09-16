//! Code finds candidate sentences and numeric spans. The model never does this.

use regex::Regex;
use std::sync::OnceLock;

/// Character budget for one sentence field in the state sent to Jev.
/// A longer sentence is split at sentence boundaries; each part is judged
/// with as much of the original as still fits in `context`.
pub const STATE_SENTENCE_LIMIT: usize = 12_000;

fn unit_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)\d[\d,]*(?:\.\d+)?(?:\s*(?:to|-|–|—)\s*\d[\d,]*(?:\.\d+)?)?\s*(?:ft|feet|foot|in\.|inches|inch|m\b|metres?|meters?|cm|mm|years?|yr|rings/in)",
        )
        .expect("unit regex")
    })
}

fn sentence_end_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"[.!?]+\s+").expect("sentence regex"))
}

/// Visible text from fetched source bytes. Tags are stripped; the checksum
/// stays on the raw bytes.
pub fn visible_text(bytes: &[u8]) -> String {
    let raw = String::from_utf8_lossy(bytes);
    let without_script = strip_blocks(&raw, "script");
    let without_style = strip_blocks(&without_script, "style");
    let mut out = String::with_capacity(without_style.len());
    let mut in_tag = false;
    for ch in without_style.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    decode_entities(&out)
}

fn strip_blocks(input: &str, tag: &str) -> String {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let lower = input.to_ascii_lowercase();
    let mut out = String::new();
    let mut cursor = 0;
    while let Some(start) = lower[cursor..].find(&open) {
        let abs = cursor + start;
        out.push_str(&input[cursor..abs]);
        let after_open = abs + open.len();
        let end = lower[after_open..]
            .find(&close)
            .map(|i| after_open + i + close.len())
            .unwrap_or(input.len());
        cursor = end;
    }
    out.push_str(&input[cursor..]);
    out
}

fn decode_entities(input: &str) -> String {
    input
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateSentence {
    pub sentence: String,
    pub context: String,
}

/// Every sentence that carries a number with a length, age or rate unit.
pub fn candidate_sentences(text: &str) -> Vec<CandidateSentence> {
    let mut found = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for mat in unit_re().find_iter(text) {
        let sentence = enclosing_sentence(text, mat.start(), mat.end());
        let trimmed = collapse_ws(&sentence);
        if trimmed.is_empty() || !seen.insert(trimmed.clone()) {
            continue;
        }
        let context = window(text, mat.start(), mat.end(), 320);
        found.push(CandidateSentence {
            sentence: trimmed,
            context,
        });
    }
    found
}

/// Numeric spans the selection tool presents, plus the coverage of `foot`.
pub fn candidate_spans(text: &str) -> Vec<String> {
    let mut spans = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for mat in unit_re().find_iter(text) {
        let span = collapse_ws(mat.as_str());
        if seen.insert(span.clone()) {
            spans.push(span);
        }
    }
    spans
}

fn enclosing_sentence(text: &str, start: usize, end: usize) -> String {
    let before = text[..start]
        .rfind(['.', '!', '?'])
        .map(|i| i + 1)
        .unwrap_or(0);
    let after = text[end..]
        .find(['.', '!', '?'])
        .map(|i| end + i + 1)
        .unwrap_or(text.len());
    text[before..after].trim().to_string()
}

fn window(text: &str, start: usize, end: usize, radius: usize) -> String {
    let from = start.saturating_sub(radius);
    let to = (end + radius).min(text.len());
    collapse_ws(&text[from..to])
}

fn collapse_ws(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Split a sentence that exceeds the state limit. Each part is judged with
/// the original sentence as `context`, truncated to the remaining budget.
pub fn split_for_state(sentence: &str) -> Vec<(String, String)> {
    if sentence.len() <= STATE_SENTENCE_LIMIT {
        return vec![(sentence.to_string(), sentence.to_string())];
    }
    let mut parts = Vec::new();
    let mut cursor = 0;
    let bytes = sentence.as_bytes();
    while cursor < sentence.len() {
        let remaining = &sentence[cursor..];
        if remaining.len() <= STATE_SENTENCE_LIMIT {
            parts.push(remaining.to_string());
            break;
        }
        let window = &remaining[..STATE_SENTENCE_LIMIT];
        let split_at = sentence_end_re()
            .find_iter(window)
            .last()
            .map(|m| m.end())
            .or_else(|| window.rfind(char::is_whitespace))
            .unwrap_or(STATE_SENTENCE_LIMIT);
        let take = if split_at == 0 {
            STATE_SENTENCE_LIMIT
        } else {
            split_at
        };
        // Keep the split on a char boundary.
        let take = remaining
            .char_indices()
            .map(|(i, _)| i)
            .take_while(|&i| i <= take)
            .last()
            .filter(|&i| i > 0)
            .unwrap_or(take.min(remaining.len()));
        parts.push(remaining[..take].trim().to_string());
        cursor += take;
        while cursor < sentence.len() && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
    }
    parts
        .into_iter()
        .filter(|part| !part.is_empty())
        .map(|part| (part, sentence.to_string()))
        .collect()
}

/// Significant words used to locate a claim inside a source.
pub fn key_terms(claim: &str) -> Vec<String> {
    const STOP: &[&str] = &[
        "the", "and", "for", "that", "with", "from", "this", "into", "over", "under", "about",
        "after", "before", "between", "reaches", "reach", "grows", "grow", "years", "year", "tree",
        "trees",
    ];
    claim
        .split(|ch: char| !ch.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(|word| word.to_ascii_lowercase())
        .filter(|word| {
            if STOP.contains(&word.as_str()) {
                return false;
            }
            word.chars().all(|ch| ch.is_ascii_digit()) || word.len() > 3
        })
        .collect()
}

/// The source window that holds the most of `terms`, or none.
pub fn section_for_terms(source: &str, terms: &[String], radius: usize) -> Option<String> {
    if terms.is_empty() {
        return None;
    }
    let lower = source.to_ascii_lowercase();
    let mut best: Option<(usize, usize)> = None;
    for term in terms {
        let mut from = 0;
        while let Some(at) = lower[from..].find(term) {
            let abs = from + at;
            let score = terms
                .iter()
                .filter(|other| {
                    let window_from = abs.saturating_sub(radius);
                    let window_to = (abs + term.len() + radius).min(lower.len());
                    lower[window_from..window_to].contains(other.as_str())
                })
                .count();
            match best {
                Some((best_score, _)) if score <= best_score => {}
                _ => best = Some((score, abs)),
            }
            from = abs + term.len();
        }
    }
    let (score, at) = best?;
    if score == 0 {
        return None;
    }
    let from = at.saturating_sub(radius);
    let to = (at + radius).min(source.len());
    Some(collapse_ws(&source[from..to]))
}
