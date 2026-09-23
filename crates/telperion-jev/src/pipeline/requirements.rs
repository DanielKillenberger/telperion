//! The requirements table (fn-118): per growth form, the evidence fields a
//! manifest must ask the literature for with the lowest bar each must reach,
//! and the appearance traits it must describe. Each appearance level maps, by
//! code, to a range per material field it feeds, so a described level becomes
//! a value range code owns and copies. The table binds a manifest at schema
//! version 2 and later; an earlier manifest loads unchanged.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::Deserialize;

use super::manifest::{Manifest, Sufficiency};
use super::sets::DescribedLevel;

pub const REQUIREMENTS_JSON: &str = include_str!("../../data/species-requirements.json");
/// The first manifest schema version the table binds.
pub const REQUIRED_FROM_VERSION: u32 = 2;

#[derive(Debug, Clone, Deserialize)]
pub struct FieldTerms {
    /// Words a sentence must carry to count as a point for the field, each
    /// matched as a whole word or its plural (fn-131).
    pub terms: Vec<String>,
    /// The screen kinds a row must carry to count for the field (fn-131):
    /// an organ size by its organ class, a named cultivar's size toward the
    /// species. Empty keeps the tree-size kinds.
    #[serde(default)]
    pub kinds: Vec<String>,
    /// How the field is asked when a growth form names no other way.
    #[serde(default)]
    pub asked: Asked,
}

/// How a field's sufficiency is asked (fn-127, fn-132).
#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Asked {
    /// Measured sizes at the manifest's required ages.
    #[default]
    Age,
    /// A stated mature value or range, at no age.
    Mature,
    /// A stated growth rate, a size per year, at no age.
    Rate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppearanceLevel {
    pub key: String,
    pub summary: String,
    /// Material field -> `[low, high]`, one entry per field the trait feeds.
    pub ranges: BTreeMap<String, [f64; 2]>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppearanceTable {
    /// Words a source sentence must carry to be judged for the trait (fn-128).
    pub terms: Vec<String>,
    pub feeds: Vec<String>,
    pub levels: Vec<AppearanceLevel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GrowthForm {
    pub fields: BTreeMap<String, Sufficiency>,
    /// Fields this form asks otherwise than the field's own way (fn-132).
    #[serde(default)]
    pub asked: BTreeMap<String, Asked>,
    pub appearance: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Requirements {
    pub version: u32,
    pub fields: BTreeMap<String, FieldTerms>,
    pub appearance: BTreeMap<String, AppearanceTable>,
    pub growth_forms: BTreeMap<String, GrowthForm>,
}

/// The shipped table, parsed once.
pub fn table() -> &'static Requirements {
    static TABLE: OnceLock<Requirements> = OnceLock::new();
    TABLE.get_or_init(|| {
        serde_json::from_str(REQUIREMENTS_JSON).expect("species-requirements.json parses")
    })
}

impl Requirements {
    /// The level table of one appearance trait as the described question set
    /// takes it; the no-match level is appended there.
    pub fn levels(&self, trait_name: &str) -> Option<Vec<DescribedLevel>> {
        self.appearance.get(trait_name).map(|t| {
            t.levels
                .iter()
                .map(|l| DescribedLevel {
                    key: l.key.clone(),
                    summary: l.summary.clone(),
                })
                .collect()
        })
    }

    pub fn level(&self, trait_name: &str, key: &str) -> Option<&AppearanceLevel> {
        self.appearance
            .get(trait_name)?
            .levels
            .iter()
            .find(|l| l.key == key)
    }
}

fn bound(m: &Manifest) -> Option<&'static GrowthForm> {
    (m.schema_version >= REQUIRED_FROM_VERSION)
        .then(|| table().growth_forms.get(&m.growth_form))
        .flatten()
}

/// The table's bar for `field` when the manifest is bound by the table and
/// its growth form requires the field.
pub fn required_bar(m: &Manifest, field: &str) -> Option<Sufficiency> {
    bound(m).and_then(|form| form.fields.get(field).copied())
}

/// How the table asks `field` of a manifest at version 2 or later: its
/// growth form's way, else the field's own; a legacy manifest asks every
/// field at its ages.
pub fn asked(m: &Manifest, field: &str) -> Asked {
    if m.schema_version < REQUIRED_FROM_VERSION {
        return Asked::Age;
    }
    let table = table();
    let own = table.fields.get(field).map_or(Asked::Age, |f| f.asked);
    table
        .growth_forms
        .get(&m.growth_form)
        .and_then(|form| form.asked.get(field).copied())
        .unwrap_or(own)
}

/// Whether the manifest's growth form requires `trait_name` described.
pub fn requires_appearance(m: &Manifest, trait_name: &str) -> bool {
    bound(m).is_some_and(|form| form.appearance.iter().any(|t| t == trait_name))
}

/// Every way the manifest falls short of the table, each naming its field
/// or trait; empty when it covers the table or predates it.
pub fn shortfalls(m: &Manifest) -> Vec<String> {
    if m.schema_version < REQUIRED_FROM_VERSION {
        return Vec::new();
    }
    let table = table();
    let Some(form) = table.growth_forms.get(&m.growth_form) else {
        return vec![format!(
            "growth form {} has no row in the requirements table (rows: {})",
            m.growth_form,
            table
                .growth_forms
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        )];
    };
    let mut short = Vec::new();
    for (name, bar) in &form.fields {
        match m.field(name) {
            None => short.push(format!("field {name} is missing (bar {})", bar.key())),
            Some(field) if field.bar < *bar => short.push(format!(
                "field {name} is listed at {}, below the table's bar {}",
                field.bar.key(),
                bar.key()
            )),
            Some(_) => {}
        }
    }
    for name in &form.appearance {
        if !m.appearance.iter().any(|a| &a.trait_name == name) {
            short.push(format!("appearance trait {name} is missing"));
        }
    }
    for listed in &m.appearance {
        if !table.appearance.contains_key(&listed.trait_name) {
            short.push(format!(
                "appearance trait {} has no level table in the requirements table",
                listed.trait_name
            ));
        }
    }
    short
}

/// Words a sentence must carry to be about `field`; `None` passes every
/// sentence.
pub fn terms(field: &str) -> Option<&'static [String]> {
    table().fields.get(field).map(|f| f.terms.as_slice())
}

