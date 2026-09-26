//! One field's confident aggregate (fn-157), all in code.
//!
//! Every reading is a span code parsed from a document and Jev labelled
//! with its field and its basis; Jev never chooses between sources. Pages of
//! one site are one source, and each source's typical readings are one
//! point, the median of their midpoints. Sources are ranked by their
//! document's kind (host decision, 2026-09-26): the value comes from the best
//! tier that holds `AGREEING_SOURCES` points within `AGREEMENT_RATIO` of each
//! other, and a lower tier fills a field only when no tier agrees and no
//! better tier holds a value. Within the deciding tier a point beyond
//! `OUTLIER_RATIO` of the tier's median is set aside and noted. The value is
//! the median of the points left, the range their extent. A record or a
//! single specimen is the field's maximum and never its typical value.

use std::collections::BTreeMap;

/// Points a tier needs, within `AGREEMENT_RATIO` of each other, to agree.
pub const AGREEING_SOURCES: usize = 2;
pub const AGREEMENT_RATIO: f64 = 1.5;
/// A point beyond this factor of its tier's median is set aside, once the
/// tier holds three: two points cannot outvote each other.
pub const OUTLIER_RATIO: f64 = 2.0;

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
    /// The rank of the source's kind, 0 best (`sets::tier`).
    pub tier: usize,
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

/// The field's aggregate. `value`, `range` and `tier` are None when no
/// source states a typical value; the maximum stands either way.
#[derive(Debug, Clone, PartialEq)]
pub struct Aggregate {
    pub value: Option<f64>,
    pub range: Option<[f64; 2]>,
    /// The tier that decided the value.
    pub tier: Option<usize>,
    /// The sources behind the value, in first-read order.
    pub sources: Vec<String>,
    /// The readings behind the value.
    pub contributions: Vec<Reading>,
    /// The largest point over the smallest, 1 when one source stands.
    pub spread_ratio: Option<f64>,
    pub confidence: &'static str,
    pub maximum: Option<Reading>,
    pub set_aside: Vec<SetAside>,
    /// Each tier's independent sources with a typical value.
    pub held: BTreeMap<usize, usize>,
}

/// One source's point: its readings and the median of their midpoints.
struct Point {
    site: String,
    point: f64,
    readings: Vec<Reading>,
}

/// One tier's points after its outliers are set aside.
struct Tier {
    rank: usize,
    points: Vec<Point>,
    set_aside: Vec<SetAside>,
    agreed: bool,
}

pub fn aggregate(readings: &[Reading]) -> Aggregate {
    let maximum = readings
        .iter()
        .filter(|r| r.basis == Basis::Record)
        .max_by(|a, b| a.range[1].total_cmp(&b.range[1]))
        .cloned();
    let tiers = tiers(readings);
    let held = tiers.iter().map(|t| (t.rank, t.points.len())).collect();
    let chosen = tiers
        .iter()
        .find(|t| t.agreed)
        .or_else(|| tiers.iter().find(|t| !t.points.is_empty()));
    let Some(tier) = chosen else {
        return Aggregate {
            value: None,
            range: None,
            tier: None,
            sources: Vec::new(),
            contributions: Vec::new(),
            spread_ratio: None,
            confidence: "unsourced",
            maximum,
            set_aside: Vec::new(),
            held,
        };
    };
    let contributions: Vec<Reading> = tier
        .points
        .iter()
        .flat_map(|p| p.readings.clone())
        .collect();
    let mut sources: Vec<String> = Vec::new();
    for reading in &contributions {
        if !sources.contains(&reading.source) {
            sources.push(reading.source.clone());
        }
    }
    Aggregate {
        value: median(tier.points.iter().map(|p| p.point).collect()),
        range: extent(&contributions),
        tier: Some(tier.rank),
        sources,
        spread_ratio: spread(&tier.points),
        confidence: if tier.agreed { "agreed" } else { "thin" },
        contributions,
        maximum,
        set_aside: tier.set_aside.clone(),
        held,
    }
}

/// The typical readings of each tier as points, best tier first.
fn tiers(readings: &[Reading]) -> Vec<Tier> {
    let mut by_rank: BTreeMap<usize, Vec<Reading>> = BTreeMap::new();
    for reading in readings.iter().filter(|r| r.basis == Basis::Typical) {
        by_rank
            .entry(reading.tier)
            .or_default()
            .push(reading.clone());
    }
    by_rank
        .into_iter()
        .map(|(rank, readings)| {
            let (points, set_aside) = outliers(points(&readings));
            let agreed = points.len() >= AGREEING_SOURCES
                && spread(&points).is_some_and(|r| r <= AGREEMENT_RATIO);
            Tier {
                rank,
                points,
                set_aside,
                agreed,
            }
        })
        .collect()
}

/// Each site's readings as one point, in first-read order.
fn points(readings: &[Reading]) -> Vec<Point> {
    let mut by_site: Vec<Point> = Vec::new();
    for reading in readings {
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

/// The points within `OUTLIER_RATIO` of the tier's median, and every reading
/// of the ones beyond it.
fn outliers(points: Vec<Point>) -> (Vec<Point>, Vec<SetAside>) {
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
                "{} is {:.1} times the median {} of its tier",
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
mod tests;
