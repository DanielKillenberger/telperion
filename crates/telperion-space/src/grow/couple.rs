//! The coupling pass in the grower (fn-211.4; `couple.rs`): the oak's
//! structure above fn-209's cut grown year by year with today's passes,
//! each fine root an aggregate state in them.
//!
//! A lateral whose expected kept phytomers (the closed form's `count`,
//! light-neutral) are at most the cut leaves the living apexes when its
//! pencil is made and becomes a fine root. Every cycle it takes part in
//! the balance, the allocation and the light as its subtree would, and
//! its records are written; in the full re-lays it stands with its
//! expected section and load (`couple/stand.rs`).
use crate::transcendental::Transcendental;
mod stand;

use super::sketch::Sketch;
use super::{Apex, Grower};
use crate::closed_form::boundary::Rows;
use crate::couple::{Aggregate, Inputs, Records};
use crate::dormant::Sleeper;
use crate::fixed::sum::Sum;
use crate::presence::Draws;
use crate::structure::{Axis, Origin, Vec3};
pub(crate) use stand::Probe;
use std::collections::HashMap;
use std::sync::Arc;

/// How a fine subtree takes its root's recorded vigour (probe P4's
/// question 1, open for the host): its whole recorded share, or that
/// share per light its aggregate collected times the light the subtree
/// itself collects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Split {
    Recorded,
    PerLight,
}

/// The coupling pass beside the grower.
pub(crate) struct Pass {
    pub rows: Arc<Rows>,
    pub cut: f64,
    pub split: Split,
    pub fine: Vec<FineRoot>,
    /// Each fine root by its axis.
    pub of_axis: HashMap<usize, usize>,
    /// Per cycle, each PA's mean of (v/p)^ψ over coupling apexes and fine
    /// roots alike (decision 23), which every fine subtree reads.
    pub means: Vec<Vec<f64>>,
    /// Per cycle and PA, the coupling apexes' presence and presence times
    /// size (decision 4's drift).
    pub growth: Vec<Vec<[f64; 2]>>,
    pub coupled: usize,
    /// The pass's last field before the tree's last cycle.
    pub probe: Probe,
}

/// A fine root: its axis in the pass, its base scale, its aggregate, its
/// records and what its subtree is grown from.
pub(crate) struct FineRoot {
    pub axis: usize,
    pub pa: usize,
    pub birth: u32,
    pub scale: f64,
    pub agg: Aggregate,
    pub records: Records,
    pub seed: FineSeed,
    /// Its vigour per presence this cycle.
    ratio: f64,
    pub dead: bool,
}

/// What a fine subtree is grown from: its first axis as the pass left it,
/// its draws and lines, the cycle its growth carries on from, and the
/// sleeping buds its first unit put to sleep.
#[derive(Clone)]
pub(crate) struct FineSeed {
    pub axis: Axis,
    pub draws: Draws,
    pub root: u64,
    pub horizon: f64,
    pub units: f64,
    pub spent: f64,
    pub from: u32,
    pub asleep: Vec<(usize, Sleeper)>,
}

impl Pass {
    pub fn new(rows: Arc<Rows>, cut: f64, split: Split) -> Self {
        Self {
            rows,
            cut,
            split,
            fine: Vec::new(),
            of_axis: HashMap::new(),
            means: Vec::new(),
            growth: Vec::new(),
            coupled: 0,
            probe: Probe::default(),
        }
    }

    pub(super) fn living(&self) -> impl Iterator<Item = &FineRoot> {
        self.fine.iter().filter(|f| !f.dead)
    }
}

/// Grows `species` as `grow` does, above `cut` year by year with today's
/// passes and every fine root an aggregate in them, then each fine
/// subtree alone from its records, joined and laid whole: the tree and
/// the pass measured.
pub fn grow_coupled(
    species: &crate::species::Species,
    request: super::Request,
    expected: &crate::bounded::Expected,
    (cut, split): (f64, Split),
) -> crate::error::Result<(crate::structure::Structure, super::subtree::Measured)> {
    let beside = super::Beside {
        couple: Some(Pass::new(expected.rows(), cut, split)),
        ..super::Beside::default()
    };
    let quiet = &mut |_| {};
    let options = super::Options::default();
    let (grown, besides) = super::run_beside(species, request, (quiet, true), options, beside)?;
    let mut m = besides.measured.expect("measured");
    let index = |i: usize| grown.index.get(i).copied().unwrap_or(usize::MAX);
    let mut tag = vec![usize::MAX; grown.structure.axes.len()];
    for (old, &k) in m.tag.iter().enumerate() {
        if let Some(t) = tag.get_mut(index(old)) {
            *t = k;
        }
    }
    m.tag = tag;
    m.reads.iter_mut().for_each(|r| r.0 = index(r.0));
    Ok((grown.structure, m))
}