/// The screen kinds that count for `field`; the tree-size kinds when the
/// table names none.
pub fn kinds(field: &str) -> Vec<&'static str> {
    const TREE: [&str; 2] = ["measured_size_at_age", "mature_size_range"];
    match table().fields.get(field) {
        Some(f) if !f.kinds.is_empty() => f.kinds.iter().map(String::as_str).collect(),
        _ => TREE.to_vec(),
    }
}

/// Whether `sentence` is about `field`: it carries one of the field's terms
/// as whole words, a term's last word also in its plural, so "leaf" never
/// matches "leaflet". A field with no word list passes every sentence.
pub fn names_field(field: &str, sentence: &str) -> bool {
    let Some(list) = terms(field) else {
        return true;
    };
    let words: Vec<String> = sentence
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_lowercase)
        .collect();
    list.iter().any(|term| {
        let wanted: Vec<&str> = term.split_whitespace().collect();
        let last = wanted.len().saturating_sub(1);
        words.windows(wanted.len().max(1)).any(|window| {
            window
                .iter()
                .zip(&wanted)
                .enumerate()
                .all(|(i, (word, want))| word == want || (i == last && plural_of(word, want)))
        })
    })
}

fn plural_of(word: &str, term: &str) -> bool {
    word.strip_prefix(term)
        .is_some_and(|rest| rest == "s" || rest == "es")
}
#[cfg(test)]
mod tests {
    use super::*;

    /// The bounds `MaterialParams::validate` holds each fed field to.
    fn material_bounds(field: &str) -> Option<[f64; 2]> {
        let colour = ["bark_", "leaf_front_", "leaf_back_"]
            .iter()
            .any(|p| field.starts_with(p))
            && ["_red", "_green", "_blue"]
                .iter()
                .any(|s| field.ends_with(s));
        match field {
            _ if colour => Some([0.0, 1.0]),
            "bark_roughness" => Some([0.0, 1.0]),
            "hue_range_low" | "hue_range_high" => Some([-0.5, 0.5]),
            "brightness_range_low" | "brightness_range_high" => Some([-1.0, 1.0]),
            _ => None,
        }
    }

