//! fn-212 task 3 scratch (never committed): what each CPU step does with a
//! tree that keeps every phytomer.
//!   f212_d1 <preset> <age> <seed>
use std::time::Instant;
use telperion_core::{mesh, pipeline::executor, presets::Preset};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let mut f = Preset::from_id(&a[0]).ok_or("preset")?.parameters();
    (f.age, f.skeleton.seed) = (a[1].parse()?, a[2].parse()?);
    let t = Instant::now();
    let x = executor::grow(&f)?.expansion()?;
    let tree = x.tree();
    let thin = tree.nodes.iter().filter(|n| n.radius < 2e-4).count();
    let short = tree.nodes.iter().skip(1)
        .filter(|n| (n.position - tree.nodes[n.parent.unwrap() as usize].position).length() < 1e-3)
        .count();
    println!("{} {} {}: nodes {} thin {thin} short {short} grow {:.1}s seats {}", a[0], a[1], a[2],
        tree.nodes.len(), t.elapsed().as_secs_f64(), x.seats());
    let t = Instant::now();
    let curve = x.curve();
    println!("  curve {:?} {:.1}s", curve.as_ref().map(|c| c.points.len()).map_err(|e| e.to_string()), t.elapsed().as_secs_f64());
    let t = Instant::now();
    let stations = x.stations().map(|s| s.map(|s| s.count));
    println!("  stations {:?} {:.1}s", stations.map_err(|e| e.to_string()), t.elapsed().as_secs_f64());
    if x.seats() {
        let c = x.compact_with_contacts().map(|c| c.surface().qualified());
        println!("  compact qualified {:?}", c.map_err(|e| e.to_string()));
        // Which runs hold an unqualified ring, and whether a station seats on one.
        let shared = x.compact_with_contacts()?;
        let surface = shared.surface();
        let q = |r: &[u32; 16]| {
            let c = [r[0], r[1], r[2]].map(f32::from_bits);
            let radius = f32::from_bits(r[3]);
            let most = c.iter().map(|v| v.abs()).fold(1.0_f32, f32::max);
            c.iter().all(|v| v.is_finite() && v.abs() <= 64.0) && radius.is_finite() && radius <= 32.0 && radius >= most / 131072.0
        };
        let bad: Vec<(u32, u32)> = surface.runs.iter()
            .filter(|r| surface.rings[r.ring_start as usize..(r.ring_start + r.rings) as usize].iter().any(|g| !q(g)))
            .map(|r| (r.base, r.base + r.rings * surface.segments))
            .collect();
        let st = x.compact_stations(&shared)?.unwrap();
        let mut runs = std::collections::BTreeSet::new();
        let mut on_bad = 0usize;
        let mut bad_rings_seated = 0usize;
        for s in &st.segments {
            if let Some(c) = s.contact {
                runs.insert(c[2]);
                if bad.iter().any(|&(lo, hi)| c[0] >= lo && c[0] < hi) { on_bad += s.count as usize; }
            }
        }
        for r in surface.runs.iter().filter(|r| runs.contains(&r.base)) {
            bad_rings_seated += surface.rings[r.ring_start as usize..(r.ring_start + r.rings) as usize].iter().filter(|g| !q(g)).count();
        }
        println!("  runs {} with unqualified rings {}; bearing runs {}; stations on unqualified runs {} of {}; unqualified rings on bearing runs {}",
            surface.runs.len(), bad.len(), runs.len(), on_bad, st.count, bad_rings_seated);
        let p = x.prepared_with_contacts().map(|p| p.is_some());
        println!("  canonical {:?}", p.map_err(|e| e.to_string()));
    }
    let t = Instant::now();
    let m = mesh::build(&f);
    println!("  mesh {:?} {:.1}s", m.as_ref().map(|m| (m.wood_triangles(), m.foliage_instances())).map_err(|e| e.to_string()), t.elapsed().as_secs_f64());
    Ok(())
}
