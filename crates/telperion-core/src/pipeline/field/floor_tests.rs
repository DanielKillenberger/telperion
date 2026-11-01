//! fn-323's measurements of the coarse route, under goal fn-321's manifest:
//! what a field needs of a tree grown whole, and the floor's cost. All are
//! ignored and print one line per result. Each reads a tree grown whole, so
//! none shows that coupled growth under a density gives the same tree.
use super::splat::{self, Tally};
use super::{Frame, Grid, Occupancy};
use crate::math::Vec3;
use crate::pipeline::foliage::clumping::systems;
use crate::pipeline::{self, executor, measures, FieldRequest, Request};
use crate::presets::{by_identity, Family};
use crate::tree::Tree;
use std::collections::HashMap;

fn env<T: std::str::FromStr>(name: &str, or: T) -> T {
    std::env::var(name).map_or(or, |v| v.parse().ok().expect("a number"))
}

fn oak(age: u32, seed: u32) -> Family {
    let mut f = by_identity("oregon-white-oak").expect("the oak");
    f.age = f64::from(age);
    f.skeleton.seed = seed;
    f
}

/// Each node's order, the manifest's: its axis's count of lateral buds from
/// the seed, read from the structure by the phytomer's key its node carries.
/// A node with no phytomer takes its parent's. The share with none is returned
/// too, since a large one would make this no reading at all.
fn orders(tree: &Tree, f: &Family) -> (Vec<u32>, f64) {
    let growth = executor::growth(f).expect("the oak's growth");
    let grown = telperion_space::grow_cycle(&growth.species, growth.request).expect("the tree");
    let by_key = telperion_space::floor::orders_by_key(&grown.structure.axes);
    let mut order = vec![0u32; tree.nodes.len()];
    let mut missed = 0usize;
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        order[i] = match by_key.get(&n.identity.birth_order()) {
            Some(&o) => o,
            None => {
                missed += 1;
                n.parent.map_or(0, |p| order[p as usize])
            }
        };
    }
    (order, missed as f64 / tree.nodes.len() as f64)
}

#[derive(Clone, Copy, PartialEq)]
enum Kernel {
    Sphere,
    Gaussian,
}

/// One cut subtree's wood and leaves, and where its leaves stand.
#[derive(Default, Clone)]
struct Cut {
    wood: f64,
    leaves: f64,
    sum: [f64; 3],
    squares: f64,
    /// The limb system it hangs from.
    system: u32,
}

impl Cut {
    fn centre(&self, at: Vec3) -> (Vec3, f64) {
        if self.leaves <= 0.0 {
            return (at, 0.0);
        }
        let m = self.sum.map(|s| s / self.leaves);
        let mean = Vec3::new(m[0], m[1], m[2]);
        let rms = (self.squares / self.leaves - m.iter().map(|v| v * v).sum::<f64>()).max(0.0);
        (mean, rms.sqrt())
    }
}

