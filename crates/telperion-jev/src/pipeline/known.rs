//! What the repository already knows before any search: the sources the
//! catalogue's own bibliographies hold, the sources every admitted manifest
//! under the evidence tree names, with the dimensions their tables cover and
//! any fetch error their run recorded, and the URLs the specs'
//! `## Resolved via Research` sections cite. Discovery lists them as
//! candidates Jev ranks like searched ones, with their origin marked; nothing
//! is fetched here and nothing is admitted by being known.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};

use super::canon::read_json;
use super::decision::{read_decisions, Status};
use super::manifest::Manifest;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KnownSource {
    pub url: String,
    pub title: String,
    pub snippet: String,
    /// `catalogue:<species id>#<source id>`, `manifest:<path>#<source id>`
    /// or `spec:<spec id>`.
    pub origin: String,
    /// The dimensions the source's admitted tables cover; empty when the
    /// source is named for every field.
    pub dimensions: Vec<String>,
    /// The fetch error the owning run recorded for it, when one did.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// How many research URLs from the specs join one field's candidate list.
pub const SPEC_URLS_PER_FIELD: usize = 8;

/// The `.flow` directory a run directory sits under: the nearest ancestor
/// named `.flow`, so the scan never depends on the process's working
/// directory. A run directory outside any `.flow` tree yields `.flow`
/// relative to the working directory, as the runbook runs from the root.
pub fn flow_root(dir: &Path) -> PathBuf {
    dir.ancestors()
        .find(|p| p.file_name().is_some_and(|name| name == ".flow"))
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from(".flow"))
}

#[derive(Debug, Clone, Default)]
pub struct KnownSources {
    pub sources: Vec<KnownSource>,
}

impl KnownSources {
    /// Every species bibliography under `catalogue`, every manifest under
    /// `<flow>/evidence` other than the run's own, and every research URL
    /// under `<flow>/specs`, one entry per URL. A catalogue that is absent or
    /// unreadable yields nothing and the rest is scanned as before.
    pub fn scan(catalogue: &Path, flow: &Path, own_manifest: &Path) -> Self {
        let own = fs::canonicalize(own_manifest).ok();
        let mut manifests = Vec::new();
        collect_manifests(&flow.join("evidence"), &mut manifests);
        manifests.sort();
        let mut by_url: BTreeMap<String, KnownSource> = BTreeMap::new();
        for source in catalogue_sources(catalogue) {
            merge(&mut by_url, source);
        }
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

    /// The sources named for `field`: every catalogued source and every
    /// manifest source whose tables cover it or that is named for every
    /// field, then at most `SPEC_URLS_PER_FIELD` research URLs from the
    /// specs, in URL order. The cap keeps the list Jev ranks from growing
    /// with the spec count.
    pub fn for_field(&self, field: &str) -> Vec<&KnownSource> {
        let named =
            |s: &&KnownSource| s.dimensions.is_empty() || s.dimensions.iter().any(|d| d == field);
        let recorded = |s: &&KnownSource| {
            s.origin.starts_with("catalogue:") || s.origin.starts_with("manifest:")
        };
        let from_records = self.sources.iter().filter(recorded).filter(named);
        let from_specs = self
            .sources
            .iter()
            .filter(|s| !recorded(s))
            .filter(named)
            .take(SPEC_URLS_PER_FIELD);
        from_records.chain(from_specs).collect()
    }
}

/// Every source the catalogue's species bibliographies hold, in folder order.
/// A folder without a readable `sources.json` is skipped, so a catalogue that
/// is absent or half-written never stops discovery.
fn catalogue_sources(catalogue: &Path) -> Vec<KnownSource> {
    let Ok(entries) = fs::read_dir(catalogue) else {
        return Vec::new();
    };
    let mut folders: Vec<PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
    folders.sort();
    let mut out = Vec::new();
    for folder in folders {
        let Ok(value) = read_json(&folder.join("sources.json")) else {
            continue;
        };
        let species = folder
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        for source in value["sources"].as_array().into_iter().flatten() {
            let (Some(url), Some(title)) = (source["url"].as_str(), source["title"].as_str())
            else {
                continue;
            };
            let id = source["id"].as_str().unwrap_or("?");
            let mut dimensions: Vec<String> = source["tables"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|table| table["dimension"].as_str().map(str::to_string))
                .collect();
            dimensions.sort();
            dimensions.dedup();
            let covers = if dimensions.is_empty() {
                String::new()
            } else {
                format!(" with tables for {}", dimensions.join(", "))
            };
            out.push(KnownSource {
                url: url.to_string(),
                title: title.to_string(),
                snippet: format!(
                    "Held by the {species} catalogue as {id}{covers}. {}",
                    source["use"].as_str().unwrap_or("").trim()
                )
                .trim_end()
                .to_string(),
                origin: format!("catalogue:{species}#{id}"),
                dimensions,
                error: None,
            });
        }
    }
    out
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
        // An error stands only while its decision is open: a source retried,
        // replaced or dropped since is not listed with a stale failure.
        .filter(|d| d.kind == "unavailable-source" && d.status == Status::Open)
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
    fn a_catalogued_source_is_known_by_its_species_folder_and_the_tables_it_holds() {
        let catalogue = std::env::temp_dir().join(format!(
            "jev-catalogue-{}-{}",
            std::process::id(),
            crate::ledger::new_entry_id()
        ));
        let species = catalogue.join("european-ash");
        fs::create_dir_all(&species).unwrap();
        fs::write(
            species.join("sources.json"),
            serde_json::to_vec(&serde_json::json!({
                "schema": "sources", "schema_version": 1,
                "sources": [{
                    "id": "E1", "url": "https://example.test/ertragstafeln",
                    "title": "Ertragstafeln", "use": "Stand height by age.",
                    "tables": [{"dimension": "height_m"}],
                }, {
                    "id": "J1", "url": "https://example.test/atlas", "title": "Atlas",
                    "use": "Species context.", "tables": [],
                }],
            }))
            .unwrap(),
        )
        .unwrap();
        // A folder without a bibliography is skipped, never an error.
        fs::create_dir_all(catalogue.join("half-written")).unwrap();

        let sources = catalogue_sources(&catalogue);
        assert_eq!(sources.len(), 2, "{sources:?}");
        assert_eq!(sources[0].origin, "catalogue:european-ash#E1");
        assert_eq!(sources[0].dimensions, vec!["height_m"]);
        assert!(sources[0].snippet.contains("Stand height by age."));
        assert!(sources[1].dimensions.is_empty());
        assert!(sources[0].error.is_none());
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

#[cfg(test)]
mod root_tests {
    use super::flow_root;
    use std::path::{Path, PathBuf};

    #[test]
    fn the_flow_root_is_the_nearest_dot_flow_ancestor() {
        assert_eq!(
            flow_root(Path::new("/repo/.flow/evidence/european-ash/pipeline")),
            PathBuf::from("/repo/.flow")
        );
        assert_eq!(
            flow_root(Path::new("/tmp/scratch/run")),
            PathBuf::from(".flow")
        );
    }
}
