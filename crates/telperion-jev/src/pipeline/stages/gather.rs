//! Gather (fn-157): everything written about the species, found by one
//! broad search and read later without asking leave.
//!
//! The documents are what the repository already knows about the species
//! (`pipeline::known`), then the web search and the research index over a
//! fixed set of plain-word queries that name the kinds of document the
//! literature lives in: floras, silvics and forestry manuals, arboreta,
//! extension pages and papers. A tertiary encyclopedic page is a lead: the
//! primary sources it cites stand in its place (`pipeline::leads`). Every
//! document found, one per address, up to `MAX_DOCUMENTS`, joins the
//! manifest as a source. A seed that states the species' native range adds
//! that range's floras and forestry literature, and the species under its
//! names in the range's own languages, `PER_NATIVE_QUERY` documents a query
//! beyond the rest (host decision, fn-157); nothing is ranked, admitted or refused before it is
//! read. Reading a document for its facts needs no licence: only a copy
//! kept in the catalogue does, and the copy is decided from the page's own
//! licence statements once it is fetched (`fetch`, the document stage).
//! The stage keys on the seed (species, taxon, fields), so adding the
//! documents to the manifest does not rerun it.

use std::collections::BTreeSet;

use serde_json::{json, Value};

use crate::pipeline::adapter::{AdapterError, FetchAdapter, SearchHit};
use crate::pipeline::canon::write_canonical;
use crate::pipeline::cost::Cost;
use crate::pipeline::known::KnownSources;
use crate::pipeline::leads;
use crate::pipeline::manifest::{seed_sha256, Manifest, Source, Taxon};
use crate::pipeline::stage::{Context, Paths, StageError};

use super::inputs;

pub const STAGE: &str = "gather";
/// Documents the broad search adds per species (host decision, fn-157).
pub const MAX_DOCUMENTS: usize = 16;
/// Documents each native-range query adds beyond them.
pub const PER_NATIVE_QUERY: usize = 2;
const HITS_PER_QUERY: usize = 8;
/// The rights line of a gathered document until its page is read.
pub const GATHERED: &str =
    "read for its facts and cited; a copy is kept only under the page's own open licence (fn-157)";

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { documents: usize },
}

/// The native-range queries a seed's range asks for: the region's floras and
/// forestry literature, and the species under each of its own names.
pub fn native_queries(taxon: &Taxon) -> Vec<String> {
    let Some(range) = &taxon.native_range else {
        return Vec::new();
    };
    let sci = &taxon.scientific_name;
    let mut out = vec![
        format!("{sci} flora {}", range.region),
        format!("{sci} forestry silviculture {}", range.region),
    ];
    out.extend(range.names.values().map(|name| format!("{name} {sci}")));
    out
}

/// The web queries and the research query, in plain words.
pub fn queries(scientific: &str, common: &str) -> (Vec<String>, String) {
    let web = vec![
        scientific.to_string(),
        format!("{scientific} flora description height leaves"),
        format!("{scientific} silvics forestry manual"),
        format!("{scientific} arboretum tree size"),
        format!("{common} tree extension fact sheet"),
    ];
    (web, format!("{scientific} tree height diameter crown"))
}

pub fn run(
    paths: &Paths,
    adapter: &dyn FetchAdapter,
    known: &KnownSources,
) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(paths, STAGE)?;
    let manifest = &ctx.admitted.manifest;
    let seed = seed_sha256(manifest);
    let mut header = ctx.header_keyed(STAGE, "gather", inputs(&[]), vec![], &seed);
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let before = adapter.spent();
    let failed = |err: AdapterError| StageError::Failed {
        stage: STAGE.into(),
        reason: err.to_string(),
    };
    let taxon = &manifest.taxon;
    let (web, research) = queries(&taxon.scientific_name, &taxon.common_name);
    let mut hits: Vec<Value> = known_hits(manifest, known);
    for query in &web {
        let found = adapter.search(query, HITS_PER_QUERY).map_err(failed)?;
        add(&mut hits, leads::follow(adapter, found), "web", query);
    }
    let found = adapter
        .research(&research, HITS_PER_QUERY)
        .map_err(failed)?;
    add(
        &mut hits,
        leads::follow(adapter, found),
        "research",
        &research,
    );
    let room = MAX_DOCUMENTS.saturating_sub(manifest.sources.len());
    let mut grown = manifest.clone();
    grown.sources.extend(documents(&grown, &hits, room));
    // The native-range literature is added beyond the broad search's
    // documents, so a search that found only horticultural pages still
    // reads the floras and manuals of the range.
    let native = native_queries(taxon);
    for query in &native {
        let found = adapter.search(query, HITS_PER_QUERY).map_err(failed)?;
        let mut these = Vec::new();
        add(&mut these, leads::follow(adapter, found), "native", query);
        // Chosen against the sources, not every hit seen: a flora the broad
        // search found past its cap is still read when the range asks for it.
        grown
            .sources
            .extend(documents(&grown, &these, PER_NATIVE_QUERY));
        for hit in these {
            if !hits.iter().any(|h| h["url"] == hit["url"]) {
                hits.push(hit);
            }
        }
    }
    let count = grown.sources.len() - manifest.sources.len();
    write_canonical(
        &ctx.paths.manifest(),
        &serde_json::to_value(&grown).expect("manifest serializes"),
    )?;
    header.cost = Cost::from_spent(&adapter.spent().since(&before));
    let mut all = web;
    all.push(research);
    all.extend(native);
    ctx.write(&header, json!({"queries": all, "hits": hits}))?;
    Ok(Outcome::Ran { documents: count })
}

