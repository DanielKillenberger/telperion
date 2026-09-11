//! The timing session: that every verdict is reachable from the samples that
//! define it, that no record which is not valid carries a number a reader could
//! mistake for a pass, and that a real device agrees with its own verdict.
mod common;

use telperion_core::{
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{
    attachment, hero_pose, judge, measure, measure_orbit, Hardware, Renderer, Report, Verdict,
    CONTENTION_RATIO, DEPTH_FORMAT, GROUND_REACH, STILL_FORMAT,
};

fn hardware() -> Hardware {
    Hardware {
        adapter: "a \"quoted\" adapter".into(),
        driver: "test driver".into(),
        backend: "vulkan",
    }
}

/// A run of durations around one millisecond, so a spread can be added to it.
fn steady() -> Vec<f64> {
    (0..120)
        .map(|frame| 1.0 + f64::from(frame % 5) * 0.01)
        .collect()
}

#[test]
fn every_verdict_is_reached_by_the_samples_that_define_it() {
    let mut backwards = steady();
    backwards[17] = -0.4;
    let mut missing = steady();
    missing[3] = f64::NAN;
    let mut spread = steady();
    // One frame in twenty above the ratio drags the p95 with it, which is what
    // a compositor stealing the GPU looks like from here.
    for frame in (0..120).step_by(10) {
        spread[frame] = 1.0 * CONTENTION_RATIO + 0.5;
    }

    for (name, samples, expected) in [
        ("a steady run", steady(), "valid"),
        ("a clock that ran backwards", backwards, "disjoint"),
        ("a sample that is not a number", missing, "disjoint"),
        ("a tail past the ratio", spread, "contended"),
        ("no samples at all", Vec::new(), "disjoint"),
    ] {
        let verdict = judge(&samples);
        assert_eq!(verdict.name(), expected, "{name} was judged {verdict:?}");
    }
    assert_eq!(
        Verdict::Unavailable("no timestamps".into()).name(),
        "unavailable"
    );
}

#[test]
fn a_valid_record_carries_its_numbers_and_an_invalid_one_carries_none() {
    let valid = Report::measured(hardware(), &steady());
    assert!(valid.verdict().is_valid(), "{:?}", valid.verdict());
    let json = valid.to_json();
    for field in [
        "\"adapter\"",
        "\"driver\"",
        "\"backend\": \"vulkan\"",
        "\"conditioning\"",
        "\"warmup\"",
        "\"measured\"",
        "\"samples\": 120",
        "\"verdict\": \"valid\"",
        "\"p50_ms\"",
        "\"p95_ms\"",
    ] {
        assert!(
            json.contains(field),
            "a valid record lacks {field}:\n{json}"
        );
    }
    assert!(
        json.contains("\\\"quoted\\\""),
        "the adapter's own quotes were not escaped:\n{json}"
    );
    assert!(
        !json.contains("\"reason\""),
        "a valid record excuses itself"
    );

    let mut wild = steady();
    wild[0] = f64::INFINITY;
    for invalid in [
        Report::unavailable(hardware(), "the adapter does not offer timestamp queries"),
        Report::measured(hardware(), &wild),
        Report::measured(hardware(), &[]),
    ] {
        assert_eq!(invalid.p50_ms(), None, "{:?}", invalid.verdict());
        assert_eq!(invalid.p95_ms(), None, "{:?}", invalid.verdict());
        let json = invalid.to_json();
        assert!(
            !json.contains("p50_ms") && !json.contains("p95_ms"),
            "a {} record still reads as a number:\n{json}",
            invalid.verdict().name()
        );
        assert!(
            invalid.verdict().reason().is_some() && json.contains("\"reason\""),
            "a {} record does not say why:\n{json}",
            invalid.verdict().name()
        );
    }
}

/// A selection pass an order of magnitude under the vegetation pass, which is
/// the shape the oak's numbers have.
fn quick() -> Vec<f64> {
    (0..120)
        .map(|frame| 0.05 + f64::from(frame % 5) * 0.001)
        .collect()
}

/// The same crown drawn every frame: two levels and the bucket of leaves no
/// level drew.
fn frames() -> Vec<Vec<u32>> {
    (0..120).map(|_| vec![400, 90, 10]).collect()
}

#[test]
fn a_valid_record_carries_every_pass_the_levels_and_the_orbit() {
    let json = Report::measured(hardware(), &steady())
        .with_passes(&steady(), &quick(), &quick())
        .with_levels(&[0.004, 0.001], &frames())
        .with_wall(&steady())
        .to_json();

    for field in [
        "\"selection_p50_ms\"",
        "\"selection_p95_ms\"",
        "\"shadow_p50_ms\"",
        "\"shadow_p95_ms\"",
        "\"total_p50_ms\"",
        "\"total_p95_ms\"",
        "\"wall_p50_ms\"",
        "\"wall_p95_ms\"",
        "\"wall_max_ms\"",
        // Coarsest first, with its tolerance in metres and what it drew.
        "{ \"deviation_m\": 0.004000, \"instances_p50\": 400 }",
        "{ \"deviation_m\": 0.001000, \"instances_p50\": 90 }",
        // The unseen bucket approximates nothing, so it has no tolerance.
        "{ \"deviation_m\": null, \"instances_p50\": 10 }",
    ] {
        assert!(json.contains(field), "a full record lacks {field}:\n{json}");
    }

    // The three passes are added frame by frame and then ranked, so the total
    // is the frame's own cost and not the sum of three separately ranked tails.
    let report = Report::measured(hardware(), &steady()).with_passes(&steady(), &quick(), &quick());
    let (Some(pass), Some(select), Some(sun), Some(total)) = (
        report.p50_ms(),
        report.selection_p50_ms(),
        report.shadow_p50_ms(),
        report.total_p50_ms(),
    ) else {
        panic!("a valid session reported no numbers");
    };
    assert!(
        (total - (pass + select + sun)).abs() < 1e-9,
        "{pass} ms, {select} ms and {sun} ms were reported together as {total} ms"
    );
    assert!(report.shadow_p95_ms().unwrap() >= sun);
}

#[test]
fn nothing_a_session_did_not_earn_reaches_a_record() {
    let mut wild = steady();
    wild[0] = f64::INFINITY;
    // An invalid session takes no number of any kind, however it is offered.
    for invalid in [
        Report::unavailable(hardware(), "the adapter does not offer timestamp queries"),
        Report::measured(hardware(), &wild),
        Report::measured(hardware(), &[]),
    ] {
        let name = invalid.verdict().name().to_owned();
        let json = invalid
            .with_passes(&steady(), &quick(), &quick())
            .with_levels(&[0.004, 0.001], &frames())
            .to_json();
        for field in ["selection_", "shadow_", "total_", "levels"] {
            assert!(
                !json.contains(field),
                "a {name} record still reads as a {field} number:\n{json}"
            );
        }
    }

    // A pass that never ran resolves to a pair of zeroes, and a zero is not a
    // duration: a valid frame does not lend it its verdict, and a pass that did
    // run keeps the number it earned.
    let unrun =
        Report::measured(hardware(), &steady()).with_passes(&steady(), &vec![0.0; 120], &quick());
    assert_eq!(unrun.selection_p50_ms(), None);
    assert_eq!(unrun.total_p50_ms(), None);
    assert!(unrun.shadow_p50_ms().is_some(), "the sun's pass was timed");
    assert!(unrun.verdict().is_valid(), "the frame itself was measured");

    let unlit =
        Report::measured(hardware(), &steady()).with_passes(&steady(), &quick(), &vec![0.0; 120]);
    assert_eq!(unlit.shadow_p50_ms(), None);
    assert_eq!(unlit.total_p50_ms(), None);
    assert!(unlit.selection_p50_ms().is_some(), "selection was timed");

    // A readback that disagrees with the ladder is a partial count, not a
    // count; and a session that read nothing back has no levels to report.
    let mut ragged = frames();
    ragged[7] = vec![400, 90];
    for counts in [ragged, Vec::new()] {
        let report = Report::measured(hardware(), &steady()).with_levels(&[0.004, 0.001], &counts);
        assert!(report.levels().is_empty(), "{}", report.to_json());
    }
}

#[test]
fn the_wall_clock_is_the_one_number_the_gpu_verdict_does_not_govern() {
    // A browser without the timestamp feature still draws frames, and the
    // cadence they kept is the frame rate its viewer saw. So the host's own
    // clock stands where the GPU's cannot: the record says the GPU time is
    // unavailable and reports the wall numbers beside it, over the frames they
    // were taken across.
    let json = Report::unavailable(hardware(), "the adapter does not offer timestamp queries")
        .with_wall(&steady())
        .to_json();
    for field in [
        "\"verdict\": \"unavailable\"",
        "\"wall_frames\": 120",
        "\"wall_p50_ms\"",
        "\"wall_p95_ms\"",
        "\"wall_max_ms\"",
    ] {
        assert!(
            json.contains(field),
            "an untimed orbit lacks {field}:\n{json}"
        );
    }
    assert!(
        !json.contains("\"p50_ms\"") && !json.contains("\"selection_p50_ms\""),
        "an untimed orbit reported a GPU number:\n{json}"
    );

    // What the wall clock does not excuse is a series that is not durations:
    // a window nobody waited through reports nothing.
    let mut stopped = steady();
    stopped[9] = 0.0;
    for waits in [stopped, Vec::new()] {
        let report = Report::measured(hardware(), &steady()).with_wall(&waits);
        assert_eq!(report.wall_p50_ms(), None, "{}", report.to_json());
    }
}

#[test]
fn a_session_on_a_device_never_reports_a_percentile_it_did_not_earn() {
    let Some(gpu) = common::gpu() else { return };
    let tree = mesh::build(&Preset::Ordinary.parameters(), Detail::Full).expect("the tree grew");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).expect("the tree fits the device");

    let size = (512, 512);
    let aspect = f64::from(size.0) / f64::from(size.1);
    let camera = hero_pose(
        renderer.bounds().expect("a tree is on stage"),
        aspect,
        GROUND_REACH,
    );
    let colour = attachment(renderer.gpu(), "timing", STILL_FORMAT, size);
    let depth = attachment(renderer.gpu(), "timing depth", DEPTH_FORMAT, size);
    let report = measure(
        &mut renderer,
        &camera,
        size,
        &colour.create_view(&Default::default()),
        &depth.create_view(&Default::default()),
    )
    .expect("the session ran");

    println!("{}", report.to_json());
    assert_eq!(
        report.verdict().is_valid(),
        report.p50_ms().is_some(),
        "the numbers and the verdict disagree: {:?}",
        report.verdict()
    );
    match report.verdict() {
        Verdict::Valid => {
            let (median, tail) = (report.p50_ms().unwrap(), report.p95_ms().unwrap());
            assert!(median > 0.0 && tail >= median, "{median} ms, {tail} ms");
            assert_eq!(report.samples, report.measured);
        }
        // Every other verdict is a legitimate outcome on shared hardware; what
        // is not legitimate is a number beside it, and that is asserted above.
        other => println!("skipped the numbers: {}", other.reason().unwrap_or("")),
    }
}

