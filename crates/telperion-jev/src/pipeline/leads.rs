//! Wikipedia is a lead, never a citation (owner; fn-82's spec, 2026-09-25).
//!
//! A tertiary encyclopedic page names where its numbers came from; the
//! numbers are the primary source's to give. Code recognises such a page by
//! its host, follows the references it cites to the primary sources
//! (silvics literature, forestry tables, floras, papers) and offers those as
//! candidates in its place. A tertiary page is never proposed, admitted or
//! fetched, so no profile value can cite one. The rule is a host list and a
//! link walk, never a judgment.
use std::collections::BTreeSet;
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

/// The external links a tertiary page's reference section cites, in order,
/// once each; the whole page's when it has no such heading.
pub fn cited(markdown: &str) -> Vec<String> {
    static HEADING: OnceLock<Regex> = OnceLock::new();
    static LINK: OnceLock<Regex> = OnceLock::new();
    let heading = HEADING.get_or_init(|| {
        Regex::new(r"(?im)^#{1,6}\s*(references|notes|citations|sources|footnotes|bibliography|further reading)\b")
            .expect("heading regex")
    });
    let link = LINK.get_or_init(|| Regex::new(r"\]\((https?://[^)\s]+)").expect("link regex"));
    let from = heading.find(markdown).map_or(0, |m| m.start());
    let mut seen = BTreeSet::new();
    link.captures_iter(&markdown[from..])
        .map(|c| c[1].to_string())
        .filter(|url| !is_tertiary(url) && !under(url, &MACHINERY))
        .filter(|url| seen.insert(url.clone()))
        .collect()
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
        let primary = cited(&page.markdown).into_iter().take(PER_LEAD);
        out.extend(primary.map(|url| SearchHit {
            title: url.clone(),
            snippet: format!("Cited by {} ({}).", hit.title, hit.url),
            url,
        }));
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

    #[test]
    fn the_references_section_yields_its_primary_links_only() {
        let page = "Intro [other](https://en.wikipedia.org/wiki/Oak) [early](https://example.test/early)\n\
            ## References\n\
            1. [Silvics](https://research.fs.usda.gov/silvics/beech)\n\
            2. [doi](https://doi.org/10.1/x) [again](https://doi.org/10.1/x)\n\
            3. [Wikidata](https://www.wikidata.org/wiki/Q1) [talk](https://en.wikipedia.org/wiki/Talk)\n";
        assert_eq!(
            cited(page),
            [
                "https://research.fs.usda.gov/silvics/beech",
                "https://doi.org/10.1/x"
            ]
        );
    }
}
