// Readiness probe for fn-100 R5, run on master 3fdfb440 (2026-09-22). Copy into
// crates/telperion-core/examples/ to rerun: base field foliage cells versus a
// capsule sweep along leaf-bearing runs with a conservative reach.
use std::time::Instant;
use telperion_core::{
    branching,
    field::Field,
    foliage::{self, TwigPlacement},
    math::Vec3,
    presets::Preset,
    tree::NodeKind,
};

fn main() {
    let name = std::env::args().nth(1).unwrap_or("oregon-white-oak".into());
    let cell: f64 = std::env::args().nth(2).map(|s| s.parse().unwrap()).unwrap_or(0.25);
    let preset = Preset::from_id(&name).unwrap();
    let f = preset.parameters();
    let report = branching::generate(&f.skeleton, f.radii).unwrap();
    let tree = &report.tree;
    let element = foliage::build_element(f.element).unwrap();
    let twigs = f.skeleton.twigs.resolved().unwrap();
    let placed = foliage::place(
        tree, f.skeleton.envelope, f.skeleton.seed, f.canopy,
        Some(TwigPlacement { internode_length: twigs.twig.internode_length, stations_per_internode: twigs.twig.stations_per_internode }),
        foliage::Reference::of(&f).unwrap(),
    ).unwrap();
    let kept = foliage::cull(placed, &element, f.skeleton.envelope, f.shell_depth).unwrap();
    let base = Field::new(tree, Some((&kept, &element))).unwrap();
    let b = base.bounds().unwrap();
    let bmin = [b.min.x, b.min.y, b.min.z]; let bmax = [b.max.x, b.max.y, b.max.z];
    let half = cell / 2.;
    let dims = [0, 1, 2].map(|a| ((bmax[a] - bmin[a]) / cell).ceil() as usize + 1);
    let origin = b.min; let org = bmin;
    let idx = |i: usize, j: usize, k: usize| (k * dims[1] + j) * dims[0] + i;
    let n = dims[0] * dims[1] * dims[2];
    // Base foliage occupancy per cell.
    let t = Instant::now();
    let mut base_f = vec![false; n];
    for k in 0..dims[2] { for j in 0..dims[1] { for i in 0..dims[0] {
        let c = Vec3::new(origin.x + (i as f64 + 0.5) * cell, origin.y + (j as f64 + 0.5) * cell, origin.z + (k as f64 + 0.5) * cell);
        base_f[idx(i, j, k)] = base.query(c, half).unwrap().foliage;
    }}}
    let base_ms = t.elapsed().as_secs_f64() * 1000.;
    // Conservative reach per bearing segment.
    let ext = element.positions.iter().map(|p| p.length()).fold(0., f64::max);
    let blade = ext * f.canopy.size * (1. + f.canopy.size_variation);
    let seat = if f.canopy.surface_contact > 0. { f.surface.fork_swell * (1. + f.surface.lobe_depth) * f.surface.flare_radius } else { 1. };
    let slender = tree.nodes[0].radius * f.canopy.shoot_radius;
    let bearing = |i: usize| { let nd = &tree.nodes[i]; nd.parent.is_some() && (nd.kind == NodeKind::Twig || (slender > 0. && nd.radius.max(nd.start_radius) <= slender)) };
    let t = Instant::now();
    let mut circ = vec![false; n];   // wood-style circumsphere test
    let mut boxed = vec![false; n];  // sphere at nearest segment point vs cube
    let mut segs = 0usize;
    let r3 = 3f64.sqrt();
    for (i, nd) in tree.nodes.iter().enumerate() {
        if !bearing(i) { continue; }
        let p = nd.parent.unwrap() as usize;
        let (a, bb) = (tree.nodes[p].position, nd.position);
        let av = [a.x, a.y, a.z]; let bv = [bb.x, bb.y, bb.z];
        let reach = nd.radius.max(nd.start_radius) * seat + blade;
        segs += 1;
        let r = reach + half * r3;
        let lo = [0, 1, 2].map(|ax| (((av[ax].min(bv[ax]) - r - org[ax]) / cell).floor().max(0.) as usize).min(dims[ax] - 1));
        let hi = [0, 1, 2].map(|ax| (((av[ax].max(bv[ax]) + r - org[ax]) / cell).floor().max(0.) as usize).min(dims[ax] - 1));
        let d = bb - a; let dd = d.dot(d);
        for k in lo[2]..=hi[2] { for j in lo[1]..=hi[1] { for ii in lo[0]..=hi[0] {
            let c = Vec3::new(origin.x + (ii as f64 + 0.5) * cell, origin.y + (j as f64 + 0.5) * cell, origin.z + (k as f64 + 0.5) * cell);
            let tt = if dd > 0. { ((c - a).dot(d) / dd).clamp(0., 1.) } else { 0. };
            let q = a + d * tt;
            let dist = (c - q).length();
            if dist <= r { circ[idx(ii, j, k)] = true; }
            let cl = Vec3::new((q.x - c.x).clamp(-half, half) + c.x, (q.y - c.y).clamp(-half, half) + c.y, (q.z - c.z).clamp(-half, half) + c.z);
            if (q - cl).length() <= reach { boxed[idx(ii, j, k)] = true; }
        }}}
    }
    let sweep_ms = t.elapsed().as_secs_f64() * 1000.;
    let count = |v: &Vec<bool>| v.iter().filter(|x| **x).count();
    let missed_c = (0..n).filter(|&i| base_f[i] && !circ[i]).count();
    let missed_b = (0..n).filter(|&i| base_f[i] && !boxed[i]).count();
    println!("{name} cell {cell} grid {:?} segments {segs} blade {blade:.3} seat {seat:.2} leaves {} base {} ({base_ms:.0} ms) circ {} boxed {} ({sweep_ms:.0} ms) ratio_circ {:.2} ratio_box {:.2} base_not_covered circ {missed_c} box {missed_b}",
        dims, kept.len(), count(&base_f), count(&circ), count(&boxed), count(&circ) as f64 / count(&base_f) as f64, count(&boxed) as f64 / count(&base_f) as f64);
}