/// Weights over the cells for a deposit at `centre`, summing to one.
fn weights(frame: Frame, centre: Vec3, rms: f64, kernel: Kernel) -> Vec<(usize, f64)> {
    let n = frame.n as i64;
    let reach = match kernel {
        Kernel::Sphere => rms,
        Kernel::Gaussian => 2.0 * rms / 3f64.sqrt(),
    };
    let index = |v: f64, o: f64| ((v - o) / frame.cell - 0.5).floor() as i64;
    let lo = [
        index(centre.x - reach, frame.origin.x),
        index(centre.y - reach, frame.origin.y),
        index(centre.z - reach, frame.origin.z),
    ];
    let hi = [
        index(centre.x + reach, frame.origin.x) + 1,
        index(centre.y + reach, frame.origin.y) + 1,
        index(centre.z + reach, frame.origin.z) + 1,
    ];
    let mut out = Vec::new();
    let mut sum = 0.0;
    for z in lo[2].max(0)..=hi[2].min(n - 1) {
        for y in lo[1].max(0)..=hi[1].min(n - 1) {
            for x in lo[0].max(0)..=hi[0].min(n - 1) {
                let c = (x + n * (y + n * z)) as usize;
                let d = frame.centre(c) - centre;
                let r2 = d.dot(d);
                if r2 > reach * reach {
                    continue;
                }
                let w = match kernel {
                    Kernel::Sphere => 1.0,
                    Kernel::Gaussian => (-1.5 * r2 / (rms * rms).max(1e-12)).exp(),
                };
                sum += w;
                out.push((c, w));
            }
        }
    }
    if out.is_empty() {
        // Cell `i` spans `i` to `i + 1` cells from the origin.
        let nearest = |v: f64, o: f64| (((v - o) / frame.cell).floor() as i64).clamp(0, n - 1);
        let (x, y, z) = (
            nearest(centre.x, frame.origin.x),
            nearest(centre.y, frame.origin.y),
            nearest(centre.z, frame.origin.z),
        );
        return vec![((x + n * (y + n * z)) as usize, 1.0)];
    }
    out.iter_mut().for_each(|e| e.1 /= sum);
    out
}

/// The field of `tree` cut at order `k`, the rest standing as density.
fn cut_field(
    (tree, plan): (&Tree, &pipeline::foliage::plan::Plan),
    frame: Frame,
    (k, kernel): (u32, Kernel),
    (order, limb_order): (&[u32], u32),
) -> Grid {
    let system = systems(tree, limb_order);
    let by_position: HashMap<[u64; 3], usize> = (tree.nodes.iter().enumerate())
        .map(|(i, n)| ([n.position.x, n.position.y, n.position.z].map(f64::to_bits), i))
        .collect();
    let mut root = vec![usize::MAX; tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        let p = n.parent.map_or(0, |p| p as usize);
        if order[i] > k {
            root[i] = if order[p] <= k { i } else { root[p] };
        }
    }
    let mut grid = Grid {
        frame,
        cells: vec![Occupancy::default(); frame.cells()],
    };
    let mut cuts: HashMap<usize, Cut> = HashMap::new();
    let sweeps: Vec<_> = splat::wood(tree).collect();
    for (i, sweep) in sweeps.iter().enumerate() {
        if order[i] <= k {
            splat::wood_into(&mut grid, sweep).expect("the wood written");
            continue;
        }
        let p = tree.nodes[i].parent.expect("a cut node has a parent") as usize;
        let cut = cuts.entry(root[i]).or_default();
        cut.wood += (tree.nodes[i].position - tree.nodes[p].position).length();
        cut.system = system[root[i]];
    }
    let mut tally = Tally::default();
    for d in &plan.descriptors {
        let tip = d.endpoints[1];
        let node = by_position
            .get(&[tip.x, tip.y, tip.z].map(f64::to_bits))
            .copied()
            .expect("a descriptor ends on a node");
        if order[node] <= k {
            let leaf = splat::Leaf::of(plan, d).expect("a leaf record");
            splat::leaf_into(&mut grid, &leaf, &mut tally).expect("the leaf written");
            continue;
        }
        let cut = cuts.entry(root[node]).or_default();
        let (w, mid) = (f64::from(d.count), (d.endpoints[0] + d.endpoints[1]) * 0.5);
        cut.leaves += w;
        let at = [mid.x, mid.y, mid.z];
        for c in 0..3 {
            cut.sum[c] += w * at[c];
            cut.squares += w * at[c] * at[c];
        }
        cut.system = system[root[node]];
    }
    tally.owners(&mut grid.cells);
    let mut wood = vec![0.0; frame.cells()];
    let mut leaves = vec![0.0; frame.cells()];
    let mut by_system: HashMap<(usize, u32), f64> = HashMap::new();
    for (&r, cut) in &cuts {
        let (centre, rms) = cut.centre(tree.nodes[r].position);
        for (c, w) in weights(frame, centre, rms, kernel) {
            wood[c] += w * cut.wood;
            leaves[c] += w * cut.leaves;
            *by_system.entry((c, cut.system)).or_default() += w * cut.leaves;
        }
    }
    let mut most: HashMap<usize, (u32, f64)> = HashMap::new();
    for (&(c, s), &sum) in &by_system {
        let e = most.entry(c).or_insert((s, sum));
        if sum > e.1 || (sum == e.1 && s < e.0) {
            *e = (s, sum);
        }
    }
    for (c, cell) in grid.cells.iter_mut().enumerate() {
        cell.leaves += leaves[c];
        cell.wood |= wood[c] >= 0.25 * frame.cell;
        cell.foliage |= leaves[c] >= 0.5;
        if cell.foliage && cell.limb.is_none() {
            cell.limb = most.get(&c).map(|e| e.0);
        }
    }
    grid
}

