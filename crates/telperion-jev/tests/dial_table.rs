//! The authored dial table, checked against the generator's own wire schema.
//!
//! Offline: no model is called, nothing is rendered and nothing is captured.
//! Every claim here is one the generator answers in process - which rows the
//! wire has, which values it accepts, what the table's bytes hash to.
use serde_json::Value;
use std::collections::BTreeMap;
use telperion_core::params;
use telperion_core::presets::Preset;
use telperion_jev::sha256_hex;
use telperion_jev::tuning::actions::{candidate, Action, Dial};

const TABLE: &str = include_str!("../data/dials.json");
const EXCLUDED: &str = include_str!("../data/dials.excluded.json");

/// The six rows the pilot qualified, exactly as the calibration manifests saw
/// them. Their hash is pinned below: a change to `Dial`'s shape that moved a
/// byte of it would unqualify every case the owner has already paid for.
const PILOT_TABLE: &str = r#"[
 {"id":"limbs","path":"/skeleton/habit/lateralsPerStation","meaning":"limbs born at each station","min":1,"max":4,"integer":true,"small":1,"substantial":2},
 {"id":"leaves","path":"/canopy/shortShootLeaves","meaning":"leaves per cluster","min":2,"max":12,"integer":true,"small":2,"substantial":4},
 {"id":"spacing","path":"/canopy/shortShootSpacing","meaning":"metres between leaf clusters","min":0.01,"max":0.08,"integer":false,"small":0.01,"substantial":0.02},
 {"id":"irregularity","path":"/skeleton/envelope/irregularity","meaning":"crown envelope lobes and hollows","min":0,"max":0.5,"integer":false,"small":0.08,"substantial":0.16},
 {"id":"crookedness","path":"/skeleton/habit/crookedness","meaning":"limb turning and zigzag amount","min":0,"max":15,"integer":false,"small":3,"substantial":6},
 {"id":"taper","path":"/skeleton/habit/twigTipTaper","meaning":"remaining wood thickness toward the crown edge; lower means thinner tips","min":0.05,"max":0.6,"integer":false,"small":0.1,"substantial":0.2}
]"#;
const PILOT_TABLE_SHA256: &str = "32a9af4ea8d920ca1ff711145a4ebbe5a7912fbd7311be6ef102693cc10c7802";

/// The rows whose value is a switch as much as a dial: the code that reads
/// them contributes nothing at zero, so a step off zero turns a feature on
/// rather than moving one. Each meaning has to say so, or a proposal that
/// raises two of them at once reads as two small steps and lands a new
/// feature. Read off the use sites, one row at a time; a row is on this list
/// only where the code guards on the value or multiplies by it.
const SWITCHES_AT_ZERO: [&str; 32] = [
    "attractor_weight",
    "stem_divergence",
    "stem_lean",
    "stem_fork_height",
    "gravitropism",
    "lean",
    "writhe_amplitude",
    "spiral_rate",
    "twig_sag",
    "twig_pendulous_variation",
    "twig_curtain_drop",
    "surface_lobe_depth",
    "canopy_shoot_radius",
    "rosette_fronds",
    "rachis_length",
    "leaf_bases",
    "leaf_base_length",
    "acanthophylls",
    "acanthophyll_length",
    "leaf_lobe_depth",
    "material_furrow_strength",
    "material_fissure_strength",
    "material_crest_strength",
    "material_plate_cell_scale",
    "material_plate_dome",
    "material_plate_edge_lift",
    "material_plate_identity",
    "material_weathering_strength",
    "material_orientation_strength",
    "material_directional_occlusion",
    "material_depth_strength",
    "material_peel_curl",
];

fn table() -> Vec<Dial> {
    serde_json::from_str(TABLE).expect("data/dials.json is a dial table")
}

fn excluded() -> BTreeMap<String, String> {
    serde_json::from_str(EXCLUDED).expect("data/dials.excluded.json is path -> reason")
}

/// Every family the table has to answer for: the shipped catalogue and the
/// tables still in work, which is what a tuning run is pointed at.
fn families() -> Vec<&'static str> {
    params::CATALOGUE
        .iter()
        .chain(params::IN_WORK)
        .map(|entry| entry.1)
        .collect()
}

fn wire(id: &str) -> Value {
    params::metadata(
        &Preset::from_id(id)
            .expect("catalogue identity")
            .parameters(),
    )
}

fn rows(value: &Value, at: &str, out: &mut Vec<(String, Value)>) {
    match value {
        Value::Object(map) => {
            for (key, inner) in map {
                rows(inner, &format!("{at}/{key}"), out);
            }
        }
        leaf => out.push((at.to_owned(), leaf.clone())),
    }
}

