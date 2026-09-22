//! Direct mature-build stage measurements; JSONL, one process per preset/seed.
use serde_json::json;
use std::time::Instant;
use telperion_core::{branching, foliage, presets::Preset, surface};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let preset = args.get(1).ok_or("preset required")?;
    let seed: u32 = args.get(2).ok_or("seed required")?.parse()?;
    let mut f = Preset::from_id(preset)
        .ok_or("unknown preset")?
        .parameters();
    f.skeleton.seed = seed;
    if std::env::var_os("GENERATION_NO_CONTACT").is_some() {
        f.canopy.surface_contact = 0.;
    }
    let samples: usize = std::env::var("GENERATION_SAMPLES")
        .unwrap_or_else(|_| "4".into())
        .parse()?;
    if samples == 0 {
        return Err("samples must be positive".into());
    }
    println!(
        "{}",
        json!({"event":"parameters","preset":preset,"seed":seed,"family":telperion_core::params::metadata(&f)})
    );
    for sample in 0..samples {

        let start = Instant::now();
        let report = branching::generate(&f.skeleton, f.radii)?;
        let growth = start.elapsed().as_secs_f64() * 1000.;
        let start = Instant::now();
        let compact = surface::compact::prepare_with_contacts(
            &report.tree,
            f.skeleton.envelope.height,
            &f.surface,
        )?;
        let compact_ms = start.elapsed().as_secs_f64() * 1000.;
        let start = Instant::now();
        let twig_params = f.skeleton.twigs.resolved()?.twig;
        let stations = foliage::prepared::prepare_compact_stations(
            &compact,
            f.skeleton.envelope,
            f.canopy,
            Some(foliage::TwigPlacement {
                internode_length: twig_params.internode_length,
                stations_per_internode: twig_params.stations_per_internode,
            }),
        )?
        .ok_or("unsupported")?;
        let stations_ms = start.elapsed().as_secs_f64() * 1000.;
        let stages: Vec<(&str, f64)> = Vec::new(); // instrumented replacement
        let compact_hash = format!(
            "{:016x}",
            compact
                .surface()
                .rings
                .iter()
                .flatten()
                .flat_map(|w| w.to_le_bytes())
                .fold(14695981039346656037_u64, |h, b| (h ^ b as u64)
                    .wrapping_mul(1099511628211))
        );
        println!(
            "{}",
            json!({"event":"preparation","preset":preset,"seed":seed,"sample":sample,"growth_ms":growth,"compact_ms":compact_ms,"stations_ms":stations_ms,"stages":stages,"compact_hash":compact_hash,"rings":compact.surface().rings.len(),"segments":stations.segments.len(),"stations":stations.count,"contact_bytes":compact.contact_bytes()})
        );
        let path = std::env::var("STATION_DUMP")?;
        let mut out = std::io::BufWriter::new(std::fs::File::create(format!("{path}-{sample}.bin"))?);
        use std::io::Write;
        out.write_all(&stations.count.to_le_bytes())?;
        for s in &stations.segments {
            for v in [s.first, s.count, s.run_station] { out.write_all(&v.to_le_bytes())?; }
            for v in s.phase.into_iter().chain(s.endpoints.into_iter().flat_map(|p| [p.x,p.y,p.z])).chain(s.radii).chain([s.along,s.span]) { out.write_all(&v.to_le_bytes())?; }
            for v in s.contact.unwrap_or([u32::MAX;4]) { out.write_all(&v.to_le_bytes())?; }
            for v in s.frame.into_iter().flat_map(|p| [p.x,p.y,p.z]) { out.write_all(&v.to_le_bytes())?; }
        }
        out.flush()?;
        if !report.tree.diagnostics.complete() { return Err("incomplete".into()); }
    }
    Ok(())
}
