//! A recording that republishes no third-party page (owner, 2026-09-25).
//!
//! A recorded page whose source is not openly licensed keeps only what the
//! run quoted from it: the passages that reached a Jev request (candidate
//! sentences with their context, verified excerpts, licence statements),
//! each whole, in page order, and nothing else. Its bytes keep only the
//! licence tags and statements the rights check reads. Every later request
//! of a replay is then the recorded one, so every key still answers; the
//! trim checks the candidate sentences and the licence lines itself and
//! refuses a page it would change. `species` records with `--record`; this
//! runs over the recording before it is committed (`tape_trim`).
use std::collections::BTreeSet;
use std::path::Path;

use serde_json::Value;

use crate::extract::{candidate_sentences, collapse_ws};
use crate::pipeline::adapter::is_pdf;
use crate::pipeline::canon::{read_json, write_atomic, write_canonical};
use crate::pipeline::rights::{host, licence_lines, licence_tags};

/// Hosts whose pages are open (CC BY-SA encyclopedia leads, open-access
/// records) and stay whole.
pub const OPEN: [&str; 2] = ["wikipedia.org", "doaj.org"];
/// Kept passages are joined by a sentence end, so no sentence spans two.
pub const SEPARATOR: &str = "\n\n.\n\n";
/// A quoted passage shorter than this is a label or a number, never text.
const SHORTEST: usize = 24;
const WORDS: usize = 4;

/// Whether a recorded page stays whole.
pub fn open(url: &str, extra: &[String]) -> bool {
    let host = host(url);
    OPEN.iter()
        .map(|h| h.to_string())
        .chain(extra.iter().cloned())
        .any(|h| host == h || host.ends_with(&format!(".{h}")))
}

/// The pages the run fetched as sources: those whose sentences it screened
/// (a screen request names its source's `url` and the `bytes` it read; a
/// rights request names the `url` alone).
pub fn screened(tape: &Path) -> Result<BTreeSet<String>, String> {
    Ok(entries(&tape.join("jev"))?
        .iter()
        .map(|e| &e["request"]["body"]["state"]["source"])
        .filter(|source| !source["bytes"].is_null())
        .filter_map(|source| source["url"].as_str())
        .map(str::to_string)
        .collect())
}

/// Every passage the run quoted to Jev: each string of each recorded request
/// body, line by line, without a row label, whitespace collapsed.
pub fn quoted(tape: &Path) -> Result<BTreeSet<String>, String> {
    let mut out = BTreeSet::new();
    for entry in entries(&tape.join("jev"))? {
        strings(&entry["request"]["body"], &mut out);
    }
    Ok(out)
}

fn entries(dir: &Path) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    let Ok(listing) = std::fs::read_dir(dir) else {
        return Ok(out);
    };
    let mut paths: Vec<_> = listing.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths
        .iter()
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
    {
        out.push(read_json(path).map_err(|e| format!("{}: {e}", path.display()))?);
    }
    Ok(out)
}

fn strings(value: &Value, out: &mut BTreeSet<String>) {
    match value {
        Value::String(s) => {
            for line in s.lines() {
                let line = unlabelled(line);
                let line = collapse_ws(line);
                // A passage of words, never an address or a label.
                if line.chars().count() >= SHORTEST && line.split(' ').count() >= WORDS {
                    out.insert(line);
                }
            }
        }
        Value::Array(list) => list.iter().for_each(|v| strings(v, out)),
        Value::Object(map) => map.values().for_each(|v| strings(v, out)),
        _ => {}
    }
}

/// A line without its row label (`F1.2: `, `h3: `).
fn unlabelled(line: &str) -> &str {
    match line.split_once(": ") {
        Some((label, rest))
            if label.len() <= 8 && label.chars().all(|c| c.is_ascii_alphanumeric() || c == '.') =>
        {
            rest
        }
        _ => line,
    }
}

