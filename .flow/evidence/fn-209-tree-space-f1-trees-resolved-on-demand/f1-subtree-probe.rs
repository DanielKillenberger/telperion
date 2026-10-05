//! fn-209 R0 probe (evidence only, not production): the engine's trees cut
//! into chains (a seed or lateral axis and every continuation and relay that
//! carries it on), each chain's subtree size, a fly-in's resolution work by a
//! half-pixel rule, and the light at living buds with each small subtree's
//! leaves gathered at its centroid.
//!
//!   cp <this> crates/telperion-space/examples/f1_probe.rs
//!   cargo run --release -p telperion-space --example f1_probe -- [age] [seeds]
//!
//! The light lattice is the engine's (`light.rs`), rewritten here because it
//! is private to the crate; the probe checks it against `bud_light` first.
use std::time::Instant;
use telperion_space::{
    beech, bud_light, grow, oak, palm, spruce, Light, Origin, Request, Species, Structure, Vec3,
};

const BUDGET: u32 = 20_000_000;
const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};
/// 960 x 720, 45 degrees vertical: pixels per radian.
const PX_PER_RAD: f64 = 720.0 / (2.0 * 0.392_699_081_698_724_1);
const HALF_FOV: f64 = 0.49;

#[allow(dead_code)]
struct Chains {
    /// Each axis's chain (the index of the chain's first axis).
    of: Vec<usize>,
    /// Chain ids in index order, each chain's parent chain.
    ids: Vec<usize>,
    parent: Vec<Option<usize>>,
    own: Vec<usize>,
    sub: Vec<usize>,
    low: Vec<Vec3>,
    high: Vec<Vec3>,
    base: Vec<Vec3>,
}

fn chains(tree: &Structure) -> Chains {
    let n = tree.axes.len();
    let mut of = vec![0; n];
    for (i, a) in tree.axes.iter().enumerate() {
        of[i] = match a.origin {
            Origin::Seed | Origin::Lateral { .. } => i,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => of[parent],
        };
    }
    let mut parent = vec![None; n];
    let mut own = vec![0; n];
    let big = Vec3::new(1e9, 1e9, 1e9);
    let (mut low, mut high) = (vec![big; n], vec![big * -1.0; n]);
    let mut base = vec![Vec3::default(); n];
    let mut ids = Vec::new();
    for (i, a) in tree.axes.iter().enumerate() {
        let c = of[i];
        if c == i {
            ids.push(i);
            base[i] = a.base;
            parent[i] = a.origin.parent().map(|p| of[p]);
        }
        own[c] += a.phytomers.len();
        for p in a.phytomers.iter().map(|p| p.tip).chain(std::iter::once(a.base)) {
            low[c] = min(low[c], p);
            high[c] = max(high[c], p);
        }
    }
    let mut sub = own.clone();
    for &c in ids.iter().rev() {
        if let Some(p) = parent[c] {
            sub[p] += sub[c];
            low[p] = min(low[p], low[c]);
            high[p] = max(high[p], high[c]);
        }
    }
    Chains {
        of,
        ids,
        parent,
        own,
        sub,
        low,
        high,
        base,
    }
}

fn min(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
}
fn max(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
}

