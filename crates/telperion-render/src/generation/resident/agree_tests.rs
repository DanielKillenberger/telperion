//! fn-256 task 3: the resident build, the device's whole tree, against
//! the CPU's reference (the CPU's cycle, drawn through `prepare_whole`),
//! by what each draws.
use super::super::cycle::tree::{self, Kernels};
use super::super::{io, Delivery, Generator};
use super::fold::{CA_WORDS, CV};
use crate::Renderer;
use std::collections::HashMap;
use telperion_core::pipeline::executor;

fn family(id: &str, age: u32) -> telperion_core::presets::Family {
    let mut family = telperion_core::presets::by_identity(id).unwrap();
    family.age = f64::from(age);
    family
}

/// A drawn node as the device's dress left it: its position, radii,
/// codominance, flags and key, and its parent's key.
#[derive(Debug, Clone, Copy)]
struct Node {
    position: [f32; 3],
    radii: [f32; 3],
    codom: f32,
    stem: bool,
    key: u32,
    /// Its phytomer's key, or none for a relay's join.
    identity: Option<u64>,
    parent: Option<u32>,
}

/// Every node the device drew, by its id.
fn device_nodes(gpu: &crate::Gpu, k: &Kernels, f: &telperion_core::presets::Family) -> Vec<(u32, Node)> {
    let growth = executor::growth(f).unwrap();
    let (dressed, trunk) = pollster::block_on(tree::drawn(gpu, k, &growth)).unwrap();
    println!("device trunk {trunk:?}");
    let mut raw: Vec<(u32, Vec<f32>)> = Vec::new();
    for w in &dressed.waves {
        let words = w.axes as usize * CA_WORDS + w.phyts as usize * CV;
        let bytes = io::read(gpu, &w.buffer, (words.max(4) * 4) as u64).unwrap();
        raw.push((w.axes, bytemuck::cast_slice(&bytes).to_vec()));
    }
    // Each id's key, then the nodes.
    let mut keys = HashMap::new();
    let mut out = Vec::new();
    for (axes, words) in &raw {
        let records = &words[*axes as usize * CA_WORDS..];
        for r in records.chunks(CV) {
            let u = |k: usize| r[k].to_bits();
            if u(6) & 2 != 0 {
                keys.insert(u(14), u(24));
            }
        }
    }
    for (w, (axes, words)) in raw.iter().enumerate() {
        let records = &words[*axes as usize * CA_WORDS..];
        for (i, r) in records.chunks(CV).enumerate() {
            let u = |k: usize| r[k].to_bits();
            if w == 0 || u(6) & 2 == 0 || u(7) != i as u32 {
                continue;
            }
            let node = Node {
                position: [r[0], r[1], r[2]],
                radii: [r[3], r[4], r[5]],
                codom: r[13],
                stem: u(6) & 128 != 0,
                key: u(24),
                identity: (u(26) | u(27) != 0).then(|| u64::from(u(26)) | u64::from(u(27)) << 32),
                parent: keys.get(&u(16)).copied(),
            };
            out.push((u(14), node));
        }
    }
    out
}

#[test]
#[ignore = "a probe"]
fn drawn_probe() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let ids = std::env::var("AGREE_PRESETS").unwrap_or("european-beech".into());
    let age: u32 = std::env::var("AGREE_AGE").ok().and_then(|a| a.parse().ok()).unwrap_or(12);
    for id in ids.split(',') {
        let f = family(id, age);
        let dev = device_nodes(&gpu, &k, &f);
        let cpu = executor::grow(&f).unwrap().expansion().unwrap();
        let tree = cpu.tree();
        println!("cpu root {} bearing {:?}", tree.nodes[0].radius, cpu.bearing_bounds());
        let mut by_key: HashMap<u32, usize> = HashMap::new();
        let mut by_identity: HashMap<u64, usize> = HashMap::new();
        for i in 1..tree.nodes.len() {
            by_key.insert(tree.key(i), i);
            by_identity.insert(tree.nodes[i].identity.birth_order(), i);
        }
        let (mut matched, mut worst, mut stems, mut parents) = (0, [0.0f64; 4], 0, 0);
        let mut first_bad = None;
        for (_, d) in &dev {
            let found = match d.identity {
                Some(k) => by_identity.get(&k),
                None => by_key.get(&d.key),
            };
            let Some(&i) = found else { continue };
            matched += 1;
            let n = &tree.nodes[i];
            let p = [n.position.x, n.position.y, n.position.z];
            let dp = (0..3).map(|c| (f64::from(d.position[c]) - p[c]).abs()).fold(0.0, f64::max);
            let dr = (f64::from(d.radii[0]) - n.radius).abs();
            let ds = (f64::from(d.radii[1]) - n.start_radius).abs();
            let db = (f64::from(d.radii[2]) - n.base_radius).abs();
            for (k, v) in [dp, dr, ds, db].into_iter().enumerate() {
                worst[k] = worst[k].max(v);
            }
            if dp > 1e-4 || d.stem != n.stem || (f64::from(d.radii[2]) - n.base_radius).abs() > 1e-4 {
                if first_bad.is_none() {
                    first_bad = Some((i, d.position, p));
                }
                let q = n.parent.unwrap() as usize;
                println!(
                    "  bad {i}: dp {dp:.2e} stem {} / {} base {:.4} / {:.4} run start {} parent {q} kids {} codom {:?} / {} identity {:#x}",
                    d.stem, n.stem, d.radii[2], n.base_radius, n.branch as usize == i,
                    tree.nodes.iter().filter(|m| m.parent == Some(q as u32)).count(),
                    n.codominant, d.codom, n.identity.birth_order()
                );
            }
            stems += usize::from(d.stem != n.stem);
            let cp = n.parent.map(|q| tree.key(q as usize));
            parents += usize::from(cp != d.parent && n.parent != Some(0));
        }
        let seen: std::collections::HashSet<u32> = dev.iter().map(|(_, d)| d.key).collect();
        let missing: Vec<usize> = (1..tree.nodes.len()).filter(|&i| !seen.contains(&tree.key(i))).collect();
        for &i in missing.iter().take(12) {
            let n = &tree.nodes[i];
            let p = n.parent.unwrap() as usize;
            println!(
                "  missing {i}: y {:.3} r {:.2e} stem {} run start {} codom {:?} parent {p} (seen {}, r {:.2e}, stem {}) kids of parent {}",
                n.position.y,
                n.radius,
                n.stem,
                n.branch as usize == i,
                n.codominant,
                seen.contains(&tree.key(p)) || p == 0,
                tree.nodes[p].radius,
                tree.nodes[p].stem,
                tree.nodes.iter().filter(|m| m.parent == Some(p as u32)).count()
            );
        }
        println!(
            "{id} {age}: device {} nodes, cpu {}, matched {matched}; worst position {:.2e} radius {:.2e} start {:.2e} base {:.2e}; stems differ {stems}, parents differ {parents}; first bad {first_bad:?}",
            dev.len(),
            tree.nodes.len() - 1,
            worst[0],
            worst[1],
            worst[2],
            worst[3]
        );
    }
}

