//! Native generation/measurement with an isolated, bounded process for each declared case.
#[path = "geometry_benchmark/metrics.rs"]
mod metrics;
#[path = "geometry_benchmark/params.rs"]
mod params;
#[allow(dead_code)]
mod species_metrics;
use serde_json::{json, Value};
use std::{fs, process::Command, time::Instant};
use telperion_core::{
    branching,
    foliage::{self, TwigPlacement},
    presets::Preset,
    surface,
};
fn capabilities(preset: &str) -> Value {
    use telperion_core::{
        branching::BranchHabit,
        foliage::{Attachment, ElementAnatomy},
    };
    let Some(p) = Preset::from_id(preset) else {
        return json!({"implemented":false,"profile_id":null,"capabilities":[]});
    };
    let f = p.parameters();
    let mut c = vec!["woody-axes"];
    match f.element.anatomy {
        ElementAnatomy::LobedBlade => c.push("lobed-blade"),
        ElementAnatomy::FourSidedNeedle => c.push("four-sided-needle"),
        _ => {}
    }
    match f.canopy.attachment {
        Attachment::Alternate => c.push("alternate-petiole"),
        Attachment::RadialNeedles => c.push("radial-peg"),
        _ => {}
    }
    if matches!(f.skeleton.habit, BranchHabit::Tiered(_)) {
        c.push("tiered-secondary");
    }
    json!({"implemented":p.profile_id().is_some(),"profile_id":p.profile_id(),"capabilities":c})
}
fn specimen(v: &Value) -> Result<Value, String> {
    let f = params::parse(v).map_err(|e| format!("invalid-parameters: {e:?}"))?;
    let start = Instant::now();
    let report =
        branching::generate(&f.skeleton, f.radii).map_err(|e| format!("generation: {e:?}"))?;
    if !report.tree.diagnostics.complete() {
        return Err("resource-cap: generation truncated".into());
    }
    let generation_ms = start.elapsed().as_secs_f64() * 1000.;
    let wood = surface::build(&report.tree, f.skeleton.envelope.height, &f.surface)
        .map_err(|e| format!("surface: {e:?}"))?;
    let element = foliage::build_element(f.element).map_err(|e| format!("element: {e:?}"))?;
    let twigs = f
        .skeleton
        .twigs
        .resolved()
        .map_err(|e| format!("twigs: {e:?}"))?;
    let placed = foliage::place_on_surface(
        &report.tree,
        f.skeleton.envelope,
        f.skeleton.seed,
        f.canopy,
        Some(TwigPlacement {
            internode_length: twigs.twig.internode_length,
            stations_per_internode: twigs.twig.stations_per_internode,
        }),
        &f.surface,
    )
    .map_err(|e| format!("placement: {e:?}"))?;
    let kept = foliage::cull(&placed, &element, f.skeleton.envelope, f.shell_depth)
        .map_err(|e| format!("cull: {e:?}"))?;
    let legacy = species_metrics::measure(
        &report.tree,
        &wood.positions,
        &element,
        placed.matrices.len(),
        &kept,
    )?;
    let axes = metrics::axes(&report.tree)?;
    let bins = metrics::foliage_bins(&report.tree, &element, &kept)?;
    let mut result =
        json!({"metrics":{"axes":axes,"foliage_bins":bins},"legacy_metrics":legacy,"costs":[]});
    result["costs"] = json!([{"status":"measured","value":generation_ms,"unit":"ms","domain":"generation","conditions":"native cold process; one sample; resource window not exclusive; observation only","reason":null}]);
    for (domain, unit) in [
        ("capture-preparation", "ms"),
        ("cpu-rss", "bytes"),
        ("wasm-capacity", "bytes"),
        ("gpu-allocation", "bytes"),
        ("gpu-time", "ms"),
    ] {
        result["costs"].as_array_mut().unwrap().push(json!({"status":"unavailable","value":null,"unit":unit,"domain":domain,"conditions":"native numeric runner","reason":"domain not measured by this run"}));
    }
    Ok(result)
}
fn run() -> Result<i32, String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args
        .first()
        .is_some_and(|a| a == "--worker" || a == "--validate-parameters")
    {
        let v: Value = serde_json::from_slice(
            &fs::read(args.get(1).ok_or("input path missing")?).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        if args[0] == "--validate-parameters" {
            params::parse(&v).map_err(|e| format!("invalid-parameters: {e:?}"))?;
            println!("{{\"valid\":true}}");
        } else {
            println!("{}", specimen(&v)?);
        }
        return Ok(0);
    }
    if args.first().is_some_and(|a| a == "--support") {
        println!("{}", capabilities(args.get(1).ok_or("preset missing")?));
        return Ok(0);
    }
    let script = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/geometry_benchmark/runner.py"
    );
    let status = Command::new("python3")
        .arg("-B")
        .arg(script)
        .arg("--binary")
        .arg(std::env::current_exe().map_err(|e| e.to_string())?)
        .args(args)
        .status()
        .map_err(|e| e.to_string())?;
    Ok(status.code().unwrap_or(2))
}
fn main() {
    match run() {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("geometry_benchmark: {e}");
            std::process::exit(2)
        }
    }
}
