//! The parameter catalogue changes how rows are declared, never what a family
//! is. Four digests hold that, each over the text of what it reads:
//!
//! - the family: every shipped and in-work preset as its table builds it and
//!   as its identity serves it;
//! - the walk: walks between the presets at points inside the walk;
//! - the overrides: families whose growth overrides are stated on one side,
//!   both or neither, read and written through the wire;
//! - the refusals: what the wire and `Family::validate` answer for a value off
//!   every row's rail, so every refusal the family's own check makes is pinned.
//!
//! A digest that moves means a change moved a value, a walk, an override or a
//! refusal. A re-pin names its spec and what moved in its commit; the history
//! is `git log -- crates/telperion-core/tests/catalogue_identity.rs`.
use telperion_core::{blend, params, presets::Preset, Family};

const PRESETS: [Preset; 8] = [
    Preset::Ordinary,
    Preset::OregonWhiteOak,
    Preset::NorwaySpruce,
    Preset::EuropeanBeech,
    Preset::SilverBirch,
    Preset::DatePalm,
    Preset::Telperion,
    Preset::Laurelin,
];

/// FNV-1a over the text, enough to tell two runs of the same code apart.
fn digest(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
    })
}

fn family(text: &mut String, f: &Family) {
    text.push_str(&format!("{f:?}\n"));
}

#[test]
fn every_preset_is_the_family_it_was() {
    let mut text = String::new();
    for preset in PRESETS {
        family(&mut text, &preset.parameters());
    }
    for (_, id, _, _) in params::CATALOGUE.iter().chain(params::IN_WORK) {
        text.push_str(&format!("{id}: {:?}\n", params::by_identity(id)));
    }
    assert_eq!(
        digest(&text),
        5_297_028_612_543_002_410,
        "{}",
        digest(&text)
    );
}

/// Families with the growth overrides stated on neither side, one or both.
fn overridden() -> Vec<Family> {
    let mut stated = Preset::OregonWhiteOak.parameters();
    let g = &mut stated.skeleton.growth;
    (g.influence_radius, g.kill_distance, g.step_distance) = (Some(3.0), Some(0.4), Some(0.3));
    (g.trunk_height, g.max_turn_per_step, g.max_nodes) = (Some(2.5), Some(20.0), Some(90_000));
    let mut half = Preset::NorwaySpruce.parameters();
    half.skeleton.growth.kill_distance = Some(0.2);
    half.skeleton.growth.max_nodes = Some(120_000);
    vec![stated, half, Preset::SilverBirch.parameters()]
}

#[test]
fn every_walk_is_the_walk_it_was() {
    let mut text = String::new();
    let mut ends: Vec<Family> = PRESETS.iter().map(|p| p.parameters()).collect();
    ends.extend(overridden());
    // Every pair's walk at once (fn-271), its lines in the pairs' order.
    let lines: Vec<String> = std::thread::scope(|scope| {
        let pairs: Vec<_> = ends
            .iter()
            .enumerate()
            .flat_map(|(i, a)| ends[i + 1..].iter().map(move |b| (a, b)))
            .map(|(a, b)| {
                scope.spawn(move || {
                    [0.0, 0.13, 0.5, 0.731, 1.0]
                        .map(|t| format!("{:?}\n", blend::families(a, b, t)))
                        .concat()
                })
            })
            .collect();
        pairs.into_iter().map(|pair| pair.join().unwrap()).collect()
    });
    for line in lines {
        text.push_str(&line);
    }
    assert_eq!(
        digest(&text),
        16_166_154_903_190_333_668,
        "{}",
        digest(&text)
    );
}

#[test]
fn every_override_reads_and_writes_as_it_did() {
    let mut text = String::new();
    for f in overridden() {
        let wire = params::metadata(&f);
        text.push_str(&format!("{wire}\n{:?}\n", params::parse(&wire)));
    }
    let mut none = params::metadata(&Preset::DatePalm.parameters());
    for row in [
        "influenceRadius",
        "killDistance",
        "maxTurnPerStep",
        "maxNodes",
    ] {
        none["skeleton"]["growth"][row] = serde_json::Value::Null;
    }
    text.push_str(&format!("{none}\n{:?}\n", params::parse(&none)));
    assert_eq!(
        digest(&text),
        10_922_909_758_982_052_223,
        "{}",
        digest(&text)
    );
}

/// Every numeric leaf of the wire, as a JSON pointer.
fn leaves(value: &serde_json::Value, at: String, out: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map {
                leaves(child, format!("{at}/{key}"), out);
            }
        }
        _ => out.push(at),
    }
}

/// What the wire and the family's own check answer for one row set to one
/// value, and for a second row set with it.
fn answer(base: &serde_json::Value, rows: &[(&str, serde_json::Value)]) -> String {
    let mut wire = base.clone();
    for (row, value) in rows {
        *wire.pointer_mut(row).unwrap() = value.clone();
    }
    match params::parse(&wire) {
        Ok(f) => format!("{:?}", f.validate()),
        Err(error) => format!("parse {error:?}"),
    }
}

#[test]
fn every_refusal_is_the_refusal_it_was() {
    let base = params::metadata(&Family::default());
    let mut rows = Vec::new();
    leaves(&base, String::new(), &mut rows);
    let values: [serde_json::Value; 7] = [
        serde_json::json!(-1e9),
        serde_json::json!(-1),
        serde_json::json!(0),
        serde_json::json!(0.5),
        serde_json::json!(3),
        serde_json::json!(1e9),
        serde_json::json!(true),
    ];
    let mut text = String::new();
    for (i, row) in rows.iter().enumerate() {
        for value in &values {
            text.push_str(&format!(
                "{row}={value}: {}\n",
                answer(&base, &[(row, value.clone())])
            ));
        }
        // A second row off its rail with it: the first refusal must stay first.
        let other = &rows[(i * 7 + 3) % rows.len()];
        let pair = [
            (row.as_str(), values[5].clone()),
            (other.as_str(), values[0].clone()),
        ];
        text.push_str(&format!("{row}+{other}: {}\n", answer(&base, &pair)));
    }
    assert_eq!(
        digest(&text),
        17_142_609_772_195_174_599,
        "{}",
        digest(&text)
    );
}

/// fn-297 R6: a rosette's divergence is one turn either way. -360 and 360
/// are admitted; a thousandth of a degree past either, and a value that is
/// not finite, are refused by name.
#[test]
fn a_rosette_divergence_is_one_turn() {
    let base = params::metadata(&Family::default());
    let row = "/canopy/rosetteDivergence";
    for turn in [-360.0, 360.0] {
        assert_eq!(answer(&base, &[(row, serde_json::json!(turn))]), "Ok(())");
    }
    for past in [-360.001, 360.001] {
        let refused = answer(&base, &[(row, serde_json::json!(past))]);
        assert!(refused.contains("rosette divergence"), "{past}: {refused}");
    }
    for bad in [f64::NAN, f64::INFINITY] {
        let mut f = Family::default();
        f.canopy.rosette_divergence = bad;
        let refused = format!("{:?}", f.validate());
        assert!(refused.contains("rosette divergence"), "{bad}: {refused}");
    }
}
