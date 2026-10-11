//! Host decision 3 under realized mechanics (fn-209 F1 task 6): a fine
//! root that can reach the ground's landing zone is laid on the CPU
//! reference executor, whichever executor grew it, so the ground's
//! terms are taken in f64 where the wood may rest on it. A work placement
//! only: the root is the one the executor grew, to the bit. Decided from
//! its bearer's frames as laid, before bending (phase 3) and under sag
//! (phase 4), by a conservative bound: the closed form's reach of a bud
//! of its PA and birth at its base scale with its fade taken as whole,
//! below the lower of its node and the point below it, against the
//! landing zone of twice its base radius and the longest internode.
use super::expansion::{node, Loaded, RootSeed};
use super::likely::once_laid;
use super::realized::{Root, Stub};
use super::spare::{grown, Spare};
use super::{each, Plan};
use crate::closed_form::boundary::Rows;
use crate::error::Result;
use crate::contract::presence::{real, times};
use crate::geometry::ground_zone;
use crate::grow::Rules;
use crate::sag::any;
use crate::species::Species;
use crate::structure::Structure;
use std::sync::Arc;

/// The roots laid on the CPU, by their place among the live roots, and
/// each live root's bearer's stub as laid before bending.
pub(crate) struct Landing<'a> {
    species: &'a Species,
    rules: &'a Rules<'a>,
    /// The closed form's reach bounds; none where the CPU executor grows
    /// every root already.
    rows: Option<&'a Rows>,
    plan: Plan,
    threads: usize,
    internode: f64,
    pub roots: Vec<(usize, Root)>,
    unbent: Arc<Vec<Stub>>,
    /// The roots grown ahead (task 9).
    pub spare: Option<&'a Spare>,
}

/// Whether `seed`'s root, at base scale `(base, drawn)` and base radius
/// `radius`, can reach the landing zone from a stub whose lowest point is
/// `low`, the longest internode `internode`.
pub(crate) fn reaches(
    rows: &Rows,
    internode: f64,
    seed: &RootSeed,
    (base, drawn): (f64, f64),
    radius: f64,
    low: f64,
) -> bool {
    let axis = &seed.handed.bud.axis;
    let s = base * drawn * real(times(seed.birth[0], seed.birth[1]));
    let zone = ground_zone(2.0 * radius, internode * s);
    low - rows.lateral(axis.pa, axis.birth).reach * s <= zone
}

impl<'a> Landing<'a> {
    pub fn new(
        species: &'a Species,
        (rules, rows): (&'a Rules<'a>, Option<&'a Rows>),
        (plan, threads): (Plan, usize),
    ) -> Self {
        let internode = species
            .states
            .iter()
            .map(|p| p.internode * (1.0 + p.form.internode_spread))
            .fold(0.0, f64::max);
        Self {
            species,
            rules,
            rows,
            plan,
            threads,
            internode,
            roots: Vec::new(),
            unbent: Arc::default(),
            spare: None,
        }
    }

    /// The root at live place `j` (`seed`'s) grown and scaled, or taken
    /// as grown ahead, and laid on the CPU from `unbent`.
    fn land(&self, j: usize, seed: &RootSeed, scales: (f64, f64), unbent: &Stub) -> Result<Root> {
        let (root, laid) = match self.spare.and_then(|s| s.take(j)) {
            Some(ahead) => ahead,
            None => (grown(self.species, self.rules, seed, scales), false),
        };
        let mut root = root?;
        if !laid {
            root.lay_on(self.species, unbent.axis())?;
        }
        Ok(root)
    }

    /// Each live root's bearer's stub as laid before bending, from `tree`
    /// so laid; the roots grown ahead lay on them from now, and are those
    /// likeliest to land on the tree so laid.
    pub fn frame(
        &mut self,
        tree: &Structure,
        (seeds, live, bearers): (&[RootSeed], &[usize], &[usize]),
        (scales, radii): (&[(f64, f64)], &[f64]),
    ) {
        let Some(rows) = self.rows else {
            return;
        };
        let stubs = live.iter().zip(bearers).map(|(&k, &b)| {
            let phase = crate::geometry::phase(&tree.axes, self.species, b);
            Stub::of((&tree.axes[b], phase), node(&seeds[k]))
        });
        self.unbent = Arc::new(stubs.collect());
        if let Some(spare) = self.spare {
            let at = (self.species, tree, rows);
            let roots = (seeds, live, bearers, scales);
            let laid = (radii, &self.unbent[..], self.internode);
            let order = once_laid(at, roots, laid, any(self.species));
            spare.laid(self.unbent.clone(), order);
        }
    }

    /// Phase 3: each live root that can reach the ground from its bearer
    /// as laid before bending (`frame`), laid on the CPU; its load
    /// replaces the executor's.
    pub fn lay(
        &mut self,
        (seeds, live): (&[RootSeed], &[usize]),
        (scales, radii): (&[(f64, f64)], &[f64]),
        loads: &mut [Loaded],
    ) -> Result<()> {
        let Some(rows) = self.rows else {
            return Ok(());
        };
        let ground: Vec<usize> = (0..live.len())
            .filter(|&j| {
                let low = self.unbent[j].low();
                reaches(
                    rows,
                    self.internode,
                    &seeds[live[j]],
                    scales[j],
                    radii[j],
                    low,
                )
            })
            .collect();
        for (j, root) in self.land_all(&ground, (seeds, live), scales)? {
            if any(self.species) {
                loads[j] = root.load.real();
            }
            self.roots.push((j, root));
        }
        Ok(())
    }

    /// The live roots at `ground` landed, on the plan's cores.
    fn land_all(
        &self,
        ground: &[usize],
        (seeds, live): (&[RootSeed], &[usize]),
        scales: &[(f64, f64)],
    ) -> Result<Vec<(usize, Root)>> {
        let mut slots: Vec<usize> = ground.to_vec();
        let landed = each(&mut slots, self.plan, self.threads, &|_, j| {
            let j = *j;
            self.land(j, &seeds[live[j]], scales[j], &self.unbent[j])
                .map(|root| (j, root))
        });
        landed.into_iter().collect()
    }

    /// Phase 4: each live root that can reach the ground from either lay
    /// of its bearer laid on the CPU, then every root it lays bent.
    pub fn bend(
        &mut self,
        tree: &Structure,
        (seeds, live, bearers): (&[RootSeed], &[usize], &[usize]),
        (scales, radii): (&[(f64, f64)], &[f64]),
    ) -> Result<()> {
        let Some(rows) = self.rows else {
            return Ok(());
        };
        let bent: Vec<Stub> = live
            .iter()
            .zip(bearers)
            .map(|(&k, &b)| {
                let phase = crate::geometry::phase(&tree.axes, self.species, b);
                Stub::of((&tree.axes[b], phase), node(&seeds[k]))
            })
            .collect();
        let mut landed = vec![false; live.len()];
        for (j, _) in &self.roots {
            landed[*j] = true;
        }
        let ground: Vec<usize> = (0..live.len())
            .filter(|&j| {
                let low = self.unbent[j].low().min(bent[j].low());
                let seed = &seeds[live[j]];
                !landed[j] && reaches(rows, self.internode, seed, scales[j], radii[j], low)
            })
            .collect();
        let more = self.land_all(&ground, (seeds, live), scales)?;
        self.roots.extend(more);
        let species = self.species;
        let bend = each(&mut self.roots, self.plan, self.threads, &|_, (j, root)| {
            root.bend_on(species, bent[*j].axis())
        });
        bend.into_iter().collect()
    }
}
