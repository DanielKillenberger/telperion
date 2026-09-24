use std::time::Instant;
use telperion_core::{mesh, presets::Preset, surface};
fn main() {
    let mut f = Preset::from_id("date-palm").unwrap().parameters();
    f.skeleton.seed = 1;
    println!("surface {:?}", f.surface);
    for bases in [0u32, 32, 96, 100, 256] {
        f.canopy.leaf_bases = bases;
        let tree = mesh::grow(&f).unwrap();
        let mut best = f64::MAX;
        let mut tris = 0;
        for _ in 0..30 {
            let t = Instant::now();
            let w = surface::build(&tree, f.skeleton.envelope.height, &f.surface).unwrap();
            best = best.min(t.elapsed().as_secs_f64() * 1e3);
            tris = w.indices.len() / 3;
        }
        let t = Instant::now();
        let m = mesh::build(&f, mesh::Detail::Full).unwrap();
        let full = t.elapsed().as_secs_f64() * 1e3;
        println!("bases {bases:3} nodes {} wood_tris {tris} wood_ms {best:.3} mesh_ms {full:.1} runs {}", tree.nodes.len(), m.wood.runs);
    }
}
