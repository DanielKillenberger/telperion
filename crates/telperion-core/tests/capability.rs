//! The declared capability vocabulary, and the preset derivation beside it.

use std::collections::BTreeSet;

use telperion_core::capability::{self, Support, DERIVABLE, EXPRESSED, UNEXPRESSED};
use telperion_core::presets::Preset;

/// Every preset a caller can name.
const PRESETS: &[&str] = &[
    "ordinary",
    "oregon-white-oak",
    "norway-spruce",
    "european-beech",
    "silver-birch",
    "date-palm",
    "telperion",
    "laurelin",
];

fn declared() -> Vec<&'static capability::Capability> {
    EXPRESSED.iter().chain(UNEXPRESSED).collect()
}

#[test]
fn every_name_is_declared_once_and_carries_one_line_of_meaning() {
    let mut seen = BTreeSet::new();
    for entry in declared() {
        assert!(
            seen.insert(entry.name),
            "{} is declared twice; a name belongs to one list once",
            entry.name
        );
        assert!(
            !entry.name.is_empty() && !entry.meaning.is_empty(),
            "{} has no meaning written beside it",
            entry.name
        );
        assert!(
            !entry.meaning.contains('\n'),
            "{}'s meaning is more than one line",
            entry.name
        );
        assert!(
            entry
                .name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c == '-'),
            "{} is not in the register the other names are written in",
            entry.name
        );
    }
}

#[test]
fn the_nine_names_the_derivation_produces_are_all_in_the_vocabulary() {
    assert_eq!(DERIVABLE.len(), 9);
    for name in DERIVABLE {
        assert_eq!(
            capability::support(name),
            Support::Expressed,
            "{name} is a name the derivation produces, so the generator expresses it"
        );
    }
}

#[test]
fn the_derivation_produces_no_name_the_vocabulary_does_not_carry() {
    let mut produced = BTreeSet::new();
    for id in PRESETS {
        let preset = Preset::from_id(id).unwrap();
        for name in capability::derived(preset) {
            assert!(
                DERIVABLE.contains(&name),
                "{id} derives {name}, which is outside the derivable set"
            );
            produced.insert(name);
        }
    }
    let derivable: BTreeSet<&str> = DERIVABLE.iter().copied().collect();
    assert_eq!(
        produced, derivable,
        "the presets between them produce every derivable name"
    );
}

/// The shipped presets' own value tables, as they read today. A value move
/// that changes one of these lines changes what the preset claims to draw.
#[test]
fn the_shipped_presets_derive_what_they_derive_today() {
    let derived = |id: &str| capability::derived(Preset::from_id(id).unwrap());
    assert_eq!(
        derived("oregon-white-oak"),
        ["woody-axes", "lobed-blade", "alternate-petiole"]
    );
    assert_eq!(
        derived("norway-spruce"),
        [
            "woody-axes",
            "four-sided-needle",
            "radial-peg",
            "tiered-secondary"
        ]
    );
    assert_eq!(
        derived("european-beech"),
        ["woody-axes", "alternate-petiole"]
    );
    assert_eq!(
        derived("silver-birch"),
        [
            "woody-axes",
            "lobed-blade",
            "alternate-petiole",
            "tiered-secondary"
        ]
    );
}

#[test]
fn the_date_palms_recorded_needs_read_as_three_met_and_three_absent() {
    // The list `.flow/evidence/date-palm/pipeline/manifest.json` recorded on
    // 2026-09-19, which the gate then read as six missing capabilities. The
    // rosette and the frond are drawn from fn-109 on, so the gate halts on the
    // trunk organs and the infructescence alone.
    for name in ["woody-axes", "apical-rosette", "pinnate-frond"] {
        assert_eq!(capability::support(name), Support::Expressed, "{name}");
    }
    for name in ["acanthophyll", "persistent-leaf-base", "infructescence"] {
        assert_eq!(capability::support(name), Support::Absent, "{name}");
    }
}

/// The palm's own value table is what produces the two names the gate reads,
/// and the grouping it draws is the compound leaf the vocabulary already had a
/// word for.
#[test]
fn the_palms_table_produces_the_rosette_the_frond_and_the_compound_leaf() {
    let derived = capability::derived(Preset::from_id("date-palm").unwrap());
    for name in ["apical-rosette", "pinnate-frond", "pinnate-compound"] {
        assert!(
            derived.contains(&name),
            "the date palm derives {name}: {derived:?}"
        );
    }
}

#[test]
fn a_name_nobody_has_declared_is_unrecognised() {
    assert_eq!(capability::support("woody-axe"), Support::Unrecognised);
    assert_eq!(capability::support(""), Support::Unrecognised);
}

/// A shipped species never reads as unrecognised: every name a catalogue
/// packet requires is one the vocabulary carries, met or absent.
#[test]
fn the_catalogue_requires_no_name_the_vocabulary_lacks() {
    let catalogue = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalogue");
    let mut seen = 0;
    for entry in std::fs::read_dir(&catalogue).unwrap().flatten() {
        let path = entry.path().join("packet").join("species.json");
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let species: serde_json::Value = serde_json::from_str(&text).unwrap();
        for name in species["required_capabilities"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|name| name.as_str())
        {
            assert_ne!(
                capability::support(name),
                Support::Unrecognised,
                "{} requires {name}, which no list declares",
                path.display()
            );
            seen += 1;
        }
    }
    assert!(seen > 0, "no catalogue species was read from {catalogue:?}");
}

#[test]
fn the_version_is_a_digest_of_the_declared_names() {
    let version = capability::version();
    assert!(
        version.starts_with('v') && version.len() == 17,
        "the version reads as a digest: {version}"
    );
    assert_eq!(version, capability::version(), "and it is stable");
}

#[test]
fn the_printed_vocabulary_carries_the_version_and_every_declared_name() {
    let printed = capability::vocabulary();
    assert_eq!(printed["vocabulary_version"], capability::version());
    for (key, list) in [("expressed", EXPRESSED), ("unexpressed", UNEXPRESSED)] {
        let entries = printed[key].as_array().expect("a list of entries");
        assert_eq!(entries.len(), list.len(), "{key} is printed whole");
        for (entry, declared) in entries.iter().zip(list) {
            assert_eq!(entry["name"], declared.name);
            assert_eq!(entry["meaning"], declared.meaning);
        }
    }
}
