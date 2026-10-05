//! fn-203: sag measures its lever on the bent branch. The load beyond a
//! phytomer turns with the bends before it, so a heavy branch hangs and
//! stops, and the ground carries what rests on it.
use telperion_space::{grow, Form, NodeLaw, PaState, Request, Species, Structure, Vec3, Zone};

fn zone(lateral: Vec<f64>, buds: u8) -> Zone {
    Zone {
        nodes: NodeLaw::Uniform { min: 1, max: 1 },
        buds,
        dormant: vec![0.0; lateral.len()],
        delay: 0.0,
        rate: 0.0,
        lateral,
    }
}

/// A branch's own form: its tip's turn towards its elevation, and how it
/// wanders.
#[derive(Clone, Copy)]
struct Bough {
    tropism: f64,
    elevation: f64,
    wander: f64,
}

/// A pole `height` m tall bearing one long branch that leaves it near
/// level, with twigs on both sides, under `sag`.
fn tree(height: f64, sag: f64, seed: u64, bough: Bough) -> Structure {
    let pole = PaState {
        lifespan: 1,
        next: None,
        viability: 1.0,
        zones: vec![zone(vec![0.0, 1.0, 0.0], 1)],
        shedding: None,
        internode: height,
        insertion: 0.0,
        divergence: 0.0,
        abortion: 0.0,
        abortion_rise: 0.0,
        relay: 0.0,
        relay_at: 1.0,
        epitony: 0.0,
        erection: 0.0,
        readiness: 1.0,
        rhythm: 1.0,
        leaf_area: 0.0,
        shade_hazard: 0.0,
        shade_size: 0.0,
        apical_control: 0.5,
        upkeep: 0.0,
        balance_hazard: 0.0,
        tolerance: 0.0,
        straightening: 0.0,
        form: Form {
            pipe: 0.05,
            ..Form::default()
        },
    };
    let branch = PaState {
        lifespan: 1_000,
        zones: vec![zone(vec![0.0, 0.0, 0.8], 2)],
        internode: 0.05,
        insertion: 1.5,
        divergence: std::f64::consts::PI,
        form: Form {
            tropism: bough.tropism,
            elevation: bough.elevation,
            wander: bough.wander,
            pipe: 0.0008,
            exponent: 2.2,
            ripening: 10.0,
            roll: 0.8,
            sag,
            ..Form::default()
        },
        ..pole.clone()
    };
    let twig = PaState {
        lifespan: 6,
        zones: vec![zone(vec![0.0, 0.0, 0.0], 1)],
        internode: 0.05,
        insertion: 0.9,
        form: Form {
            pipe: 0.0005,
            tropism: 2.0,
            elevation: -1.0,
            ..Form::default()
        },
        ..pole.clone()
    };
    let species = Species {
        states: vec![pole, branch, twig],
    };
    let request = Request {
        age: 70,
        seed,
        budget: 1_000_000,
        light: telperion_space::Light::NEUTRAL,
    };
    grow(&species, request).unwrap()
}

/// The branch's internode directions, base first.
fn directions(tree: &Structure) -> Vec<Vec3> {
    let branch = &tree.axes[1];
    let mut from = branch.base;
    let mut out = Vec::new();
    for p in &branch.phytomers {
        if let Some(d) = (p.tip - from).unit() {
            out.push(d);
        }
        from = p.tip;
    }
    out
}

/// R1 (i): in free air, a long, heavily loaded branch converges towards
/// hanging: where it hangs within 0.3 rad of straight down, each
/// phytomer turns it by almost nothing more.
#[test]
fn a_heavy_branch_converges_towards_hanging() {
    let still = Bough {
        tropism: 0.0,
        elevation: 0.0,
        wander: 0.0,
    };
    for seed in 1..=3 {
        let d = directions(&tree(10.0, 2e-3, seed, still));
        let hanging: Vec<f64> = d
            .windows(2)
            .filter(|w| -w[0].z > 0.3f64.cos())
            .map(|w| w[0].dot(w[1]).clamp(-1.0, 1.0).acos())
            .collect();
        let fall = d.iter().map(|v| -v.z).fold(f64::MIN, f64::max);
        let last = hanging.last().copied().unwrap_or(f64::NAN);
        println!(
            "seed {seed}: steepest {fall:.4}, {} hanging, last turn {last:.5}",
            hanging.len()
        );
        assert!(hanging.len() > 10, "seed {seed}: the branch hangs");
        assert!(last < 1e-3, "seed {seed}: still turning by {last} rad");
    }
}

