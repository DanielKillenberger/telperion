use super::{check::GROUP_LIMIT, entries, Dial, Growth, Kind};
use crate::{presets::Preset, Family};
use std::collections::BTreeMap;

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

#[test]
fn every_wire_row_has_one_entry() {
    let paths: Vec<_> = entries().map(|e| e.path()).collect();
    assert_eq!(paths.len(), 249);
    let mut seen = std::collections::BTreeSet::new();
    for path in &paths {
        assert!(seen.insert(*path), "{path} is declared twice");
        let key = path.rsplit('/').next().unwrap();
        assert!(
            key.chars().next().is_some_and(|c| c.is_ascii_lowercase()) && !key.contains('_'),
            "{path}: a wire key is camelCase"
        );
    }
}

#[test]
fn every_site_judges_its_rows_in_one_order() {
    // Rank is per site and per group: two rows of one group on one site never
    // share a rank, and a site's ranks run from zero without a gap.
    let mut ranks: BTreeMap<(String, String), Vec<u8>> = BTreeMap::new();
    for e in entries() {
        if let Some(c) = e.info().check {
            let group = e.path().rsplit_once('/').unwrap().0.to_owned();
            ranks
                .entry((group, format!("{:?}", c.site)))
                .or_default()
                .push(c.rank);
        }
    }
    for ((group, site), mut r) in ranks {
        r.sort_unstable();
        let first = r[0];
        let expected: Vec<u8> = (first..first + r.len() as u8).collect();
        assert_eq!(r, expected, "{group} on {site}: ranks {r:?}");
    }
}

#[test]
fn every_group_fits_the_check() {
    let mut groups: BTreeMap<&str, usize> = BTreeMap::new();
    for e in entries() {
        *groups
            .entry(e.path().rsplit_once('/').unwrap().0)
            .or_default() += 1;
    }
    assert!(groups.values().all(|&n| n <= GROUP_LIMIT), "{groups:?}");
}

#[test]
fn every_checked_row_admits_every_preset() {
    for preset in PRESETS {
        let f = preset.parameters();
        for e in entries().filter(|e| e.info().check.is_some()) {
            if let Some(v) = e.get(&f).number() {
                assert!(e.info().bounds.admits(v), "{preset:?}: {} = {v}", e.path());
            }
        }
    }
}

#[test]
fn every_dial_steps_inside_its_row() {
    let mut ids = std::collections::BTreeSet::new();
    for e in entries() {
        let info = e.info();
        let Dial::Tuned(t) = info.dial else { continue };
        assert!(ids.insert(t.id), "dial id {} is used twice", t.id);
        assert!(
            !info.deprecated,
            "{}: a deprecated row offers a dial",
            e.path()
        );
        let [low, high] = t.window.unwrap_or([info.bounds.low, info.bounds.high]);
        assert!(
            low < high && high.is_finite(),
            "{}: window [{low}, {high}]",
            e.path()
        );
        // The pilot's `leaves` window is pinned by the hash its calibrated
        // manifests were qualified against, and runs past the eight leaves a
        // cluster may carry; the generator refuses a step above eight.
        let pinned = t.id == "leaves";
        assert!(
            pinned || info.bounds.admits(low) && info.bounds.admits(high),
            "{}: window [{low}, {high}] leaves its bounds",
            e.path()
        );
        assert!(0.0 < t.small && t.small < t.substantial && t.substantial <= high - low);
    }
}

#[test]
fn every_row_without_a_dial_says_why() {
    let defaults = Family::default();
    for e in entries() {
        let info = e.info();
        if info.dial != Dial::Derived {
            continue;
        }
        let kind = e.get(&defaults).kind();
        assert!(
            info.deprecated
                || info.growth == Growth::Only
                || matches!(kind, Kind::Switch | Kind::OptionalReal | Kind::OptionalSize),
            "{}: no dial and no reason",
            e.path()
        );
    }
}

#[test]
fn the_three_rows_no_stage_reads_are_deprecated() {
    let deprecated: Vec<_> = entries()
        .filter(|e| e.info().deprecated)
        .map(|e| e.path())
        .collect();
    assert_eq!(
        deprecated,
        ["/canopy/spacing", "/canopy/clump", "/canopy/clumpSpan"]
    );
}

/// Deprecated rows stay on the wire: a family written with them reads, lays
/// over and writes back exactly as before.
#[cfg(feature = "json")]
#[test]
fn the_deprecated_rows_parse_overlay_and_serialise_as_before() {
    use crate::params;
    let over = serde_json::json!({"canopy": {"spacing": 0.02, "clump": 3, "clumpSpan": 0.4}});
    let f = params::overlay(&Preset::OregonWhiteOak.parameters(), &over).unwrap();
    assert_eq!(
        (f.canopy.spacing, f.canopy.clump, f.canopy.clump_span),
        (0.02, 3, 0.4)
    );
    let wire = params::metadata(&f);
    for (key, value) in [("spacing", 0.02), ("clump", 3.0), ("clumpSpan", 0.4)] {
        assert_eq!(wire["canopy"][key].as_f64(), Some(value), "{key}");
    }
    assert_eq!(params::parse(&wire).unwrap().canopy.clump, 3);
    let off = serde_json::json!({"canopy": {"clump": 65}});
    assert_eq!(
        params::overlay(&f, &off).and_then(|f| f.validate()),
        Err(crate::Error::InvalidInput("foliage clump"))
    );
}

#[test]
fn every_row_has_its_own_rank_on_the_wire() {
    let mut ranks: Vec<u16> = entries().map(|e| e.info().wire).collect();
    ranks.sort_unstable();
    assert_eq!(ranks, (0..249).collect::<Vec<u16>>());
}

/// A row no direct-build stage reads says so: the growth path's own rows
/// and the deprecated ones name no stage, and every other row names one.
#[test]
fn every_row_names_the_stages_that_read_it() {
    for e in entries() {
        let info = e.info();
        let unread = info.deprecated || info.growth == Growth::Only;
        assert_eq!(info.reads.is_empty(), unread, "{}", e.path());
    }
}
