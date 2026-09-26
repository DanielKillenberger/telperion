//! Native generation/measurement with an isolated, bounded process for each declared case.
#[path = "geometry_benchmark/metrics.rs"]
mod metrics;
#[allow(dead_code)]
mod species_metrics;
use serde_json::{json, Value};
use std::{fs, process::Command};
use telperion_core::{
    capability, params,
    pipeline::{self, Request},
    presets::Preset,
};
fn capabilities(preset: &str) -> Value {
    let Some(p) = Preset::from_id(preset) else {
        return json!({"implemented":false,"profile_id":null,"capabilities":[]});
    };
    // What this preset's value table produces, which is not what the generator
    // can express; that list is `capability::EXPRESSED`.
    json!({"implemented":p.profile_id().is_some(),"profile_id":p.profile_id(),
           "capabilities":capability::derived(p)})
}
fn specimen(v: &Value) -> Result<Value, String> {
    let f = params::parse(v).map_err(|e| format!("invalid-parameters: {e:?}"))?;
    // The tree the pipeline ships: its skeleton, wood and culled leaves.
    let built = pipeline::build(&f, Request::mesh()).map_err(|e| format!("build: {e:?}"))?;
    let (report, o) = (built.skeleton, built.outputs);
    if !report.tree.diagnostics.complete() {
        return Err("resource-cap: generation truncated".into());
    }
    let generation_ms = o.stages.skeleton_ms;
    let (Some(wood), Some(leaves), Some(element)) = (o.wood, o.leaves, o.element) else {
        return Err("build: the mesh request left an output out".into());
    };
    let (pre_cull_instances, kept) = (leaves.placed, leaves.instances);
    let legacy = species_metrics::measure(
        &report.tree,
        &wood.positions,
        &element,
        pre_cull_instances,
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
    // What the generator declares it can express, for the assessment round to
    // read and for the version it has to record.
    if args.first().is_some_and(|a| a == "--vocabulary") {
        println!("{}", capability::vocabulary());
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
