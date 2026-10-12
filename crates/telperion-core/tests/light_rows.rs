//! fn-264 R2a: each preset's light rows proved where they live. Under the
//! site's light (`SITE`), a preset whose leaves carry area lowers the light
//! its own buds sample in the lattice; a preset with none leaves every bud
//! in the open sky's light. Leaf area alone, with no response row, changes
//! nothing the tree grows: the light is read only through a response.
use telperion_core::presets::{space::SITE, Preset};
use telperion_space::{bud_light, grow, Request, Species};

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
        budget: 20_000_000,
        light: SITE,
    }
}

/// The darkest light any living apex samples.
fn darkest(species: &Species) -> f64 {
    let structure = grow(species, request()).unwrap();
    let lights = bud_light(&structure, species, &SITE);
    assert!(!lights.is_empty(), "a tree with living apices");
    lights.iter().map(|&(_, l)| l).fold(1.0, f64::min)
}

/// Every shipped preset by whether its leaves carry area (R2): the
/// spruce, the birch and the palm wait on the host (fn-264 NEEDS_HUMAN).
const SHADES: [(Preset, bool); 8] = [
    (Preset::OregonWhiteOak, true),
    (Preset::Ordinary, true),
    (Preset::EuropeanBeech, true),
    (Preset::Laurelin, true),
    (Preset::Telperion, true),
    (Preset::NorwaySpruce, false),
    (Preset::SilverBirch, false),
    (Preset::DatePalm, false),
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

/// A species with no response row grows, under the site's light and with
/// its leaf area, the tree it grew before either (no extinction, no leaf
/// area): its leaves shade, and nothing reads the shade.
#[test]
fn leaf_area_alone_changes_no_growth() {
    let unread = [
        Preset::EuropeanBeech,
        Preset::Laurelin,
        Preset::Telperion,
        Preset::NorwaySpruce,
        Preset::SilverBirch,
        Preset::DatePalm,
    ];
    for preset in unread {
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

/// Each response row a preset populates, set to its neutral.
const RESPONSES: [(&str, fn(&mut telperion_space::PaState)); 5] = [
    ("shade_hazard", |s| s.shade_hazard = 0.0),
    ("shade_size", |s| s.shade_size = 0.0),
    ("apical_control", |s| s.apical_control = 0.5),
    ("balance_hazard", |s| s.balance_hazard = 0.0),
    ("leaf_girth", |s| s.leaf_girth = 0.0),
];

/// The light-only response: with no leaf shading it reads a light of one,
/// where it is neutral to the bit. The vigour rows (shade size, apical
/// control) also follow the split apical control makes in an even light,
/// the balance weighs its wood's upkeep, and the leaf girth's term at a
/// light of one is not bit-identical to its dormant path (measured), so
/// those move an unshaded tree too.
fn light_only(row: &str) -> bool {
    row == "shade_hazard"
}

/// The rows a preset's own values leave neutral under the site's light:
/// the ordinary's upkeep (0.006) keeps every balance above its tolerance
/// (0), so its balance hazard sheds nothing.
fn neutral_by_its_values(preset: Preset, row: &str) -> bool {
    preset == Preset::Ordinary && row == "balance_hazard"
}

/// R2a on the presets whose response rows are populated (the oak, the
/// ordinary), at their own age (40, where the oak's balance has shed):
/// each row moves the tree through its consumer under the site's light,
/// but where its values make it neutral, which is stated and tested; and
/// the light-only row moves nothing where no leaf shades.
#[test]
fn each_populated_response_moves_the_tree_through_its_consumer() {
    for preset in [Preset::OregonWhiteOak, Preset::Ordinary] {
        let full = species(preset);
        let lit_at = Request {
            age: preset.parameters().age as u32,
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
            assert!(without != full, "{preset:?} populates {row}");
            let moves = grow(&without, lit_at).unwrap() != lit_tree;
            assert_eq!(
                moves,
                !neutral_by_its_values(preset, row),
                "{preset:?}: {row} under the site's light"
            );
            if light_only(row) {
                let dark = grow(&without, unlit).unwrap();
                assert!(dark == dark_tree, "{preset:?}: {row} is neutral unshaded");
            }
        }
    }
}
