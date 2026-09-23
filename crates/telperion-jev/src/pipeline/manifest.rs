//! The manifest: the human boundary. A person admits it; the stages read it
//! and never edit it. The one exception is a source the pipeline admits
//! itself (fn-129, `pipeline::admission`): only the `sources` list grows,
//! each new entry with its rights class. Every proxy, composition, value
//! table, engineering value and version the run uses is stated here or is a
//! decision.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::canon::{canonical_sha256, file_sha256, read_json, CanonError};
use super::curve::{BelowFirstRow, GouldCoefficients};
use super::routes::{RelationLevel, ValueTable};

/// The version new manifests are written at. Version 1 still loads; from
/// version 2 the requirements table binds the manifest's coverage.
pub const MANIFEST_SCHEMA_VERSION: u32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Taxon {
    pub scientific_name: String,
    pub common_name: String,
    pub rank: String,
    #[serde(default)]
    pub cultivar: Option<String>,
}

/// An admitted table inside a source: which markdown table, the label of the
/// row that opens its block when one markdown table packs several species,
/// how many age-indexed rows a person counted, and which columns carry the
/// dimension.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdmittedTable {
    pub id: String,
    pub table_index: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<String>,
    pub expected_rows: usize,
    pub dimension: String,
    pub unit: String,
    pub value_column: usize,
    pub condition: String,
    pub taxon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Source {
    pub id: String,
    pub url: String,
    pub title: String,
    #[serde(default)]
    pub sha256: Option<String>,
    pub rights: String,
    /// The rights class the pipeline recorded when it admitted the source
    /// itself (`open-licence` or `public-cite-only`); absent on a source a
    /// person admitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rights_class: Option<String>,
    #[serde(default)]
    pub tables: Vec<AdmittedTable>,
}

/// The bar a field must clear at the data-quality gate.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
#[repr(usize)]
pub enum Sufficiency {
    None,
    ProxyOnly,
    Partial,
    Sufficient,
}

impl Sufficiency {
    pub const LEVELS: [Sufficiency; 4] = [
        Sufficiency::None,
        Sufficiency::ProxyOnly,
        Sufficiency::Partial,
        Sufficiency::Sufficient,
    ];

    pub fn from_index(index: usize) -> Sufficiency {
        Self::LEVELS[index.min(3)]
    }

    /// The level's key, the same table the sufficiency question set scores.
    pub fn key(self) -> &'static str {
        super::sets::SUFFICIENCY_LEVELS[self as usize]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Proxy {
    pub taxon: String,
    pub source: String,
}

/// One evidence field the literature must supply.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field {
    pub field: String,
    pub condition: String,
    #[serde(default)]
    pub required_ages_years: Vec<f64>,
    pub bar: Sufficiency,
    #[serde(default)]
    pub proxy: Option<Proxy>,
    /// The selection question for this field, as `jev select` takes it.
    pub question: String,
}

