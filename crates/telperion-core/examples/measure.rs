//! A preset's build through the pipeline, timed by stage, and a 32³ query of
//! its field: `measure <preset> [--field]`. The surface run asks for the wood
//! and the leaves; `--field` asks for the field alone, which reads the leaf
//! plan and places nothing where the family has one, as the binding does.
use std::hint::black_box;
use telperion_core::{
    field::Field,
    foliage,
    math::Vec3,
    pipeline::{self, Request},
    presets::Preset,
};

/// Occupied cells of a 32³ grid over the field's bounds.
fn occupied(field: &Field) -> usize {
    let b = field.bounds().unwrap();
    let span = b.max - b.min;
    let step = span.x.max(span.y).max(span.z) / 32.;
    let mut occupied = 0;
    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                let at = Vec3::new(x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5) * step;
                let q = field.query(b.min + at, step / 2.).unwrap();
                occupied += usize::from(q.wood || q.foliage);
            }
        }
    }
    occupied
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
            field: Some(None),
            ..Request::default()
        }
    } else {
        Request::mesh()
    };
    for sample in -1..5 {
        let built = pipeline::build(&preset.parameters(), request).unwrap();
        let (tree, o) = (&built.skeleton.tree, &built.outputs);
        let s = o.stages;
        let t = std::time::Instant::now();
        let occupied = o.field.as_ref().map_or(0, occupied);
        let query_ms = t.elapsed().as_secs_f64() * 1000.;
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
                "buildMs": s.total_ms, "queryMs": query_ms, "occupied": occupied,
                "fieldBytes": o.field.as_ref().map_or(0, Field::storage_bytes)
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
