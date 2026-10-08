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
use telperion_space::{
    beech, expected_kept, oak, palm, settings, spruce, zone_settings, Light, Species,
};

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

/// The ordinary tree's space is the passed branching species' centroid
/// (fn-215 R1): its shape the equal-weight mean on the space's own walk,
/// the k-th corner a share 1/k of the way from the mean of those before,
/// any order alike; its size the corners' geometric mean, its lateral
/// probabilities all multiplied by one factor so its light-neutral expected
/// kept count at 80 is theirs (host, 2026-10-08). The value file is written
/// by `examples/centroid.rs`.
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
    let near = |name: &str, p: f64, q: f64| {
        let close = p == q || (p - q).abs() <= 1e-9 * p.abs().max(1.0);
        assert!(close, "{name}: {p} {q}");
    };
    let (shape, other) = (centroid([0, 1, 2]), centroid([2, 0, 1]));
    let ordinary = Preset::Ordinary.parameters().space;
    assert_eq!(
        (&ordinary.trunk, ordinary.light),
        (&shape.trunk, shape.light)
    );
    assert_eq!(other.trunk, shape.trunk);
    near("extinction", other.light.extinction, shape.light.extinction);
    let (a, b) = (ordinary.species.unwrap(), shape.species.unwrap());
    let c = other.species.unwrap();
    let factor = a.states[2].zones[1].lateral[9] / b.states[2].zones[1].lateral[9];
    assert!(factor > 0.0 && factor < 1.0, "{factor}");
    for ((x, y), z) in a.states.iter().zip(&b.states).zip(&c.states) {
        let (mut x, mut y, mut z) = (x.clone(), y.clone(), z.clone());
        let named = settings(&mut x).into_iter().zip(settings(&mut y));
        for (((name, p), (_, q)), (_, r)) in named.zip(settings(&mut z)) {
            assert_eq!(p.to_bits(), q.to_bits(), "{name}");
            near(name, *q, *r);
        }
        for (p, q) in x.zones.iter_mut().zip(&mut y.zones) {
            let both = zone_settings(p).unwrap().into_iter();
            for ((name, p), (_, q)) in both.zip(zone_settings(q).unwrap()) {
                let scale = if name == "zone.lateral" { factor } else { 1.0 };
                near(name, *p, *q * scale);
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
        .map(|p| kept(&p.parameters().space.species.unwrap()).ln())
        .sum();
    let target = (logs / 3.0).exp();
    assert!(
        (kept(&a) / target - 1.0).abs() < 1e-6,
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
