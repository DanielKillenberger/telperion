//! The engine species as preset values (fn-198 host decision 1): each of the
//! four engine presets carries the engine's own species, stem ages and site
//! light as `/space` rows, read back to the bit through the value file and
//! the wire; the other presets carry none.
use telperion_core::{
    blend, params,
    presets::{
        space::{self, Cell, Space},
        values, Preset,
    },
};
use telperion_space::{beech, oak, palm, settings, spruce, Light, Species};

/// The species each engine preset states, as the tree-space examples grew it
/// (`crates/telperion-render/examples/space_*.rs`, BASELINE.md's oak light).
fn engine() -> [(Preset, Species, Vec<usize>, Light); 4] {
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

#[test]
fn a_preset_without_an_engine_species_carries_none() {
    for preset in [Preset::SilverBirch, Preset::Telperion, Preset::Laurelin] {
        assert_eq!(preset.parameters().space, Space::default(), "{preset:?}");
    }
}

/// The ordinary tree's space is the equal-weight centroid of the passed
/// branching species on the space's own walk (fn-215 R1): the k-th corner a
/// share 1/k of the way from the mean of those before, any order alike.
/// The value file is written by `examples/centroid.rs`.
#[test]
fn the_ordinary_tree_is_the_centroid_of_the_branching_species() {
    let corners = [
        Preset::EuropeanBeech,
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
    ];
    let centroid = |order: [usize; 3]| {
        let mut mean = corners[order[0]].parameters();
        for (k, &i) in order.iter().enumerate().skip(1) {
            let corner = corners[i].parameters();
            mean = blend::families(&mean, &corner, 1.0 / (k + 1) as f64).unwrap();
        }
        mean.space
    };
    let ordinary = Preset::Ordinary.parameters().space;
    assert_eq!(ordinary, centroid([0, 1, 2]));
    let other = centroid([2, 0, 1]);
    assert_eq!(other.trunk, ordinary.trunk);
    let near = |name: &str, p: f64, q: f64| {
        let close = p == q || (p - q).abs() <= 1e-9 * p.abs().max(1.0);
        assert!(close, "{name}: {p} {q}");
    };
    near(
        "extinction",
        other.light.extinction,
        ordinary.light.extinction,
    );
    near("sky", other.light.sky, ordinary.light.sky);
    let (a, b) = (ordinary.species.unwrap(), other.species.unwrap());
    for (x, y) in a.states.iter().zip(&b.states) {
        let (mut x, mut y) = (x.clone(), y.clone());
        for ((name, p), (_, q)) in settings(&mut x).into_iter().zip(settings(&mut y)) {
            near(name, *p, *q);
        }
    }
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
