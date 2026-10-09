//! Leaves by their wood's age (fn-248): a station keeps its leaf by its
//! wood's retention, clamp(leaf_years - age, 0, 1), counted the same in
//! the plan, the placement and the stations both executors place from;
//! a station that sheds its leaf still takes its draws, so every leaf
//! that stays is the one the tree bore for ever.
use telperion_core::{
    foliage::Leaf,
    mesh,
    pipeline::executor,
    presets::{Family, Preset},
};

/// The beech young, every age's wood bearing for `years`, its short
/// shoots bearing along their wood as its long shoots do: no cluster,
/// which bears whatever its wood's age.
fn retaining(years: f64) -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    f.age = 15.0;
    let species = f.space.species.as_mut().expect("an engine species");
    for s in &mut species.states {
        (s.leaf_years, s.cluster_leaves) = (years, 0.0);
    }
    f
}

fn leaves(f: &Family) -> Vec<Leaf> {
    mesh::build(f).unwrap().foliage.instances.leaves
}

/// Whether `part` is `whole` with some leaves taken out, in order.
fn within(part: &[Leaf], whole: &[Leaf]) -> bool {
    let mut rest = whole.iter();
    part.iter().all(|leaf| rest.any(|w| w == leaf))
}

/// Fewer years keep fewer leaves, and each that stays is one the tree
/// bearing for ever bore, bit for bit; none stays where no wood bears.
#[test]
fn a_leaf_stays_only_where_its_wood_keeps_it() {
    let all = leaves(&retaining(f64::INFINITY));
    let mut fewer = all.len() + 1;
    for years in [3.0, 2.0, 1.5, 1.0] {
        let kept = leaves(&retaining(years));
        assert!(kept.len() < fewer, "{years}: {} leaves", kept.len());
        assert!(!kept.is_empty(), "{years}");
        assert!(within(&kept, &all), "{years}: a leaf moved");
        fewer = kept.len();
    }
    assert!(leaves(&retaining(0.0)).is_empty());
}

/// A fraction of a year keeps that share of the year it splits, within
/// four standard errors of its draws.
#[test]
fn a_share_of_a_year_keeps_that_share() {
    let n = |years: f64| {
        let mut f = retaining(years);
        f.age = 30.0;
        leaves(&f).len() as f64
    };
    let (one, two) = (n(1.0), n(2.0));
    for share in [0.25, 0.5, 0.75] {
        let kept = (n(1.0 + share) - one) / (two - one);
        let error = (share * (1.0 - share) / (two - one)).sqrt();
        assert!(
            (kept - share).abs() < 4.0 * error,
            "{share}: {kept} of {}",
            two - one
        );
    }
}

/// The plan, the placement and the prepared stations count the same.
#[test]
fn every_count_is_the_stations_kept() {
    for years in [1.0, 1.5] {
        let f = retaining(years);
        let x = executor::grow(&f).unwrap().expansion().unwrap();
        let prepared = x.stations().unwrap().unwrap();
        let placed = mesh::build(&f).unwrap().foliage.instances;
        assert_eq!(placed.placed(), prepared.count as usize, "{years}");
        let kept: u32 = prepared.segments.iter().map(|s| s.count).sum();
        assert_eq!(kept, prepared.count, "{years}");
        // A segment some of whose stations shed says where: a share of a
        // year sheds some of a node's.
        let partial = prepared.segments.iter().filter(|s| s.retention.is_some());
        assert!(years.fract() == 0.0 || partial.count() > 0, "{years}");
    }
}
