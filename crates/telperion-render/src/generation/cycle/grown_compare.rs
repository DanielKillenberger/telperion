//! The device's grown state held to the CPU's: every word exactly, and
//! by lineage, axes matched by their lineage's key, their columns, units,
//! phytomers and pencils compared row by row, a float by its relative
//! difference (COUNCIL.md's agreement standard).
use super::compare::Agreement;
use super::grown::{Kind, Snapshot, Table};
use std::collections::HashMap;

/// Every word of every column, exactly.
pub(super) fn exact(dev: &Snapshot, cpu: &Snapshot) -> Agreement {
    let mut a = Agreement::default();
    a.eq("phytomers grown", &[dev.grown], &[cpu.grown]);
    for (d, c) in tables(dev).into_iter().zip(tables(cpu)) {
        for (fd, fc) in d.fields.iter().zip(&c.fields) {
            a.eq(&fd.name, &fd.words, &fc.words);
        }
    }
    a
}

fn tables(s: &Snapshot) -> [&Table; 7] {
    [
        &s.axes,
        &s.units,
        &s.phyts,
        &s.sleepers,
        &s.woken,
        &s.live,
        &s.pencils,
    ]
}

/// A field compared by lineage: its kind, the rows compared, those that
/// differ, the largest relative and absolute difference, and the largest
/// magnitude of the CPU's values.
#[derive(Debug, Clone, Default)]
pub(super) struct Matched {
    pub name: String,
    pub kind: Option<Kind>,
    pub rows: usize,
    pub differ: usize,
    pub worst: f64,
    pub apart: f64,
    pub scale: f64,
}

/// The two trees by lineage: the axes either holds that the other does
/// not, and every compared field.
#[derive(Debug, Default)]
pub(super) struct ByLineage {
    pub axes: (usize, usize),
    pub unmatched: (usize, usize),
    pub fields: Vec<Matched>,
}

impl ByLineage {
    pub(super) fn report(&self) -> String {
        let mut out = format!(
            "axes device {} cpu {}; unmatched device {} cpu {}\n",
            self.axes.0, self.axes.1, self.unmatched.0, self.unmatched.1
        );
        for m in self.fields.iter().filter(|m| m.differ > 0) {
            out += &format!(
                "{:>26} {:>10} rows {:>8} differ, worst relative {:.3e}\n",
                m.name, m.rows, m.differ, m.worst
            );
        }
        out
    }
}

/// A field's word as a number.
fn value(kind: Kind, a: &[u32]) -> f64 {
    match kind {
        Kind::F32 => f64::from(f32::from_bits(a[0])),
        Kind::F64 => f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32),
        Kind::U32 => f64::from(a[0]),
        Kind::U64 => (u64::from(a[0]) | u64::from(a[1]) << 32) as f64,
    }
}

fn relative(kind: Kind, a: &[u32], b: &[u32]) -> f64 {
    let (x, y) = match kind {
        Kind::F32 | Kind::F64 => (value(kind, a), value(kind, b)),
        _ => return if a == b { 0.0 } else { 1.0 },
    };
    if x == y {
        return 0.0;
    }
    let scale = x.abs().max(y.abs());
    if scale.is_finite() && scale > 0.0 {
        (x - y).abs() / scale
    } else {
        1.0
    }
}

/// Each axis's rows in a table kept in axis order: its first and count.
fn rows_by_axis(t: &Table) -> HashMap<u32, (usize, usize)> {
    let mut out: HashMap<u32, (usize, usize)> = HashMap::new();
    for (r, &a) in t.axis.iter().enumerate() {
        out.entry(a).and_modify(|e| e.1 += 1).or_insert((r, 1));
    }
    out
}

/// `t`'s fields compared over the row pairs `pairs`.
fn fields(d: &Table, c: &Table, pairs: &[(usize, usize)], out: &mut Vec<Matched>) {
    for (fd, fc) in d.fields.iter().zip(&c.fields) {
        let w = fd.kind.words();
        let mut m = Matched {
            name: fd.name.clone(),
            kind: Some(fd.kind),
            ..Matched::default()
        };
        for &(i, j) in pairs {
            let (a, b) = (&fd.words[w * i..w * i + w], &fc.words[w * j..w * j + w]);
            m.rows += 1;
            m.scale = m.scale.max(value(fd.kind, b).abs());
            if a != b {
                m.differ += 1;
                m.worst = m.worst.max(relative(fd.kind, a, b));
                m.apart = m.apart.max((value(fd.kind, a) - value(fd.kind, b)).abs());
            }
        }
        out.push(m);
    }
}

/// Each axis's lineage and which of the axes of that lineage it is (an
/// age a bud slept through carries its bud's).
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

/// The two trees matched by lineage.
pub(super) fn by_lineage(dev: &Snapshot, cpu: &Snapshot) -> ByLineage {
    let at: HashMap<(u64, u32), usize> = keys(&cpu.lineage)
        .into_iter()
        .enumerate()
        .map(|(i, l)| (l, i))
        .collect();
    let pairs: Vec<(usize, usize)> = (keys(&dev.lineage).iter().enumerate())
        .filter_map(|(i, l)| at.get(l).map(|&j| (i, j)))
        .collect();
    let mut out = ByLineage {
        axes: (dev.lineage.len(), cpu.lineage.len()),
        unmatched: (
            dev.lineage.len() - pairs.len(),
            cpu.lineage.len() - pairs.len(),
        ),
        fields: Vec::new(),
    };
    fields(&dev.axes, &cpu.axes, &pairs, &mut out.fields);
    for (d, c) in [
        (&dev.units, &cpu.units),
        (&dev.phyts, &cpu.phyts),
        (&dev.pencils, &cpu.pencils),
    ] {
        let (rd, rc) = (rows_by_axis(d), rows_by_axis(c));
        let mut rows = Vec::new();
        let mut short = 0;
        for &(i, j) in &pairs {
            let (a, b) = (rd.get(&(i as u32)), rc.get(&(j as u32)));
            let (a, b) = (a.copied().unwrap_or((0, 0)), b.copied().unwrap_or((0, 0)));
            short += a.1.abs_diff(b.1);
            rows.extend((0..a.1.min(b.1)).map(|k| (a.0 + k, b.0 + k)));
        }
        fields(d, c, &rows, &mut out.fields);
        let name = d.fields.first().map_or("rows", |f| f.name.as_str());
        out.fields.push(Matched {
            name: format!("{name}: rows unmatched"),
            rows: rows.len(),
            differ: short,
            ..Matched::default()
        });
    }
    out
}
