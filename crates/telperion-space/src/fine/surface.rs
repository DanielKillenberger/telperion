//! The GPU executor's surface for a chain's growth (fn-209 F1, task 2): a
//! species point's per-PA rows, as the contract's thresholds, ticks and
//! f32 values (`contract.rs`), every chain of a tree as the executor takes it (its bud),
//! and its topology as the CPU reference executor grows it, which the
//! executor's tests hold the GPU's against.
use super::tables::Units;
use crate::error::Result;
use crate::contract::presence::q;
use crate::contract::{logit_of, neg_log2_of, ticks, INF};
use crate::lineage::threshold;
use crate::presence::{FLOOR, RATE, SPAN};
use crate::species::Species;

/// A PA's row: what a growth unit's survival, abortion, stop, move and
/// relay decide on, and its laterals' horizon and lasting.
#[derive(Debug, Clone, PartialEq)]
pub struct PaRow {
    /// Its lifespan in ticks, and whether it lives at all.
    pub life: u64,
    pub lives: bool,
    /// The age a bud of it grows in (`Species::lived`).
    pub lived: Option<usize>,
    /// D5: a whole unit's threshold, and `-log2` for a share.
    pub viability: (u64, f32),
    /// D6: none where it never aborts; else a whole, unrising unit's
    /// threshold, `-log2 (1 - a)`, and the rise where it rises.
    pub abortion: Option<(u64, f32, Option<f32>)>,
    /// D9: the relay thresholds of a stop by abortion, end and failure.
    pub relay: [u64; 3],
    /// D8 and D10: the age it moves on to, the move's threshold and the
    /// log-odds of its chance (`logit_of`), and its window's rate and floor
    /// at its stake.
    pub successor: Option<(usize, u64, f32)>,
    pub moved: (f32, f32),
    /// D14: its shedding delay plus a cycle, where it sheds, and the bound
    /// on its subtree's life, where it has one, in cycles.
    pub delay: Option<f32>,
    pub lasting: Option<f32>,
    /// D18: where along its last unit a relay of it stands.
    pub relay_at: f32,
    /// Host decision 8: the window's rate and floor at stake 1, at
    /// a failed unit's (1 - relay_failed) and at an abortion's (1 -
    /// relay); its successor's chance and its ended age's relay
    /// (`Grower::living`); each stage's reach from a bud of it, its own
    /// first (`Species::lived`, `dormant::aged`); and its fade's delay
    /// plus a cycle (`shed.rs`).
    pub stakes: [(f32, f32); 3],
    pub living: (f32, f32),
    pub reaches: Vec<f32>,
    pub fade_delay: f32,
}

/// A species point's tables for a chain's growth.
#[derive(Debug, Clone)]
pub struct ChainTables {
    pub pas: Vec<PaRow>,
    pub units: Units,
    /// Per PA, per cycle from 0 to the age plus one, the log of the wood
    /// a bud of it is expected to grow from then (`-INF` for none).
    pub log_woods: Vec<Vec<f32>>,
    /// The widest window.
    pub span: f32,
    pub age: u32,
}

impl ChainTables {

    /// The tables of `species` grown to `age` through the one cycle
    /// (fn-255), whose buds may read light and which decides no lasting
    /// before growth: the device's units and wake (fn-256).
    pub fn cycle(species: &Species, age: u32) -> Result<Self> {
        species.validate()?;
        if age == 0 {
            return crate::error::refuse("age", "a tree grows at least one cycle");
        }
        let mut rules = crate::grow::alone::drawn(species, age, 0);
        rules.lasting = vec![None; species.states.len()].into();
        Ok(Self::of(species, &rules))
    }

    /// The tables of `species` by `rules`' laterals, lasting and windows.
    fn of(species: &Species, rules: &crate::grow::Rules) -> Self {
        let age = rules.age;
        let pas = (0..species.states.len())
            .map(|pa| {
                let s = &species.states[pa];
                let rises = s.rises().then(|| s.abortion_rise as f32);
                let abortion = (s.abortion > 0.0).then(|| {
                    (
                        threshold(s.abortion),
                        neg_log2_of(1.0 - s.abortion),
                        rises,
                    )
                });
                let stake = 1.0 - s.relay_ended;
                PaRow {
                    life: ticks(s.life()),
                    lives: s.lifespan > 0.0,
                    lived: species.lived(pa).map(|l| l.0),
                    viability: (threshold(s.viability), neg_log2_of(s.viability)),
                    abortion,
                    relay: [s.relay, s.relay_ended, s.relay_failed].map(threshold),
                    successor: species
                        .successor(pa)
                        .map(|(next, go)| (next, threshold(go), logit_of(go))),
                    moved: ((RATE * stake) as f32, (FLOOR * stake) as f32),
                    delay: s.shedding.is_finite().then(|| (s.shedding + 1.0) as f32),
                    lasting: rules.lasting[pa].map(|l| l as f32),
                    relay_at: s.relay_at.clamp(0.0, 1.0) as f32,
                    stakes: [1.0, 1.0 - s.relay_failed, 1.0 - s.relay]
                        .map(|stake| ((RATE * stake) as f32, (FLOOR * stake) as f32)),
                    living: (
                        q(species.successor(pa).map_or(0.0, |(_, go)| go)),
                        q(s.relay_ended),
                    ),
                    reaches: reaches(species, pa),
                    fade_delay: if s.shedding.is_finite() {
                        q(s.shedding + 1.0)
                    } else {
                        0.0
                    },
                }
            })
            .collect();
        let log_woods = (0..species.states.len())
            .map(|pa| {
                // Cycle 0 grows nothing and reads none.
                (0..=age + 1)
                    .map(|c| match c {
                        0 => -INF,
                        _ => crate::contract::log32(rules.windows.log_wood(pa, c)),
                    })
                    .collect()
            })
            .collect();
        Self {
            pas,
            units: Units::of(species, rules),
            log_woods,
            span: SPAN as f32,
            age,
        }
    }
}

/// Each stage a bud of `pa` reaches, its reach: the age it grows in
/// and each it moves on to (`dormant::aged`'s walk, its products real).
fn reaches(species: &Species, pa: usize) -> Vec<f32> {
    let mut out = Vec::new();
    let Some((mut at, mut reach)) = species.lived(pa) else {
        return out;
    };
    loop {
        out.push(q(reach));
        let Some((next, go)) = species.successor(at) else {
            return out;
        };
        (at, reach) = (next, reach * go);
    }
}
