//! A coarse tree's fine roots in the stepped executor (fn-214 task 2,
//! host option 1): each root the cut hands over at birth stands as an
//! aggregate (`couple::Aggregate`, fn-211.4) in place of a grower of its
//! own, and every crossing of the four phases is its expectation:
//!
//! - (i) its subtree's light, upkeep and living, from its cohorts, wood
//!   and the light it read; the coarse grower decides its shedding at its
//!   placeholder and hands down its kept share;
//! - (ii) its lit presence, and the vigour its placeholder is given;
//! - (iii) its per-PA normaliser partials, from its cohorts and its
//!   vigour per presence;
//! - (iv) its last unit's leaves spread over its closed form row's
//!   extent, into the one lattice, and the light at its centroid read.
//!
//! What it stands for at the end is the coarse field's (`fine/coarse.rs`).
use super::handover::{push, reparent, Rest, Tail, To};
use super::Executor;
use crate::allocation::drawn;
use crate::closed_form::boundary::Rows;
use crate::couple::{Aggregate, Inputs};
use crate::grow::Apex;
use crate::presence::Draws;
use crate::structure::{Axis, Vec3};
use crate::transcendental::Transcendental;
use std::sync::Arc;

/// The stencil a root's leaves are spread over: its centroid and six
/// points on its frame's axes, each a seventh (`couple/stand.rs`).
const STENCIL: f64 = 1.0 / 7.0;

/// A fine root standing as its aggregate: its placeholder in the coarse
/// grower, its first axis (no phytomers), the draws that made it, its
/// base scale, its aggregate, the light it last read, its kept share,
/// its vigour per presence, and the cycle it was shed or died out in.
pub(crate) struct Stood {
    pub at: usize,
    pub axis: Axis,
    pub birth: [f64; 2],
    pub scale: f64,
    pub agg: Aggregate,
    pub light: f64,
    pub kept: f64,
    ratio: f64,
    pub died: Option<u32>,
}

/// The roots standing, and the closed form's rows their extents read.
pub(crate) struct Stands {
    pub rows: Arc<Rows>,
    pub roots: Vec<Stood>,
}

impl Stands {
    fn living_mut(&mut self) -> impl Iterator<Item = &mut Stood> {
        self.roots.iter_mut().filter(|r| r.died.is_none())
    }

    fn living(&self) -> impl Iterator<Item = &Stood> {
        self.roots.iter().filter(|r| r.died.is_none())
    }
}

impl Stood {
    fn presence(&self) -> f64 {
        self.scale * self.agg.living() * self.kept
    }
}

