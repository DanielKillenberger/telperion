//! A captured cycle as the device's columns (`arena.wgsl`): the unit
//! tables and each PA's row, the living apexes and pencils, and the
//! tree's axes, units and sleeping buds before the cycle, every column
//! one word a row, its rows `cap` apart.
use super::super::fine::units::{f, pack};
use telperion_space::{Capture, ChainTables};

pub(super) const NONE: u32 = u32::MAX;
/// An axis's columns (`frame.wgsl`'s `AXES_COLUMNS`), a unit's, a
/// phytomer's, a sleeping bud's and a woken bud's.
pub(super) const AXES: usize = 35;
pub(super) const UNITS: usize = 13;
pub(super) const PHYTS: usize = 5;
pub(super) const SLEEPERS: usize = 14;
pub(super) const WOKEN: usize = 5;
/// The tally's words at the head of `lists`.
pub(super) const TALLY: usize = 64;

/// A capture's arrays by name.
pub(super) struct Cap<'a>(pub &'a Capture);

impl Cap<'_> {
    fn bytes(&self, name: &str) -> &[u8] {
        self.0
            .array(name)
            .unwrap_or_else(|| panic!("no array {name}"))
            .1
    }

    pub fn u32s(&self, name: &str) -> Vec<u32> {
        let b = self.bytes(name);
        b.as_chunks::<4>()
            .0
            .iter()
            .map(|w| u32::from_le_bytes(*w))
            .collect()
    }

    pub fn u64s(&self, name: &str) -> Vec<u64> {
        let b = self.bytes(name);
        b.as_chunks::<8>()
            .0
            .iter()
            .map(|w| u64::from_le_bytes(*w))
            .collect()
    }

    pub fn f32s(&self, name: &str) -> Vec<f32> {
        self.u32s(name).into_iter().map(f32::from_bits).collect()
    }
}

/// How many rows each arena holds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Caps {
    pub axes: u32,
    pub units: u32,
    pub phyts: u32,
    pub sleepers: u32,
    pub next: u32,
    pub woken: u32,
}

/// The tree before the cycle: its counts, the columns it starts from and
/// the tables.
pub(super) struct Columns {
    pub cycle: u32,
    pub age: u32,
    pub lights: u32,
    pub axes: u32,
    pub units: u32,
    pub sleepers: u32,
    pub live: u32,
    pub pencils: u32,
    pub table: Vec<u32>,
    pub words: Vec<u64>,
    pub rows: u32,
    pub whorls: u32,
    pub inputs: Vec<u32>,
    axis_cols: Vec<Vec<u32>>,
    unit_cols: Vec<Vec<u32>>,
    sleeper_cols: Vec<Vec<u32>>,
}

fn split(v: &[u64]) -> (Vec<u32>, Vec<u32>) {
    (
        v.iter().map(|&x| x as u32).collect(),
        v.iter().map(|&x| (x >> 32) as u32).collect(),
    )
}

impl Columns {
    pub(super) fn new(c: &Cap, tables: &ChainTables) -> Self {
        let [cycle, age, _] = c.u32s("cycle")[..] else {
            panic!("cycle, age, budget")
        };
        let (table, words, rows, whorls) = packed(c, tables, age);
        let live = c.u32s("in_live_axis");
        let spent = c.u32s("in_live_spent");
        let mut inputs: Vec<u32> = live
            .iter()
            .zip(&spent)
            .flat_map(|(&a, &s)| [a, s])
            .collect();
        let (light, size, kept) = (c.u64s("pen_light"), c.u64s("pen_size"), c.u64s("pen_kept"));
        let real = |x: &u64| f64::from_bits(*x);
        // `light as f32` and `q(kept)`, as the CPU's survival reads them.
        inputs.extend(light.iter().map(|x| (real(x) as f32).to_bits()));
        inputs.extend(
            kept.iter()
                .map(|x| telperion_space::contract::presence::q(real(x)).to_bits()),
        );
        for v in [&size, &light] {
            let (lo, hi) = split(v);
            inputs.extend(lo);
            inputs.extend(hi);
        }
        let (unit_cols, heads, tails) = unit_columns(c);
        let axis_cols = axis_columns(c, &heads, &tails);
        let sleeper_cols = sleeper_columns(c, &heads);
        Self {
            cycle,
            age,
            lights: c.u32s("lights")[0],
            axes: axis_cols[1].len() as u32,
            units: unit_cols[0].len() as u32,
            sleepers: sleeper_cols[0].len() as u32,
            live: live.len() as u32,
            pencils: light.len() as u32,
            table,
            words,
            rows,
            whorls,
            inputs,
            axis_cols,
            unit_cols,
            sleeper_cols,
        }
    }

