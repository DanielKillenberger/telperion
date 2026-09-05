use std::{hint::black_box, time::Instant};
use telperion_core::{
    branching,
    field::Field,
    foliage::{self, TwigPlacement},
    math::Vec3,
    presets::Preset,
    surface,
};
fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.
}
fn main() {
    let name = std::env::args().nth(1).unwrap_or("ordinary".into());
    let preset = match name.as_str() {
        "telperion" => Preset::Telperion,
        "laurelin" => Preset::Laurelin,
        _ => Preset::Ordinary,
    };
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
        let t = Instant::now();
        let element = foliage::build_element(f.element).unwrap();
        let twigs = f.skeleton.twigs.resolved().unwrap();
        let placed = foliage::place(
            &report.tree,
            f.skeleton.envelope,
            f.skeleton.seed,
            f.canopy,
            Some(TwigPlacement {
                internode_length: twigs.twig.internode_length,
                stations_per_internode: twigs.twig.stations_per_internode,
            }),
        )
        .unwrap();
        let placement_ms = ms(t);
        let t = Instant::now();
        let kept = foliage::cull(&placed, &element, f.skeleton.envelope, f.shell_depth).unwrap();
        let cull_ms = ms(t);
        let t = Instant::now();
        if !field_only {
            black_box(kept.bounds(&element).unwrap());
        }
        let bounds_ms = ms(t);
        let t = Instant::now();
        let field = if field_only {
            Some(Field::new(&report.tree, Some((&kept, &element))).unwrap())
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
        println!("{{\"subject\":\"{name}\",\"sample\":{sample},\"fieldOnly\":{field_only},\"nodes\":{},\"vertices\":{},\"leaves\":{},\"growthMs\":{growth_ms},\"surfaceMs\":{surface_ms},\"placementMs\":{placement_ms},\"cullMs\":{cull_ms},\"boundsMs\":{bounds_ms},\"fieldMs\":{field_ms},\"buildMs\":{build_ms},\"queryMs\":{query_ms},\"occupied\":{occupied},\"fieldBytes\":{}}}",report.tree.nodes.len(),mesh.as_ref().map_or(0,|m|m.positions.len()/3),kept.matrices.len(),field.as_ref().map_or(0,Field::storage_bytes));
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
