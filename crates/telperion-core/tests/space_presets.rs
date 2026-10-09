//! The engine species as preset values (fn-198 host decision 1): every
//! preset carries the engine's own species, stem ages and site light as
//! `/space` rows, read back to the bit through the value file and the wire.
use telperion_core::{
    blend, params,
    presets::{
        space::{self, Cell},
        values, Preset,
    },
};
use telperion_space::{
    beech, birch, expected_kept, oak, palm, settings, spruce, zone_settings, Light, Species,
};

/// The species each engine preset states, as the tree-space examples grew it
/// (`crates/telperion-render/examples/space_*.rs`, BASELINE.md's oak light).
fn engine() -> [(Preset, Species, Vec<usize>, Light); 7] {
    let oak_light = Light {
        extinction: 0.5,
        sky: 0.5,
    };
    [
        (
            Preset::EuropeanBeech,
            beech(),
            vec![2, 4, 5],
            Light::NEUTRAL,
        ),
        (
            Preset::NorwaySpruce,
            spruce(),
            vec![0, 1, 2, 4],
            Light::NEUTRAL,
        ),
        (Preset::OregonWhiteOak, oak(), vec![2, 3, 4], oak_light),
        (Preset::DatePalm, palm(), vec![2], Light::NEUTRAL),
        (
            Preset::SilverBirch,
            birch(),
            vec![2, 3, 4, 5],
            Light::NEUTRAL,
        ),
        // The Two Trees' natural base is the beech (fn-218, fn-219).
        (Preset::Laurelin, beech(), vec![2, 4, 5], Light::NEUTRAL),
        (Preset::Telperion, beech(), vec![2, 4, 5], Light::NEUTRAL),
    ]
}

#[test]
fn every_engine_preset_carries_the_engines_species() {
    for (preset, species, trunk, light) in engine() {
        let space = preset.parameters().space;
        assert_eq!(space.species.as_ref(), Some(&species), "{preset:?}");
        assert_eq!((space.trunk, space.light), (trunk, light), "{preset:?}");
    }
}

/// The ordinary tree's space is the passed branching species' point at
/// equal weights (fn-215 R1, fn-230 R3): the space's one blend rule,
/// `blend::centroid`, with every corner at 80, the age its size is matched
/// at. Its shape is the centroid of every row, any order alike; its size
/// the corners' geometric mean, its lateral probabilities all multiplied by
/// one factor so its light-neutral expected kept count at 80 is theirs
/// (host, 2026-10-08). The value file is written by `examples/centroid.rs`
/// in the order beech, oak, spruce; this reads it back in another.
#[test]
#[ignore = "fn-231 (host, 2026-10-09): the ordinary tree keeps its pre-fn-231 rows; \
            a fresh solve on the continuous walk reads as a hedge at 80, so its sizing \
            measure is the owner's question (ENDS.md, seventh answer)"]
fn the_ordinary_tree_is_the_centroid_of_the_branching_species() {
    let corners = [
        Preset::NorwaySpruce,
        Preset::EuropeanBeech,
        Preset::OregonWhiteOak,
    ]
    .map(|p| {
        let mut corner = p.parameters();
        corner.age = 80.0;
        corner
    });
    let points: Vec<_> = corners.iter().map(|c| (c, 1.0)).collect();
    let shape = blend::centroid(&points).unwrap().space;
    let near = |name: &str, p: f64, q: f64| {
        let close = p == q || (p - q).abs() <= 1e-9 * p.abs().max(1.0);
        assert!(close, "{name}: {p} {q}");
    };
    let ordinary = Preset::Ordinary.parameters().space;
    assert_eq!(ordinary.trunk, shape.trunk);
    near(
        "extinction",
        ordinary.light.extinction,
        shape.light.extinction,
    );
    near("sky", ordinary.light.sky, shape.light.sky);
    let (a, b) = (ordinary.species.unwrap(), shape.species.unwrap());
    for (x, y) in a.states.iter().zip(&b.states) {
        let (mut x, mut y) = (x.clone(), y.clone());
        for ((name, p), (_, q)) in settings(&mut x).into_iter().zip(settings(&mut y)) {
            near(name, *p, *q);
        }
        for (p, q) in x.zones.iter_mut().zip(&mut y.zones) {
            let both = zone_settings(p).unwrap().into_iter();
            for ((name, p), (_, q)) in both.zip(zone_settings(q).unwrap()) {
                near(name, *p, *q);
            }
        }
    }
    let kept = |species: &Species| {
        let mut neutral = species.clone();
        for s in &mut neutral.states {
            (s.shade_hazard, s.shade_size, s.balance_hazard, s.leaf_girth) = (0.0, 0.0, 0.0, 0.0);
        }
        expected_kept(&neutral, 80).unwrap().unwrap()
    };
    let logs: f64 = corners
        .iter()
        .map(|c| kept(c.space.species.as_ref().unwrap()).ln())
        .sum();
    let target = (logs / 3.0).exp();
    assert!(
        (kept(&a) / target - 1.0).abs() < 1e-9,
        "{} {target}",
        kept(&a)
    );
}

#[test]
fn a_species_reads_back_from_its_rows_the_value_file_and_the_wire() {
    let ordinary = Preset::Ordinary;
    for preset in engine()
        .map(|(preset, ..)| preset)
        .into_iter()
        .chain([ordinary])
    {
        let family = preset.parameters();
        let rows = space::rows(&family.space).unwrap();
        let read = space::read(rows.iter().map(|(p, c)| (p.as_str(), *c))).unwrap();
        assert_eq!(read, family.space, "{preset:?} rows");
        let text = values::write(&family, "", "").unwrap();
        assert_eq!(
            values::read(&text).unwrap().space,
            family.space,
            "{preset:?} file"
        );
        let wire = params::parse(&params::metadata(&family)).unwrap();
        assert_eq!(wire.space, family.space, "{preset:?} wire");
    }
}

#[test]
fn a_space_row_off_its_place_or_form_is_refused_by_name() {
    let refused = |rows: &[(&str, Cell)]| space::read(rows.iter().copied()).unwrap_err();
    let life = ("/space/ages/2/lifespan", Cell::Number(3.0));
    for (row, why) in [
        (
            ("/space/ages/2/internode", Cell::Never),
            "/space/ages/2/internode is never only as a shedding delay",
        ),
        (
            ("/space/ages/15/lifespan", Cell::Number(1.0)),
            "/space/ages/15/lifespan: 15 is not an index below 15",
        ),
        (
            ("/space/ages/2/girth", Cell::Number(1.0)),
            "/space/ages/2/girth is not a space row",
        ),
        (
            ("/space/ages/2/trunk", Cell::Number(1.0)),
            "/space/ages/2/trunk holds true or false",
        ),
        (
            ("/space/ages/2/zones/4/buds", Cell::Number(1.0)),
            "/space/ages/2/zones/4/buds: 4 is not an index below 4",
        ),
    ] {
        assert_eq!(refused(&[life, row]), why);
    }
    assert_eq!(
        refused(&[("/space/ages/3/internode", Cell::Number(0.1))]),
        "/space/ages/3 states an age with no lifespan"
    );
    let never = [life, ("/space/ages/2/shedding", Cell::Never)];
    let read = space::read(never).unwrap().species.unwrap();
    assert_eq!(read.states[2].shedding, f64::INFINITY);
}
