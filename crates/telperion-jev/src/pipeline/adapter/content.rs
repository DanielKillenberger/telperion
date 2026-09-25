//! Fetch refuses what is not the source. A scrape's markdown is refused when
//! it is empty, when it is a known interstitial (a cookie wall, a bot check,
//! a login page), or when it is a sliver of the raw body. The palm's P7 and
//! P8 were a 196-byte cookie wall over a 196 KB article (fn-80, fn-130).

use crate::html::source_text;

/// Text under this share of the raw body's bytes is not the page: the
/// palm's admitted articles ran 12% to 79%, its cookie walls 0.1%.
pub const MIN_TEXT_SHARE: f64 = 0.02;

/// An interstitial is a short page; a longer one that mentions cookies is
/// an article.
const INTERSTITIAL_BYTES: usize = 20_000;

const INTERSTITIALS: [&str; 10] = [
    "cookies must be enabled",
    "enable cookies",
    "checking your browser",
    "verify you are human",
    "are you a robot",
    "please enable javascript",
    "access denied",
    "sign in to continue",
    "log in to continue",
    "captcha",
];

/// The text a stage may read and, when the scrape's markdown was refused,
/// why it was.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Readable {
    pub markdown: String,
    pub refused: Option<String>,
}

/// Why `text` is not the source `raw` holds, or None when it is. A PDF's
/// text is not held to the raw share: its bytes are compressed.
pub fn refusal(text: &str, raw: &[u8], pdf: bool) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Some("the text is empty".into());
    }
    let lower = trimmed.to_ascii_lowercase();
    if trimmed.len() < INTERSTITIAL_BYTES && INTERSTITIALS.iter().any(|p| lower.contains(p)) {
        let first = trimmed.lines().next().unwrap_or_default();
        return Some(format!("an interstitial page: {first}"));
    }
    let share = trimmed.len() as f64 / raw.len().max(1) as f64;
    if !pdf && share < MIN_TEXT_SHARE {
        return Some(format!(
            "{} bytes of text from a {}-byte body, under {}%",
            trimmed.len(),
            raw.len(),
            MIN_TEXT_SHARE * 100.0
        ));
    }
    None
}

/// The scrape's markdown when it is the source; else the raw body's own
/// conversion when that is; else why neither is, which files
/// unavailable-source.
pub fn readable(markdown: &str, raw: &[u8], pdf: bool) -> Result<Readable, String> {
    let Some(refused) = refusal(markdown, raw, pdf) else {
        return Ok(Readable {
            markdown: markdown.to_string(),
            refused: None,
        });
    };
    if pdf {
        return Err(format!("the parsed PDF was refused: {refused}"));
    }
    let converted = source_text(raw);
    match refusal(&converted, raw, false) {
        None => Ok(Readable {
            markdown: converted,
            refused: Some(refused),
        }),
        Some(again) => Err(format!(
            "the scrape was refused: {refused}; the raw body's conversion was refused: {again}"
        )),
    }
}
