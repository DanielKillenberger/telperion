//! The laid tree built plain at every preset's default age, seed 42, held
//! to the CPU's (`grow_cycle`, fn-255) by lineage, reported and never
//! asserted (`#[ignore = "a measurement"]`): the axes and phytomers either
//! holds that the other does not, each float field's largest and 99th
//! percentile relative and absolute difference over the matched rows, and each integer
//! field's rows that differ. One preset a run (`SETTLE_PRESET`, its
//! name; `SETTLE_AGE` and `SETTLE_SEED` where not its default age and
//! 42), the CPU's tree being several gigabytes at the oldest.
use super::columns::Cap;
use super::grown::{Kind, Snapshot, Table};
use super::grown_tests::{device, grown};
use super::settle_device::Kernels;
use super::settle_read::cpu_settled;
use super::settle_tests::settled;
use std::collections::HashMap;
use telperion_core::presets::Preset;
use telperion_space::SettleTo;

const PRESETS: [(Preset, &str); 8] = [
    (Preset::EuropeanBeech, "european-beech"),
    (Preset::OregonWhiteOak, "oregon-white-oak"),
    (Preset::NorwaySpruce, "norway-spruce"),
    (Preset::DatePalm, "date-palm"),
    (Preset::SilverBirch, "silver-birch"),
    (Preset::Ordinary, "ordinary"),
    (Preset::Laurelin, "laurelin"),
    (Preset::Telperion, "telperion"),
];

/// Each axis's lineage and which of that lineage's axes it is.
fn keys(lineage: &[u64]) -> Vec<(u64, u32)> {
    let mut seen: HashMap<u64, u32> = HashMap::new();
    lineage
        .iter()
        .map(|&l| {
            let k = seen.entry(l).or_insert(0);
            *k += 1;
            (l, *k)
        })
        .collect()
}

/// Each axis's first row and rows in a table kept in axis order.
fn rows_by_axis(t: &Table) -> HashMap<u32, (usize, usize)> {
    let mut out: HashMap<u32, (usize, usize)> = HashMap::new();
    for (r, &a) in t.axis.iter().enumerate() {
        out.entry(a).and_modify(|e| e.1 += 1).or_insert((r, 1));
    }
    out
}

fn value(kind: Kind, w: &[u32]) -> f64 {
    match kind {
        Kind::F32 => f64::from(f32::from_bits(w[0])),
        _ => f64::from_bits(u64::from(w[0]) | u64::from(w[1]) << 32),
    }
}

/// `t`'s fields over the row pairs: a float's rows, those that differ,
/// its largest and 99th percentile relative difference; an integer's
/// rows and those that differ.
fn report(table: &str, d: &Table, c: &Table, pairs: &[(usize, usize)]) {
    for (fd, fc) in d.fields.iter().zip(&c.fields) {
        let w = fd.kind.words();
        let mut rel = Vec::with_capacity(pairs.len());
        let mut abs = Vec::with_capacity(pairs.len());
        let mut differ = 0;
        for &(i, j) in pairs {
            let (a, b) = (&fd.words[w * i..w * i + w], &fc.words[w * j..w * j + w]);
            differ += usize::from(a != b);
            if matches!(fd.kind, Kind::F32 | Kind::F64) {
                let (x, y) = (value(fd.kind, a), value(fd.kind, b));
                let scale = x.abs().max(y.abs());
                abs.push((x - y).abs());
                rel.push(if x == y {
                    0.0
                } else if scale.is_finite() && scale > 0.0 {
                    (x - y).abs() / scale
                } else {
                    1.0
                });
            }
        }
        if rel.is_empty() {
            println!(
                "FIELD {table} {:>22} integer {:>9} rows, {differ} differ",
                fd.name,
                pairs.len()
            );
            continue;
        }
        let tail = |v: &mut Vec<f64>| {
            v.sort_by(f64::total_cmp);
            (v[v.len() - 1], v[(v.len() * 99 / 100).min(v.len() - 1)])
        };
        let ((max, p99), (amax, ap99)) = (tail(&mut rel), tail(&mut abs));
        println!(
            "FIELD {table} {:>22} float {:>9} rows, {differ} differ, relative max {max:.3e} p99 {p99:.3e}, absolute max {amax:.3e} p99 {ap99:.3e}",
            fd.name, pairs.len()
        );
    }
}

