//! The engine species as preset values (fn-198 host decision 1): every
//! preset carries the engine's own species and stem ages as `/space` rows,
//! read back to the bit through the value file and the wire, and grows
//! under the one site light no row states (fn-264 R1a).
use telperion_core::{
    blend, params,
    presets::{
        space::{self, Cell, SITE},
        values, Preset,
    },
};
use telperion_space::{
    beech, birch, expected_kept, oak, palm, settings, spruce, zone_settings, Species,
};

/// The species each engine preset states, as the tree-space examples grew it
/// (`crates/telperion-render/examples/space_*.rs`); every one grows under the
/// site's light (fn-264 R1a).
fn engine() -> [(Preset, Species, Vec<usize>); 7] {
    [
        (Preset::EuropeanBeech, beech(), vec![2, 4, 5]),
        (Preset::NorwaySpruce, spruce(), vec![0, 1, 2, 4]),
        (Preset::OregonWhiteOak, oak(), vec![2, 3, 4]),
        (Preset::DatePalm, palm(), vec![2]),
        (Preset::SilverBirch, birch(), vec![2, 3, 4, 5]),
        // The Two Trees' natural base is the beech (fn-218, fn-219).
        (Preset::Laurelin, beech(), vec![2, 4, 5]),
        (Preset::Telperion, beech(), vec![2, 4, 5]),
    ]
}

#[test]
fn every_engine_preset_carries_the_engines_species() {
    for (preset, species, trunk) in engine() {
        let space = preset.parameters().space;
        assert_eq!(space.species.as_ref(), Some(&species), "{preset:?}");
        assert_eq!((space.trunk, space.light), (trunk, SITE), "{preset:?}");
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
            "/space/ages/2/internode is never only as a shedding delay or a leaf's years",
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
        (
            ("/space/light/extinction", Cell::Number(1.0 / 6.0)),
            "/space/light/extinction: the site's light is one setting, no preset's row",
        ),
        (
            ("/space/light/sky", Cell::Number(0.0)),
            "/space/light/sky: the site's light is one setting, no preset's row",
        ),
    ] {
        assert_eq!(refused(&[life, row]), why);
    }
    assert_eq!(
        refused(&[("/space/ages/3/internode", Cell::Number(0.1))]),
        "/space/ages/3 states an age with no lifespan"
    );
    // A family written before fn-264 states the site's own light: read.
    let site = [
        ("/space/light/extinction", Cell::Number(SITE.extinction)),
        ("/space/light/sky", Cell::Number(SITE.sky)),
    ];
    assert_eq!(space::read(site).unwrap().light, SITE);
    let never = [life, ("/space/ages/2/shedding", Cell::Never)];
    let read = space::read(never).unwrap().species.unwrap();
    assert_eq!(read.states[2].shedding, f64::INFINITY);
}

/// fn-248: a leaf's years are a row like any other, `never` where they
/// have no end, and both read back as they were written.
#[test]
fn leaf_years_read_as_a_number_or_never() {
    let life = ("/space/ages/2/lifespan", Cell::Number(3.0));
    for (cell, want) in [(Cell::Number(1.5), 1.5), (Cell::Never, f64::INFINITY)] {
        let read = space::read([life, ("/space/ages/2/leafYears", cell)]).unwrap();
        assert_eq!(read.species.as_ref().unwrap().states[2].leaf_years, want);
        let rows = space::rows(&read).unwrap();
        let written = rows.iter().find(|(p, _)| p == "/space/ages/2/leafYears");
        // The neutral value is the default, so it writes no row.
        assert_eq!(written.map(|r| r.1), want.is_finite().then_some(cell));
    }
}

/// A family object written before fn-264 carried its light on the wire:
/// the oak's, the site's own, reads as the oak; the ordinary's old 1/6 is
/// refused, never taken for the site's.
#[test]
fn an_old_wire_light_reads_only_as_the_sites() {
    let oak = Preset::OregonWhiteOak.parameters();
    let mut wire = params::metadata(&oak);
    wire["space"]["light"] = serde_json::json!({"extinction": 0.5});
    let read = params::parse(&wire).unwrap();
    assert_eq!(format!("{read:?}"), format!("{oak:?}"));
    wire["space"]["light"] = serde_json::json!({"extinction": 1.0 / 6.0});
    assert!(params::parse(&wire).is_err());
}
