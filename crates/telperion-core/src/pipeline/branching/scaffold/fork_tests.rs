use super::*;

fn habit(ways: f64, divergence: f64, lean: f64, spread: f64) -> HabitParams {
    HabitParams {
        codominance: 0.5,
        fork_ways: ways,
        fork_divergence: divergence,
        fork_lean: lean,
        fork_lean_spread: spread,
        ..HabitParams::default()
    }
}

/// Degrees from the axis a heading leaves at.
fn tilt(heading: Vec3) -> f64 {
    heading.y.clamp(-1.0, 1.0).acos().to_degrees()
}

#[test]
fn rate_zero_never_forks_whatever_the_other_rows_say() {
    let h = HabitParams {
        codominance: 0.0,
        fork_height: 0.0,
        fork_height_spread: 1.0,
        fork_ways: 4.0,
        ..HabitParams::default()
    };
    for key in 0..512 {
        assert_eq!(decide(&h, key, 20.0, 0.0, None), None, "key {key}");
    }
    // Nor is a family that never forks refused for a fan it never grows.
    let mut p = crate::presets::Preset::Ordinary.parameters().skeleton;
    p.habit = h;
    assert_eq!(placed(&p), Ok(()));
}

#[test]
fn a_bell_with_no_width_forks_at_its_centre_a_station_above_the_base() {
    let h = HabitParams {
        codominance: 1.0,
        fork_height: 0.25,
        ..HabitParams::default()
    };
    let at = |base: f64, least: Option<f64>| decide(&h, 3, 20.0, base, least).map(|f| f.at);
    assert_eq!(at(1.0, Some(1.5)), Some(5.0));
    // A height at the axis's own base, or less than a station above it, is
    // no fork of it ...
    assert_eq!(at(5.0, Some(1.5)), None);
    assert_eq!(at(4.0, Some(1.5)), None);
    // ... and the root alone forks at its base, which is a clump.
    assert_eq!(at(5.0, None), Some(5.0));
}

#[test]
fn the_rate_grows_each_fork_in_and_never_takes_one_away() {
    let weights = |rate: f64| -> Vec<f64> {
        let h = HabitParams {
            codominance: rate,
            ..HabitParams::default()
        };
        (0..256)
            .map(|key| decide(&h, key, 20.0, 0.0, None).map_or(0.0, |f| f.weight))
            .collect()
    };
    assert!(weights(0.0).iter().all(|&w| w == 0.0));
    assert!(
        weights(1.0).iter().all(|&w| w == 1.0),
        "a rate of one forks every tree whole"
    );
    let mut was = weights(0.0);
    for step in 1..=400 {
        let now = weights(f64::from(step) / 400.0);
        for (key, (a, b)) in was.iter().zip(&now).enumerate() {
            assert!(b >= a, "key {key} lost weight at step {step}");
            // A step of the rate a tenth of the grow-in width grows a fork by
            // a bounded amount: in, never all at once.
            assert!(b - a < 0.2, "key {key} jumped {a} to {b} at step {step}");
        }
        was = now;
    }
}

#[test]
fn every_slot_stands_where_it_stood_whatever_the_ways() {
    let slots = |ways: f64| -> Vec<Vec3> {
        let h = habit(ways, 70.0, 25.0, 0.4);
        (0..4).map(|k| slot(&h, 7, k, 1.0)).collect()
    };
    let two = slots(2.0);
    for ways in [2.4, 3.0, 3.6, 4.0] {
        assert_eq!(slots(ways), two, "{ways} moved a slot");
    }
    // Neighbouring slots stand the divergence apart in bearing.
    let bearing = |v: Vec3| v.z.atan2(v.x);
    let turn = (bearing(two[2]) - bearing(two[1])).rem_euclid(TAU);
    assert!((turn - 70f64.to_radians()).abs() < 1e-12);
    // Every sibling leans by the whole lean, the primary by what the spread
    // leaves it of it.
    for sibling in &two[1..] {
        assert!((tilt(*sibling) - 25.0).abs() < 1e-9);
    }
    assert!((tilt(two[0]) - 25.0 * 0.6).abs() < 1e-9);
}

#[test]
fn walking_the_ways_grows_each_part_in_from_nothing() {
    let shares = |ways: f64| -> Vec<f64> {
        let mut out = vec![0.0; 3];
        for (k, share) in siblings(&habit(ways, 60.0, 25.0, 0.0)) {
            out[k - 1] = share;
        }
        out
    };
    assert_eq!(shares(2.0), [1.0, 0.0, 0.0]);
    assert_eq!(shares(3.0), [1.0, 1.0, 0.0]);
    assert_eq!(shares(4.0), [1.0, 1.0, 1.0]);
    // Just past a whole count the next part is next to nothing, and just
    // short of the next it is nearly whole.
    for whole in [2.0, 3.0] {
        let k = whole as usize - 1;
        assert!(shares(whole + 1e-9)[k] < 1e-8);
        assert!(shares(whole + 1.0 - 1e-9)[k] > 1.0 - 1e-8);
        assert!(shares(whole - 1e-9)[k] == 0.0);
    }
    let mut was = 0.0;
    for step in 1..=10 {
        let now = shares(2.0 + f64::from(step) / 10.0)[1];
        assert!(now > was);
        was = now;
    }
}

#[test]
fn a_fork_growing_in_turns_its_primary_by_as_much() {
    let h = habit(2.0, 60.0, 30.0, 0.0);
    assert_eq!(slot(&h, 5, 0, 0.0), Vec3::Y);
    assert!((tilt(slot(&h, 5, 0, 0.5)) - 15.0).abs() < 1e-9);
    assert!((tilt(slot(&h, 5, 0, 1.0)) - 30.0).abs() < 1e-9);
}

#[test]
fn a_slot_about_the_axis_is_the_slot_about_up_turned_onto_it() {
    let h = habit(3.0, 40.0, 20.0, 0.0);
    let axis = Vec3::new(0.6, 0.8, 0.0);
    for k in 0..4 {
        let local = slot(&h, 3, k, 1.0);
        let turned = about(local, axis);
        let angle = |a: Vec3, b: Vec3| a.dot(b).clamp(-1.0, 1.0).acos();
        assert!((angle(turned, axis) - angle(local, Vec3::Y)).abs() < 1e-12);
    }
    assert_eq!(about(Vec3::Y, Vec3::Y), Vec3::Y);
}

#[test]
fn parts_on_one_heading_are_refused_by_the_part_that_is_wrong() {
    let mut p = crate::presets::Preset::Ordinary.parameters().skeleton;
    for (divergence, lean, spread) in [(0.0, 20.0, 0.0), (60.0, 0.0, 0.0), (60.0, 0.0, 1.0)] {
        p.habit = habit(2.0, divergence, lean, spread);
        assert_eq!(
            placed(&p),
            Err(Error::InvalidValue {
                field: "fork parts pass through each other",
                value: "part 1".into(),
            }),
            "{divergence} deg apart at {lean} deg of lean was accepted"
        );
    }
    p.habit = habit(2.0, 60.0, 20.0, 0.0);
    assert_eq!(placed(&p), Ok(()));
    // A spread parts two parts on one bearing by standing the primary back.
    p.habit = habit(2.0, 0.0, 20.0, 0.5);
    assert_eq!(placed(&p), Ok(()));
}
