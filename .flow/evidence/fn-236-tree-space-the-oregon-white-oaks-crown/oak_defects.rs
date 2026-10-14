//! fn-236 probe, not product code: the owner's three oak defects of
//! 2026-10-14 as numbers, and close-up stills of the fork and a limb.
//! Copied into `crates/telperion-render/examples/oak_defects.rs` by
//! `defects.sh` for a run and removed after.
//!
//!   oak_defects <out dir> <ages> <seeds> [<path>=<value>...]
//!
//! The family is the preset's value file on disk with each `<path>=<value>`
//! row replaced or added (as `oak_views.rs`), so a probe needs no rebuild.
//! With `OAK_NO_STILLS` set no GPU is used.
//!
//! - (a, b) leaves: the planned leaves by the age of the wood they are
//!   planned on (0, 1 to 3, 4 and more years; the engine phytomer that drew
//!   the node, as `twig_layer.rs`) and by its radius; the placed leaves,
//!   their one-sided area, and that area over the leaves' ground hull.
//! - (c) the fork: the stem's radius node by node around its largest
//!   drop, and each child leaving there (start radius, codominance, angle).
//! - (c) crossings: pairs of structural segments (radius at least 5 % of
//!   the root's) on different runs whose axes pass closer than the sum of
//!   their radii, apart from pairs that share a node. A pair whose nearest
//!   points both lie within twice the larger radius of their runs' common
//!   node is a junction (the overlap of two shells leaving one node);
//!   every other is a crossing. Depth is the overlap over the thinner
//!   radius.
//! - Stills: `<out>/oak-<age>-<seed>-fork-{bare,whole}-<az>.png` at the
//!   fork from four azimuths, and `-limb-{bare,whole}.png` on the middle of
//!   the longest limb.
use std::collections::HashMap;
use telperion_core::{
    math::Vec3,
    pipeline::{self, executor, Request},
    presets::values,
    tree::Tree,
    Family,
};
use telperion_render::{render, write_png, Camera, Gpu, Level, Renderer, SceneRow, View, STILL_FORMAT};

const FILE: &str = "crates/telperion-core/presets/oregon-white-oak.values";
const SIZE: (u32, u32) = (960, 720);
const SUN: (f64, f64) = (115.0, 60.0);
const STRUCTURAL: f64 = 0.05;

fn family(sets: &[String]) -> Family {
    let mut text = std::fs::read_to_string(FILE).expect("the preset's value file");
    for set in sets {
        let (path, value) = set.split_once('=').expect("<path>=<value>");
        let line = format!("{path} = {value}");
        let mut lines: Vec<String> = text.lines().map(String::from).collect();
        match lines.iter().position(|l| l.split('=').next().map(str::trim) == Some(path)) {
            Some(i) => lines[i] = line,
            None => lines.push(line),
        }
        text = lines.join("\n") + "\n";
    }
    values::read(&text).expect("the value file reads")
}

/// Each node's wood age in years, by the engine phytomer that drew it.
fn ages(family: &Family, tree: &Tree) -> Result<Vec<u32>, String> {
    let applied = family.applied();
    let species = applied.space.species.as_ref().ok_or("no engine species")?;
    let request = telperion_space::Request {
        age: family.age as u32,
        seed: u64::from(family.skeleton.seed),
        budget: 20_000_000,
        light: applied.space.light,
    };
    let s = telperion_space::grow(species, request).map_err(|e| e.to_string())?;
    let cycle: HashMap<u64, u32> = (s.axes.iter())
        .flat_map(|a| a.phytomers.iter().map(|p| (p.key(), p.cycle)))
        .collect();
    Ok((tree.nodes.iter())
        .map(|n| {
            let c = cycle.get(&n.identity.birth_order()).copied();
            c.map_or(u32::MAX, |c| s.age.saturating_sub(c))
        })
        .collect())
}

