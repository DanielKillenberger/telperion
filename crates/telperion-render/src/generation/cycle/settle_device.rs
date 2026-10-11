//! fn-256 task 2 (SETTLE-PLAN.md): the settle's kernels and its buffers'
//! layout, test-only. One module reads the build's state, works in
//! `work` (its sections' columns `settle_cols.wgsl` names) and `atom`,
//! and writes the node table `nt`.
use super::super::io;
use super::Arith;
use crate::Gpu;

const SHARED: &str = include_str!("settle_shared.wgsl");
const COLS: &str = include_str!("settle_cols.wgsl");
const TREE: &str = include_str!("settle_tree.wgsl");
const ASSIGN: &str = include_str!("settle_assign.wgsl");
const SWEEP: &str = include_str!("settle_sweep.wgsl");
const ORDER: &str = include_str!("settle_order.wgsl");
const COMPACT: &str = include_str!("settle_compact.wgsl");
const STATE: &str = include_str!("settle_state.wgsl");
const THICK: &str = include_str!("settle_thick.wgsl");
const REAL: &str = include_str!("real.wgsl");

/// The entries, by their place in `ENTRIES`.
pub(super) const ENTRIES: [&str; 18] = [
    "seed_main",
    "depth_main",
    "count_main",
    "fill_main",
    "sort_main",
    "living_main",
    "woken_main",
    "assign_main",
    "vigour_main",
    "down_main",
    "leaf_main",
    "up_main",
    "keep_main",
    "root_main",
    "count_kids_main",
    "place_main",
    "sizes_main",
    "compact_main",
];
pub(super) const SEED: usize = 0;
pub(super) const DEPTH: usize = 1;
pub(super) const COUNT: usize = 2;
pub(super) const FILL: usize = 3;
pub(super) const SORT: usize = 4;
pub(super) const LIVING: usize = 5;
pub(super) const WOKEN: usize = 6;
pub(super) const ASSIGN_E: usize = 7;
pub(super) const VIGOUR: usize = 8;
pub(super) const DOWN: usize = 9;
pub(super) const LEAF: usize = 10;
pub(super) const UP: usize = 11;
pub(super) const KEEP: usize = 12;
pub(super) const ROOT: usize = 13;
pub(super) const COUNT_KIDS: usize = 14;
pub(super) const PLACE: usize = 15;
pub(super) const SIZES: usize = 16;
pub(super) const COMPACT_E: usize = 17;

/// Work's columns the host reads or scans (`settle_cols.wgsl`).
pub(super) const W_PSTART: u32 = 0;
pub(super) const W_USTART: u32 = 1;
pub(super) const W_DR: u32 = 20;
pub(super) const W_DS: u32 = 21;
pub(super) const W_DON: u32 = 22;
pub(super) const W_FINAL: u32 = 25;
pub(super) const W_NODE: u32 = 26;
pub(super) const W_COLS: u32 = 28;
pub(super) const WP_COLS: u32 = 4;
pub(super) const WU_COLS: u32 = 3;
pub(super) const WF_PSTART: u32 = 1;
pub(super) const WF_USTART: u32 = 2;
pub(super) const WF_SCAN: u32 = 5;
pub(super) const WF_COLS: u32 = 7;
pub(super) const WL_COLS: u32 = 7;
/// Each PA's constants from work's word `K_AT`, sixteen a PA.
pub(super) const K_AT: u32 = 64;
/// `atom`'s words.
pub(super) const DMAX: u32 = 4096;
pub(super) const T_FAULT: usize = 0;
pub(super) const T_DEEPEST: usize = 1;
pub(super) const T_KEPT: usize = 2;
pub(super) const T_KEPT_PHYTS: usize = 3;
pub(super) const T_KEPT_UNITS: usize = 4;
pub(super) const T_LEAF: usize = 8;
pub(super) const T_DCOUNT: u32 = 64;
pub(super) const T_DFILL: u32 = T_DCOUNT + DMAX + 64;
pub(super) const T_KDCOUNT: u32 = T_DFILL + DMAX;
pub(super) const T_KCOUNT: u32 = T_KDCOUNT + DMAX;
/// The node table's axis columns and phytomer columns.
pub(super) const NA_COLS: u32 = 41;
pub(super) const NP_COLS: u32 = 22;
/// The place's columns per kept axis (`settle_state.wgsl`'s `Y_`), and
/// its fault word in `atom`.
pub(super) const Y_COLS: u32 = 56;
pub(super) const T_PLACE: usize = 5;

/// The place module's entries.
pub(super) const PLACE_ENTRIES: [&str; 3] = ["scale_main", "attach_main", "thick_main"];
pub(super) const SCALE: usize = 0;
pub(super) const ATTACH: usize = 1;
pub(super) const THICK_E: usize = 2;

/// The settle's modules, built under one arithmetic: the shed and order
/// over the grown tree, and the place over the node table.
pub(super) struct Kernels {
    pub pass: io::Pass,
    pub place: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu, arith: Arith) -> Self {
        let reads = [true, true, true, false, false, false];
        let place = [true, true, false, false, false];
        Self {
            pass: io::Pass::new(gpu, source(arith), &reads, &ENTRIES),
            place: io::Pass::new(gpu, place_source(arith), &place, &PLACE_ENTRIES),
        }
    }
}