    /// The axes' columns at `cap` rows each.
    pub fn axes_at(&self, cap: u32) -> Vec<u32> {
        laid_out(&self.axis_cols, cap)
    }

    /// The units' columns, then the phytomers' (none before the cycle).
    pub fn arena_at(&self, caps: Caps) -> Vec<u32> {
        let mut out = laid_out(&self.unit_cols, caps.units);
        out.resize(out.len() + PHYTS * caps.phyts as usize, 0);
        out
    }

    /// The tally, the sleeping buds' columns, the living apexes', the
    /// woken buds' and the cycle's sleeping buds in order.
    pub fn lists_at(&self, caps: Caps) -> Vec<u32> {
        let mut out = vec![0; TALLY];
        out.extend(laid_out(&self.sleeper_cols, caps.sleepers));
        let rest = 2 * caps.next + WOKEN as u32 * caps.woken + self.sleepers.max(1);
        out.resize(out.len() + rest as usize, 0);
        out
    }
}

/// Columns of `cap` rows each, end to end.
fn laid_out(cols: &[Vec<u32>], cap: u32) -> Vec<u32> {
    let mut out = vec![0; cols.len() * cap as usize];
    for (k, col) in cols.iter().enumerate() {
        let at = k * cap as usize;
        out[at..at + col.len()].copy_from_slice(col);
    }
    out
}

/// The units before the cycle, axis by axis, linked along each axis; each
/// axis's first and last.
fn unit_columns(c: &Cap) -> (Vec<Vec<u32>>, Vec<u32>, Vec<u32>) {
    let axis = c.u32s("in_unit_axis");
    let n_axes = c.u32s("in_pa").len();
    let (mut heads, mut tails) = (vec![NONE; n_axes], vec![NONE; n_axes]);
    let mut cols = vec![Vec::with_capacity(axis.len()); UNITS];
    let (lo, hi) = split(&c.u64s("in_unit_shade"));
    let (survive, persist) = (c.f32s("in_unit_survive"), c.f32s("in_unit_persist"));
    let (share, size) = (c.u32s("in_unit_share"), c.u32s("in_unit_size"));
    for (r, &a) in axis.iter().enumerate() {
        let a = a as usize;
        let prev = tails[a];
        if prev == NONE {
            heads[a] = r as u32;
        } else {
            cols[10][prev as usize] = r as u32;
        }
        tails[a] = r as u32;
        let row = [
            a as u32,
            survive[r].to_bits(),
            persist[r].to_bits(),
            share[r],
            size[r],
            lo[2 * r],
            hi[2 * r],
            lo[2 * r + 1],
            hi[2 * r + 1],
            prev,
            NONE,
            NONE,
            0,
        ];
        for (col, v) in cols.iter_mut().zip(row) {
            col.push(v);
        }
    }
    (cols, heads, tails)
}

/// The axes' columns (`arena.wgsl`'s `A_` words, then the head).
fn axis_columns(c: &Cap, heads: &[u32], tails: &[u32]) -> Vec<Vec<u32>> {
    let n = heads.len();
    let mut cols = vec![Vec::new(); AXES];
    cols[0] = vec![0; n];
    let one =
        |k: usize, name: &str, cols: &mut Vec<Vec<u32>>| cols[k] = c.u32s(&format!("in_{name}"));
    let two = |k: usize, name: &str, cols: &mut Vec<Vec<u32>>| {
        let (lo, hi) = split(&c.u64s(&format!("in_{name}")));
        (cols[k], cols[k + 1]) = (lo, hi);
    };
    for (k, name) in [
        (1, "pa"),
        (2, "birth"),
        (3, "origin"),
        (4, "parent"),
        (5, "node"),
    ] {
        one(k, name, &mut cols);
    }
    for (k, name) in [
        (6, "lineage"),
        (8, "root"),
        (10, "horizon"),
        (12, "whorl"),
        (14, "blend"),
    ] {
        two(k, name, &mut cols);
    }
    for (k, name) in [(16, "apex_end"), (17, "phyt_n"), (18, "spent")] {
        one(k, name, &mut cols);
    }
    two(19, "b0", &mut cols);
    two(21, "b1", &mut cols);
    let scalars = [
        (23, "flags"),
        (24, "below"),
        (25, "outlived"),
        (26, "ended_relays"),
        (27, "aged"),
        (28, "sleep"),
        (29, "units_n"),
        (30, "grown_ticks"),
        (31, "succ0"),
        (32, "succ1"),
    ];
    for (k, name) in scalars {
        one(k, name, &mut cols);
    }
    cols[33] = tails.to_vec();
    cols[34] = heads.to_vec();
    cols
}