    #[test]
    fn the_table_covers_the_three_growth_forms_and_every_name_it_uses() {
        let table = table();
        assert_eq!(table.version, 2);
        for form in ["broadleaf", "conifer", "palm"] {
            let row = &table.growth_forms[form];
            assert!(row.fields.contains_key("height_m"), "{form}");
            assert!(row.fields.contains_key("dbh_m"), "{form}");
            assert!(row.fields.contains_key("crown_width_m"), "{form}");
            assert!(
                row.fields.values().all(|b| *b > Sufficiency::None),
                "{form}"
            );
            for field in row.fields.keys() {
                assert!(terms(field).is_some_and(|t| !t.is_empty()), "{field}");
            }
            for name in &row.appearance {
                assert!(table.appearance.contains_key(name), "{form}: {name}");
            }
        }
        for name in ["frond_length_m", "leaflet_length_m", "leaflet_width_m"] {
            assert!(table.growth_forms["palm"].fields.contains_key(name));
        }
    }

    /// A palm manifest at version 2 that lists every field and trait the
    /// table requires, at the table's bars.
    pub(crate) fn covered_palm() -> serde_json::Value {
        let mut value = crate::pipeline::manifest::tests::minimal();
        let form = &table().growth_forms["palm"];
        value["schema_version"] = serde_json::json!(2);
        value["growth_form"] = serde_json::json!("palm");
        value["fields"] = form
            .fields
            .iter()
            .map(|(name, bar)| serde_json::json!({"field": name, "condition": "open_grown", "required_ages_years": [30], "bar": bar.key(), "question": "?"}))
            .collect();
        value["appearance"] = form
            .appearance
            .iter()
            .map(|name| serde_json::json!({"trait_name": name, "sources": ["S1"]}))
            .collect();
        value
    }

    fn refusal(value: serde_json::Value) -> Option<String> {
        let manifest: Manifest = serde_json::from_value(value).unwrap();
        crate::pipeline::manifest::validate(&manifest)
            .err()
            .map(|e| e.to_string())
    }

    fn palm_manifest() -> Manifest {
        serde_json::from_value(covered_palm()).unwrap()
    }

    #[test]
    fn a_bound_manifest_reads_its_bars_and_a_legacy_one_reads_none() {
        let palm = palm_manifest();
        assert_eq!(required_bar(&palm, "height_m"), Some(Sufficiency::Partial));
        assert_eq!(required_bar(&palm, "age_years"), None);
        assert!(requires_appearance(&palm, "bark_colour"));
        let mut legacy = palm;
        legacy.schema_version = 1;
        assert_eq!(required_bar(&legacy, "height_m"), None);
        assert!(!requires_appearance(&legacy, "bark_colour"));
    }

    #[test]
    fn admission_refuses_a_short_manifest_naming_every_missing_field() {
        assert_eq!(refusal(covered_palm()), None);
        let mut short = covered_palm();
        short["fields"].as_array_mut().unwrap().retain(|f| {
            !["crown_width_m", "frond_length_m"].contains(&f["field"].as_str().unwrap())
        });
        short["fields"][0]["bar"] = serde_json::json!("none");
        let first = short["fields"][0]["field"].as_str().unwrap().to_string();
        short["appearance"]
            .as_array_mut()
            .unwrap()
            .retain(|a| a["trait_name"] != "bark_colour");
        let err = refusal(short).unwrap();
        for expect in [
            "field crown_width_m is missing".to_string(),
            "field frond_length_m is missing".to_string(),
            format!("field {first} is listed at none, below the table's bar"),
            "appearance trait bark_colour is missing".to_string(),
        ] {
            assert!(err.contains(&expect), "{expect}: {err}");
        }
        let mut unknown = covered_palm();
        unknown["growth_form"] = serde_json::json!("cycad");
        assert!(refusal(unknown)
            .unwrap()
            .contains("growth form cycad has no row"));
        // A manifest that predates the table loads as it always did.
        let mut legacy = covered_palm();
        legacy["schema_version"] = serde_json::json!(1);
        legacy["fields"].as_array_mut().unwrap().truncate(1);
        assert_eq!(refusal(legacy), None);
    }

    #[test]
    fn every_appearance_level_maps_each_fed_field_to_a_range_the_material_accepts() {
        for (name, trait_) in &table().appearance {
            assert!(!trait_.levels.is_empty(), "{name}");
            for level in &trait_.levels {
                let fed: Vec<&String> = level.ranges.keys().collect();
                let mut feeds: Vec<&String> = trait_.feeds.iter().collect();
                feeds.sort();
                assert_eq!(fed, feeds, "{name}/{}", level.key);
                for (field, [lo, hi]) in &level.ranges {
                    let [min, max] = material_bounds(field)
                        .unwrap_or_else(|| panic!("{name}: {field} is no MaterialParams field"));
                    assert!(min <= *lo && lo <= hi && *hi <= max, "{name}/{field}");
                }
            }
        }
    }
}
