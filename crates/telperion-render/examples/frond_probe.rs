//! fn-272 cut 4 diagnosis (owner, 2026-10-19: "flying leaves" on the oak
//! to palm walk): each frame's tree built as the film builds it, its frond
//! rows, its drawn wood, and where its leaves stand against that wood and
//! each other.
//!
//!   film_probe <frames.tsv> <segment> <first frame> <last frame> <step>
use std::collections::HashMap;
use telperion_core::{blend, math::Vec3, mesh, presets::by_identity};

const CELL: f64 = 0.5;

fn key(p: Vec3) -> (i64, i64, i64) {
    (
        (p.x / CELL).floor() as i64,
        (p.y / CELL).floor() as i64,
        (p.z / CELL).floor() as i64,
    )
}

/// Points hashed by cell, each with a radius.
struct Grid(HashMap<(i64, i64, i64), Vec<(Vec3, f64)>>);

impl Grid {
    fn of(points: impl Iterator<Item = (Vec3, f64)>) -> Grid {
        let mut m: HashMap<_, Vec<_>> = HashMap::new();
        for (p, r) in points {
            m.entry(key(p)).or_default().push((p, r));
        }
        Grid(m)
    }

    /// The least distance from `p` to a point's surface within `reach`
    /// cells, skipping `p` itself; none past it.
    fn nearest(&self, p: Vec3, reach: i64) -> Option<f64> {
        let (x, y, z) = key(p);
        let mut best: Option<f64> = None;
        for dx in -reach..=reach {
            for dy in -reach..=reach {
                for dz in -reach..=reach {
                    for &(q, r) in self.0.get(&(x + dx, y + dy, z + dz)).into_iter().flatten() {
                        let d = (q - p).length();
                        if d == 0.0 && r == 0.0 {
                            continue;
                        }
                        let d = (d - r).max(0.0);
                        best = Some(best.map_or(d, |b: f64| b.min(d)));
                    }
                }
            }
        }
        best.filter(|d| *d <= reach as f64 * CELL)
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [tsv, seg, first, last, step] = &args[..] else {
        return Err("usage: frond_probe <frames.tsv> <segment> <first> <last> <step>".into());
    };
    let num = |s: &str| s.parse::<usize>().map_err(|e| e.to_string());
    let (seg, first, last, step) = (num(seg)?, num(first)?, num(last)?, num(step)?);
    let rows: Vec<(usize, f64)> = std::fs::read_to_string(tsv)
        .map_err(|e| e.to_string())?
        .lines()
        .skip(1)
        .filter_map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            let frame = f[0].parse::<usize>().ok()?;
            (f[1].parse::<usize>().ok()? == seg
                && frame >= first
                && frame <= last
                && (frame - first) % step == 0)
                .then(|| Some((frame, f[2].parse::<f64>().ok()?)))?
        })
        .collect();
    let at = |id: &str| -> Result<_, String> {
        let mut f = by_identity(id).map_err(|e| e.to_string())?;
        (f.age, f.skeleton.seed) = (40.0, 7);
        Ok(f)
    };
    let (a, b) = (at("oregon-white-oak")?, at("date-palm")?);
    println!("frame\tt\tfronds\tleaflets\trachis_m\tcauline\tsize\tleaves\tleaf_len_m\twood_min_r_mm\toff_wood_med_m\toff_wood_gt0.5\tgap_med_lengths\tisolated");
    for (frame, t) in rows {
        let family = blend::families(&a, &b, t).map_err(|e| e.to_string())?;
        let tree = mesh::build(&family).map_err(|e| e.to_string())?;
        let c = &family.canopy;
        let wood = &tree.curve.points;
        let min_r = wood.iter().map(|p| p.radius).fold(f64::INFINITY, f64::min);
        let grid = Grid::of(wood.iter().map(|p| (p.centre, p.radius)));
        let inst = &tree.foliage.instances;
        let extent = {
            let e = &tree.foliage.element.positions;
            let (lo, hi) = e
                .iter()
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| {
                    (lo.min(p.y), hi.max(p.y))
                });
            hi - lo
        };
        let leaves: Vec<(Vec3, f64)> = (0..inst.leaves.len())
            .map(|i| {
                let m = inst.matrix(i);
                let scale = (1..2)
                    .map(|c| {
                        (0..3)
                            .map(|r| f64::from(m[c * 4 + r]).powi(2))
                            .sum::<f64>()
                            .sqrt()
                    })
                    .fold(0.0, f64::max);
                (inst.position(i), scale * extent)
            })
            .collect();
        let leafgrid = Grid::of(leaves.iter().map(|(p, _)| (*p, 0.0)));
        let mut off: Vec<f64> = leaves
            .iter()
            .map(|(p, _)| grid.nearest(*p, 6).unwrap_or(9.0))
            .collect();
        let mut gaps: Vec<f64> = leaves
            .iter()
            .map(|(p, l)| leafgrid.nearest(*p, 6).unwrap_or(9.0) / l.max(1e-9))
            .collect();
        let isolated = gaps.iter().filter(|g| **g > 1.0).count();
        let median = |v: &mut Vec<f64>| {
            v.sort_by(f64::total_cmp);
            v.get(v.len() / 2).copied().unwrap_or(f64::NAN)
        };
        let far = off.iter().filter(|d| **d > 0.5).count();
        let mut lens: Vec<f64> = leaves.iter().map(|(_, l)| *l).collect();
        println!(
            "{frame}\t{t:.4}\t{:.2}\t{:.2}\t{:.3}\t{:.3}\t{:.3}\t{}\t{:.3}\t{:.2}\t{:.3}\t{far}\t{:.2}\t{isolated}",
            c.rosette_fronds,
            c.leaflet_count,
            c.rachis_length,
            c.cauline,
            c.size,
            leaves.len(),
            median(&mut lens),
            min_r * 1e3,
            median(&mut off),
            median(&mut gaps),
        );
    }
    Ok(())
}