/// The sleeping buds that wake from the cycle on, each with its unit's
/// record.
fn sleeper_columns(c: &Cap, heads: &[u32]) -> Vec<Vec<u32>> {
    let get = |n: &str| c.u32s(&format!("in_sleep_{n}"));
    let (parent, unit) = (get("parent"), get("unit"));
    let (wl, wh) = split(&c.u64s("in_sleep_whorl"));
    let (kl, kh) = split(&c.u64s("in_sleep_key"));
    let rec = parent
        .iter()
        .zip(&unit)
        .map(|(&p, &u)| heads[p as usize] + u)
        .collect();
    let names = [
        "wake", "parent", "node", "unit", "slot", "pa", "own", "sleep", "slept",
    ];
    let mut cols: Vec<Vec<u32>> = names.iter().map(|n| get(n)).collect();
    cols.extend([wl, wh, kl, kh, rec]);
    cols
}

/// The unit tables (`zones.wgsl`), then each PA's row (`arena.wgsl`'s
/// `R_` words) and the zones' buds a whorl: the table, the words, where
/// the rows' offsets and the whorls start.
pub(super) fn packed(c: &Cap, t: &ChainTables, age: u32) -> (Vec<u32>, Vec<u64>, u32, u32) {
    let (mut table, mut words) = pack(&t.units);
    let rows = table.len() as u32;
    let (viability, shade) = (c.f32s("pa_viability"), c.f32s("pa_shade_hazard"));
    let (sheds, delay, windows) = (c.u32s("pa_sheds"), c.u64s("pa_delay"), c.f32s("pa_windows"));
    let span = age as usize + 2;
    let pair = |lo: u32, hi: u32| u64::from(lo) | u64::from(hi) << 32;
    let npa = t.pas.len();
    for (pa, r) in t.pas.iter().enumerate() {
        table.push(words.len() as u32);
        let (a_thr, a_l, rise) = r.abortion.unwrap_or((0, 0.0, None));
        let flags =
            u32::from(r.abortion.is_some()) | u32::from(rise.is_some()) << 1 | sheds[pa] << 2;
        let (next, go, logit) = r
            .successor
            .map_or((NONE, 0, 0.0), |(n, g, l)| (n as u32, g, l));
        let at = words.len() as u64 + 19;
        let (reaches, woods) = (at, at + r.reaches.len() as u64);
        let wins = woods + t.log_woods[pa].len() as u64;
        words.extend([
            r.life,
            f(viability[pa]),
            f(shade[pa]),
            r.relay[0],
            r.relay[1],
            r.relay[2],
            a_thr,
            f(a_l),
            f(rise.unwrap_or(0.0)),
            u64::from(flags),
            pair(r.lived.map_or(NONE, |l| l as u32), next),
            go,
            f(logit),
            pair(r.moved.0.to_bits(), r.moved.1.to_bits()),
            delay[pa],
            pair(r.stakes[0].0.to_bits(), r.stakes[0].1.to_bits()),
            reaches,
            woods,
            wins,
        ]);
        words.extend(r.reaches.iter().map(|&x| f(x)));
        words.extend(t.log_woods[pa].iter().map(|&x| f(x)));
        for stake in 0..3 {
            let from = (stake * npa + pa) * span;
            words.extend(windows[from..from + span].iter().map(|&x| f(x)));
        }
    }
    let whorls = words.len() as u32;
    words.extend(c.u64s("zone_buds"));
    (table, words, rows, whorls)
}
