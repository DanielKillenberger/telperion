//! fn-144 R3: the date palm at seed 1 as shipped, its lattice rows swept. Each
//! base's bark ring and outer end are read out of the built mesh and unrolled
//! in that base's own frame against every base near it: cover of its cell,
//! widest gap to its fourth-nearest neighbour, deepest crowding.
use telperion_core::{math::Vec3, mesh, presets::Preset, surface::SurfaceMesh, tree::Section};

fn wrap(a: f64) -> f64 {
    (a + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU) - std::f64::consts::PI
}
fn area(p: &[(f64, f64)]) -> f64 {
    (0..p.len()).map(|i| { let (a, b) = (p[i], p[(i + 1) % p.len()]); a.0 * b.1 - b.0 * a.1 }).sum::<f64>().abs() / 2.
}
fn apart(p: &[(f64, f64)], q: &[(f64, f64)]) -> f64 {
    let mut best = f64::NEG_INFINITY;
    for poly in [p, q] {
        for i in 0..poly.len() {
            let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
            let (x, y) = (b.1 - a.1, a.0 - b.0);
            let len = (x * x + y * y).sqrt();
            if len < 1e-9 { continue; }
            let span = |s: &[(f64, f64)]| s.iter().map(|t| (t.0 * x + t.1 * y) / len)
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), v| (l.min(v), h.max(v)));
            let ((pl, ph), (ql, qh)) = (span(p), span(q));
            best = best.max((ql - ph).max(pl - qh));
        }
    }
    best
}
/// Every base's two outer rings in world space, keyed by its section.
fn rings(wood: &SurfaceMesh, seg: usize, sections: &[Section]) -> Vec<(Section, [Vec<Vec3>; 2])> {
    let v = |i: usize| Vec3::new(wood.positions[i * 3] as f64, wood.positions[i * 3 + 1] as f64, wood.positions[i * 3 + 2] as f64);
    let mut out = Vec::new();
    for run in &wood.run_table {
        let span = &wood.indices[run.first_index as usize..][..run.index_count as usize];
        let (lo, hi) = (*span.iter().min().unwrap() as usize, *span.iter().max().unwrap() as usize);
        let n = (hi + 1 - lo - 2) / seg;
        if !(2..=3).contains(&n) { continue; }
        let ring = |k: usize| (0..seg).map(|j| v(lo + k * seg + j)).collect::<Vec<_>>();
        let tip = ring(n - 1);
        let centre = tip.iter().fold(Vec3::ZERO, |a, &b| a + b) * (1. / seg as f64);
        let s = sections.iter().min_by(|a, b| {
            let d = |s: &Section| s.centre(&s.rings[2]).distance(centre);
            d(a).total_cmp(&d(b))
        }).unwrap();
        out.push((*s, [ring(n - 2), tip]));
    }
    out
}
fn main() {
    let base = Preset::from_id("date-palm").unwrap().parameters();
    let seg = base.surface.radial_segments.max(base.surface.lobes * 4) as usize;
    println!("width flatness level cover_min gap_max crowd_max");
    for width in [0.8, 0.9, 0.95, 1.0, 1.05, 1.1] {
        for flatness in [0.0, 0.5, 1.0] {
            let mut f = base.clone();
            f.skeleton.seed = 1;
            if std::env::var("STRAIGHT").is_ok() { f.skeleton.habit.crookedness = 0.; f.skeleton.habit.attractor_weight = 0.; f.skeleton.bias = telperion_core::bias::BiasParams::NONE; f.canopy.leaf_base_weathering = 0.; }
            f.canopy.leaf_base_width = width;
            f.canopy.leaf_base_flatness = flatness;
            let tree = mesh::grow(&f).unwrap();
            let wood = mesh::assemble(&tree, &f).unwrap().wood;
            let found = rings(&wood, seg, &tree.sections);
            // Metres along the stem from one base to the next, off the
            // origins in the spiral's own order.
            let mut order: Vec<&Section> = tree.sections.iter().collect();
            order.sort_by_key(|s| s.node);
            let spacing = order.windows(2).map(|w| w[0].origin.distance(w[1].origin)).sum::<f64>()
                / (order.len() - 1) as f64;
            let rank = |s: &Section| order.iter().position(|o| o.node == s.node).unwrap();
            for level in 0..2 {
                let (mut cover, mut gap, mut crowd) = (f64::INFINITY, 0f64, 0f64);
                let last = order.len() - 1;
                for (i, (s, r)) in found.iter().enumerate() {
                    let unroll = |p: Vec3| {
                        let q = p - s.origin;
                        let h = q.dot(s.axis);
                        let flat = q - s.axis * h;
                        (flat.dot(s.across).atan2(flat.dot(s.radial)), h, flat.length())
                    };
                    let lay = |ring: &[Vec3], radius: f64, lap: f64| {
                        let raw: Vec<_> = ring.iter().map(|&p| unroll(p)).collect();
                        let mut a = raw[0].0 + lap;
                        let mut out = Vec::new();
                        for (j, p) in raw.iter().enumerate() {
                            if j > 0 { a += wrap(p.0 - raw[j - 1].0); }
                            out.push((a * radius, p.1));
                        }
                        out
                    };
                    let radius = r[level].iter().map(|&p| unroll(p).2).sum::<f64>() / seg as f64;
                    let mine = lay(&r[level], radius, 0.);
                    let owed = std::f64::consts::TAU * radius * spacing;
                    let size = owed.sqrt();
                    let k = rank(s);
                    let inner = (k as f64) * spacing > 2. * size && ((last - k) as f64) * spacing > 2. * size;
                    let mut near = Vec::new();
                    for (j, (t, o)) in found.iter().enumerate() {
                        if i == j || t.origin.distance(s.origin) > 3. * size + 2. * radius { continue; }
                        for lap in [-1., 0., 1.] {
                            let d = apart(&mine, &lay(&o[level], radius, lap * std::f64::consts::TAU)) / size;
                            crowd = crowd.max(-d);
                            near.push(d.abs());
                        }
                    }
                    if inner {
                        cover = cover.min(area(&mine) / owed);
                        near.sort_by(f64::total_cmp);
                        gap = gap.max(near[3]);
                    }
                }
                println!("{width:.2} {flatness:.1} {} {cover:.3} {gap:.3} {crowd:.3}", ["bark", "tip"][level]);
            }
        }
    }
}
