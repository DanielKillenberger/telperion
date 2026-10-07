//! The light tool (fn-211.4): the coupling pass's last field (before the
//! tree's last cycle) against the laid tree's field of the same cycle's
//! leaves, read at the same points. The error splits into position (the
//! pass's field at its own points against at the laid points) and field
//! (the two fields at the laid points), and the field into its coupling
//! leaves and its fine stand-ins (each population swapped in turn).
use super::stats::quantiles;
use std::collections::HashMap;
use telperion_space::{field_reads, Light, Measured, Species, Structure, Vec3};

/// The laid tree's leaves of `cycle`, marked where a fine root's subtree
/// bears them.
fn laid_leaves(
    s: &Structure,
    species: &Species,
    tag: &[usize],
    cycle: u32,
) -> Vec<(Vec3, f64, bool)> {
    let mut out = Vec::new();
    for (i, a) in s.axes.iter().enumerate() {
        let area = species.states[a.pa].leaf_area;
        for p in a.phytomers.iter().filter(|p| p.cycle == cycle) {
            out.push((p.tip, p.scale * area, tag[i] != usize::MAX));
        }
    }
    out
}

fn kept(v: &[(Vec3, f64, bool)], keep: &dyn Fn(bool) -> bool) -> Vec<(Vec3, f64)> {
    v.iter().filter(|l| keep(l.2)).map(|l| (l.0, l.1)).collect()
}

fn line(label: &str, v: &[f64], mean: f64) -> String {
    let abs: Vec<f64> = v.iter().map(|x| x.abs()).collect();
    let (_, p50, p95) = quantiles(&abs);
    let signed = v.iter().sum::<f64>() / v.len().max(1) as f64;
    format!(
        "    {label}: p50 {p50:.4} ({:.1} %) p95 {p95:.4} ({:.1} %), signed {signed:+.4}",
        100.0 * p50 / mean,
        100.0 * p95 / mean
    )
}

/// Prints the pass's light against the laid tree's, for coupling buds
/// and fine roots apart.
pub fn compare(s: &Structure, m: &Measured, species: &Species, light: &Light) {
    let last = s.age - 1;
    let laid = laid_leaves(s, species, &m.tag, last);
    let area = |v: &[(Vec3, f64, bool)], fine: bool| -> f64 {
        v.iter().filter(|l| l.2 == fine).map(|l| l.1).sum()
    };
    println!(
        "  leaf area of cycle {last}: the pass's coupling {:.1}, stand-ins {:.1}; the laid tree's coupling {:.1}, fine {:.1} m²",
        area(&m.leaves, false),
        area(&m.leaves, true),
        area(&laid, false),
        area(&laid, true)
    );
    let tip = |i: usize| {
        let a = &s.axes[i];
        let p = a.phytomers.iter().rev().find(|p| p.cycle <= last);
        p.map_or(a.base, |p| p.tip)
    };
    // A fine root's buds that grew in the last cycle, by its tag.
    let mut buds: HashMap<usize, Vec<Vec3>> = HashMap::new();
    for (i, a) in s
        .axes
        .iter()
        .enumerate()
        .filter(|(i, _)| m.tag[*i] != usize::MAX)
    {
        if let Some(p) = a.phytomers.iter().rev().find(|p| p.cycle == last) {
            buds.entry(m.tag[i]).or_default().push(p.tip);
        }
    }
    let (mut pass_at, mut laid_at, mut groups) = (Vec::new(), Vec::new(), Vec::new());
    for &(j, at, read, fine) in &m.reads {
        if j >= s.axes.len() || fine != (m.tag[j] != usize::MAX) {
            continue;
        }
        let start = laid_at.len();
        match fine {
            true => laid_at.extend(buds.get(&m.tag[j]).into_iter().flatten()),
            false => laid_at.push(tip(j)),
        }
        if laid_at.len() == start {
            continue;
        }
        pass_at.push(at);
        groups.push((fine, read, start, laid_at.len()));
    }
    let points: Vec<Vec3> = pass_at.iter().chain(&laid_at).copied().collect();
    let np = pass_at.len();
    let read = |leaves: &[(Vec3, f64)]| field_reads(light, leaves, &points, 0.5);
    let pass = read(&kept(&m.leaves, &|_| true));
    let real = read(&kept(&laid, &|_| true));
    let mut swap = kept(&laid, &|f| !f);
    swap.extend(kept(&m.leaves, &|f| f));
    let stand_ins = read(&swap);
    let mean =
        |v: &[f64], a: usize, b: usize| v[np + a..np + b].iter().sum::<f64>() / (b - a) as f64;
    for kind in [false, true] {
        let mut d: [Vec<f64>; 4] = Default::default();
        let mut lit = 0.0;
        for (g, &(fine, _, a, b)) in groups.iter().enumerate().filter(|(_, g)| g.0 == kind) {
            let _ = fine;
            let l = mean(&real, a, b);
            lit += l;
            d[0].push(pass[g] - l);
            d[1].push(pass[g] - mean(&pass, a, b));
            d[2].push(mean(&pass, a, b) - l);
            d[3].push(mean(&stand_ins, a, b) - l);
        }
        let n = d[0].len();
        if n == 0 {
            continue;
        }
        let lit = lit / n as f64;
        let who = if kind {
            "fine roots (their buds' mean)"
        } else {
            "coupling buds"
        };
        println!("  {who}, n {n}, laid light mean {lit:.3}:");
        println!(
            "{}",
            line("total, pass at its point against laid", &d[0], lit)
        );
        println!(
            "{}",
            line(
                "position, the pass's field at its point against at the laid",
                &d[1],
                lit
            )
        );
        println!(
            "{}",
            line(
                "field, the pass's against the laid at the laid points",
                &d[2],
                lit
            )
        );
        println!(
            "{}",
            line(
                "  of which the stand-ins (laid coupling leaves and stand-ins)",
                &d[3],
                lit
            )
        );
    }
}
