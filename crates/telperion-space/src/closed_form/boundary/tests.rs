//! The rows pinned bit for bit, four species at ages spread to 80; and
//! against the values the kept tables read on 9f063433, before each bud's
//! table came from its first unit and the rest of its axis (fn-209, P1).
use super::{Row, Rows};
use crate::{beech, oak, palm, spruce, Species};

/// Each species and age, and the hash of its rows' bits as each bud's
/// table is read from its first unit and the rest of its axis (fn-209,
/// P1 round 2); re-pinned when the engine took the pinned libm (fn-198
/// task 8): the bits move at rounding, and the rows stay near the old
/// reading (the next test); re-pinned for the beech and the oak by fn-199
/// (R1's lived share and relays, C1's abortion after it, rule 2's stand),
/// the spruce and the palm holding to the bit; re-taken once at the F2b
/// merge (fn-198): contract A's ripe and the decision contract's
/// polynomial logs move the beech's, spruce's and oak's rows at rounding
/// level, the palm's holding; re-pinned by fn-254: the growth contract in
/// f32 moves 7,734 of 125,640 fields, by at most 2.9e-7 relative (E[S²]
/// of the oak's first age), kept, reach and e to the bit.
// fn-248 re-pinned the beech's from age 5 on: its short shoots live four
// years, not three. Re-taken at fn-254's merge on its floats: those seven
// alone moved, every other row holding fn-254's f32 pin to the bit.
// fn-275 R2a re-pinned the palm's: its internode, 0.031 m to 0.0339 m,
// raises the stem by what its crown's establishment lowers it.
// fn-236 (2026-10-15) re-pinned the oak's: its young stem's laterals grow
// 0.031 m nodes, the sapling rate Hibbs and Yoder measured, not 0.07 m.
// fn-354 (2026-11-11) cut the rows to the chance kept, the phytomers and
// the reach when the pipe model's girth went: the pins are re-taken over
// those three fields. fn-354 R7 re-pinned the beech's from 2 years on: its
// tables hold full light, 85% of its long laterals' kind (Lemoine 2002).
const PINNED: [(&str, u32, u64); 40] = [
    ("beech", 1, 0xa2e130ba18028771),
    ("beech", 2, 0x03bc696d4cdfb040),
    ("beech", 3, 0x009517b81c6fa372),
    ("beech", 5, 0x613360fc6c404a9f),
    ("beech", 8, 0xe98b8a669dffbbc6),
    ("beech", 13, 0x7f2fc5431ef443db),
    ("beech", 21, 0x0a2f3002ce70f181),
    ("beech", 34, 0x45152ee12b0ea134),
    ("beech", 55, 0x4c2be3b2b7dfd2d1),
    ("beech", 80, 0x93f1f15db3c8025b),
    ("spruce", 1, 0x1bd7503c848b7e4e),
    ("spruce", 2, 0x63658628179a71ef),
    ("spruce", 3, 0x1e5ee6e850a43c37),
    ("spruce", 5, 0xef2d2ef62ec223e8),
    ("spruce", 8, 0x645aa5147e7101be),
    ("spruce", 13, 0xab4e5787746cd792),
    ("spruce", 21, 0x2f902f0a824dec1b),
    ("spruce", 34, 0xb83837f7f3106c9d),
    ("spruce", 55, 0xa9cab43f03d04d34),
    ("spruce", 80, 0x87064a4dbac8eca5),
    ("oak", 1, 0xe977ae2b69a0d744),
    ("oak", 2, 0x6e89997a17c69ecf),
    ("oak", 3, 0xf7b38ea083456564),
    ("oak", 5, 0x3ab37aac3931ef48),
    ("oak", 8, 0x9268783ae7b7e589),
    ("oak", 13, 0x5f17d42f54aaefaf),
    ("oak", 21, 0x0fb84f45740b210e),
    ("oak", 34, 0xc15b53bef7bd8c04),
    ("oak", 55, 0x195eba1890ff0358),
    ("oak", 80, 0x0616657716130d14),
    ("palm", 1, 0xe48e7cc1296888a8),
    ("palm", 2, 0xbe604c115631bda8),
    ("palm", 3, 0xde0e1c605fdc5598),
    ("palm", 5, 0x7fe3463534ba163d),
    ("palm", 8, 0x1856f757b027d525),
    ("palm", 13, 0x4c65739378feaa18),
    ("palm", 21, 0x7c9e2d76ea9888f6),
    ("palm", 34, 0xb7e2ac08a2e9fd99),
    ("palm", 55, 0xfc1bf47547e91c5e),
    ("palm", 80, 0xf7fae13e8d99d9e2),
];

/// Each row's fields as their bits, the seed's last.
fn bits(rows: &Rows) -> Vec<[u64; 3]> {
    let field = |r: &Row| [r.kept, r.count, r.reach].map(f64::to_bits);
    let mut out: Vec<_> = rows.rows.iter().map(field).collect();
    out.push(field(&rows.seed));
    out
}

/// FNV-1a over the bits, a pin that names a change without holding it.
fn hash(bits: &[[u64; 3]]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for word in bits.iter().flatten() {
        for byte in word.to_le_bytes() {
            h = (h ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3);
        }
    }
    h
}

fn species(name: &str) -> Species {
    match name {
        "beech" => beech(),
        "spruce" => spruce(),
        "oak" => oak(),
        _ => palm(),
    }
}

/// The pinned species and ages, each read on one core and on four; the
/// ones whose rows changed.
fn changed() -> Vec<String> {
    let mut out = Vec::new();
    for &(name, age, pin) in &PINNED {
        let species = species(name);
        for threads in [1, 4] {
            let got = hash(&bits(&Rows::new(&species, age, threads)));
            if got != pin {
                out.push(format!(
                    "{name} {age} on {threads}: {got:016x}, pinned {pin:016x}"
                ));
            }
        }
    }
    out
}

