//! R1's errors: an input the engine cannot draw is refused by name, and a
//! tree that collapses, outgrows its budget or puts wood below the ground
//! is an error, never a substitute.
use std::f64::consts::PI;
use telperion_space::{grow, Error, Form, NodeLaw, PaState, Request, Species, Zone};

fn species() -> Species {
    let state = |lifespan, continuation, lateral: &[f64]| PaState {
        lifespan,
        continuation,
        viability: 1.0,
        zones: vec![Zone {
            nodes: NodeLaw::Uniform { min: 1.0, max: 2.0 },
            buds: 1.0,
            dormant: vec![0.0; lateral.len()],
            delay: 0.0,
            rate: 0.0,
            lateral: lateral.to_vec(),
        }],
        shedding: f64::INFINITY,
        internode: 1.0,
        insertion: PI / 4.0,
        divergence: PI,
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
        shade_hazard: 0.0,
        shade_size: 0.0,
        apical_control: 0.5,
        upkeep: 0.0,
        balance_hazard: 0.0,
        tolerance: 0.0,
        retained: 0.0,
        leaf_girth: 0.0,
        straightening: 0.0,
        form: Form::default(),
    };
    Species {
        states: vec![state(6.0, 0.0, &[0.0, 0.5]), state(2.0, 1.0, &[0.0, 0.0])],
    }
}

const REQUEST: Request = Request {
    age: 6,
    seed: 1,
    budget: 10_000,
    light: telperion_space::Light::NEUTRAL,
};

#[test]
fn every_input_the_engine_cannot_draw_is_refused_by_name() {
    type Edit = fn(&mut Species);
    let cases: [(Edit, &str); 42] = [
        (|s| s.states.clear(), "states"),
        (|s| s.states[0].continuation = 1.5, "states[0].continuation"),
        (
            |s| s.states[1].continuation = f64::NAN,
            "states[1].continuation",
        ),
        (|s| s.states[0].viability = f64::NAN, "states[0].viability"),
        (|s| s.states[0].shedding = -1.0, "states[0].shedding"),
        (|s| s.states[1].lifespan = -0.5, "states[1].lifespan"),
        (|s| s.states[0].zones.clear(), "states[0].zones"),
        (
            |s| s.states[0].zones = vec![s.states[0].zones[0].clone(); 5],
            "states[0].zones",
        ),
        (
            |s| s.states[0].zones[0].nodes = NodeLaw::Uniform { min: 3.0, max: 2.0 },
            "states[0].zones[0].nodes.min",
        ),
        (
            |s| {
                s.states[0].zones[0].nodes = NodeLaw::Uniform {
                    min: 0.0,
                    max: 1001.0,
                }
            },
            "states[0].zones[0].nodes.max",
        ),
        (
            |s| s.states[0].zones[0].nodes = NodeLaw::Poisson { mean: -1.0 },
            "states[0].zones[0].nodes.mean",
        ),
        (
            |s| s.states[0].zones[0].buds = 6.5,
            "states[0].zones[0].buds",
        ),
        (
            |s| s.states[0].zones[0].buds = 0.5,
            "states[0].zones[0].buds",
        ),
        (
            |s| s.states[0].zones[0].lateral.push(0.0),
            "states[0].zones[0].lateral",
        ),
        (
            |s| s.states[1].zones[0].lateral[0] = 0.5,
            "states[1].zones[0].lateral[0]",
        ),
        (
            |s| s.states[0].zones[0].lateral = vec![0.6, 0.6],
            "states[0].zones[0].lateral",
        ),
        (|s| s.states[0].internode = 0.0, "states[0].internode"),
        (|s| s.states[0].internode = 1e308, "states[0].internode"),
        (|s| s.states[1].divergence = 1e300, "states[1].divergence"),
        (|s| s.states[1].insertion = 4.0, "states[1].insertion"),
        (|s| s.states[0].abortion = 1.5, "states[0].abortion"),
        (|s| s.states[1].relay = -0.1, "states[1].relay"),
        (|s| s.states[0].readiness = f64::NAN, "states[0].readiness"),
        (|s| s.states[1].rhythm = 2.0, "states[1].rhythm"),
        (
            |s| s.states[0].straightening = -1.0,
            "states[0].straightening",
        ),
        (
            |s| s.states[0].form.tropism = -0.1,
            "states[0].form.tropism",
        ),
        (
            |s| s.states[1].form.wander = f64::NAN,
            "states[1].form.wander",
        ),
        (|s| s.states[0].form.pipe = 2.0, "states[0].form.pipe"),
        (
            |s| s.states[1].form.elevation = 2.0,
            "states[1].form.elevation",
        ),
        (|s| s.states[0].form.plane = 1e9, "states[0].form.plane"),
        (|s| s.states[1].form.roll = -0.1, "states[1].form.roll"),
        (
            |s| s.states[0].form.exponent = 1.0,
            "states[0].form.exponent",
        ),
        (
            |s| s.states[1].form.ripening = -1.0,
            "states[1].form.ripening",
        ),
        (
            |s| s.states[1].form.dominance = 1.5,
            "states[1].form.dominance",
        ),
        (|s| s.states[1].relay_at = 1.5, "states[1].relay_at"),
        (|s| s.states[0].epitony = -0.5, "states[0].epitony"),
        (
            |s| s.states[1].abortion_rise = 9.0,
            "states[1].abortion_rise",
        ),
        (|s| s.states[0].erection = -1.0, "states[0].erection"),
        (|s| s.states[1].form.sag = -0.1, "states[1].form.sag"),
        (|s| s.states[0].form.sag = f64::NAN, "states[0].form.sag"),
        (
            |s| s.states[0].form.secondary = -0.1,
            "states[0].form.secondary",
        ),
        (
            |s| s.states[1].form.secondary = f64::NAN,
            "states[1].form.secondary",
        ),
    ];
    for (edit, input) in cases {
        let mut s = species();
        edit(&mut s);
        match grow(&s, REQUEST) {
            Err(Error::Refused { input: named, .. }) => assert_eq!(named, input),
            other => panic!("{input}: {other:?}"),
        }
    }
    let refused = grow(&species(), Request { age: 0, ..REQUEST });
    assert!(matches!(refused, Err(Error::Refused { input, .. }) if input == "age"));
    let lights = [
        (f64::NAN, 0.5, "light.extinction"),
        (-0.1, 0.5, "light.extinction"),
        (11.0, 0.5, "light.extinction"),
        (0.5, 1.5, "light.sky"),
        (0.5, f64::NAN, "light.sky"),
    ];
    for (extinction, sky, input) in lights {
        let light = telperion_space::Light { extinction, sky };
        match grow(&species(), Request { light, ..REQUEST }) {
            Err(Error::Refused { input: named, .. }) => assert_eq!(named, input),
            other => panic!("{input}: {other:?}"),
        }
    }
}

