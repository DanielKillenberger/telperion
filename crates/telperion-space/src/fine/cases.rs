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
use crate::structure::{Axis, Origin, Vec3};

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
    /// The presences of the draws that made its first axis, Q32, as its
    /// own grower takes them (`grow::alone::start`).
    pub birth: [u64; 2],
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
            let r = Root::shed(species, &rules, h, assigned, crate::clock::seconds());
            let mut kept = chain_axes(r.axes());
            if let Some(first) = kept.first_mut() {
                first.node = node;
            }
            Ok(RealizedCase {
                bud: input(species, request.seed, Some(&h.bud)),
                made: (h.bud.axis.vigour, whorl),
                birth: super::expansion::birth(&h.bud),
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

/// Each fine root of `fine` (by its stand-in's order): its kept axes, its
/// first axis's parent none and the rest numbered among the root's, each
/// with its phytomers' tips and radii, and its bearer's node as laid,
/// which its positions are taken from; none where the root was shed.
pub fn root_axes(fine: &super::Fine) -> Vec<Option<RootAxes>> {
    let axes = &fine.structure.axes;
    let mut live = 0;
    fine.roots
        .iter()
        .map(|r| {
            let at = r.at?;
            let (from, to) = (fine.starts[live], fine.starts[live + 1]);
            live += 1;
            let Origin::Lateral { parent, node, .. } = axes[at].origin else {
                unreachable!("a fine root is a lateral bud")
            };
            let origin = axes[parent].phytomers[node].tip;
            let mut stub = vec![axes[parent].clone()];
            let local = |p: usize| if p == at { 1 } else { p - from + 2 };
            for i in std::iter::once(at).chain(from..to) {
                let mut a = axes[i].clone();
                match &mut a.origin {
                    Origin::Lateral { parent, .. } if i == at => *parent = 0,
                    Origin::Lateral { parent, .. }
                    | Origin::Continuation { parent }
                    | Origin::Relay { parent, .. } => *parent = local(*parent),
                    Origin::Seed => unreachable!("a root's axes are borne"),
                }
                stub.push(a);
            }
            let tips = stub[1..]
                .iter()
                .map(|a| a.phytomers.iter().map(|p| (p.tip, p.radius)).collect());
            Some((chain_axes(&stub).into_iter().zip(tips).collect(), origin))
        })
        .collect()
}

/// The roots `settled` laid on the CPU (host decision 3), by root, as
/// `root_axes` gives a root: its axes with their tips and radii, and its
/// bearer's node as laid.
pub fn landed(settled: &super::Settled) -> Vec<(usize, RootAxes)> {
    settled
        .landed
        .iter()
        .map(|(j, root)| {
            let axes = root.axes();
            let tips = axes[1..]
                .iter()
                .map(|a| a.phytomers.iter().map(|p| (p.tip, p.radius)).collect());
            let mut chained = chain_axes(axes);
            if let Some(first) = chained.first_mut() {
                first.node = root.node;
            }
            let kept = chained.into_iter().zip(tips).collect();
            (settled.live[*j], (kept, axes[0].phytomers[0].tip))
        })
        .collect()
}

/// A root's kept axes with their tips and radii, and its origin.
pub type RootAxes = (Vec<(ChainAxis, Vec<(Vec3, f64)>)>, Vec3);
