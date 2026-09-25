//! Reference photographs, found by the Profile stage (owner, 2026-09-25,
//! option A): no run from a name needs a person to supply photographs.
//!
//! Code collects candidates from the admitted open-licence sources' pages
//! and from Wikimedia Commons, at most `MAX_CANDIDATES`. Jev classes each
//! Commons file's licence statements with the rights question set, and only
//! an open licence goes on. Code downloads the rest and pins each by its
//! bytes, then one look (`screen`) keeps those that show a mature,
//! open-grown whole tree of the species, or its bark: up to two in leaf,
//! one bare and one bark close-up. The kept ones are appended to
//! `packet/references.json` with their source, attribution and sha256; a
//! rerun never erases a recorded reference (fn-142) and finds nothing once
//! `ENOUGH` are recorded.
use std::io::Read;
use std::path::PathBuf;
use std::sync::OnceLock;

use regex::Regex;
use serde_json::{json, Value};

use super::canon::{read_json, write_canonical};
use super::judge::Judge;
use super::manifest::{self, Manifest};
use super::rights::{self, host, OPEN_LICENCE};
use super::stage::Paths;
use crate::tuning::evaluation::Image;

pub mod commons;
pub mod screen;

pub use screen::{Screen, Verdict, Vision};

/// Candidates looked at, all in one batch.
pub const MAX_CANDIDATES: usize = 12;
/// Candidates taken from the admitted sources' own pages.
const FROM_SOURCES: usize = 4;
/// Recorded references that make a search unnecessary.
pub const ENOUGH: usize = 2;
/// Kept per view: in leaf, bare, bark.
const PER_VIEW: [(&str, usize); 3] = [("leaf-on", 2), ("bare", 1), ("bark", 1)];

#[derive(Debug, Clone, PartialEq)]
pub enum Origin {
    Commons,
    /// An admitted source's page, by its id.
    Source(String),
}

/// One photograph a search found: its page, the image, who made it and the
/// licence statements the rights question reads.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub page: String,
    pub image: String,
    pub title: String,
    pub attribution: String,
    pub statements: Vec<String>,
    pub origin: Origin,
}

/// A plain GET: the bytes and the content type.
pub trait Web {
    fn get(&self, url: &str) -> Result<(Vec<u8>, String), String>;
}

pub struct Http;

impl Web for Http {
    fn get(&self, url: &str) -> Result<(Vec<u8>, String), String> {
        let response = ureq::get(url)
            .set(
                "User-Agent",
                "telperion-jev/0.1 (species runner reference search)",
            )
            .call()
            .map_err(|e| format!("{url}: {e}"))?;
        let kind = response.content_type().to_string();
        let mut bytes = Vec::new();
        response
            .into_reader()
            .take(20 << 20)
            .read_to_end(&mut bytes)
            .map_err(|e| format!("{url}: {e}"))?;
        Ok((bytes, kind))
    }
}

/// Finds, checks, screens and records reference photographs; the words say
/// what it found and what it spent.
pub fn find(
    paths: &Paths,
    web: &dyn Web,
    judge: &Judge<'_>,
    look: &dyn Screen,
) -> Result<String, String> {
    let path = paths.packet("references");
    let mut doc = read_json(&path).unwrap_or_else(|_| json!({"references": []}));
    let recorded = doc["references"].as_array().map_or(0, Vec::len);
    if recorded >= ENOUGH {
        return Ok(format!("{recorded} reference photographs recorded"));
    }
    let admitted = manifest::load(&paths.manifest()).map_err(|e| e.to_string())?;
    let m = &admitted.manifest;
    let mut candidates = from_sources(paths, m);
    candidates.extend(commons::candidates(web, &m.taxon.scientific_name));
    candidates.truncate(MAX_CANDIDATES);
    let mut report = json!({"candidates": candidates.len(), "rights_calls": 0});
    let mut open = Vec::new();
    for candidate in candidates {
        let class = match candidate.origin {
            Origin::Source(_) => OPEN_LICENCE.to_string(),
            Origin::Commons => {
                let state = json!({"source": {"url": candidate.page, "host": host(&candidate.page),
                    "title": candidate.title}, "lines": candidate.statements, "records": []});
                report["rights_calls"] = json!(report["rights_calls"].as_u64().unwrap_or(0) + 1);
                rights::classify(judge, &state)
                    .map_err(|e| e.to_string())?
                    .class
            }
        };
        if class == OPEN_LICENCE {
            open.push(candidate);
        }
    }
    let (images, fetched) = download(paths, web, open);
    report["open_licence_downloaded"] = json!(images.len());
    let kept = match images.is_empty() {
        true => Vec::new(),
        false => {
            let (verdicts, usage) = look.screen(&m.taxon.scientific_name, &images)?;
            report["vision_calls"] = json!(1);
            report["usage"] = usage;
            choose(&verdicts)
        }
    };
    let list = doc["references"]
        .as_array_mut()
        .ok_or("references is no list")?;
    for (at, view) in &kept {
        let record = record(m, list, &fetched[*at], &images[*at], view);
        list.push(record);
    }
    write_canonical(&path, &doc).map_err(|e| e.to_string())?;
    report["kept"] = json!(kept.len());
    write_canonical(&dir(paths).join("find.json"), &report).map_err(|e| e.to_string())?;
    Ok(format!(
        "reference photographs: {} candidates, {} open-licence, {} kept",
        report["candidates"],
        report["open_licence_downloaded"],
        kept.len()
    ))
}

