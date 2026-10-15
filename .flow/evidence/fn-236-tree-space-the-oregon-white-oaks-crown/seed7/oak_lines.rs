//! fn-236 seed-7 probe, not product code: the oak's limbs as the engine
//! grew them. Copied into `crates/telperion-space/examples/` for a run.
//!
//!   oak_lines <age> <seeds, comma-separated> [limbs]
//!
//! OAK_VALUES names another value file (a probe's rows); OAK_LINES_POINTS
//! adds each line's points.
//!
//! A stem is the seed's axis and every axis carried on from a stem
//! (continuation or relay); a limb is a lateral off a stem. A limb's line
//! is the limb and the axes it carries on through, relays standing at
//! their node of the axis they replace. One JSON line per tree, then one
//! per limb (the `limbs` largest by subtree phytomers, default 8).
use telperion_core::presets::values;
use telperion_space::{grow, Origin, Request, Structure, Vec3};

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let age: u32 = a[0].parse().unwrap();
    let keep: usize = a.get(2).map_or(8, |v| v.parse().unwrap());
    let path = std::env::var("OAK_VALUES")
        .unwrap_or_else(|_| "crates/telperion-core/presets/oregon-white-oak.values".into());
    let f = values::read(&std::fs::read_to_string(path).unwrap()).unwrap();
    let sp = f.space.species.clone().unwrap();
    for seed in a[1].split(',').map(|v| v.parse::<u64>().unwrap()) {
        let r = Request { age, seed, budget: 20_000_000, light: f.space.light };
        match grow(&sp, r) {
            Ok(s) => report(&s, seed, keep),
            Err(e) => println!("{{\"seed\":{seed},\"error\":\"{e}\"}}"),
        }
    }
}

fn reach(p: Vec3) -> f64 {
    p.x.hypot(p.y)
}

fn report(s: &Structure, seed: u64, keep: usize) {
    let n = s.axes.len();
    // Children by kind: carried on (continuation, relay) and laterals.
    let mut carried: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    let mut all: Vec<Vec<usize>> = vec![vec![]; n];
    let mut stem = vec![false; n];
    for (i, x) in s.axes.iter().enumerate() {
        match x.origin {
            Origin::Seed => stem[i] = true,
            Origin::Continuation { parent } => {
                carried[parent].push((i, usize::MAX));
                all[parent].push(i);
            }
            Origin::Relay { parent, node } => {
                carried[parent].push((i, node));
                all[parent].push(i);
            }
            Origin::Lateral { parent, .. } => all[parent].push(i),
        }
    }
    // Axes are in lineage order, parents first, so one pass marks stems.
    for i in 0..n {
        if let Origin::Continuation { parent } | Origin::Relay { parent, .. } = s.axes[i].origin {
            stem[i] = stem[i] || stem[parent];
        }
    }
    let tips: Vec<Vec3> = s.axes.iter().flat_map(|x| x.phytomers.iter().map(|p| p.tip)).collect();
    let height = tips.iter().map(|p| p.z).fold(0.0, f64::max);
    let mut r: Vec<f64> = tips.iter().map(|&p| reach(p)).collect();
    r.sort_by(f64::total_cmp);
    let crown = r[r.len() * 95 / 100];
    println!(
        "{{\"seed\":{seed},\"tree\":true,\"height\":{height:.2},\"crown95\":{crown:.2},\"crownmax\":{:.2},\"phytomers\":{}}}",
        r.last().unwrap(),
        tips.len()
    );
    // Subtree phytomer counts and reach, children after parents.
    let mut count = vec![0usize; n];
    let mut far = vec![0.0f64; n];
    let mut low = vec![f64::MAX; n];
    for i in (0..n).rev() {
        count[i] += s.axes[i].phytomers.len();
        for p in &s.axes[i].phytomers {
            far[i] = far[i].max(reach(p.tip));
            low[i] = low[i].min(p.tip.z);
        }
        for &c in &all[i] {
            count[i] += count[c];
            far[i] = far[i].max(far[c]);
            low[i] = low[i].min(low[c]);
        }
    }
    let mut limbs: Vec<usize> = (0..n)
        .filter(|&i| matches!(s.axes[i].origin, Origin::Lateral { parent, .. } if stem[parent]))
        .collect();
    limbs.sort_by_key(|&i| std::cmp::Reverse(count[i]));
    for &l in limbs.iter().take(keep) {
        line(s, l, &carried, seed, height, crown, count[l], far[l], low[l]);
    }
}

