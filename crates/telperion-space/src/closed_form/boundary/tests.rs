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
const PINNED: [(&str, u32, u64); 40] = [
    ("beech", 1, 0x744ad9d4c38baff6),
    ("beech", 2, 0xd9f87b741891c18c),
    ("beech", 3, 0xd3d7e320ab50986a),
    ("beech", 5, 0xa25780f4606665e1),
    ("beech", 8, 0x43868d5a16f4f5e6),
    ("beech", 13, 0xc15c3e16c911ad80),
    ("beech", 21, 0x2e6e97941a820b8f),
    ("beech", 34, 0x25d4b089f88103ef),
    ("beech", 55, 0x7fbaa3b3b032c06d),
    ("beech", 80, 0xb2611e6c03bc5cd5),
    ("spruce", 1, 0xac6954d6503540ef),
    ("spruce", 2, 0x888227367b6eebbf),
    ("spruce", 3, 0x7c582f520c81ba87),
    ("spruce", 5, 0xa3aaad14e39da2f6),
    ("spruce", 8, 0xe2ae9b3d587cb8cc),
    ("spruce", 13, 0x43f9877dcae3d07c),
    ("spruce", 21, 0xb25b6c20092f8cd4),
    ("spruce", 34, 0x8591291469644b73),
    ("spruce", 55, 0xd8d66197922ef708),
    ("spruce", 80, 0xa453b49bcdabd3a7),
    ("oak", 1, 0x85a24b1a6d792878),
    ("oak", 2, 0xc8a959d1e850b247),
    ("oak", 3, 0x8f9a1180135f3a6e),
    ("oak", 5, 0xcd76f1424072db54),
    ("oak", 8, 0x4acc0e62c26bf48a),
    ("oak", 13, 0x0e9f262e999e438f),
    ("oak", 21, 0x5a3f45e2a91bd524),
    ("oak", 34, 0xe842d2ff6a339d4e),
    ("oak", 55, 0xc0327a273d354879),
    ("oak", 80, 0xa647e4d74297988f),
    ("palm", 1, 0x0ce2f02f7186f784),
    ("palm", 2, 0x5aab3ecd4c102ab4),
    ("palm", 3, 0x2373698dcb356a49),
    ("palm", 5, 0x1c8c78a14b8ad458),
    ("palm", 8, 0x29f142d42c6555a2),
    ("palm", 13, 0x13939ec283bdf95f),
    ("palm", 21, 0xaf09a40e56020f89),
    ("palm", 34, 0xbb9c3490b560c4ab),
    ("palm", 55, 0x0c0c45ae81a6006b),
    ("palm", 80, 0x0b6b03e0e9395181),
];

/// Each row's fields as their bits, the seed's last.
fn bits(rows: &Rows) -> Vec<[u64; 9]> {
    let field = |r: &Row| {
        [
            r.kept, r.s1, r.s2, r.grown1, r.grown2, r.mass, r.moment, r.reach, r.e,
        ]
        .map(f64::to_bits)
    };
    let mut out: Vec<_> = rows.rows.iter().map(field).collect();
    out.push(field(&rows.seed));
    out
}

/// FNV-1a over the bits, a pin that names a change without holding it.
fn hash(bits: &[[u64; 9]]) -> u64 {
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
/// the bits of kept, E[S; kept], E[S²; kept], mass, moment and reach.
const OLD: &str = include_str!("rows-9f063433.txt");

/// Each field's largest relative difference from the old rows that this
/// reading may make. The chance kept, the section and the reach are the
/// same recursion summed in another order. Mass and moment take each
/// unit's section beyond it from the rest of the axis as one mixture,
/// where 9f063433 took it end by end, and a failed relay's section at the
/// rest's base rather than at its node: the radius's power of a mixture,
/// measured at most 5.5e-3 (beech's moment at 80).
const TOLERANCE: [(&str, f64); 6] = [
    ("kept", 1e-9),
    ("s1", 1e-9),
    ("s2", 1e-9),
    ("mass", 1e-2),
    ("moment", 1e-2),
    ("reach", 1e-9),
];

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
            for (q, field) in [0, 1, 2, 5, 6, 7].into_iter().enumerate() {
                let o = f64::from_bits(u64::from_str_radix(w[3 + q], 16).unwrap());
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
            let (field, tolerance) = TOLERANCE[q];
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
