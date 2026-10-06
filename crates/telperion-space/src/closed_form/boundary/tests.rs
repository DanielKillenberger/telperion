//! The rows pinned bit for bit, four species at ages spread to 80; and
//! against the values the kept tables read on 9f063433, before each bud's
//! table came from its first unit and the rest of its axis (fn-209, P1).
use super::{Row, Rows};
use crate::{beech, oak, palm, spruce, Species};

/// Each species and age, and the hash of its rows' bits as each bud's
/// table is read from its first unit and the rest of its axis (fn-209,
/// P1 round 2).
const PINNED: [(&str, u32, u64); 40] = [
    ("beech", 1, 0x6c6d014abadb96ac),
    ("beech", 2, 0x3e13297b503060e0),
    ("beech", 3, 0x5e1b7e53e0921178),
    ("beech", 5, 0xb1a3789f67ee4b5a),
    ("beech", 8, 0x84e4d7bbbce190c4),
    ("beech", 13, 0xd259c2cb57e84431),
    ("beech", 21, 0x8c80cb094d91cc3f),
    ("beech", 34, 0x214579d5d3dd5d82),
    ("beech", 55, 0x1d066e037079968d),
    ("beech", 80, 0xa98dc8cc8cbd9c27),
    ("spruce", 1, 0x0a0b4289900822d9),
    ("spruce", 2, 0x5289e752724f26a7),
    ("spruce", 3, 0xcf94d4731d5dd17b),
    ("spruce", 5, 0x752f51e07e87e61a),
    ("spruce", 8, 0xe9dc8f81af417c31),
    ("spruce", 13, 0xb9cb3a5a9a9176cf),
    ("spruce", 21, 0xeccbeaea16912870),
    ("spruce", 34, 0xc9ae9569ff738158),
    ("spruce", 55, 0xb171fcc1e07e8501),
    ("spruce", 80, 0x2c685ece967d2594),
    ("oak", 1, 0x06db4fdc944ba9a4),
    ("oak", 2, 0x01c6bf115e5f9035),
    ("oak", 3, 0x45dfbcd84a19a148),
    ("oak", 5, 0x5dbee4d7a8b7be82),
    ("oak", 8, 0xe93861a49c45481b),
    ("oak", 13, 0x92a88c6a7c572b0d),
    ("oak", 21, 0x839c44a3afe865c6),
    ("oak", 34, 0x062b95803d31b9e4),
    ("oak", 55, 0x377e1b3e2eba989c),
    ("oak", 80, 0x9b935ce8e1ebcb09),
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
    for (name, age) in ["beech", "spruce", "oak", "palm"]
        .iter()
        .flat_map(|n| [(n, 40), (n, 80)])
    {
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