#[test]
fn a_seed_that_dies_before_it_grows_is_a_collapsed_tree() {
    let mut s = species();
    s.states[0].viability = 0.0;
    assert_eq!(grow(&s, REQUEST), Err(Error::Collapsed));
}

#[test]
fn a_tree_past_its_budget_is_refused() {
    assert_eq!(
        grow(
            &species(),
            Request {
                budget: 5,
                light: telperion_space::Light::NEUTRAL,
                ..REQUEST
            }
        ),
        Err(Error::Budget { limit: 5 })
    );
}

#[test]
fn a_trunk_below_the_ground_is_an_error() {
    let mut s = species();
    s.states[0].form.tropism = 1.0;
    s.states[0].form.elevation = -std::f64::consts::FRAC_PI_2;
    assert!(matches!(grow(&s, REQUEST), Err(Error::BelowGround { .. })));
}

/// A tree whose only wood stands exactly at its draw has no size: it is a
/// collapsed tree, not a tree of zero length.
#[test]
fn a_tree_of_no_size_is_a_collapsed_tree() {
    let mut s = species();
    s.states.truncate(1);
    s.states[0].zones[0].lateral = vec![0.0];
    s.states[0].zones[0].dormant = vec![0.0];
    // The greatest mean at which the seed's one node stands at its draw
    // under the keys of fn-206 (axes keyed by their age), found by
    // bisection: a hair more and it grows in.
    s.states[0].zones[0].nodes = NodeLaw::Poisson {
        mean: 0.27738411705607313,
    };
    assert_eq!(
        grow(&s, Request { age: 1, ..REQUEST }),
        Err(Error::Collapsed)
    );
}
