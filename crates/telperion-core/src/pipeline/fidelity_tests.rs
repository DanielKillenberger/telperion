//! fn-326: a family's fidelity, through the one cycle. The ordinary tests hold
//! its contract on every preset; the ignored ones measure R4's six statistics
//! and search a preset's `/space/coarse` rows.
use super::{executor, measures, skeleton, GrowInput};
use crate::presets::{by_identity, Family, CATALOGUE};
use crate::tree::Tree;
use telperion_space::Coarsening;

/// The manifest's fidelities, finest first.
const FIDELITIES: [f64; 4] = [0.0, 0.05, 0.25, 1.0];
/// Young enough for an ordinary test, and old enough to branch on every preset.
const AGE: f64 = 24.0;

fn family(id: &str, age: f64, seed: u32, fidelity: f64) -> Family {
    let mut f = by_identity(id).expect("a preset");
    f.age = age;
    f.skeleton.seed = seed;
    f.fidelity = fidelity;
    f
}

fn grown(f: &Family) -> u64 {
    let g = executor::growth(f).expect("its growth");
    telperion_space::grow_cycle(&g.species, g.request)
        .expect("a tree")
        .grown
}

#[test]
fn the_finest_grows_the_presets_own_species_on_every_preset() {
    for &(_, id, ..) in CATALOGUE {
        let with_rows = family(id, AGE, 7, 0.0);
        let mut without = with_rows.clone();
        without.space.coarse = Coarsening::NONE;
        let species = |f: &Family| executor::growth(f).expect("its growth").species;
        assert_eq!(species(&with_rows), species(&without), "{id}");
        let tree = |f: &Family| skeleton(GrowInput::of(f)).expect("a tree").tree.nodes;
        assert_eq!(tree(&with_rows), tree(&without), "{id}");
    }
}

#[test]
fn a_coarser_request_never_grows_more_on_any_preset() {
    for &(_, id, ..) in CATALOGUE {
        let counts = FIDELITIES.map(|fidelity| grown(&family(id, AGE, 7, fidelity)));
        assert!(counts.windows(2).all(|w| w[1] <= w[0]), "{id}: {counts:?}");
    }
}

#[test]
fn the_oak_spruce_and_birch_grow_strictly_less_at_each_step() {
    for id in ["oregon-white-oak", "norway-spruce", "silver-birch"] {
        let counts = FIDELITIES.map(|fidelity| grown(&family(id, AGE, 7, fidelity)));
        assert!(counts.windows(2).all(|w| w[1] < w[0]), "{id}: {counts:?}");
    }
}

#[test]
fn a_fidelity_off_its_range_is_refused_by_name() {
    for bad in [-0.5, f64::NAN, f64::INFINITY] {
        let f = Family {
            fidelity: bad,
            ..family("oregon-white-oak", AGE, 7, 0.0)
        };
        let refused = f.validate().expect_err("off its range");
        assert!(refused.to_string().contains("fidelity"), "{refused}");
        let grown = executor::growth(&f)
            .err()
            .expect("the growth entry refuses");
        assert!(grown.to_string().contains("fidelity"), "{grown}");
        let built = skeleton(GrowInput::of(&f))
            .err()
            .expect("the skeleton refuses");
        assert!(built.to_string().contains("fidelity"), "{built}");
    }
}

#[test]
fn a_walk_between_two_fractional_requests_keeps_their_fidelity() {
    let oak = family("oregon-white-oak", AGE, 7, 0.25);
    let spruce = family("norway-spruce", AGE, 7, 0.25);
    let walked = crate::blend::families(&oak, &spruce, 0.5).expect("a walk");
    assert_eq!(walked.fidelity, 0.25);
}

#[test]
fn a_walk_between_two_dials_moves_its_first_age_by_degree() {
    let oak = family("oregon-white-oak", AGE, 7, 1.0);
    let spruce = family("norway-spruce", AGE, 7, 1.0);
    let first = |t| {
        crate::blend::families(&spruce, &oak, t)
            .expect("a walk")
            .space
            .coarse
            .first
    };
    assert_eq!(first(0.5), 8.5);
    assert!((first(0.001) - 8.0).abs() < 0.01 && (first(0.999) - 9.0).abs() < 0.01);
}

#[test]
fn rows_off_their_domain_are_refused_when_the_family_is_judged() {
    let mut f = family("oregon-white-oak", AGE, 7, 1.0);
    f.space.coarse.factor = 0.0;
    let refused = f.validate().expect_err("off its domain");
    assert!(refused.to_string().contains("coarse factor"), "{refused}");
}

#[test]
fn a_species_with_no_rows_is_dormant_to_the_dial() {
    let mut f = family("oregon-white-oak", AGE, 7, 1.0);
    f.space.coarse = Coarsening::NONE;
    let mut finest = f.clone();
    finest.fidelity = 0.0;
    let species = |f: &Family| executor::growth(f).expect("its growth").species;
    assert_eq!(species(&f), species(&finest));
}