#[allow(clippy::too_many_arguments)]
fn line(
    s: &Structure,
    first: usize,
    carried: &[Vec<(usize, usize)>],
    seed: u64,
    height: f64,
    crown: f64,
    count: usize,
    far: f64,
    low: f64,
) {
    // The line: each axis's phytomers up to where the next carries on.
    let mut pts = vec![s.axes[first].base];
    let mut joins = vec![]; // (arc index of join, kind, angle at join)
    let mut axis = first;
    let mut pas = vec![s.axes[first].pa];
    loop {
        let x = &s.axes[axis];
        // The carried-on child that stands farthest along (a relay at its
        // node, a continuation at the tip).
        let next = carried[axis].iter().max_by_key(|(_, node)| *node).copied();
        let upto = match next {
            Some((_, usize::MAX)) | None => x.phytomers.len(),
            Some((_, node)) => (node + 1).min(x.phytomers.len()),
        };
        for p in &x.phytomers[..upto] {
            pts.push(p.tip);
        }
        let Some((c, node)) = next else { break };
        let before = x.phytomers[..upto].last().map(|p| p.heading).unwrap_or(x.heading);
        let after = s.axes[c].heading;
        let ang = before.dot(after).clamp(-1.0, 1.0).acos().to_degrees();
        let kind = if node == usize::MAX { "continuation" } else { "relay" };
        joins.push((pts.len() - 1, kind, ang, x.phytomers.len() - upto));
        if node != usize::MAX {
            pts.push(s.axes[c].base);
        }
        pas.push(s.axes[c].pa);
        axis = c;
    }
    // Arc length along the line, and turns between half-metre chords.
    let mut arc = vec![0.0];
    for w in pts.windows(2) {
        arc.push(arc.last().unwrap() + (w[1] - w[0]).length());
    }
    let len = *arc.last().unwrap();
    let at = |d: f64| {
        let k = arc.partition_point(|&v| v < d).min(pts.len() - 1);
        pts[k]
    };
    let (step, chord) = (0.25, 0.5);
    let mut kinks = vec![];
    let mut d = chord;
    while d + chord <= len {
        let (a, b, c) = (at(d - chord), at(d), at(d + chord));
        let (u, v) = (b - a, c - b);
        if u.length() > 1e-6 && v.length() > 1e-6 {
            let ang = (u.dot(v) / (u.length() * v.length())).clamp(-1.0, 1.0).acos().to_degrees();
            if ang > 45.0 {
                kinks.push((d, ang));
            }
        }
        d += step;
    }
    // Collapse neighbouring samples into one kink, its largest turn.
    let mut merged: Vec<(f64, f64)> = vec![];
    for (d, ang) in kinks {
        match merged.last_mut() {
            Some(m) if d - m.0 <= chord + 1e-9 => {
                if ang > m.1 {
                    *m = (d, ang)
                }
            }
            _ => merged.push((d, ang)),
        }
    }
    if std::env::var_os("OAK_LINES_POINTS").is_some() {
        let p: Vec<String> = pts.iter().map(|p| format!("[{:.3},{:.3},{:.3}]", p.x, p.y, p.z)).collect();
        println!("{{\"seed\":{seed},\"axis\":{first},\"points\":[{}]}}", p.join(","));
    }
    let base = s.axes[first].base;
    let tip = *pts.last().unwrap();
    let dip = pts.iter().map(|p| p.z).fold(f64::MAX, f64::min) - base.z;
    let joins: Vec<String> = joins
        .iter()
        .map(|(k, kind, ang, left)| {
            format!(
                "{{\"at\":{:.2},\"kind\":\"{kind}\",\"turn\":{ang:.0},\"left\":{left}}}",
                arc[*k]
            )
        })
        .collect();
    let kinks: Vec<String> = merged.iter().map(|(d, a)| format!("[{d:.2},{a:.0}]")).collect();
    let r0 = s.axes[first].phytomers.first().map_or(0.0, |p| p.radius);
    println!(
        "{{\"seed\":{seed},\"axis\":{first},\"born\":{},\"woken\":{},\"pa\":{:?},\"z\":{:.2},\"zshare\":{:.2},\"r0\":{r0:.3},\"len\":{len:.2},\"tipreach\":{:.2},\"far\":{far:.2},\"farshare\":{:.2},\"low\":{:.2},\"dip\":{dip:.2},\"tipz\":{:.2},\"phytomers\":{count},\"joins\":[{}],\"kinks\":[{}]}}",
        s.axes[first].birth,
        matches!(s.axes[first].origin, Origin::Lateral { woken: true, .. }),
        pas,
        base.z,
        base.z / height,
        reach(tip),
        far / crown,
        low,
        tip.z,
        joins.join(","),
        kinks.join(",")
    );
}
