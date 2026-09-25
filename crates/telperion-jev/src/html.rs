//! HTML is converted to text once, where it enters: a raw body the fetch
//! stage falls back on, a page the cite tool loads. Markdown is never passed
//! through here; a "<" in a p-value is text, not a tag.

/// Elements whose content is never read: scripts, styles and the head.
const HIDDEN: [&str; 5] = ["script", "style", "noscript", "head", "template"];

/// Elements that end a line of text.
const BLOCK: &str = "p div br li ul ol tr table section article header footer \
    h1 h2 h3 h4 h5 h6 figure figcaption blockquote pre";

/// True when the bytes open as an HTML document.
pub fn looks_like_html(bytes: &[u8]) -> bool {
    let head = String::from_utf8_lossy(&bytes[..bytes.len().min(1024)]).to_ascii_lowercase();
    let head = head.trim_start();
    head.starts_with("<!doctype html") || head.starts_with("<html")
}

/// The text of a loaded source: an HTML document converted, anything else
/// (markdown, plain text) read as it is.
pub fn source_text(bytes: &[u8]) -> String {
    if looks_like_html(bytes) {
        html_text(bytes)
    } else {
        String::from_utf8_lossy(bytes).into_owned()
    }
}

/// The visible text of an HTML document: hidden elements and comments
/// dropped, block elements ending a line, entities decoded, blank runs
/// collapsed. A "<" that opens no tag stays text.
pub fn html_text(bytes: &[u8]) -> String {
    let raw = String::from_utf8_lossy(bytes);
    let lower = raw.to_ascii_lowercase();
    let mut out = String::with_capacity(raw.len() / 2);
    let mut cursor = 0;
    while let Some(rel) = raw[cursor..].find('<') {
        let at = cursor + rel;
        out.push_str(&raw[cursor..at]);
        let Some(tag) = tag_at(&lower, at) else {
            out.push('<');
            cursor = at + 1;
            continue;
        };
        cursor = match tag {
            Tag::Comment => lower[at..].find("-->").map_or(raw.len(), |i| at + i + 3),
            Tag::Element { name, end, closing } => {
                if BLOCK.split_whitespace().any(|block| block == name) {
                    out.push('\n');
                } else if name == "td" || name == "th" {
                    out.push(' ');
                }
                match HIDDEN.contains(&name.as_str()) && !closing {
                    true => close_of(&lower, &name, end),
                    false => end,
                }
            }
        };
    }
    out.push_str(&raw[cursor..]);
    tidy(&decode_entities(&out))
}

enum Tag {
    Comment,
    Element {
        name: String,
        end: usize,
        closing: bool,
    },
}

/// The tag opening at `at`, or None when the "<" opens no tag.
fn tag_at(lower: &str, at: usize) -> Option<Tag> {
    let rest = &lower[at + 1..];
    if rest.starts_with("!--") {
        return Some(Tag::Comment);
    }
    let closing = rest.starts_with('/');
    let body = rest.trim_start_matches('/');
    let first = body.chars().next()?;
    if !(first.is_ascii_alphabetic() || first == '!' || first == '?') {
        return None;
    }
    let name: String = body
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric())
        .collect();
    let end = rest.find('>').map_or(lower.len(), |i| at + 1 + i + 1);
    Some(Tag::Element { name, end, closing })
}

/// The byte after `</name>`, searching from `from`, or the end of the text.
fn close_of(lower: &str, name: &str, from: usize) -> usize {
    let close = format!("</{name}");
    lower[from..]
        .find(&close)
        .and_then(|i| lower[from + i..].find('>').map(|j| from + i + j + 1))
        .unwrap_or(lower.len())
}

fn decode_entities(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(at) = rest.find('&') {
        out.push_str(&rest[..at]);
        let tail = &rest[at..];
        match tail
            .find(';')
            .filter(|&end| end <= 10)
            .and_then(|end| entity(&tail[1..end]).map(|ch| (ch, end + 1)))
        {
            Some((ch, used)) => {
                out.push(ch);
                rest = &tail[used..];
            }
            None => {
                out.push('&');
                rest = &tail[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

fn entity(name: &str) -> Option<char> {
    let named = match name {
        "nbsp" => Some(' '),
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        "ndash" => Some('–'),
        "mdash" => Some('—'),
        "deg" => Some('°'),
        "times" => Some('×'),
        _ => None,
    };
    if named.is_some() {
        return named;
    }
    let number = name.strip_prefix('#')?;
    let code = match number.strip_prefix(['x', 'X']) {
        Some(hex) => u32::from_str_radix(hex, 16).ok()?,
        None => number.parse().ok()?,
    };
    char::from_u32(code).map(|ch| if ch == '\u{a0}' { ' ' } else { ch })
}

/// Trims each line and keeps at most one blank line between runs of text.
fn tidy(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut blank = true;
    for line in text.lines().map(str::trim) {
        if line.is_empty() {
            if !blank {
                out.push('\n');
            }
            blank = true;
            continue;
        }
        out.push_str(line);
        out.push('\n');
        blank = false;
    }
    out.trim_end().to_string()
}