/// Where the downloaded copies live, named by their bytes.
pub fn dir(paths: &Paths) -> PathBuf {
    paths.cache().join("photos")
}

/// The local copy of a recorded reference, if the run downloaded it.
pub fn copy(paths: &Paths, sha256: &str) -> Option<PathBuf> {
    ["jpg", "png"]
        .into_iter()
        .map(|ext| dir(paths).join(format!("{sha256}.{ext}")))
        .find(|p| p.exists())
}

/// Images on the pages of the admitted open-licence sources.
fn from_sources(paths: &Paths, m: &Manifest) -> Vec<Candidate> {
    static IMAGE: OnceLock<Regex> = OnceLock::new();
    let image = IMAGE.get_or_init(|| {
        Regex::new(r"(?i)!\[([^\]]*)\]\((https?://[^)\s]+\.(?:jpe?g|png))\)").expect("image regex")
    });
    let fetch = read_json(&paths.artifact("fetch")).unwrap_or_default();
    let mut out = Vec::new();
    for source in m
        .sources
        .iter()
        .filter(|s| s.rights_class.as_deref() == Some(OPEN_LICENCE))
    {
        let name = fetch["body"]["sources"][&source.id]["cached"]["markdown"].as_str();
        let Some(text) = name.and_then(|n| std::fs::read_to_string(paths.cache().join(n)).ok())
        else {
            continue;
        };
        out.extend(image.captures_iter(&text).take(2).map(|c| Candidate {
            page: source.url.clone(),
            image: c[2].to_string(),
            title: c[1].to_string(),
            attribution: format!("{} ({})", source.title, source.url),
            statements: vec![],
            origin: Origin::Source(source.id.clone()),
        }));
    }
    out.truncate(FROM_SOURCES);
    out
}

/// Downloads each candidate that is a JPEG or a PNG by its bytes.
fn download(paths: &Paths, web: &dyn Web, open: Vec<Candidate>) -> (Vec<Image>, Vec<Candidate>) {
    let (mut images, mut kept) = (Vec::new(), Vec::new());
    for candidate in open {
        let Ok((bytes, _)) = web.get(&candidate.image) else {
            continue;
        };
        let ext = match bytes.get(..4) {
            Some([0x89, b'P', b'N', b'G']) => "png",
            Some([0xFF, 0xD8, 0xFF, _]) => "jpg",
            _ => continue,
        };
        let sha256 = crate::sha256_hex(&bytes);
        let path = dir(paths).join(format!("{sha256}.{ext}"));
        if std::fs::create_dir_all(dir(paths))
            .and_then(|_| std::fs::write(&path, &bytes))
            .is_err()
        {
            continue;
        }
        images.push(Image {
            path,
            sha256,
            view: "candidate".into(),
            seed: 0,
        });
        kept.push(candidate);
    }
    (images, kept)
}

/// The photographs kept, by index and view: up to two in leaf, one bare
/// and one bark close-up, in the order they were found.
pub fn choose(verdicts: &[Verdict]) -> Vec<(usize, String)> {
    let mut out: Vec<(usize, String)> = Vec::new();
    for (at, verdict) in verdicts.iter().enumerate() {
        let Some(view) = verdict.kept_view() else {
            continue;
        };
        let cap = PER_VIEW
            .iter()
            .find(|(v, _)| *v == view)
            .map_or(0, |(_, n)| *n);
        if out.iter().filter(|(_, v)| v == view).count() < cap {
            out.push((at, view.to_string()));
        }
    }
    out
}

/// A kept photograph's record, its source a new `R<n>` for a Commons file.
fn record(m: &Manifest, list: &[Value], found: &Candidate, image: &Image, view: &str) -> Value {
    let taken = |id: &str| {
        m.sources.iter().any(|s| s.id == id) || list.iter().any(|r| r["source_id"] == id)
    };
    let source_id = match &found.origin {
        Origin::Source(id) => id.clone(),
        Origin::Commons => (1..)
            .map(|n| format!("R{n}"))
            .find(|id| !taken(id))
            .unwrap_or_default(),
    };
    let licence = match found.statements.is_empty() {
        true => format!("the open-licence source {source_id}"),
        false => found.statements.join("; "),
    };
    json!({
        "id": format!("photo-{}", list.len() + 1), "species_id": m.species, "kind": "real",
        "source_id": source_id, "url": found.page, "asset_sha256": image.sha256,
        "attribution": found.attribution, "kept": false, "matching": "qualitative",
        "accessed_at": &crate::pipeline::stage::now()[..10], "view": view,
        "scale": [if view == "bark" { "bark" } else { "whole" }],
        "usage": format!("{licence}; attribution recorded; local copy in the run cache, not redistributed"),
        "context": format!("Found by the Profile stage (fn-149): a {view} view the look kept."),
        "limitations": ["Uncalibrated photograph; no measured height, crown fraction or age.",
                        "Kept by one look for species, maturity, open growth and framing; qualitative evidence only."],
        "age_years": null, "dimensions_m": null,
    })
}
