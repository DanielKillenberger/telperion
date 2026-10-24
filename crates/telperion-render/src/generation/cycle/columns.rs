//! A capture's arrays by name, and the unit tables and each PA's row as
//! the units module reads them (`zones.wgsl`, `arena.wgsl`).
use super::units::{f, pack};
use telperion_space::{Capture, ChainTables};

pub(super) const NONE: u32 = u32::MAX;

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

/// The unit tables (`zones.wgsl`), then each PA's row (`arena.wgsl`'s
/// `R_` words) and the zones' buds a whorl: the table, the words, where
/// the rows' offsets and the whorls start.
pub(super) fn packed(c: &Cap, t: &ChainTables, age: u32) -> (Vec<u32>, Vec<u64>, u32, u32) {
    let (mut table, mut words) = pack(&t.units);
    let rows = table.len() as u32;
    let (viability, shade) = (c.f32s("pa_viability"), c.f32s("pa_shade_hazard"));
    let ground = c.f32s("pa_ground_hazard");
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
        let at = words.len() as u64 + 20;
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
            f(ground[pa]),
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