impl Executor<'_> {
    /// Phase (i), up: each living root's light, upkeep and whether it
    /// lives, at its placeholder.
    pub(super) fn stand_totals(&self) -> Vec<(usize, f32, f32, bool)> {
        let Some(stands) = self.stands.as_ref() else {
            return Vec::new();
        };
        let species = self.coarse.species;
        let total = |r: &Stood| {
            let living = r.agg.living();
            let upkeep = species.states[r.axis.pa].upkeep;
            let light = (r.scale * living * r.light) as f32;
            (
                r.at,
                light,
                (upkeep * r.scale * r.agg.wood) as f32,
                living > 0.0,
            )
        };
        stands.living().map(total).collect()
    }

    /// Phase (i), down: each living root shed, or its kept share.
    pub(super) fn stand_kept(&mut self, cycle: u32, shed: &[bool], kept: &[f64]) {
        let Some(stands) = self.stands.as_mut() else {
            return;
        };
        for r in stands.living_mut() {
            match shed[r.at] {
                true => r.died = Some(cycle),
                false => r.kept = kept[r.at],
            }
        }
    }

    /// Phase (ii), up: each living root's lit presence at its placeholder.
    pub(super) fn stand_lit(&self) -> Vec<(usize, f64)> {
        let Some(stands) = self.stands.as_ref() else {
            return Vec::new();
        };
        stands
            .living()
            .map(|r| (r.at, r.presence() * r.light))
            .collect()
    }

    /// Phases (ii) down and (iii) up: each living root's vigour per
    /// presence from what its placeholder was given (`got`), and its
    /// per-PA partials added into `total` (presence, presence × (v/p)^ψ).
    pub(super) fn stand_sums(&mut self, got: &[f64], total: &mut [f32]) {
        let species = self.coarse.species;
        let Some(stands) = self.stands.as_mut() else {
            return;
        };
        let mut by_pa = vec![0.0; species.states.len()];
        for r in stands.living_mut() {
            let presence = r.presence();
            r.ratio = if presence > 0.0 {
                got[r.at] / presence
            } else {
                0.0
            };
            if presence <= 0.0 {
                continue;
            }
            by_pa.iter_mut().for_each(|x| *x = 0.0);
            r.agg.living_by_pa(r.scale * r.kept, &mut by_pa);
            for (g, &pg) in by_pa.iter().enumerate().filter(|(_, &pg)| pg > 0.0) {
                let p = pg as f32;
                let d = drawn(r.ratio as f32, 1.0, species.states[g].shade_size as f32);
                total[2 * g] += p;
                total[2 * g + 1] += d * p;
            }
        }
    }

    /// Phase (iv)'s growth: each living root's aggregate grows its unit
    /// of `cycle`, sized against each PA's merged sums (`factors`); one
    /// that dies out dies in `cycle`.
    pub(super) fn stand_advance(&mut self, cycle: u32, factors: &[[f32; 2]]) {
        let (species, threads) = (self.coarse.species, self.coarse.threads);
        let laterals = self.coarse.rules.laterals.clone();
        let Some(stands) = self.stands.as_mut() else {
            return;
        };
        // Each PA's mean (v/p)^ψ; whole where nothing is allotted.
        let mean = |g: usize| match factors.get(g) {
            Some(&[t, w]) if t > 0.0 && w > 0.0 => f64::from(w / t),
            _ => 1.0,
        };
        let means: Vec<f64> = (0..species.states.len()).map(mean).collect();
        let live: Vec<&mut Stood> = stands.living_mut().collect();
        crate::cores::map(live, crate::cores::PIECE, threads, &|_, r| {
            // Each PA's unit size, where any of its apexes lives.
            let mut size = vec![1.0; means.len()];
            for g in r.agg.pas() {
                let psi = species.states[g].shade_size;
                if psi != 0.0 {
                    size[g] = r.ratio.powf_fixed(psi) / means[g];
                }
            }
            let at = Inputs {
                light: r.light,
                kept: r.kept,
                size: &size,
            };
            r.agg.advance(species, &laterals, at);
            if r.agg.living() <= 0.0 {
                r.died = Some(cycle);
            }
        });
    }

    /// Phase (iv)'s light, up: each living root's leaves of `cycle`
    /// spread over its row's extent then, about its expected centroid,
    /// and the centroid it reads its light at, in the roots' order.
    pub(super) fn stand_leaves(&self, cycle: u32) -> (Vec<(Vec3, f64)>, Vec<Vec3>) {
        let Some(stands) = self.stands.as_ref() else {
            return (Vec::new(), Vec::new());
        };
        let age = self.coarse.rules.age;
        let (mut leaves, mut reads) = (Vec::new(), Vec::new());
        for r in stands.living() {
            let years = cycle.saturating_sub(r.axis.birth).max(1);
            let row = stands.rows.lateral(r.axis.pa, age.saturating_sub(years));
            let a = &self.coarse.axes[r.at];
            let (heading, side) = (a.heading, a.side);
            let at = a.base + heading * (row.along * r.scale);
            reads.push(at);
            let area = r.scale * r.agg.leaves;
            if area <= 0.0 {
                continue;
            }
            // The spread about the centroid, from the spread about the base.
            let about = (row.spread.powi(2) - row.along.powi(2)).max(0.0).sqrt();
            let reach = about * r.scale / 3f64.sqrt() * 3.5f64.sqrt();
            let up = heading.cross(side);
            leaves.push((at, area * STENCIL));
            for d in [heading, side, up] {
                leaves.push((at + d * reach, area * STENCIL));
                leaves.push((at - d * reach, area * STENCIL));
            }
        }
        (leaves, reads)
    }

    /// Phase (iv)'s light, down: each living root's read, in the roots'
    /// order.
    pub(super) fn stand_read(&mut self, read: &[f64]) {
        if let Some(stands) = self.stands.as_mut() {
            for (r, &light) in stands.living_mut().zip(read) {
                r.light = light;
            }
        }
    }

    /// Hands every fine root the coarse grower made since axis
    /// `first_new` to a stand: its placeholder kept as `hand_over` keeps
    /// it, its aggregate begun from its bud; every axis, apex and sleeping
    /// bud on it is the aggregate's, never grown.
    pub(super) fn stand_over(&mut self, first_new: usize) {
        let to = self.destinations(first_new);
        if to.iter().all(|t| matches!(t, To::Coarse(_))) {
            return;
        }
        let c = &mut self.coarse;
        let at = |i: usize| match i {
            i if i < first_new => i,
            i => match to[i - first_new] {
                To::Coarse(j) | To::Root(_, j) => j,
                To::Item(..) => unreachable!("the coarse grower holds nothing on a fine root"),
            },
        };
        let kept = |i: usize| i < first_new || matches!(to[i - first_new], To::Coarse(_));
        let spent = |i: usize| c.live.iter().find(|a| a.axis == i).map_or(0.0, |a| a.spent);
        let spent: Vec<f64> = (first_new..first_new + to.len()).map(spent).collect();
        let mut stood = Vec::new();
        for (n, (mut axis, rest)) in Tail::drained(c, first_new).into_iter().enumerate() {
            match to[n] {
                To::Coarse(_) => {
                    reparent(&mut axis.origin, at);
                    let successor = rest.successor.iter().map(|&s| at(s)).collect();
                    push(c, axis, Rest { successor, ..rest });
                }
                To::Root(_, j) => {
                    reparent(&mut axis.origin, at);
                    let pencil = rest.pencil.as_ref().expect("laid");
                    axis.phytomers.clear();
                    axis.units.clear();
                    stood.push(Stood {
                        at: j,
                        axis: axis.clone(),
                        birth: rest.draws.birth,
                        scale: pencil.base_scale,
                        agg: Aggregate::new(axis.pa, spent[n]),
                        light: pencil.light,
                        kept: pencil.kept,
                        ratio: 0.0,
                        died: None,
                    });
                    let held = Rest {
                        draws: Draws::default(),
                        units: 0.0,
                        successor: Vec::new(),
                        ..rest
                    };
                    push(c, axis, held);
                }
                To::Item(..) => {}
            }
        }
        for s in c.successor[..first_new].iter_mut().flatten() {
            *s = at(*s);
        }
        let live = std::mem::take(&mut c.live);
        c.live = (live.into_iter().filter(|a| kept(a.axis)))
            .map(|a| Apex {
                axis: at(a.axis),
                ..a
            })
            .collect();
        for list in c.asleep.iter_mut() {
            list.retain(|s| kept(s.parent));
            list.iter_mut().for_each(|s| s.parent = at(s.parent));
        }
        for woken in &mut c.woken {
            woken.axis = at(woken.axis);
            woken.bearer = at(woken.bearer);
        }
        let stands = self.stands.as_mut().expect("standing");
        stands.roots.extend(stood);
    }
}