/// The manifest's measures of `cut` against `full`.
fn measure(full: &[Occupancy], cut: &[Occupancy]) -> [f64; 5] {
    let jaccard = |f: fn(&Occupancy) -> bool| {
        let (mut both, mut either) = (0u64, 0u64);
        for (a, b) in full.iter().zip(cut) {
            both += u64::from(f(a) && f(b));
            either += u64::from(f(a) || f(b));
        }
        if either == 0 { 1.0 } else { both as f64 / either as f64 }
    };
    let sum: f64 = full.iter().map(|c| c.leaves).sum();
    let got: f64 = cut.iter().map(|c| c.leaves).sum();
    let diff: f64 = full.iter().zip(cut).map(|(a, b)| (a.leaves - b.leaves).abs()).sum();
    let mut shared: HashMap<u32, HashMap<u32, u64>> = HashMap::new();
    for (a, b) in full.iter().zip(cut) {
        if let (Some(a), Some(b)) = (a.limb, b.limb) {
            *shared.entry(b).or_default().entry(a).or_default() += 1;
        }
    }
    let name: HashMap<u32, u32> = shared
        .iter()
        .map(|(b, m)| (*b, *m.iter().max_by_key(|(a, n)| (**n, std::cmp::Reverse(**a))).expect("a cell").0))
        .collect();
    let (mut agree, mut held) = (0u64, 0u64);
    for (a, b) in full.iter().zip(cut) {
        if a.limb.is_some() || b.limb.is_some() {
            held += 1;
            agree += u64::from(a.limb.is_some() && b.limb.and_then(|b| name.get(&b).copied()) == a.limb);
        }
    }
    [
        jaccard(|c| c.wood),
        jaccard(|c| c.foliage),
        diff / sum.max(1e-12),
        got / sum.max(1e-12),
        if held == 0 { 1.0 } else { agree as f64 / held as f64 },
    ]
}

#[test]
fn a_subtree_reaching_no_cell_centre_goes_whole_to_the_cell_it_stands_in() {
    let frame = Frame {
        n: 4,
        cell: 1.0,
        origin: Vec3::ZERO,
    };
    for kernel in [Kernel::Sphere, Kernel::Gaussian] {
        // In cell (2, 1, 3), a quarter of a cell from its low faces.
        let at = Vec3::new(2.25, 1.25, 3.25);
        let got = weights(frame, at, 0.01, kernel);
        assert_eq!(got, vec![(2 + 4 * (1 + 4 * 3), 1.0)]);
        // Three quarters across it, so a rounding would take the next one.
        let at = Vec3::new(2.75, 1.75, 3.75);
        let got = weights(frame, at, 0.01, kernel);
        assert_eq!(got, vec![(2 + 4 * (1 + 4 * 3), 1.0)]);
    }
}

