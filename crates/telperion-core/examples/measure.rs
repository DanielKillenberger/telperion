//! A preset's build through the pipeline, timed by stage, and its field
//! read to a 32³ grid: `measure <preset> [--field]`. The surface run asks for
//! the wood and the leaves; `--field` asks for the field alone, which reads
//! the leaf plan and places nothing, as the binding does.
use std::hint::black_box;
use telperion_core::{
    field::{Grid, Occupancy},
    foliage,
    pipeline::{self, FieldRequest, Request},
    presets::Preset,
};

/// Occupied cells of the field's grid.
fn occupied(grid: &Grid) -> usize {
    grid.cells.iter().filter(|q| q.wood || q.foliage).count()
}

fn main() {
    let name = std::env::args().nth(1).unwrap_or("ordinary".into());
    let preset = Preset::from_id(&name).unwrap_or_else(|| {
        eprintln!("unknown preset: {name}");
        std::process::exit(2);
    });
    let field_only = std::env::args().any(|a| a == "--field");
    let request = if field_only {
        Request {
            field: Some(FieldRequest {
                n: 32,
                limb_order: None,
            }),
            ..Request::default()
        }
    } else {
        Request::mesh()
    };
    for sample in -1..5 {
        let built = pipeline::build(&preset.parameters(), request).unwrap();
        let (tree, o) = (&built.skeleton.tree, &built.outputs);
        let s = o.stages;
        let occupied = o.field.as_ref().map_or(0, occupied);
        let kept = o.leaves.as_ref().map_or(0, |l| l.retained);
        let mesh = o.wood.as_ref();
        println!(
            "{}",
            serde_json::json!({
                "subject": name, "sample": sample, "fieldOnly": field_only,
                "nodes": tree.nodes.len(), "vertices": mesh.map_or(0, |m| m.positions.len()/3),
                "triangles": mesh.map_or(0, |m| m.indices.len()/3), "leaves": kept,
                "woodBytes": mesh.map_or(0, |m| (m.positions.len()+m.normals.len()+m.indices.len())*4),
                "matrixBytes": kept*std::mem::size_of::<foliage::Leaf>(),
                "planned": o.plan.as_ref().map_or(0, |p| p.total),
                "growthMs": s.skeleton_ms, "surfaceMs": s.rings_ms + s.wood_ms, "planMs": s.plan_ms,
                "placementMs": s.placement_ms, "cullMs": s.cull_ms, "fieldMs": s.field_ms,
                "buildMs": s.total_ms, "occupied": occupied,
                "fieldBytes": o.field.as_ref().map_or(0, |g| g.cells.len() * std::mem::size_of::<Occupancy>())
            })
        );
        black_box(&built);
    }
    eprintln!(
        "{}",
        std::fs::read_to_string("/proc/self/status")
            .unwrap()
            .lines()
            .filter(|line| line.starts_with("VmHWM:") || line.starts_with("VmRSS:"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