/// (a, b): planned leaves by the age and radius of their wood.
fn leaves_by_wood(f: &Family) -> Result<String, String> {
    let request = Request { leaves: true, field: Some(None), ..Request::default() };
    let built = pipeline::build(f, request).map_err(|e| e.to_string())?;
    let tree = &built.skeleton.tree;
    let plan = built.outputs.plan.as_ref().ok_or("no leaf plan")?;
    let age = ages(f, tree)?;
    let at: HashMap<[u64; 3], usize> = (tree.nodes.iter().enumerate())
        .map(|(i, n)| ([n.position.x.to_bits(), n.position.y.to_bits(), n.position.z.to_bits()], i))
        .collect();
    let (mut by_age, mut by_r) = ([0u64; 4], [0u64; 4]);
    for d in &plan.descriptors {
        let p = d.endpoints[1];
        let a = match at.get(&[p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]) {
            Some(&i) if age[i] != u32::MAX => match age[i] {
                0 => 0,
                1..=3 => 1,
                _ => 2,
            },
            _ => 3,
        };
        by_age[a] += u64::from(d.count);
        let mm = 1e3 * 0.5 * (d.radii[0] + d.radii[1]);
        by_r[[2.0, 5.0, 10.0].iter().position(|&r| mm <= r).unwrap_or(3)] += u64::from(d.count);
    }
    let t = by_age.iter().sum::<u64>().max(1) as f64;
    let pc = |v: u64| 100.0 * v as f64 / t;
    Ok(format!(
        "\"planned\":{},\"wood_age_0\":{},\"wood_age_1_3\":{},\"wood_age_4p\":{},\"wood_age_unknown\":{},\"pct_age_0\":{:.1},\"pct_age_4p\":{:.1},\"r_le2mm\":{},\"r_2_5mm\":{},\"r_5_10mm\":{},\"r_gt10mm\":{},\"bearing_limit_mm\":{:.1}",
        plan.total, by_age[0], by_age[1], by_age[2], by_age[3], pc(by_age[0]), pc(by_age[2]),
        by_r[0], by_r[1], by_r[2], by_r[3],
        1e3 * tree.nodes[0].radius * f.canopy.shoot_radius
    ))
}

/// The placed leaves' one-sided area and its ratio to their ground hull.
fn leaf_area(mesh: &telperion_core::mesh::TreeMesh, x: &executor::Expansion) -> (f64, f64) {
    let element = x.element();
    let p = &element.positions;
    let blade: f64 = (element.indices.chunks(3))
        .map(|t| {
            let [a, b, c] = [0, 1, 2].map(|k| p[t[k] as usize]);
            let (u, v) = (b - a, c - a);
            (0.5 * (u.x * v.y - u.y * v.x)).max(0.0)
        })
        .sum();
    let l = &mesh.foliage.instances;
    let area: f64 = l.leaves.iter().map(|q| blade * l.reference.scale(*q).powi(2)).sum();
    let seats: Vec<(f64, f64)> = (0..l.len()).map(|k| (l.position(k).x, l.position(k).z)).collect();
    (area, area / hull_area(seats).max(1e-9))
}

fn hull_area(mut pts: Vec<(f64, f64)>) -> f64 {
    pts.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    pts.dedup();
    if pts.len() < 3 {
        return 0.0;
    }
    let cross = |o: (f64, f64), a: (f64, f64), b: (f64, f64)| (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0);
    let mut hull: Vec<(f64, f64)> = Vec::new();
    for pass in 0..2 {
        let start = hull.len();
        let order: Vec<_> = if pass == 0 { pts.clone() } else { pts.iter().rev().copied().collect() };
        for q in order {
            while hull.len() >= start + 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], q) <= 0.0 {
                hull.pop();
            }
            hull.push(q);
        }
        hull.pop();
    }
    let n = hull.len();
    (0..n).map(|i| hull[i].0 * hull[(i + 1) % n].1 - hull[(i + 1) % n].0 * hull[i].1).sum::<f64>().abs() / 2.0
}

