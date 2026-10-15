//! fn-253 R1: where a tree's inflorescences are seated and what joins
//! their florets to the wood. For one preset at its own age (or a stated
//! one) and seed, prints each flowering age's seats: how many of its nodes
//! may bear a bloom, how many do, the wood age under them, whether the seat
//! is its axis's tip and how far below the tip it stands; then each drawn
//! floret's distance from the wood it belongs to, which nothing drawn spans.
//!
//!   cargo run --profile ci -p telperion-core --example bloom_seats -- <preset> [age] [seed]
use std::collections::HashMap;

use telperion_core::{
    math::Vec3,
    pipeline::{self, Request},
    presets::{Family, Preset},
    tree::Tree,
};
use telperion_space::{
    contract::{child, draw},
    lineage_keys::BLOOM,
};

/// `bloom.rs`'s window: a bloom grows in over this share past its draw.
const WINDOW: f64 = 0.1;

/// How grown the bloom on a node keyed `key` is at `share` (`presence`).
fn presence(key: u64, share: f64) -> f64 {
    let u = draw(child(key, BLOOM)) as f64 / (1u64 << 53) as f64;
    ((share - u * (1.0 - WINDOW)) / WINDOW).clamp(0.0, 1.0)
}

/// Each phytomer key's axis, age and cycle in the engine's structure,
/// regrown at the family's applied rows (the conversion keeps neither).
struct Laid {
    axis: HashMap<u64, (usize, usize, u32)>,
    age: u32,
}

fn laid(family: &Family) -> Result<Laid, String> {
    let applied = family.applied();
    let species = applied.space.species.as_ref().ok_or("no engine species")?;
    let request = telperion_space::Request {
        age: family.age as u32,
        seed: u64::from(family.skeleton.seed),
        budget: 20_000_000,
        light: applied.space.light,
    };
    let s = telperion_space::grow(species, request).map_err(|e| e.to_string())?;
    let axis = (s.axes.iter().enumerate())
        .flat_map(|(a, x)| {
            x.phytomers
                .iter()
                .map(move |p| (p.key(), (a, x.pa, p.cycle)))
        })
        .collect();
    Ok(Laid { axis, age: s.age })
}

fn quantiles(v: &mut [f64]) -> String {
    if v.is_empty() {
        return "-".into();
    }
    v.sort_by(f64::total_cmp);
    let q = |f: f64| v[((v.len() - 1) as f64 * f).round() as usize];
    format!("{:.1} / {:.1} / {:.1}", q(0.1), q(0.5), q(0.9))
}

/// Each node's next node on its own axis, where it has one.
fn successors(tree: &Tree, axis_of: &[Option<usize>]) -> Vec<Option<usize>> {
    let mut next = vec![None; tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        let p = n.parent.unwrap() as usize;
        if axis_of[i].is_some() && axis_of[i] == axis_of[p] {
            next[p] = Some(i);
        }
    }
    next
}

