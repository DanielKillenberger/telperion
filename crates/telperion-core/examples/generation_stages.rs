//! Direct mature-build stage measurements through the pipeline; JSONL, one
//! process per preset/seed. Placement and the cull are timed apart, and the
//! contact rings apart from both; wood and leaves run side by side unless
//! GENERATION_SERIAL is set or the leaves are seated on the wood.
use serde_json::json;
use telperion_core::{pipeline, presets::Preset};

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
        .unwrap_or_else(|_| "6".into())
        .parse()?;
    if samples == 0 {
        return Err("samples must be positive".into());
    }
    println!(
        "{}",
        json!({"event":"parameters","preset":preset,"seed":seed,"family":telperion_core::params::metadata(&f)})
    );
    let schedule = if std::env::var_os("GENERATION_SERIAL").is_some() {
        pipeline::Schedule::Serial
    } else {
        pipeline::Schedule::Concurrent
    };
    for sample in 0..samples {
        let request = pipeline::Request {
            schedule,
            ..pipeline::Request::mesh()
        };
        let built = pipeline::build(&f, request)?;
        let tree = &built.skeleton.tree;
        let o = built.outputs;
        let t = o.stages;
        let wood = o.wood.ok_or("no wood")?;
        let element = o.element.ok_or("no element")?;
        let leaves = o.leaves.ok_or("no leaves")?;
        let (instances, bounds) = (&leaves.instances, leaves.bounds);
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
            "complete":tree.diagnostics.complete(),"nodes":tree.nodes.len(),"concurrent":t.concurrent,
            "placed":leaves.placed,"retained":instances.len(),"wood_vertices":wood.positions.len()/3,
            "wood_triangles":wood.indices.len()/3,"wood_dropped":wood.dropped,"foliage_bounds_present":bounds.is_some(),
            "milliseconds":{"growth":t.skeleton_ms,"rings":t.rings_ms,"surface":t.wood_ms,
                "placement":t.placement_ms,"cull":t.cull_ms,"total":t.total_ms}})
        );
        if !tree.diagnostics.complete() {
            return Err("incomplete tree".into());
        }
    }
    Ok(())
}
