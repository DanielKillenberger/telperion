//! fn-230: every point between species is a tree of its ends' size. A blend
//! between engine species is every row mixed, then one factor on the
//! lateral probabilities, shrinking only, so neither the light-neutral
//! expected kept nor grown phytomers at the blend's age pass the ends'
//! weighted geometric mean of them (host, 2026-10-08). Built frames and their strips are the evidence sheet's; these
//! tests read the prediction, which costs no growth.
use telperion_core::{blend, presets::Preset, Family};
use telperion_space::{expected_counts, expected_kept, Species};

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

fn neutral(species: &Species) -> Species {
    let mut neutral = species.clone();
    for s in &mut neutral.states {
        (s.shade_hazard, s.shade_size, s.balance_hazard, s.leaf_girth) = (0.0, 0.0, 0.0, 0.0);
    }
    neutral
}

fn kept(species: &Species, age: u32) -> f64 {
    expected_kept(&neutral(species), age).unwrap().unwrap()
}

/// Grown phytomers, the shed included: what the engine's budget counts.
fn grown(species: &Species, age: u32) -> f64 {
    let counts = expected_counts(&neutral(species), age).unwrap();
    (0..species.states.len()).map(|pa| counts.total(pa)).sum()
}

/// The point's kept and grown predictions over the ends' weighted geometric
/// means of them.
fn sizes(point: &Species, a: &Species, b: &Species, t: f64, age: u32) -> [f64; 2] {
    [kept, grown].map(|count: fn(&Species, u32) -> f64| {
        let logs = (1.0 - t) * count(a, age).ln() + t * count(b, age).ln();
        count(point, age) / logs.exp()
    })
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

/// R1, R2: points between two branching species at 40 and 80 are predicted
/// at their ends' weighted geometric mean in one count and at most it in the
/// other, by a factor below one on the mix the engine's own walk gives, or
/// at most it in both with no factor where that walk is no larger (host
/// decision 1: only shrink; since fn-231 made the walk continuous, many
/// points are); every other row is that walk's.
#[test]
fn a_walk_between_branching_species_keeps_its_ends_size_at_40_and_80() {
    for age in [40, 80] {
        for (i, &p) in BRANCHING.iter().enumerate() {
            for &q in &BRANCHING[i + 1..] {
                let (a, b) = (at(p, f64::from(age)), at(q, f64::from(age)));
                // A grown prediction costs 5 to 7 s on most blends at 80
                // (SHEET.md): there the middle only, and of the ordinary
                // tree's walks the beech's; every frame is built in the sheet.
                let ts: &[f64] = match (age, p, q) {
                    (80, Preset::EuropeanBeech, Preset::Ordinary) => &[0.5],
                    (80, _, Preset::Ordinary) => &[],
                    (80, ..) => &[0.5],
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
                    let [kept, grown] = sizes(species(&walked), species(&a), species(&b), t, age);
                    let name = format!("{p:?} to {q:?} at {t}, {age}: factor {f}, {kept} {grown}");
                    assert!(f <= 1.0, "{name}");
                    assert!(kept < 1.0 + 1e-9 && grown < 1.0 + 1e-9, "{name}");
                    assert!(f == 1.0 || (kept.max(grown) - 1.0).abs() < 1e-9, "{name}");
                }
            }
        }
    }
}

/// Host decision (2026-10-08, option A): kept phytomers do not bound grown
/// ones, since a blend can shed a larger share than either end. Three
/// quarters of the way from the beech to the spruce at 80, the grown count
/// binds while the kept one is 5 % under the ends' mean. (Before fn-231 the
/// case was the oak to the spruce, predicted to grow 21.7M phytomers sized
/// on kept alone; the continuous walk puts that point under its ends.)
#[test]
fn the_grown_count_binds_where_a_blend_sheds_more_than_its_ends() {
    let (a, b) = (
        at(Preset::EuropeanBeech, 80.0),
        at(Preset::NorwaySpruce, 80.0),
    );
    let walked = blend::families(&a, &b, 0.75).unwrap();
    let [kept_share, grown_share] = sizes(species(&walked), species(&a), species(&b), 0.75, 80);
    assert!(
        (grown_share - 1.0).abs() < 1e-9 && kept_share < 0.99,
        "{kept_share} {grown_share}"
    );
    let predicted = grown(species(&walked), 80);
    assert!(predicted < 20_000_000.0, "{predicted}");
}

/// fn-230 R1: the factor is exactly 1 at either end, and the size is
/// continuous off it.
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
        let near = kept(species(end), 40);
        assert!(
            (walked / near - 1.0).abs() < 1e-5,
            "at {t}: {walked} {near}"
        );
    }
}

