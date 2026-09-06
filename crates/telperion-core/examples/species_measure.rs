//! CPU-only botanical checks; each JSONL event is flushed before the next case.
mod species_metrics;
use serde_json::{json, Value};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
    time::{Instant, SystemTime, UNIX_EPOCH},
};
use telperion_core::{
    branching,
    foliage::{self, TwigPlacement},
    presets::Preset,
    surface,
};
const HELP:&str="species_measure --case ID:PROFILE:PRESET:SEED [--case ...] --output FILE [--profiles FILE]
Profiles default to .flow/evidence/fn9/profiles.json relative to the repository.
Presets: ordinary, oregon-white-oak, norway-spruce, telperion, laurelin. Unknown IDs fail; cases continue independently.
Example (compile first, then bound the entire run):
  cargo build --release -p telperion-core --example species_measure
  timeout 120s target/release/examples/species_measure --case oak-1:oregon-white-oak:oregon-white-oak:1 --output /tmp/oak-1.jsonl
Output must be new: existing evidence is never overwritten. JSONL contains run, pending,
started and completed/failed records. An interrupted run retains its completed cases;
a pending/started case without a terminal record is unassessed, never passed.
Ignore an unterminated final line after interruption; earlier newline-terminated events survive.
Exit 0 requires every case to pass its numeric gates. Visual/anatomical approval is separate.
";
fn command(program: &str, args: &[&str]) -> String {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_else(|| "unavailable".into())
}
fn event(file: &mut File, value: &Value) -> Result<(), String> {
    let mut bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    file.write_all(&bytes)
        .and_then(|_| file.sync_data())
        .map_err(|e| e.to_string())
}
fn specimen(preset: &str, seed: u32) -> Result<Value, String> {
    let preset = Preset::from_id(preset).ok_or_else(|| format!("unknown preset: {preset}"))?;
    let mut f = preset.parameters();
    f.skeleton.seed = seed;
    let total = Instant::now();
    let start = Instant::now();
    let report =
        branching::generate(&f.skeleton, f.radii).map_err(|e| format!("generation: {e:?}"))?;
    let growth_ms = start.elapsed().as_secs_f64() * 1000.;
    let start = Instant::now();
    let wood = surface::build(&report.tree, f.skeleton.envelope.height, &f.surface)
        .map_err(|e| format!("surface: {e:?}"))?;
    let surface_ms = start.elapsed().as_secs_f64() * 1000.;
    let start = Instant::now();
    let element = foliage::build_element(f.element).map_err(|e| format!("element: {e:?}"))?;
    let twigs = f
        .skeleton
        .twigs
        .resolved()
        .map_err(|e| format!("twigs: {e:?}"))?;
    let placed = foliage::place_on_surface(
        &report.tree,
        f.skeleton.envelope,
        seed,
        f.canopy,
        Some(TwigPlacement {
            internode_length: twigs.twig.internode_length,
            stations_per_internode: twigs.twig.stations_per_internode,
        }),
        &f.surface,
    )
    .map_err(|e| format!("placement: {e:?}"))?;
    let kept = foliage::cull(&placed, &element, f.skeleton.envelope, f.shell_depth)
        .map_err(|e| format!("culling: {e:?}"))?;
    let foliage_ms = start.elapsed().as_secs_f64() * 1000.;
    let start = Instant::now();
    let metrics = species_metrics::measure(
        &report.tree,
        &wood.positions,
        &element,
        placed.matrices.len(),
        &kept,
    )?;
    Ok(
        json!({"metrics":metrics,"timing_ms":{"growth":growth_ms,"surface":surface_ms,"foliage":foliage_ms,"measurement":start.elapsed().as_secs_f64()*1000.,"total":total.elapsed().as_secs_f64()*1000.},"counts":{"wood_vertices":wood.positions.len()/3,"wood_triangles":wood.indices.len()/3,"prototype_vertices":element.positions.len(),"prototype_triangles":element.indices.len()/3,"shed_nodes":report.shed},"output_bytes":{"wood_positions":wood.positions.len()*4,"wood_indices":wood.indices.len()*4,"retained_matrices":kept.matrices.len()*64}}),
    )
}
fn run() -> Result<bool, String> {
    let mut args = std::env::args().skip(1);
    let mut cases = Vec::new();
    let mut output = None;
    let mut profiles =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.flow/evidence/fn9/profiles.json");
    while let Some(arg) = args.next() {
        if arg == "--help" || arg == "-h" {
            print!("{HELP}");
            return Ok(true);
        }
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {arg}"))?;
        match arg.as_str() {
            "--case" => cases.push(value),
            "--output" => output = Some(value),
            "--profiles" => profiles = value.into(),
            _ => return Err(format!("unknown argument: {arg}")),
        }
    }
    if cases.is_empty() {
        return Err("at least one --case is required (see --help)".into());
    }
    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&profiles).map_err(|e| format!("profiles: {e}"))?)
            .map_err(|e| format!("profiles: {e}"))?;
    if manifest["schema_version"] != 1
        || manifest["status"] != "ready"
        || !manifest["frozen_at"].is_string()
    {
        return Err("invalid or unready profile manifest".into());
    }
    let targets = manifest["profiles"]
        .as_array()
        .ok_or("profiles array missing")?;
    let mut ids = std::collections::BTreeSet::new();
    for p in targets {
        let id = p["id"].as_str().ok_or("profile id missing")?;
        if !ids.insert(id) {
            return Err(format!("duplicate profile: {id}"));
        }
    }
    let mut case_ids = std::collections::BTreeSet::new();
    for case in &cases {
        if !case_ids.insert(case.split(':').next().unwrap()) {
            return Err("duplicate case ID".into());
        }
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output.ok_or("--output is required")?)
        .map_err(|e| format!("output: {e}"))?;
    event(
        &mut file,
        &json!({"event":"run","schema_version":1,"profile_revision":manifest["frozen_at"],"profile_manifest":manifest,"unix_time":SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e|e.to_string())?.as_secs(),"machine":{"os":std::env::consts::OS,"arch":std::env::consts::ARCH,"host":command("uname",&["-n"]),"cpu":fs::read_to_string("/proc/cpuinfo").ok().and_then(|s|s.lines().find(|l|l.starts_with("model name")).map(str::to_owned)),"rustc":command("rustc",&["--version"])},"git":{"commit":command("git",&["rev-parse","HEAD"]),"status":command("git",&["status","--porcelain"])},"case_count":cases.len()}),
    )?;
    for case in &cases {
        event(&mut file, &json!({"event":"pending","case":case}))?;
    }
    let mut all_pass = true;
    for case in cases {
        event(&mut file, &json!({"event":"started","case":case}))?;
        let start = Instant::now();
        let result = (|| {
            let parts: Vec<_> = case.split(':').collect();
            if parts.len() != 4 || parts.iter().any(|p| p.is_empty()) {
                return Err("case must be ID:PROFILE:PRESET:SEED".into());
            }
            let seed = parts[3]
                .parse::<u32>()
                .map_err(|_| "seed must be an unsigned 32-bit integer")?;
            let profile = targets
                .iter()
                .find(|p| p["id"] == parts[1])
                .ok_or_else(|| format!("unknown profile: {}", parts[1]))?;
            if profile["readiness"] != "ready" {
                return Err("profile not ready".into());
            }
            if let Some(id) = Preset::from_id(parts[2]).and_then(Preset::profile_id) {
                if id != parts[1] {
                    return Err(format!("preset {} requires profile {id}", parts[2]));
                }
            }
            species_metrics::compare(profile, &json!({}))?;
            let mut data = specimen(parts[2], seed)?;
            let (pass, checks) = species_metrics::compare(profile, &data["metrics"])?;
            data["event"] = json!("completed");
            data["case"] = json!(case);
            data["case_id"] = json!(parts[0]);
            data["profile_id"] = json!(parts[1]);
            data["preset"] = json!(parts[2]);
            data["seed"] = json!(seed);
            data["profile_revision"] = manifest["frozen_at"].clone();
            data["numeric_status"] = json!(if pass { "pass" } else { "fail" });
            data["checks"] = checks;
            data["visual_status"] = json!("unassessed");
            Ok((pass, data))
        })();
        let (pass,data)=result.unwrap_or_else(|reason:String|(false,json!({"event":"failed","case":case,"reason":reason,"elapsed_ms":start.elapsed().as_secs_f64()*1000.})));
        all_pass &= pass;
        event(&mut file, &data)?;
        eprintln!("{case}: {}", if pass { "pass" } else { "fail/unassessed" });
    }
    event(
        &mut file,
        &json!({"event":"run_completed","numeric_status":if all_pass{"pass"}else{"fail"}}),
    )?;
    Ok(all_pass)
}
fn main() {
    match run() {
        Ok(true) => {}
        Ok(false) => std::process::exit(1),
        Err(e) => {
            eprintln!("species_measure: {e}");
            std::process::exit(2);
        }
    }
}
