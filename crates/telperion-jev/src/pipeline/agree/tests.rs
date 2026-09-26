use serde::Deserialize;

use super::*;
use crate::pipeline::sets::{tier, KINDS};

fn reading(source: &str, site: &str, rank: usize, range: [f64; 2], basis: Basis) -> Reading {
    Reading {
        source: source.into(),
        site: site.into(),
        tier: rank,
        range,
        span: format!("{}-{} m", range[0], range[1]),
        sentence: String::new(),
        basis,
        ledger: "l".into(),
    }
}

#[test]
fn the_value_is_the_median_of_one_point_per_site() {
    let readings = [
        reading("P1", "a.org", 0, [25.0, 35.0], Basis::Typical),
        reading("P2", "b.org", 0, [30.0, 30.0], Basis::Typical),
        // A second page of b.org is the same source, not a vote.
        reading("P3", "b.org", 0, [40.0, 40.0], Basis::Typical),
        reading("P4", "c.org", 0, [40.0, 40.0], Basis::Typical),
    ];
    let got = aggregate(&readings);
    // Points: a 30, b median(30, 40) = 35, c 40.
    assert_eq!(got.value, Some(35.0));
    assert_eq!(got.range, Some([25.0, 40.0]));
    assert_eq!(got.sources, ["P1", "P2", "P3", "P4"]);
    assert_eq!((got.confidence, got.tier), ("agreed", Some(0)));
    assert_eq!(got.held[&0], 3);
}

#[test]
fn a_record_is_the_maximum_and_never_the_typical_value() {
    let readings = [
        reading("P1", "a.org", 0, [4.36, 4.36], Basis::Record),
        reading("P2", "b.org", 3, [1.0, 1.5], Basis::Typical),
    ];
    let got = aggregate(&readings);
    assert_eq!(got.value, Some(1.25));
    assert_eq!(got.maximum.unwrap().source, "P1");
    assert_eq!(
        (got.sources.as_slice(), got.tier),
        (&["P2".to_string()][..], Some(3))
    );
    let alone = aggregate(&readings[..1]);
    assert_eq!((alone.value, alone.range, alone.tier), (None, None, None));
    assert_eq!(alone.confidence, "unsourced");
    assert!(alone.maximum.is_some());
}

#[test]
fn a_site_is_the_registered_name_of_its_host() {
    let cases = [
        (
            "https://plants.ces.ncsu.edu/plants/fagus-sylvatica/",
            "ncsu.edu",
        ),
        (
            "https://www.woodlandtrust.org.uk/trees/",
            "woodlandtrust.org.uk",
        ),
        ("https://extension.wsu.edu/clark/heritage-tree/", "wsu.edu"),
        ("http://bomeninfo.nl/tall%20trees.htm", "bomeninfo.nl"),
        (
            "https://www.vdberk.com/trees/fagus-sylvatica/",
            "vdberk.com",
        ),
    ];
    for (url, want) in cases {
        assert_eq!(site(url), want, "{url}");
    }
}

#[derive(Deserialize)]
struct Cases {
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    id: String,
    readings: Vec<CaseReading>,
    expect: Expect,
}

#[derive(Deserialize)]
struct CaseReading {
    site: String,
    kind: String,
    range: [f64; 2],
}

#[derive(Deserialize)]
struct Expect {
    kind: String,
    value: f64,
    confidence: String,
    set_aside: Vec<String>,
}

/// Host decision 3: the agreement and outlier bounds are the ones that
/// answer every labelled case (`data/cases/aggregate.json`).
#[test]
fn the_bounds_answer_every_labelled_aggregate_case() {
    let raw = include_str!("../../../data/cases/aggregate.json");
    let set: Cases = serde_json::from_str(raw).unwrap();
    assert!(set.cases.len() >= 5);
    for case in set.cases {
        let readings: Vec<Reading> = case
            .readings
            .iter()
            .enumerate()
            .map(|(i, r)| {
                let source = format!("P{}", i + 1);
                reading(&source, &r.site, tier(&r.kind), r.range, Basis::Typical)
            })
            .collect();
        let got = aggregate(&readings);
        let id = &case.id;
        assert_eq!(
            got.tier.map(|t| KINDS[t]),
            Some(case.expect.kind.as_str()),
            "{id}"
        );
        assert!(
            (got.value.unwrap() - case.expect.value).abs() < 1e-3,
            "{id}: {got:?}"
        );
        assert_eq!(got.confidence, case.expect.confidence, "{id}");
        let aside: Vec<&str> = got
            .set_aside
            .iter()
            .map(|s| s.reading.site.as_str())
            .collect();
        assert_eq!(aside, case.expect.set_aside, "{id}");
    }
}
