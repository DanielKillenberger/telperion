//! Candidate photographs from Wikimedia Commons, a photograph host (the
//! no-Wikipedia rule is about citations of values, not photographs). Code
//! walks the taxon's category: its subcategories that file single trees
//! (standalone, solitary, famous, park or field trees) first, then Commons'
//! copies of geograph.org.uk's photographs (CC BY-SA, often one named tree
//! in a field) by search, then the bare tree in winter and the bark (host
//! decision, fn-157). Each answer is parsed by code: the file page, a
//! bounded-width copy, the author and the licence statements the rights
//! question reads.
use serde_json::Value;

use super::{Candidate, Origin, Web};

const API: &str = "https://commons.wikimedia.org/w/api.php";
/// The width of the copy downloaded, enough for a reviewer's look.
const WIDTH: u32 = 1280;

/// Words in a subcategory's name that file single trees, and the files
/// each such category may add.
const WHOLE: [&str; 6] = [
    "standalone",
    "solitary",
    "single",
    "famous",
    "park",
    "field",
];
const PER_WHOLE: usize = 3;

/// The geograph searches: its photographs as Commons keeps them.
pub fn searches(taxon: &str, common: &str) -> [(String, usize); 2] {
    [
        (format!("{taxon} geograph"), 3),
        (format!("{common} geograph"), 3),
    ]
}

/// The request listing the taxon category's subcategories.
pub fn subcategories_url(taxon: &str) -> String {
    let title = encode(&format!("Category:{taxon}"));
    format!("{API}?action=query&format=json&list=categorymembers&cmtitle={title}&cmtype=subcat&cmlimit=100")
}

/// The request for up to `limit` files of `category`, with their images.
pub fn category_url(category: &str, limit: usize) -> String {
    let title = encode(category);
    format!(
        "{API}?action=query&format=json&generator=categorymembers&gcmtitle={title}&gcmtype=file\
         &gcmlimit={limit}&prop=imageinfo&iiprop=url|mime|extmetadata&iiurlwidth={WIDTH}"
    )
}

/// Every request for candidates, whole trees first.
fn requests(web: &dyn Web, taxon: &str, common: &str) -> Vec<String> {
    let names: Vec<String> = answer(web, &subcategories_url(taxon))
        .map(|body| {
            body["query"]["categorymembers"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|m| m["title"].as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    let lower = |n: &String| n.to_lowercase();
    let mut out: Vec<String> = names
        .iter()
        .filter(|n| WHOLE.iter().any(|w| lower(n).contains(w)))
        .map(|n| category_url(n, PER_WHOLE))
        .collect();
    out.extend(searches(taxon, common).iter().map(|(q, l)| url(q, *l)));
    let named = |suffix: &str| names.iter().find(|n| lower(n).ends_with(suffix));
    out.extend(named("in winter").map(|n| category_url(n, 2)));
    out.extend(named("(bark)").map(|n| category_url(n, 1)));
    out
}

fn answer(web: &dyn Web, url: &str) -> Option<Value> {
    let (bytes, _) = web.get(url).ok()?;
    serde_json::from_slice(&bytes).ok()
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

/// The Commons candidates for `taxon`, whole trees first, JPEG or PNG only.
pub fn candidates(web: &dyn Web, taxon: &str, common: &str) -> Vec<Candidate> {
    let mut out: Vec<Candidate> = Vec::new();
    for request in requests(web, taxon, common) {
        let Some(body) = answer(web, &request) else {
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

/// The files of one API answer, ordered by their search rank (a category
/// listing has none, and keeps its own order).
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

    /// fn-157 R7: the single-tree categories come first, then geograph, then
    /// the bare tree and the bark; a category of leaves or forests is never
    /// asked.
    #[test]
    fn the_whole_tree_is_asked_for_first() {
        struct Listing;
        impl super::Web for Listing {
            fn get(&self, url: &str) -> Result<(Vec<u8>, String), String> {
                if url != super::subcategories_url("Fagus sylvatica") {
                    return Err(url.into());
                }
                let names = [
                    "Category:Fagus sylvatica (leaves)",
                    "Category:Fagus sylvatica (forests)",
                    "Category:Fagus sylvatica (standalone)",
                    "Category:Famous Fagus sylvatica",
                    "Category:Fagus sylvatica in winter",
                    "Category:Fagus sylvatica (bark)",
                ];
                let members: Vec<_> = names
                    .iter()
                    .map(|t| serde_json::json!({"title": t}))
                    .collect();
                let body = serde_json::json!({"query": {"categorymembers": members}});
                Ok((
                    serde_json::to_vec(&body).unwrap(),
                    "application/json".into(),
                ))
            }
        }
        let asked = super::requests(&Listing, "Fagus sylvatica", "European beech");
        let cat = super::category_url;
        let search = |q: &str| super::url(q, 3);
        assert_eq!(
            asked,
            [
                cat("Category:Fagus sylvatica (standalone)", 3),
                cat("Category:Famous Fagus sylvatica", 3),
                search("Fagus sylvatica geograph"),
                search("European beech geograph"),
                cat("Category:Fagus sylvatica in winter", 2),
                cat("Category:Fagus sylvatica (bark)", 1),
            ]
        );
    }

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
