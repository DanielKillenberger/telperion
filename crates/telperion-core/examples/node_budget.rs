//! What a node budget does to one tree. One JSON line: the node count split by
//! kind, the axes that end short of their tips (a structural or branch node
//! with no child), whether the build met its budget, the build's time, the
//! skeleton's share of it, and the process's peak memory.
//! BUDGET_FAMILY names a partial wire laid over the preset, a candidate table
//! that is no shipped preset; BUDGET_MAX_NODES and BUDGET_GENERATIONS override
//! the budget and the twig generations after it, and BUDGET_MESH builds the
//! wood and leaves as well, so the time is the whole request's.
use serde_json::json;
use telperion_core::{
    params,
    pipeline::{self, Request},
    tree::NodeKind,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let preset = args.get(1).ok_or("preset required")?;
    let seed: u32 = args.get(2).ok_or("seed required")?.parse()?;
    let mut f = telperion_core::presets::Preset::from_id(preset)
        .ok_or("unknown preset")?
        .parameters();
    if let Some(path) = std::env::var_os("BUDGET_FAMILY") {
        let rows: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;
        f = params::overlay(&f, &rows).map_err(|e| format!("{e:?}"))?;
    }
    if let Ok(v) = std::env::var("BUDGET_MAX_NODES") {
        f.skeleton.growth.max_nodes = Some(v.parse()?);
    }
    if let Ok(v) = std::env::var("BUDGET_GENERATIONS") {
        f.skeleton.twigs.generations = v.parse()?;
    }
    f.skeleton.seed = seed;
    let start = std::time::Instant::now();
    let request = if std::env::var_os("BUDGET_MESH").is_some() {
        Request::mesh()
    } else {
        Request::default()
    };
    let built = pipeline::build(&f, request)?;
    let (report, skeleton_ms) = (built.skeleton, built.outputs.stages.skeleton_ms);
    let ms = start.elapsed().as_secs_f64() * 1e3;
    let tree = &report.tree;
    let mut children = vec![0_u32; tree.nodes.len()];
    for n in &tree.nodes[1..] {
        children[n.parent.unwrap() as usize] += 1;
    }
    let count = |k: NodeKind| tree.nodes.iter().filter(|n| n.kind == k).count();
    let stubs = |k: NodeKind| {
        (1..tree.nodes.len())
            .filter(|&i| tree.nodes[i].kind == k && children[i] == 0)
            .count()
    };
    println!(
        "{}",
        json!({"preset": preset, "seed": seed, "nodes": tree.nodes.len(),
            "crossover": tree.crossover, "branch": count(NodeKind::Branch),
            "twig": count(NodeKind::Twig), "structural_stubs": stubs(NodeKind::Structural),
            "branch_stubs": stubs(NodeKind::Branch), "generations": f.skeleton.twigs.generations,
            "node_capped": tree.diagnostics.node_capped,
            "twig_detail": tree.diagnostics.twig_detail, "ms": ms, "skeleton_ms": skeleton_ms, "peak_mb": peak_mb()})
    );
    Ok(())
}

/// The process's peak resident memory in MB, where the kernel reports it.
fn peak_mb() -> Option<f64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find(|l| l.starts_with("VmHWM:"))?;
    let kb: f64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some(kb / 1024.0)
}
