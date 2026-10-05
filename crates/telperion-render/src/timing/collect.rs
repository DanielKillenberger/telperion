//! Native session assembly, with geometry counts independent of clock validity.
use super::*;

pub(super) fn collect(
    renderer: &mut Renderer,
    viewport: (u32, u32),
    target: Target<'_>,
    pose: impl Fn(f64) -> Camera,
    walls: bool,
) -> Result<Report> {
    let mut hardware = Hardware::from(&renderer.gpu().adapter);
    hardware.adapter.push_str(&format!(
        "; comparison filtering: {:?}",
        renderer.gpu().shadow_filter()
    ));
    // What the frame was drawn at belongs to every record, measured or not: a
    // number is only comparable with another taken at the same count.
    let samples = renderer.samples();
    let qualify = |report: Report| {
        report
            .with_multisample(samples)
            .with_casters(renderer.caster_triangles(), renderer.caster_instances())
    };
    let instances = renderer.caster_instances();
    let session = match Session::new(renderer.gpu()) {
        Ok(session) => session,
        Err(reason) => return Ok(qualify(Report::unavailable(hardware, reason))),
    };
    let start = pose(0.0);
    for _ in 0..CONDITIONING {
        renderer.draw(&start, viewport, target);
    }
    // The wood the sun was given is surfaced by a frame, so it is counted
    // after one.
    let triangles = renderer.caster_triangles();
    for _ in 0..WARMUP {
        session.sample(renderer, &start, viewport, target)?;
    }

    // Only a view that draws the crown runs selection, so only it has counters
    // worth reading; a bare or leaf session records the passes and no levels.
    let crown = renderer.view().selects();
    let deviations = renderer.level_deviations().to_vec();
    let mut vegetation = Vec::with_capacity(MEASURED);
    let mut selection = Vec::with_capacity(MEASURED);
    let mut shadow = Vec::with_capacity(MEASURED);
    let mut surfacing = Vec::with_capacity(MEASURED);
    let mut counted: Vec<Vec<u32>> = Vec::with_capacity(MEASURED);
    let mut wall = Vec::with_capacity(MEASURED);
    let mut previous: Option<std::time::Instant> = None;
    for frame in 0..MEASURED {
        let now = std::time::Instant::now();
        if let Some(last) = previous.replace(now) {
            wall.push((now - last).as_secs_f64() * 1e3);
        }
        let camera = pose(frame as f64 / MEASURED as f64);
        let [pass, select, sun, surfaced] = session.sample(renderer, &camera, viewport, target)?;
        vegetation.push(pass);
        selection.push(select);
        shadow.push(sun);
        surfacing.push(surfaced);
        if let Some(counts) = crown.then(|| renderer.level_counts()).flatten() {
            counted.push(counts);
        }
    }

    let read = |r: Option<crate::CurveReport>| r.map(|r| (r.scale, r.overrun));
    let wood = (
        read(renderer.curve_report()),
        read(renderer.curve_sun_report()),
    );
    let report = Report::measured(hardware, &vegetation)
        .with_wood(wood.0, wood.1)
        .with_multisample(samples)
        .with_casters(triangles, instances)
        .with_passes(&vegetation, &selection, &shadow, &surfacing)
        .with_levels(&deviations, &counted);
    Ok(if walls {
        report.with_wall(&wall)
    } else {
        report
    })
}
