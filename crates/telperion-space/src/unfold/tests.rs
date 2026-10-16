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
    Request {
        age,
        seed: 7,
        budget: 20_000_000,
        light: Light {
            extinction: 0.5,
            sky: 0.5,
        },
    }
}

/// Every organ's drawing by its axis's identity and its place: its tip,
/// girth, scale and its leaf's share.
fn organs(p: &Presented) -> HashMap<(u64, usize), [f64; 6]> {
    let ids = identities(&p.structure);
    let mut m = HashMap::new();
    for (i, (a, id)) in p.structure.axes.iter().zip(ids).enumerate() {
        for (k, q) in a.phytomers.iter().enumerate() {
            let leaf = p.leaves[i].get(k).map_or(1.0, |&l| f64::from(l));
            m.insert(
                (id, k),
                [q.tip.x, q.tip.y, q.tip.z, q.radius, q.scale, leaf],
            );
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
            Some(y) => x
                .iter()
                .zip(y)
                .map(|(p, q)| (p - q).abs())
                .fold(0.0, f64::max),
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
    let mut p = Paused::begin(&species, r, Retiring::AtAge, false)
        .unwrap()
        .presenting();
    p.grow_to(24).unwrap();
    let before = p.digest();
    let now = p.settle().unwrap();
    let last = p.previous().expect("the year before").clone();
    assert_eq!(last.age, 23);
    let draw = |f: f64| present(now.clone(), &last, &species, f);

    // At the fraction's start the old organs are year 23's to the bit,
    // the new ones none, and nothing gone has faded.
    let start = draw(0.0);
    let year = organs(&Presented::whole(last.clone()));
    let mut new = 0;
    for (key, x) in organs(&start) {
        match year.get(&key) {
            Some(y) => assert_eq!(
                x.map(f64::to_bits)[..5],
                y.map(f64::to_bits)[..5],
                "{key:?}"
            ),
            None => {
                assert_eq!((x[4], x[5]), (0.0, 0.0), "a new organ starts from nothing");
                new += 1;
            }
        }
    }
    assert!(new > 0 && organs(&start).len() > year.len());
    // An axis that stays holds every phytomer it held.
    let held = organs(&Presented::whole(now.clone()));
    assert!(year
        .keys()
        .all(|key| held.contains_key(key) || !held.contains_key(&(key.0, 0))));

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
        let steps: Vec<f64> = [4e-4, 2e-4, 1e-4, 5e-5]
            .iter()
            .map(|&d| most(&at, &organs(&draw(near(d)))))
            .collect();
        for w in steps.windows(2) {
            assert!(w[1] <= 0.6 * w[0] + 1e-9, "at {f}: {steps:?}");
        }
    }
    assert!(
        most(
            &organs(&draw(1e-6)),
            &year.iter().map(|(k, v)| (*k, *v)).collect()
        ) < 1e-3
    );
    assert!(most(&organs(&draw(1.0 - 1e-6)), &whole) < 1e-3);
    assert_eq!(
        p.digest(),
        before,
        "a draw within the year moves the grower"
    );
    let drawn = p.settle_at(23.5).unwrap();
    assert_eq!(drawn, draw(0.5));
}

/// A subtree gone this year fades uniformly (host, 2026-10-15): a gone
/// twig a hair above the thinnest drawn keeps `1 - f` of its leaf and its
/// wood at f = 1e-6, as every other gone organ does, where the old law
/// cut its leaf by its wood's distance to `FINEST`.
#[test]
fn a_gone_twig_at_the_thinnest_drawn_fades_as_everything_gone() {
    let species = crate::oak();
    let mut p = Paused::begin(&species, request(24), Retiring::AtAge, false)
        .unwrap()
        .presenting();
    p.grow_to(24).unwrap();
    let now = p.settle().unwrap();
    let mut last = p.previous().expect("the year before").clone();
    let here = index(&now);
    let ids = identities(&last);
    let twig = (0..last.axes.len())
        .find(|&j| !here.contains_key(&ids[j]) && !last.axes[j].phytomers.is_empty())
        .expect("an axis gone this year");
    let r = FINEST + 1e-7;
    last.axes[twig].phytomers[0].radius = r;
    let f = 1e-6;
    let drawn = present(now, &last, &species, f);
    let drawn_ids = identities(&drawn.structure);
    let at = (drawn.gone..drawn.structure.axes.len())
        .find(|&i| drawn_ids[i] == ids[twig])
        .expect("the twig drawn, fading");
    assert_eq!(drawn.leaves[at][0], (1.0 - f) as f32, "its leaf");
    assert_eq!(
        drawn.structure.axes[at].phytomers[0].radius,
        r * (1.0 - f),
        "its wood"
    );
}

/// fn-286 review: a gone relay's join on a bearer gone with it fades with
/// everything gone, so wood above it judged too thin a year before stays
/// too thin for the fading cutoff (`FINEST` by `1 - f`) all the way; the
/// join no longer mixes towards its bearer's thinning wood, which drew a
/// stub there part-way through the year.
#[test]
fn a_join_gone_with_its_bearer_fades_with_it() {
    let species = crate::oak();
    // The oak at 27, seed 7, is the first from 20 with a relay gone with
    // its bearer.
    let mut p = Paused::begin(&species, request(27), Retiring::AtAge, false).unwrap().presenting();
    p.grow_to(27).unwrap();
    let now = p.settle().unwrap();
    let mut last = p.previous().expect("the year before").clone();
    let (here, ids) = (index(&now), identities(&last));
    let gone = |j: usize| !here.contains_key(&ids[j]);
    let relay = (0..last.axes.len())
        .find(|&j| {
            let a = &last.axes[j];
            gone(j) && a.joined.is_some() && a.origin.parent().is_some_and(gone)
        })
        .expect("a relay gone with its bearer");
    let (below, above) = (3e-4, 1.5e-4);
    let j = last.axes[relay].joined.as_mut().unwrap();
    (j.below, j.above) = (below, above);
    for f in [0.25, 0.5, 0.9] {
        let drawn = present(now.clone(), &last, &species, f);
        let drawn_ids = identities(&drawn.structure);
        let at = (drawn.gone..drawn.structure.axes.len())
            .find(|&i| drawn_ids[i] == ids[relay])
            .expect("the relay drawn, fading");
        let j = drawn.structure.axes[at].joined.expect("its join");
        let keep = kept(f);
        assert_eq!((j.below, j.above), (below * keep, above * keep), "at {f}");
        assert!(j.above < FINEST * keep, "at {f}: wood above the join drawn while it fades");
    }
}

/// R5c: an organ of both years holds last year's drawing until budburst
/// and this year's once its shoot has elongated; summer, autumn and
/// winter stand still but for what falls and fades.
#[test]
fn old_organs_move_only_while_the_shoots_grow() {
    let species = crate::oak();
    let mut p = Paused::begin(&species, request(12), Retiring::AtAge, false)
        .unwrap()
        .presenting();
    p.grow_to(12).unwrap();
    let now = p.settle().unwrap();
    let last = p.previous().expect("the year before").clone();
    let today = organs(&Presented::whole(now.clone()));
    let both: Vec<_> = organs(&Presented::whole(last.clone()))
        .into_keys()
        .filter(|k| today.contains_key(k))
        .collect();
    let draw = |f: f64| organs(&present(now.clone(), &last, &species, f));
    let held = |a: &HashMap<(u64, usize), [f64; 6]>, b: &HashMap<(u64, usize), [f64; 6]>| {
        let still = both
            .iter()
            .filter(|k| a.contains_key(k) && b.contains_key(k));
        still.filter(|k| a[k][..5] != b[k][..5]).count()
    };
    let seasons = species.states.iter().map(Season::of);
    let burst = seasons.clone().map(|s| s.budburst).fold(YEAR, f64::min) / YEAR;
    let grown = seasons
        .map(|s| s.budburst + s.elongation)
        .fold(0.0, f64::max)
        / YEAR;
    assert_eq!(
        held(&draw(0.0), &draw(burst * 0.99)),
        0,
        "the crown moved before budburst"
    );
    assert_eq!(
        held(&draw(grown + 1e-3), &draw(1.0 - 1e-6)),
        0,
        "the crown moved after the shoots"
    );
    assert!(
        held(&draw(0.0), &draw(1.0 - 1e-6)) > 0,
        "the crown never moved at all"
    );
}

/// R5c (host): a colouring and fall that would run past the next budburst
/// end at it, compressed by degree; one that fits is as stated.
#[test]
fn a_fall_past_budburst_is_compressed_to_end_at_it() {
    let mut state = crate::oak().states[0].clone();
    (
        state.budburst,
        state.senescence,
        state.colouring,
        state.leaf_fall,
    ) = (128.0, 264.0, 16.0, 45.0);
    (state.elongation, state.expansion) = (14.0, 35.0);
    let fits = Season::of(&state);
    assert_eq!(
        (fits.colouring, fits.fall),
        (state.colouring, state.leaf_fall)
    );
    let mut ends = Vec::new();
    for grow in [150.0, 200.0, 250.0, 300.0] {
        (state.elongation, state.expansion) = (grow / 2.0, grow / 2.0);
        let s = Season::of(&state);
        let end = s.senescence + s.colouring + s.fall;
        assert!(
            end <= s.budburst + 1e-9,
            "{grow}: {end} past {}",
            s.budburst
        );
        assert!(
            (s.colouring * state.leaf_fall - s.fall * state.colouring).abs() < 1e-9,
            "{grow}: one share of both"
        );
        ends.push(end);
    }
    assert!(ends.windows(2).all(|w| w[1] <= w[0]), "{ends:?}");
}
