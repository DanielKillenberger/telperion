use serde_json::json;
use std::{path::Path, time::Instant};
use telperion_core::{branching, presets::Preset};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let preset = &args[1];
    let seed: u32 = args[2].parse()?;
    let output = Path::new(&args[3]).join(format!("{preset}-{seed}.bin"));
    let mut family = Preset::from_id(preset).ok_or("preset")?.parameters();
    family.skeleton.seed = seed;
    println!(
        "{}",
        json!({"event":"parameters","preset":preset,"seed":seed,"family":telperion_core::params::metadata(&family)})
    );
    for sample in 0..4 {
        let start = Instant::now();
        let report = branching::generate(&family.skeleton, family.radii)?;
        let growth_ms = start.elapsed().as_secs_f64() * 1000.;
        report.tree.validate_solved()?;
        assert!(report.tree.diagnostics.complete());
        let bytes = bincode::serialize(&(&report.tree, report.shed))?;
        if sample == 0 {
            std::fs::write(&output, &bytes)?;
        } else {
            assert_eq!(bytes, std::fs::read(&output)?);
        }
        println!(
            "{}",
            json!({"event":"sample","preset":preset,"seed":seed,"sample":sample,"growth_ms":growth_ms,"nodes":report.tree.nodes.len(),"exact_bytes":bytes.len(),"complete":true})
        );
    }
    Ok(())
}
