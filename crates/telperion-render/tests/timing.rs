//! The timing session: that every verdict is reachable from the samples that
//! define it, that no record which is not valid carries a number a reader could
//! mistake for a pass, and that a real device agrees with its own verdict.
mod common;

use telperion_core::{
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{
    attachment, hero_pose, judge, measure, Hardware, Renderer, Report, Verdict, CONTENTION_RATIO,
    DEPTH_FORMAT, GROUND_REACH, STILL_FORMAT,
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
        aspect,
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