impl Chains {
    fn size(&self, c: usize) -> f64 {
        (self.high[c] - self.low[c]).length()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let age: u32 = args.first().map_or(80, |a| a.parse().unwrap());
    let seeds: Vec<u64> = args
        .get(1)
        .map_or(vec![1], |s| s.split(',').map(|v| v.parse().unwrap()).collect());
    let only = args.get(2).cloned();
    let all: [(&str, fn() -> Species, Light); 4] = [
        ("oak", oak, LIT),
        ("beech", beech, Light::NEUTRAL),
        ("spruce", spruce, Light::NEUTRAL),
        ("palm", palm, Light::NEUTRAL),
    ];
    for (name, make, light) in all {
        if only.as_deref().is_some_and(|o| o != name) {
            continue;
        }
        for &seed in &seeds {
            let species = make();
            let started = Instant::now();
            let request = Request {
                age,
                seed,
                budget: BUDGET,
                light,
            };
            let tree = grow(&species, request).expect("grows");
            let took = started.elapsed().as_secs_f64();
            report(name, seed, &species, &tree, took, light);
        }
    }
}

fn report(name: &str, seed: u64, species: &Species, tree: &Structure, took: f64, light: Light) {
    let ch = chains(tree);
    let phytomers: usize = tree.phytomer_count();
    let height = tree
        .axes
        .iter()
        .flat_map(|a| &a.phytomers)
        .map(|p| p.tip.z)
        .fold(0.0, f64::max);
    println!(
        "\n{name} seed {seed}: grown in {took:.2} s ({:.2} us a kept phytomer), {} axes, {} chains, {phytomers} phytomers, height {height:.2} m",
        took * 1e6 / phytomers as f64,
        tree.axes.len(),
        ch.ids.len()
    );
    // The coupling tree: chains whose subtree is larger than `x`.
    println!("  coupling tree by subtree size (chains over x: count, own phytomers, share of tree)");
    for x in [0.1, 0.25, 0.5, 1.0, 2.0, 4.0] {
        let over: Vec<usize> = ch.ids.iter().copied().filter(|&c| ch.size(c) > x).collect();
        let own: usize = over.iter().map(|&c| ch.own[c]).sum();
        println!(
            "    x {x:>4} m: {:>7} chains, {:>9} phytomers ({:.1} %)",
            over.len(),
            own,
            100.0 * own as f64 / phytomers as f64
        );
    }
    // Chain sizes: own phytomers, the largest chains.
    let mut owns: Vec<usize> = ch.ids.iter().map(|&c| ch.own[c]).collect();
    owns.sort_unstable();
    let q = |f: f64| owns[((owns.len() - 1) as f64 * f) as usize];
    println!(
        "  own phytomers a chain: median {}, p90 {}, p99 {}, max {}",
        q(0.5),
        q(0.9),
        q(0.99),
        q(1.0)
    );
    fly_in(&ch, tree, height);
    if light.extinction > 0.0 {
        gathered_light(&ch, tree, species, light);
    }
}

/// The outermost childless chain at mid-height, then a 2 s fly-in at 60 fps
/// from 40 m to 5 cm towards it: per frame, the own phytomers of chains whose
/// stand-in (taken at its subtree's box) newly projects over half a pixel
/// inside the view, every ancestor resolved first.
fn fly_in(ch: &Chains, tree: &Structure, height: f64) {
    let mut childless = vec![true; tree.axes.len()];
    for &c in &ch.ids {
        if let Some(p) = ch.parent[c] {
            childless[p] = false;
        }
    }
    let tip = |c: usize| {
        let a = tree
            .axes
            .iter()
            .enumerate()
            .filter(|(i, _)| ch.of[*i] == c)
            .last()
            .unwrap()
            .1;
        a.phytomers.last().map_or(a.base, |p| p.tip)
    };
    let target = ch
        .ids
        .iter()
        .copied()
        .filter(|&c| childless[c] && ch.own[c] > 0)
        .map(|c| (c, tip(c)))
        .filter(|(_, t)| t.z > 0.4 * height && t.z < 0.6 * height)
        .max_by(|a, b| {
            let r = |v: Vec3| (v.x * v.x + v.y * v.y).sqrt();
            r(a.1).total_cmp(&r(b.1))
        });
    let Some((_, at)) = target else {
        println!("  fly-in: no target");
        return;
    };
    let out = Vec3::new(at.x, at.y, 0.0).unit().unwrap_or(Vec3::new(1.0, 0.0, 0.0));
    let mut resolved = vec![false; tree.axes.len()];
    let (mut total, mut worst, mut first) = (0usize, (0usize, 0.0), 0usize);
    let mut frames = Vec::new();
    for k in 0..120 {
        let d = 40.0 * (0.05f64 / 40.0).powf(k as f64 / 119.0);
        let eye = at + out * d;
        let look = (at - eye).unit().unwrap();
        let mut fresh = 0;
        for &c in &ch.ids {
            if resolved[c] || ch.parent[c].is_some_and(|p| !resolved[p]) {
                continue;
            }
            let centre = (ch.low[c] + ch.high[c]) * 0.5;
            let radius = ch.size(c) / 2.0;
            let to = centre - eye;
            let dist = to.length();
            let near = (dist - radius).max(1e-3);
            let angle = (to.dot(look) / dist).clamp(-1.0, 1.0).acos();
            let spread = (radius / dist).min(1.0).asin();
            let visible = dist <= radius || angle - spread < HALF_FOV;
            if visible && ch.size(c) / near * PX_PER_RAD > 0.5 {
                resolved[c] = true;
                fresh += ch.own[c];
            }
        }
        if k == 0 {
            first = fresh;
        }
        total += fresh;
        if fresh > worst.0 {
            worst = (fresh, d);
        }
        frames.push(fresh);
    }
    let after: usize = frames[1..].iter().sum();
    let worst_after = frames[1..].iter().copied().max().unwrap_or(0);
    println!(
        "  fly-in to ({:.2}, {:.2}, {:.2}): first frame {first} phytomers; after it {after} in 119 frames, the most in one {worst_after}; in all {total} of {} ({:.1} %), worst frame {} at {:.2} m",
        at.x,
        at.y,
        at.z,
        tree.phytomer_count(),
        100.0 * total as f64 / tree.phytomer_count() as f64,
        worst.0,
        worst.1
    );
    // Whole-tree views, 40 m and 200 m away, with a stand-in whose error
    // is its whole box, half of it, or a quarter.
    for dist in [40.0, 200.0] {
        let eye = Vec3::new(dist, 0.0, height / 2.0);
        let mut line = format!("  whole tree at {dist} m, resolved by error share 1 / 0.5 / 0.25:");
        for share in [1.0, 0.5, 0.25] {
            let mut far = vec![false; tree.axes.len()];
            let mut count = 0;
            for &c in &ch.ids {
                if ch.parent[c].is_some_and(|p| !far[p]) {
                    continue;
                }
                let centre = (ch.low[c] + ch.high[c]) * 0.5;
                let near = ((centre - eye).length() - ch.size(c) / 2.0).max(1e-3);
                if share * ch.size(c) / near * PX_PER_RAD > 0.5 {
                    far[c] = true;
                    count += ch.own[c];
                }
            }
            line += &format!(" {:.1} %", 100.0 * count as f64 / tree.phytomer_count() as f64);
        }
        println!("{line}");
    }
    junctions(ch, tree, 0.5);
}

/// How far a small subtree's base girth stands from the mean of its kind
/// (its PA and birth cycle): what a stand-in of expected girth would get
/// wrong at its junction.
fn junctions(ch: &Chains, tree: &Structure, x: f64) {
    use std::collections::HashMap;
    let mut groups: HashMap<(usize, u32), Vec<f64>> = HashMap::new();
    let mut roots = Vec::new();
    for &c in &ch.ids {
        let small = ch.size(c) <= x;
        let parent_big = ch.parent[c].is_some_and(|p| ch.size(p) > x);
        let a = &tree.axes[c];
        if small && parent_big {
            if let Some(p) = a.phytomers.first() {
                let section = p.radius * p.radius;
                groups.entry((a.pa, a.birth)).or_default().push(section);
                roots.push(((a.pa, a.birth), section));
            }
        }
    }
    let mut ratios: Vec<f64> = roots
        .iter()
        .filter(|(k, _)| groups[k].len() >= 4)
        .map(|(k, s)| {
            let g = &groups[k];
            let mean = g.iter().sum::<f64>() / g.len() as f64;
            (s / mean).sqrt()
        })
        .collect();
    ratios.sort_by(f64::total_cmp);
    if ratios.is_empty() {
        return;
    }
    let q = |f: f64| ratios[((ratios.len() - 1) as f64 * f) as usize];
    println!(
        "  junctions of {} subtrees under {x} m: base radius over the radius of its kind's mean section p5 {:.2}, p25 {:.2}, p50 {:.2}, p75 {:.2}, p95 {:.2}",
        ratios.len(), q(0.05), q(0.25), q(0.5), q(0.75), q(0.95)
    );
}

/// Light at every living bud, its leaves where they grew against each small
/// subtree's leaves gathered at their area's centroid (its chain under `x`,
/// its parent's over it).
fn gathered_light(ch: &Chains, tree: &Structure, species: &Species, light: Light) {
    let leaves: Vec<(usize, Vec3, f64)> = tree
        .axes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            let area = species.states[a.pa].leaf_area;
            a.phytomers
                .iter()
                .filter(|p| p.cycle == tree.age)
                .map(move |p| (i, p.tip, p.scale * area))
        })
        .collect();
    let tips: Vec<(usize, Vec3)> = tree
        .axes
        .iter()
        .enumerate()
        .filter(|(_, a)| a.apex_end.is_none())
        .map(|(i, a)| (i, a.phytomers.last().map_or(a.base, |p| p.tip)))
        .collect();
    let reads: Vec<Vec3> = tips.iter().map(|t| t.1).collect();
    let flat: Vec<(Vec3, f64)> = leaves.iter().map(|l| (l.1, l.2)).collect();
    let exact = Field::new(&light, &flat, &reads);
    let theirs = bud_light(tree, species, &light);
    let check = theirs
        .iter()
        .zip(&tips)
        .map(|(&(_, l), &(_, at))| (l - exact.at(at)).abs())
        .fold(0.0, f64::max);
    let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    let base: Vec<f64> = reads.iter().map(|&r| exact.at(r)).collect();
    println!(
        "  light at {} living buds: mean {:.3}; the rewrite against bud_light, most apart {check:.1e}",
        reads.len(),
        mean(&base)
    );
    for x in [0.25, 0.5, 1.0, 2.0] {
        // Each axis's gathering chain: the outermost chain under x whose
        // parent is over x, or none.
        let mut top = vec![None; tree.axes.len()];
        for &c in &ch.ids {
            let small = ch.size(c) <= x;
            top[c] = match ch.parent[c] {
                Some(p) if top[p].is_some() => top[p],
                _ if small => Some(c),
                _ => None,
            };
        }
        let mut sum = vec![(Vec3::default(), 0.0); tree.axes.len()];
        for &(i, at, area) in &leaves {
            if let Some(g) = top[ch.of[i]] {
                sum[g].0 = sum[g].0 + at * area;
                sum[g].1 += area;
            }
        }
        let mut moved: Vec<(Vec3, f64)> = leaves
            .iter()
            .filter(|l| top[ch.of[l.0]].is_none())
            .map(|l| (l.1, l.2))
            .collect();
        let groups = sum.iter().filter(|s| s.1 > 0.0).count();
        moved.extend(sum.iter().filter(|s| s.1 > 0.0).map(|s| (s.0 * (1.0 / s.1), s.1)));
        let field = Field::new(&light, &moved, &reads);
        let got: Vec<f64> = reads.iter().map(|&r| field.at(r)).collect();
        let diff: Vec<f64> = got.iter().zip(&base).map(|(a, b)| (a - b).abs()).collect();
        let mut sorted = diff.clone();
        sorted.sort_by(f64::total_cmp);
        let p95 = sorted[(sorted.len() - 1) * 95 / 100];
        // A gathered subtree's own light read at its centroid, against the
        // sum over its buds.
        let mut q = vec![(0.0, 0usize); tree.axes.len()];
        for (&(i, _), &l) in tips.iter().zip(&base) {
            if let Some(g) = top[ch.of[i]] {
                q[g].0 += l;
                q[g].1 += 1;
            }
        }
        let mut errs = Vec::new();
        for (g, &(sum_l, count)) in q.iter().enumerate() {
            if count == 0 || sum[g].1 <= 0.0 {
                continue;
            }
            let at = sum[g].0 * (1.0 / sum[g].1);
            let est = count as f64 * field.at(at);
            errs.push(((est - sum_l) / sum_l).abs());
        }
        errs.sort_by(f64::total_cmp);
        // Each bud's light against its stand-in's one read at its centroid.
        let mut spread: Vec<f64> = tips
            .iter()
            .zip(&base)
            .filter_map(|(&(i, _), &l)| {
                let g = top[ch.of[i]]?;
                (sum[g].1 > 0.0).then(|| (field.at(sum[g].0 * (1.0 / sum[g].1)) - l).abs())
            })
            .collect();
        spread.sort_by(f64::total_cmp);
        let s50 = spread.get(spread.len() / 2).copied().unwrap_or(0.0);
        let s95 = spread.get((spread.len().max(1) - 1) * 95 / 100).copied().unwrap_or(0.0);
        println!("      one read a stand-in: |bud light - its read| median {s50:.4}, p95 {s95:.4}");
        let med = errs.get(errs.len() / 2).copied().unwrap_or(0.0);
        let e95 = errs.get((errs.len().max(1) - 1) * 95 / 100).copied().unwrap_or(0.0);
        println!(
            "    gathered under {x} m ({groups} stand-ins): bud light mean {:.3}, |change| mean {:.4}, p95 {p95:.4}; a stand-in's light at its centroid against its buds' sum: median {:.1} %, p95 {:.1} %",
            mean(&got),
            mean(&diff),
            100.0 * med,
            100.0 * e95
        );
    }
}