/// The byte ranges of `text` each quoted passage covers, whitespace taken
/// loosely, each widened over its bordering whitespace, merged in order.
pub fn covered(text: &str, quotes: &BTreeSet<String>) -> Vec<(usize, usize)> {
    let (flat, at) = flattened(text);
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for quote in quotes {
        for (i, _) in flat.match_indices(quote.as_str()) {
            let (from, to) = (at[i], at[i + quote.len() - 1]);
            let to = text[to..].chars().next().map_or(to, |c| to + c.len_utf8());
            ranges.push(widen(text, from, to));
        }
    }
    ranges.sort_unstable();
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (from, to) in ranges {
        match merged.last_mut() {
            Some(last) if from <= last.1 => last.1 = last.1.max(to),
            _ => merged.push((from, to)),
        }
    }
    merged
}

/// `text` with whitespace runs as one space, and each byte's source offset.
fn flattened(text: &str) -> (String, Vec<usize>) {
    let (mut flat, mut at) = (String::new(), Vec::new());
    let mut space = true;
    for (i, c) in text.char_indices() {
        if c.is_whitespace() {
            if !space {
                flat.push(' ');
                at.push(i);
            }
            space = true;
            continue;
        }
        space = false;
        let before = flat.len();
        flat.push(c);
        at.extend(std::iter::repeat_n(i, flat.len() - before));
    }
    (flat, at)
}

fn widen(text: &str, mut from: usize, mut to: usize) -> (usize, usize) {
    while let Some(c) = text[..from]
        .chars()
        .next_back()
        .filter(|c| c.is_whitespace())
    {
        from -= c.len_utf8();
    }
    while let Some(c) = text[to..].chars().next().filter(|c| c.is_whitespace()) {
        to += c.len_utf8();
    }
    (from, to)
}

/// The quoted passages of `text`, in page order, joined by `SEPARATOR`.
pub fn passages(text: &str, quotes: &BTreeSet<String>) -> String {
    let parts: Vec<&str> = covered(text, quotes)
        .into_iter()
        .map(|(from, to)| &text[from..to])
        .collect();
    parts.join(SEPARATOR)
}

