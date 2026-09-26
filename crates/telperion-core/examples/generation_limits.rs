//! Direct mature-build cost and output fingerprints for the generation-limit audit.
use std::fs;
use telperion_core::{
    pipeline::{self, Request},
    presets::Preset,
};

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
            let built = pipeline::build(&family, Request::mesh()).unwrap();
            let stages = built.outputs.stages;
            let generation_ms = stages.skeleton_ms;
            let assembly_ms = stages.total_ms - stages.skeleton_ms;
            let tree = &built.skeleton.tree;
            let wood = built.outputs.wood.as_ref().unwrap();
            let leaves = &built.outputs.leaves.as_ref().unwrap().instances.leaves;
            let tree_hash = hash(bincode::serialize(tree).unwrap());
            let wood_hash = hash(
                wood.positions
                    .iter()
                    .chain(&wood.normals)
                    .chain(&wood.coords)
                    .flat_map(|v| v.to_le_bytes())
                    .chain(wood.indices.iter().flat_map(|v| v.to_le_bytes())),
            );
            let foliage_hash = hash(
                leaves
                    .iter()
                    .flat_map(|leaf| leaf.iter().flat_map(|word| word.to_le_bytes())),
            );
            println!(
                "{}",
                serde_json::json!({"preset":id, "sample":sample,
                "path":"direct mature", "seed":family.skeleton.seed, "authored_age":family.age,
                "loadavg":load.trim(), "generation_ms":generation_ms,
                "assembly_ms":assembly_ms, "build_ms":generation_ms+assembly_ms,
                "nodes":tree.nodes.len(), "capped":tree.diagnostics.node_capped,
                "leaves":leaves.len(), "tree_hash":tree_hash,
                "wood_hash":wood_hash, "foliage_hash":foliage_hash})
            );
        }
    }
}