// The engine's lattice (`light.rs`), rewritten for the probe.
const CELL: f64 = 0.5;
const RINGS: [(f64, f64); 2] = [(60.0, 0.0), (30.0, 45.0)];
const BANDS: [f64; 4] = [90.0, 75.0, 45.0, 0.0];

fn directions(light: &Light) -> Vec<(Vec3, f64)> {
    let s = light.sky;
    let bands = |g: fn(f64) -> f64| {
        let g = |d: f64| g(f64::to_radians(d));
        [
            g(BANDS[0]) - g(BANDS[1]),
            g(BANDS[1]) - g(BANDS[2]),
            g(BANDS[2]) - g(BANDS[3]),
        ]
    };
    let standard = |e: f64| {
        let s = e.sin();
        (s * s / 2.0 + 2.0 * s.powi(3) / 3.0) / (7.0 / 6.0)
    };
    let uniform = |e: f64| e.sin().powi(2);
    let (zenith, st, un) = ([1.0, 0.0, 0.0], bands(standard), bands(uniform));
    let blend = |i: usize| {
        if s <= 0.5 {
            (1.0 - 2.0 * s) * zenith[i] + 2.0 * s * st[i]
        } else {
            (2.0 - 2.0 * s) * st[i] + (2.0 * s - 1.0) * un[i]
        }
    };
    let mut out = vec![(Vec3::new(0.0, 0.0, 1.0), blend(0))];
    for (ring, &(elevation, turn)) in RINGS.iter().enumerate() {
        let (e, share) = (f64::to_radians(elevation), blend(ring + 1) / 4.0);
        for k in 0..4 {
            let a = f64::to_radians(turn + 90.0 * f64::from(k));
            out.push((Vec3::new(e.cos() * a.cos(), e.cos() * a.sin(), e.sin()), share));
        }
    }
    out
}

