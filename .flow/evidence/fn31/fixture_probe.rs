use telperion_core::{
    branching::Specimen,
    mesh::{self, Detail},
    presets::Preset,
};
fn main() {
    for (preset, cap) in [
        (Preset::Ordinary, Some(400)),
        (Preset::Ordinary, Some(20)),
        (Preset::OregonWhiteOak, None),
        (Preset::NorwaySpruce, None),
    ] {
        let mut f = preset.parameters();
        f.skeleton.seed = if cap.is_none() { 7 } else { f.skeleton.seed };
        f.skeleton.growth.max_nodes = cap;
        let s = Specimen::build(&f).unwrap();
        let t = s.tree();
        println!(
            "{preset:?} cap={cap:?} age={} requested={} nodes={} crossover={} diagnostics={:?}",
            s.age(),
            f.age,
            t.nodes.len(),
            t.crossover,
            t.diagnostics
        );
        for (i, n) in t.nodes.iter().take(12).enumerate() {
            println!(
                "node {i}: p={:?} parent={:?} radius={} proximal={} kind={:?}",
                n.position, n.parent, n.radius, n.start_radius, n.kind
            );
        }
        let mesh = mesh::build(&f, Detail::Full).unwrap();
        println!(
            "wood={} placements={} bounds={:?}",
            mesh.wood_triangles(),
            mesh.foliage_instances(),
            mesh.bounds
        );
        if let Some(run) = mesh.wood.run_table.first() {
            let mut lo = f32::INFINITY;
            let mut hi = f32::NEG_INFINITY;
            for &i in &mesh.wood.indices
                [run.first_index as usize..(run.first_index + run.index_count) as usize]
            {
                let y = mesh.wood.positions[i as usize * 3 + 1];
                lo = lo.min(y);
                hi = hi.max(y);
            }
            println!("run0 indices={} y={lo}..{hi}", run.index_count);
        }
    }
}