/// A composition choice naming admitted tables by id; the fit stage
/// substitutes the rows the fetch stage parsed and builds the curve module's
/// composition from them.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "method", rename_all = "kebab-case")]
pub enum CompositionSpec {
    Table {
        table: String,
        below_first_row: BelowFirstRow,
    },
    ScaledTable {
        table: String,
        factor: f64,
        below_first_row: BelowFirstRow,
    },
    GouldIntegration {
        anchor_table: String,
        anchor_age_years: f64,
        coefficients: GouldCoefficients,
        site_index_m: f64,
        max_age_years: f64,
        below_anchor: BelowFirstRow,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Curves {
    pub reference_ages_years: [f64; 3],
    pub tolerance_percent: f64,
    pub envelope_height_m: f64,
    /// The direct build's measured mature trunk diameter, an engineering
    /// value a person records with its measurement in `engineering`.
    pub mature_dbh_m: f64,
    pub height: CompositionSpec,
    pub dbh: CompositionSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Described {
    pub trait_name: String,
    pub sources: Vec<String>,
    pub table: ValueTable,
}

/// An appearance trait the literature must describe. Its level table is the
/// requirements table's; select scores it and code copies the level's ranges
/// into the profile, and nothing renders it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Appearance {
    pub trait_name: String,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transfer {
    pub dial: String,
    pub levels: Vec<RelationLevel>,
    pub forbid_other_growth_form: bool,
    #[serde(default)]
    pub target_range: Option<[f64; 2]>,
    #[serde(default)]
    pub measured_metric: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Engineering {
    pub value: Value,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Versions {
    pub question_sets: BTreeMap<String, u32>,
    pub tools: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Manifest {
    pub schema: String,
    pub schema_version: u32,
    pub species: String,
    pub taxon: Taxon,
    pub context: String,
    pub growth_form: String,
    pub preset: String,
    pub profile_id: String,
    pub seed: u32,
    pub sources: Vec<Source>,
    pub fields: Vec<Field>,
    #[serde(default)]
    pub curves: Option<Curves>,
    #[serde(default)]
    pub described: Vec<Described>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub appearance: Vec<Appearance>,
    #[serde(default)]
    pub transfers: Vec<Transfer>,
    #[serde(default)]
    pub engineering: BTreeMap<String, Engineering>,
    pub versions: Versions,
    pub model: String,
}

/// A loaded manifest with the checksum of its bytes on disk.
#[derive(Debug, Clone)]
pub struct Admitted {
    pub manifest: Manifest,
    pub sha256: String,
}

#[derive(Debug)]
pub enum ManifestError {
    File(CanonError),
    Invalid(String),
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(err) => write!(f, "manifest: {err}"),
            Self::Invalid(msg) => write!(f, "manifest: {msg}"),
        }
    }
}

impl std::error::Error for ManifestError {}

pub fn load(path: &Path) -> Result<Admitted, ManifestError> {
    let value = read_json(path).map_err(ManifestError::File)?;
    let manifest: Manifest =
        serde_json::from_value(value).map_err(|e| ManifestError::Invalid(e.to_string()))?;
    validate(&manifest)?;
    let sha256 = file_sha256(path).map_err(ManifestError::File)?;
    Ok(Admitted { manifest, sha256 })
}

pub fn validate(m: &Manifest) -> Result<(), ManifestError> {
    let bad = |msg: String| Err(ManifestError::Invalid(msg));
    if m.schema != "manifest" || !(1..=MANIFEST_SCHEMA_VERSION).contains(&m.schema_version) {
        return bad(format!(
            "schema must be manifest version 1 to {MANIFEST_SCHEMA_VERSION}"
        ));
    }
    if m.species.is_empty() || m.species != m.species.to_lowercase() {
        return bad("species must be a lowercase stable id".into());
    }
    let mut ids = std::collections::BTreeSet::new();
    for source in &m.sources {
        if !ids.insert(source.id.as_str()) {
            return bad(format!("duplicate source id {}", source.id));
        }
        if source.rights.trim().is_empty() {
            return bad(format!("source {} has an empty rights field", source.id));
        }
    }
    for field in &m.fields {
        // A mature size (fn-127) is judged on a stated mature value, at no age.
        if field.required_ages_years.is_empty() && !super::requirements::is_mature(m, &field.field)
        {
            return bad(format!("field {} names no required age", field.field));
        }
        if let Some(proxy) = &field.proxy {
            if !ids.contains(proxy.source.as_str()) {
                return bad(format!(
                    "field {} names proxy source {} which is not admitted",
                    field.field, proxy.source
                ));
            }
        }
    }
    for described in &m.described {
        for id in &described.sources {
            if !ids.contains(id.as_str()) {
                return bad(format!(
                    "described trait {} names source {} which is not admitted",
                    described.trait_name, id
                ));
            }
        }
        if described.table.levels.is_empty() {
            return bad(format!(
                "described trait {} names a value table with no level",
                described.trait_name
            ));
        }
    }
    for appearance in &m.appearance {
        for id in &appearance.sources {
            if !ids.contains(id.as_str()) {
                return bad(format!(
                    "appearance trait {} names source {} which is not admitted",
                    appearance.trait_name, id
                ));
            }
        }
    }
    let short = super::requirements::shortfalls(m);
    if !short.is_empty() {
        return bad(format!(
            "{} growth form {} falls short of the requirements table: {}",
            m.species,
            m.growth_form,
            short.join("; ")
        ));
    }
    for (dial, engineering) in &m.engineering {
        if engineering.rationale.trim().is_empty() {
            return bad(format!("engineering parameter {dial} has no rationale"));
        }
    }
    Ok(())
}

/// The checksum of what a person seeds before discovery: the species, the
/// taxon and the evidence fields with their conditions and required ages.
/// Discovery keys on it, so admitting sources, curves or engineering rows
/// does not rerun it; a seed edit does.
pub fn seed_sha256(m: &Manifest) -> String {
    let fields: Vec<Value> = m
        .fields
        .iter()
        .map(|f| {
            json!({
                "field": f.field,
                "condition": f.condition,
                "required_ages_years": f.required_ages_years,
            })
        })
        .collect();
    canonical_sha256(&json!({"species": m.species, "taxon": m.taxon, "fields": fields}))
}

impl Manifest {
    pub fn source(&self, id: &str) -> Option<&Source> {
        self.sources.iter().find(|s| s.id == id)
    }

    pub fn field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.field == name)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use serde_json::json;

    pub(crate) fn minimal() -> Value {
        json!({
            "schema": "manifest", "schema_version": 1,
            "species": "oregon-white-oak",
            "taxon": {"scientific_name": "Quercus garryana", "common_name": "Oregon white oak", "rank": "species"},
            "context": "mature open-grown", "growth_form": "broadleaf",
            "preset": "oregon-white-oak", "profile_id": "oregon-white-oak", "seed": 7,
            "sources": [{"id": "S1", "url": "https://example.test/s1", "title": "Silvics", "rights": "public domain"}],
            "fields": [{"field": "height_m", "condition": "open_grown", "required_ages_years": [10, 25], "bar": "partial", "question": "What height does a tree reach at a stated age?"}],
            "versions": {"question_sets": {"screen": 1}, "tools": {"species-pipeline": "0.1.0"}},
            "model": "jev-latest"
        })
    }

    #[test]
    fn a_minimal_manifest_loads_with_its_file_checksum() {
        let dir = std::env::temp_dir().join(format!("jev-manifest-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("manifest.json");
        std::fs::write(&path, serde_json::to_vec(&minimal()).unwrap()).unwrap();
        let admitted = load(&path).unwrap();
        assert_eq!(admitted.manifest.species, "oregon-white-oak");
        assert_eq!(admitted.sha256, file_sha256(&path).unwrap());
        assert_eq!(
            admitted.manifest.field("height_m").unwrap().bar,
            Sufficiency::Partial
        );
    }

    #[test]
    fn validation_names_the_broken_entry() {
        let cases: Vec<(&str, Value, &str)> = vec![
            (
                "empty rights",
                json!(["sources", 0, "rights"]),
                "empty rights",
            ),
            (
                "no ages",
                json!(["fields", 0, "required_ages_years"]),
                "no required age",
            ),
            (
                "unknown proxy",
                json!(["fields", 0, "proxy"]),
                "not admitted",
            ),
        ];
        for (name, pointer, expect) in cases {
            let mut value = minimal();
            let path = pointer.as_array().unwrap();
            let target = value.pointer_mut(&format!(
                "/{}/{}/{}",
                path[0].as_str().unwrap(),
                path[1],
                path[2].as_str().unwrap()
            ));
            match name {
                "empty rights" => *target.unwrap() = json!(" "),
                "no ages" => *target.unwrap() = json!([]),
                _ => {
                    value["fields"][0]["proxy"] = json!({"taxon": "Quercus robur", "source": "E9"})
                }
            }
            let manifest: Manifest = serde_json::from_value(value).unwrap();
            let err = validate(&manifest).unwrap_err().to_string();
            assert!(err.contains(expect), "{name}: {err}");
        }
    }

    #[test]
    fn the_seed_checksum_ignores_admission_and_follows_a_seed_edit() {
        let seed: Manifest = serde_json::from_value(minimal()).unwrap();
        let mut admitted = seed.clone();
        admitted.sources.push(Source {
            id: "E1".into(),
            url: "https://example.test/e1".into(),
            title: "Yield table".into(),
            sha256: None,
            rights: "cited".into(),
            rights_class: None,
            tables: vec![],
        });
        admitted.fields[0].bar = Sufficiency::ProxyOnly;
        admitted.engineering.insert(
            "curves.mature_dbh_m".into(),
            Engineering {
                value: json!(0.55),
                rationale: "midpoint".into(),
            },
        );
        assert_eq!(seed_sha256(&seed), seed_sha256(&admitted));
        let mut edited = seed.clone();
        edited.fields[0].required_ages_years.push(50.0);
        assert_ne!(seed_sha256(&seed), seed_sha256(&edited));
    }

    #[test]
    fn sufficiency_levels_order_none_below_sufficient() {
        assert!(Sufficiency::None < Sufficiency::ProxyOnly);
        assert!(Sufficiency::Partial < Sufficiency::Sufficient);
        assert_eq!(Sufficiency::from_index(9), Sufficiency::Sufficient);
        assert_eq!(Sufficiency::from_index(1).key(), "proxy_only");
    }
}
