//! fn-270 R5 and R5a on the CPU, before any device work: one tree, the
//! oak, drawn at fractions through one year, joins the year before with
//! no pop, moves by degree, and is today's tree at the whole year.
use super::*;
use crate::grow::cycle::{grow_cycle, tree_digest, Paused, Retiring};
use crate::grow::Request;
use crate::light::Light;

/// The thinnest wood the pipeline draws (`space.rs`'s `FINEST`).
const FINEST: f64 = 2e-4;

fn request(age: u32) -> Request {
    Request { age, seed: 7, budget: 20_000_000, light: Light { extinction: 0.5, sky: 0.5 } }
}

/// Every organ's drawing by its axis's identity and its place: its tip,
/// girth, scale and its leaf's share.
fn organs(p: &Presented) -> HashMap<(u64, usize), [f64; 6]> {
    let ids = identities(&p.structure);
    let mut m = HashMap::new();
    for (i, (a, id)) in p.structure.axes.iter().zip(ids).enumerate() {
        for (k, q) in a.phytomers.iter().enumerate() {
            let leaf = p.leaves[i].get(k).map_or(1.0, |&l| f64::from(l));
            m.insert((id, k), [q.tip.x, q.tip.y, q.tip.z, q.radius, q.scale, leaf]);
        }
    }
    m
}

/// How much of an organ is seen: its leaf, and its wood where it has both
/// length and girth past the thinnest drawn.
fn seen(x: &[f64; 6]) -> f64 {
    x[5].max(x[4].min(x[3] / FINEST))
}

/// The most any organ of both drawings moves between them, by its
/// place, girth, scale and leaf; an organ of one only by how much of it
/// is seen.
fn most(a: &HashMap<(u64, usize), [f64; 6]>, b: &HashMap<(u64, usize), [f64; 6]>) -> f64 {
    let mut most: f64 = 0.0;
    for (key, x) in a {
        let d = match b.get(key) {
            Some(y) => x.iter().zip(y).map(|(p, q)| (p - q).abs()).fold(0.0, f64::max),
            None => seen(x),
        };
        most = most.max(d);
    }
    for (key, y) in b {
        if !a.contains_key(key) {
            most = most.max(seen(y));
        }
    }
    most
}

#[test]
fn the_course_runs_from_nothing_to_whole_and_back() {
    assert_eq!((course(0.0), course(1.0)), (0.0, 1.0));
    for k in 1..100 {
        let x = f64::from(k) / 100.0;
        assert!(course(x) > course(x - 0.01));
        assert!((reached(course(x)) - x).abs() < 1e-9, "{x}");
    }
}

/// R5, R5a and R2a: the oak at 24, drawn at fractions of its last year.
#[test]
fn a_tree_drawn_within_its_year_joins_both_years_with_no_pop() {
    let species = crate::oak();
    let r = request(24);
    let mut p = Paused::begin(&species, r, Retiring::AtAge, false).unwrap().presenting(FINEST);
    p.grow_to(24).unwrap();
    let before = p.digest();
    let now = p.settle().unwrap();
    let last = p.previous().expect("the year before").clone();
    assert_eq!(last.age, 23);
    let draw = |f: f64| present(now.clone(), &last, &species, (f, FINEST));

    // At the fraction's start the old organs are year 23's to the bit,
    // the new ones none, and nothing gone has faded.
    let start = draw(0.0);
    let year = organs(&Presented::whole(last.clone()));
    let mut new = 0;
    for (key, x) in organs(&start) {
        match year.get(&key) {
            Some(y) => assert_eq!(x.map(f64::to_bits)[..5], y.map(f64::to_bits)[..5], "{key:?}"),
            None => {
                assert_eq!((x[4], x[5]), (0.0, 0.0), "a new organ starts from nothing");
                new += 1;
            }
        }
    }
    assert!(new > 0 && organs(&start).len() > year.len());
    // An axis that stays holds every phytomer it held.
    let held = organs(&Presented::whole(now.clone()));
    assert!(year.keys().all(|key| held.contains_key(key) || !held.contains_key(&(key.0, 0))));

    // At its end it is today's tree, which `settle_at` draws, the grower
    // left as it was.
    let today = tree_digest(&grow_cycle(&species, r).unwrap().structure);
    assert_eq!(tree_digest(&p.settle_at(24.0).unwrap().structure), today);
    let whole = organs(&Presented::whole(now.clone()));

    // No pop: between close fractions every organ moves by degree, and
    // half the step moves it about half as far; just past the start and
    // just before the end it is the year before and this one.
    let ladder = [1e-6, 0.02, 0.05, 0.1, 0.2, 0.5, 0.9, 1.0 - 1e-6];
    for &f in &ladder {
        let at = organs(&draw(f));
        let near = |d: f64| if f + d < 1.0 { f + d } else { f - d };
        let steps: Vec<f64> = [4e-4, 2e-4, 1e-4, 5e-5].iter().map(|&d| most(&at, &organs(&draw(near(d))))).collect();
        for w in steps.windows(2) {
            assert!(w[1] <= 0.6 * w[0] + 1e-9, "at {f}: {steps:?}");
        }
    }
    assert!(most(&organs(&draw(1e-6)), &year.iter().map(|(k, v)| (*k, *v)).collect()) < 1e-3);
    assert!(most(&organs(&draw(1.0 - 1e-6)), &whole) < 1e-3);
    assert_eq!(p.digest(), before, "a draw within the year moves the grower");
    let drawn = p.settle_at(23.5).unwrap();
    assert_eq!(drawn, draw(0.5));
}