/// Minimal markup the rights check reads as it read the page: the page's
/// licence tags and its licence statements as paragraphs.
fn markup(tags: &[String], statements: &[String]) -> Vec<u8> {
    let body: String = statements
        .iter()
        .map(|s| format!("<p>{}</p>\n", escape(s)))
        .collect();
    format!(
        "<html><head>{}</head><body>\n{body}</body></html>\n",
        tags.join("")
    )
    .into_bytes()
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// One recorded scrape trimmed: its markdown and its bytes, checked to read
/// as the page read in every way a stage reads it.
pub fn page(
    url: &str,
    content_type: &str,
    markdown: &str,
    raw: &[u8],
    quotes: &BTreeSet<String>,
    fetched: bool,
) -> Result<(String, Vec<u8>), String> {
    let pdf = is_pdf(content_type, url);
    let lines = licence_lines(raw, markdown, content_type, url);
    let statements: Vec<String> = lines
        .iter()
        .filter(|l| !l.starts_with("metadata: "))
        .cloned()
        .collect();
    let mut kept: BTreeSet<String> = quotes.clone();
    kept.extend(statements.iter().cloned());
    let trimmed = passages(markdown, &kept);
    let bytes = match pdf || raw.is_empty() {
        true => Vec::new(),
        false => markup(&licence_tags(raw), &statements),
    };
    // A page the run fetched as a source had every candidate sentence
    // screened with its context: they read the same. A page it only checked
    // for rights was read for its licence lines alone.
    let sentences = |md: &str| -> Vec<(String, String)> {
        candidate_sentences(md)
            .into_iter()
            .map(|c| (c.sentence, c.context))
            .collect()
    };
    if fetched {
        let (after, before) = (sentences(&trimmed), sentences(markdown));
        if after != before {
            let apart = after.iter().zip(&before).find(|(a, b)| a != b);
            return Err(format!(
                "{url}: {} candidate sentences for {}; first apart: {apart:?}",
                after.len(),
                before.len()
            ));
        }
    }
    if licence_lines(&bytes, &trimmed, content_type, url) != lines {
        return Err(format!(
            "{url}: the trimmed page yields other licence lines"
        ));
    }
    Ok((trimmed, bytes))
}

/// Trims every recorded scrape of a page not openly licensed in `tape`,
/// in place; the words name each page and its size before and after.
pub fn tape(tape: &Path, extra_open: &[String]) -> Result<Vec<String>, String> {
    let quotes = quoted(tape)?;
    let fetched = screened(tape)?;
    let mut words = Vec::new();
    let dir = tape.join("firecrawl");
    for entry in entries(&dir)? {
        let request = &entry["request"];
        let page = &entry["response"]["ok"];
        let url = request["url"].as_str().unwrap_or_default();
        if request["op"] != "scrape" || page.is_null() || open(url, extra_open) {
            continue;
        }
        let key = entry["key"].as_str().unwrap_or_default();
        let path = dir.join(format!("{}.json", &key[..32]));
        let blob = path.with_extension("bin");
        let raw = std::fs::read(&blob).unwrap_or_default();
        let markdown = page["markdown"].as_str().unwrap_or_default();
        let ct = page["content_type"].as_str().unwrap_or_default();
        let (trimmed, bytes) = self::page(url, ct, markdown, &raw, &quotes, fetched.contains(url))?;
        words.push(format!(
            "{url}: markdown {} -> {}, bytes {} -> {}",
            markdown.len(),
            trimmed.len(),
            raw.len(),
            bytes.len()
        ));
        let mut entry = entry.clone();
        entry["response"]["ok"]["markdown"] = trimmed.into();
        write_canonical(&path, &entry).map_err(|e| e.to_string())?;
        match bytes.is_empty() {
            true => {
                let _ = std::fs::remove_file(&blob);
            }
            false => write_atomic(&blob, &bytes).map_err(|e| e.to_string())?,
        }
    }
    Ok(words)
}

/// Files every recorded answer under the key the current tape computes for
/// its request, so a recording follows a change of what a key covers; the
/// words name each answer moved.
pub fn rekey(tape: &Path) -> Result<Vec<String>, String> {
    let mut moved = Vec::new();
    for kind in ["firecrawl", "jev", "web"] {
        let dir = tape.join(kind);
        for mut entry in entries(&dir)? {
            let old = entry["key"].as_str().unwrap_or_default().to_string();
            let new = super::entry_key(kind, &entry["request"]);
            if old == new {
                continue;
            }
            let (from, to) = (dir.join(&old[..32]), dir.join(&new[..32]));
            entry["key"] = new.clone().into();
            write_canonical(&to.with_extension("json"), &entry).map_err(|e| e.to_string())?;
            std::fs::remove_file(from.with_extension("json")).map_err(|e| e.to_string())?;
            if from.with_extension("bin").exists() {
                std::fs::rename(from.with_extension("bin"), to.with_extension("bin"))
                    .map_err(|e| e.to_string())?;
            }
            moved.push(format!("{kind} {} -> {}", &old[..12], &new[..12]));
        }
    }
    Ok(moved)
}

/// Whether every passage a trimmed recorded page keeps is one the run
/// quoted: the check a committed fixture must pass.
pub fn only_quoted(tape: &Path, extra_open: &[String]) -> Result<Vec<String>, String> {
    let quotes = quoted(tape)?;
    let mut over = Vec::new();
    for entry in entries(&tape.join("firecrawl"))? {
        let request = &entry["request"];
        let page = &entry["response"]["ok"];
        let url = request["url"].as_str().unwrap_or_default();
        if request["op"] != "scrape" || page.is_null() || open(url, extra_open) {
            continue;
        }
        let markdown = page["markdown"].as_str().unwrap_or_default();
        let raw = std::fs::read(tape.join("firecrawl").join(format!(
            "{}.bin",
            &entry["key"].as_str().unwrap_or_default()[..32]
        )))
        .unwrap_or_default();
        let lines = licence_lines(&raw, markdown, "", url);
        let mut kept = quotes.clone();
        kept.extend(lines.iter().cloned());
        let quoted: usize = covered(markdown, &kept).iter().map(|(f, t)| t - f).sum();
        // Each separator's full stop is the one byte no passage covers.
        if quoted + markdown.matches(SEPARATOR).count() < markdown.len() {
            over.push(format!(
                "{url}: {quoted} of {} bytes quoted",
                markdown.len()
            ));
        }
        let statements: usize = lines.iter().map(|l| l.len() + 1).sum();
        let text = collapse_ws(&crate::html::source_text(&raw));
        if text.len() > statements {
            over.push(format!("{url}: {} bytes of page text kept", text.len()));
        }
    }
    Ok(over)
}
