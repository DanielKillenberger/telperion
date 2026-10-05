//! Step 5's girth terms on a small tree: the leaf term keeps the tree's
//! own pipe and thickens the lit limb, and a retained pipe thickens its
//! bearer by degree.
use super::*;
use crate::species::{NodeLaw, PaState, Zone};
use crate::structure::{Axis, Vec3};

fn state(leaf_girth: f64) -> PaState {
    PaState {
        lifespan: 4,
        next: None,
        viability: 1.0,
        zones: vec![Zone {
            nodes: NodeLaw::Uniform { min: 1, max: 1 },
            buds: 1,
            lateral: vec![0.0],
            dormant: vec![0.0],
            delay: 0.0,
            rate: 0.0,
        }],
        shedding: None,
        internode: 0.1,
        insertion: 0.5,
        divergence: 2.4,
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
        retained: 0.0,
        leaf_girth,
        straightening: 0.0,
        form: Form::default(),
    }
}

fn phytomer(light: f64) -> Phytomer {
    Phytomer {
        cycle: 1,
        tip: Vec3::default(),
        heading: Vec3::default(),
        side: Vec3::default(),
        radius: 0.0,
        scale: 1.0,
        key: 0,
        rank: 0.0,
        size: 1.0,
        light,
    }
}

fn axis(origin: Origin, lights: &[f64]) -> Axis {
    Axis {
        lineage: 0,
        pa: 0,
        birth: 0,
        origin,
        vigour: 1.0,
        blend: 1.0,
        apex_end: None,
        base: Vec3::default(),
        heading: Vec3::default(),
        side: Vec3::default(),
        phytomers: lights.iter().map(|&l| phytomer(l)).collect(),
        units: Vec::new(),
        alive: 1.0,
        rank: 0.0,
        sleep: 0.0,
    }
}

/// A trunk of two nodes bearing a lit limb and a shaded one.
fn tree() -> Structure {
    let lateral = |node| Origin::Lateral {
        parent: 0,
        node,
        slot: 0,
        whorl: 1,
        woken: false,
    };
    Structure {
        age: 1,
        pas: 1,
        axes: vec![
            axis(Origin::Seed, &[0.6, 0.6]),
            axis(lateral(0), &[0.9, 0.9, 0.9]),
            axis(lateral(1), &[0.2, 0.2, 0.2]),
        ],
    }
}

fn own_total(t: &Structure, species: &Species) -> f64 {
    let mean = leaf_terms(t, species);
    let state = &species.states[0];
    t.axes
        .iter()
        .flat_map(|a| &a.phytomers)
        .map(|p| {
            let leaf = mean.map_or(1.0, |m| p.light.powf(state.leaf_girth) * m);
            own(&state.form, p, t.age, p.scale, leaf)
        })
        .sum()
}

#[test]
fn the_leaf_term_keeps_the_trees_pipe_and_thickens_the_lit_limb() {
    let neutral = Species {
        states: vec![state(0.0)],
    };
    let leafy = Species {
        states: vec![state(1.0)],
    };
    let (mut a, mut b) = (tree(), tree());
    thicken(&mut a, &neutral, &Girth::default());
    thicken(&mut b, &leafy, &Girth::default());
    let (ta, tb) = (own_total(&a, &neutral), own_total(&b, &leafy));
    assert!((ta - tb).abs() < 1e-12 * ta, "own pipe {ta} against {tb}");
    assert_eq!(a.axes[1].phytomers[0].radius, a.axes[2].phytomers[0].radius);
    assert!(b.axes[1].phytomers[0].radius > a.axes[1].phytomers[0].radius);
    assert!(b.axes[2].phytomers[0].radius < a.axes[2].phytomers[0].radius);
}

#[test]
fn a_retained_pipe_thickens_its_bearer_by_degree() {
    let species = Species {
        states: vec![state(0.0)],
    };
    let base = |share: f64| {
        let mut t = tree();
        let disused = [Disused {
            axis: 0,
            node: Some(0),
            radius: 0.02,
            share,
        }];
        let girth = Girth {
            disused: disused.to_vec(),
            leaf_mean: None,
        };
        thicken(&mut t, &species, &girth);
        t.axes[0].phytomers[0].radius
    };
    let mut last = base(0.0);
    for step in 1..=100 {
        let now = base(f64::from(step) / 100.0);
        assert!(
            now > last && now - last < 1e-3,
            "step {step}: {last} to {now}"
        );
        last = now;
    }
}