/// The raw engine's predicted kept and grown phytomers and its expected
/// metres of wood (its phytomers weighted by their size).
fn measures(species: &Species, age: u32) -> [f64; 3] {
    let length = telperion_space::expected_length(&neutral(species), age).unwrap();
    [kept(species, age), grown(species, age), length]
}

/// fn-231 R3: the engine's own walk (`telperion_space::blend`, before
/// fn-230's size factor) is continuous at both its ends, for every pair of
/// the passed branching species and the ordinary tree at 40 and 80: a
/// hundredth, a ten-thousandth and a millionth of the way off either end,
/// its kept and grown counts and its wood draw in on the end's, within
/// 1e-4 at a millionth. Before fn-231 a millionth off the beech towards
/// the spruce kept 19.6 times the beech at 40 and 85 times at 80.
#[test]
fn the_engines_walk_converges_on_either_end_at_40_and_80() {
    let pairs: Vec<(Preset, Preset)> = (0..BRANCHING.len())
        .flat_map(|i| (i + 1..BRANCHING.len()).map(move |j| (BRANCHING[i], BRANCHING[j])))
        .collect();
    let failed: Vec<String> = std::thread::scope(|scope| {
        let jobs: Vec<_> = pairs
            .iter()
            .flat_map(|&pair| [40, 80].map(|age| (pair, age)))
            .map(|((p, q), age)| scope.spawn(move || converges(p, q, age)))
            .collect();
        jobs.into_iter().flat_map(|j| j.join().unwrap()).collect()
    });
    assert!(failed.is_empty(), "{failed:#?}");
}

/// Where the walk from `p` to `q` at `age` fails to draw in on its ends.
fn converges(p: Preset, q: Preset, age: u32) -> Vec<String> {
    let (a, b) = (at(p, f64::from(age)), at(q, f64::from(age)));
    let (a, b) = (species(&a), species(&b));
    let mut failed = Vec::new();
    for (end, toward) in [(0.0, 1.0), (1.0, -1.0)] {
        let exact = measures(if end == 0.0 { a } else { b }, age);
        let off: Vec<[f64; 3]> = [1e-2, 1e-4, 1e-6]
            .iter()
            .map(|d| {
                let walked = telperion_space::blend(a, b, end + toward * d).unwrap();
                let m = measures(&walked, age);
                std::array::from_fn(|k| (m[k] / exact[k] - 1.0).abs())
            })
            .collect();
        for k in 0..3 {
            let (far, mid, near) = (off[0][k], off[1][k], off[2][k]);
            if near > 1e-4 || near > mid.max(1e-9) || mid > far.max(1e-9) {
                failed.push(format!("{p:?} to {q:?} at {age}, end {end}, measure {k}: {far:e} {mid:e} {near:e}"));
            }
        }
    }
    failed
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
    // Weights weigh as their ratios, however large.
    same(
        &blend::centroid(&[(&a, 5e307), (&b, 5e307)]).unwrap(),
        &blend::centroid(&[(&a, 1.0), (&b, 1.0)]).unwrap(),
    );
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

/// A first year grows no laterals, so in its first year a point between two
/// species can grow more than its ends' mean with no lateral to shrink: it
/// is a tree all the same, with its laterals at none, the nearest the factor
/// comes (Codex on fn-230).
#[test]
fn a_first_year_blend_is_a_tree_though_no_factor_reaches_its_size() {
    for (p, q) in [
        (Preset::EuropeanBeech, Preset::OregonWhiteOak),
        (Preset::EuropeanBeech, Preset::NorwaySpruce),
    ] {
        let (a, b) = (at(p, 1.0), at(q, 1.0));
        let walked = blend::families(&a, &b, 0.5).unwrap();
        let laterals = species(&walked)
            .states
            .iter()
            .flat_map(|s| &s.zones)
            .flat_map(|z| &z.lateral);
        assert!(laterals.copied().all(|l| l == 0.0), "{p:?} to {q:?}");
    }
}
