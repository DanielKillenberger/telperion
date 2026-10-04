//! The tropism's turn near straight down: a direction swept through the
//! vertical turns by degree, never flipping from one side to the other.
use super::*;

#[test]
fn a_direction_swept_through_straight_down_turns_by_degree() {
    // The axis's carried side is level and square to the sweep, as a
    // lateral's frame is.
    let side = Vec3::new(1.0, 0.0, 0.0);
    let turned = |angle: f64| {
        let direction = Vec3::new(0.0, angle.sin(), -angle.cos());
        toward_elevation(direction, side, 0.4, 0.3)
    };
    let steps = 2_000;
    let span = 0.2;
    let step = 2.0 * span / f64::from(steps);
    let mut worst = 0.0f64;
    for i in 0..steps {
        let a = -span + step * f64::from(i);
        worst = worst.max((turned(a + step) - turned(a)).length());
    }
    // Continuous: a step of 2e-4 rad moves the turn by a small amount (a
    // flip moves it by about the turn's length, near 1).
    assert!(worst < 0.01, "one step moves the turn by {worst}");
}