impl Grower<'_> {
    /// Whether the new axis `i`, of base scale `scale`, roots a fine
    /// subtree; if so it leaves the living apexes and the pass takes it.
    pub(super) fn classify(&mut self, i: usize, scale: f64, cycle: u32) -> bool {
        let Some(pass) = self.couple.as_mut() else {
            return false;
        };
        let axis = &self.axes[i];
        if !matches!(axis.origin, Origin::Lateral { .. }) {
            return false;
        }
        if pass.rows.lateral(axis.pa, axis.birth).count > pass.cut {
            pass.coupled += 1;
            return false;
        }
        let spent = self
            .live
            .iter()
            .find(|a| a.axis == i)
            .map_or(0.0, |a| a.spent);
        self.live.retain(|a: &Apex| a.axis != i);
        let mut asleep = Vec::new();
        for (wakes, list) in self.asleep.iter_mut().enumerate() {
            list.retain(|s| {
                let mine = s.parent == i;
                if mine {
                    asleep.push((wakes, *s));
                }
                !mine
            });
        }
        let pass = self.couple.as_mut().expect("coupling");
        pass.of_axis.insert(i, pass.fine.len());
        pass.fine.push(FineRoot {
            axis: i,
            pa: axis.pa,
            birth: axis.birth,
            scale,
            agg: Aggregate::new(axis.pa, spent),
            records: Records::starting(cycle + 1),
            seed: FineSeed {
                axis: axis.clone(),
                draws: self.draws[i].clone(),
                root: self.roots[i],
                horizon: self.horizon[i],
                units: self.units[i],
                spent,
                from: cycle + 1,
                asleep,
            },
            ratio: 0.0,
            dead: false,
        });
        true
    }

    /// Each living fine root's light, upkeep and whether it lives, for
    /// the balance: `(axis, light, cost, living)`.
    pub(super) fn fine_balance(&self, sketch: &Sketch) -> Vec<(usize, f64, f64, bool)> {
        let Some(pass) = self.couple.as_ref() else {
            return Vec::new();
        };
        pass.living()
            .map(|f| {
                let living = f.agg.living();
                let upkeep = self.species.states[f.pa].upkeep;
                let light = f.scale * living * sketch.pencils[f.axis].light;
                (f.axis, light, upkeep * f.scale * f.agg.wood, living > 0.0)
            })
            .collect()
    }

    /// What the balance made of each fine root: shed, or its kept share.
    pub(super) fn fine_kept(&mut self, cycle: u32, shed: &[bool], kept: &[f64]) {
        let (Some(pass), Some(sketch)) = (self.couple.as_mut(), self.sketch.as_mut()) else {
            return;
        };
        for f in pass.fine.iter_mut().filter(|f| !f.dead) {
            if shed[f.axis] {
                f.dead = true;
                f.records.shed = Some(cycle);
                continue;
            }
            f.records.set_kept(cycle, kept[f.axis]);
            sketch.pencils[f.axis].kept = kept[f.axis];
        }
    }

    /// Each living fine root's presence and lit presence, by axis.
    pub(super) fn fine_presence(&self, sketch: &Sketch) -> Vec<(usize, f64, f64)> {
        let Some(pass) = self.couple.as_ref() else {
            return Vec::new();
        };
        pass.living()
            .map(|f| {
                let p = &sketch.pencils[f.axis];
                let presence = f.scale * f.agg.living() * p.kept;
                (f.axis, presence, presence * p.light)
            })
            .collect()
    }

    /// Each PA's presence and presence-weighted (v/p)^ψ over the coupling
    /// apexes (their vigour `got`, presence and ψ by axis, PA by `pa`),
    /// summed in index order as `allocation::sizes` sums them, then over
    /// the fine roots; each fine root's vigour and light recorded, and
    /// each PA's mean recorded for the fine subtrees.
    pub(super) fn fine_means(
        &mut self,
        cycle: u32,
        got: &[f64],
        (presence, psi, pa): (&[f64], &[f64], &[usize]),
    ) -> Vec<[f64; 2]> {
        let species = self.species;
        let pas = species.states.len();
        let mut acc = vec![[Sum::default(); 2]; pas];
        for i in (0..got.len()).filter(|&i| presence[i] > 0.0) {
            acc[pa[i]][0].add(presence[i]);
            acc[pa[i]][1].add((got[i] / presence[i]).powf_fixed(psi[i]) * presence[i]);
        }
        let value = |acc: &[[Sum; 2]]| -> Vec<[f64; 2]> {
            acc.iter().map(|[t, w]| [t.value(), w.value()]).collect()
        };
        let (Some(pass), Some(sketch)) = (self.couple.as_mut(), self.sketch.as_ref()) else {
            return value(&acc);
        };
        let mut by_pa = vec![0.0; pas];
        for f in pass.fine.iter_mut().filter(|f| !f.dead) {
            f.records.set_vigour(cycle, got[f.axis]);
            let p = &sketch.pencils[f.axis];
            let presence = f.scale * f.agg.living() * p.kept;
            f.records.set_q(cycle, presence * p.light);
            f.ratio = if presence > 0.0 {
                got[f.axis] / presence
            } else {
                0.0
            };
            if presence <= 0.0 {
                continue;
            }
            by_pa.iter_mut().for_each(|x| *x = 0.0);
            f.agg.living_by_pa(f.scale * p.kept, &mut by_pa);
            for (g, &pg) in by_pa.iter().enumerate().filter(|(_, &pg)| pg > 0.0) {
                acc[g][0].add(pg);
                acc[g][1].add(f.ratio.powf_fixed(species.states[g].shade_size) * pg);
            }
        }
        let sums = value(&acc);
        let means = sums
            .iter()
            .map(|&[t, w]| if t > 0.0 && w > 0.0 { w / t } else { 1.0 });
        let means: Vec<f64> = means.collect();
        if pass.means.len() <= cycle as usize {
            pass.means.resize(cycle as usize + 1, vec![1.0; pas]);
        }
        pass.means[cycle as usize] = means;
        sums
    }

    /// Every living fine root's aggregate grows its unit of `cycle`, on
    /// every core; an extinct one grows nothing and casts nothing.
    pub(super) fn advance_fine(&mut self, cycle: u32) {
        let (species, laterals, threads) = (self.species, &self.rules.laterals, self.threads);
        let (Some(pass), Some(sketch)) = (self.couple.as_mut(), self.sketch.as_ref()) else {
            return;
        };
        let means = pass
            .means
            .get(cycle as usize)
            .cloned()
            .unwrap_or_else(|| vec![1.0; species.states.len()]);
        let mut live: Vec<&mut FineRoot> = pass.fine.iter_mut().filter(|f| !f.dead).collect();
        crate::cores::map(
            live.iter_mut().collect(),
            crate::cores::PIECE,
            threads,
            &|_, f| {
                let p = &sketch.pencils[f.axis];
                let size: Vec<f64> = species
                    .states
                    .iter()
                    .zip(&means)
                    .map(|(s, m)| match s.shade_size {
                        0.0 => 1.0,
                        psi => f.ratio.powf_fixed(psi) / m,
                    })
                    .collect();
                let at = Inputs {
                    light: p.light,
                    kept: p.kept,
                    size: &size,
                };
                f.agg.advance(species, laterals, at);
            },
        );
    }

    /// Records each living fine root's light, read for its growth in
    /// `cycle + 1`.
    pub(super) fn fine_lit(&mut self, cycle: u32, read: &[(usize, f64)]) {
        let (Some(pass), Some(sketch)) = (self.couple.as_mut(), self.sketch.as_mut()) else {
            return;
        };
        for &(i, light) in read {
            let k = pass.of_axis[&i];
            pass.fine[k].records.set_light(cycle + 1, light);
            sketch.pencils[i].light = light;
        }
    }

    /// The axes of the fine roots whose aggregates live: they stand in the
    /// full re-lays.
    pub(super) fn fine_standing(&self) -> Vec<usize> {
        self.couple.as_ref().map_or_else(Vec::new, |pass| {
            pass.living()
                .filter(|f| f.agg.alive())
                .map(|f| f.axis)
                .collect()
        })
    }

    /// The frame a fine root's stand-in reads: its axis's base, heading
    /// and side as the pass last laid them.
    pub(super) fn fine_frame(&self, i: usize) -> (Vec3, Vec3, Vec3) {
        let a = &self.axes[i];
        (a.base, a.heading, a.side)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded::Expected;
    use crate::grow::{grow, Request};
    use crate::light::Light;

    fn request(age: u32) -> Request {
        Request {
            age,
            seed: 1,
            budget: 20_000_000,
            light: Light {
                extinction: 0.5,
                sky: 0.5,
            },
        }
    }

    /// With no bud below the cut the pass is today's tree to the bit.
    #[test]
    fn a_pass_with_no_fine_root_is_todays_tree() {
        let species = crate::oak();
        let r = request(24);
        let expected = Expected::new(&species, r.age, 0);
        let (tree, m) = grow_coupled(&species, r, &expected, (-1.0, Split::Recorded)).unwrap();
        assert_eq!(m.fine, 0);
        let today = grow(&species, r).unwrap();
        assert_eq!(crate::fine::digest(&tree), crate::fine::digest(&today));
    }

    /// Below the cut, fine roots are taken, recorded and grown back: the
    /// tree keeps its trunk and every axis is either above the cut or in
    /// a fine root's subtree.
    #[test]
    fn fine_roots_are_grown_from_their_records() {
        let species = crate::oak();
        let r = request(24);
        let expected = Expected::new(&species, r.age, 0);
        let (tree, m) = grow_coupled(&species, r, &expected, (64.0, Split::PerLight)).unwrap();
        assert!(m.fine > 0 && m.fine_grown > 0, "{} roots", m.fine);
        assert_eq!(m.tag.len(), tree.axes.len());
        assert_eq!(m.tag[0], usize::MAX, "the trunk is above the cut");
        assert!(m.record_bytes > 0);
    }
}