/// (c): the stem's largest radius drop between consecutive stem nodes, the
/// profile around it, and the children that leave its upper node.
fn fork(tree: &Tree) -> (usize, String) {
    let n = &tree.nodes;
    let mut kids: Vec<Vec<usize>> = vec![Vec::new(); n.len()];
    for (i, m) in n.iter().enumerate().skip(1) {
        kids[m.parent.unwrap() as usize].push(i);
    }
    // The stem's run from the root: through each node's stem child, the
    // widest where several are.
    let mut path = vec![0usize];
    while let Some(&next) = kids[*path.last().unwrap()]
        .iter()
        .filter(|&&k| n[k].stem)
        .max_by(|&&a, &&b| n[a].start_radius.total_cmp(&n[b].start_radius))
    {
        path.push(next);
    }
    let (mut worst, mut at) = (1.0, 1);
    for k in 1..path.len() - 1 {
        let ratio = n[path[k + 1]].radius / n[path[k]].radius;
        if ratio < worst {
            (worst, at) = (ratio, k);
        }
    }
    let f = path[at];
    let mut out = format!(
        "\"fork_node\":{f},\"fork_height\":{:.2},\"stem_drop_ratio\":{worst:.3},\"profile\":[",
        n[f].position.y
    );
    let lo = at.saturating_sub(3);
    let hi = (at + 4).min(path.len());
    let profile: Vec<String> = path[lo..hi]
        .iter()
        .map(|&i| format!("[{:.2},{:.4}]", n[i].position.y, n[i].radius))
        .collect();
    out += &profile.join(",");
    out += "],\"children\":[";
    let axis = (n[f].position - n[n[f].parent.unwrap() as usize].position).normalized();
    let children: Vec<String> = kids[f]
        .iter()
        .map(|&k| {
            let d = (n[k].position - n[f].position).normalized();
            format!(
                "{{\"start_r\":{:.4},\"r\":{:.4},\"codominant\":{},\"stem\":{},\"deg\":{:.0}}}",
                n[k].start_radius,
                n[k].radius,
                n[k].codominant.map_or(-1.0, |w| w),
                n[k].stem,
                d.dot(axis).clamp(-1.0, 1.0).acos().to_degrees()
            )
        })
        .collect();
    out += &children.join(",");
    out += "]";
    (f, out)
}

/// (c): along the stem's run, every node where the drawn girth (the
/// node's radius, eased over twice the trunk's radius past a node a
/// codominant child leaves, as `surface/samples.rs` `girths` draws it)
/// falls by more than 7 % from the node below: its height, the girths,
/// and the children leaving the node below with their share of its radius
/// and whether any is codominant.
fn stem_steps(tree: &Tree) -> String {
    let n = &tree.nodes;
    let mut kids: Vec<Vec<usize>> = vec![Vec::new(); n.len()];
    for (i, m) in n.iter().enumerate().skip(1) {
        kids[m.parent.unwrap() as usize].push(i);
    }
    let mut path = vec![0usize];
    while let Some(&next) = kids[*path.last().unwrap()]
        .iter()
        .filter(|&&k| n[k].stem && n[k].codominant.is_none())
        .max_by(|&&a, &&b| n[a].start_radius.total_cmp(&n[b].start_radius))
    {
        path.push(next);
    }
    let mut along = 0.0;
    let (mut fork, mut prev): (Option<(usize, f64)>, f64) = (None, n[0].radius);
    let mut steps = Vec::new();
    for (k, &i) in path.iter().enumerate() {
        if k > 0 {
            along += n[i].position.distance(n[path[k - 1]].position);
        }
        let own = n[i].radius;
        let girth = fork.map_or(own, |(f, d): (usize, f64)| {
            let trunk = n[f].radius;
            let t = (along - d) / (2.0 * trunk).max(1e-9);
            let held = if t < 1.0 { (1.0 - t) * (1.0 - t) * (1.0 + 2.0 * t) } else { 0.0 };
            own + (trunk - own).max(0.0) * held
        });
        if k > 0 && girth < 0.93 * prev {
            let below = path[k - 1];
            let laterals: Vec<String> = kids[below]
                .iter()
                .filter(|&&c| c != i)
                .map(|&c| format!("{:.2}{}", n[c].start_radius / n[below].radius, if n[c].codominant.is_some() { "c" } else { "" }))
                .collect();
            steps.push(format!("[{:.2},{:.4},{:.4},\"{}\"]", n[i].position.y, prev, girth, laterals.join(" ")));
        }
        if k > 0 && kids[i].iter().any(|&c| n[c].codominant.is_some()) {
            fork = Some((i, along));
        }
        prev = girth;
    }
    format!("\"stem_steps\":[{}]", steps.join(","))
}

