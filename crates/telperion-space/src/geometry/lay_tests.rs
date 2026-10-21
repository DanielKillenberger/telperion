//! Wood resting on the ground (fn-301 R2): a step whose bark would pass
//! below the ground ends with its centreline one radius up and keeps its
//! length, along its travel's horizontal part, or its side's where it
//! heads straight down.
use super::{rested, Layer};
use crate::geometry::q::{length, sub};
use crate::species::Form;
use crate::structure::{Phytomer, Vec3};

/// A descending step, a vertical one leaning by a hair and one whose
/// tip starts below its radius: each ends at its radius, along its
/// heading's horizontal part, its length kept.
#[test]
fn a_resting_step_keeps_its_length_and_rests_its_girth() {
    let cases: [([f32; 3], [f32; 3], f32, f32); 3] = [
        ([0.0, 0.0, 0.3], [0.6, 0.0, -0.8], 0.05, 0.5),
        ([1.0, 2.0, 0.2], [0.0, 1e-4, -1.0], 0.02, 0.4),
        ([0.0, 0.0, 0.01], [0.0, -0.6, -0.8], 0.03, 0.1),
    ];
    for (tip, direction, radius, step) in cases {
        let next = rested(tip, (direction, [1.0, 0.0, 0.0]), (radius, step));
        assert_eq!(next[2], radius, "{tip:?} {direction:?}");
        let kept = length(sub(next, tip));
        assert!((kept - step).abs() < 1e-6, "{kept} of {step}");
        let run = [next[0] - tip[0], next[1] - tip[1]];
        let cross = run[0] * direction[1] - run[1] * direction[0];
        let along = run[0] * direction[0] + run[1] * direction[1];
        assert!(
            cross.abs() < 1e-6 && along > 0.0,
            "{run:?} off {direction:?}"
        );
    }
}

/// A step heading straight down rests along its phytomer's side (host,
/// 2026-10-21), never refused. The device's `rested` (`lay_walk.wgsl`)
/// is pinned to the same words (`settle_tests.rs`).
#[test]
fn a_step_straight_down_rests_along_its_side() {
    let next = rested(
        [0.25, 0.5, 1.0],
        ([0.0, 0.0, -1.0], [0.0, -1.0, 0.0]),
        (0.25, 1.25),
    );
    assert_eq!(next, [0.25, -0.5, 0.25]);
}

/// A phytomer stepping straight down onto the ground rests along its side,
/// and the frame it stores stays whole: its side a unit square to its
/// heading, as every lateral it bears is framed on (review, 2026-10-21).
#[test]
fn a_step_straight_down_keeps_a_square_frame() {
    let mut state = crate::beech::beech().states[0].clone();
    (state.internode, state.form) = (1.0, Form::default());
    let (x, down) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
    let mut layer = Layer::bent((Vec3::new(0.0, 0.0, 0.25), down, x), Vec3::default(), 1.0);
    let mut p = Phytomer {
        cycle: 1,
        tip: Vec3::default(),
        heading: Vec3::default(),
        side: Vec3::default(),
        radius: 0.25,
        scale: 1.0,
        key: 7,
        rank: 0.0,
        size: 1.0,
        light: 1.0,
        grown: 1.0,
    };
    layer.step(&mut p, &state, (0.0, 0.0), None, false).unwrap();
    assert_eq!(p.tip, Vec3::new(1.0, 0.0, 0.25));
    assert_eq!(p.heading, x);
    let square = p.heading.dot(p.side).abs();
    assert!(
        square < 1e-6 && (p.side.length() - 1.0).abs() < 1e-6,
        "side {:?}",
        p.side
    );
}
