//! One field's confident aggregate (fn-157), all in code.
//!
//! Every reading is a span code parsed from a document and Jev labelled
//! with its field and its basis; Jev never chooses between sources. Pages of
//! one site are one source. Each source's typical readings become one point,
//! the median of their midpoints, and one range, their extent. A point far
//! from the others' median is set aside and noted. The value is the median
//! of the points left, the range their extent, and the confidence the count
//! of sources behind it and how far their points spread. A record or a single
//! specimen is kept as the field's maximum and never as its typical value.
//!
//! The three bounds below are the spec's open question (fn-157, Unknown): a
//! proposal measured on the recorded beech pages, for the host to settle.

/// A point more than this factor from the median of the points is set aside,
/// once there are enough points to have a median worth the name.
pub const OUTLIER_RATIO: f64 = 2.0;
/// Independent sources a value needs, their points within
/// `AGREEMENT_RATIO` of each other, to be `agreed` rather than `thin`.
pub const AGREEING_SOURCES: usize = 3;
pub const AGREEMENT_RATIO: f64 = 1.5;

/// What a reading says the number is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Basis {
    /// What trees of the species usually reach.
    Typical,
    /// A record, or one individual tree.
    Record,
}

/// One labelled span, parsed by code.
#[derive(Debug, Clone, PartialEq)]
pub struct Reading {
    pub source: String,
    /// The site the source is on: pages of one site are one source.
    pub site: String,
    /// `[low, high]` in the field's unit.
    pub range: [f64; 2],
    pub span: String,
    pub sentence: String,
    pub basis: Basis,
    pub ledger: String,
}

/// A reading left out of the value, and why.
#[derive(Debug, Clone, PartialEq)]
pub struct SetAside {
    pub reading: Reading,
    pub reason: String,
}

/// The field's aggregate. `value` and `range` are None when no source states
/// a typical value; the maximum and the set-aside readings stand either way.
#[derive(Debug, Clone, PartialEq)]
pub struct Aggregate {
    pub value: Option<f64>,
    pub range: Option<[f64; 2]>,
    /// The sources behind the value, in first-read order.
    pub sources: Vec<String>,
    /// The readings behind the value.
    pub contributions: Vec<Reading>,
    /// The largest point over the smallest, 1 when one source stands.
    pub spread_ratio: Option<f64>,
    pub confidence: &'static str,
    pub maximum: Option<Reading>,
    pub set_aside: Vec<SetAside>,
}

/// One source's point: its readings and the median of their midpoints.
struct Point {
    site: String,
    point: f64,
    readings: Vec<Reading>,
}

pub fn aggregate(readings: &[Reading]) -> Aggregate {
    let maximum = readings
        .iter()
        .filter(|r| r.basis == Basis::Record)
        .max_by(|a, b| a.range[1].total_cmp(&b.range[1]))
        .cloned();
    let (points, set_aside) = agreeing(points(readings));
    let value = median(points.iter().map(|p| p.point).collect());
    let contributions: Vec<Reading> = points.iter().flat_map(|p| p.readings.clone()).collect();
    let range = extent(&contributions);
    let spread_ratio = spread(&points);
    let mut sources: Vec<String> = Vec::new();
    for reading in &contributions {
        if !sources.contains(&reading.source) {
            sources.push(reading.source.clone());
        }
    }
    let confidence = match (points.len(), spread_ratio) {
        (0, _) => "unsourced",
        (n, Some(r)) if n >= AGREEING_SOURCES && r <= AGREEMENT_RATIO => "agreed",
        _ => "thin",
    };
    Aggregate {
        value,
        range,
        sources,
        contributions,
        spread_ratio,
        confidence,
        maximum,
        set_aside,
    }
}

/// Each site's typical readings as one point, in first-read order.
fn points(readings: &[Reading]) -> Vec<Point> {
    let mut by_site: Vec<Point> = Vec::new();
    for reading in readings.iter().filter(|r| r.basis == Basis::Typical) {
        match by_site.iter_mut().find(|p| p.site == reading.site) {
            Some(point) => point.readings.push(reading.clone()),
            None => by_site.push(Point {
                site: reading.site.clone(),
                point: 0.0,
                readings: vec![reading.clone()],
            }),
        }
    }
    for point in &mut by_site {
        let mids = point.readings.iter().map(|r| midpoint(r.range)).collect();
        point.point = median(mids).unwrap_or_default();
    }
    by_site
}

/// The points within `OUTLIER_RATIO` of the median, and every reading of the
/// ones beyond it. Below three points no median outvotes a source.
fn agreeing(points: Vec<Point>) -> (Vec<Point>, Vec<SetAside>) {
    let Some(centre) = median(points.iter().map(|p| p.point).collect()) else {
        return (points, Vec::new());
    };
    if points.len() < 3 || centre <= 0.0 {
        return (points, Vec::new());
    }
    let (kept, far): (Vec<Point>, Vec<Point>) = points
        .into_iter()
        .partition(|p| ratio(p.point, centre) <= OUTLIER_RATIO);
    let set_aside = far
        .into_iter()
        .flat_map(|p| {
            let reason = format!(
                "{} is {:.1} times the median {} of every source's point",
                p.site,
                ratio(p.point, centre),
                round(centre)
            );
            p.readings.into_iter().map(move |reading| SetAside {
                reading,
                reason: reason.clone(),
            })
        })
        .collect();
    (kept, set_aside)
}

