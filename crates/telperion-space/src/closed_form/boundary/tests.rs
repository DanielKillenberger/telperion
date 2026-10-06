//! The rows against the values the kept tables read before each table
//! was cut to the cycles its readers reach (fn-209, probe P1): every field
//! of every row, bit for bit, for the four species at ages spread to 80.
use super::{Row, Rows};
use crate::{beech, oak, palm, spruce, Species};

/// Each species and age, and the hash of its rows' bits as the kept tables
/// read them on 9f063433, every table over every cycle.
const PINNED: [(&str, u32, u64); 40] = [
    ("beech", 1, 0x6c6d014abadb96ac),
    ("beech", 2, 0x39d52260151f6314),
    ("beech", 3, 0xdc728319648c796b),
    ("beech", 5, 0x78ad08cc4d5f2873),
    ("beech", 8, 0x53cc52dff0b8bdd3),
    ("beech", 13, 0x7926e2d9ecd41154),
    ("beech", 21, 0x9e22b50c6b81e4e4),
    ("beech", 34, 0x8adb52ba3088d894),
    ("beech", 55, 0x514dbee5a2c94a34),
    ("beech", 80, 0x570d8dabe93aa3c5),
    ("spruce", 1, 0x0a0b4289900822d9),
    ("spruce", 2, 0x7f33d3ea05952863),
    ("spruce", 3, 0xff36787434c8b78b),
    ("spruce", 5, 0x83a47ea06d5dbc9d),
    ("spruce", 8, 0x950b50f62a6fd7ff),
    ("spruce", 13, 0x1fa7557f9810752a),
    ("spruce", 21, 0xca5b9dcfd48c8d28),
    ("spruce", 34, 0xf94fafd7320730c6),
    ("spruce", 55, 0x4070a2a082d4643f),
    ("spruce", 80, 0xb833128b42eef75a),
    ("oak", 1, 0x06db4fdc944ba9a4),
    ("oak", 2, 0x859b9f58e38002e2),
    ("oak", 3, 0x91fbc8359826c982),
    ("oak", 5, 0x0a27df9f5ea0f8d9),
    ("oak", 8, 0x1359f6a6d7075a34),
    ("oak", 13, 0xd1a49359ad47ac27),
    ("oak", 21, 0x6d2c89804fa91931),
    ("oak", 34, 0x00c6b2b080b6c5d7),
    ("oak", 55, 0x12402a8f0a45e608),
    ("oak", 80, 0xd9c80fc3c1bde792),
    ("palm", 1, 0xda237154468f69ae),
    ("palm", 2, 0xf30b6b627d772f9e),
    ("palm", 3, 0x306e87bf6255eac4),
    ("palm", 5, 0x61037bbb2251841a),
    ("palm", 8, 0xe3f2162b79340d98),
    ("palm", 13, 0x9dc19dc3331b5f83),
    ("palm", 21, 0xac31146daffc2c2c),
    ("palm", 34, 0x5ec581e2a90b17af),
    ("palm", 55, 0xdd8abb84177c7f98),
    ("palm", 80, 0xc6b1ddd546261318),
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

/// The pinned species and ages up to `most`, each read on one core and
/// on four; the ones whose rows changed.
fn changed(most: u32) -> Vec<String> {
    let pinned = PINNED.iter().filter(|p| p.1 <= most);
    let mut out = Vec::new();
    for &(name, age, pin) in pinned {
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

/// The rows hold their values to the bit at ages to 34 (about a second).
#[test]
fn the_rows_hold_their_values() {
    let changed = changed(34);
    assert!(changed.is_empty(), "rows changed: {changed:#?}");
}

/// And at 55 and 80, the ages that cost (about a minute, on one core and on four).
#[test]
#[ignore = "about a minute; run when the kept tables change"]
fn the_rows_hold_their_values_to_80() {
    let changed = changed(80);
    assert!(changed.is_empty(), "rows changed: {changed:#?}");
}

/// Writes every row's bits to `TELPERION_ROWS_DUMP` (a directory), one
/// file per species and age, to find which row a change moved.
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
    }
}