/// The azimuth a branch's level-going internodes wind through: a branch
/// that swings round its base or coils winds by a large angle.
fn winding(d: &[Vec3]) -> f64 {
    let level: Vec<f64> = d
        .iter()
        .filter(|v| v.x.hypot(v.y) > 0.3)
        .map(|v| v.y.atan2(v.x))
        .collect();
    let tau = std::f64::consts::TAU;
    level
        .windows(2)
        .map(|w| (w[1] - w[0] + 3.0 * std::f64::consts::PI).rem_euclid(tau) - std::f64::consts::PI)
        .sum::<f64>()
        .abs()
}

/// R1 (ii), round 9's fault: a heavy, low bough, upturned and wandering
/// as the spruce's are, that comes down to the ground does not swing
/// round its base. On the small-deflection sag it reached the ground and
/// wound 1.5 to 1.6 rad at every seed.
#[test]
fn a_heavy_low_bough_on_the_ground_does_not_swing_round() {
    let spruce = Bough {
        tropism: 0.8,
        elevation: 0.7,
        wander: 1.0,
    };
    for seed in 1..=4 {
        let tree = tree(0.4, 6e-4, seed, spruce);
        let low = tree.axes[1]
            .phytomers
            .iter()
            .map(|p| p.tip.z)
            .fold(f64::MAX, f64::min);
        let wound = winding(&directions(&tree));
        println!("seed {seed}: winds {wound:.2} rad, comes within {low:.3} m of the ground");
        assert!(
            low < 0.1,
            "seed {seed}: the bough comes down to the ground ({low})"
        );
        assert!(wound < 1.2, "seed {seed}: the bough winds {wound} rad");
    }
}

/// R2: a lightly loaded branch bends as the small-deflection beam says:
/// within one percent of fn-200's bends, and in proportion to its load,
/// doubling a light sag doubling every bend to within a few percent (the
/// correction is first order in the bend).
#[test]
fn a_light_branch_bends_as_the_small_deflection_beam_does() {
    let still = Bough {
        tropism: 0.0,
        elevation: 0.0,
        wander: 0.0,
    };
    let plain = directions(&tree(10.0, 0.0, 1, still));
    let bends = |sag: f64| -> Vec<f64> {
        let bent = directions(&tree(10.0, sag, 1, still));
        plain
            .iter()
            .zip(&bent)
            .map(|(u, v)| u.dot(*v).clamp(-1.0, 1.0).acos())
            .collect()
    };
    let (one, two) = (bends(1e-6), bends(2e-6));
    // Every tenth internode's bend under fn-200's small-deflection sag
    // (commit 0706c79c), the reference: the corrected bend stays within
    // one percent of it.
    let reference = [
        0.000924, 0.009364, 0.016523, 0.022312, 0.027202, 0.030718, 0.033358,
    ];
    for (k, (&ours, small)) in one.iter().step_by(10).zip(reference).enumerate() {
        assert!(
            (ours - small).abs() <= 0.01 * small + 1e-6,
            "internode {}: {ours} rad against the small-deflection {small}",
            10 * k
        );
    }
    let most = one.iter().fold(0.0f64, |m, &b| m.max(b));
    let off = one
        .iter()
        .zip(&two)
        .map(|(a, b)| (b - 2.0 * a).abs())
        .fold(0.0f64, f64::max);
    println!("a light load bends up to {most:.4} rad; doubled, off by {off:.5}");
    assert!(most > 0.01, "the load bends the branch ({most})");
    assert!(
        off < 0.05 * 2.0 * most,
        "doubling bends {off} rad off double"
    );
}
