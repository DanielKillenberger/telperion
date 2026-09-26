//! The trim's own check: a trimmed recording holds no more of a page than
//! the run quoted.
use std::path::Path;

use super::{covered, entries, open, quoted, PDF_MAGIC, SEPARATOR};
use crate::extract::collapse_ws;
use crate::pipeline::adapter::is_pdf_body;
use crate::pipeline::rights::licence_lines;

/// Whether every passage a trimmed recorded page keeps is one the run
/// quoted: the check a committed fixture must pass.
pub fn only_quoted(tape: &Path, extra_open: &[String]) -> Result<Vec<String>, String> {
    let quotes = quoted(tape)?;
    let mut over = Vec::new();
    for entry in entries(&tape.join("firecrawl"))? {
        let request = &entry["request"];
        let page = &entry["response"]["ok"];
        let url = request["url"].as_str().unwrap_or_default();
        if let (Some("parse"), Some(parsed)) = (request["op"].as_str(), page.as_str()) {
            let quoted: usize = covered(parsed, &quotes).iter().map(|(f, t)| t - f).sum();
            if quoted + parsed.matches(SEPARATOR).count() < parsed.len() {
                let file = request["file"].as_str().unwrap_or_default();
                over.push(format!(
                    "{file}: {quoted} of {} parsed bytes quoted",
                    parsed.len()
                ));
            }
            continue;
        }
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
        let raw = if raw == PDF_MAGIC { &[][..] } else { &raw[..] };
        let text = collapse_ws(&crate::html::source_text(raw));
        if text.len() > statements {
            over.push(format!("{url}: {} bytes of page text kept", text.len()));
        }
    }
    Ok(over)
}
