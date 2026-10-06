//! The GPU executor's surface under realized mechanics (fn-209 F1 task
//! 6): every fine root of a tree as the realized CPU executor grows it
//! (`realized.rs`), with its bud as the executor takes it, its axes as
//! grown and assigned before any shedding, the presences its fades read
//! (Q32, host decision 8), its axes kept by its own subtree's shedding,
//! and the living it reports up. The executor's tests hold the GPU's
//! against these.
use super::realized::Root;
use super::surface::input;
use super::{ChainAxis, ChainBud, Plan};
use crate::bounded::Expected;
use crate::error::Result;
use crate::fixed::presence::q;
use crate::grow::alone::rules;
use crate::grow::realized::{above, root, Hands};
use crate::grow::Request;
use crate::species::Species;
use crate::structure::{Axis, Origin};
use std::time::Instant;

/// One axis's presences as its fades read them, Q32: its vigour, each
/// growth unit's presence, each phytomer's scale by presence, its living
/// apex's presence and the share of its first cycle gone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Presences {
    pub vigour: u64,
    pub units: Vec<u64>,
    pub scales: Vec<u64>,
    pub alive: u64,
    pub sleep: u64,
}

/// A fine root as the realized CPU executor grows it.
#[derive(Debug, Clone)]
pub struct RealizedCase {
    pub bud: ChainBud,
    /// Its first axis's vigour as made and its node's buds a whorl.
    pub made: (f64, f64),
    /// Its axes as grown and assigned, before shedding, its first axis's
    /// parent none and the rest numbered among the root's.
    pub grown: Vec<ChainAxis>,
    pub presences: Vec<Presences>,
    /// Its axes kept by its own subtree's shedding (none where its first
    /// axis is shed).
    pub kept: Vec<ChainAxis>,
    /// Its subtree's living as reported up: lived (Q32.32), alive (Q32).
    pub living: (u64, u64),
}

/// Every fine root of `species` grown at `request` above `plan`'s cut,
/// in its stand-ins' order.
pub fn realized_cases(
    species: &Species,
    request: Request,
    plan: Plan,
) -> Result<Vec<RealizedCase>> {
    let rules = rules(species, request.age, request.budget)?;
    let expected = Expected::new(species, request.age, 0);
    let rows = expected.rows();
    let hands = Hands::new(species.states.len(), request.age, |pa, birth| {
        rows.lateral(pa, birth).count <= plan.cut
    });
    let threads = crate::cores::available();
    let (_, handed) = above(&rules, hands, request.seed, threads)?;
    handed
        .iter()
        .map(|h| {
            let assigned = root(&rules, &h.bud)?;
            let grown = chain_axes(&assigned.axes);
            let presences = assigned.axes[1..].iter().map(presences).collect();
            let Origin::Lateral { node, whorl, .. } = h.bud.axis.origin else {
                unreachable!("a fine root is a lateral bud")
            };
            let r = Root::shed(species, &rules, h, assigned, Instant::now());
            let mut kept = chain_axes(r.axes());
            if let Some(first) = kept.first_mut() {
                first.node = node;
            }
            Ok(RealizedCase {
                bud: input(species, request.seed, Some(&h.bud)),
                made: (h.bud.axis.vigour, whorl),
                grown,
                presences,
                kept,
                living: (r.living.lived, r.living.alive),
            })
        })
        .collect()
}

/// A root's axes past its bearer's stub (axis 0), as `ChainAxis`.
fn chain_axes(axes: &[Axis]) -> Vec<ChainAxis> {
    axes[1..]
        .iter()
        .map(|a| {
            let parent = |p: usize| p.checked_sub(1);
            let (kind, node, slot, woken, parent) = match a.origin {
                Origin::Lateral {
                    parent: p,
                    node,
                    slot,
                    woken,
                    ..
                } => (1, node, slot, woken, parent(p)),
                Origin::Continuation { parent: p } => (2, 0, 0, false, parent(p)),
                Origin::Relay { parent: p, node } => (3, node, 0, false, parent(p)),
                Origin::Seed => unreachable!("only the stub is the seed"),
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
                kept: true,
                context: None,
            }
        })
        .collect()
}

fn presences(a: &Axis) -> Presences {
    Presences {
        vigour: q(a.vigour),
        units: a.units.iter().map(|&u| q(u)).collect(),
        scales: a.phytomers.iter().map(|p| q(p.scale)).collect(),
        alive: q(a.alive),
        sleep: q(a.sleep),
    }
}
