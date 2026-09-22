use std::{hint::black_box, time::Instant};
use telperion_core::{
    branching,
    field::Field,
    foliage::{self, plan, TwigPlacement},
    math::Vec3,
    presets::Preset,
    surface,
};
fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.
}
fn main() {
    let name = std::env::args().nth(1).unwrap_or("ordinary".into());
    let preset = Preset::from_id(&name).unwrap_or_else(|| {
        eprintln!("unknown preset: {name}");
        std::process::exit(2);
    });
    let field_only = std::env::args().any(|a| a == "--field");
    for sample in -1..5 {
        let f = preset.parameters();
        let total = Instant::now();
        let t = Instant::now();
        let report = branching::generate(&f.skeleton, f.radii).unwrap();
        let growth_ms = ms(t);
        let t = Instant::now();
        let mesh = if field_only {
            None
        } else {
            Some(surface::build(&report.tree, f.skeleton.envelope.height, &f.surface).unwrap())
        };
        let surface_ms = ms(t);
        let element = foliage::build_element(f.element).unwrap();
        let twig = TwigPlacement::of(&f).unwrap();
        // A field-only run reads the plan and places nothing, as the binding
        // does; the surface run places and culls for the render.
        let t = Instant::now();
        let leaf_plan = if field_only {
            plan::plan(
                &report.tree,
                f.skeleton.envelope,
                f.canopy,
                Some(twig),
                &f.surface,
                &element,
                None,
            )
            .unwrap()
        } else {
            None
        };
        let plan_ms = ms(t);
        let t = Instant::now();
        let kept = if leaf_plan.is_some() {
            foliage::Instances::default()
        } else {
            let placed = foliage::place(
                &report.tree,
                f.skeleton.envelope,
                f.skeleton.seed,
                f.canopy,
                Some(twig),
                foliage::Reference::of(&f).unwrap(),
            )
            .unwrap();
            foliage::cull(placed, &element, f.skeleton.envelope, f.shell_depth).unwrap()
        };
        let placement_ms = ms(t);
        let t = Instant::now();
        if !field_only {
            black_box(kept.bounds(&element).unwrap());
        }
        let bounds_ms = ms(t);
        let t = Instant::now();
        let field = if field_only {
            Some(match &leaf_plan {
                Some(leaf_plan) => Field::planned(&report.tree, leaf_plan).unwrap(),
                None => Field::new(&report.tree, Some((&kept, &element))).unwrap(),
            })
        } else {
            None
        };
        let field_ms = ms(t);
        let build_ms = ms(total);
        let t = Instant::now();
        let mut occupied = 0;
        if let Some(field) = &field {
            let b = field.bounds().unwrap();
            let span = b.max - b.min;
            let step = span.x.max(span.y).max(span.z) / 32.;
            for x in 0..32 {
                for y in 0..32 {
                    for z in 0..32 {
                        let q = field
                            .query(
                                b.min
                                    + Vec3::new(x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5)
                                        * step,
                                step / 2.,
                            )
                            .unwrap();
                        occupied += usize::from(q.wood || q.foliage);
                    }
                }
            }
        }
        let query_ms = ms(t);
        println!(
            "{}",
            serde_json::json!({
                "subject": name, "sample": sample, "fieldOnly": field_only,
                "nodes": report.tree.nodes.len(), "vertices": mesh.as_ref().map_or(0, |m| m.positions.len()/3),
                "triangles": mesh.as_ref().map_or(0, |m| m.indices.len()/3), "leaves": kept.len(),
                "woodBytes": mesh.as_ref().map_or(0, |m| (m.positions.len()+m.normals.len()+m.indices.len())*4),
                "matrixBytes": kept.len()*std::mem::size_of::<foliage::Leaf>(),
                "planned": leaf_plan.as_ref().map_or(0, |p| p.total),
                "growthMs": growth_ms, "surfaceMs": surface_ms, "planMs": plan_ms, "placementMs": placement_ms,
                "boundsMs": bounds_ms, "fieldMs": field_ms, "buildMs": build_ms,
                "queryMs": query_ms, "occupied": occupied, "fieldBytes": field.as_ref().map_or(0, Field::storage_bytes)
            })
        );
        black_box((&report, &mesh, &kept, &field));
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
