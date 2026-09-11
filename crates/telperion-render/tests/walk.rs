//! The plan a sequence follows, read without a device. The `headless` example's
//! own module is compiled in here, so what is asserted is the schedule the
//! target runs and not a copy of it.
#[path = "../examples/headless/walk.rs"]
#[allow(dead_code)]
mod walk;

use walk::{parse, Schedule, Walk, FPS};

use telperion_render::SceneRow;

fn frame_run(extra: &[&str]) -> Result<walk::Arguments, String> {
    let mut argv = vec![
        "--preset",
        "oregon-white-oak",
        "--to",
        "norway-spruce",
        "--seed",
        "7",
        "--out",
        "/tmp/walk/frame.png",
    ];
    argv.extend_from_slice(extra);
    parse(argv.iter().map(|word| (*word).to_owned()))
}

#[test]
fn a_frame_count_schedules_exactly_what_it_always_did() {
    // The pre-walk rule, written out: the blend spread evenly from end to end
    // and a camera that never moves. A walk must not have disturbed it.
    for count in [2u32, 24, 240] {
        let schedule = Schedule::Frames(count);
        assert_eq!(schedule.frames(), count);
        for frame in 0..count {
            let step = schedule.at(frame);
            let was = f64::from(frame) / f64::from(count - 1);
            assert_eq!(step.blend, was, "frame {frame} of {count} moved");
            assert_eq!(step.azimuth, 0.0, "a frame count turned the camera");
        }
    }
}

#[test]
fn the_parser_reads_a_frame_run_the_way_it_always_read_one() {
    let given = frame_run(&["--frames", "240"]).expect("a plain frame run parses");
    assert_eq!(given.schedule, Schedule::Frames(240));
    assert_eq!(given.size, (1024, 1024));
    assert_eq!(given.seed, 7);
    let defaulted = frame_run(&[]).expect("a frame run without a count parses");
    assert_eq!(defaulted.schedule, Schedule::Frames(240));
}

#[test]
fn a_walk_holds_at_each_end_and_eases_between_them() {
    let given = frame_run(&["--walk", "2", "--hold", "0.5", "--sweep", "30"])
        .expect("a walk in seconds parses");
    let schedule = given.schedule;
    assert_eq!(
        schedule,
        Schedule::Walk(Walk {
            seconds: 2.0,
            hold: 0.5,
            sweep: 30.0
        })
    );
    let (held, walked) = (12, 48);
    assert_eq!(schedule.frames(), walked + held * 2);
    assert_eq!(schedule.seconds(), 3.0);

    let steps: Vec<_> = (0..schedule.frames()).map(|f| schedule.at(f)).collect();
    for (frame, step) in steps.iter().enumerate() {
        let frame = frame as u32;
        if frame < held {
            assert_eq!(step.blend, 0.0, "frame {frame} left the near hold");
        } else if frame >= held + walked {
            assert_eq!(step.blend, 1.0, "frame {frame} left the far hold");
        }
        // The camera drifts through the holds as well: the sweep is time, not
        // blend, so a still tree is still being walked around.
        let share = f64::from(frame) / f64::from(schedule.frames() - 1);
        assert!((step.azimuth - 30.0 * share).abs() < 1e-12);
    }
    for pair in steps.windows(2) {
        assert!(pair[1].blend >= pair[0].blend, "the walk went backwards");
    }
    // Smoothstep is symmetric: the far half mirrors the near half exactly.
    for k in 0..walked {
        let (near, far) = (
            steps[(held + k) as usize].blend,
            steps[(held + walked - 1 - k) as usize].blend,
        );
        assert!(
            (near + far - 1.0).abs() < 1e-12,
            "the ease is lopsided at {k}"
        );
    }
}

#[test]
fn a_walk_leaves_and_arrives_at_rest() {
    let schedule = Schedule::Walk(Walk {
        seconds: 4.0,
        hold: 0.0,
        sweep: 0.0,
    });
    let steps: Vec<_> = (0..schedule.frames())
        .map(|f| schedule.at(f).blend)
        .collect();
    let middle = steps.len() / 2;
    let stride = |i: usize| steps[i + 1] - steps[i];
    assert!(
        stride(0) < stride(middle) / 2.0 && stride(steps.len() - 2) < stride(middle) / 2.0,
        "the walk sets off and pulls up as fast as it travels"
    );
    assert_eq!(steps.first().copied(), Some(0.0));
    assert_eq!(steps.last().copied(), Some(1.0));
    assert_eq!(schedule.frames(), 4 * FPS);
}

#[test]
fn a_value_a_flag_cannot_mean_is_refused_by_that_flag_s_name() {
    for (flag, extra) in [
        ("--walk", vec!["--walk", "0"]),
        ("--walk", vec!["--walk", "-3"]),
        ("--walk", vec!["--walk", "soon"]),
        ("--hold", vec!["--walk", "2", "--hold", "-1"]),
        ("--sweep", vec!["--walk", "2", "--sweep", "400"]),
        ("--sweep", vec!["--walk", "2", "--sweep", "-361"]),
        ("--hold", vec!["--hold", "1"]),
        ("--sweep", vec!["--sweep", "30"]),
        ("--walk", vec!["--walk", "2", "--frames", "48"]),
    ] {
        let refused = frame_run(&extra).expect_err(&format!("{extra:?} was accepted"));
        assert!(
            refused.contains(flag),
            "{extra:?} was refused without naming {flag}: {refused}"
        );
    }
    // A walk is a walk between two trees, like a frame count before it.
    let alone = parse(
        [
            "--preset",
            "oregon-white-oak",
            "--seed",
            "7",
            "--out",
            "/tmp/x.png",
            "--walk",
            "2",
        ]
        .iter()
        .map(|word| (*word).to_owned()),
    )
    .expect_err("a walk with nowhere to walk to was accepted");
    assert!(
        alone.contains("--walk") && alone.contains("--to"),
        "{alone}"
    );
}

#[test]
fn a_scene_row_off_the_command_line_moves_what_it_names_and_nothing_else() {
    let default = SceneRow::default();
    // No flag is the default sky, so a run that says nothing about the sun
    // still has one to draw under.
    assert_eq!(frame_run(&[]).unwrap().scene, default);
    let stated = frame_run(&["--scene", r#"{"sunElevation":12.0}"#])
        .unwrap()
        .scene;
    assert_eq!(stated.sun_elevation, 12.0);
    assert_eq!(stated.sun_azimuth, default.sun_azimuth);
    // A row the renderer will not have stops the run by the name of what was
    // wrong with it, rather than drawing under a sky nobody asked for.
    for (bad, named) in [
        (r#"{"sunElevation":120.0}"#, "sun elevation"),
        (r#"{"sunHeight":1.0}"#, "sunHeight"),
        ("{", "not JSON"),
    ] {
        let error = frame_run(&["--scene", bad]).expect_err("the row was taken anyway");
        assert!(error.contains(named), "{error} does not name {named}");
    }
}