fn measure(family: &Family) -> Result<(), String> {
    let request = Request {
        leaves: true,
        ..Request::default()
    };
    let built = pipeline::build(family, request).map_err(|e| e.to_string())?;
    let tree = &built.skeleton.tree;
    let laid = laid(family)?;
    let key = |i: usize| tree.nodes[i].identity.birth_order();
    let axis_of: Vec<Option<usize>> = (0..tree.nodes.len())
        .map(|i| laid.axis.get(&key(i)).map(|a| a.0))
        .collect();
    let next = successors(tree, &axis_of);
    let tips: std::collections::HashSet<u32> = tree.clusters.iter().map(|c| c.node).collect();
    let p = &family.applied().bloom;
    println!(
        "{} nodes, {} bloom seats, {} cluster tips; rachis {:.3} m, pedicel {:.3} m, florets {}, droop {}",
        tree.nodes.len(),
        tree.blooms.len(),
        tips.len(),
        p.rachis,
        p.pedicel,
        p.florets,
        p.droop
    );

    // Seats by age: candidates, flowering, at the tip, below the tip.
    #[derive(Default)]
    struct Row {
        seats: usize,
        flowering: usize,
        at_tip: usize,
        cluster_tip: usize,
        below: Vec<f64>,
        wood_age: HashMap<u32, usize>,
        radius: Vec<f64>,
        per_axis: HashMap<usize, usize>,
    }
    let mut rows: HashMap<usize, Row> = HashMap::new();
    let mut flowering_seats = Vec::new();
    for b in &tree.blooms {
        let i = b.node as usize;
        let Some(&(axis, pa, cycle)) = laid.axis.get(&key(i)) else {
            continue;
        };
        let row = rows.entry(pa).or_default();
        row.seats += 1;
        if presence(key(i), b.share) <= 0.0 {
            continue;
        }
        flowering_seats.push(i);
        row.flowering += 1;
        *row.per_axis.entry(axis).or_default() += 1;
        *row.wood_age
            .entry(laid.age.saturating_sub(cycle))
            .or_default() += 1;
        row.radius.push(2e3 * tree.nodes[i].radius);
        row.cluster_tip += usize::from(tips.contains(&b.node));
        // The grown wood from the seat to its axis's last drawn node.
        let (mut j, mut below) = (i, 0.0);
        while let Some(k) = next[j] {
            below += tree.grown_length(k);
            j = k;
        }
        row.at_tip += usize::from(j == i);
        row.below.push(1e3 * below);
    }
    let mut ages: Vec<_> = rows.keys().copied().collect();
    ages.sort_unstable();
    for pa in ages {
        let r = rows.get_mut(&pa).unwrap();
        let mut wood: Vec<_> = r.wood_age.iter().map(|(a, n)| (*a, *n)).collect();
        wood.sort_unstable();
        let axes = r.per_axis.len();
        let multi = r.per_axis.values().filter(|&&n| n > 1).count();
        println!(
            "age {pa}: {} seats, {} flowering ({} at their axis's tip, {} at a cluster tip); \
             {axes} axes bear them, {multi} bear more than one",
            r.seats, r.flowering, r.at_tip, r.cluster_tip
        );
        println!(
            "  below the tip mm p10/median/p90 {}; seat wood diameter mm {}; wood age (years: seats) {wood:?}",
            quantiles(&mut r.below),
            quantiles(&mut r.radius)
        );
    }

    // Each drawn floret against its nearest flowering seat: the span from
    // the seat's wood surface to the floret's base, which the rachis and
    // the pedicel span where they are drawn (fn-253). A rachis instance,
    // marked by the scale's sign bit, is counted apart.
    let Some(organ) = built.outputs.organ.as_ref() else {
        println!("no organ drawn");
        return Ok(());
    };
    let cell = (p.rachis + p.pedicel).max(1e-3);
    let at = |v: Vec3| {
        let c = v * (1.0 / cell);
        (c.x.floor() as i64, c.y.floor() as i64, c.z.floor() as i64)
    };
    let mut grid: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();
    for &i in &flowering_seats {
        grid.entry(at(tree.nodes[i].position)).or_default().push(i);
    }
    let instances = &organ.instances;
    let (mut gaps, mut far) = (Vec::with_capacity(instances.len()), 0usize);
    let rachis = |k: usize| telperion_core::foliage::withered(instances.leaves[k]);
    let rachises = (0..instances.len()).filter(|&k| rachis(k)).count();
    for k in (0..instances.len()).filter(|&k| !rachis(k)) {
        let q = instances.position(k);
        let (x, y, z) = at(q);
        let mut best = f64::INFINITY;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for &i in grid.get(&(x + dx, y + dy, z + dz)).into_iter().flatten() {
                        let n = &tree.nodes[i];
                        best = best.min(n.position.distance(q) - n.radius);
                    }
                }
            }
        }
        if best.is_finite() {
            gaps.push(1e3 * best);
        } else {
            far += 1;
        }
    }
    println!(
        "{} florets drawn; gap from the seat's wood to the floret base mm p10/median/p90 {} \
         ({far} with no seat within a reach); {rachises} rachis instances drawn, the stalks \
         spanning the gap where any",
        instances.len() - rachises,
        quantiles(&mut gaps)
    );
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [id, rest @ ..] = &args[..] else {
        return Err("usage: bloom_seats <preset> [age] [seed]".into());
    };
    let preset = Preset::from_id(id).ok_or_else(|| format!("unknown tree {id}"))?;
    let mut family = preset.parameters();
    if let Some(age) = rest.first() {
        family.age = age.parse().map_err(|e| format!("age {age}: {e}"))?;
    }
    if let Some(seed) = rest.get(1) {
        family.skeleton.seed = seed.parse().map_err(|e| format!("seed: {e}"))?;
    }
    println!("== {id} {} seed {}", family.age, family.skeleton.seed);
    measure(&family)
}
