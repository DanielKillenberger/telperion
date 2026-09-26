//! A recording that republishes no third-party page (owner, 2026-09-25).
//!
//! A recorded page whose source is not openly licensed keeps only what the
//! run quoted from it: the passages that reached a Jev request (candidate
//! sentences with their context, verified excerpts, licence statements),
//! each whole, in page order, and nothing else. Its bytes keep only the
//! licence tags and statements the rights check reads. Every later request
//! of a replay is then the recorded one, so every key still answers; the
//! trim checks the sentences the run read and the licence lines itself and
//! refuses a page it would change. `species` records with `--record`; this
//! runs over the recording before it is committed (`tape_trim`).
use std::collections::BTreeSet;
use std::path::Path;

use serde_json::Value;

use crate::extract::{candidate_sentences, collapse_ws};
use crate::pipeline::adapter::is_pdf_body;
use crate::pipeline::canon::{read_json, write_atomic, write_canonical};
use crate::pipeline::requirements::table;
use crate::pipeline::rights::{host, licence_lines, licence_tags};
use crate::pipeline::stages::read::occurrences;

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
/// body, line by line, without a row label, whitespace collapsed, and each
/// passage a document's kind was asked over whatever its length.
pub fn quoted(tape: &Path) -> Result<BTreeSet<String>, String> {
    let mut out = BTreeSet::new();
    for entry in entries(&tape.join("jev"))? {
        let body = &entry["request"]["body"];
        strings(body, &mut out);
        let passages = body["state"]["passages"].as_array().into_iter().flatten();
        out.extend(passages.filter_map(Value::as_str).map(collapse_ws));
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

/// The page text the licence statements were cut from, each run of
/// overlapping statements as the one passage it is on the page: statements
/// cut around neighbouring notices overlap, and set apart they would be cut
/// otherwise when read again (the Morton Arboretum's photograph credits,
/// fn-157). A statement not found on the page stands as it is.
fn regions(raw: &[u8], statements: &[String]) -> Vec<String> {
    let text = collapse_ws(&crate::html::source_text(raw));
    let mut spans: Vec<(usize, usize)> = Vec::new();
    let mut loose = Vec::new();
    let mut from = 0;
    for statement in statements {
        match text[from..].find(statement.as_str()) {
            Some(at) => {
                spans.push((from + at, from + at + statement.len()));
                from += at;
            }
            None => loose.push(statement.clone()),
        }
    }
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (start, end) in spans {
        match merged.last_mut() {
            Some(last) if start <= last.1 => last.1 = last.1.max(end),
            _ => merged.push((start, end)),
        }
    }
    let mut out: Vec<String> = merged
        .iter()
        .map(|&(a, b)| text[a..b].to_string())
        .collect();
    out.extend(loose);
    out
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
    let pdf = is_pdf_body(content_type, url, raw);
    let kind = if pdf { "application/pdf" } else { content_type };
    let lines = licence_lines(raw, markdown, kind, url);
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
        false => markup(&licence_tags(raw), &regions(raw, &statements)),
    };
    // A page the run fetched as a source had the candidate sentences that
    // name a field read with their context (fn-157): each of those the run
    // quoted reads the same in the trimmed page, and the trimmed page holds
    // no candidate sentence the page did not.
    let sentences = |md: &str| -> Vec<(String, String)> {
        candidate_sentences(md)
            .into_iter()
            .map(|c| (c.sentence, c.context))
            .collect()
    };
    if fetched {
        let (after, before) = (sentences(&trimmed), sentences(markdown));
        let asked = before.iter().filter(|(s, _)| quotes.contains(s));
        if let Some(lost) = asked.clone().find(|b| !after.contains(b)) {
            return Err(format!(
                "{url}: the trimmed page reads {:?} otherwise",
                lost.0
            ));
        }
        // A passage cut mid-sentence starts a sentence the page never had;
        // it is harmless unless the read stage would ask it.
        let own: BTreeSet<&String> = before.iter().map(|(s, _)| s).collect();
        let fields: Vec<String> = table().fields.keys().cloned().collect();
        let asks = |sentence: &str| !occurrences(sentence, &fields).is_empty();
        if let Some((extra, _)) = after.iter().find(|(s, _)| !own.contains(s) && asks(s)) {
            return Err(format!(
                "{url}: the trimmed page yields a new sentence {extra:?}"
            ));
        }
    }
    if licence_lines(&bytes, &trimmed, kind, url) != lines {
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
        let ct = page["content_type"].as_str().unwrap_or_default();
        let ct = if is_pdf_body(ct, url, &raw) {
            "application/pdf"
        } else {
            ct
        };
        let lines = licence_lines(&raw, markdown, ct, url);
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::page;
    use crate::extract::candidate_sentences;

    const URL: &str = "https://example.test/beech";

    /// fn-157: a fetched page keeps the sentences the read stage asked, each
    /// with the context it was asked in, and drops the candidate sentences
    /// it never asked; a sentence kept without its context is refused.
    #[test]
    fn a_page_keeps_the_sentences_read_asked_and_nothing_else() {
        let filler = "Beech woods are shaded and quiet in every season. ".repeat(12);
        let text = format!(
            "The nuts are 2 cm across and fall in autumn. {filler}\
             The tree reaches 30 m tall in parks and gardens across Europe."
        );
        let text = text.as_str();
        let asked = candidate_sentences(text)
            .into_iter()
            .find(|c| c.sentence.contains("30 m"))
            .unwrap();
        let quotes: BTreeSet<String> = [asked.sentence.clone(), asked.context.clone()].into();
        let (trimmed, _) = page(URL, "text/markdown", text, &[], &quotes, true).unwrap();
        assert!(trimmed.contains("reaches 30 m tall"), "{trimmed}");
        assert!(!trimmed.contains("nuts"), "{trimmed}");
        let bare: BTreeSet<String> = [asked.sentence].into();
        let err = page(URL, "text/markdown", text, &[], &bare, true).unwrap_err();
        assert!(err.contains("reads"), "{err}");
    }

    /// fn-157, the oak's replay: a document's kind is asked over up to three
    /// candidate sentences, and one may be a single word (an address cut at
    /// its full stop). Each is quoted whatever its length, or the trimmed
    /// page asks the kind question over other passages.
    #[test]
    fn a_kind_question_keeps_every_passage_it_asked_over() {
        let tape = std::env::temp_dir().join(format!("trim-kind-{}", std::process::id()));
        std::fs::create_dir_all(tape.join("jev")).unwrap();
        let entry = serde_json::json!({"request": {"body": {"state": {
            "passages": ["org%2Fportal%2Ftaxa%2Findex."]}}}});
        std::fs::write(tape.join("jev/a.json"), entry.to_string()).unwrap();
        let quoted = super::quoted(&tape).unwrap();
        assert!(
            quoted.contains("org%2Fportal%2Ftaxa%2Findex."),
            "{quoted:?}"
        );
    }

    /// fn-157, the oak's replay: a page fetch dropped live was never read,
    /// and keeps nothing, not even its licence tags; kept, the tags alone
    /// read as a page and the replay asks its kind.
    #[test]
    fn a_page_the_run_never_read_keeps_nothing() {
        let tape = std::env::temp_dir().join(format!("trim-unread-{}", std::process::id()));
        std::fs::create_dir_all(tape.join("firecrawl")).unwrap();
        let key = "a".repeat(64);
        let entry = serde_json::json!({"key": key,
            "request": {"op": "scrape", "url": "https://video.test/watch"},
            "response": {"ok": {"content_type": "text/html", "markdown": "A video page of 3 m trees."}}});
        let path = tape.join(format!("firecrawl/{}.json", &key[..32]));
        std::fs::write(&path, entry.to_string()).unwrap();
        let raw = r#"<html><link rel="license" href="https://creativecommons.org/licenses/by/3.0/"></html>"#;
        std::fs::write(path.with_extension("bin"), raw).unwrap();
        super::tape(&tape, &[]).unwrap();
        let after: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(after["response"]["ok"]["markdown"], "");
        assert!(!path.with_extension("bin").exists());
    }
}