struct Field {
    low: [i64; 3],
    size: [usize; 3],
    density: Vec<f64>,
    light: Vec<f64>,
}

impl Field {
    fn new(light: &Light, leaves: &[(Vec3, f64)], reads: &[Vec3]) -> Self {
        let (mut low, mut high) = ([i64::MAX; 3], [i64::MIN; 3]);
        for p in leaves.iter().map(|l| l.0).chain(reads.iter().copied()) {
            for (a, c) in [p.x, p.y, p.z].into_iter().enumerate() {
                let n = (c / CELL).floor() as i64;
                low[a] = low[a].min(n);
                high[a] = high[a].max(n + 1);
            }
        }
        let size = [0, 1, 2].map(|a| (high[a] - low[a] + 1) as usize);
        let cells = size[0] * size[1] * size[2];
        let mut f = Self {
            low,
            size,
            density: vec![0.0; cells],
            light: vec![1.0; cells],
        };
        for &(at, area) in leaves {
            for (node, w) in corners(at) {
                let i = f.index(node).unwrap();
                f.density[i] += w * area / CELL.powi(3);
            }
        }
        f.sweep(light.extinction, &directions(light));
        f
    }

    fn index(&self, node: [i64; 3]) -> Option<usize> {
        let mut at = [0usize; 3];
        for a in 0..3 {
            let n = node[a] - self.low[a];
            if n < 0 || n as usize >= self.size[a] {
                return None;
            }
            at[a] = n as usize;
        }
        Some((at[2] * self.size[1] + at[1]) * self.size[0] + at[0])
    }

