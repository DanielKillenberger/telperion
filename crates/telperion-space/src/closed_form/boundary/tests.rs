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
/// level, the palm's holding.
const PINNED: [(&str, u32, u64); 40] = [
    ("beech", 1, 0x47f411f2dbf2035e),
    ("beech", 2, 0x301aa5429ed7b286),
    ("beech", 3, 0x796c3af822113e02),
    ("beech", 5, 0x0b826e702aef527a),
    ("beech", 8, 0xe477e89727289c01),
    ("beech", 13, 0x58c8d1f71470851e),
    ("beech", 21, 0x8988aa11b829e0aa),
    ("beech", 34, 0x27dbc368f2d27c6c),
    ("beech", 55, 0x27dfb9e33835ee3d),
    ("beech", 80, 0xcba5cfde9d8b79b1),
    ("spruce", 1, 0xd237b64de0045cbb),
    ("spruce", 2, 0xdcd92aa357638f82),
    ("spruce", 3, 0x8aff78070a417e48),
    ("spruce", 5, 0xa41a1277de5fb838),
    ("spruce", 8, 0xc743d61c5fd2ff97),
    ("spruce", 13, 0x610450c7c1e36b6e),
    ("spruce", 21, 0x90499d2709d9cea6),
    ("spruce", 34, 0xd8cc06b282db064f),
    ("spruce", 55, 0xce07d27f7121dda6),
    ("spruce", 80, 0x07b0d433934ea534),
    ("oak", 1, 0xb1510c22c30ba70d),
    ("oak", 2, 0x884ce027d783b35b),
    ("oak", 3, 0x94cc9e1e53d5204f),
    ("oak", 5, 0x1768d54a1c98f9d7),
    ("oak", 8, 0xc66060b681ea8f70),
    ("oak", 13, 0xf68a26640039a2d8),
    ("oak", 21, 0xbe16c39fad170799),
    ("oak", 34, 0xda28ee387c3fbb8c),
    ("oak", 55, 0x4fe8ba39077c82ee),
    ("oak", 80, 0x3dbb3bbdf7d8c58b),
    ("palm", 1, 0xda237154468f69ae),
    ("palm", 2, 0xf30b6b627d772f9e),
    ("palm", 3, 0xa76f80968375a52b),
    ("palm", 5, 0x61037bbb2251841a),
    ("palm", 8, 0xaf0699f75d0e1787),
    ("palm", 13, 0x9dc19dc3331b5f83),
    ("palm", 21, 0x06a4b228ae4c227f),
    ("palm", 34, 0x5390f7d1673f40c9),
    ("palm", 55, 0x1ac5e4768edbdbd6),
    ("palm", 80, 0x6f5bf58ab70abd20),
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
    for (name, age) in ["spruce", "palm"].iter().flat_map(|n| [(n, 40), (n, 80)]) {
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

/// Every field of every row and curve, to the bit.
fn whole(rows: &Rows) -> String {
    format!(
        "{:?} {:?} {:?} {}",
        rows.rows, rows.curves, rows.seed, rows.age
    )
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
