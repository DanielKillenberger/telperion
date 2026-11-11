//! fn-203: sag measures its lever on the bent branch. The load beyond a
//! phytomer turns with the bends before it, so a heavy branch hangs and
//! stops, and the ground carries what rests on it.
use telperion_space::{
    grow, Form, NodeLaw, Origin, PaState, Request, Species, Structure, Vec3, Zone,
};

fn zone(lateral: Vec<f64>, buds: u8) -> Zone {
    Zone {
        nodes: NodeLaw::Uniform { min: 1.0, max: 1.0 },
        buds: f64::from(buds),
        dormant: vec![0.0; lateral.len()],
        delay: 0.0,
        rate: 0.0,
        lateral,
    }
}

/// The primary radius of the pole, the branch and its twigs, m: wood that
/// lays no sapwood, so every phytomer keeps it.
const PRIMARY: [f64; 3] = [0.05, 0.0008, 0.0005];

/// The small-deflection beam's bend of each of the branch's internodes on
/// the unbent tree `plain` under `sag`: each phytomer turns by `sag` times
/// the level moment about its end of the load beyond it, wood and foliage
/// as `sag.rs` weighs them, over its radius to the fourth, times its
/// length; an internode's direction moves by its own turn and every turn
/// before it.
fn beam(plain: &Structure, sag: f64) -> Vec<f64> {
    let n = plain.axes.len();
    // Each axis's phytomers' masses and centres, base to tip.
    let parts: Vec<Vec<(f64, Vec3)>> = plain
        .axes
        .iter()
        .map(|a| {
            let mut from = a.base;
            let own = PRIMARY[a.pa];
            a.phytomers
                .iter()
                .map(|p| {
                    let leaf = own * p.scale;
                    let part = (
                        (p.tip - from).length() * (p.radius * p.radius + leaf * leaf),
                        (from + p.tip) * 0.5,
                    );
                    from = p.tip;
                    part
                })
                .collect()
        })
        .collect();
    // Each axis's whole load, borne by its parent at a node.
    let mut whole = vec![(0.0, Vec3::default()); n];
    for i in (0..n).rev() {
        let (mut m, mut c) = parts[i]
            .iter()
            .fold((0.0, Vec3::default()), |(m, c), &(w, x)| (m + w, c + x * w));
        for (j, a) in plain.axes.iter().enumerate().skip(i + 1) {
            if a.origin.parent() == Some(i) {
                m += whole[j].0;
                c = c + whole[j].1;
            }
        }
        whole[i] = (m, c);
    }
    let branch = &plain.axes[1];
    let mut from = branch.base;
    let (mut turned, mut out) = (0.0, Vec::new());
    for (k, p) in branch.phytomers.iter().enumerate() {
        // The load beyond the phytomer's end: the branch's later wood and
        // every axis it bears at this node or beyond.
        let (mut m, mut c) = (0.0, Vec3::default());
        for &(w, x) in &parts[1][k + 1..] {
            (m, c) = (m + w, c + x * w);
        }
        for (j, a) in plain.axes.iter().enumerate() {
            let at = match a.origin {
                Origin::Lateral {
                    parent: 1, node, ..
                } => node,
                Origin::Continuation { parent: 1 } | Origin::Relay { parent: 1, .. } => usize::MAX,
                _ => continue,
            };
            if at >= k {
                (m, c) = (m + whole[j].0, c + whole[j].1);
            }
        }
        let lever = c - p.tip * m;
        let level = lever.x.hypot(lever.y);
        let turn = sag * level / p.radius.powi(4) * (p.tip - from).length();
        turned += turn;
        out.push(turned);
        from = p.tip;
    }
    out
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
    grown(height, sag, seed, bough, (0.8, 0.8))
}

