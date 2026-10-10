//! Step 5's girth terms on a small tree: the leaf term keeps the tree's
//! own pipe and thickens the lit limb, and a retained pipe thickens its
//! bearer by degree.
use super::*;
use crate::species::{NodeLaw, PaState, Zone};
use crate::structure::{Axis, Vec3};
use crate::transcendental::Transcendental;

fn state(leaf_girth: f64) -> PaState {
    PaState {
        lifespan: 4.0,
        continuation: 0.0,
        viability: 1.0,
        zones: vec![Zone {
            nodes: NodeLaw::Uniform { min: 1.0, max: 1.0 },
            buds: 1.0,
            lateral: vec![0.0],
            dormant: vec![0.0],
            delay: 0.0,
            rate: 0.0,
        }],
        shedding: f64::INFINITY,
        internode: 0.1,
        insertion: 0.5,
        divergence: 2.4,
        abortion: 0.0,
        abortion_rise: 0.0,
        relay: 0.0,
        relay_ended: 0.0,
        relay_failed: 0.0,
        relay_at: 1.0,
        epitony: 0.0,
        erection: 0.0,
        readiness: 1.0,
        rhythm: 1.0,
        leaf_area: 0.0,
        flowering: 0.0,
        cluster_leaves: 0.0,
        cluster_spread: 0.0,
        leaf_years: f64::INFINITY,
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
        grown: 1.0,
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
        phase: 0.0,
        joined: None,
    }
}

/// A trunk of two nodes bearing a lit limb and a shaded one.
fn tree() -> Structure {
    let lateral = |node| Origin::Lateral {
        parent: 0,
        node,
        slot: 0,
        whorl: 1.0,
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
            let leaf = mean.map_or(1.0, |m| p.light.powf_fixed(state.leaf_girth) * m);
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
    thicken(&mut a, &neutral, &Girth::default(), 1);
    thicken(&mut b, &leafy, &Girth::default(), 1);
    let (ta, tb) = (own_total(&a, &neutral), own_total(&b, &leafy));
    // The mean and each pipe are f32 (fn-254), so the tree's pipe holds
    // to f32's last places summed (1e-6 relative).
    assert!((ta - tb).abs() < 1e-6 * ta, "own pipe {ta} against {tb}");
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
        thicken(&mut t, &species, &girth, 1);
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

/// Host review: the leaf term's mean is one population, the tree as grown,
/// at every retained share, so the kept wood's girth meets the same value
/// as the retained share runs to 0.
#[test]
fn the_leaf_mean_is_the_grown_trees_at_every_retained_share() {
    use crate::shed::Shed;
    let mut species = Species {
        states: vec![state(1.0)],
    };
    let grown = tree();
    let shed = Shed {
        axes: vec![grown.axes[0].clone(), grown.axes[1].clone()],
        index: vec![0, 1, usize::MAX],
        fade: vec![1.0; 3],
    };
    let radius = |retained: f64, species: &mut Species| {
        species.states[0].retained = retained;
        let g = attachments(&grown.axes, species, grown.age, 1);
        let girth = disused(&g, &shed, species);
        let mut kept = Structure {
            age: grown.age,
            pas: 1,
            axes: shed.axes.clone(),
        };
        thicken(&mut kept, species, &girth, 1);
        kept.axes[1].phytomers[0].radius
    };
    let (at_zero, near_zero) = (radius(0.0, &mut species), radius(1e-9, &mut species));
    assert!(
        (at_zero - near_zero).abs() < 1e-9,
        "{at_zero} against {near_zero}"
    );
}

/// A relay's wood enters its parent where it joins (fn-244): every
/// sample below the span it stands in carries it whole, every one above
/// carries none, and the join's radii are the parent's there with the
/// relay whole (below) and with its stopped module's own wood (above).
#[test]
fn a_relays_wood_enters_where_it_joins_and_its_stub_carries_its_own() {
    let mut states = vec![state(0.0)];
    // A share and a blend that stand the relay part-way along a span.
    states[0].relay_at = 0.3;
    let species = Species { states };
    let grown = |origin: Option<Origin>| {
        let mut axes = vec![axis(Origin::Seed, &[0.5; 4])];
        if let Some(origin) = origin {
            let mut child = axis(origin, &[0.5; 3]);
            child.blend = 0.4;
            axes.push(child);
        }
        let mut t = Structure { age: 1, pas: 1, axes };
        thicken(&mut t, &species, &Girth::default(), 1);
        t
    };
    let relay = grown(Some(Origin::Relay { parent: 0, node: 1 }));
    let join = relay.axes[1].joined().expect("a relay's join");
    let k = join.node;
    assert!(join.t > 0.0 && join.t < 1.0, "part-way along its span");
    let lateral = grown(Some(Origin::Lateral {
        parent: 0,
        node: k,
        slot: 0,
        whorl: 1.0,
        woken: false,
    }));
    let alone = grown(None);
    let radii = |t: &Structure| t.axes[0].phytomers.iter().map(|p| p.radius).collect::<Vec<_>>();
    let (relay, lateral, alone) = (radii(&relay), radii(&lateral), radii(&alone));
    assert_eq!(relay[..k], lateral[..k], "the parent below the join");
    assert_eq!(relay[k + 1..], alone[k + 1..], "the stub");
    assert!(alone[k] < relay[k] && relay[k] < lateral[k], "the span by degree");
    assert_eq!(join.below, lateral[k], "the bole below the join");
    assert_eq!(join.above, alone[k], "the stub's first span");
}

/// A node growing in from nothing beneath a half-blended relay moves the
/// relay's attachment by nothing (fn-244 review): it is blended along the
/// parent's scaled length, not its node count.
#[test]
fn a_node_growing_in_moves_no_relays_attachment() {
    let mut states = vec![state(0.0)];
    states[0].relay_at = 0.0;
    let species = Species { states };
    // Where relay 1 joins its parent of nodes at heights `tips` and scales
    // `scales`, in space.
    let joined = |tips: &[f64], scales: &[f64]| {
        let mut parent = axis(Origin::Seed, &vec![0.5; tips.len()]);
        for ((p, &z), &s) in parent.phytomers.iter_mut().zip(tips).zip(scales) {
            (p.tip, p.scale, p.grown) = (Vec3::new(0.0, 0.0, z), s, s);
        }
        let mut relay = axis(Origin::Relay { parent: 0, node: 0 }, &[0.5]);
        relay.blend = 0.5;
        let axes = vec![parent, relay];
        let x = crate::geometry::attach(&axes, &species, 1).expect("a relay");
        let (k, t) = crate::geometry::split(x, tips.len() - 1);
        let start = if k == 0 { 0.0 } else { tips[k - 1] };
        start + (tips[k] - start) * f64::from(t)
    };
    let two = joined(&[1.0, 2.0], &[1.0, 1.0]);
    let grown_in = joined(&[1.0, 2.0, 2.0], &[1.0, 1.0, 0.0]);
    let barely = joined(&[1.0, 2.0, 2.0 + 1e-6], &[1.0, 1.0, 1e-6]);
    assert!((two - 1.0).abs() < 1e-9, "half way to the tip: {two}");
    assert!((grown_in - two).abs() < 1e-9, "{grown_in} against {two}");
    assert!((barely - two).abs() < 1e-5, "{barely} against {two}");
}
