//! Candidate photographs from Wikimedia Commons, a photograph host (the
//! no-Wikipedia rule is about citations of values, not photographs). One
//! free API query per view the reviewer wants, parsed by code: the file
//! page, a bounded-width copy, the author and the licence statements the
//! rights question reads.
use serde_json::Value;

use super::{Candidate, Origin, Web};

const API: &str = "https://commons.wikimedia.org/w/api.php";
/// The width of the copy downloaded, enough for a reviewer's look.
const WIDTH: u32 = 1280;

/// The queries per taxon and the candidates each may add: the whole tree
/// (the bare taxon found leaves, buds and nuts for the beech), its bark
/// close up and its bare winter form.
pub fn queries(taxon: &str) -> [(String, usize); 3] {
    [
        (format!("{taxon} tree"), 6),
        (format!("{taxon} bark"), 3),
        (format!("{taxon} winter"), 3),
    ]
}

pub fn url(query: &str, limit: usize) -> String {
    let q = encode(&format!("{query} filetype:bitmap"));
    format!(
        "{API}?action=query&format=json&generator=search&gsrnamespace=6&gsrlimit={limit}\
         &gsrsearch={q}&prop=imageinfo&iiprop=url|mime|extmetadata&iiurlwidth={WIDTH}"
    )
}

fn encode(text: &str) -> String {
    text.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            b' ' => "+".into(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

/// The Commons candidates for `taxon`, in search order, JPEG or PNG only.
pub fn candidates(web: &dyn Web, taxon: &str) -> Vec<Candidate> {
    let mut out: Vec<Candidate> = Vec::new();
    for (query, limit) in queries(taxon) {
        let Ok((bytes, _)) = web.get(&url(&query, limit)) else {
            continue;
        };
        let Ok(body) = serde_json::from_slice::<Value>(&bytes) else {
            continue;
        };
        for found in parse(&body) {
            if !out.iter().any(|c| c.page == found.page) {
                out.push(found);
            }
        }
    }
    out
}

/// The files of one API answer, ordered by their search rank.
pub fn parse(body: &Value) -> Vec<Candidate> {
    let mut pages: Vec<&Value> = body["query"]["pages"]
        .as_object()
        .map(|m| m.values().collect())
        .unwrap_or_default();
    pages.sort_by_key(|p| p["index"].as_u64().unwrap_or(u64::MAX));
    pages.into_iter().filter_map(file).collect()
}

fn file(page: &Value) -> Option<Candidate> {
    let info = &page["imageinfo"][0];
    let mime = info["mime"].as_str()?;
    if mime != "image/jpeg" && mime != "image/png" {
        return None;
    }
    let meta = &info["extmetadata"];
    let field = |key: &str| strip(meta[key]["value"].as_str().unwrap_or_default());
    let licence = field("LicenseShortName");
    let statements = [
        "License",
        "LicenseShortName",
        "UsageTerms",
        "LicenseUrl",
        "AttributionRequired",
        "Copyrighted",
        "Restrictions",
        "Artist",
    ]
    .into_iter()
    .map(|key| (key, field(key)))
    .filter(|(_, value)| !value.is_empty())
    .map(|(key, value)| format!("{key}: {value}"))
    .collect();
    Some(Candidate {
        page: info["descriptionurl"].as_str()?.to_string(),
        image: info["thumburl"]
            .as_str()
            .or(info["url"].as_str())?
            .to_string(),
        title: page["title"].as_str().unwrap_or_default().to_string(),
        attribution: format!("{}, Wikimedia Commons, {licence}", field("Artist")),
        licence: field("License"),
        statements,
        origin: Origin::Commons,
    })
}

/// Whether a file's machine-readable licence code is an open one the
/// rights question admits: CC0, public domain, CC BY or CC BY-SA. Any other
/// code, or none, goes to the rights question.
pub fn open_code(code: &str) -> bool {
    let code = code.to_ascii_lowercase();
    let versioned = |prefix: &str| {
        code.strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_digit()))
    };
    code == "cc0" || code.starts_with("pd") || versioned("cc-by-") || versioned("cc-by-sa-")
}

/// Text without HTML tags or surrounding space.
fn strip(html: &str) -> String {
    let mut out = String::new();
    let mut inside = false;
    for c in html.chars() {
        match c {
            '<' => inside = true,
            '>' => inside = false,
            _ if !inside => out.push(c),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn an_answer_yields_its_bitmaps_in_rank_order_with_their_licence() {
        let info = |mime: &str, name: &str| {
            json!([{"mime": mime,
            "descriptionurl": format!("https://commons.wikimedia.org/wiki/File:{name}"),
            "thumburl": format!("https://upload.wikimedia.org/{name}"),
            "extmetadata": {"LicenseShortName": {"value": "CC BY-SA 4.0"},
                            "Artist": {"value": "<a href=\"x\">Ann</a> "}}}])
        };
        let body = json!({"query": {"pages": {
            "2": {"index": 2, "title": "File:b.jpg", "imageinfo": info("image/jpeg", "b.jpg")},
            "1": {"index": 1, "title": "File:a.png", "imageinfo": info("image/png", "a.png")},
            "3": {"index": 3, "title": "File:c.svg", "imageinfo": info("image/svg+xml", "c.svg")},
        }}});
        let found = parse(&body);
        let titles: Vec<&str> = found.iter().map(|c| c.title.as_str()).collect();
        assert_eq!(titles, ["File:a.png", "File:b.jpg"]);
        assert_eq!(found[0].attribution, "Ann, Wikimedia Commons, CC BY-SA 4.0");
        assert_eq!(
            found[0].statements,
            ["LicenseShortName: CC BY-SA 4.0", "Artist: Ann"]
        );
        for code in [
            "cc0",
            "pd-old-100",
            "cc-by-3.0",
            "cc-by-sa-4.0",
            "CC-BY-SA-2.5-NL",
        ] {
            assert!(open_code(code), "{code}");
        }
        for code in ["", "cc-by-nc-4.0", "cc-by-nd-2.0", "gfdl", "cc-by-sa"] {
            assert!(!open_code(code), "{code}");
        }
        assert!(url("Fagus sylvatica", 6).contains("gsrsearch=Fagus+sylvatica+filetype%3Abitmap"));
    }
}
