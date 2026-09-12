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
    let triangles = renderer.caster_triangles();
    let instances = renderer.caster_instances();
    let session = match Session::new(renderer.gpu()) {
        Ok(session) => session,
        Err(reason) => return Ok(qualify(Report::unavailable(hardware, reason))),
    };
    let start = pose(0.0);
    for _ in 0..CONDITIONING {
        renderer.draw(&start, viewport, target);
    }
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
    let mut counted: Vec<Vec<u32>> = Vec::with_capacity(MEASURED);
    let mut wall = Vec::with_capacity(MEASURED);
    let mut previous: Option<std::time::Instant> = None;
    for frame in 0..MEASURED {
        let now = std::time::Instant::now();
        if let Some(last) = previous.replace(now) {
            wall.push((now - last).as_secs_f64() * 1e3);
        }
        let camera = pose(frame as f64 / MEASURED as f64);
        let (pass, select, sun) = session.sample(renderer, &camera, viewport, target)?;
        vegetation.push(pass);
        selection.push(select);
        shadow.push(sun);
        if let Some(counts) = crown.then(|| renderer.level_counts()).flatten() {
            counted.push(counts);
        }
    }

    let report = Report::measured(hardware, &vegetation)
        .with_multisample(samples)
        .with_casters(triangles, instances)
        .with_passes(&vegetation, &selection, &shadow)
        .with_levels(&deviations, &counted);
    Ok(if walls {
        report.with_wall(&wall)
    } else {
        report
    })
}
