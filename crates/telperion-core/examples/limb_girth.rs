//! Girth along a tree's thickest limbs, split into what the tip count takes
//! and what `lengthTaper` takes. One JSON line per limb: the radius at each
//! tenth of the limb's path to a structural tip, through the thickest child
//! at every fork, as a share of its base, from the family as
//! given, from the same family with `lengthTaper` zero (the pipe part), and
//! the structural tips each point carries. LIMB_FAMILY names a partial wire
//! laid over the preset, a candidate table that is no shipped preset.
use serde_json::json;
use telperion_core::{
    params,
    pipeline::{self, Request},
    tree::{BudFate, Tree},
    Family,
};

const LIMBS: usize = 4;
const STATIONS: usize = 11;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let preset = args.get(1).ok_or("preset required")?;
    let seed: u32 = args.get(2).ok_or("seed required")?.parse()?;
    let mut f = telperion_core::presets::Preset::from_id(preset)
        .ok_or("unknown preset")?
        .parameters();
    if let Some(path) = std::env::var_os("LIMB_FAMILY") {
        let rows: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;
        f = params::overlay(&f, &rows).map_err(|e| format!("{e:?}"))?;
    }
    f.skeleton.seed = seed;
    let full = skeleton(&f)?;
    let mut pipe_only = f.clone();
    pipe_only.radii.length_taper = 0.0;
    let pipe = skeleton(&pipe_only)?;
    // Radius feeds growth, so the pipe part is read only on the same wood.
    let same = pipe.nodes.len() == full.nodes.len() && pipe.crossover == full.crossover;
    let tips = tips(&full);
    let mut axes = axes(&full);
    axes.sort_by(|a, b| full.nodes[b[0]].radius.total_cmp(&full.nodes[a[0]].radius));
    let twig = full.nodes[..full.crossover]
        .iter()
        .map(|n| n.radius)
        .fold(f64::INFINITY, f64::min);
    let thickest = thickest(&full);
    let limbs = axes.iter().filter(|a| a.len() > 2).take(LIMBS);
    for (rank, axis) in limbs.map(|a| path(a[0], &thickest)).enumerate() {
        let axis = &axis;
        let path = arc(&full, axis);
        let length = *path.last().unwrap();
        let at = |s: f64| {
            let k = path.partition_point(|&d| d < s * length);
            axis[k.min(axis.len() - 1)]
        };
        let (b, t0) = (axis[0], tips[axis[0]] as f64);
        let r0 = full.nodes[b].radius;
        let p0 = if same { pipe.nodes[b].radius } else { f64::NAN };
        let profile: Vec<_> = (0..STATIONS)
            .map(|k| {
                let i = at(k as f64 / (STATIONS - 1) as f64);
                let r = full.nodes[i].radius / r0;
                let p = if same {
                    pipe.nodes[i].radius / p0
                } else {
                    f64::NAN
                };
                json!({"s": k as f64 / (STATIONS - 1) as f64, "radius": r, "pipe": p,
                    "taper": r / p, "tips": tips[i] as f64 / t0})
            })
            .collect();
        println!(
            "{}",
            json!({"preset": preset, "seed": seed, "rank": rank, "first": b,
                "base_radius_m": r0, "twig_radius_m": twig, "length_m": length,
                "height_share": full.nodes[b].position.y / f.skeleton.envelope.height,
                "nodes": axis.len(), "base_tips": t0, "role": role(&full, b),
                "stem_end": stem_end(&full, axis, &path, r0), "profile": profile})
        );
    }
    Ok(())
}

/// How an axis began: a stem, a limb (a first-order axis off a stem), or
/// deeper wood.
fn role(tree: &Tree, i: usize) -> &'static str {
    let n = &tree.nodes[i];
    match n.parent.map(|p| &tree.nodes[p as usize]) {
        _ if n.stem => "stem",
        Some(p) if p.stem => "limb",
        None => "root",
        _ => "deeper",
    }
}

/// Where the path leaves the stems for a limb: its share of the path and
/// the radius there as a share of the base; null on a path with no stem.
fn stem_end(tree: &Tree, axis: &[usize], path: &[f64], r0: f64) -> serde_json::Value {
    let Some(k) = axis.iter().rposition(|&i| tree.nodes[i].stem) else {
        return serde_json::Value::Null;
    };
    json!({"s": path[k] / path.last().unwrap(), "radius": tree.nodes[axis[k]].radius / r0})
}

fn skeleton(f: &Family) -> Result<Tree, Box<dyn std::error::Error>> {
    Ok(pipeline::build(f, Request::default())?.skeleton.tree)
}

/// Structural tips each structural node carries.
fn tips(tree: &Tree) -> Vec<usize> {
    let count = tree.crossover;
    let mut tips = vec![0; count];
    let mut children = vec![0; count];
    for n in &tree.nodes[1..count] {
        children[n.parent.unwrap() as usize] += 1;
    }
    for i in (0..count).rev() {
        tips[i] += usize::from(children[i] == 0);
        if let Some(p) = tree.nodes[i].parent {
            tips[p as usize] += tips[i];
        }
    }
    tips
}

/// Every structural axis as its node chain: an axis starts at the root, a
/// lateral, a codominant sibling, or the primary of a fork, and runs on
/// through each node's primary continuation.
fn axes(tree: &Tree) -> Vec<Vec<usize>> {
    let count = tree.crossover;
    let mut primary = vec![None; count];
    let mut forks = vec![false; count];
    for (i, n) in tree.nodes[..count].iter().enumerate().skip(1) {
        let p = n.parent.unwrap() as usize;
        if n.codominant.is_some() {
            forks[p] = true;
        } else if n.shoot.bud_fate == BudFate::Terminal {
            primary[p] = Some(i);
        }
    }
    let starts = (0..count).filter(|&i| match tree.nodes[i].parent {
        None => true,
        Some(p) => {
            let n = &tree.nodes[i];
            n.shoot.bud_fate == BudFate::Lateral || n.codominant.is_some() || forks[p as usize]
        }
    });
    starts
        .map(|s| {
            let mut axis = vec![s];
            while let Some(next) = primary[*axis.last().unwrap()] {
                if forks[*axis.last().unwrap()] {
                    break;
                }
                axis.push(next);
            }
            axis
        })
        .collect()
}

/// Each structural node's thickest structural child.
fn thickest(tree: &Tree) -> Vec<Option<usize>> {
    let mut best: Vec<Option<usize>> = vec![None; tree.crossover];
    for i in 1..tree.crossover {
        let p = tree.nodes[i].parent.unwrap() as usize;
        if best[p].is_none_or(|b| tree.nodes[i].radius > tree.nodes[b].radius) {
            best[p] = Some(i);
        }
    }
    best
}

/// From `first` on through the thickest child to a structural tip: the limb
/// to the crown's edge, across every fork it meets.
fn path(first: usize, thickest: &[Option<usize>]) -> Vec<usize> {
    std::iter::successors(Some(first), |&i| thickest[i]).collect()
}

fn arc(tree: &Tree, axis: &[usize]) -> Vec<f64> {
    let mut d = vec![0.0];
    for w in axis.windows(2) {
        let step = tree.nodes[w[0]]
            .position
            .distance(tree.nodes[w[1]].position);
        d.push(d.last().unwrap() + step);
    }
    d
}