    fn at(&self, at: Vec3) -> f64 {
        let (mut sum, mut weight) = (0.0, 0.0);
        for (node, w) in corners(at) {
            if let Some(i) = self.index(node) {
                sum += w * self.light[i];
                weight += w;
            }
        }
        sum + (1.0 - weight)
    }

    fn sweep(&mut self, k: f64, dirs: &[(Vec3, f64)]) {
        let [nx, ny, nz] = self.size;
        let layer = nx * ny;
        self.light.iter_mut().for_each(|l| *l = 0.0);
        let (mut above_d, mut depth) = (vec![0.0; layer], vec![0.0; layer]);
        for &(toward, share) in dirs.iter().filter(|d| d.1 > 0.0) {
            let (ox, oy) = (toward.x / toward.z, toward.y / toward.z);
            let path = CELL / toward.z;
            above_d.iter_mut().for_each(|d| *d = 0.0);
            for z in (0..nz).rev() {
                for j in 0..ny {
                    for i in 0..nx {
                        let (x, y) = (i as f64 + ox, j as f64 + oy);
                        let (d0, r0) = if z + 1 < nz {
                            let a = &self.density[(z + 1) * layer..(z + 2) * layer];
                            (bilinear(&above_d, nx, ny, x, y), bilinear(a, nx, ny, x, y))
                        } else {
                            (0.0, 0.0)
                        };
                        let rho = self.density[z * layer + j * nx + i];
                        depth[j * nx + i] = d0 + k * path * (rho + r0) / 2.0;
                    }
                }
                for (l, &d) in self.light[z * layer..(z + 1) * layer].iter_mut().zip(&depth) {
                    *l += share * (-d).exp();
                }
                std::mem::swap(&mut above_d, &mut depth);
            }
        }
    }
}

