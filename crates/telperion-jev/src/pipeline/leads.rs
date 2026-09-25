//! Wikipedia is a lead, never a citation (owner; fn-82's spec, 2026-09-25).
//!
//! A tertiary encyclopedic page names where its numbers came from; the
//! numbers are the primary source's to give. Code recognises such a page by
//! its host, follows the references it cites to the primary sources
//! (silvics literature, forestry tables, floras, papers) and offers those as
//! candidates in its place. A tertiary page is never proposed, admitted or
//! fetched, so no profile value can cite one. The rule is a host list and a
//! link walk, never a judgment.
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use regex::Regex;

use super::adapter::{FetchAdapter, SearchHit};
use super::rights::host;

/// Hosts whose pages are tertiary encyclopedias, matched with their
/// subdomains (`en.wikipedia.org`).
const TERTIARY: [&str; 6] = [
    "wikipedia.org",
    "wikiwand.com",
    "britannica.com",
    "encyclopedia.com",
    "newworldencyclopedia.org",
    "dbpedia.org",
];

/// Hosts a reference list links to that are the encyclopedia's own
/// machinery, never a primary source.
const MACHINERY: [&str; 5] = [
    "wikimedia.org",
    "wikidata.org",
    "wiktionary.org",
    "wikisource.org",
    "wikibooks.org",
];

/// Primary sources taken from one tertiary page.
pub const PER_LEAD: usize = 8;

fn under(url: &str, hosts: &[&str]) -> bool {
    let host = host(url);
    hosts
        .iter()
        .any(|h| host == *h || host.ends_with(&format!(".{h}")))
}

/// Whether `url` is a tertiary encyclopedic page.
pub fn is_tertiary(url: &str) -> bool {
    under(url, &TERTIARY)
}

/// Characters kept of a citation, of one citing sentence, and of a snippet.
const CITATION: usize = 200;
const SENTENCE: usize = 300;
const SNIPPET: usize = 700;

fn regex(cell: &'static OnceLock<Regex>, pattern: &str) -> &'static Regex {
    cell.get_or_init(|| Regex::new(pattern).expect("leads regex"))
}

/// The primary sources a tertiary page's reference list cites, one per
/// reference: its DOI link when it has one, else its first outside link.
/// Each carries the citation as its title and, as its snippet, the citation
/// and every sentence of the page that cites it, so the ranking judges the
/// words the page gave for it, never a bare address.
pub fn primary(markdown: &str, page: &str) -> Vec<SearchHit> {
    static HEADING: OnceLock<Regex> = OnceLock::new();
    static ITEM: OnceLock<Regex> = OnceLock::new();
    let heading = regex(
        &HEADING,
        r"(?im)^#{1,6}\s*(references|notes|citations|sources|footnotes|bibliography)\b",
    );
    let item = regex(&ITEM, r"^\s*0*(\d+)\.\s+(.*)$");
    let split = heading.find(markdown).map_or(markdown.len(), |m| m.start());
    let (body, list) = markdown.split_at(split);
    let citing = sentences(body);
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for line in list.lines() {
        let Some(found) = item.captures(line) else {
            continue;
        };
        let Some(url) = link_of(&found[2]).filter(|u| seen.insert(u.clone())) else {
            continue;
        };
        let citation = clip(&clean(&back_links(&found[2])), CITATION);
        let said = citing.get(&found[1]).map_or(String::new(), |s| s.join(" "));
        let snippet = match said.is_empty() {
            true => format!("{citation} Cited by {page}."),
            false => format!("{citation} Cited by {page} for: {said}"),
        };
        out.push(SearchHit {
            url,
            title: citation,
            snippet: clip(&snippet, SNIPPET),
        });
    }
    out
}

/// A reference's link: its DOI when it has one, else its first link that is
/// neither an encyclopedia page nor the encyclopedia's machinery.
fn link_of(text: &str) -> Option<String> {
    static LINK: OnceLock<Regex> = OnceLock::new();
    let link = regex(&LINK, r"\]\((https?://[^)\s]+)");
    let outside: Vec<String> = link
        .captures_iter(text)
        .map(|c| c[1].to_string())
        .filter(|u| !is_tertiary(u) && !under(u, &MACHINERY))
        .collect();
    let doi = outside.iter().find(|u| host(u) == "doi.org");
    doi.or(outside.first()).cloned()
}

/// Each footnote number, with the sentences of `body` that cite it: the text
/// from the sentence's start, or the previous footnote, to its marker.
fn sentences(body: &str) -> BTreeMap<String, Vec<String>> {
    static MARKER: OnceLock<Regex> = OnceLock::new();
    let marker = regex(&MARKER, r"\[\\\[(\d+)\\\]\]\([^)]*#cite_note-[^)]*\)");
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for line in body.lines() {
        let mut from = 0;
        for found in marker.captures_iter(line) {
            let whole = found.get(0).expect("a match");
            let prefix = &line[from..whole.start()];
            let start = prefix.trim_end().rfind(". ").map_or(0, |at| at + 2);
            let sentence = clip(&clean(&prefix[start..]), SENTENCE);
            from = whole.end();
            if !sentence.is_empty() {
                out.entry(found[1].to_string()).or_default().push(sentence);
            }
        }
    }
    out
}

/// Markdown to plain words: links to their labels, anchors and emphasis gone.
fn clean(text: &str) -> String {
    static LINK: OnceLock<Regex> = OnceLock::new();
    let link = regex(&LINK, r"\[([^\]]*)\]\([^)]*\)");
    let text = link.replace_all(text, "$1");
    let text = text.replace(['_', '*', '"'], "").replace('↑', "");
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A footnote without its back-links to the places that cite it.
fn back_links(text: &str) -> String {
    static BACK: OnceLock<Regex> = OnceLock::new();
    let back = regex(&BACK, r"\[[^\]]*\]\([^)]*#cite_ref-[^)]*\)");
    back.replace_all(text, "").into_owned()
}

fn clip(text: &str, most: usize) -> String {
    match text.char_indices().nth(most) {
        Some((at, _)) => format!("{}...", &text[..at]),
        None => text.to_string(),
    }
}

/// `hits` with every tertiary page replaced by the primary sources it
/// cites. A lead that cannot be read yields nothing.
pub fn follow(adapter: &dyn FetchAdapter, hits: Vec<SearchHit>) -> Vec<SearchHit> {
    let mut out = Vec::new();
    for hit in hits {
        if !is_tertiary(&hit.url) {
            out.push(hit);
            continue;
        }
        let Ok(page) = adapter.scrape(&hit.url) else {
            continue;
        };
        out.extend(
            primary(&page.markdown, &hit.title)
                .into_iter()
                .take(PER_LEAD),
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_host_rule_names_the_encyclopedias_and_their_subdomains() {
        assert!(is_tertiary("https://en.wikipedia.org/wiki/Fagus_sylvatica"));
        assert!(is_tertiary("https://www.britannica.com/plant/beech"));
        assert!(!is_tertiary("https://research.fs.usda.gov/silvics/beech"));
        assert!(!is_tertiary("https://notwikipedia.org/x"));
    }
}
