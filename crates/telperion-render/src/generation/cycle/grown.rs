//! The grown state, the device's and the CPU's, as the same named
//! columns in the CPU's order (`grow_capture_grown`): every axis's
//! columns by index, each axis's units and phytomers in order, the
//! sleeping buds left by the cycle they wake in, the woken buds, the
//! living apexes, the pencils and the phytomers grown.
use super::build::Build;
use super::columns::Cap;
use super::state::{State, A_HEAD, A_LAYOUT, PENCILS, U_COUNT, U_FIRST, U_NEXT};
use crate::{Gpu, Result};

/// How a column's words read: an integer, two words of one, an f32, or
/// an f64's two words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Kind {
    U32,
    U64,
    F32,
    F64,
}

impl Kind {
    pub(super) fn words(self) -> usize {
        match self {
            Kind::U32 | Kind::F32 => 1,
            Kind::U64 | Kind::F64 => 2,
        }
    }
}

/// One named column, its words row by row.
pub(super) struct Field {
    pub name: String,
    pub kind: Kind,
    pub words: Vec<u32>,
}

/// A table: its columns, and each row's axis (to match by lineage).
#[derive(Default)]
pub(super) struct Table {
    pub fields: Vec<Field>,
    pub axis: Vec<u32>,
}

impl Table {
    fn push(&mut self, name: &str, kind: Kind, words: Vec<u32>) {
        self.fields.push(Field { name: name.to_string(), kind, words });
    }
}

/// The grown state's tables.
#[derive(Default)]
pub(super) struct Snapshot {
    pub axes: Table,
    pub units: Table,
    pub phyts: Table,
    pub sleepers: Table,
    pub woken: Table,
    pub live: Table,
    pub pencils: Table,
    pub lineage: Vec<u64>,
    pub grown: u64,
}

/// The axes' columns by name: the units' (with their kinds), then the
/// layout's.
pub(super) const AXIS_COLUMNS: [(&str, usize, Kind); 25] = [
    ("pa", 1, Kind::U32),
    ("birth", 2, Kind::U32),
    ("origin", 3, Kind::U32),
    ("parent", 4, Kind::U32),
    ("node", 5, Kind::U32),
    ("lineage", 6, Kind::U64),
    ("root", 8, Kind::U64),
    ("horizon", 10, Kind::F64),
    ("whorl", 12, Kind::F64),
    ("blend", 14, Kind::F64),
    ("apex_end", 16, Kind::U32),
    ("phyt_n", 17, Kind::U32),
    ("spent", 18, Kind::U32),
    ("b0", 19, Kind::F64),
    ("b1", 21, Kind::F64),
    ("flags", 23, Kind::U32),
    ("below", 24, Kind::U32),
    ("outlived", 25, Kind::U32),
    ("ended_relays", 26, Kind::U32),
    ("aged", 27, Kind::U32),
    ("sleep", 28, Kind::U32),
    ("units_n", 29, Kind::U32),
    ("grown_ticks", 30, Kind::U32),
    ("succ0", 31, Kind::U32),
    ("succ1", 32, Kind::U32),
];
/// The layout's per axis, in `grow_capture_grown`'s `sk_axes` order, by
/// their columns past `A_LAYOUT`.
pub(super) const SKETCH_AXES: [(&str, u64); 11] = [
    ("rank", 5),
    ("base x", 6),
    ("base y", 7),
    ("base z", 8),
    ("heading x", 9),
    ("heading y", 10),
    ("heading z", 11),
    ("side x", 12),
    ("side y", 13),
    ("side z", 14),
    ("phase", 4),
];
pub(super) const UNIT_COLUMNS: [(&str, usize, Kind); 7] = [
    ("unit axis", 0, Kind::U32),
    ("unit survive", 1, Kind::F32),
    ("unit persist", 2, Kind::F32),
    ("unit share", 3, Kind::U32),
    ("unit size", 4, Kind::U32),
    ("unit shade size", 5, Kind::F64),
    ("unit shade light", 7, Kind::F64),
];
pub(super) const PHYT_COLUMNS: [(&str, usize, Kind); 15] = [
    ("phytomer axis", 2, Kind::U32),
    ("phytomer key", 0, Kind::U64),
    ("phytomer cycle", 3, Kind::U32),
    ("phytomer node", 4, Kind::F32),
    ("tip x", 5, Kind::F32),
    ("tip y", 6, Kind::F32),
    ("tip z", 7, Kind::F32),
    ("heading x", 8, Kind::F32),
    ("heading y", 9, Kind::F32),
    ("heading z", 10, Kind::F32),
    ("side x", 11, Kind::F32),
    ("side y", 12, Kind::F32),
    ("side z", 13, Kind::F32),
    ("scale", 14, Kind::F32),
    ("grown", 15, Kind::F32),
];
pub(super) const PHYT_TAIL: [(&str, usize); 2] = [("size", 16), ("rank", 17)];
pub(super) const SLEEPER_COLUMNS: [(&str, usize, Kind); 11] = [
    ("wake", 0, Kind::U32),
    ("parent", 1, Kind::U32),
    ("node", 2, Kind::U32),
    ("unit", 3, Kind::U32),
    ("slot", 4, Kind::U32),
    ("pa", 5, Kind::U32),
    ("own", 6, Kind::F32),
    ("sleep", 7, Kind::U32),
    ("slept", 8, Kind::U32),
    ("whorl", 9, Kind::F64),
    ("key", 11, Kind::U64),
];
pub(super) const WOKEN_NAMES: [&str; 5] = ["axis", "bearer", "next", "cycle", "share"];