#[test]
fn every_numeric_row_of_every_family_is_a_dial_or_an_excluded_row() {
    let (dials, excluded) = (table(), excluded());
    let paths: Vec<&str> = dials.iter().map(|d| d.path.as_str()).collect();
    for (path, reason) in &excluded {
        assert!(!reason.is_empty(), "{path} is excluded without a reason");
        assert!(!paths.contains(&path.as_str()), "{path} is in both tables");
    }
    for id in families() {
        let family = wire(id);
        let mut found = Vec::new();
        rows(&family, "", &mut found);
        for (path, value) in &found {
            if value.is_number() {
                assert!(
                    paths.contains(&path.as_str()) || excluded.contains_key(path),
                    "{id}: {path} is a numeric row the table neither offers nor excludes"
                );
            }
        }
        for path in &paths {
            // A proposal batch reads every dial's current value off the wire
            // and refuses the whole batch when one of them is null, so a row
            // the wire leaves unset is not a dial the loop can offer.
            assert!(
                family.pointer(path).is_some_and(Value::is_number),
                "{id}: the table names {path}, which this family's wire leaves \
                 without a number"
            );
        }
    }
}

#[test]
fn every_authored_row_is_a_dial_the_loop_can_ask_about() {
    let dials = table();
    let mut ids = Vec::new();
    for dial in &dials {
        dial.validate().unwrap_or_else(|e| panic!("{e}"));
        assert!(!ids.contains(&dial.id), "duplicate dial id {}", dial.id);
        ids.push(dial.id.clone());
        let width = dial.max - dial.min;
        assert!(
            dial.small > 0. && dial.small < dial.substantial && dial.substantial <= width,
            "{}: steps {} and {} do not fit [{}, {}]",
            dial.id,
            dial.small,
            dial.substantial,
            dial.min,
            dial.max
        );
        assert!(
            dial.small <= width / 2.,
            "{}: a small step is half the range or more",
            dial.id
        );
        let group = dial.path.split('/').nth(1).unwrap();
        assert_eq!(dial.group.as_deref(), Some(group), "{}", dial.id);
        assert!(dial.score_visible.is_some(), "{}", dial.id);
        assert!(
            matches!(
                dial.meaning_basis.as_deref(),
                Some("doc comment" | "use site" | "prototype")
            ),
            "{}: meaning basis {:?}",
            dial.id,
            dial.meaning_basis
        );
        assert!(
            matches!(
                dial.range_basis.as_deref(),
                Some("validated bound" | "preset span" | "authored")
            ),
            "{}: range basis {:?}",
            dial.id,
            dial.range_basis
        );
        assert!(
            dial.source.as_deref().is_some_and(|s| s.contains(".rs:")),
            "{}: source {:?}",
            dial.id,
            dial.source
        );
    }
}

#[test]
fn every_dial_steps_to_a_value_the_generator_accepts() {
    let dials = table();
    let mut stuck = Vec::new();
    for id in families() {
        let family = wire(id);
        for dial in &dials {
            let Some(current) = family.pointer(&dial.path).and_then(Value::as_f64) else {
                stuck.push(format!("{id}: {} has no value on the wire", dial.id));
                continue;
            };
            let mut offered = 0;
            let mut refused = Vec::new();
            for action in [Action::SmallIncrease, Action::SmallDecrease] {
                if dial.value(current, action).is_err() {
                    continue;
                }
                offered += 1;
                if let Err(e) = candidate(id, &family, dial, action) {
                    refused.push(format!("{id}: {} {:?} {e}", dial.id, action));
                }
            }
            if offered == 0 {
                stuck.push(format!(
                    "{id}: {} sits at {current} in [{}, {}] and cannot step",
                    dial.id, dial.min, dial.max
                ));
                continue;
            }
            assert!(
                refused.len() < offered,
                "the generator refused every step of a dial: {refused:?}"
            );
            for note in refused {
                println!("one-sided dial: {note}");
            }
        }
    }
    for note in &stuck {
        println!("dial that cannot move here: {note}");
    }
}

#[test]
fn every_row_that_switches_a_feature_on_says_what_zero_does() {
    let dials = table();
    for id in SWITCHES_AT_ZERO {
        let dial = dials
            .iter()
            .find(|d| d.id == id)
            .unwrap_or_else(|| panic!("the table dropped the switching row {id}"));
        assert_eq!(dial.min, 0., "{id}: a switching row's floor is not zero");
        assert!(
            dial.meaning.contains("zero"),
            "{id}: the meaning does not say what zero does: {}",
            dial.meaning
        );
    }
}

#[test]
fn the_six_rows_the_pilot_qualified_still_hash_to_what_it_paid_for() {
    let pilot: Vec<Dial> = serde_json::from_str(PILOT_TABLE).expect("pilot table");
    assert_eq!(
        sha256_hex(&serde_json::to_vec(&pilot).unwrap()),
        PILOT_TABLE_SHA256,
        "the dial table's shape changed under the calibrated manifests"
    );
    let authored = table();
    for dial in &pilot {
        let now = authored
            .iter()
            .find(|d| d.id == dial.id)
            .unwrap_or_else(|| panic!("the table dropped the pilot row {}", dial.id));
        assert_eq!(
            (
                &now.path,
                &now.meaning,
                now.min,
                now.max,
                now.integer,
                now.small,
                now.substantial
            ),
            (
                &dial.path,
                &dial.meaning,
                dial.min,
                dial.max,
                dial.integer,
                dial.small,
                dial.substantial
            ),
            "the table restated the pilot row {}",
            dial.id
        );
    }
}
