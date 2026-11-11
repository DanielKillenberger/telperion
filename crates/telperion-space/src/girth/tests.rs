//! The wood on a small tree (PRODUCT-DESIGN.md, "Tests"): what a year
//! lays, what it never takes away, and where a relay joins.
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
        leaf_years: 1.0,
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
        section: 0.0,
        sapwood: 0.0,
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

/// A lateral at `node` of axis 0.
fn lateral(node: usize) -> Origin {
    Origin::Lateral {
        parent: 0,
        node,
        slot: 0,
        whorl: 1.0,
        woken: false,
    }
}

/// The wood's species: one age bearing `leaf_area` a node at `sapwood`
/// cm² per m² and 20 sapwood rings.
fn wooded(sapwood: f64, leaf_area: f64) -> Species {
    let mut state = state();
    (
        state.leaf_area,
        state.form.sapwood,
        state.form.sapwood_rings,
    ) = (leaf_area, sapwood, 20.0);
    Species {
        states: vec![state],
    }
}

/// Every phytomer's `[T, S]`, axis by axis.
fn wood(axes: &[Axis]) -> Vec<Vec<[f32; 2]>> {
    let each = |a: &Axis| a.phytomers.iter().map(|p| [p.section, p.sapwood]).collect();
    axes.iter().map(each).collect()
}

/// A constant need, year on year: the sapwood is the need from the first
/// year, and the section grows by the need in that year and by one in
/// `rings` of it every year after, whatever `rings` is.
#[test]
fn constant_leaf_holds_its_sapwood_and_adds_one_ring_of_it_a_year() {
    for rings in [1.0, 20.0, 50.0] {
        let (need, start) = (2e-4, 1e-5);
        let mut held = [start, 0.0];
        for year in 1..=80 {
            held = ring(held, need, rings);
            let years = f64::from(year - 1);
            let want = start + need + years * need / rings;
            assert!((held[1] - need).abs() < 1e-15, "{rings}, {year}: {held:?}");
            assert!(
                (held[0] - want).abs() < 1e-12 * want,
                "{rings}, {year}: {held:?}"
            );
        }
    }
}

/// A stem whose crown is removed, or whose lateral is shed, keeps its
/// section the next year: the leaf above it lays nothing more, and wood is
/// never lost; its sapwood retires by one ring.
#[test]
fn wood_is_never_lost_when_the_leaf_above_goes() {
    let species = wooded(3.0, 0.01);
    for crown in [Origin::Continuation { parent: 0 }, lateral(1)] {
        let mut axes = vec![axis(Origin::Seed, 3), axis(crown, 4)];
        lay(&mut axes, &species, (1, &[]), (&|_| false, 1));
        let before = wood(&axes);
        assert!(
            before[0][0][0] > (PI * 0.005f64.powi(2)) as f32,
            "{crown:?} laid wood"
        );
        axes.truncate(1);
        lay(&mut axes, &species, (2, &[]), (&|_| false, 1));
        for (k, (now, was)) in wood(&axes)[0].iter().zip(&before[0]).enumerate() {
            assert_eq!(now[0], was[0], "{crown:?}, node {k}: its section kept");
            assert!(now[1] < was[1], "{crown:?}, node {k}: its sapwood retires");
        }
    }
}

/// A closed lateral, whose subtree bears no leaf again, is skipped: every
/// section stands as a full pass lays it, and only the skipped wood's
/// sapwood, unread while it is skipped, is left as it was.
#[test]
fn a_closed_subtree_is_skipped_and_every_section_stays_exact() {
    let species = wooded(3.0, 0.01);
    let grown = |skip: bool| {
        let mut axes = vec![axis(Origin::Seed, 3), axis(lateral(1), 4)];
        lay(&mut axes, &species, (1, &[]), (&|_| false, 1));
        lay(&mut axes, &species, (2, &[]), (&|i| skip && i == 1, 1));
        wood(&axes)
    };
    let (full, skipped) = (grown(false), grown(true));
    assert_eq!(full[0], skipped[0], "the bearer");
    for (f, s) in full[1].iter().zip(&skipped[1]) {
        assert_eq!(f[0], s[0], "its section");
        assert!(s[1] > f[1], "its sapwood, unread, not retired");
    }
}