#[test]
fn a_wide_subtree_is_spread_over_the_cells_it_reaches_and_sums_to_one() {
    let frame = Frame {
        n: 8,
        cell: 1.0,
        origin: Vec3::ZERO,
    };
    for kernel in [Kernel::Sphere, Kernel::Gaussian] {
        let got = weights(frame, Vec3::new(4.0, 4.0, 4.0), 2.0, kernel);
        assert!(got.len() > 8, "{} cells", got.len());
        let sum: f64 = got.iter().map(|e| e.1).sum();
        assert!((sum - 1.0).abs() < 1e-12, "{sum}");
    }
}

/// R4: each candidate's 36-cell field against the full tree's, at `FLOOR_AGE`
/// and each of `FLOOR_SEEDS`.
#[test]
#[ignore = "a measurement: fn-323 R4"]
fn floor_field() {
    let age = env("FLOOR_AGE", 40u32);
    let seeds = std::env::var("FLOOR_SEEDS").unwrap_or("1,7,42,1234,5678".into());
    for seed in seeds.split(',').map(|s| s.parse::<u32>().expect("a seed")) {
        let f = oak(age, seed);
        let ask = Request {
            field: Some(FieldRequest { n: 36, limb_order: None }),
            ..Request::default()
        };
        let built = pipeline::build(&f, ask).expect("the full tree");
        let full = built.outputs.field.as_ref().expect("a field");
        let tree = &built.skeleton.tree;
        let plan = built.outputs.plan.as_ref().expect("a plan");
        let limb_order = f.canopy.clump_system_order;
        let limbs = measures::limb_count(tree, limb_order).expect("a limb count");
        let (order, missed) = orders(tree, &f);
        println!(
            "FULL age {age} seed {seed} nodes {} limbs {limbs} limb_order {limb_order} cell {:.3} most_order {} no_phytomer {missed:.4}",
            tree.nodes.len(),
            full.frame.cell,
            order.iter().max().expect("a node")
        );
        for k in 1..=5 {
            for (name, kernel) in [("sphere", Kernel::Sphere), ("gaussian", Kernel::Gaussian)] {
                let grid = cut_field((tree, plan), full.frame, (k, kernel), (&order, limb_order));
                let m = measure(&full.cells, &grid.cells);
                let kept = order.iter().filter(|&&o| o <= k).count();
                println!(
                    "CUT age {age} seed {seed} k {k} {name} kept_nodes {kept} wood {:.4} foliage {:.4} leaves {:.4} total {:.4} limbs {:.4}",
                    m[0], m[1], m[2], m[3], m[4]
                );
            }
        }
    }
}

/// R4, R5, R6: at `FLOOR_AGE` and `FLOOR_SEED`, one full cycle's census, its
/// shed share, and the light's replay. Run under the protocol's one thread.
#[test]
#[ignore = "a measurement: fn-323 R4, R5 and R6"]
fn floor_cost() {
    let age = env("FLOOR_AGE", 40u32);
    let seed = env("FLOOR_SEED", 1u32);
    let repeats = env("FLOOR_REPEATS", 5usize);
    let growth = executor::growth(&oak(age, seed)).expect("the oak's growth");
    let (species, request) = (&growth.species, growth.request);
    let mut last = None;
    for r in 0..repeats {
        let c = telperion_space::grow_census(species, request).expect("the tree");
        println!("CYCLE age {age} seed {seed} repeat {r} seconds {:.4} light {:.4}", c.seconds, c.light_seconds);
        last = Some(c);
    }
    let c = last.expect("a repeat");
    let census = c.census.as_ref().expect("a census");
    let kept: u64 = c.structure.axes.iter().map(|a| a.phytomers.len() as u64).sum();
    println!(
        "SHED age {age} seed {seed} kept {kept} grown {} share {:.4} axes {}",
        c.grown,
        measures::shed_share(kept, c.grown).expect("a shed share"),
        c.structure.axes.len()
    );
    println!("PHASES {:?}", census.phases);
    let mut by_order = [0u64; 8];
    for y in &census.cycles {
        let line: Vec<String> = y.by_order.iter().map(usize::to_string).collect();
        println!("LIVE age {age} seed {seed} year {} {}", y.cycle, line.join(" "));
        y.by_order.iter().enumerate().for_each(|(o, n)| by_order[o] += *n as u64);
    }
    println!("APEX_CYCLES age {age} seed {seed} {by_order:?}");
    let axes = &c.structure.axes;
    for k in 1..=5 {
        let mut sum = 0.0;
        let mut seen = None;
        let years: Vec<u32> = (10..=age).step_by(10).collect();
        for &y in &years {
            let mut runs: Vec<f64> = Vec::new();
            for _ in 0..repeats {
                let rep = telperion_space::floor::light_replay(species, &request.light, axes, (k, y))
                    .expect("a replay");
                runs.push(rep.seconds);
                seen = Some(rep);
            }
            runs.sort_by(f64::total_cmp);
            sum += runs[runs.len() / 2];
        }
        println!(
            "LIGHT age {age} seed {seed} k {k} all_years_seconds {:.4} last {seen:?}",
            sum * f64::from(age) / years.len() as f64
        );
    }
}

