//! The GPU executor's surface for a chain's growth (fn-209 F1, task 2): a
//! species point's per-PA rows, as the contract's thresholds, ticks and
//! f32 values (`contract.rs`), every chain of a tree as the executor takes it (its bud),
//! and its topology as the CPU reference executor grows it, which the
//! executor's tests hold the GPU's against.
use super::tables::Units;
use crate::alone::{chains, seed_item, Seen};
use crate::bounded::Expected;
use crate::error::Result;
use crate::contract::presence::q;
use crate::contract::{neg_log2, ticks, INF};
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
    pub viability: (u64, f32),
    /// D6: none where it never aborts; else a whole, unrising unit's
    /// threshold, `-log2 (1 - a)`, and the rise where it rises.
    pub abortion: Option<(u64, f32, Option<f32>)>,
    /// D9: the relay thresholds of a stop by abortion, end and failure.
    pub relay: [u64; 3],
    /// D8 and D10: the age it moves on to, the move's threshold and
    /// chance, and its window's rate and floor at its stake.
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
    /// The tables of `species` grown to `age`, for a tree no bud of which
    /// reads light.
    pub fn new(species: &Species, age: u32) -> Result<Self> {
        let rules = rules(species, age, 0)?;
        let pas = (0..species.states.len())
            .map(|pa| {
                let s = &species.states[pa];
                let rises = s.rises().then(|| s.abortion_rise as f32);
                let abortion = (s.abortion > 0.0).then(|| {
                    (
                        threshold(s.abortion),
                        neg_log2((1.0 - s.abortion) as f32),
                        rises,
                    )
                });
                let stake = 1.0 - s.relay_ended;
                PaRow {
                    life: ticks(s.life()),
                    lives: s.lifespan > 0.0,
                    lived: species.lived(pa).map(|l| l.0),
                    viability: (threshold(s.viability), neg_log2(s.viability as f32)),
                    abortion,
                    relay: [s.relay, s.relay_ended, s.relay_failed].map(threshold),
                    successor: species
                        .successor(pa)
                        .map(|(next, go)| (next, threshold(go), go as f32)),
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
        Ok(Self {
            pas,
            units: Units::new(species, age)?,
            log_woods,
            span: SPAN as f32,
            age,
        })
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
    /// Its horizon, in cycles (`-INF` for none).
    pub horizon: f32,
    /// Where it slept and woke, for a bud that woke.
    pub woke: Option<Woke>,
    /// Its bearer's shedding context (D15), where its keeping is
    /// conditioned on it.
    pub context: Option<f32>,
}

/// A woken bud's waking (`ChainBud::woke`): the sleeping bud's key and
/// PA, the share of its waking cycle it slept (ticks), the stage it wakes
/// in and its time there (ticks), and the cycle it woke in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Woke {
    pub key: u64,
    pub pa: usize,
    pub sleep: u64,
    pub stage: usize,
    pub spent: u64,
    pub cycle: u32,
}

/// A chain (or a fine root) as the executor takes it, and its axes as the
/// CPU reference executor grows them.
pub type Case = (ChainBud, Vec<ChainAxis>);

/// A chain's axis as grown: its parent among the chain's axes (none: the
/// bearer), how it stands there, its lineage, PA and birth, where its
/// apex stopped, and its phytomers' cycles and keys. Borne laterals are
/// axes of no phytomers.
#[derive(Debug, Clone, PartialEq)]
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
    /// A sleeping bud that woke on the chain: a bud it bears, after its
    /// axes, one for every axis that carried its bearer on.
    pub woken: bool,
    /// D15: whether the chain keeps it (its own axes), and, for a bud it
    /// bears, the context its own chain's keeping is conditioned on.
    pub kept: bool,
    pub context: Option<f32>,
}

/// Every chain of `species`'s tree grown alone as the CPU reference grows
/// it, breadth first, each with its bud and its axes.
pub fn chain_cases(species: &Species, request: Request) -> Result<Vec<Case>> {
    let rules = rules(species, request.age, request.budget)?;
    let expected = Expected::new(species, request.age, 0);
    let rows = expected.rows();
    let mut out = Vec::new();
    let mut observe = |seen: Seen| {
        let bud = ChainBud {
            context: seen.context,
            ..input(species, request.seed, seen.bud)
        };
        out.push((bud, topology(seen.chain, seen.kept, seen.contexts)));
    };
    let start = (Vec::new(), Vec::new(), seed_item());
    chains(&rules, &rows, request.seed, start, &mut observe, true)?;
    Ok(out)
}

pub(super) fn input(species: &Species, seed: u64, bud: Option<&Bud>) -> ChainBud {
    let horizon = |h: f64| if h.is_finite() { h as f32 } else { -INF };
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
                woke: None,
                context: None,
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
                woke: b.woke().map(|(s, (stage, spent), cycle)| Woke {
                    key: s.key,
                    pa: s.pa,
                    sleep: ticks(s.sleep),
                    stage,
                    spent: ticks(spent),
                    cycle,
                }),
                context: None,
            }
        }
    }
}

fn topology(chain: &Chain, kept: &[bool], contexts: &[Option<f32>]) -> Vec<ChainAxis> {
    let root = chain.root;
    let woken = chain.buds.iter().filter(|b| b.woken()).map(|b| &b.axis);
    // Each bud's context by its axis: the laterals in the chain's order,
    // then the woken buds after them, as `chain.buds` holds them.
    let laterals = chain.buds.iter().filter(|b| !b.woken()).count();
    let mut context = vec![None; chain.axes.len() - root + chain.buds.len() - laterals];
    let mut lateral = (root + 1..chain.axes.len())
        .filter(|&i| matches!(chain.axes[i].origin, Origin::Lateral { .. }));
    for (k, b) in chain.buds.iter().enumerate() {
        let at = match b.woken() {
            false => lateral.next().expect("a lateral axis per bud") - root,
            true => chain.axes.len() - root + (k - laterals),
        };
        context[at] = contexts[k];
    }
    chain.axes[root..]
        .iter()
        .chain(woken)
        .enumerate()
        .map(|(k, a)| {
            let parent = a.origin.parent().filter(|&p| p >= root).map(|p| p - root);
            let (kind, node, slot, woken) = match a.origin {
                Origin::Seed => (0, 0, 0, false),
                Origin::Lateral {
                    node, slot, woken, ..
                } => (1, node, slot, woken),
                Origin::Continuation { .. } => (2, 0, 0, false),
                Origin::Relay { node, .. } => (3, node, 0, false),
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
                woken,
                kept: kept.get(root + k).copied().unwrap_or(false),
                context: context[k],
            }
        })
        .collect()
}
