//! Scratch-only timing and complete output-byte comparison. No production instrumentation.
use serde_json::json;
use std::{io::Write, time::Instant};
use telperion_core::{branching, presets::Preset, surface};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let preset = &args[1];
    let seed: u32 = args[2].parse()?;
    let mut f = Preset::from_id(preset).ok_or("preset")?.parameters();
    f.skeleton.seed = seed;
    println!(
        "{}",
        json!({"event":"parameters", "preset":preset,"seed":seed,"family":telperion_core::params::metadata(&f)})
    );
    for sample in 0..4 {
        let report = branching::generate(&f.skeleton, f.radii)?;
        assert!(report.tree.diagnostics.complete());
        let start = Instant::now();
        let compact = surface::compact::prepare_with_contacts(
            &report.tree,
            f.skeleton.envelope.height,
            &f.surface,
        )?;
        let compact_ms = start.elapsed().as_secs_f64() * 1000.;
        let start = Instant::now();
        let wood = surface::build(&report.tree, f.skeleton.envelope.height, &f.surface)?;
        let wood_ms = start.elapsed().as_secs_f64() * 1000.;
        // Bit representations include signed zero. Private contact ranges are exposed only in scratch.
        let mut bytes = Vec::new();
        macro_rules! write_values {
            ($values:expr) => {
                for v in $values {
                    bytes.extend(v.to_le_bytes());
                }
            };
        }
        write_values!(&wood.positions);
        write_values!(&wood.normals);
        write_values!(&wood.coords);
        write_values!(&wood.indices);
        write_values!([
            wood.positions.len() as u64,
            wood.normals.len() as u64,
            wood.coords.len() as u64,
            wood.indices.len() as u64,
            wood.runs as u64,
            wood.dropped as u64
        ]);
        if let Some(b) = wood.bounds {
            write_values!([b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z]);
        }
        bytes.push(u8::from(wood.bounds.is_some()));
        let c = compact.surface();
        write_values!([c.vertices, c.indices, c.segments, c.lobes]);
        write_values!([c.depth]);
        write_values!([
            c.rings.len() as u64,
            c.angular.len() as u64,
            c.runs.len() as u64,
            c.run_table.len() as u64,
            wood.run_table.len() as u64
        ]);
        write_values!(c.rings.iter().flatten());
        write_values!(c.angular.iter().flatten());
        for r in &c.runs {
            write_values!([r.base, r.first_index, r.ring_start, r.rings, r.index_count]);
        }
        for r in wood.run_table.iter().chain(&c.run_table) {
            write_values!([r.first_index, r.index_count]);
            write_values!([r.largest_radius]);
        }
        for edge in compact.diagnostic_edges() {
            bytes.push(u8::from(edge.is_some()));
            if let Some(e) = edge {
                for &v in e {
                    write_values!([v as u64]);
                }
            }
        }
        let file = format!("{}/{}-{}.bin", args[3], preset, seed);
        if sample == 0 {
            std::fs::File::create(&file)?.write_all(&bytes)?;
        } else {
            assert_eq!(std::fs::read(&file)?, bytes, "repeat output differs");
        }
        println!(
            "{}",
            json!({"event":"sample","preset":preset,"seed":seed,"sample":sample,"compact_ms":compact_ms,"wood_ms":wood_ms,"exact_bytes":bytes.len(),"nodes":report.tree.nodes.len(),"vertices":wood.positions.len()/3,"dropped":wood.dropped,"contact_bytes":compact.contact_bytes()})
        );
    }
    Ok(())
}
