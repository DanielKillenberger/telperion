//! fn-264 R2a: each preset's light rows proved where they live. Under the
//! site's light (`SITE`), a preset whose leaves carry area lowers the light
//! its own buds sample in the lattice; a preset with none leaves every bud
//! in the open sky's light. Leaf area alone, with no response row, changes
//! nothing the tree grows: the light is read only through a response.
use telperion_core::presets::{space::SITE, Preset};
use telperion_space::{grow, grow_capture_light_at, Request, Species};

/// Young enough to grow in a moment, old enough for a crown to shade.
const AGE: u32 = 15;
const SEED: u64 = 7;

fn species(preset: Preset) -> Species {
    preset
        .parameters()
        .space
        .species
        .expect("an engine species")
}

fn request() -> Request {
    Request {
        age: AGE,
        seed: SEED,
        light: SITE,
    }
}

/// The darkest light any living apex samples in the lattice in the last
/// cycle that reads it, as the one cycle reads it (`cpu_reads`).
fn darkest(species: &Species) -> f64 {
    let capture = grow_capture_light_at(species, request(), AGE - 1).unwrap();
    let (_, bytes) = capture.array("cpu_reads").expect("the reads");
    let lights: Vec<f32> = (bytes.chunks_exact(4))
        .map(|b| f32::from_le_bytes(b.try_into().expect("four bytes")))
        .collect();
    assert!(!lights.is_empty(), "a tree with living apices");
    lights.iter().map(|&l| f64::from(l)).fold(1.0, f64::min)
}

/// Every shipped preset carries leaf area (R2, SOURCES.md).
const SHADES: [(Preset, bool); 8] = [
    (Preset::OregonWhiteOak, true),
    (Preset::Ordinary, true),
    (Preset::EuropeanBeech, true),
    (Preset::Laurelin, true),
    (Preset::Telperion, true),
    (Preset::NorwaySpruce, true),
    (Preset::SilverBirch, true),
    (Preset::DatePalm, true),
];

#[test]
fn a_presets_foliage_lowers_the_light_its_buds_sample() {
    for (preset, shades) in SHADES {
        let species = species(preset);
        let leafy = species.states.iter().any(|s| s.leaf_area > 0.0);
        assert_eq!(leafy, shades, "{preset:?} carries leaf area");
        let darkest = darkest(&species);
        // With no leaf the swept lattice reads the open sky to f32's
        // rounding of the sky's shares, not exactly 1 as the unswept one did.
        match shades {
            true => assert!(darkest < 1.0 - 1e-3, "{preset:?}: darkest bud {darkest}"),
            false => assert!(darkest > 1.0 - 1e-6, "{preset:?}: darkest bud {darkest}"),
        }
    }
}

/// The palm, whose responses stay neutral by decision (no laterals to
/// shed, no published tolerance), grows under the site's light and its
/// fronds the tree it grew before either (no extinction, no leaf area):
/// its leaves shade, and nothing reads the shade.
#[test]
fn leaf_area_alone_changes_no_growth() {
    for preset in [Preset::DatePalm] {
        let lit = species(preset);
        let read = |s: &telperion_space::PaState| {
            s.shade_hazard != 0.0
                || s.shade_size != 0.0
                || s.balance_hazard != 0.0
                || s.leaf_girth != 0.0
        };
        assert!(!lit.states.iter().any(read), "{preset:?} reads no light");
        let mut bare = lit.clone();
        for s in &mut bare.states {
            s.leaf_area = 0.0;
        }
        let before = Request {
            light: telperion_space::Light::NEUTRAL,
            ..request()
        };
        let (a, b) = (grow(&lit, request()).unwrap(), grow(&bare, before).unwrap());
        assert!(a == b, "{preset:?}");
    }
}

/// Each response row a preset populates, set to its neutral; apical
/// control is architecture, each species' own (host, 2026-10-12).
const RESPONSES: [(&str, fn(&mut telperion_space::PaState)); 4] = [
    ("shade_hazard", |s| s.shade_hazard = 0.0),
    ("shade_size", |s| s.shade_size = 0.0),
    ("balance_hazard", |s| s.balance_hazard = 0.0),
    ("leaf_girth", |s| s.leaf_girth = 0.0),
];

/// The light-only response: with no leaf shading it reads a light of one,
/// where it is neutral to the bit. The shade size also follows the split
/// apical control makes in an even light, the balance charges its living
/// tips by their path, and the leaf girth's term at a light of one is not
/// bit-identical to its dormant path (measured), so those move an
/// unshaded tree too.
fn light_only(row: &str) -> bool {
    row == "shade_hazard"
}

/// Whether a preset populates `row`: the size response is no species'
/// but the oak's and the ordinary's, dormant elsewhere until a source or an
/// owner verdict sets it (host, 2026-10-12; fn-264 BIRCH.md).
fn populates(preset: Preset, row: &str) -> bool {
    row != "shade_size" || matches!(preset, Preset::OregonWhiteOak | Preset::Ordinary)
}

/// A species whose balance acts by 40: a potential height of 10 m, the top
/// of the span real trees stand in, so its charge sheds a short tree's
/// laterals by the age the test grows (fn-349). The presets' own heights
/// first shed between 40 and 60.
fn acting(mut species: Species) -> Species {
    species
        .states
        .iter_mut()
        .for_each(|s| s.inverse_height = 0.1);
    species
}

/// R2a on every preset whose response rows are populated, at 40, each with
/// the balance acting (`acting`): the Two Trees carry the beech's species
/// (`space_presets.rs`), so the beech stands for them;
/// each row moves the tree through its consumer under the site's light; and
/// the light-only row moves nothing where no leaf shades.
#[test]
fn each_populated_response_moves_the_tree_through_its_consumer() {
    let populated = [
        Preset::OregonWhiteOak,
        Preset::Ordinary,
        Preset::EuropeanBeech,
        Preset::NorwaySpruce,
        Preset::SilverBirch,
    ];
    for preset in populated {
        let full = acting(species(preset));
        let lit_at = Request {
            age: 40,
            ..request()
        };
        let unlit = Request {
            light: telperion_space::Light::NEUTRAL,
            ..lit_at
        };
        let (lit_tree, dark_tree) = (grow(&full, lit_at).unwrap(), grow(&full, unlit).unwrap());
        for (row, neutral) in RESPONSES {
            let mut without = full.clone();
            without.states.iter_mut().for_each(neutral);
            assert_eq!(
                without != full,
                populates(preset, row),
                "{preset:?} populates {row}"
            );
            if !populates(preset, row) {
                continue;
            }
            let moves = grow(&without, lit_at).unwrap() != lit_tree;
            assert!(moves, "{preset:?}: {row} under the site's light");
            if light_only(row) {
                let dark = grow(&without, unlit).unwrap();
                assert!(dark == dark_tree, "{preset:?}: {row} is neutral unshaded");
            }
        }
    }
}
