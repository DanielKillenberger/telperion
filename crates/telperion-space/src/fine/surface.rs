//! The GPU executor's surface for a chain's growth (fn-209 F1, task 2): a
//! species point's per-PA rows, as integers on the contract's fixed point
//! (`fixed.rs`), every chain of a tree as the executor takes it (its bud),
//! and its topology as the CPU reference executor grows it, which the
//! executor's tests hold the GPU's against.
use super::tables::Units;
use crate::alone::{chains, seed_item};
use crate::bounded::Expected;
use crate::error::{refuse, Result};
use crate::fixed::{neg_log2, q32, q62, ticks, INF};
use crate::grow::alone::{rules, Bud, Chain};
use crate::grow::Request;
use crate::lineage::{threshold, Key};
use crate::presence::{FLOOR, RATE, SPAN};
use crate::species::Species;
use crate::structure::Origin;

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
    pub viability: (u64, i64),
    /// D6: none where it never aborts; else a whole, unrising unit's
    /// threshold, `-log2 (1 - a)`, and the rise (Q32) where it rises.
    pub abortion: Option<(u64, i64, Option<i64>)>,
    /// D9: the relay thresholds of a stop by abortion, end and failure.
    pub relay: [u64; 3],
    /// D8 and D10: the age it moves on to, the move's threshold and Q62
    /// chance, and its window's rate and floor (Q32) at its stake.
    pub successor: Option<(usize, u64, u64)>,
    pub moved: (i64, i64),
    /// D14: its shedding delay plus a cycle (Q32), where it sheds, and
    /// the bound on its subtree's life (Q32), where it has one.
    pub delay: Option<i64>,
    pub lasting: Option<i64>,
    /// D18: where along its last unit a relay of it stands (Q32).
    pub relay_at: u64,
}

/// A species point's tables for a chain's growth.
#[derive(Debug, Clone)]
pub struct ChainTables {
    pub pas: Vec<PaRow>,
    pub units: Units,
    /// Per PA, per cycle from 0 to the age plus one, the log of the wood
    /// a bud of it is expected to grow from then (Q32; `-INF` for none).
    pub log_woods: Vec<Vec<i64>>,
    /// The widest window (Q32).
    pub span: i64,
    pub age: u32,
}

impl ChainTables {
    /// The tables of `species` grown to `age`, for a tree no bud of which
    /// reads light, and none of which sleeps (dormancy is not yet on the
    /// GPU executor, and is refused by name).
    pub fn new(species: &Species, age: u32) -> Result<Self> {
        let rules = rules(species, age, 0)?;
        if species
            .states
            .iter()
            .any(|s| s.zones.iter().any(|z| z.dormant.iter().any(|&p| p > 0.0)))
        {
            return refuse("dormant", "the GPU executor grows no sleeping buds yet");
        }
        let pas = (0..species.states.len())
            .map(|pa| {
                let s = &species.states[pa];
                let rises = s.rises().then(|| q32(s.abortion_rise));
                let abortion = (s.abortion > 0.0).then(|| {
                    (
                        threshold(s.abortion),
                        neg_log2(q62(1.0 - s.abortion)),
                        rises,
                    )
                });
                let stake = 1.0 - s.relay_ended;
                PaRow {
                    life: ticks(s.life()),
                    lives: s.lifespan > 0.0,
                    lived: species.lived(pa).map(|l| l.0),
                    viability: (threshold(s.viability), neg_log2(q62(s.viability))),
                    abortion,
                    relay: [s.relay, s.relay_ended, s.relay_failed].map(threshold),
                    successor: species
                        .successor(pa)
                        .map(|(next, go)| (next, threshold(go), q62(go))),
                    moved: (q32(RATE * stake), q32(FLOOR * stake)),
                    delay: s.shedding.is_finite().then(|| q32(s.shedding + 1.0)),
                    lasting: rules.lasting[pa].map(q32),
                    relay_at: q32(s.relay_at.clamp(0.0, 1.0)) as u64,
                }
            })
            .collect();
        let log_woods = (0..species.states.len())
            .map(|pa| {
                // Cycle 0 grows nothing and reads none.
                (0..=age + 1)
                    .map(|c| match c {
                        0 => -INF,
                        _ => q32(rules.windows.log_wood(pa, c)),
                    })
                    .collect()
            })
            .collect();
        Ok(Self {
            pas,
            units: Units::new(species, age)?,
            log_woods,
            span: q32(SPAN),
            age,
        })
    }
}

