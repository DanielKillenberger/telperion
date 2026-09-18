//! What the repository already knows before any search: the sources every
//! admitted manifest under the evidence tree names, with the dimensions their
//! tables cover and any fetch error their run recorded, and the URLs the
//! specs' `## Resolved via Research` sections cite. Discovery lists them as
//! candidates Jev ranks like searched ones, with their origin marked; nothing
//! is fetched here and nothing is admitted by being known.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};

use super::canon::read_json;
use super::decision::read_decisions;
use super::manifest::Manifest;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KnownSource {
    pub url: String,
    pub title: String,
    pub snippet: String,
    /// `manifest:<path>#<source id>` or `spec:<spec id>`.
    pub origin: String,
    /// The dimensions the source's admitted tables cover; empty when the
    /// source is named for every field.
    pub dimensions: Vec<String>,
    /// The fetch error the owning run recorded for it, when one did.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct KnownSources {
    pub sources: Vec<KnownSource>,
}

impl KnownSources {
    /// Every manifest under `<flow>/evidence` other than the run's own, and
    /// every research URL under `<flow>/specs`, one entry per URL.
    pub fn scan(flow: &Path, own_manifest: &Path) -> Self {
        let own = fs::canonicalize(own_manifest).ok();
        let mut manifests = Vec::new();
        collect_manifests(&flow.join("evidence"), &mut manifests);
        manifests.sort();
        let mut by_url: BTreeMap<String, KnownSource> = BTreeMap::new();
        for path in manifests {
            if fs::canonicalize(&path).ok() == own {
                continue;
            }
            for source in manifest_sources(&path) {
                merge(&mut by_url, source);
            }
        }
        for source in spec_sources(&flow.join("specs")) {
            merge(&mut by_url, source);
        }
        Self {
            sources: by_url.into_values().collect(),
        }
    }

    /// The sources named for `field`: those whose tables cover it and those
    /// named for every field.
    pub fn for_field(&self, field: &str) -> Vec<&KnownSource> {
        self.sources
            .iter()
            .filter(|s| s.dimensions.is_empty() || s.dimensions.iter().any(|d| d == field))
            .collect()
    }
}

fn collect_manifests(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_manifests(&path, out);
        } else if path.file_name().is_some_and(|name| name == "manifest.json") {
            out.push(path);
        }
    }
}

/// The sources one admitted manifest names, with the fetch errors the
/// decisions beside it recorded.
fn manifest_sources(path: &Path) -> Vec<KnownSource> {
    let Ok(value) = read_json(path) else {
        return Vec::new();
    };
    let Ok(manifest) = serde_json::from_value::<Manifest>(value) else {
        return Vec::new();
    };
    let errors: BTreeMap<String, String> = path
        .parent()
        .map(|dir| dir.join("decisions.json"))
        .and_then(|decisions| read_decisions(&decisions).ok())
        .unwrap_or_default()
        .into_iter()
        .filter(|d| d.kind == "unavailable-source")
        .filter_map(|d| {
            let source = d.payload["source"].as_str()?.to_string();
            let error = d.payload["error"].as_str()?.to_string();
            Some((source, error))
        })
        .collect();
    manifest
        .sources
        .iter()
        .map(|source| {
            let mut dimensions: Vec<String> =
                source.tables.iter().map(|t| t.dimension.clone()).collect();
            dimensions.sort();
            dimensions.dedup();
            let covers = if dimensions.is_empty() {
                String::new()
            } else {
                format!(" with tables for {}", dimensions.join(", "))
            };
            KnownSource {
                url: source.url.clone(),
                title: source.title.clone(),
                snippet: format!(
                    "Admitted by the {} manifest as {}{covers}.",
                    manifest.species, source.id
                ),
                origin: format!("manifest:{}#{}", path.display(), source.id),
                dimensions,
                error: errors.get(&source.id).cloned(),
            }
        })
        .collect()
}

/// The URLs every spec cites under `## Resolved via Research`.
fn spec_sources(dir: &Path) -> Vec<KnownSource> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let url = Regex::new(r#"https?://[^\s<>()\[\]"']+"#).expect("a url pattern");
    let mut paths: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
        .collect();
    paths.sort();
    let mut out = Vec::new();
    for path in paths {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let spec = path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        for found in url.find_iter(&research_section(&text)) {
            out.push(KnownSource {
                url: found
                    .as_str()
                    .trim_end_matches(['.', ',', ';', ':'])
                    .to_string(),
                title: spec.clone(),
                snippet: format!("Cited in the research section of spec {spec}."),
                origin: format!("spec:{spec}"),
                dimensions: Vec::new(),
                error: None,
            });
        }
    }
    out
}

/// The text under `## Resolved via Research`, up to the next `## ` heading.
fn research_section(text: &str) -> String {
    let mut inside = false;
    let mut out = String::new();
    for line in text.lines() {
        if line.starts_with("## ") {
            inside = line.trim_start_matches("## ").trim() == "Resolved via Research";
            continue;
        }
        if inside {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

/// One entry per URL: the first title and snippet stand, the dimensions
/// widen (any entry named for every field keeps it so), an error stands.
fn merge(by_url: &mut BTreeMap<String, KnownSource>, source: KnownSource) {
    match by_url.get_mut(&source.url) {
        None => {
            by_url.insert(source.url.clone(), source);
        }
        Some(existing) => {
            if existing.dimensions.is_empty() || source.dimensions.is_empty() {
                existing.dimensions.clear();
            } else {
                for dimension in source.dimensions {
                    if !existing.dimensions.contains(&dimension) {
                        existing.dimensions.push(dimension);
                    }
                }
                existing.dimensions.sort();
            }
            if existing.error.is_none() {
                existing.error = source.error;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_research_section_ends_at_the_next_heading_and_urls_lose_trailing_punctuation() {
        let text = "# Spec\n\n## Resolved via Research\n\nSee https://example.test/a, and https://example.test/b.\n\n## Boundaries\n\nNot https://example.test/c\n";
        let dir = std::env::temp_dir().join(format!("jev-known-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("fn-1-spec.md"), text).unwrap();
        let sources = spec_sources(&dir);
        let urls: Vec<&str> = sources.iter().map(|s| s.url.as_str()).collect();
        assert_eq!(
            urls,
            vec!["https://example.test/a", "https://example.test/b"]
        );
        assert_eq!(sources[0].origin, "spec:fn-1-spec");
        assert!(sources[0].dimensions.is_empty());
    }

    #[test]
    fn merging_widens_dimensions_and_keeps_an_error() {
        let mut by_url = BTreeMap::new();
        let known = |dimensions: &[&str], error: Option<&str>| KnownSource {
            url: "https://example.test/e1".into(),
            title: "E1".into(),
            snippet: String::new(),
            origin: "manifest:x#E1".into(),
            dimensions: dimensions.iter().map(|d| d.to_string()).collect(),
            error: error.map(str::to_string),
        };
        merge(&mut by_url, known(&["height_m"], None));
        merge(&mut by_url, known(&["dbh_m"], Some("tls")));
        let merged = &by_url["https://example.test/e1"];
        assert_eq!(merged.dimensions, vec!["dbh_m", "height_m"]);
        assert_eq!(merged.error.as_deref(), Some("tls"));
        merge(&mut by_url, known(&[], None));
        assert!(by_url["https://example.test/e1"].dimensions.is_empty());
    }
}
