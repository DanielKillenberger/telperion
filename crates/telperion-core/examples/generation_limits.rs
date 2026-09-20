//! Direct mature-build cost and output fingerprints for the generation-limit audit.
use std::{fs, time::Instant};
use telperion_core::{branching, mesh, presets::Preset};

fn hash(bytes: impl IntoIterator<Item = u8>) -> String {
    format!(
        "{:016x}",
        bytes
            .into_iter()
            .fold(14695981039346656037u64, |h, b| (h ^ u64::from(b))
                .wrapping_mul(1099511628211))
    )
}

fn main() {
    for id in [
        "ordinary",
        "oregon-white-oak",
        "norway-spruce",
        "european-beech",
        "silver-birch",
        "telperion",
        "laurelin",
    ] {
        if std::env::args().nth(1).is_some_and(|wanted| wanted != id) {
            continue;
        }
        for sample in 0..3 {
            let family = Preset::from_id(id).unwrap().parameters();
            let load = fs::read_to_string("/proc/loadavg").unwrap();
            let started = Instant::now();
            let report = branching::generate(&family.skeleton, family.radii).unwrap();
            let generation_ms = started.elapsed().as_secs_f64() * 1000.;
            let assembled = Instant::now();
            let mesh = mesh::assemble(&report.tree, &family).unwrap();
            let assembly_ms = assembled.elapsed().as_secs_f64() * 1000.;
            let tree_hash = hash(bincode::serialize(&report.tree).unwrap());
            let wood_hash = hash(
                mesh.wood
                    .positions
                    .iter()
                    .chain(&mesh.wood.normals)
                    .chain(&mesh.wood.coords)
                    .flat_map(|v| v.to_le_bytes())
                    .chain(mesh.wood.indices.iter().flat_map(|v| v.to_le_bytes())),
            );
            let foliage_hash = hash(
                mesh.foliage
                    .instances
                    .leaves
                    .iter()
                    .flat_map(|leaf| leaf.iter().flat_map(|word| word.to_le_bytes())),
            );
            println!(
                "{}",
                serde_json::json!({"preset":id, "sample":sample,
                "path":"direct mature", "seed":family.skeleton.seed, "authored_age":family.age,
                "loadavg":load.trim(), "generation_ms":generation_ms,
                "assembly_ms":assembly_ms, "build_ms":generation_ms+assembly_ms,
                "nodes":report.tree.nodes.len(), "capped":report.tree.diagnostics.node_capped,
                "leaves":mesh.foliage_instances(), "tree_hash":tree_hash,
                "wood_hash":wood_hash, "foliage_hash":foliage_hash})
            );
        }
    }
}