/// The place's WGSL: the contract and the marked root, the reals, the
/// settle's words, the node table under the layout's names, the layout's
/// vectors, angles and stand, then the place's passes.
pub(super) fn place_source(arith: Arith) -> String {
    format!(
        "{}\n{}\n{REAL}\n{SHARED}\n{STATE}\n{}\n{}\n{}\n{THICK}",
        super::super::fine::contract_with(arith.marks()),
        arith.root(),
        super::Q,
        super::SINCOS,
        super::LAY_STAND
    )
}

/// The settle's WGSL: the contract, the reals, the software f64 (its
/// own word product left for the contract's), then the settle's columns
/// and passes.
pub(super) fn source(arith: Arith) -> String {
    const SOFT64: &str = include_str!("soft64.wgsl");
    let start = SOFT64.find("fn mul32(").expect("soft64's word product");
    let end = start + SOFT64[start..].find("\n}\n").expect("its end") + 3;
    let soft = format!("{}{}", &SOFT64[..start], &SOFT64[end..]);
    format!(
        "{}\n{REAL}\n{soft}\n{SHARED}\n{COLS}\n{TREE}\n{ASSIGN}\n{SWEEP}\n{ORDER}\n{COMPACT}",
        super::super::fine::contract_with(arith.marks())
    )
}

fn round64(n: u64) -> u64 {
    n.next_multiple_of(64)
}

/// Where each section of `work` starts and its stride, for a tree of
/// `n` axes, `phyts` phytomers and `units` units, `listed` living apexes
/// or woken buds at most, and whether the tree as grown is kept.
#[derive(Debug, Clone, Copy)]
pub(super) struct Layout {
    pub n: u32,
    pub phyts: u32,
    pub sa: u32,
    pub sp: u32,
    pub su: u32,
    pub sl: u32,
    pub at_a: u32,
    pub at_p: u32,
    pub at_u: u32,
    pub at_f: u32,
    pub at_l: u32,
    pub words: u64,
}

impl Layout {
    pub(super) fn new(n: u32, phyts: u32, units: u32, listed: u32, pas: u32) -> Self {
        let [sa, sp, su, sl] = [n, phyts, units, listed].map(|x| round64(u64::from(x) + 1));
        let at_a = round64(u64::from(K_AT) + 16 * u64::from(pas));
        let at_p = at_a + u64::from(W_COLS) * sa;
        let at_u = at_p + u64::from(WP_COLS) * sp;
        let at_f = at_u + u64::from(WU_COLS) * su;
        let at_l = at_f + u64::from(WF_COLS) * sa;
        let words = at_l + u64::from(WL_COLS) * sl;
        let small = |x: u64| u32::try_from(x).expect("work's words in a u32");
        Self {
            n,
            phyts,
            sa: small(sa),
            sp: small(sp),
            su: small(su),
            sl: small(sl),
            at_a: small(at_a),
            at_p: small(at_p),
            at_u: small(at_u),
            at_f: small(at_f),
            at_l: small(at_l),
            words,
        }
    }

    /// `atom`'s words: the tallies, the depths, each axis's children
    /// counted and filled.
    pub(super) fn atom_words(&self) -> u64 {
        u64::from(self.kfill()) + u64::from(self.n) + 64
    }

    /// Where `atom` fills each parent's children.
    pub(super) fn kfill(&self) -> u32 {
        T_KCOUNT + self.sa
    }

    /// Work's word of grown-axis column `c`, row `i`.
    pub(super) fn a(&self, c: u32, i: u32) -> u64 {
        u64::from(self.at_a) + u64::from(c) * u64::from(self.sa) + u64::from(i)
    }

    /// Work's word of final-axis column `c`, row `f`.
    pub(super) fn f(&self, c: u32, f: u32) -> u64 {
        u64::from(self.at_f) + u64::from(c) * u64::from(self.sa) + u64::from(f)
    }
}

/// The node table's layout for `kept` axes, `units` units and `phyts`
/// phytomers: the axes' columns, then the units', then the phytomers'.
#[derive(Debug, Clone, Copy)]
pub(super) struct Table {
    pub kept: u32,
    pub units: u32,
    pub phyts: u32,
    pub sk: u32,
    pub skp: u32,
    pub at_nu: u32,
    pub at_np: u32,
    pub words: u64,
}

impl Table {
    pub(super) fn new(kept: u32, units: u32, phyts: u32) -> Self {
        let sk = round64(u64::from(kept) + 1);
        let skp = round64(u64::from(phyts) + 1);
        let at_nu = u64::from(NA_COLS) * sk;
        let at_np = round64(at_nu + u64::from(units) + 1);
        let words = at_np + u64::from(NP_COLS) * skp;
        let small = |x: u64| u32::try_from(x).expect("the node table's words in a u32");
        Self { kept, units, phyts, sk: small(sk), skp: small(skp), at_nu: small(at_nu), at_np: small(at_np), words }
    }
}