/// A phytomer bears its whole leaf while its age is under its PA's
/// `leaf_years`, the share left in the year it passes it and none after,
/// each at its axis's fade (LEAF-ACCOUNT.md, "The design").
#[test]
fn a_phytomer_bears_its_leaf_for_its_leaf_years_at_its_fade() {
    let mut species = wooded(3.0, 0.01);
    species.states[0].leaf_years = 5.5;
    let state = &species.states[0];
    let p = Phytomer {
        scale: 0.8,
        size: 0.5,
        ..phytomer()
    };
    let whole = crate::light::leaf_area(p.scale, p.size, state.leaf_area);
    let kept = [1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.0, 0.0];
    for (year, kept) in (1..).zip(kept) {
        for fade in [1.0, 0.25] {
            let want = whole * kept * fade;
            assert_eq!(leaf(state, &p, year, fade), want, "{year}, {fade}");
        }
        let bears = usize::from(kept == 0.0);
        assert_eq!(bearing(state, &[p], year), bears, "{year}");
    }
}

/// A closed lateral whose leaf is kept from earlier years is laid as any
/// other: its leaf stands above it and its bearer.
#[test]
fn a_closed_subtree_still_in_leaf_is_laid() {
    let mut species = wooded(3.0, 0.01);
    species.states[0].leaf_years = 3.0;
    let grown = |skip: bool| {
        let mut axes = vec![axis(Origin::Seed, 3), axis(lateral(1), 4)];
        lay(&mut axes, &species, (1, &[]), (&|_| false, 1));
        lay(&mut axes, &species, (2, &[]), (&|i| skip && i == 1, 1));
        wood(&axes)
    };
    assert_eq!(grown(true), grown(false));
}

/// An age with no sapwood draws its primary radius at its scale, whatever
/// the leaf above it.
#[test]
fn no_sapwood_draws_the_primary_radius_whatever_its_leaf() {
    for leaf in [0.0, 0.01, 1.0] {
        let species = wooded(0.0, leaf);
        let mut axes = vec![axis(Origin::Seed, 3), axis(lateral(0), 3)];
        axes[0].phytomers[1].scale = 0.5;
        for year in 1..=3 {
            lay(&mut axes, &species, (year, &[]), (&|_| false, 1));
        }
        let mut t = Structure {
            age: 3,
            pas: 1,
            axes,
        };
        thicken(&mut t, &species, &[1.0, 1.0]);
        for (k, p) in t.axes[0].phytomers.iter().enumerate() {
            let primary = established(&species.states[0].form, p.scale);
            assert!(
                (p.radius - primary).abs() < 1e-9,
                "{leaf}, node {k}: {}",
                p.radius
            );
        }
    }
}

/// A relay's leaf enters its bearer at the tip, so its join stands at the
/// radius of the wood it joins, on both sides.
#[test]
fn a_relay_joins_at_the_radius_of_the_wood_it_stands_in() {
    let mut species = wooded(3.0, 0.01);
    // A share and a blend that stand the relay part-way along a span.
    species.states[0].relay_at = 0.3;
    let mut relay = axis(Origin::Relay { parent: 0, node: 1 }, 3);
    relay.blend = 0.4;
    let mut axes = vec![axis(Origin::Seed, 4), relay];
    lay(&mut axes, &species, (1, &[]), (&|_| false, 1));
    let mut t = Structure {
        age: 1,
        pas: 1,
        axes,
    };
    thicken(&mut t, &species, &[1.0, 1.0]);
    let join = t.axes[1].joined().expect("a relay's join");
    assert!(join.t > 0.0 && join.t < 1.0, "part-way along its span");
    let r = t.axes[0].phytomers[join.node].radius;
    assert_eq!((join.below, join.above), (r, r));
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
