//! fn-230: every point between species is a tree of its ends' size. A blend
//! between engine species is every row mixed, then one factor on the
//! lateral probabilities, shrinking only, so the light-neutral expected kept
//! phytomers at the blend's age are the ends' weighted geometric mean (host,
//! 2026-10-08). Built frames and their strips are the evidence sheet's; these
//! tests read the prediction, which costs no growth.
use telperion_core::{blend, presets::Preset, Family};
use telperion_space::{expected_kept, Species};

/// The passed branching species and the ordinary tree (fn-230 R2).
const BRANCHING: [Preset; 4] = [
    Preset::EuropeanBeech,
    Preset::OregonWhiteOak,
    Preset::NorwaySpruce,
    Preset::Ordinary,
];

fn at(preset: Preset, age: f64) -> Family {
    let mut family = preset.parameters();
    family.age = age;
    family
}

/// Two families alike in every row (a family has no equality of its own).
fn same(x: &Family, y: &Family) {
    assert_eq!(format!("{x:?}"), format!("{y:?}"));
}

fn species(family: &Family) -> &Species {
    family.space.species.as_ref().expect("an engine preset")
}

fn kept(species: &Species, age: u32) -> f64 {
    let mut neutral = species.clone();
    for s in &mut neutral.states {
        (s.shade_hazard, s.shade_size, s.balance_hazard, s.leaf_girth) = (0.0, 0.0, 0.0, 0.0);
    }
    expected_kept(&neutral, age).unwrap().unwrap()
}

/// Every lateral probability of `sized` over the bare mix's, which must be
/// one factor.
fn factor(sized: &Species, mixed: &Species) -> f64 {
    let pairs = sized.states.iter().zip(&mixed.states).flat_map(|(s, m)| {
        s.zones
            .iter()
            .zip(&m.zones)
            .flat_map(|(p, q)| p.lateral.iter().zip(&q.lateral))
    });
    let ratios: Vec<f64> = pairs
        .filter(|(_, q)| **q > 0.0)
        .map(|(p, q)| p / q)
        .collect();
    for r in &ratios {
        assert!((r / ratios[0] - 1.0).abs() < 1e-12, "{ratios:?}");
    }
    ratios[0]
}

/// R1, R2: every point at a quarter, half and three quarters between two
/// branching species, at 40 and 80, is predicted at its ends' weighted
/// geometric mean, by a factor below one on the mix the engine's own walk
/// gives; every other row is that walk's.
#[test]
fn a_walk_between_branching_species_keeps_its_ends_size_at_40_and_80() {
    for age in [40, 80] {
        for (i, &p) in BRANCHING.iter().enumerate() {
            for &q in &BRANCHING[i + 1..] {
                let (a, b) = (at(p, f64::from(age)), at(q, f64::from(age)));
                let ts: &[f64] = match (age, q) {
                    // The ordinary tree's predictions cost a second each at 80.
                    (80, Preset::Ordinary) => &[0.5],
                    _ => &[0.25, 0.5, 0.75],
                };
                for &t in ts {
                    let walked = blend::families(&a, &b, t).unwrap();
                    let mixed = telperion_space::blend(species(&a), species(&b), t).unwrap();
                    let f = factor(species(&walked), &mixed);
                    let mut bare = species(&walked).clone();
                    for (s, m) in bare.states.iter_mut().zip(&mixed.states) {
                        for (z, y) in s.zones.iter_mut().zip(&m.zones) {
                            z.lateral.clone_from(&y.lateral);
                        }
                    }
                    assert_eq!(bare, mixed, "{p:?} to {q:?} at {t}, {age}");
                    let logs =
                        (1.0 - t) * kept(species(&a), age).ln() + t * kept(species(&b), age).ln();
                    let size = kept(species(&walked), age) / logs.exp();
                    let name = format!("{p:?} to {q:?} at {t}, {age}: factor {f}, size {size}");
                    assert!(f < 1.0, "{name}");
                    assert!((size - 1.0).abs() < 1e-9, "{name}");
                }
            }
        }
    }
}

/// R1: the factor is exactly 1 at either end, and the size is continuous off
/// it. The engine's own walk is not: a millionth of the way from the beech
/// to the spruce it is predicted 20 times the beech, because an age only
/// the spruce lives in starts being lived (fn-230's evidence). The factor
/// takes that jump out of the size, so it is far below 1 right off the end.
#[test]
fn the_size_is_neutral_at_either_end_and_continuous_off_it() {
    let (a, b) = (
        at(Preset::EuropeanBeech, 40.0),
        at(Preset::NorwaySpruce, 40.0),
    );
    same(&blend::families(&a, &b, 0.0).unwrap(), &a);
    same(&blend::families(&a, &b, 1.0).unwrap(), &b);
    for (t, end) in [(1e-6, &a), (1.0 - 1e-6, &b)] {
        let walked = kept(species(&blend::families(&a, &b, t).unwrap()), 40);
        let engine_walk = telperion_space::blend(species(&a), species(&b), t).unwrap();
        let near = kept(species(end), 40);
        assert!(
            (walked / near - 1.0).abs() < 1e-5,
            "at {t}: {walked} {near}"
        );
        assert!(
            kept(&engine_walk, 40) > 1.5 * near,
            "the engine's walk mended"
        );
    }
}

/// Host decision 1 (2026-10-08): the factor only shrinks. A walk to the
/// date palm, whose size is fronds and not laterals, is predicted below
/// its ends' mean and keeps the engine's walk as it is; at its middle it is
/// smaller than either end, a known property and not a defect.
#[test]
fn a_walk_to_the_palm_is_never_grown_to_its_ends_size() {
    let age = 40;
    let palm = at(Preset::DatePalm, 40.0);
    for p in BRANCHING {
        let a = at(p, 40.0);
        let walked = blend::families(&a, &palm, 0.5).unwrap();
        let mixed = telperion_space::blend(species(&a), species(&palm), 0.5).unwrap();
        assert_eq!(species(&walked), &mixed, "{p:?}");
    }
    let beech = at(Preset::EuropeanBeech, 40.0);
    let middle = kept(species(&blend::families(&beech, &palm, 0.5).unwrap()), age);
    assert!(middle < kept(species(&palm), age), "{middle}");
}

/// Host decision 2: a two-way walk is the N = 2 point, and weights that name
/// no point are refused by name.
#[test]
fn a_walk_is_the_two_point_centroid_and_bad_weights_are_refused() {
    let (a, b) = (
        at(Preset::OregonWhiteOak, 40.0),
        at(Preset::NorwaySpruce, 40.0),
    );
    same(
        &blend::centroid(&[(&a, 0.75), (&b, 0.25)]).unwrap(),
        &blend::families(&a, &b, 0.25).unwrap(),
    );
    same(&blend::centroid(&[(&a, 0.0), (&b, 2.0)]).unwrap(), &b);
    for weights in [
        [-1.0, 2.0],
        [f64::NAN, 1.0],
        [0.0, 0.0],
        [f64::INFINITY, 1.0],
    ] {
        let refused = blend::centroid(&[(&a, weights[0]), (&b, weights[1])]);
        assert_eq!(
            refused.unwrap_err().to_string(),
            "invalid input: blend weights",
            "{weights:?}"
        );
    }
    assert!(blend::centroid(&[]).is_err());
}
