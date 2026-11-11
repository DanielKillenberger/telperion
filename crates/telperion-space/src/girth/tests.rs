//! Girth on a small tree: a relay's wood where it joins.
use super::*;
use crate::species::{NodeLaw, PaState, Zone};
use crate::structure::{Axis, Phytomer, Vec3};

fn state() -> PaState {
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
        form: Form::default(),
    }
}

fn phytomer() -> Phytomer {
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
        grown: 1.0,
    }
}

fn axis(origin: Origin, nodes: usize) -> Axis {
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
        phytomers: (0..nodes).map(|_| phytomer()).collect(),
        units: Vec::new(),
        alive: 1.0,
        rank: 0.0,
        sleep: 0.0,
        phase: 0.0,
        joined: None,
    }
}

/// A relay's wood enters its parent where it joins (fn-244): every
/// sample below the span it stands in carries it whole, every one above
/// carries none, and the join's radii are the parent's there with the
/// relay whole (below) and with its stopped module's own wood (above).
#[test]
fn a_relays_wood_enters_where_it_joins_and_its_stub_carries_its_own() {
    let mut states = vec![state()];
    // A share and a blend that stand the relay part-way along a span.
    states[0].relay_at = 0.3;
    let species = Species { states };
    let grown = |origin: Option<Origin>| {
        let mut axes = vec![axis(Origin::Seed, 4)];
        if let Some(origin) = origin {
            let mut child = axis(origin, 3);
            child.blend = 0.4;
            axes.push(child);
        }
        let mut t = Structure {
            age: 1,
            pas: 1,
            axes,
        };
        thicken(&mut t, &species, 1);
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
    let radii = |t: &Structure| {
        t.axes[0]
            .phytomers
            .iter()
            .map(|p| p.radius)
            .collect::<Vec<_>>()
    };
    let (relay, lateral, alone) = (radii(&relay), radii(&lateral), radii(&alone));
    assert_eq!(relay[..k], lateral[..k], "the parent below the join");
    assert_eq!(relay[k + 1..], alone[k + 1..], "the stub");
    assert!(
        alone[k] < relay[k] && relay[k] < lateral[k],
        "the span by degree"
    );
    assert_eq!(join.below, lateral[k], "the bole below the join");
    assert_eq!(join.above, alone[k], "the stub's first span");
}

/// A node growing in from nothing beneath a half-blended relay moves the
/// relay's attachment by nothing (fn-244 review): it is blended along the
/// parent's scaled length, not its node count.
#[test]
fn a_node_growing_in_moves_no_relays_attachment() {
    let mut states = vec![state()];
    states[0].relay_at = 0.0;
    let species = Species { states };
    // Where relay 1 joins its parent of nodes at heights `tips` and scales
    // `scales`, in space.
    let joined = |tips: &[f64], scales: &[f64]| {
        let mut parent = axis(Origin::Seed, tips.len());
        for ((p, &z), &s) in parent.phytomers.iter_mut().zip(tips).zip(scales) {
            (p.tip, p.scale, p.grown) = (Vec3::new(0.0, 0.0, z), s, s);
        }
        let mut relay = axis(Origin::Relay { parent: 0, node: 0 }, 1);
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
