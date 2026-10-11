//! The light's f64 steps as short programs (`light_vm.wgsl`), each a
//! list of operations on a file of f64 registers run by one loop, so
//! every software operation has one call site an entry and the driver
//! inlines each once (fn-235, UNITS.md). The sweep is four programs:
//! a direction's bilinear weights at each column, each node's path term
//! from the densities, the depth recurrence down the layers (the one
//! sequential part, 8 operations a node), and the light, `exp` and the
//! directions' sum. The integer steps between them (floors, loads,
//! `exp`'s cases, stores) are hooks. Each list is the CPU's own order:
//! `light.rs::swept` and `bilinear`, libm 0.2.16's `exp`, and
//! `field.rs::corners` in f32, each f32 operation taken exactly in f64
//! and rounded to f32 once.
use std::fmt::Write;

// Operations; a hook from `HOOK` on.
const ADD: u32 = 0;
const SUB: u32 = 1;
const MUL: u32 = 2;
const DIV: u32 = 3;
/// Rounded to f32 and widened back: an f32 operation's one rounding.
const R32: u32 = 4;
const HOOK: u32 = 8;

/// Registers, each one f64, and the hooks, as the WGSL names them.
const NAMES: &[&str] = &[
    // Constants, loaded once.
    "ONE", "TWO", "HALF", "INVLN2", "LN2HI", "LN2LO", "P1", "P2", "P3", "P4", "P5",
    // A direction's reals, and a node's.
    "OX", "OY", "EP", "SHARE", "I", "J", "RHO", "X", "Y", "FX", "FY", "TX", "TY", "UX", "UY", "W0",
    "W1", "W2", "W3", "D0", "D1", "D2", "D3", "V0", "V1", "V2", "V3", "T", "DS", "RS", "A", "DEP",
    "SUM", // `exp`.
    "XN", "HALFS", "KT", "KF", "M", "HI", "LO", "XR", "XX", "Q", "CC", "XC", "DC", "Z", "YY",
    "TINY", "E", "S1", "S2", "S3", "L",
];
/// The corner weights' registers alias the sweep's: no entry runs both.
const CORNER_NAMES: &[&str] = &[
    "S_0", "S_1", "S_2", "N_0", "N_1", "N_2", "F_0", "F_1", "F_2", "G_0", "G_1", "G_2", "PR",
    "WC0", "WC1", "WC2", "WC3", "WC4", "WC5", "WC6", "WC7",
];
const HOOKS: &[&str] = &[
    "H_COLUMN",
    "H_STORE_W",
    "H_LOAD_PATH",
    "H_STORE_A",
    "H_LOAD_DEPTH",
    "H_STORE_DEP",
    "H_SIGN",
    "H_K",
    "H_SCALE",
    "H_FLOOR",
];
/// Where the corner weights' registers begin.
const CORNERS_AT: usize = 11;

fn reg(name: &str) -> u32 {
    let at = |list: &[&str]| list.iter().position(|n| *n == name);
    match (at(NAMES), at(CORNER_NAMES)) {
        (Some(i), _) => i as u32,
        (None, Some(i)) => (CORNERS_AT + i) as u32,
        _ => panic!("no register {name}"),
    }
}

fn hook(name: &str) -> u32 {
    HOOK + HOOKS.iter().position(|n| *n == name).expect("a hook") as u32
}

/// One instruction's word: operation, destination, operands.
fn op(code: u32, d: &str, a: &str, b: &str) -> u32 {
    code | reg(d) << 8 | reg(a) << 16 | reg(b) << 24
}

/// `swept`'s bilinear weights at a column of a direction's layers, the
/// same at every layer: (dx, dy) in (0,0), (0,1), (1,0), (1,1).
fn weights() -> Vec<u32> {
    vec![
        op(ADD, "X", "I", "OX"),
        op(ADD, "Y", "J", "OY"),
        hook("H_COLUMN"),
        op(SUB, "TX", "X", "FX"),
        op(SUB, "TY", "Y", "FY"),
        op(SUB, "UX", "ONE", "TX"),
        op(SUB, "UY", "ONE", "TY"),
        op(MUL, "W0", "UX", "UY"),
        op(MUL, "W1", "UX", "TY"),
        op(MUL, "W2", "TX", "UY"),
        op(MUL, "W3", "TX", "TY"),
        hook("H_STORE_W"),
    ]
}