/// A chain's bud as the executor takes it: the seed's, or a lateral's
/// first axis as its bearer made it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChainBud {
    pub seed: bool,
    pub lineage: u64,
    pub pa: usize,
    pub birth: u32,
    /// Its place on its bearer: node and slot.
    pub node: usize,
    pub slot: u8,
    /// The lineage its chain of ages began from.
    pub root: u64,
    /// Its horizon (Q32; `-INF` for none).
    pub horizon: i64,
}

/// A chain's axis as grown: its parent among the chain's axes (none: the
/// bearer), how it stands there, its lineage, PA and birth, where its
/// apex stopped, and its phytomers' cycles and keys. Borne laterals are
/// axes of no phytomers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainAxis {
    pub parent: Option<usize>,
    /// 0 seed, 1 lateral, 2 continuation, 3 relay.
    pub kind: u8,
    pub node: usize,
    pub slot: u8,
    pub lineage: u64,
    pub pa: usize,
    pub birth: u32,
    pub apex_end: Option<u32>,
    pub phytomers: Vec<(u32, u64)>,
}

/// Every chain of `species`'s tree grown alone as the CPU reference grows
/// it, breadth first, each with its bud and its axes.
pub fn chain_cases(species: &Species, request: Request) -> Result<Vec<(ChainBud, Vec<ChainAxis>)>> {
    let rules = rules(species, request.age, request.budget)?;
    let expected = Expected::new(species, request.age, 0);
    let rows = expected.rows();
    let mut out = Vec::new();
    let mut observe = |bud: Option<&Bud>, chain: &Chain| {
        out.push((input(species, request.seed, bud), topology(chain)));
    };
    let start = (Vec::new(), Vec::new(), seed_item());
    chains(
        &rules,
        &rows,
        request.seed,
        start,
        (&|_| false, &mut observe),
        true,
    )?;
    Ok(out)
}

fn input(species: &Species, seed: u64, bud: Option<&Bud>) -> ChainBud {
    let horizon = |h: f64| if h.is_finite() { q32(h) } else { -INF };
    match bud {
        None => {
            let key = Key::root(seed);
            let pa = species.lived(0).expect("a seed that grows").0;
            ChainBud {
                seed: true,
                lineage: key.onto(pa).0,
                pa,
                birth: 0,
                node: 0,
                slot: 0,
                root: key.0,
                horizon: -INF,
            }
        }
        Some(b) => {
            let Origin::Lateral { node, slot, .. } = b.axis.origin else {
                unreachable!("a bud is a lateral")
            };
            ChainBud {
                seed: false,
                lineage: b.axis.lineage,
                pa: b.axis.pa,
                birth: b.axis.birth,
                node,
                slot,
                root: b.root(),
                horizon: horizon(b.horizon()),
            }
        }
    }
}

fn topology(chain: &Chain) -> Vec<ChainAxis> {
    let root = chain.root;
    chain.axes[root..]
        .iter()
        .map(|a| {
            let parent = a.origin.parent().filter(|&p| p >= root).map(|p| p - root);
            let (kind, node, slot) = match a.origin {
                Origin::Seed => (0, 0, 0),
                Origin::Lateral { node, slot, .. } => (1, node, slot),
                Origin::Continuation { .. } => (2, 0, 0),
                Origin::Relay { node, .. } => (3, node, 0),
            };
            ChainAxis {
                parent,
                kind,
                node,
                slot,
                lineage: a.lineage,
                pa: a.pa,
                birth: a.birth,
                apex_end: a.apex_end,
                phytomers: a.phytomers.iter().map(|p| (p.cycle, p.key())).collect(),
            }
        })
        .collect()
}
