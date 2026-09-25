//! What the repository already knows about this species before any search:
//! the sources its catalogue bibliography holds and the sources every
//! admitted manifest of the same species or taxon under the evidence tree
//! names, with the dimensions their tables cover and any fetch error their
//! run recorded. Discovery lists them as candidates Jev ranks like searched
//! ones, with their origin marked; nothing is fetched here and nothing is
//! admitted by being known.
//!
//! Only the same species is known (owner, 2026-09-25): the beech's second
//! proof run ranked spruce and oak validation sources and fn-11's
//! growth-model papers first for every field. Specs' method references are
//! never sources, and raw evidence (`raw/`, a proof run's scratch) is never
//! read.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::canon::read_json;
use super::decision::{read_decisions, Status};
use super::manifest::Manifest;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KnownSource {
    pub url: String,
    pub title: String,
    pub snippet: String,
    /// `catalogue:<species id>#<source id>` or `manifest:<path>#<source id>`.
    pub origin: String,
    /// The dimensions the source's admitted tables cover; empty when the
    /// source is named for every field.
    pub dimensions: Vec<String>,
    /// The fetch error the owning run recorded for it, when one did.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

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
    /// The bibliography of the run's own species under `catalogue` and every
    /// manifest of the same species or taxon under `<flow>/evidence` other
    /// than the run's own and outside any `raw/` directory, one entry per
    /// URL. A run whose manifest cannot be read knows nothing; a catalogue
    /// that is absent or unreadable yields nothing and the rest is scanned.
    pub fn scan(catalogue: &Path, flow: &Path, own_manifest: &Path) -> Self {
        let Some(own) = load(own_manifest) else {
            return Self::default();
        };
        let same = |m: &Manifest| {
            m.species == own.species
                || m.taxon
                    .scientific_name
                    .eq_ignore_ascii_case(&own.taxon.scientific_name)
        };
        let own_path = fs::canonicalize(own_manifest).ok();
        let mut manifests = Vec::new();
        collect_manifests(&flow.join("evidence"), &mut manifests);
        manifests.sort();
        let mut by_url: BTreeMap<String, KnownSource> = BTreeMap::new();
        for source in catalogue_sources(catalogue, &own.species) {
            merge(&mut by_url, source);
        }
        for path in manifests {
            if fs::canonicalize(&path).ok() == own_path {
                continue;
            }
            let Some(manifest) = load(&path).filter(|m| same(m)) else {
                continue;
            };
            for source in manifest_sources(&path, &manifest) {
                merge(&mut by_url, source);
            }
        }
        Self {
            sources: by_url.into_values().collect(),
        }
    }

    /// The sources named for `field`: every one whose tables cover it or
    /// that is named for every field.
    pub fn for_field(&self, field: &str) -> Vec<&KnownSource> {
        self.sources
            .iter()
            .filter(|s| s.dimensions.is_empty() || s.dimensions.iter().any(|d| d == field))
            .collect()
    }
}

fn load(path: &Path) -> Option<Manifest> {
    serde_json::from_value(read_json(path).ok()?).ok()
}

/// Every source the species' own catalogue bibliography holds. A folder
/// without a readable `sources.json` yields nothing, so a catalogue that is
/// absent or half-written never stops discovery.
fn catalogue_sources(catalogue: &Path, species: &str) -> Vec<KnownSource> {
    let Ok(value) = read_json(&catalogue.join(species).join("sources.json")) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for source in value["sources"].as_array().into_iter().flatten() {
        let (Some(url), Some(title)) = (source["url"].as_str(), source["title"].as_str()) else {
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
    out
}

fn collect_manifests(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && entry.file_name() != "raw" {
            collect_manifests(&path, out);
        } else if path.file_name().is_some_and(|name| name == "manifest.json") {
            out.push(path);
        }
    }
}

/// The sources one admitted manifest names, with the fetch errors the
/// decisions beside it recorded.
fn manifest_sources(path: &Path, manifest: &Manifest) -> Vec<KnownSource> {
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

        let sources = catalogue_sources(&catalogue, "european-ash");
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
