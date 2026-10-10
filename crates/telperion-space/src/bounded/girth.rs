//! A chain's girth against its committed base (fn-209, option 4).
//!
//! The chain's own profile is the pipe model over what it holds: each of
//! its phytomers' own pipe, and at each node every bud it bore, kept or
//! shed, by that bud's committed section, E[S^(e_p / e_c); kept] taken
//! from its boundary by the delta method (`closed_form::kept::section`),
//! plus the share of its pipe the bearer retains (the oak's `retained`),
//! as the boundary's grown section less its kept one.
//!
//! The profile is then fitted to the chain's own committed base radius
//! R, its boundary's E[r | kept], blended with the radius it was
//! established at as secondary growth blends it. With r₀ the profile's
//! radius at the chain's first phytomer and ρ = R / r₀, each phytomer's
//! radius r becomes
//!
//!   r · ρ^(q^β),  q = min(1, r / r₀),  β = min(1, 1 / |ln ρ|):
//!
//! the base meets R exactly (q = 1), and the correction fades with the
//! share of the base radius the wood keeps, to none at a fine tip, so
//! the chain follows its own wood along its length. β keeps the radius
//! falling wherever the profile falls: d ln r' / d ln q = 1 + β q^β ln ρ
//! ≥ 1 − β |ln ρ| ≥ 0. The fit is the chain's alone; nothing it bears is
//! scaled by it.
use super::Bounded;
use crate::closed_form::boundary::Row;
use crate::closed_form::kept::section::delta;
use crate::girth::ripe;
use crate::grow::Record;
use crate::species::Species;
use crate::structure::{Axis, Origin};
use crate::transcendental::Transcendental;

/// What `bud_frame` reads of a record.
pub(super) fn bud(record: &Record) -> (Origin, usize, u64) {
    (record.origin, record.pa, record.lineage)
}

impl Bounded<'_> {
    /// A record's committed section handed to a bearer of exponent `e`.
    pub(crate) fn handed(&self, record: &Record, e: f64) -> f64 {
        let row = self.rows.lateral(record.pa, record.birth);
        let s = self.base_scale(record);
        let (ec, k) = (row.e, row.kept);
        let (p1, p2) = (s.powf_fixed(ec), s.powf_fixed(2.0 * ec));
        let r = e / ec;
        let kept = delta(k, row.s1 * p1, row.s2 * p2, r);
        let share = self.species.states[record.pa].retained;
        if share <= 0.0 {
            return kept;
        }
        let lost1 = (row.grown1 - row.s1).max(0.0) * p1;
        let lost2 = (row.grown2 - row.s2).max(0.0) * p2;
        kept + share * delta(1.0, lost1, lost2, r)
    }

    /// Chain `c`'s committed base radius: its boundary's E[r | kept],
    /// blended with the radius a phytomer of its base scale is
    /// established at, by its PA's secondary growth.
    pub(super) fn committed(&self, c: usize) -> f64 {
        let chain = &self.chains[c];
        let (row, s) = match chain.record {
            Some(r) => {
                let record = &self.records[r];
                (
                    self.rows.lateral(record.pa, record.birth),
                    self.base_scale(record),
                )
            }
            None => (self.rows.seed(), 1.0),
        };
        self.base_radius(row, s, self.bent.axes[chain.axes[0]].pa)
    }

    /// A committed base radius from its row and base scale `s`, for a
    /// chain whose first axis is of `pa`.
    pub(crate) fn base_radius(&self, row: &Row, s: f64, pa: usize) -> f64 {
        committed_radius(self.species, row, s, pa)
    }

    /// Chain `c`'s radii by axis, its axes `axes[first..]` renumbered
    /// from `first`, `global` their indices in the tree.
    pub(super) fn radii(
        &self,
        c: usize,
        axes: &[Axis],
        first: usize,
        global: &[usize],
    ) -> Vec<Vec<f64>> {
        let species = self.species;
        let age = self.bent.age;
        let blank =
            || -> Vec<Vec<f64>> { axes.iter().map(|a| vec![0.0; a.phytomers.len()]).collect() };
        let mut at_node = blank();
        let mut at_tip = vec![0.0; axes.len()];
        let mut at_base = vec![0.0; axes.len()];
        for &r in &self.chains[c].borne {
            let record = &self.records[r];
            let Origin::Lateral { parent, node, .. } = record.origin else {
                continue;
            };
            let j = global.binary_search(&parent).expect("borne on the chain");
            let e = species.states[axes[j].pa].form.exponent;
            if let Some(at) = at_node[j].get_mut(node) {
                *at += self.handed(record, e);
            }
        }
        let mut radii = blank();
        for j in (first..axes.len()).rev() {
            let carried = (&at_node[j][..], at_tip[j]);
            let section = own_profile(
                species,
                (axes, j, age),
                self.leaf_mean,
                carried,
                &mut radii[j],
            ) + at_base[j];
            hand_on(
                species,
                axes,
                (j, first),
                section,
                (&mut at_node, &mut at_tip, &mut at_base),
            );
        }
        fit(&mut radii[first..], self.committed(c));
        radii
    }
}

