//! The crown's outline: the envelope's two irregularity rows, the shell they
//! shape, and what every stage that consults the shell sees through them.
//! Neutral first - a tree whose table leaves the amplitude at zero is the
//! smooth superellipse it always was, to the byte - then the rails, the bound
//! on the perturbed radius, containment on every fixed seed, one outline per
//! seed, and the amplitude's walk between two families.
use telperion_core::{
    blend, branching, envelope::Envelope, math::Vec3, presets::Preset, radius::RadiusParams, Error,
};

/// Every shipped table, so neutrality is asserted on all of them at once.
const PRESETS: [Preset; 7] = [
    Preset::Ordinary,
    Preset::OregonWhiteOak,
    Preset::NorwaySpruce,
    Preset::EuropeanBeech,
    Preset::SilverBirch,
    Preset::Telperion,
    Preset::Laurelin,
];

/// Heights and bearings dense enough to cross several lobes of either row.
fn grid() -> impl Iterator<Item = (f64, f64)> {
    (0..=64).flat_map(|i| {
        (0..48).map(move |k| (i as f64 / 64.0, k as f64 * std::f64::consts::TAU / 48.0))
    })
}

#[test]
fn a_neutral_amplitude_is_the_smooth_radius_to_the_byte() {
    for preset in PRESETS {
        let e = Envelope {
            irregularity: 0.0,
            ..preset.parameters().skeleton.envelope
        };
        for seed in [0, 1, 7, 4_294_967_295] {
            for (t, azimuth) in grid() {
                let y = e.height * t;
                assert_eq!(
                    e.radius_at_bearing(y, azimuth, seed).to_bits(),
                    e.radius_at(y).to_bits(),
                    "{preset:?}: seed {seed} moved the neutral outline at {y}"
                );
            }
        }
    }
}

#[test]
fn the_outline_rows_are_refused_by_their_own_names() {
    let e = Envelope::default();
    for bad in [-0.001, 0.501, f64::NAN, f64::INFINITY] {
        assert!(
            matches!(
                Envelope {
                    irregularity: bad,
                    ..e
                }
                .validate(),
                Err(Error::InvalidInput("envelope irregularity"))
            ),
            "irregularity {bad} was not refused by name"
        );
    }
    for bad in [0.049, 1.001, f64::NAN, f64::NEG_INFINITY] {
        assert!(
            matches!(
                Envelope {
                    lobe_scale: bad,
                    ..e
                }
                .validate(),
                Err(Error::InvalidInput("envelope lobe scale"))
            ),
            "lobe scale {bad} was not refused by name"
        );
    }
    assert!(Envelope {
        irregularity: 0.5,
        lobe_scale: 0.05,
        ..e
    }
    .validate()
    .is_ok());
}

#[test]
fn the_perturbed_radius_stays_within_the_amplitude() {
    for (irregularity, lobe_scale) in [(0.05, 0.05), (0.25, 0.4), (0.5, 1.0)] {
        let e = Envelope {
            irregularity,
            lobe_scale,
            ..Envelope::default()
        };
        let mut moved = false;
        for seed in [0, 3, 7, 91] {
            for (t, azimuth) in grid() {
                let y = e.height * t;
                let smooth = e.radius_at(y);
                let lobed = e.radius_at_bearing(y, azimuth, seed);
                assert!(
                    lobed >= 0.0
                        && lobed <= smooth * (1.0 + irregularity) + 1e-12
                        && lobed >= smooth * (1.0 - irregularity) - 1e-12,
                    "seed {seed}: {lobed} is outside {smooth} by more than {irregularity}"
                );
                moved |= (lobed - smooth).abs() > 1e-9;
            }
        }
        assert!(moved, "the amplitude {irregularity} changed nothing");
    }
}

#[test]
fn one_seed_is_one_outline_and_two_seeds_are_two() {
    let e = Envelope {
        irregularity: 0.3,
        lobe_scale: 0.6,
        ..Envelope::default()
    };
    let read = |seed| {
        grid()
            .map(|(t, azimuth)| e.radius_at_bearing(e.height * t, azimuth, seed))
            .collect::<Vec<_>>()
    };
    assert_eq!(read(7), read(7), "the same seed gave two outlines");
    assert_ne!(read(7), read(8), "two seeds gave one outline");
    // The lobes close on themselves: a full turn is the same bearing.
    for (t, azimuth) in grid() {
        let y = e.height * t;
        let turned = azimuth + std::f64::consts::TAU;
        assert!(
            (e.radius_at_bearing(y, azimuth, 7) - e.radius_at_bearing(y, turned, 7)).abs() < 1e-9,
            "the outline does not close at {azimuth}"
        );
    }
}

#[test]
fn every_crown_node_lies_inside_the_shell_its_seed_shapes() {
    let mut params = Preset::Ordinary.parameters().skeleton;
    params.envelope.irregularity = 0.3;
    params.envelope.lobe_scale = 0.5;
    params.growth.max_nodes = Some(6_000);
    let mut silhouettes = Vec::new();
    for seed in [1, 7, 4242] {
        params.seed = seed;
        let grown = branching::generate(&params, RadiusParams::default()).unwrap();
        assert!(grown.tree.nodes.len() > grown.tree.crossover);
        for node in grown.tree.nodes.iter().skip(grown.tree.crossover) {
            assert!(
                params.envelope.contains(node.position, 1e-8, seed),
                "seed {seed}: {:?} is outside the shell that seed shapes",
                node.position
            );
        }
        silhouettes.push(
            grown
                .tree
                .nodes
                .iter()
                .map(|n| ((n.position.x.powi(2) + n.position.z.powi(2)).sqrt() * 1e6) as i64)
                .max(),
        );
    }
    assert!(
        silhouettes
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            > 1,
        "three seeds filled one silhouette: {silhouettes:?}"
    );
}

#[test]
fn a_shell_with_an_amplitude_is_not_the_smooth_shell() {
    let smooth = Envelope::default();
    let lobed = Envelope {
        irregularity: 0.4,
        lobe_scale: 0.3,
        ..smooth
    };
    let outside = grid()
        .map(|(t, azimuth)| {
            let y = smooth.height * t;
            let r = smooth.radius_at(y);
            let p = Vec3::new(r * azimuth.cos(), y, r * azimuth.sin());
            usize::from(smooth.contains(p, 0.0, 7) && !lobed.contains(p, 0.0, 7))
        })
        .sum::<usize>();
    assert!(
        outside > 0,
        "the amplitude rejected nothing the smooth shell held"
    );
}

#[test]
fn a_blend_walks_the_amplitude_and_the_wavelength() {
    let mut a = Preset::EuropeanBeech.parameters();
    let mut b = Preset::SilverBirch.parameters();
    a.skeleton.envelope.irregularity = 0.1;
    a.skeleton.envelope.lobe_scale = 0.2;
    b.skeleton.envelope.irregularity = 0.5;
    b.skeleton.envelope.lobe_scale = 1.0;
    for step in 0..=10 {
        let t = step as f64 / 10.0;
        let e = blend::families(&a, &b, t).unwrap().skeleton.envelope;
        assert!(
            (e.irregularity - (0.1 + 0.4 * t)).abs() < 1e-12,
            "{t}: {e:?}"
        );
        assert!((e.lobe_scale - (0.2 + 0.8 * t)).abs() < 1e-12, "{t}: {e:?}");
        e.validate().unwrap();
    }
}