fn ratio(a: f64, b: f64) -> f64 {
    if a <= 0.0 || b <= 0.0 {
        return f64::INFINITY;
    }
    (a / b).max(b / a)
}

fn spread(points: &[Point]) -> Option<f64> {
    let lo = points.iter().map(|p| p.point).reduce(f64::min)?;
    let hi = points.iter().map(|p| p.point).reduce(f64::max)?;
    Some(ratio(hi, lo))
}

fn extent(readings: &[Reading]) -> Option<[f64; 2]> {
    let lo = readings.iter().map(|r| r.range[0]).reduce(f64::min)?;
    let hi = readings.iter().map(|r| r.range[1]).reduce(f64::max)?;
    Some([lo, hi])
}

fn midpoint(range: [f64; 2]) -> f64 {
    (range[0] + range[1]) / 2.0
}

pub fn median(mut values: Vec<f64>) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    let mid = values.len() / 2;
    Some(match values.len() % 2 {
        0 => (values[mid - 1] + values[mid]) / 2.0,
        _ => values[mid],
    })
}

/// A number as a note prints it: four significant figures at most.
pub fn round(value: f64) -> String {
    let text = format!("{value:.4}");
    let text = text.trim_end_matches('0').trim_end_matches('.');
    text.to_string()
}

/// The site a URL is on, so pages of one site count once: the host without
/// `www.`, cut to its registered name - `plants.ces.ncsu.edu` and
/// `content.ces.ncsu.edu` are both `ncsu.edu`, `www.woodlandtrust.org.uk`
/// is `woodlandtrust.org.uk`.
pub fn site(url: &str) -> String {
    let host = super::rights::host(url);
    let labels: Vec<&str> = host.split('.').filter(|l| !l.is_empty()).collect();
    let second_level = ["ac", "co", "com", "edu", "gov", "net", "org"];
    let keep = match labels.as_slice() {
        [.., second, tld] if tld.len() == 2 && second_level.contains(second) => 3,
        _ => 2,
    };
    labels[labels.len().saturating_sub(keep)..].join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reading(source: &str, site: &str, range: [f64; 2], basis: Basis) -> Reading {
        Reading {
            source: source.into(),
            site: site.into(),
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
            reading("P1", "a.org", [25.0, 35.0], Basis::Typical),
            reading("P2", "b.org", [30.0, 30.0], Basis::Typical),
            // A second page of b.org is the same source, not a vote.
            reading("P3", "b.org", [40.0, 40.0], Basis::Typical),
            reading("P4", "c.org", [40.0, 40.0], Basis::Typical),
        ];
        let got = aggregate(&readings);
        // Points: a 30, b median(30, 40) = 35, c 40.
        assert_eq!(got.value, Some(35.0));
        assert_eq!(got.range, Some([25.0, 40.0]));
        assert_eq!(got.sources, ["P1", "P2", "P3", "P4"]);
        assert_eq!(got.confidence, "agreed");
        assert!((got.spread_ratio.unwrap() - 40.0 / 30.0).abs() < 1e-9);
    }

    #[test]
    fn a_point_far_from_the_median_is_set_aside_and_named() {
        let readings = [
            reading("P1", "a.org", [0.05, 0.1], Basis::Typical),
            reading("P2", "b.org", [0.06, 0.09], Basis::Typical),
            reading("P3", "c.org", [0.2, 0.3], Basis::Typical),
        ];
        let got = aggregate(&readings);
        assert_eq!(got.sources, ["P1", "P2"]);
        assert_eq!(got.set_aside.len(), 1);
        assert_eq!(got.set_aside[0].reading.source, "P3");
        assert!(got.set_aside[0]
            .reason
            .starts_with("c.org is 3.3 times the median"));
        assert_eq!(got.confidence, "thin");
    }

    #[test]
    fn a_record_is_the_maximum_and_never_the_typical_value() {
        let readings = [
            reading("P1", "a.org", [4.36, 4.36], Basis::Record),
            reading("P2", "b.org", [1.0, 1.5], Basis::Typical),
        ];
        let got = aggregate(&readings);
        assert_eq!(got.value, Some(1.25));
        assert_eq!(got.maximum.unwrap().source, "P1");
        assert_eq!(got.sources, ["P2"]);
        let alone = aggregate(&readings[..1]);
        assert_eq!((alone.value, alone.range), (None, None));
        assert_eq!(alone.confidence, "unsourced");
        assert!(alone.maximum.is_some());
    }

    #[test]
    fn two_sources_are_never_outvoted_and_stay_thin() {
        let readings = [
            reading("P1", "a.org", [10.0, 10.0], Basis::Typical),
            reading("P2", "b.org", [40.0, 40.0], Basis::Typical),
        ];
        let got = aggregate(&readings);
        assert!(got.set_aside.is_empty());
        assert_eq!(got.value, Some(25.0));
        assert_eq!(got.confidence, "thin");
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
}
