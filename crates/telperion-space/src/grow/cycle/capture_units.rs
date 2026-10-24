//! fn-256 task 2 (DESIGN.md section 7): the columns a grown state is
//! captured by for the device (`capture_grown.rs`), and the unit tables
//! it reads. Times on the tick grid are held as ticks, reals as their
//! bits.
use super::super::Grower;
use super::capture::Capture;
use crate::contract::{grid, CYCLE};
use crate::species::ALONG;
use crate::structure::Origin;

const NONE: u32 = u32::MAX;

pub(super) struct Before {
    phyts: Vec<usize>,
    units: Vec<usize>,
    asleep: Vec<usize>,
    woken: usize,
    cycle: u32,
}

impl Before {
    /// Nothing before: every axis's units, phytomers and sleeping buds
    /// (from cycle 0 on) and every woken bud are taken as added.
    pub(super) fn nothing() -> Self {
        Self {
            phyts: Vec::new(),
            units: Vec::new(),
            asleep: Vec::new(),
            woken: 0,
            cycle: 0,
        }
    }
}

/// A time on the tick grid as its ticks, which an f64 holds exactly.
fn tk(t: f64) -> u32 {
    assert!(t >= 0.0 && grid(t) == t, "a time off the tick grid: {t}");
    let ticks = t * CYCLE;
    assert!(
        ticks < f64::from(1u32 << 31),
        "a time past the columns: {t}"
    );
    ticks as u32
}

/// A presence an f64 holds: the f32 it is.
fn f(x: f64) -> f32 {
    assert_eq!(f64::from(x as f32), x, "a presence that is no f32");
    x as f32
}

/// An origin as one word: kind, slot, woken; its parent, node and whorl.
fn origin(o: Origin) -> (u32, u32, u32, u64) {
    match o {
        Origin::Seed => (0, NONE, 0, 0),
        Origin::Lateral {
            parent,
            node,
            slot,
            whorl,
            woken,
        } => (
            1 | u32::from(slot) << 8 | u32::from(woken) << 16,
            parent as u32,
            node as u32,
            whorl.to_bits(),
        ),
        Origin::Continuation { parent } => (2, parent as u32, 0, 0),
        Origin::Relay { parent, node } => (3, parent as u32, node as u32, 0),
    }
}