/// A pencil column's name: its walk's, a pull's field, or a scalar's.
pub(super) fn pencil_name(k: usize) -> String {
    const WALK: [&str; 16] = [
        "running x", "running y", "running z", "across x", "across y", "across z", "tip x", "tip y",
        "tip z", "run", "pull", "curve x", "curve y", "curve z", "turning", "pulls",
    ];
    const TAIL: [&str; 11] = [
        "base scale", "running", "rank", "units", "laid", "trunk", "wood", "light", "size", "memory",
        "kept",
    ];
    match k {
        0..16 => format!("pencil {}", WALK[k]),
        16..48 => format!("pencil pull {} {}", (k - 16) / 4, ["bend", "pull", "total", "offset"][(k - 16) % 4]),
        _ => format!("pencil {}", TAIL[k - 48]),
    }
}

/// A pencil column's kind: its counts and flags are integers.
pub(super) fn pencil_kind(k: usize) -> Kind {
    match k {
        15 | 51 | 52 | 53 => Kind::U32,
        _ => Kind::F32,
    }
}

fn words(gpu: &Gpu, b: &wgpu::Buffer) -> Result<Vec<u32>> {
    let raw = super::super::io::read(gpu, b, b.size())?;
    Ok(bytemuck::cast_slice(&raw).to_vec())
}

/// Column `k` of a table `cap` rows apart from word `at`, its rows taken
/// in `order`, each `kind`'s words.
fn gather(from: &[u32], (at, cap, k): (usize, usize, usize), kind: Kind, order: &[usize]) -> Vec<u32> {
    let col = |c: usize, r: usize| from[at + c * cap + r];
    match kind.words() {
        1 => order.iter().map(|&r| col(k, r)).collect(),
        _ => order.iter().flat_map(|&r| [col(k, r), col(k + 1, r)]).collect(),
    }
}