/// What the repository already knows about the species, first.
fn known_hits(manifest: &Manifest, known: &KnownSources) -> Vec<Value> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for field in &manifest.fields {
        for source in known.for_field(&field.field) {
            if leads::is_tertiary(&source.url) || !seen.insert(source.url.clone()) {
                continue;
            }
            let mut hit = json!({"url": source.url, "title": source.title,
                "snippet": source.snippet, "kind": "known", "origin": source.origin});
            if let Some(error) = &source.error {
                hit["error"] = json!(error);
            }
            out.push(hit);
        }
    }
    out
}

/// Appends each hit whose address is new, with where it was found.
fn add(out: &mut Vec<Value>, found: Vec<SearchHit>, kind: &str, query: &str) {
    for hit in found {
        if out.iter().any(|h| h["url"] == json!(hit.url)) {
            continue;
        }
        out.push(
            json!({"url": hit.url, "title": hit.title, "snippet": hit.snippet,
            "kind": kind, "query": query}),
        );
    }
}

/// At most `room` new sources from `hits`, in the order found, numbered
/// after the manifest's: never an address the manifest holds, a tertiary
/// page or one whose earlier fetch failed.
fn documents(manifest: &Manifest, hits: &[Value], room: usize) -> Vec<Source> {
    let mut next = manifest.sources.len() + 1;
    let mut out = Vec::new();
    for hit in hits {
        if out.len() >= room {
            break;
        }
        let url = hit["url"].as_str().unwrap_or_default();
        let taken = manifest.sources.iter().any(|s| s.url == url);
        if taken || leads::is_tertiary(url) || hit.get("error").is_some() || url.is_empty() {
            continue;
        }
        while manifest.source(&format!("P{next}")).is_some() {
            next += 1;
        }
        out.push(Source {
            id: format!("P{next}"),
            url: url.into(),
            title: hit["title"].as_str().unwrap_or(url).into(),
            sha256: None,
            rights: GATHERED.into(),
            rights_class: None,
            tables: vec![],
        });
        next += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_queries_name_the_kinds_of_document_in_plain_words() {
        let (web, research) = queries("Fagus sylvatica", "European beech");
        assert_eq!(web[0], "Fagus sylvatica");
        assert!(web.iter().any(|q| q.contains("silvics")));
        assert!(web.iter().all(|q| !q.contains('_')));
        assert_eq!(research, "Fagus sylvatica tree height diameter crown");
    }

    #[test]
    fn a_native_range_asks_its_floras_and_its_own_names() {
        let mut taxon: Taxon = serde_json::from_value(serde_json::json!({
            "scientific_name": "Fagus sylvatica", "common_name": "European beech", "rank": "species"
        }))
        .unwrap();
        assert!(native_queries(&taxon).is_empty());
        taxon.native_range = Some(crate::pipeline::manifest::NativeRange {
            region: "Europe".into(),
            names: [("de".to_string(), "Rotbuche".to_string())].into(),
        });
        assert_eq!(
            native_queries(&taxon),
            [
                "Fagus sylvatica flora Europe",
                "Fagus sylvatica forestry silviculture Europe",
                "Rotbuche Fagus sylvatica"
            ]
        );
    }
}
