//! fn-256 DESIGN.md 9.4: the leaf mean taken in the one blocked order
//! moves a tree by f32 order. A measurement (`#[ignore]`): a preset grown
//! through the CPU's cycle (`grow_cycle`) at `LEAF_AGE` from `LEAF_SEED`,
//! its kept axes and phytomers, height and crown printed; written to
//! `LEAF_DUMP` where no file stands there, else held by lineage to the
//! tree written there (each phytomer's tip, radius and scale).
use std::collections::HashMap;
use telperion_core::presets::Preset;
use telperion_space::{grow_cycle, Request, Structure};

/// Each axis's lineage, and each phytomer's tip, radius and scale.
type Tree = Vec<(u64, Vec<[f64; 5]>)>;

fn of(s: &Structure) -> Tree {
    let row = |p: &telperion_space::Phytomer| [p.tip.x, p.tip.y, p.tip.z, p.radius, p.scale];
    s.axes
        .iter()
        .map(|a| (a.lineage, a.phytomers.iter().map(row).collect()))
        .collect()
}

fn written(t: &Tree) -> Vec<u8> {
    let mut out = (t.len() as u64).to_le_bytes().to_vec();
    for (lineage, rows) in t {
        out.extend(lineage.to_le_bytes());
        out.extend((rows.len() as u64).to_le_bytes());
        out.extend(rows.iter().flatten().flat_map(|v| v.to_le_bytes()));
    }
    out
}

fn read(bytes: &[u8]) -> Tree {
    let mut at = 0;
    let mut word = || {
        let w = u64::from_le_bytes(bytes[at..at + 8].try_into().expect("a word"));
        at += 8;
        w
    };
    let n = word();
    (0..n)
        .map(|_| {
            let lineage = word();
            let rows = (0..word())
                .map(|_| std::array::from_fn(|_| f64::from_bits(word())))
                .collect();
            (lineage, rows)
        })
        .collect()
}

/// Each axis keyed by its lineage and which of that lineage's axes it is.
fn keyed(t: &Tree) -> HashMap<(u64, u32), usize> {
    let mut seen: HashMap<u64, u32> = HashMap::new();
    let mut out = HashMap::new();
    for (i, (l, _)) in t.iter().enumerate() {
        let k = seen.entry(*l).or_insert(0);
        *k += 1;
        out.insert((*l, *k), i);
    }
    out
}

/// Kept axes, kept phytomers, height (the highest tip) and crown (the
/// tips' widest horizontal extent).
fn shape(t: &Tree) -> (usize, usize, f64, f64) {
    let tips = || t.iter().flat_map(|a| &a.1);
    let extent = |k: usize| {
        let (lo, hi) = tips().fold((f64::MAX, f64::MIN), |(lo, hi), r| {
            (lo.min(r[k]), hi.max(r[k]))
        });
        hi - lo
    };
    let height = tips().map(|r| r[2]).fold(0.0, f64::max);
    (t.len(), tips().count(), height, extent(0).max(extent(1)))
}

fn compared(now: &Tree, before: &Tree) {
    let at = keyed(before);
    let mut pairs = Vec::new();
    let mut unmatched = (0, 0);
    let mut seen: HashMap<u64, u32> = HashMap::new();
    for (i, (l, rows)) in now.iter().enumerate() {
        let k = seen.entry(*l).or_insert(0);
        *k += 1;
        match at.get(&(*l, *k)) {
            Some(&j) if before[j].1.len() == rows.len() => {
                pairs.extend((0..rows.len()).map(|r| (i, j, r)))
            }
            Some(_) => unmatched.1 += 1,
            None => unmatched.0 += 1,
        }
    }
    println!(
        "LEAF by lineage: {} axes now, {} before; {} now unmatched, {} matched with other phytomer counts; {} phytomers compared",
        now.len(), before.len(), unmatched.0, unmatched.1, pairs.len()
    );
    let names = ["tip x", "tip y", "tip z", "radius", "scale"];
    for (k, name) in names.iter().enumerate() {
        let (mut abs, mut rel, mut differ) = (Vec::new(), Vec::new(), 0);
        for &(i, j, r) in &pairs {
            let (x, y) = (now[i].1[r][k], before[j].1[r][k]);
            differ += usize::from(x.to_bits() != y.to_bits());
            let scale = x.abs().max(y.abs());
            abs.push((x - y).abs());
            rel.push(if x == y || scale == 0.0 {
                0.0
            } else {
                (x - y).abs() / scale
            });
        }
        let tail = |v: &mut Vec<f64>| {
            v.sort_by(f64::total_cmp);
            v.last().map_or((0.0, 0.0), |&m| (m, v[v.len() * 99 / 100]))
        };
        let ((amax, ap99), (rmax, rp99)) = (tail(&mut abs), tail(&mut rel));
        println!("LEAF {name:>7}: {differ} differ; absolute max {amax:.3e} p99 {ap99:.3e}; relative max {rmax:.3e} p99 {rp99:.3e}");
    }
}

#[test]
#[ignore = "a measurement"]
fn the_leaf_order_moves_a_tree() {
    let env = |k: &str| std::env::var(k).unwrap_or_else(|_| panic!("{k} is set"));
    let preset = Preset::from_id(&env("LEAF_PRESET")).expect("a preset");
    let (age, seed): (u32, u64) = (
        env("LEAF_AGE").parse().expect("an age"),
        env("LEAF_SEED").parse().expect("a seed"),
    );
    let family = preset.parameters();
    let species = family.space.species.expect("an engine species");
    let request = Request {
        age,
        seed,
        budget: 20_000_000,
        light: family.space.light,
    };
    let t = of(&grow_cycle(&species, request).expect("a tree").structure);
    let (axes, phytomers, height, crown) = shape(&t);
    println!("LEAF {preset:?} {age} seed {seed}: {axes} axes, {phytomers} phytomers, height {height:.4} m, crown {crown:.4} m");
    let path = std::path::PathBuf::from(env("LEAF_DUMP"));
    match std::fs::read(&path) {
        Ok(bytes) => compared(&t, &read(&bytes)),
        Err(_) => std::fs::write(&path, written(&t)).expect("written"),
    }
}