impl Build {
    /// The device's grown state, read back.
    pub(super) fn snapshot(&self, gpu: &Gpu, cycle: u32) -> Result<Snapshot> {
        let s: &State = &self.state;
        let (c, h) = (s.caps, self.held);
        let (axes, arena, pencils, lists) = (words(gpu, &s.axes)?, words(gpu, &s.arena)?, words(gpu, &s.pencils)?, words(gpu, &s.lists)?);
        let mut out = Snapshot { grown: self.grown, ..Snapshot::default() };
        let n = h.axes as usize;
        let ax = |k: u64, i: usize| axes[k as usize * c.axes as usize + i];
        let every: Vec<usize> = (0..n).collect();
        for (name, k, kind) in AXIS_COLUMNS {
            out.axes.push(name, kind, gather(&axes, (0, c.axes as usize, k), kind, &every));
        }
        for (name, k) in SKETCH_AXES {
            out.axes.push(name, Kind::F32, every.iter().map(|&i| ax(A_LAYOUT + k, i)).collect());
        }
        out.axes.axis = (0..n as u32).collect();
        out.lineage = (0..n).map(|i| u64::from(ax(6, i)) | u64::from(ax(7, i)) << 32).collect();
        // Each axis's units in order, and their phytomers.
        let unit = |k: u64, r: usize| arena[k as usize * c.units as usize + r];
        let (mut units, mut phyts) = (Vec::new(), Vec::new());
        for i in 0..n {
            let mut r = ax(A_HEAD, i);
            // cap: the axis's units.
            while r != u32::MAX {
                let r_ = r as usize;
                units.push(r_);
                let first = unit(U_FIRST, r_) as usize;
                phyts.extend(first..first + unit(U_COUNT, r_) as usize);
                r = unit(U_NEXT, r_);
            }
        }
        for (name, k, kind) in UNIT_COLUMNS {
            out.units.push(name, kind, gather(&arena, (0, c.units as usize, k), kind, &units));
        }
        out.units.axis = out.units.fields[0].words.clone();
        let phyt_at = c.phyt_at() as usize;
        for (name, k, kind) in PHYT_COLUMNS {
            out.phyts.push(name, kind, gather(&arena, (phyt_at, c.phyts as usize, k), kind, &phyts));
        }
        for (name, k) in PHYT_TAIL {
            out.phyts.push(name, Kind::F32, gather(&arena, (phyt_at, c.phyts as usize, k), Kind::F32, &phyts));
        }
        out.phyts.axis = out.phyts.fields[0].words.clone();
        self.lists_snapshot(&mut out, &lists, cycle);
        let laid: Vec<usize> = (0..self.laid as usize).collect();
        for k in 0..PENCILS as usize {
            let kind = pencil_kind(k);
            out.pencils.push(&pencil_name(k), kind, gather(&pencils, (0, c.axes as usize, k), kind, &laid));
        }
        out.pencils.axis = (0..self.laid).collect();
        Ok(out)
    }

    /// The sleeping buds left (waking after `cycle`, by their cycle, each
    /// in its order), the woken buds and the living apexes.
    fn lists_snapshot(&self, out: &mut Snapshot, lists: &[u32], cycle: u32) {
        let (c, h) = (self.state.caps, self.held);
        let at = c.sleep_at() as usize;
        let cap = c.sleepers as usize;
        let mut left: Vec<usize> = (0..h.sleepers as usize).filter(|&r| lists[at + r] > cycle).collect();
        left.sort_by_key(|&r| lists[at + r]);
        for (name, k, kind) in SLEEPER_COLUMNS {
            out.sleepers.push(&format!("sleeper {name}"), kind, gather(lists, (at, cap, k), kind, &left));
        }
        out.sleepers.axis = gather(lists, (at, cap, 1), Kind::U32, &left);
        let woken: Vec<usize> = (0..h.woken as usize).collect();
        for (k, name) in WOKEN_NAMES.iter().enumerate() {
            out.woken.push(&format!("woken {name}"), Kind::U32, gather(lists, (c.woken_at() as usize, c.woken as usize, k), Kind::U32, &woken));
        }
        let next: Vec<usize> = (0..h.next as usize).collect();
        let next_at = (c.next_at() as usize, c.next as usize);
        out.live.push("live axis", Kind::U32, gather(lists, (next_at.0, next_at.1, 0), Kind::U32, &next));
        out.live.push("live spent", Kind::U32, gather(lists, (next_at.0, next_at.1, 1), Kind::U32, &next));
        out.live.axis = out.live.fields[0].words.clone();
    }
}

