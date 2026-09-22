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
        let total = Instant::now();
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
        drop(stations);
        drop(compact);
        let start = Instant::now();
        let wood = surface::build(&report.tree, f.skeleton.envelope.height, &f.surface)?;
        let surface = start.elapsed().as_secs_f64() * 1000.;
        let start = Instant::now();
        let element = foliage::build_element(f.element)?;
        let twig = f.skeleton.twigs.resolved()?.twig;
        let reference = foliage::Reference::of(&f)?;
        let preparation = start.elapsed().as_secs_f64() * 1000.;
        let start = Instant::now();
        let placed = foliage::place_on_surface(
            &report.tree,
            f.skeleton.envelope,
            seed,
            f.canopy,
            Some(foliage::TwigPlacement {
                internode_length: twig.internode_length,
                stations_per_internode: twig.stations_per_internode,
            }),
            &f.surface,
            reference,
        )?;
        let placement = start.elapsed().as_secs_f64() * 1000.;
        let placed_count = placed.len();
        let start = Instant::now();
        let instances = foliage::cull(placed, &element, f.skeleton.envelope, f.shell_depth)?;
        let cull = start.elapsed().as_secs_f64() * 1000.;
        let start = Instant::now();
        let bounds = instances.bounds(&element)?;
        let bounds_ms = start.elapsed().as_secs_f64() * 1000.;
        let elapsed = total.elapsed().as_secs_f64() * 1000.;
        let mut hash = 14695981039346656037_u64;
        for byte in wood
            .positions
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .chain(wood.indices.iter().flat_map(|v| v.to_le_bytes()))
            .chain(
                instances
                    .leaves
                    .iter()
                    .flat_map(|leaf| leaf.iter().flat_map(|w| w.to_le_bytes())),
            )
            .chain(
                element
                    .positions
                    .iter()
                    .flat_map(|p| [p.x, p.y, p.z])
                    .flat_map(|v| v.to_le_bytes()),
            )
            .chain(element.indices.iter().flat_map(|v| v.to_le_bytes()))
            .chain(
                bounds
                    .iter()
                    .flat_map(|b| [b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z])
                    .flat_map(|v| v.to_le_bytes()),
            )
        {
            hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
        }
        println!(
            "{}",
            json!({"event":"sample","output_fnv1a64":format!("{hash:016x}"),"sample":sample,"cold_process_first_build":sample==0,
            "preset":preset,"seed":seed,"height_m":f.skeleton.envelope.height,
            "complete":report.tree.diagnostics.complete(),"nodes":report.tree.nodes.len(),
            "placed":placed_count,"retained":instances.len(),"wood_vertices":wood.positions.len()/3,
            "wood_triangles":wood.indices.len()/3,"wood_dropped":wood.dropped,"foliage_bounds_present":bounds.is_some(),
            "milliseconds":{"growth":growth,"surface":surface,"element_and_reference":preparation,
                "placement_including_attachment":placement,"cull":cull,"foliage_bounds":bounds_ms,"total":elapsed}})
        );
        if !report.tree.diagnostics.complete() {
            return Err("incomplete tree".into());
        }
    }
    Ok(())
}