/// A committed base radius from its row and base scale `s`, for a chain
/// whose first axis is of `pa`: its boundary's E[r | kept], blended with
/// the radius a phytomer of its base scale is established at, by its PA's
/// secondary growth.
pub(crate) fn committed_radius(species: &Species, row: &Row, s: f64, pa: usize) -> f64 {
    let form = &species.states[pa].form;
    let (e, k) = (row.e, row.kept);
    let pipe = if k > 0.0 {
        delta(
            k,
            row.s1 * s.powf_fixed(e),
            row.s2 * s.powf_fixed(2.0 * e),
            1.0 / e,
        ) / k
    } else {
        0.0
    };
    let established = form.pipe * s * ripe(1.0, form.ripening);
    form.secondary * pipe + (1.0 - form.secondary) * established
}

/// Axis `j`'s pipe-model radii from its own pipes and what its nodes and
/// tip carry; its base section.
fn own_profile(
    species: &Species,
    (axes, j, age): (&[Axis], usize, u32),
    mean: Option<f64>,
    (at_node, at_tip): (&[f64], f64),
    radii: &mut [f64],
) -> f64 {
    let state = &species.states[axes[j].pa];
    let form = state.form;
    let (e, s) = (form.exponent, form.secondary);
    let mut section = at_tip;
    for (k, p) in axes[j].phytomers.iter().enumerate().rev() {
        let leaf = mean.map_or(1.0, |m| p.light.powf_fixed(state.leaf_girth) * m);
        let own = crate::girth::own(&form, p, age, p.scale, leaf);
        section += own.powf_fixed(e) + at_node[k];
        let established = form.pipe * p.scale * ripe(1.0, form.ripening);
        radii[k] = s * section.powf_fixed(1.0 / e) + (1.0 - s) * established;
    }
    section
}

/// Hands axis `j`'s base section to the chain axis carrying it, as
/// `girth::thicken` does; the chain's first axis hands nothing on.
fn hand_on(
    species: &Species,
    axes: &[Axis],
    (j, first): (usize, usize),
    section: f64,
    (at_node, at_tip, at_base): (&mut [Vec<f64>], &mut [f64], &mut [f64]),
) {
    if j == first {
        return;
    }
    let radius = section.powf_fixed(1.0 / species.states[axes[j].pa].form.exponent);
    let carried = |p: usize| radius.powf_fixed(species.states[axes[p].pa].form.exponent);
    match axes[j].origin {
        // A relay's wood enters where it joins (`girth::thicken`, fn-244).
        Origin::Relay { parent, .. } => {
            let section = carried(parent);
            match crate::girth::attached(axes, species, j) {
                Some((k, t)) => {
                    let share = f64::from(t);
                    at_node[parent][k] += section * share;
                    match k.checked_sub(1) {
                        Some(n) => at_node[parent][n] += section * (1.0 - share),
                        None => at_base[parent] += section * (1.0 - share),
                    }
                }
                None => at_tip[parent] += section,
            }
        }
        Origin::Continuation { parent } => at_tip[parent] += carried(parent),
        Origin::Seed | Origin::Lateral { .. } => {}
    }
}

/// Fits a chain's radii (its axes in order) to its committed base radius
/// `base`, as the module's doc gives.
fn fit(radii: &mut [Vec<f64>], base: f64) {
    let Some(&r0) = radii.iter().find_map(|r| r.first()) else {
        return;
    };
    if r0 <= 0.0 || base <= 0.0 {
        // Wood of no size meets a base of none.
        radii.iter_mut().flatten().for_each(|r| *r = 0.0);
        return;
    }
    let log = (base / r0).ln_fixed();
    let beta = if log.abs() > 1.0 {
        1.0 / log.abs()
    } else {
        1.0
    };
    for r in radii.iter_mut().flatten() {
        let q = (*r / r0).min(1.0);
        *r *= (log * q.powf_fixed(beta)).exp_fixed();
    }
}