#[test]
#[ignore = "a probe"]
fn probe() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let ids = std::env::var("AGREE_PRESETS").unwrap_or("european-beech".into());
    let age: u32 = std::env::var("AGREE_AGE").ok().and_then(|a| a.parse().ok()).unwrap_or(12);
    for id in ids.split(',') {
        let f = family(id, age);
        let device = gen.prepare(&f, Delivery::Resident).unwrap();
        let cpu = pollster::block_on(gen.prepare_whole(&f, Delivery::Resident)).unwrap();
        println!(
            "{id} {age}: leaves {} / {}, bounds {:?} / {:?}",
            device.count(),
            cpu.count(),
            device.mesh.bounds,
            cpu.mesh.bounds
        );
    }
}

#[test]
#[ignore = "a probe"]
fn stations_probe() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let ids = std::env::var("AGREE_PRESETS").unwrap_or("european-beech".into());
    let age: u32 = std::env::var("AGREE_AGE").ok().and_then(|a| a.parse().ok()).unwrap_or(12);
    for id in ids.split(',') {
        let f = family(id, age);
        let kit = gen.kit();
        let built = pollster::block_on(super::build::build(&gpu, &gen, &kit.kernels, &kit.tools, &f)).unwrap();
        let d = &built.device;
        let words = d.segments as usize * super::stations::GW;
        let raw = io::read(&gpu, &d.buffer, (words.max(4) * 4) as u64).unwrap();
        let mine: &[u32] = bytemuck::cast_slice(&raw);
        let mut dev: HashMap<u32, u64> = HashMap::new();
        for g in mine.chunks(super::stations::GW) {
            *dev.entry(g[27]).or_default() += u64::from(g[25]);
        }
        let x = executor::grow(&f).unwrap().expansion().unwrap();
        let (_, shared) = x.curve_with_contacts().unwrap();
        let theirs = x.compact_stations(&shared).unwrap().unwrap();
        let mut cpu: HashMap<u32, u64> = HashMap::new();
        for s in &theirs.segments {
            *cpu.entry(s.key).or_default() += u64::from(s.count);
        }
        let keys: std::collections::BTreeSet<u32> = dev.keys().chain(cpu.keys()).copied().collect();
        let differ: Vec<(u32, u64, u64)> = keys
            .into_iter()
            .map(|k| (k, dev.get(&k).copied().unwrap_or(0), cpu.get(&k).copied().unwrap_or(0)))
            .filter(|(_, a, b)| a != b)
            .collect();
        let total = |m: &HashMap<u32, u64>| m.values().sum::<u64>();
        println!("{id} {age}: stations {} / {}, keys differing {}: {:?}", total(&dev), total(&cpu), differ.len(), &differ[..differ.len().min(12)]);
        let tree = x.tree();
        for &(k, _, _) in differ.iter().take(3) {
            for g in mine.chunks(super::stations::GW).filter(|g| g[27] == k) {
                let fl = |w: u32| f32::from_bits(w);
                println!("  dev seg key {k:#x}: count {} station0 {} tile {} a ({:.4},{:.4},{:.4}) r {:.2e} b ({:.4},{:.4},{:.4}) r {:.2e} along {:.4} wood {:.4}", g[25], g[24], g[26], fl(g[0]), fl(g[1]), fl(g[2]), fl(g[3]), fl(g[4]), fl(g[5]), fl(g[6]), fl(g[7]), fl(g[20]), fl(g[21]));
            }
            for s in theirs.segments.iter().filter(|s| s.key == k) {
                println!("  cpu seg key {k:#x}: count {} first {} run_station {} a {:?} b {:?} radii {:?} along {:.4} span {:.4}", s.count, s.first, s.run_station, s.endpoints[0], s.endpoints[1], s.radii, s.along, s.span);
            }
        }
        for &(k, a, b) in differ.iter().take(4) {
            let nodes: Vec<usize> = (0..tree.nodes.len()).filter(|&i| tree.key(i) == k).collect();
            for i in nodes {
                let n = &tree.nodes[i];
                println!("  key {k:#x} ({a} / {b}): node {i} r {:.2e} start {:.2e} kind {:?} stem {} parent r {:.2e}", n.radius, n.start_radius, n.kind, n.stem, tree.nodes[n.parent.unwrap_or(0) as usize].radius);
            }
        }
    }
}