#[test]
fn a_device_session_times_the_selection_pass_and_counts_what_it_chose() {
    let Some(gpu) = common::gpu() else { return };
    let tree = mesh::build(&Preset::Ordinary.parameters(), Detail::Full).expect("the tree grew");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).expect("the tree fits the device");
    let crown = tree.foliage_instances();
    let deviations = renderer.level_deviations().len();

    let size = (512, 512);
    let camera = hero_pose(
        renderer.bounds().expect("a tree is on stage"),
        f64::from(size.0) / f64::from(size.1),
        GROUND_REACH,
    );
    let colour = attachment(renderer.gpu(), "timing", STILL_FORMAT, size);
    let depth = attachment(renderer.gpu(), "timing depth", DEPTH_FORMAT, size);
    let (colour, depth) = (
        colour.create_view(&Default::default()),
        depth.create_view(&Default::default()),
    );

    let still = measure(&mut renderer, &camera, size, &colour, &depth).expect("the session ran");
    let turning =
        measure_orbit(&mut renderer, &camera, size, &colour, &depth).expect("the session ran");
    println!("{}\n{}", still.to_json(), turning.to_json());

    if still.verdict().is_valid() {
        let select = still
            .selection_p50_ms()
            .expect("the selection pass was timed");
        assert!(select > 0.0, "the selection pass took {select} ms");
        let sun = still.shadow_p50_ms().expect("the shadow pass was timed");
        assert!(sun > 0.0, "the shadow pass took {sun} ms");
        // Every frame's total is that frame's three passes added, so the ranked
        // total stands at or above the ranked cost of any one of them.
        let total = still.total_p50_ms().expect("every pass together");
        assert!(
            total >= still.p50_ms().unwrap() && total >= sun,
            "the three passes together came to less than one of them"
        );
        // The camera never moves, so every measured frame counted the same
        // crown and the medians account for all of it, bucket included.
        assert_eq!(still.levels().len(), deviations + 1);
        assert_eq!(
            still.levels().iter().map(|l| l.instances_p50).sum::<u32>(),
            crown as u32,
            "the levels and the bucket do not account for the crown: {:?}",
            still.levels()
        );
        assert_eq!(still.levels().last().unwrap().deviation_m, None);
        assert_eq!(still.wall_p50_ms(), None, "a still session did not orbit");
    }

    if turning.verdict().is_valid() {
        let wall = turning
            .wall_p50_ms()
            .expect("the host timed its own frames");
        assert!(
            wall > 0.0 && turning.wall_max_ms().unwrap() >= wall,
            "the worst frame came in under the median: {}",
            turning.to_json()
        );
        assert_eq!(turning.levels().len(), deviations + 1);
    }
}