/// (c): pairs of structural segments on different runs that overlap.
fn crossings(tree: &Tree) -> String {
    let n = &tree.nodes;
    let floor = STRUCTURAL * n[0].radius;
    let segs: Vec<usize> = (1..n.len()).filter(|&i| n[i].radius >= floor).collect();
    let depth: Vec<u32> = {
        let mut d = vec![0u32; n.len()];
        for i in 1..n.len() {
            d[i] = d[n[i].parent.unwrap() as usize] + 1;
        }
        d
    };
    let lca = |mut a: usize, mut b: usize| {
        while depth[a] > depth[b] {
            a = n[a].parent.unwrap() as usize;
        }
        while depth[b] > depth[a] {
            b = n[b].parent.unwrap() as usize;
        }
        while a != b {
            a = n[a].parent.unwrap() as usize;
            b = n[b].parent.unwrap() as usize;
        }
        a
    };
    let cell = 0.5;
    let key = |p: Vec3| ((p.x / cell).floor() as i32, (p.y / cell).floor() as i32, (p.z / cell).floor() as i32);
    let mut grid: HashMap<(i32, i32, i32), Vec<usize>> = HashMap::new();
    for (s, &i) in segs.iter().enumerate() {
        let (a, b) = (n[n[i].parent.unwrap() as usize].position, n[i].position);
        let r = n[i].radius.max(n[i].start_radius);
        let lo = key(Vec3::new(a.x.min(b.x) - r, a.y.min(b.y) - r, a.z.min(b.z) - r));
        let hi = key(Vec3::new(a.x.max(b.x) + r, a.y.max(b.y) + r, a.z.max(b.z) + r));
        for x in lo.0..=hi.0 {
            for y in lo.1..=hi.1 {
                for z in lo.2..=hi.2 {
                    grid.entry((x, y, z)).or_default().push(s);
                }
            }
        }
    }
    let mut seen = std::collections::HashSet::new();
    let (mut junction, mut cross, mut deepest) = (0usize, Vec::new(), 0.0f64);
    let mut worst = String::new();
    for list in grid.values() {
        for (x, &s) in list.iter().enumerate() {
            for &t in &list[x + 1..] {
                let (i, j) = (segs[s].min(segs[t]), segs[s].max(segs[t]));
                let (pi, pj) = (n[i].parent.unwrap() as usize, n[j].parent.unwrap() as usize);
                if n[i].branch == n[j].branch || pi == pj || pi == j || pj == i || !seen.insert((i, j)) {
                    continue;
                }
                let (ri, rj) = (n[i].radius, n[j].radius);
                let (ci, cj, dist) = closest(n[pi].position, n[i].position, n[pj].position, n[j].position);
                let overlap = ri + rj - dist;
                if overlap <= 0.0 {
                    continue;
                }
                let c = lca(i, j);
                let reach = 2.0 * n[c].radius.max(ri).max(rj);
                if ci.distance(n[c].position) <= reach && cj.distance(n[c].position) <= reach {
                    junction += 1;
                    continue;
                }
                let d = overlap / ri.min(rj);
                if d > deepest {
                    deepest = d;
                    let m = (ci + cj) * 0.5;
                    worst = format!("[{:.2},{:.2},{:.2}],\"worst_radii\":[{ri:.3},{rj:.3}]", m.x, m.y, m.z);
                }
                cross.push(d);
            }
        }
    }
    cross.sort_by(f64::total_cmp);
    let q = |p: f64| cross.get(((cross.len().max(1) - 1) as f64 * p) as usize).copied().unwrap_or(0.0);
    format!(
        "\"structural_segments\":{},\"junction_pairs\":{junction},\"crossing_pairs\":{},\"crossing_through\":{},\"depth_median\":{:.2},\"depth_max\":{deepest:.2},\"worst_at\":{}",
        segs.len(),
        cross.len(),
        cross.iter().filter(|&&d| d >= 1.0).count(),
        q(0.5),
        if worst.is_empty() { "null".into() } else { worst }
    )
}