/// R6: the full cycle's reach on the CPU at `FLOOR_AGE`: its seconds and
/// shape, or the refusal by name.
#[test]
#[ignore = "a measurement: fn-323 R6"]
fn floor_reach() {
    let age = env("FLOOR_AGE", 120u32);
    let growth = executor::growth(&oak(age, env("FLOOR_SEED", 1u32))).expect("the oak's growth");
    match telperion_space::grow_census(&growth.species, growth.request) {
        Ok(c) => println!(
            "REACH cpu age {age} seconds {:.2} light {:.2} grown {} axes {}",
            c.seconds,
            c.light_seconds,
            c.grown,
            c.structure.axes.len()
        ),
        Err(e) => println!("REACH cpu age {age} refused: {e}"),
    }
}

/// After fn-323's verdict, for the owner's options, and no candidate of the
/// manifest's: a cut by subtree size rather than by order. Nodes whose subtree
/// holds fewer than N nodes stand as the Gaussian's density.
#[test]
#[ignore = "a measurement: fn-323's options"]
fn floor_subtree_cut() {
    let age = env("FLOOR_AGE", 40u32);
    let seeds = std::env::var("FLOOR_SEEDS").unwrap_or("1,42".into());
    for seed in seeds.split(',').map(|s| s.parse::<u32>().expect("a seed")) {
        let f = oak(age, seed);
        let ask = Request {
            field: Some(FieldRequest { n: 36, limb_order: None }),
            ..Request::default()
        };
        let built = pipeline::build(&f, ask).expect("the full tree");
        let full = built.outputs.field.as_ref().expect("a field");
        let tree = &built.skeleton.tree;
        let plan = built.outputs.plan.as_ref().expect("a plan");
        let mut held = vec![1u64; tree.nodes.len()];
        for (i, n) in tree.nodes.iter().enumerate().skip(1).rev() {
            held[n.parent.map_or(0, |p| p as usize)] += held[i];
        }
        for n in [10u64, 30, 100, 300, 1000, 3000] {
            let order: Vec<u32> = held.iter().map(|&h| u32::from(h < n)).collect();
            let limb_order = f.canopy.clump_system_order;
            let grid = cut_field((tree, plan), full.frame, (0, Kernel::Gaussian), (&order, limb_order));
            let m = measure(&full.cells, &grid.cells);
            let kept = order.iter().filter(|&&o| o == 0).count();
            println!(
                "SUBTREE age {age} seed {seed} under {n} kept {:.4} wood {:.4} foliage {:.4} leaves {:.4} total {:.4} limbs {:.4}",
                kept as f64 / tree.nodes.len() as f64,
                m[0], m[1], m[2], m[3], m[4]
            );
        }
    }
}
