//! Code finds candidate sentences and numeric spans. The model never does this.

use regex::Regex;
use std::sync::OnceLock;

/// Byte budget for one sentence field in the state sent to Jev.
/// A longer sentence is split at sentence boundaries; each part is judged
/// with a neighbourhood of the original that still fits in `context`.
pub const STATE_SENTENCE_LIMIT: usize = 12_000;

/// Greatest char boundary at or before `index`.
pub fn floor_char_boundary(text: &str, index: usize) -> usize {
    if index >= text.len() {
        return text.len();
    }
    let mut i = index;
    while i > 0 && !text.is_char_boundary(i) {
        i -= 1;
    }
    i
}

/// Least char boundary at or after `index`.
pub fn ceil_char_boundary(text: &str, index: usize) -> usize {
    if index >= text.len() {
        return text.len();
    }
    let mut i = index;
    while i < text.len() && !text.is_char_boundary(i) {
        i += 1;
    }
    i
}

/// Slice `text[from..to]` after clamping both ends onto char boundaries.
pub fn slice_at(text: &str, from: usize, to: usize) -> &str {
    let from = floor_char_boundary(text, from);
    let to = ceil_char_boundary(text, to.max(from));
    &text[from..to]
}

/// Truncate to the state limit on a char boundary.
pub fn bound_state_text(text: &str) -> String {
    if text.len() <= STATE_SENTENCE_LIMIT {
        text.to_string()
    } else {
        slice_at(text, 0, STATE_SENTENCE_LIMIT).to_string()
    }
}

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
    let start = floor_char_boundary(text, start);
    let end = ceil_char_boundary(text, end);
    let before = text[..start]
        .rfind(['.', '!', '?'])
        .map(|i| i + 1)
        .unwrap_or(0);
    let after = text[end..]
        .find(['.', '!', '?'])
        .map(|i| end + i + 1)
        .unwrap_or(text.len());
    slice_at(text, before, after).trim().to_string()
}

fn window(text: &str, start: usize, end: usize, radius: usize) -> String {
    let from = start.saturating_sub(radius);
    let to = end.saturating_add(radius).min(text.len());
    collapse_ws(slice_at(text, from, to))
}

fn collapse_ws(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Split a sentence that exceeds the state limit. Each part is judged with
/// a bounded neighbourhood of the original sentence as `context`.
pub fn split_for_state(sentence: &str) -> Vec<(String, String)> {
    if sentence.len() <= STATE_SENTENCE_LIMIT {
        return vec![(sentence.to_string(), sentence.to_string())];
    }
    let mut out = Vec::new();
    let mut cursor = 0;
    while cursor < sentence.len() {
        cursor = ceil_char_boundary(sentence, cursor);
        if cursor >= sentence.len() {
            break;
        }
        let remaining = sentence.len() - cursor;
        let cap = floor_char_boundary(sentence, cursor + STATE_SENTENCE_LIMIT);
        let end = if remaining <= STATE_SENTENCE_LIMIT || cap <= cursor {
            sentence.len()
        } else {
            let window = &sentence[cursor..cap];
            let rel = sentence_end_re()
                .find_iter(window)
                .last()
                .map(|m| m.end())
                .or_else(|| window.rfind(char::is_whitespace))
                .unwrap_or(window.len());
            let abs = floor_char_boundary(sentence, cursor + rel);
            if abs <= cursor {
                cap
            } else {
                abs.min(cap)
            }
        };
        let part = sentence[cursor..end].trim().to_string();
        if !part.is_empty() {
            let context = neighborhood(sentence, cursor, end, STATE_SENTENCE_LIMIT);
            out.push((part, context));
        }
        cursor = end;
        while cursor < sentence.len() && sentence.as_bytes()[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
    }
    out
}

/// Neighbourhood of `[start, end)` inside `whole`, at most `limit` bytes,
/// clamped to char boundaries.
pub fn neighborhood(whole: &str, start: usize, end: usize, limit: usize) -> String {
    let start = floor_char_boundary(whole, start);
    let end = ceil_char_boundary(whole, end).max(start);
    if whole.len() <= limit {
        return whole.to_string();
    }
    let part_len = end - start;
    if part_len >= limit {
        return bound_state_text(&whole[start..end]);
    }
    let extra = limit - part_len;
    let left = extra / 2;
    let right = extra - left;
    let from = floor_char_boundary(whole, start.saturating_sub(left));
    let mut to = ceil_char_boundary(whole, end.saturating_add(right).min(whole.len()));
    while to > from && to - from > limit {
        to = floor_char_boundary(whole, to.saturating_sub(1));
    }
    if to <= from {
        return bound_state_text(slice_at(whole, start, start.saturating_add(limit)));
    }
    whole[from..to].to_string()
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
                    let window_from = floor_char_boundary(&lower, abs.saturating_sub(radius));
                    let window_to =
                        ceil_char_boundary(&lower, (abs + term.len() + radius).min(lower.len()));
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
    let to = at.saturating_add(radius).min(source.len());
    Some(collapse_ws(slice_at(source, from, to)))
}
