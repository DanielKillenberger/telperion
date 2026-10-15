//! fn-252 R2: a shoot whose tip reaches the ground. While the rough
//! layout grown with the tree rests an apex's tip on the ground, its
//! yearly survival is its viability times (1 - ground_hazard): at 1 no
//! such tip lives through its next cycle; at 0 the tree is today's.
use std::collections::HashMap;
use telperion_space::{birch, grow, sketch, Light, Request, Species, Structure};

/// The site's light: the rough layout is grown only where light shades.
const SITE: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};
/// The birch's stem ages, which the rule leaves alone.
const STEM: [usize; 4] = [2, 3, 4, 5];

fn request(age: u32, light: Light) -> Request {
    Request {
        age,
        seed: 1,
        budget: 20_000_000,
        light,
    }
}

fn with_hazard(h: f64) -> Species {
    let mut species = birch();
    for s in &mut species.states {
        s.ground_hazard = h;
    }
    species
}

/// The living apexes, off the stem, whose tip the rough layout at `age`
/// rests on the ground, by lineage, age and birth.
fn resting(rough: &Structure) -> Vec<(u64, usize, u32)> {
    let rests = |a: &telperion_space::Axis| a.phytomers.last().is_some_and(|p| p.tip.z <= 0.0);
    (rough.axes.iter())
        .filter(|a| !STEM.contains(&a.pa) && a.apex_end.is_none() && rests(a))
        .map(|a| (a.lineage, a.pa, a.birth))
        .collect()
}

/// Of the apexes resting at `age`, those still living a cycle later.
fn survivors(species: &Species, age: u32) -> (usize, usize) {
    let rough = sketch(species, request(age, SITE)).unwrap();
    let rest = resting(&rough);
    let next = sketch(species, request(age + 1, SITE)).unwrap();
    let by: HashMap<(u64, usize, u32), bool> = (next.axes.iter())
        .map(|a| ((a.lineage, a.pa, a.birth), a.apex_end.is_none()))
        .collect();
    let living = rest
        .iter()
        .filter(|k| by.get(k).copied().unwrap_or(false))
        .count();
    (rest.len(), living)
}

/// At 1, no tip the rough layout rests on the ground lives through its
/// next cycle; at 0 most do (the young birch's strands run on).
#[test]
fn a_tip_on_the_ground_dies_at_one_and_runs_on_at_zero() {
    let (rest, living) = survivors(&with_hazard(0.0), 10);
    assert!(
        rest > 10 && living * 2 > rest,
        "at 0: {living} of {rest} live on"
    );
    let (rest, living) = survivors(&with_hazard(1.0), 10);
    assert!(rest > 0, "at 1 some tip still reaches the ground");
    assert_eq!(
        living, 0,
        "at 1: {living} of {rest} tips on the ground live on"
    );
}

/// The hazard reads the rough layout, which a ground hazard grows where
/// no leaf shades too: under neutral light it still ends the tips that
/// reach the ground, so the young birch grows less wood.
#[test]
fn a_ground_hazard_acts_without_shade() {
    let count = |h| {
        grow(&with_hazard(h), request(10, Light::NEUTRAL))
            .unwrap()
            .phytomer_count()
    };
    let (runs, stops) = (count(0.0), count(1.0));
    assert!(
        stops < runs,
        "neutral light: {stops} phytomers at 1, {runs} at 0"
    );
}
