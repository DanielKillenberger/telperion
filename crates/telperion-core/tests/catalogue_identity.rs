//! The parameter catalogue changes how rows are declared, never what a family
//! is. These digests were recorded on master before the catalogue (d6a3405c):
//! every shipped and in-work preset as its table builds it and as its identity
//! serves it, walks between them at points inside the walk, families whose
//! growth overrides are stated on one side, both or neither, and what the wire
//! and the family's own check answer for a value off every row's rail. A
//! digest that moves means the catalogue changed a value, a walk or a refusal.
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
        8_706_588_091_042_390_827,
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
    for (i, a) in ends.iter().enumerate() {
        for b in &ends[i + 1..] {
            for t in [0.0, 0.13, 0.5, 0.731, 1.0] {
                text.push_str(&format!("{:?}\n", blend::families(a, b, t)));
            }
        }
    }
    assert_eq!(
        digest(&text),
        6_751_395_173_143_375_865,
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
        2_343_201_054_118_623_675,
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
    let base = params::metadata(&Preset::Ordinary.parameters());
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
        17_635_382_561_672_926_617,
        "{}",
        digest(&text)
    );
}
