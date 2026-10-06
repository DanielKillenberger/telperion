//! fn-209 F1, FINE-CONTRACT.md: how far a change to a decision's
//! representation moves the four species' trees. `dump` writes every
//! tree's axes by lineage (identity, phytomers, tips, radii); `compare`
//! reads two dumps and prints, per tree, the axes and phytomers one has
//! that the other lacks, and how far the shared phytomers' tips moved.
//!
//!   cargo run --profile ci -p telperion-space --example contract -- dump <file> [ages] [seeds]
//!   cargo run --profile ci -p telperion-space --example contract -- compare <a> <b>
//!
//! Self-contained (it computes identities itself), so it builds on any
//! commit the engine's public API has not changed on.
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Read, Write};
use telperion_space::{
    beech, grow, grow_alone, oak, palm, spruce, Expected, Light, Origin, Request, Species,
    Structure,
};

type Tree = HashMap<u64, Vec<[f64; 4]>>;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("dump") => {
            let list = |i: usize, d: &str| -> Vec<u64> {
                let text = args.get(i).map_or(d, String::as_str);
                text.split(',')
                    .map(|v| v.parse().expect("a number"))
                    .collect()
            };
            dump(&args[1], &list(2, "40,80"), &list(3, "1,7"));
        }
        Some("compare") => compare(&args[1], &args[2]),
        _ => eprintln!("dump <file> [ages] [seeds] | compare <a> <b>"),
    }
}

fn identities(s: &Structure) -> Vec<u64> {
    let mix = |mut z: u64| {
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    };
    let step = |h: u64, v: u64| mix(h.rotate_left(23) ^ mix(v.wrapping_add(0x6a09_e667)));
    let mut ids = vec![0u64; s.axes.len()];
    for (i, a) in s.axes.iter().enumerate() {
        ids[i] = match a.origin {
            Origin::Seed => step(1, a.pa as u64),
            Origin::Lateral {
                parent,
                node,
                slot,
                woken,
                ..
            } => step(
                step(step(step(ids[parent], 1), node as u64), u64::from(slot)),
                woken as u64,
            ),
            Origin::Continuation { parent } => step(step(ids[parent], 2), a.pa as u64),
            Origin::Relay { parent, .. } => step(step(ids[parent], 3), a.pa as u64),
        };
    }
    ids
}

fn put(w: &mut impl Write, v: u64) {
    w.write_all(&v.to_le_bytes()).unwrap();
}

fn dump(path: &str, ages: &[u64], seeds: &[u64]) {
    let mut w = BufWriter::new(std::fs::File::create(path).unwrap());
    let all: [(&str, fn() -> Species, bool); 4] = [
        ("beech", beech, false),
        ("spruce", spruce, false),
        ("oak", oak, true),
        ("palm", palm, false),
    ];
    for (name, make, lit) in all {
        let species = make();
        for &age in ages {
            let expected = (!lit).then(|| Expected::new(&species, age as u32, 0));
            for &seed in seeds {
                let light = if lit {
                    Light {
                        extinction: 0.5,
                        sky: 0.5,
                    }
                } else {
                    Light::NEUTRAL
                };
                let request = Request {
                    age: age as u32,
                    seed,
                    budget: 20_000_000,
                    light,
                };
                let t = std::time::Instant::now();
                let tree = grow(&species, request).expect("grows");
                write(&mut w, &format!("{name} {age} {seed} whole"), &tree);
                let mut done = vec![("whole", t.elapsed().as_secs_f64())];
                if let Some(expected) = &expected {
                    let t = std::time::Instant::now();
                    let mut alone = grow_alone(&species, request, expected).expect("alone");
                    alone.bounded.resolve_all(0).expect("resolves");
                    let label = format!("{name} {age} {seed} alone");
                    write(&mut w, &label, alone.bounded.structure());
                    done.push(("alone", t.elapsed().as_secs_f64()));
                }
                eprintln!("{name} {age} {seed}: {done:?}");
            }
        }
    }
}

fn write(w: &mut impl Write, label: &str, tree: &Structure) {
    let bytes = label.as_bytes();
    put(w, bytes.len() as u64);
    w.write_all(bytes).unwrap();
    let ids = identities(tree);
    put(w, tree.axes.len() as u64);
    for (a, id) in tree.axes.iter().zip(ids) {
        put(w, id);
        put(w, a.phytomers.len() as u64);
        for p in &a.phytomers {
            for v in [p.tip.x, p.tip.y, p.tip.z, p.radius] {
                put(w, v.to_bits());
            }
        }
    }
}

fn get(r: &mut impl Read) -> Option<u64> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b).ok()?;
    Some(u64::from_le_bytes(b))
}

fn read(path: &str) -> Vec<(String, Tree)> {
    let mut r = BufReader::new(std::fs::File::open(path).unwrap());
    let mut out = Vec::new();
    while let Some(n) = get(&mut r) {
        let mut label = vec![0u8; n as usize];
        r.read_exact(&mut label).unwrap();
        let axes = get(&mut r).unwrap();
        let mut tree = Tree::new();
        for _ in 0..axes {
            let id = get(&mut r).unwrap();
            let n = get(&mut r).unwrap();
            let ps = (0..n)
                .map(|_| std::array::from_fn(|_| f64::from_bits(get(&mut r).unwrap())))
                .collect();
            tree.insert(id, ps);
        }
        out.push((String::from_utf8(label).unwrap(), tree));
    }
    out
}

fn compare(a: &str, b: &str) {
    let (a, b) = (read(a), read(b));
    println!("| Tree | Axes | Axes only before / after | Phytomers | Phytomers only before / after | Tip moved max / RMS (m) | Radius max rel. |");
    println!("|---|--:|--:|--:|--:|--:|--:|");
    for ((label, ta), (_, tb)) in a.iter().zip(&b) {
        let (mut only_a, mut only_b, mut pa, mut pb) = (0, 0, 0usize, 0usize);
        let (mut phy, mut max, mut sum, mut n, mut rad) = (0usize, 0.0f64, 0.0, 0usize, 0.0f64);
        for (id, ps) in ta {
            phy += ps.len();
            let Some(qs) = tb.get(id) else {
                only_a += 1;
                pa += ps.len();
                continue;
            };
            pa += ps.len().saturating_sub(qs.len());
            pb += qs.len().saturating_sub(ps.len());
            for (p, q) in ps.iter().zip(qs) {
                let d =
                    ((p[0] - q[0]).powi(2) + (p[1] - q[1]).powi(2) + (p[2] - q[2]).powi(2)).sqrt();
                max = max.max(d);
                sum += d * d;
                n += 1;
                if p[3] > 0.0 {
                    rad = rad.max(((q[3] - p[3]) / p[3]).abs());
                }
            }
        }
        for (id, qs) in tb {
            if !ta.contains_key(id) {
                only_b += 1;
                pb += qs.len();
            }
        }
        let rms = (sum / n.max(1) as f64).sqrt();
        println!(
            "| {label} | {} | {only_a} / {only_b} | {phy} | {pa} / {pb} | {max:.2e} / {rms:.2e} | {rad:.2e} |",
            ta.len()
        );
    }
}