fn shape(tree: &Tree) -> Option<[f64; 3]> {
    let base = tree.nodes.first()?.position;
    let (mut top, mut lo, mut hi) = (f64::MIN, [f64::MAX; 2], [f64::MIN; 2]);
    let mut breast = 0f64;
    for n in &tree.nodes {
        let p = n.position;
        top = top.max(p.y - base.y);
        lo = [lo[0].min(p.x), lo[1].min(p.z)];
        hi = [hi[0].max(p.x), hi[1].max(p.z)];
        if (1.0..=1.6).contains(&(p.y - base.y)) {
            breast = breast.max(n.radius);
        }
    }
    (breast > 0.0).then_some([top, ((hi[0] - lo[0]) + (hi[1] - lo[1])) / 2.0, 2.0 * breast])
}

/// Height, spread, trunk, limbs, shed share, leaf area, cycle seconds, and the
/// axes and apex-cycles grown.
fn read(f: &Family, repeats: usize) -> Option<[f64; 9]> {
    let g = executor::growth(f).ok()?;
    let first = telperion_space::grow_census(&g.species, g.request).ok()?;
    let census = first.census.as_ref()?;
    let apexes: usize = census.cycles.iter().map(|c| c.active).sum();
    let work = [census.axes as f64, apexes as f64];
    let mut times = vec![first.seconds];
    if repeats > 1 && first.seconds <= 0.5 {
        for _ in 1..repeats {
            times.push(
                telperion_space::grow_census(&g.species, g.request)
                    .ok()?
                    .seconds,
            );
        }
        times.remove(0);
    }
    let kept: u64 = first
        .structure
        .axes
        .iter()
        .map(|a| a.phytomers.len() as u64)
        .sum();
    let shed = measures::shed_share(kept, first.grown).ok()?;
    let age = f.age as u32;
    let leaf = telperion_space::floor::bearing_area(&g.species, &first.structure.axes, age);
    let tree = skeleton(GrowInput::of(f)).ok()?.tree;
    let limbs = measures::limb_count(&tree, f.canopy.clump_system_order).ok()? as f64;
    let [height, spread, trunk] = shape(&tree)?;
    Some([
        height,
        spread,
        trunk,
        limbs,
        shed,
        leaf,
        median(times),
        work[0],
        work[1],
    ])
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    (v[(v.len() - 1) / 2] + v[v.len() / 2]) / 2.0
}

fn medians(fs: &[Family], repeats: usize) -> Option<[f64; 9]> {
    let rows = fs
        .iter()
        .map(|f| read(f, repeats))
        .collect::<Option<Vec<_>>>()?;
    Some(std::array::from_fn(|k| {
        median(rows.iter().map(|r| r[k]).collect())
    }))
}

fn list<T: std::str::FromStr>(name: &str) -> Vec<T> {
    let v = std::env::var(name).unwrap_or_else(|_| panic!("{name}"));
    v.split(',')
        .map(|x| x.parse().ok().expect("a value"))
        .collect()
}

/// One line per row of `SEARCH_FILE`, `first factor leaf bud hazard shedding
/// fidelity`, for `SEARCH_PRESET` on `SEARCH_SEEDS`, at `SEARCH_AGE`
/// or the preset's own: each statistic over the same seeds' full cycle.
#[test]
#[ignore = "a measurement: fn-326 R5, and the search for a preset's rows"]
fn fidelity_rows() {
    let id = std::env::var("SEARCH_PRESET").expect("a preset");
    let seeds: Vec<u32> = list("SEARCH_SEEDS");
    let repeats = list::<usize>("SEARCH_REPEATS")[0];
    let preset = by_identity(&id).expect("a preset");
    let age = std::env::var("SEARCH_AGE").map_or(preset.age, |a| a.parse().expect("an age"));
    let at = |fidelity: f64, coarse: Option<Coarsening>| -> Vec<Family> {
        let row = |&seed: &u32| {
            let mut f = family(&id, age, seed, fidelity);
            f.space.coarse = coarse.unwrap_or(f.space.coarse);
            f
        };
        seeds.iter().map(row).collect()
    };
    let reference = medians(&at(0.0, None), 1).expect("the full cycle");
    println!("REFERENCE {id} age {age} {reference:?}");
    let rows =
        std::fs::read_to_string(std::env::var("SEARCH_FILE").expect("a file")).expect("rows");
    for row in rows.lines().filter(|l| !l.trim().is_empty()) {
        let v: Vec<f64> = row
            .split_whitespace()
            .map(|x| x.parse().expect("a value"))
            .collect();
        let c = Coarsening {
            first: v[0],
            factor: v[1],
            leaf: v[2],
            bud: v[3],
            hazard: v[4],
            shedding: v[5],
        };
        let Some(m) = medians(&at(v[6], Some(c)), repeats) else {
            println!("FAILED {row}");
            continue;
        };
        let q: [f64; 6] = std::array::from_fn(|k| m[k] / reference[k]);
        let worst = q.iter().map(|x| (x - 1.0).abs()).fold(0.0, f64::max);
        println!(
            "CAND age {age} first {} factor {} leaf {} bud {} hazard {} shedding {} fidelity {} height {:.3} spread {:.3} trunk {:.3} limbs {:.3} shed {:.3} area {:.3} worst {worst:.3} seconds {:.4} axes {:.0} apexes {:.0}",
            c.first, c.factor, c.leaf, c.bud, c.hazard, c.shedding, v[6], q[0], q[1], q[2], q[3], q[4], q[5], m[6], m[7], m[8]
        );
    }
}