/// The nearest points of segments ab and cd, and their distance.
fn closest(a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> (Vec3, Vec3, f64) {
    let (u, v, w) = (b - a, d - c, a - c);
    let (aa, bb, cc, dd, ee) = (u.dot(u), u.dot(v), v.dot(v), u.dot(w), v.dot(w));
    let den = aa * cc - bb * bb;
    let mut s = if den > 1e-12 { ((bb * ee - cc * dd) / den).clamp(0.0, 1.0) } else { 0.0 };
    let mut t = if cc > 1e-12 { ((bb * s + ee) / cc).clamp(0.0, 1.0) } else { 0.0 };
    if aa > 1e-12 {
        s = ((bb * t - dd) / aa).clamp(0.0, 1.0);
    }
    if cc > 1e-12 {
        t = ((bb * s + ee) / cc).clamp(0.0, 1.0);
    }
    let (p, q) = (a + u * s, c + v * t);
    (p, q, p.distance(q))
}

/// The middle of the longest run that leaves the fork node.
fn limb_middle(tree: &Tree, f: usize) -> Vec3 {
    let n = &tree.nodes;
    let mut best = (0.0, n[f].position);
    for (i, m) in n.iter().enumerate().skip(1) {
        if m.parent == Some(f as u32) && !m.stem {
            let run: Vec<usize> = (i..n.len()).filter(|&k| n[k].branch == m.branch).collect();
            let length: f64 = run.iter().map(|&k| tree.grown_length(k)).sum();
            if length > best.0 {
                best = (length, n[run[run.len() / 2]].position);
            }
        }
    }
    best.1
}

fn shoot(r: &mut Renderer, mesh: &telperion_core::mesh::TreeMesh, f: &Family, cam: &Camera, path: String) -> Result<(), String> {
    for (view, tag) in [(View::Bare, "bare"), (View::Whole, "whole")] {
        r.submit_at(mesh, Level::Chosen).map_err(|e| e.to_string())?;
        r.set_family(f);
        r.set_view(view);
        r.set_scene(SceneRow { sun_azimuth: SUN.0, sun_elevation: SUN.1, ..SceneRow::default() });
        let still = render(r, cam, SIZE.0, SIZE.1).map_err(|e| e.to_string())?;
        write_png(std::path::Path::new(&path.replace("{v}", tag)), &still).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// A camera `distance` off `target` horizontally at `azimuth`, at eye
/// height (1.7 m), looking at it.
fn eye(target: Vec3, azimuth: f64, distance: f64) -> Camera {
    let a = azimuth.to_radians();
    Camera {
        position: Vec3::new(target.x + a.cos() * distance, 1.7, target.z + a.sin() * distance),
        target,
        field_of_view: 50.0,
        near: 0.02,
        far: 200.0,
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let list = |i: usize| -> Vec<u32> { args[i].split(',').map(|v| v.parse().unwrap()).collect() };
    let (out, base) = (&args[0], family(&args[3..]));
    let mut renderer = match std::env::var_os("OAK_NO_STILLS") {
        Some(_) => None,
        None => {
            let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
            Some(Renderer::new(gpu, STILL_FORMAT))
        }
    };
    for &age in &list(1) {
        for &seed in &list(2) {
            let mut f = base.clone();
            f.age = f64::from(age);
            f.skeleton.seed = seed;
            let x = executor::grow(&f).map_err(|e| format!("{age} {seed}: {e}"))?;
            let x = x.expansion().map_err(|e| e.to_string())?;
            let mesh = x.mesh().map_err(|e| e.to_string())?;
            let tree = x.tree();
            let (area, index) = leaf_area(&mesh, &x);
            let (f_node, fork_json) = fork(tree);
            println!(
                "{{\"age\":{age},\"seed\":{seed},\"nodes\":{},\"leaves\":{},\"leaf_m2\":{area:.0},\"leaf_over_hull\":{index:.1},\"root_r\":{:.3},{},{},{},{}}}",
                tree.nodes.len(),
                mesh.foliage_instances(),
                tree.nodes[0].radius,
                leaves_by_wood(&f)?,
                fork_json,
                crossings(tree),
                stem_steps(tree)
            );
            if let Some(r) = renderer.as_mut() {
                // From eye height a few metres off, as the owner stood in
                // the harness (owner-2026-10-14-fork.png).
                let at = tree.nodes[f_node].position;
                for az in [0.0, 90.0, 180.0, 270.0] {
                    let path = format!("{out}/oak-{age}-{seed}-fork-{{v}}-{az:.0}.png");
                    shoot(r, &mesh, &f, &eye(at, az, 3.0), path)?;
                }
                // Level with the fork, 1.6 m off, the trunk's top filling the frame.
                for az in [0.0, 90.0, 180.0, 270.0] {
                    let mut cam = eye(at, az, 1.6);
                    cam.position.y = at.y - 0.4;
                    let path = format!("{out}/oak-{age}-{seed}-side-{{v}}-{az:.0}.png");
                    shoot(r, &mesh, &f, &cam, path)?;
                }
                let limb = limb_middle(tree, f_node);
                let path = format!("{out}/oak-{age}-{seed}-limb-{{v}}.png");
                shoot(r, &mesh, &f, &eye(limb, 30.0, 3.5), path)?;
            }
        }
    }
    Ok(())
}