fn bilinear(layer: &[f64], nx: usize, ny: usize, x: f64, y: f64) -> f64 {
    let (fx, fy) = (x.floor(), y.floor());
    let (tx, ty) = (x - fx, y - fy);
    let mut sum = 0.0;
    for (dx, wx) in [(0, 1.0 - tx), (1, tx)] {
        for (dy, wy) in [(0, 1.0 - ty), (1, ty)] {
            let (i, j) = (fx as i64 + dx, fy as i64 + dy);
            if i >= 0 && j >= 0 && (i as usize) < nx && (j as usize) < ny {
                sum += wx * wy * layer[j as usize * nx + i as usize];
            }
        }
    }
    sum
}

fn corners(at: Vec3) -> impl Iterator<Item = ([i64; 3], f64)> {
    let f = [at.x / CELL, at.y / CELL, at.z / CELL];
    let n = f.map(f64::floor);
    let t = [f[0] - n[0], f[1] - n[1], f[2] - n[2]];
    (0..8).map(move |c: usize| {
        let mut node = [0i64; 3];
        let mut w = 1.0;
        for a in 0..3 {
            let up = (c >> a) & 1;
            node[a] = n[a] as i64 + up as i64;
            w *= if up == 1 { t[a] } else { 1.0 - t[a] };
        }
        (node, w)
    })
}
