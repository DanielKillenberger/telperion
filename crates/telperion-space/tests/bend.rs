//! fn-207 R2: wander bends over a length. At a long `bend_length` an axis's
//! successive turns are correlated as a curvature that relaxes over that
//! length (lag-one correlation exp(-internode / bend_length)), and the
//! spread of a turn per metre is the per-node draw's, so the same wander
//! makes slow arcs instead of jitter.
mod walk;
use telperion_space::{grow, Light, Request, Species, Structure, Vec3};

const INTERNODE: f64 = 0.1;
const WANDER: f64 = 0.1;

/// The walk tree's trunk alone, long, unbranched and free of tropism.
fn stem(bend_length: f64) -> Species {
    let mut s = walk::species();
    let trunk = &mut s.states[0];
    trunk.lifespan = 1_000.0;
    trunk.readiness = 0.0;
    trunk.internode = INTERNODE;
    trunk.form.tropism = 0.0;
    trunk.form.wander = WANDER;
    trunk.form.bend_length = bend_length;
    s
}

fn grown(species: &Species, seed: u64) -> Structure {
    let request = Request {
        age: 40,
        seed,
        budget: 1_000_000,
        light: Light::NEUTRAL,
    };
    grow(species, request).unwrap()
}

/// Each node's turn, as a rotation vector, along the trunk.
fn turns(tree: &Structure) -> Vec<Vec3> {
    let p = &tree.axes[0].phytomers;
    p.windows(2)
        .map(|w| w[0].heading.cross(w[1].heading))
        .collect()
}

/// The turns' lag-one correlation and their mean square per square metre.
fn stats(bend_length: f64) -> (f64, f64) {
    let (mut lag, mut square, mut count) = (0.0, 0.0, 0.0);
    let species = stem(bend_length);
    for seed in 1..=60 {
        let t = turns(&grown(&species, seed));
        for k in 0..t.len() - 1 {
            lag += t[k].dot(t[k + 1]);
            square += t[k].dot(t[k]);
            count += 1.0;
        }
    }
    (lag / square, square / count / (INTERNODE * INTERNODE))
}

#[test]
fn a_long_bend_length_correlates_turns_and_keeps_their_spread() {
    let (memoryless, spread) = stats(0.0);
    let (correlated, kept) = stats(2.0);
    let expected = (-INTERNODE / 2.0f64).exp();
    assert!(
        memoryless.abs() < 0.05,
        "per-node draw correlated {memoryless}"
    );
    assert!(
        (correlated - expected).abs() < 0.05,
        "correlation {correlated} against {expected}"
    );
    // A draw's spread per metre: wander^2 E[(2u - 1)^2] = wander^2 / 3.
    let draw = WANDER * WANDER / 3.0;
    for (name, value) in [("per node", spread), ("bent", kept)] {
        assert!(
            (value / draw - 1.0).abs() < 0.1,
            "{name}: {value} against {draw}"
        );
    }
}