/// `bilinear`'s sum of the four `v`s into `sum`, in its order.
fn bilinear(sum: &str, v: [&str; 4]) -> Vec<u32> {
    let mut p = vec![op(MUL, sum, "W0", v[0])];
    for (w, v) in ["W1", "W2", "W3"].iter().zip(&v[1..]) {
        p.extend([op(MUL, "T", w, v), op(ADD, sum, sum, "T")]);
    }
    p
}

/// A node's path term, `extinction path (rho + rho0) / 2`, the
/// extinction's path taken on the host as the CPU takes it: it reads no
/// depth, so every node takes it at once.
fn path() -> Vec<u32> {
    let mut p = vec![hook("H_LOAD_PATH")];
    p.extend(bilinear("RS", ["V0", "V1", "V2", "V3"]));
    p.extend([
        op(ADD, "A", "RHO", "RS"),
        op(MUL, "A", "EP", "A"),
        op(MUL, "A", "A", "HALF"),
        hook("H_STORE_A"),
    ]);
    p
}

/// A node's depth, `d0 + path term`, `d0` read bilinearly in the layer
/// above's depths: the recurrence.
fn depth() -> Vec<u32> {
    let mut p = vec![hook("H_LOAD_DEPTH")];
    p.extend(bilinear("DS", ["D0", "D1", "D2", "D3"]));
    p.extend([op(ADD, "DEP", "DS", "A"), hook("H_STORE_DEP")]);
    p
}

/// A direction's light at a node, `share exp(XN)`, added to the sum
/// over the directions, `SUM`, in their order.
fn shade() -> Vec<u32> {
    let mut p = exp();
    p.push(op(ADD, "SUM", "SUM", "L"));
    p
}

/// `share exp(XN)`, libm 0.2.16's `exp` with its cases in the hooks.
fn exp() -> Vec<u32> {
    let mut p = vec![
        hook("H_SIGN"),
        op(MUL, "KT", "INVLN2", "XN"),
        op(ADD, "KT", "KT", "HALFS"),
        hook("H_K"),
        op(MUL, "M", "KF", "LN2HI"),
        op(SUB, "HI", "XN", "M"),
        op(MUL, "LO", "KF", "LN2LO"),
        op(SUB, "XR", "HI", "LO"),
        op(MUL, "XX", "XR", "XR"),
        op(MUL, "Q", "XX", "P5"),
    ];
    for c in ["P4", "P3", "P2", "P1"] {
        p.extend([op(ADD, "Q", c, "Q"), op(MUL, "Q", "XX", "Q")]);
    }
    p.extend([
        op(SUB, "CC", "XR", "Q"),
        op(MUL, "XC", "XR", "CC"),
        op(SUB, "DC", "TWO", "CC"),
        op(DIV, "Z", "XC", "DC"),
        op(SUB, "Z", "Z", "LO"),
        op(ADD, "Z", "Z", "HI"),
        op(ADD, "YY", "ONE", "Z"),
        op(ADD, "TINY", "ONE", "XN"),
        hook("H_SCALE"),
        op(MUL, "E", "E", "S1"),
        op(MUL, "E", "E", "S2"),
        op(MUL, "E", "E", "S3"),
        op(MUL, "L", "SHARE", "E"),
    ]);
    p
}

/// `corners`' weights of a point: each axis's fraction and its
/// complement, then each corner's product, axis by axis, in f32.
fn corners() -> Vec<u32> {
    let mut p = vec![hook("H_FLOOR")];
    for a in 0..3 {
        let (s, n, f, g) = (
            format!("S_{a}"),
            format!("N_{a}"),
            format!("F_{a}"),
            format!("G_{a}"),
        );
        p.extend([op(SUB, &f, &s, &n), op(R32, &f, &f, &f)]);
        p.extend([op(SUB, &g, "ONE", &f), op(R32, &g, &g, &g)]);
    }
    for c in 0..8 {
        let w = |a: usize| format!("{}_{a}", if (c >> a) & 1 == 1 { "F" } else { "G" });
        let out = format!("WC{c}");
        p.extend([op(MUL, "PR", &w(0), &w(1)), op(R32, "PR", "PR", "PR")]);
        p.extend([op(MUL, &out, "PR", &w(2)), op(R32, &out, &out, &out)]);
    }
    p
}

