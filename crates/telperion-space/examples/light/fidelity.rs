//! Shared by the oracle and coupling examples (fn-211.4): one tree's
//! fidelity against today's.
use super::stats::quantiles;
use std::collections::HashMap;
use telperion_space::{bud_light, oak, Light, Origin, Structure};

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

/// A tree and its growth per cycle and PA (presence, presence × size).
pub type Grown<'a> = (&'a Structure, &'a [Vec<[f64; 2]>]);

pub fn orders(s: &Structure) -> Vec<usize> {
    let mut o = vec![0usize; s.axes.len()];
    for (i, a) in s.axes.iter().enumerate() {
        o[i] = match a.origin {
            Origin::Seed => 0,
            Origin::Lateral { parent, .. } => o[parent] + 1,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => o[parent],
        };
    }
    o
}

fn per_order(s: &Structure) -> [usize; 5] {
    let mut k = [0usize; 5];
    for (a, o) in s.axes.iter().zip(orders(s)) {
        k[o.min(4)] += a.phytomers.len();
    }
    k
}

/// Per PA, the whole growth (Σ presence × size over cycles).
fn totals(g: &[Vec<[f64; 2]>]) -> Vec<f64> {
    let mut t = Vec::new();
    for row in g {
        if t.len() < row.len() {
            t.resize(row.len(), 0.0);
        }
        for (pa, v) in row.iter().enumerate() {
            t[pa] += v[1];
        }
    }
    t
}

/// The crown: its height, and the horizontal reach of its leaves' tips
/// from the trunk's base (p50, p95, most).
fn extent(s: &Structure) -> (f64, [f64; 3]) {
    let base = s.axes[0].base;
    let mut top: f64 = 0.0;
    let mut reach = Vec::new();
    for a in &s.axes {
        for p in &a.phytomers {
            top = top.max(p.tip.z);
            if p.cycle == s.age {
                let (dx, dy) = (p.tip.x - base.x, p.tip.y - base.y);
                reach.push((dx * dx + dy * dy).sqrt());
            }
        }
    }
    reach.sort_by(f64::total_cmp);
    let q = quantiles(&reach);
    (top, [q.1, q.2, *reach.last().unwrap_or(&0.0)])
}

/// One tree against today's: kept phytomers per order, growth per PA
/// (whole life and worst cycle), the trunk's base, the order-1 junctions,
/// the crown's height and reach, and the bud light in the laid tree.
pub fn describe(name: &str, (o, og): Grown, (t, tg): Grown) {
    let (ko, kt) = (per_order(o), per_order(t));
    let by_order: Vec<String> = (0..5)
        .map(|i| {
            format!(
                "{}: {} ({:+.1} %)",
                if i == 4 { "4+".into() } else { i.to_string() },
                ko[i],
                pct(ko[i] as f64, kt[i] as f64)
            )
        })
        .collect();
    let all: (usize, usize) = (ko.iter().sum(), kt.iter().sum());
    println!(
        "  {name}: kept {} ({:+.1} %); by order {}",
        all.0,
        pct(all.0 as f64, all.1 as f64),
        by_order.join(", ")
    );
    let (go, gt) = (totals(og), totals(tg));
    let mut drift = Vec::new();
    for pa in 0..gt.len().max(go.len()) {
        let (a, b) = (
            go.get(pa).copied().unwrap_or(0.0),
            gt.get(pa).copied().unwrap_or(0.0),
        );
        if b > 0.0 {
            // The worst cycle, among cycles holding 1 % of the PA's growth.
            let mut worst: f64 = 0.0;
            for (c, row) in tg.iter().enumerate() {
                let x = row.get(pa).map_or(0.0, |v| v[1]);
                let y = og.get(c).and_then(|r| r.get(pa)).map_or(0.0, |v| v[1]);
                if x > 0.01 * b {
                    worst = worst.max((y / x - 1.0).abs());
                }
            }
            drift.push(format!(
                "PA {pa} {:+.2} % (worst cycle {:.1} %)",
                pct(a, b),
                100.0 * worst
            ));
        }
    }
    println!("    growth per PA: {}", drift.join(", "));
    let base = |s: &Structure| s.axes[0].phytomers[0].radius;
    let first = |s: &Structure| -> HashMap<u64, f64> {
        let oo = orders(s);
        s.axes
            .iter()
            .enumerate()
            .filter(|(i, a)| oo[*i] == 1 && matches!(a.origin, Origin::Lateral { .. }))
            .filter_map(|(_, a)| a.phytomers.first().map(|p| (a.lineage, p.radius)))
            .collect()
    };
    let (fo, ft) = (first(o), first(t));
    let ratios: Vec<f64> = fo
        .iter()
        .filter_map(|(l, r)| ft.get(l).filter(|&&x| x > 0.0).map(|x| r / x))
        .collect();
    let q = quantiles(&ratios);
    let (eo, et) = (extent(o), extent(t));
    println!(
        "    trunk base {:.4} m ({:+.1} %); order-1 base radius over today's, shared lineages (n {} of {}): p5 {:.3} p50 {:.3} p95 {:.3}; height {:.2} m ({:+.1} %); leaf reach p50 {:.2} p95 {:.2} most {:.2} m ({:+.1} / {:+.1} / {:+.1} %)",
        base(o),
        pct(base(o), base(t)),
        ratios.len(),
        ft.len(),
        q.0,
        q.1,
        q.2,
        eo.0,
        pct(eo.0, et.0),
        eo.1[0],
        eo.1[1],
        eo.1[2],
        pct(eo.1[0], et.1[0]),
        pct(eo.1[1], et.1[1]),
        pct(eo.1[2], et.1[2])
    );
    let lights = bud_light(o, &oak(), &LIT);
    let mean = lights.iter().map(|l| l.1).sum::<f64>() / lights.len().max(1) as f64;
    println!(
        "    living buds {}, mean light in the laid tree {mean:.3}",
        lights.len()
    );
}

pub fn pct(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return 0.0;
    }
    100.0 * (a / b - 1.0)
}
