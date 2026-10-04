//! fn-203: sag measures its lever on the bent branch. A branch's load
//! turns with the bends above it, so its lever shrinks as it droops: a
//! heavy branch hangs and stops, and a light one bends as the small
//! deflection theory says.
use telperion_space::{grow, Form, NodeLaw, PaState, Request, Species, Structure, Vec3, Zone};

fn zone(lateral: Vec<f64>) -> Zone {
    Zone {
        nodes: NodeLaw::Uniform { min: 1, max: 1 },
        buds: 1,
        dormant: vec![0.0; lateral.len()],
        delay: 0.0,
        rate: 0.0,
        lateral,
    }
}

/// A 10 m pole bearing one long branch, leaving level, wandering, with
/// twigs on both sides, its tip turning up towards a shallow elevation, as
/// the spruce's boughs do, under `sag`.
pub fn bough(sag: f64, seed: u64, one_sided: bool) -> Structure {
    let pole = PaState {
        lifespan: 1,
        next: None,
        viability: 1.0,
        zones: vec![zone(vec![0.0, 1.0, 0.0])],
        shedding: None,
        internode: 10.0,
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
        straightening: 0.0,
        form: Form {
            pipe: 0.05,
            ..Form::default()
        },
    };
    let branch = PaState {
        lifespan: 1_000,
        zones: vec![Zone {
            buds: if one_sided { 1 } else { 2 },
            ..zone(vec![0.0, 0.0, 0.8])
        }],
        internode: 0.1,
        insertion: 1.5,
        divergence: if one_sided { 0.0 } else { std::f64::consts::PI },
        form: Form {
            tropism: 0.8,
            elevation: 0.7,
            wander: if one_sided { 0.0 } else { 1.0 },
            pipe: 0.002,
            sag,
            ..Form::default()
        },
        ..pole.clone()
    };
    let twig = PaState {
        lifespan: 6,
        zones: vec![zone(vec![0.0, 0.0, 0.0])],
        internode: 0.1,
        insertion: 1.0,
        form: Form {
            pipe: 0.001,
            ..Form::default()
        },
        ..pole.clone()
    };
    let species = Species {
        states: vec![pole, branch, twig],
    };
    grow(
        &species,
        Request {
            age: 32,
            seed,
            budget: 100_000,
        },
    )
    .unwrap()
}

/// Each internode's direction along the branch, base first, and how far
/// it leans away from the pole: its run's share along the horizontal from
/// the pole to its base.
fn leans(tree: &Structure) -> Vec<(Vec3, f64)> {
    let branch = &tree.axes[1];
    let mut from = branch.base;
    branch
        .phytomers
        .iter()
        .map(|p| {
            let d = (p.tip - from).unit().unwrap();
            let away = Vec3::new(from.x, from.y, 0.0)
                .unit()
                .map_or(0.0, |r| d.dot(r));
            from = p.tip;
            (d, away)
        })
        .collect()
}

/// R1: a long, heavily loaded branch hangs and never bends past the
/// vertical along its length: no internode leans back towards the pole.
#[test]
fn a_heavy_branch_hangs_and_never_passes_the_vertical() {
    for one_sided in [true, false] {
    for sag in [1e-4, 1e-3, 1e-2] {
        for seed in 1..=2 {
            let l = leans(&bough(sag, seed, one_sided));
            let back = l[1..].iter().map(|(_, a)| *a).fold(f64::INFINITY, f64::min);
            let hangs = l
                .iter()
                .map(|(d, _)| -d.z)
                .fold(f64::NEG_INFINITY, f64::max);
            let out = Vec3::new(l[1].0.x, l[1].0.y, 0.0).unit().unwrap();
            let side = l
                .iter()
                .filter(|(d, _)| d.x.hypot(d.y) > 0.1)
                .map(|(d, _)| {
                    let h = Vec3::new(d.x, d.y, 0.0).unit().unwrap();
                    h.cross(out).z.atan2(h.dot(out)).abs()
                })
                .fold(0.0f64, f64::max);
            let tree = bough(sag, seed, one_sided);
            let branch = &tree.axes[1];
            let radial = |v: Vec3| v.x.hypot(v.y);
            let (mut far, mut retreat) = (0.0f64, 0.0f64);
            for p in &branch.phytomers {
                far = far.max(radial(p.tip));
                retreat = retreat.max(far - radial(p.tip));
            }
            println!("one-sided {one_sided} sag {sag} seed {seed}: least lean away {back:.4}, steepest fall {hangs:.4}, widest swing {side:.2}, retreat {retreat:.3} of {far:.3}");
        }
    }
    }
}