/// The same tree with each bud place bearing a twig by chance `twigs.0`,
/// each twig rolled by up to `twigs.1`: certain and unrolled, it is the
/// same tree under every key.
fn grown(height: f64, sag: f64, seed: u64, bough: Bough, twigs: (f64, f64)) -> Structure {
    let pole = PaState {
        lifespan: 1.0,
        continuation: 0.0,
        viability: 1.0,
        zones: vec![zone(vec![0.0, 1.0, 0.0], 1)],
        shedding: f64::INFINITY,
        internode: height,
        insertion: 0.0,
        divergence: 0.0,
        abortion: 0.0,
        abortion_rise: 0.0,
        relay: 0.0,
        relay_ended: 0.0,
        relay_failed: 0.0,
        relay_at: 1.0,
        epitony: 0.0,
        takeover: 0.0,
        erection: 0.0,
        readiness: 1.0,
        rhythm: 1.0,
        leaf_area: 0.0,
        flowering: 0.0,
        cluster_leaves: 0.0,
        cluster_spread: 0.0,
        leaf_years: f64::INFINITY,
        elongation: 0.0,
        expansion: 0.0,
        leaf_life: f64::INFINITY,
        budburst: 0.0,
        senescence: 0.0,
        colouring: 0.0,
        leaf_fall: 0.0,
        shade_hazard: 0.0,
        ground_hazard: 0.0,
        shade_size: 0.0,
        apical_control: 0.5,
        inverse_height: 0.0,
        balance_hazard: 0.0,
        tolerance: 0.0,
        straightening: 0.0,
        form: Form {
            primary: PRIMARY[0],
            ..Form::default()
        },
    };
    let branch = PaState {
        lifespan: 100.0,
        zones: vec![zone(vec![0.0, 0.0, twigs.0], 2)],
        internode: 0.05,
        insertion: 1.5,
        divergence: std::f64::consts::PI,
        form: Form {
            tropism: bough.tropism,
            elevation: bough.elevation,
            wander: bough.wander,
            primary: PRIMARY[1],
            roll: twigs.1,
            sag,
            ..Form::default()
        },
        ..pole.clone()
    };
    let twig = PaState {
        lifespan: 6.0,
        zones: vec![zone(vec![0.0, 0.0, 0.0], 1)],
        internode: 0.05,
        insertion: 0.9,
        form: Form {
            primary: PRIMARY[2],
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
/// within one percent of the beam's bends on its unbent tree (`beam`; fn-354
/// replaced fn-200's pinned bends, taken on a pipe-model girth that is
/// gone), and in proportion to its load,
/// doubling a light sag doubling every bend to within a few percent (the
/// correction is first order in the bend).
#[test]
fn a_light_branch_bends_as_the_small_deflection_beam_does() {
    // A sag light enough that the bend stays small on wood that keeps its
    // primary radius.
    const SAG: f64 = 1e-9;
    let still = Bough {
        tropism: 0.0,
        elevation: 0.0,
        wander: 0.0,
    };
    // Every place bears a twig, unrolled: a tree no key moves.
    let certain = |sag: f64| grown(10.0, sag, 1, still, (1.0, 0.0));
    let plain = directions(&certain(0.0));
    let bends = |sag: f64| -> Vec<f64> {
        let bent = directions(&certain(sag));
        plain
            .iter()
            .zip(&bent)
            // The angle by its sine and cosine: `acos` near one cannot
            // resolve a bend under a milliradian between f32 directions.
            .map(|(u, v)| u.cross(*v).length().atan2(u.dot(*v)))
            .collect()
    };
    let (one, two) = (bends(SAG), bends(2.0 * SAG));
    // The small-deflection beam on the unbent tree (`beam`): the corrected
    // bend stays within one percent of it, and within the f32 directions'
    // own noise of a bend, 1.5e-5 rad measured on the first internode's
    // (fn-254).
    let small = beam(&certain(0.0), SAG);
    for (k, (&ours, &small)) in one.iter().zip(&small).enumerate().step_by(10) {
        assert!(
            (ours - small).abs() <= 0.01 * small + 2e-5,
            "internode {k}: {ours} rad against the small-deflection {small}"
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