/// An f64 constant's WGSL: its words.
fn real(x: f64) -> String {
    let b = x.to_bits();
    format!(
        "vec2<u32>({:#010x}u, {:#010x}u)",
        b as u32,
        (b >> 32) as u32
    )
}

/// The constants libm's `exp` takes, in its order of `NAMES`: 1, 2,
/// 1/2, then `exp.rs`'s INVLN2, LN2HI, LN2LO and P1 to P5 by the words
/// its comments give.
pub(super) const CONSTANTS: [u64; 11] = [
    0x3ff0_0000_0000_0000,
    0x4000_0000_0000_0000,
    0x3fe0_0000_0000_0000,
    0x3ff7_1547_652b_82fe,
    0x3fe6_2e42_fee0_0000,
    0x3dea_39ef_3579_3c76,
    0x3fc5_5555_5555_553e,
    0xbf66_c16c_16be_bd93,
    0x3f11_566a_af25_de2c,
    0xbebb_bd41_c5d2_6bf1,
    0x3e66_3769_72be_a4d0,
];

/// The depth recurrence's work bound: the nodes one thread takes in one
/// dispatch, a direction's layers split so none takes more (fn-256's
/// 2 ms bound).
pub(super) const SWEEP_WORK: u32 = 64;
/// The recurrence's workgroup, one a direction.
pub(super) const SWEEP_WG: u32 = 256;

/// The dispatches the sweep of a lattice of `size` takes: whole layers
/// each, as many as `SWEEP_WORK` allows, at least one.
pub(super) fn sweep_dispatches(size: [u32; 3]) -> u32 {
    let per = (SWEEP_WORK / (size[0] * size[1]).div_ceil(SWEEP_WG)).max(1);
    size[2].div_ceil(per).max(1)
}

/// The programs, registers, hooks and constants as WGSL.
pub(super) fn wgsl() -> String {
    let mut s = String::new();
    let _ = writeln!(s, "const SWEEP_WORK: u32 = {SWEEP_WORK}u;");
    let _ = writeln!(s, "const SWEEP_WG: u32 = {SWEEP_WG}u;");
    let mut list = |name: &str, p: &[u32]| {
        let words: Vec<String> = p.iter().map(|w| format!("{w:#010x}u")).collect();
        let _ = writeln!(s, "const {name}_LEN: u32 = {}u;", p.len());
        let _ = writeln!(
            s,
            "const {name}: array<u32, {}> = array<u32, {}>({});",
            p.len(),
            p.len(),
            words.join(", ")
        );
    };
    list("WEIGHTS", &weights());
    list("PATH", &path());
    list("DEPTH", &depth());
    list("SHADE", &shade());
    list("CORNERS", &corners());
    let _ = writeln!(
        s,
        "const REGISTERS: u32 = {}u;",
        NAMES.len().max(CORNERS_AT + CORNER_NAMES.len())
    );
    for n in NAMES.iter().chain(CORNER_NAMES) {
        let _ = writeln!(s, "const R_{n}: u32 = {}u;", reg(n));
    }
    for (k, h) in HOOKS.iter().enumerate() {
        let _ = writeln!(s, "const {h}: u32 = {}u;", HOOK + k as u32);
    }
    let consts: Vec<String> = CONSTANTS.iter().map(|&c| real(f64::from_bits(c))).collect();
    let _ = writeln!(
        s,
        "const CONSTANTS: array<vec2<u32>, 11> = array<vec2<u32>, 11>({});",
        consts.join(", ")
    );
    // `exp`'s thresholds and scales.
    for (name, x) in [
        // 745.13321910194110842 and 709.782712893383973096.
        ("EXP_ZERO", f64::from_bits(0x4087_4910_d52d_3051)),
        ("EXP_OVER", f64::from_bits(0x4086_2e42_fefa_39ef)),
        ("TWO_1023", f64::from_bits(0x7fe0_0000_0000_0000)),
        (
            "TWO_M969",
            f64::from_bits(0x0010_0000_0000_0000) * f64::from_bits(0x4340_0000_0000_0000),
        ),
    ] {
        let _ = writeln!(s, "const {name}: vec2<u32> = {};", real(x));
    }
    s
}