impl Grower<'_> {
    /// The species' tables the device reads beside `ChainTables::cycle`:
    /// each PA's viability and shade hazard as the f32 its table holds, its
    /// shedding delay plus a cycle, the windows of a whole, a failed
    /// unit's and an abortion's stake per cycle, and each zone's buds a
    /// whorl, in the units' order.
    pub(super) fn capture_tables(&self, c: &mut Capture) {
        let states = &self.species.states;
        let age = self.rules.age;
        let st = |g: &dyn Fn(&crate::species::PaState) -> f64| -> Vec<f32> {
            states.iter().map(|s| g(s) as f32).collect()
        };
        c.f32s("pa_viability", &st(&|s| s.viability));
        c.f32s("pa_shade_hazard", &st(&|s| s.shade_hazard));
        c.f32s("pa_ground_hazard", &st(&|s| s.ground_hazard));
        let sheds: Vec<u32> = states
            .iter()
            .map(|s| u32::from(s.shedding.is_finite()))
            .collect();
        c.u32s("pa_sheds", &sheds);
        let delay: Vec<u64> = states
            .iter()
            .map(|s| (s.shedding + 1.0).to_bits())
            .collect();
        c.u64s("pa_delay", &delay);
        let mut windows = Vec::new();
        for stake in 0..3 {
            for (pa, s) in states.iter().enumerate() {
                let stake = [1.0, 1.0 - s.relay_failed, 1.0 - s.relay][stake];
                windows.push(0.0);
                for at in 1..=age + 1 {
                    windows.push(self.rules.windows.window((pa, at, None), stake));
                }
            }
        }
        c.f32s("pa_windows", &windows);
        let buds: Vec<u64> = (states.iter())
            .flat_map(|s| {
                ALONG
                    .iter()
                    .filter(|&&z| z < s.zones.len())
                    .map(|&z| s.zones[z].buds)
            })
            .map(f64::to_bits)
            .collect();
        c.u64s("zone_buds", &buds);
        let lights = self.reads().lights;
        c.u32s("lights", &[u32::from(lights)]);
    }

    /// The state the cycle reads and writes, under `prefix`: every axis's
    /// columns, then what was added past `before`.
    pub(super) fn capture_state(&self, c: &mut Capture, prefix: &str, before: &Before) {
        let name = |n: &str| format!("{prefix}_{n}");
        let n = self.axes.len();
        let (mut kind, mut parent, mut node, mut whorl) = (vec![], vec![], vec![], vec![]);
        for a in &self.axes {
            let (k, p, j, w) = origin(a.origin);
            kind.push(k);
            parent.push(p);
            node.push(j);
            whorl.push(w);
        }
        let ax = |g: &dyn Fn(usize) -> u32| -> Vec<u32> { (0..n).map(g).collect() };
        let ax64 = |g: &dyn Fn(usize) -> u64| -> Vec<u64> { (0..n).map(g).collect() };
        c.u64s(&name("lineage"), &ax64(&|i| self.axes[i].lineage));
        c.u32s(&name("pa"), &ax(&|i| self.axes[i].pa as u32));
        c.u32s(&name("birth"), &ax(&|i| self.axes[i].birth));
        c.u32s(&name("origin"), &kind);
        c.u32s(&name("parent"), &parent);
        c.u32s(&name("node"), &node);
        c.u64s(&name("whorl"), &whorl);
        c.u64s(&name("blend"), &ax64(&|i| self.axes[i].blend.to_bits()));
        c.u32s(
            &name("apex_end"),
            &ax(&|i| self.axes[i].apex_end.map_or(0, |e| e + 1)),
        );
        c.u32s(
            &name("phyt_n"),
            &ax(&|i| self.axes[i].phytomers.len() as u32),
        );
        c.u64s(&name("root"), &self.roots);
        c.u64s(&name("horizon"), &ax64(&|i| self.horizon[i].to_bits()));
        c.u32s(&name("spent"), &ax(&|i| tk(self.units[i])));
        self.capture_draws(c, prefix);
        let succ = |k: usize| -> Vec<u32> {
            (self.successor.iter())
                .map(|s| {
                    assert!(s.len() <= 2, "at most a continuation and a relay");
                    s.get(k).map_or(NONE, |&x| x as u32)
                })
                .collect()
        };
        c.u32s(&name("succ0"), &succ(0));
        c.u32s(&name("succ1"), &succ(1));
        self.capture_added(c, prefix, before);
        let live: Vec<u32> = self.live.iter().map(|a| a.axis as u32).collect();
        c.u32s(&name("live_axis"), &live);
        c.u32s(
            &name("live_spent"),
            &self.live.iter().map(|a| tk(a.spent)).collect::<Vec<_>>(),
        );
        c.u64s(&name("grown"), &[self.rules.grown]);
    }

    /// Each axis's draws' scalars.
    fn capture_draws(&self, c: &mut Capture, prefix: &str) {
        let name = |n: &str| format!("{prefix}_{n}");
        let d = &self.draws;
        let ax =
            |g: &dyn Fn(&crate::presence::Draws) -> u32| -> Vec<u32> { d.iter().map(g).collect() };
        c.u64s(
            &name("b0"),
            &d.iter().map(|x| x.birth[0].to_bits()).collect::<Vec<_>>(),
        );
        c.u64s(
            &name("b1"),
            &d.iter().map(|x| x.birth[1].to_bits()).collect::<Vec<_>>(),
        );
        c.u32s(
            &name("flags"),
            &ax(&|x| {
                u32::from(x.failed)
                    | u32::from(x.ended) << 1
                    | u32::from(x.rest) << 2
                    | u32::from(x.carried) << 3
                    | u32::from(x.woke) << 4
                    | u32::from(x.outlived().is_some()) << 5
                    | u32::from(x.alive) << 6
            }),
        );
        c.u32s(
            &name("below"),
            &ax(&|x| u32::try_from(x.below).expect("below a tick")),
        );
        c.u32s(&name("outlived"), &ax(&|x| x.outlived().map_or(0, tk)));
        c.u32s(&name("ended_relays"), &ax(&|x| x.ended_relays));
        c.u32s(&name("aged"), &ax(&|x| tk(x.aged)));
        c.u32s(&name("sleep"), &ax(&|x| tk(x.sleep)));
        c.u32s(&name("units_n"), &ax(&|x| x.units.len() as u32));
        c.u32s(&name("grown_ticks"), &ax(&|x| tk(x.grown())));
    }

    /// The units, phytomers, sleeping buds and woken buds added past
    /// `before`, by axis (and by the cycle they wake in).
    fn capture_added(&self, c: &mut Capture, prefix: &str, before: &Before) {
        let name = |n: &str| format!("{prefix}_{n}");
        let (mut ua, mut us, mut up, mut ush, mut usz, mut ushade) =
            (vec![], vec![], vec![], vec![], vec![], vec![]);
        for (i, d) in self.draws.iter().enumerate() {
            assert_eq!(d.shade.len(), d.units.len(), "a shade a unit");
            for (k, u) in d
                .units
                .iter()
                .enumerate()
                .skip(before.units.get(i).copied().unwrap_or(0))
            {
                ua.push(i as u32);
                us.push(f(u.survive));
                up.push(f(u.persist));
                ush.push(tk(u.share));
                usz.push(tk(u.size));
                ushade.extend(d.shade[k].map(f64::to_bits));
            }
        }
        c.u32s(&name("unit_axis"), &ua);
        c.f32s(&name("unit_survive"), &us);
        c.f32s(&name("unit_persist"), &up);
        c.u32s(&name("unit_share"), &ush);
        c.u32s(&name("unit_size"), &usz);
        c.u64s(&name("unit_shade"), &ushade);
        let (mut pa, mut pk, mut pc, mut pn) = (vec![], vec![], vec![], vec![]);
        for (i, a) in self.axes.iter().enumerate() {
            let from = before.phyts.get(i).copied().unwrap_or(0);
            for (j, p) in a.phytomers.iter().enumerate().skip(from) {
                pa.push(i as u32);
                pk.push(p.key);
                pc.push(p.cycle);
                pn.push(f(self.draws[i].nodes[j]));
            }
        }
        c.u32s(&name("phyt_axis"), &pa);
        c.u64s(&name("phyt_key"), &pk);
        c.u32s(&name("phyt_cycle"), &pc);
        c.f32s(&name("phyt_node"), &pn);
        self.capture_asleep(c, prefix, before);
        let woken = &self.woken[before.woken..];
        let w = |g: &dyn Fn(&crate::dormant::Woken) -> u32| -> Vec<u32> {
            woken.iter().map(g).collect()
        };
        c.u32s(&name("woken_axis"), &w(&|x| x.axis as u32));
        c.u32s(&name("woken_bearer"), &w(&|x| x.bearer as u32));
        c.u32s(&name("woken_next"), &w(&|x| x.next as u32));
        c.u32s(&name("woken_cycle"), &w(&|x| x.cycle));
        c.u32s(&name("woken_share"), &w(&|x| tk(x.share)));
    }

    /// The sleeping buds that wake from the captured cycle on, past
    /// `before`, by the cycle they wake in, each in its order.
    fn capture_asleep(&self, c: &mut Capture, prefix: &str, before: &Before) {
        let name = |n: &str| format!("{prefix}_sleep_{n}");
        let mut cols: [Vec<u32>; 9] = Default::default();
        let (mut whorl, mut key) = (vec![], vec![]);
        for (k, list) in self.asleep.iter().enumerate().skip(before.cycle as usize) {
            // The wake empties the cycle's own list.
            let from = before.asleep.get(k).copied().unwrap_or(0);
            for s in list.get(from..).unwrap_or_default() {
                let row = [
                    k as u32,
                    s.parent as u32,
                    s.node as u32,
                    s.unit as u32,
                    u32::from(s.slot),
                    s.pa as u32,
                    s.own.to_bits(),
                    tk(s.sleep),
                    s.slept,
                ];
                for (col, v) in cols.iter_mut().zip(row) {
                    col.push(v);
                }
                whorl.push(s.whorl.to_bits());
                key.push(s.key);
            }
        }
        let names = [
            "wake", "parent", "node", "unit", "slot", "pa", "own", "sleep", "slept",
        ];
        for (n, col) in names.iter().zip(&cols) {
            c.u32s(&name(n), col);
        }
        c.u64s(&name("whorl"), &whorl);
        c.u64s(&name("key"), &key);
    }
}