#[test]
fn the_rows_hold_their_values() {
    let changed = changed();
    assert!(changed.is_empty(), "rows changed: {changed:#?}");
}

/// The rows of 9f063433 at 40 and 80, each borne PA's at every eighth age,
/// its oldest and the seed (a sample that held the largest mass and moment
/// differences of every row for beech and spruce): species, age, row, and
/// the bits of kept, E[S; kept], E[S²; kept], mass, moment and reach, of
/// which the chance kept and the reach are still read.
const OLD: &str = include_str!("rows-9f063433.txt");

/// Each field's largest relative difference from the old rows that this
/// reading may make, by its column in the old rows and in `bits`: the
/// chance kept and the reach are the same recursion summed in another
/// order.
const TOLERANCE: [(&str, usize, usize, f64); 2] = [("kept", 0, 0, 1e-9), ("reach", 5, 2, 1e-9)];

/// The rows stay within the tolerance of 9f063433's, field by field;
/// prints the largest and the 95th percentile of each.
#[test]
fn the_rows_stay_near_the_old_reading() {
    let mut worst = Vec::new();
    // The beech and the oak relay their failed units, whose law fn-199
    // changed (R1's lived share, C1, rule 2'): their rows are no longer
    // 9f063433's reading summed another way, and are held by their pins.
    // The palm's internode moved (fn-275 R2a, 0.031 m to 0.0339 m): its
    // mass and reach scale by the length, its moment by its square, so it
    // too is held by its pins.
    for (name, age) in [("spruce", 40), ("spruce", 80)] {
        let new = bits(&Rows::new(&species(name), age, 1));
        let mut d = vec![Vec::new(); TOLERANCE.len()];
        let prefix = format!("{name} {age} ");
        for line in OLD.lines().filter(|l| l.starts_with(&prefix)) {
            let w: Vec<&str> = line.split(' ').collect();
            let row = &new[w[2].parse::<usize>().unwrap()];
            for (q, &(_, old, field, _)) in TOLERANCE.iter().enumerate() {
                let o = f64::from_bits(u64::from_str_radix(w[3 + old], 16).unwrap());
                let n = f64::from_bits(row[field]);
                let scale = o.abs().max(n.abs());
                d[q].push(if scale > 0.0 {
                    (n - o).abs() / scale
                } else {
                    0.0
                });
            }
        }
        assert!(!d[0].is_empty(), "{name} {age}: no old rows");
        let mut line = format!("{name} {age}:");
        for (q, d) in d.iter_mut().enumerate() {
            d.sort_by(f64::total_cmp);
            let (max, p95) = (d[d.len() - 1], d[(d.len() * 95 / 100).min(d.len() - 1)]);
            let (field, _, _, tolerance) = TOLERANCE[q];
            line += &format!(" {field} {max:.1e}/{p95:.1e}");
            if max > tolerance {
                worst.push(format!("{name} {age} {field} {max:.2e}"));
            }
        }
        println!("{line}");
    }
    assert!(worst.is_empty(), "past the tolerance: {worst:#?}");
}

/// Writes every row's bits to `TELPERION_ROWS_DUMP` (a directory), one
/// file per species and age, to find which row a change moved; prints
/// each one's pin.
#[test]
#[ignore = "writes the rows for a comparison by hand"]
fn dump_rows() {
    let dir = std::env::var("TELPERION_ROWS_DUMP").expect("a directory");
    for &(name, age, _) in &PINNED {
        let rows = Rows::new(&species(name), age, 1);
        let text: Vec<String> = bits(&rows)
            .iter()
            .map(|b| b.map(|w| format!("{w:016x}")).join(" "))
            .collect();
        let path = format!("{dir}/{name}-{age}.txt");
        std::fs::write(&path, text.join("\n")).expect("written");
        println!("    (\"{name}\", {age}, 0x{:016x}),", hash(&bits(&rows)));
    }
}

/// Every field of every row, to the bit.
fn whole(rows: &Rows) -> String {
    format!("{:?} {:?} {}", rows.rows, rows.seed, rows.age)
}

/// fn-249 R1: a shelf reads a species' rows once per age and mode, the
/// same rows a fresh read gives, and keeps the most recently used.
#[test]
fn a_shelf_reads_rows_once_and_keeps_the_latest() {
    use super::Shelf;
    use std::sync::Arc;
    let oak = oak();
    let mut varied = oak.clone();
    varied.states[0].viability = f64::from_bits(oak.states[0].viability.to_bits() - 1);
    let shelf = Shelf::new(3);
    let first = shelf.rows(&oak, 9, 1, false);
    assert!(Arc::ptr_eq(&first, &shelf.rows(&oak, 9, 4, false)));
    assert_eq!(whole(&first), whole(&Rows::laterals(&oak, 9, 1)));
    let seeded = shelf.rows(&oak, 9, 1, true);
    assert_eq!(whole(&seeded), whole(&Rows::new(&oak, 9, 1)));
    for other in [&seeded, &shelf.rows(&varied, 9, 1, false)] {
        assert!(!Arc::ptr_eq(&first, other));
    }
    // A fourth key evicts the least recently used: `seeded`, not `first`,
    // read again since.
    assert!(Arc::ptr_eq(&first, &shelf.rows(&oak, 9, 1, false)));
    let older = shelf.rows(&oak, 8, 1, false);
    assert!(Arc::ptr_eq(&older, &shelf.rows(&oak, 8, 1, false)));
    assert!(Arc::ptr_eq(&first, &shelf.rows(&oak, 9, 1, false)));
    assert!(!Arc::ptr_eq(&seeded, &shelf.rows(&oak, 9, 1, true)));
}
