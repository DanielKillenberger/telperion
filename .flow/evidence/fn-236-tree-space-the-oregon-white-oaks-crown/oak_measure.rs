//! fn-236 probe, not product code: the oak's crown measures from the
//! current preset, grown through the pipeline as `space_oak` grows it.
//! Copied into `crates/telperion-core/examples/oak_measure.rs` by
//! `measure.sh` for a run and removed after.
//!
//!   cargo run --profile ci -p telperion-core --example oak_measure -- <ages> <seeds>
//!
//! One JSON line per tree. Crown points are the nodes of fine wood (radius
//! under 5 mm), where the leaves sit; y is up, the trunk's root at x = z = 0.
//! - `height`, `width` (mean of the x and z extents of the crown points),
//!   `wh` (width over height);
//! - `base`: the 2nd percentile of the crown points' height, the crown base;
//! - `fill`: in two side views (along x, along z) on a grid of H/60 cells,
//!   occupied cells over the cells inside each row's outline (gaps inside
//!   the crown open it); `solid`: the row outline's area over its convex
//!   hull's (a limb past the edge lowers it); means of the two views;
//! - `spikes`: of 36 plan sectors about the trunk, those whose farthest
//!   crown point lies past 1.25 times the median sector's reach (limbs past
//!   the crown's edge); `top`: height of the highest crown point over the
//!   99th percentile, a tuft above the dome;
//! - `dbh`: the stem's diameter in cm at the node of the root's run nearest
//!   1.3 m, and `mcw`: Paine and Hann's (1982) open-grown maximum crown
//!   width for it, 3.0786 + 1.9242 D ft at D in inches, in metres.
use std::time::Instant;
use telperion_core::{pipeline::executor, presets::Preset, tree::Tree};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let list = |i: usize| -> Vec<u32> { args[i].split(',').map(|v| v.parse().unwrap()).collect() };
    let base = Preset::from_id("oregon-white-oak").unwrap().parameters();
    for &age in &list(0) {
        for &seed in &list(1) {
            let mut family = base.clone();
            family.age = f64::from(age);
            family.skeleton.seed = seed;
            let t = Instant::now();
            let grown = executor::grow(&family).expect("grows");
            let x = grown.expansion().expect("expands");
            let ms = t.elapsed().as_secs_f64() * 1e3;
            let m = measure(x.tree());
            println!(
                "{{\"age\":{age},\"seed\":{seed},\"ms\":{ms:.0},\"nodes\":{},\"height\":{:.1},\"width\":{:.1},\"wh\":{:.2},\"base\":{:.1},\"fill\":{:.2},\"solid\":{:.2},\"spikes\":{},\"top\":{:.1},\"dbh\":{:.1},\"mcw\":{:.1}}}",
                x.tree().nodes.len(), m.height, m.width, m.width / m.height, m.base, m.fill, m.solid, m.spikes, m.top,
                m.dbh, 0.3048 * (3.0786 + 1.9242 * m.dbh / 2.54)
            );
        }
    }
}

struct Measures {
    height: f64,
    width: f64,
    base: f64,
    fill: f64,
    solid: f64,
    spikes: usize,
    top: f64,
    dbh: f64,
}

fn measure(tree: &Tree) -> Measures {
    let crown: Vec<[f64; 3]> = tree
        .nodes
        .iter()
        .filter(|n| n.radius < 0.005)
        .map(|n| [n.position.x, n.position.y, n.position.z])
        .collect();
    let all = tree.nodes.iter().map(|n| n.position.y);
    let height = all.fold(f64::MIN, f64::max);
    let extent = |k: usize| {
        let (lo, hi) = crown.iter().fold((f64::MAX, f64::MIN), |(l, h), p| (l.min(p[k]), h.max(p[k])));
        hi - lo
    };
    let mut ys: Vec<f64> = crown.iter().map(|p| p[1]).collect();
    ys.sort_by(f64::total_cmp);
    let at = |q: f64| ys[((ys.len() - 1) as f64 * q) as usize];
    let cell = height / 60.0;
    let (fx, sx) = silhouette(&crown, 0, cell);
    let (fz, sz) = silhouette(&crown, 2, cell);
    let mut reach = [0.0f64; 36];
    for p in &crown {
        let s = ((p[2].atan2(p[0]) + std::f64::consts::PI) / std::f64::consts::TAU * 36.0) as usize % 36;
        reach[s] = reach[s].max(p[0].hypot(p[2]));
    }
    let mut sorted = reach;
    sorted.sort_by(f64::total_cmp);
    let edge = sorted[18];
    Measures {
        height,
        width: 0.5 * (extent(0) + extent(2)),
        base: at(0.02),
        fill: 0.5 * (fx + fz),
        solid: 0.5 * (sx + sz),
        spikes: reach.iter().filter(|&&r| r > 1.25 * edge).count(),
        top: ys[ys.len() - 1] - at(0.99),
        dbh: dbh(tree),
    }
}

/// The stem's diameter in cm at the root run's node nearest 1.3 m.
fn dbh(tree: &Tree) -> f64 {
    let stem = tree.nodes[0].branch;
    let n = tree
        .nodes
        .iter()
        .filter(|n| n.branch == stem)
        .min_by(|a, b| (a.position.y - 1.3).abs().total_cmp(&(b.position.y - 1.3).abs()))
        .unwrap();
    200.0 * n.radius
}

/// The side view along axis `k` (0: x across, 2: z across): occupied cells
/// over the row outlines' cells, and the outlines' area over their hull's.
fn silhouette(points: &[[f64; 3]], k: usize, cell: f64) -> (f64, f64) {
    use std::collections::{BTreeMap, HashSet};
    let mut cells = HashSet::new();
    for p in points {
        cells.insert(((p[k] / cell).floor() as i64, (p[1] / cell).floor() as i64));
    }
    let mut rows: BTreeMap<i64, (i64, i64)> = BTreeMap::new();
    for &(u, v) in &cells {
        let r = rows.entry(v).or_insert((u, u));
        *r = (r.0.min(u), r.1.max(u));
    }
    let outline: i64 = rows.values().map(|(a, b)| b - a + 1).sum();
    // The hull of the outline's corners, by the monotone chain.
    let mut pts: Vec<(f64, f64)> = rows
        .iter()
        .flat_map(|(&v, &(a, b))| {
            let (v, a, b) = (v as f64, a as f64, b as f64 + 1.0);
            [(a, v), (b, v), (a, v + 1.0), (b, v + 1.0)]
        })
        .collect();
    pts.sort_by(|p, q| p.partial_cmp(q).unwrap());
    let cross = |o: (f64, f64), a: (f64, f64), b: (f64, f64)| (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0);
    let mut hull: Vec<(f64, f64)> = Vec::new();
    for pass in 0..2 {
        let start = hull.len();
        let iter: Box<dyn Iterator<Item = &(f64, f64)>> =
            if pass == 0 { Box::new(pts.iter()) } else { Box::new(pts.iter().rev()) };
        for &p in iter {
            while hull.len() >= start + 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0.0 {
                hull.pop();
            }
            hull.push(p);
        }
        hull.pop();
    }
    let area: f64 = (0..hull.len())
        .map(|i| {
            let (a, b) = (hull[i], hull[(i + 1) % hull.len()]);
            a.0 * b.1 - b.0 * a.1
        })
        .sum::<f64>()
        .abs()
        / 2.0;
    (cells.len() as f64 / outline as f64, outline as f64 / area)
}