/// The two laid trees by lineage, reported.
fn compared(label: &str, dev: &Snapshot, cpu: &Snapshot) {
    let at: HashMap<(u64, u32), usize> = keys(&cpu.lineage)
        .into_iter()
        .enumerate()
        .map(|(i, l)| (l, i))
        .collect();
    let pairs: Vec<(usize, usize)> = keys(&dev.lineage)
        .iter()
        .enumerate()
        .filter_map(|(i, l)| at.get(l).map(|&j| (i, j)))
        .collect();
    println!(
        "{label}: axes device {} cpu {}, unmatched device {} cpu {}",
        dev.lineage.len(),
        cpu.lineage.len(),
        dev.lineage.len() - pairs.len(),
        cpu.lineage.len() - pairs.len()
    );
    report("axes", &dev.axes, &cpu.axes, &pairs);
    for (name, d, c) in [
        ("units", &dev.units, &cpu.units),
        ("phytomers", &dev.phyts, &cpu.phyts),
        ("pencils", &dev.pencils, &cpu.pencils),
    ] {
        let (rd, rc) = (rows_by_axis(d), rows_by_axis(c));
        let (mut rows, mut short) = (Vec::new(), 0);
        let total = |t: &Table| t.axis.len();
        for &(i, j) in &pairs {
            let a = rd.get(&(i as u32)).copied().unwrap_or((0, 0));
            let b = rc.get(&(j as u32)).copied().unwrap_or((0, 0));
            short += a.1.abs_diff(b.1);
            rows.extend((0..a.1.min(b.1)).map(|k| (a.0 + k, b.0 + k)));
        }
        println!(
            "{label}: {name} device {} cpu {}, matched {}, unmatched device {} cpu {} ({short} of them on matched axes)",
            total(d), total(c), rows.len(), total(d) - rows.len(), total(c) - rows.len()
        );
        report(name, d, c, &rows);
    }
}

#[test]
#[ignore = "a measurement"]
fn default_ages_laid_plain_by_lineage() {
    let _held = super::super::whole_tree();
    let Ok(only) = std::env::var("SETTLE_PRESET") else {
        println!("SETTLE_PRESET names the preset");
        return;
    };
    let Some((gpu, kit)) = device() else {
        return;
    };
    let k = Kernels::new(&gpu);
    for (preset, name) in PRESETS.into_iter().filter(|p| p.1 == only) {
        let given = |k: &str| std::env::var(k).ok().and_then(|v| v.parse().ok());
        let (age, seed) = (
            given("SETTLE_AGE").unwrap_or(preset.parameters().age as u32),
            given("SETTLE_SEED").unwrap_or(42),
        );
        let g = grown(preset, age, u64::from(seed));
        let (b, s, dev, beside) = settled(&gpu, (&kit, &k), &g, SettleTo::Laid);
        drop((s, b));
        let capture =
            telperion_space::grow_capture_settled_at(&g.species, g.request, SettleTo::Laid)
                .expect("the CPU's tree");
        let cpu = cpu_settled(&Cap(&capture), SettleTo::Laid);
        drop(capture);
        let mean = |v: &[u64]| v.get(1).map(|&m| f64::from_bits(m));
        let (d, c) = (mean(&beside.leaf_mean), mean(&cpu.1.leaf_mean));
        println!("{name} {age} seed {seed}: leaf mean device {d:?} cpu {c:?}");
        compared(&format!("{name} {age} seed {seed}"), &dev, &cpu.0);
    }
}