/// The CPU's grown state from its capture, in the same columns.
pub(super) fn cpu_snapshot(c: &Cap) -> Snapshot {
    let mut out = Snapshot { grown: c.u64s("g_grown")[0], ..Snapshot::default() };
    let two = |v: Vec<u64>| -> Vec<u32> { v.iter().flat_map(|&x| [x as u32, (x >> 32) as u32]).collect() };
    let get = |name: &str, kind: Kind| match kind.words() {
        1 => c.u32s(&format!("g_{name}")),
        _ => two(c.u64s(&format!("g_{name}"))),
    };
    for (name, _, kind) in AXIS_COLUMNS {
        out.axes.push(name, kind, get(name, kind));
    }
    let n = c.u32s("g_pa").len();
    out.lineage = c.u64s("g_lineage");
    let sk = c.u32s("sk_axes");
    for (j, (name, _)) in SKETCH_AXES.iter().enumerate() {
        out.axes.push(name, Kind::F32, (0..n).map(|i| sk[11 * i + j]).collect());
    }
    out.axes.axis = (0..n as u32).collect();
    let unit_names = ["axis", "survive", "persist", "share", "size"];
    for ((name, _, kind), cpu) in UNIT_COLUMNS.iter().zip(unit_names) {
        out.units.push(name, *kind, c.u32s(&format!("g_unit_{cpu}")));
    }
    let shade = c.u64s("g_unit_shade");
    for (k, (name, _, kind)) in UNIT_COLUMNS[5..].iter().enumerate() {
        out.units.push(name, *kind, two(shade.iter().skip(k).step_by(2).copied().collect()));
    }
    out.units.axis = c.u32s("g_unit_axis");
    let phyt_names = ["axis", "key", "cycle", "node"];
    for ((name, _, kind), cpu) in PHYT_COLUMNS.iter().zip(phyt_names) {
        let words = match cpu {
            "key" => two(c.u64s("g_phyt_key")),
            _ => c.u32s(&format!("g_phyt_{cpu}")),
        };
        out.phyts.push(name, *kind, words);
    }
    let sp = c.u32s("sk_phyts");
    let rows = sp.len() / 13;
    let sketch_names: Vec<&str> = PHYT_COLUMNS[4..].iter().map(|f| f.0).chain(PHYT_TAIL.iter().map(|f| f.0)).collect();
    for (j, name) in sketch_names.iter().enumerate() {
        out.phyts.push(name, Kind::F32, (0..rows).map(|r| sp[13 * r + j]).collect());
    }
    out.phyts.axis = c.u32s("g_phyt_axis");
    for (name, _, kind) in SLEEPER_COLUMNS {
        let words = match kind.words() {
            1 => c.u32s(&format!("g_sleep_{name}")),
            _ => two(c.u64s(&format!("g_sleep_{name}"))),
        };
        out.sleepers.push(&format!("sleeper {name}"), kind, words);
    }
    out.sleepers.axis = c.u32s("g_sleep_parent");
    for name in WOKEN_NAMES {
        out.woken.push(&format!("woken {name}"), Kind::U32, c.u32s(&format!("g_woken_{name}")));
    }
    out.live.push("live axis", Kind::U32, c.u32s("g_live_axis"));
    out.live.push("live spent", Kind::U32, c.u32s("g_live_spent"));
    out.live.axis = c.u32s("g_live_axis");
    let pens = c.u32s("sk_pencils");
    let laid = pens.len() / PENCILS as usize;
    for k in 0..PENCILS as usize {
        let words = (0..laid).map(|r| pens[PENCILS as usize * r + k]).collect();
        out.pencils.push(&pencil_name(k), pencil_kind(k), words);
    }
    out.pencils.axis = (0..laid as u32).collect();
    out
}
