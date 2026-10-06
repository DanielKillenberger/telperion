//! The rough layout grown with the tree (fn-197, host decisions
//! 2026-10-05). At the end of each cycle every growth unit grown in it is
//! laid on from where its axis's walk stood (`geometry::Layer`), each new
//! axis framed from its parent as the final lay frames it, every phytomer
//! at the scale its draws give it so far. Sag is left out, erection and
//! the base's straightening are taken at the axis's age when the unit
//! grew, and the fade of a straightening over the axis's reach at its
//! reach so far: what the final lay knows only once the tree has grown.
//! The leaves grown in the cycle then shade the sky for the buds that grow
//! in the next. What it writes into the phytomers and axes (their tip,
//! frame, scale and rank) is all written again once the tree has grown.
use super::couple::Split;
use super::subtree::tally;
use super::{Grower, Pass, Stage};
use crate::allocation::{sizes, vigours_from, Link, TIP};
use crate::cores::fill;
use crate::geometry::Layer;
use crate::light::{Field, Light};
use crate::structure::{Origin, Vec3};

/// The axes in a piece of the gathering of rows.
const ROWS: usize = 1 << 15;

/// Each laid axis's link to its bearer and its PA, by index.
pub(super) struct Rows {
    pub(super) links: Vec<Link>,
    pub(super) pa: Vec<usize>,
}

/// The rough layout and the light it casts.
pub(super) struct Sketch {
    light: Light,
    pub(super) pencils: Vec<Pencil>,
}

/// Where an axis's rough layout stands.
#[derive(Debug, Clone)]
pub(super) struct Pencil {
    pub(super) layer: Layer,
    /// Its scale at its base, and its draws' presence over the growth
    /// units laid.
    pub(super) base_scale: f64,
    pub(super) running: f64,
    /// Its nodes counted by their presence, the units and phytomers laid.
    pub(super) rank: f64,
    pub(super) units: usize,
    pub(super) laid: usize,
    pub(super) trunk: bool,
    /// The light at its tip, from the last cycle's leaves, and its
    /// growth unit's size among its siblings this cycle.
    pub(super) light: f64,
    pub(super) size: f64,
    /// Its own present wood in metres, its remembered carbon balance and
    /// what its subtrees' survival of shedding makes of its unit this
    /// cycle (`balance.rs`).
    pub(super) wood: f64,
    pub(super) memory: f64,
    pub(super) kept: f64,
}

impl Sketch {
    pub fn new(light: Light) -> Self {
        Self {
            light,
            pencils: Vec::new(),
        }
    }
}

impl Grower<'_> {
    /// Sweeps the cycle's leaves from the sky and reads the light at every
    /// living apex's tip; in the coupling pass, every fine root's stand-in
    /// leaves cast too and each root reads its light at its centroid; in a
    /// fine subtree, every bud reads its root's record.
    pub(super) fn shade(&mut self, cycle: u32, report: &mut dyn FnMut(Stage)) {
        let mut sketch = self.sketch.take().expect("sketching");
        if let Some(reads) = self.fine.as_ref() {
            let light = reads.records.light(cycle + 1);
            for apex in &self.live {
                sketch.pencils[apex.axis].light = light;
            }
            self.sketch = Some(sketch);
            return;
        }
        let mut tips: Vec<Vec3> = self.live.iter().map(|a| self.tip(a.axis)).collect();
        let fine = self.fine_leaves(cycle);
        let own = self.leaves.len();
        for (_, at, leaves) in &fine {
            self.leaves.extend(leaves);
            tips.push(*at);
        }
        let field = match self
            .oracle
            .as_mut()
            .and_then(|o| o.grouping.filter(|g| g.leaves))
        {
            Some(g) => {
                let oracle = self.oracle.as_mut().expect("an oracle");
                let axes = std::mem::take(&mut oracle.leaf_axes);
                let owner: Vec<Option<usize>> =
                    axes.iter().map(|&i| oracle.root(&self.axes, i)).collect();
                let leaves = super::partition::regroup(&self.leaves, &owner, &g);
                Field::on(&sketch.light, &leaves, &tips, self.threads)
            }
            None => Field::on(&sketch.light, &self.leaves, &tips, self.threads),
        };
        report(Stage::Pass(Pass::ShadeField));
        let mut read = vec![0.0; tips.len()];
        crate::cores::fill(&mut read, 1 << 10, self.threads, &|k| field.at(tips[k]));
        let n = self.live.len();
        if let Some(oracle) = self.oracle.as_mut() {
            oracle.reread(&self.axes, &self.live, &tips, &mut read[..n]);
        }
        for (apex, &light) in self.live.iter().zip(&read) {
            sketch.pencils[apex.axis].light = light;
        }
        self.sketch = Some(sketch);
        if self.couple.is_some() {
            let lit: Vec<(usize, f64)> = fine
                .iter()
                .map(|f| f.0)
                .zip(read[n..].iter().copied())
                .collect();
            self.fine_lit(cycle, &lit);
            if cycle + 1 == self.rules.age {
                self.keep_field(own, &tips, &read);
            }
        }
        self.leaves.truncate(own);
        self.mark(cycle, 3);
        report(Stage::Pass(Pass::ShadeRead));
    }

    /// Each laid axis's link and PA, gathered on every core, so the
    /// passes over the whole tree each cycle read them packed.
    pub(super) fn rows(&self) -> Option<Rows> {
        let n = self.sketch.as_ref()?.pencils.len();
        let axes = &self.axes[..n];
        let mut links = vec![
            Link {
                parent: None,
                node: 0
            };
            n
        ];
        fill(&mut links, ROWS, self.threads, &|i| match axes[i].origin {
            Origin::Seed => Link {
                parent: None,
                node: 0,
            },
            Origin::Lateral { parent, node, .. } => Link {
                parent: Some(parent),
                node,
            },
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => Link {
                parent: Some(parent),
                node: TIP,
            },
        });
        let mut pa = vec![0; n];
        fill(&mut pa, ROWS, self.threads, &|i| axes[i].pa);
        Some(Rows { links, pa })
    }

    /// Gives each living apex its unit's size this cycle from the vigour
    /// the tree's light allots it per presence, against the mean of the
    /// growing buds of its PA (`allocation.rs`): before any apex grows.
    /// In the coupling pass the fine roots take part as their subtrees
    /// would; in a fine subtree the root's recorded vigour is split and
    /// each size read against the pass's recorded means.
    pub(super) fn allot(&mut self, rows: Option<&Rows>, cycle: u32) {
        let species = self.species;
        let (Some(sketch), Some(rows)) = (self.sketch.as_ref(), rows) else {
            return;
        };
        if species.states.iter().all(|s| s.shade_size == 0.0) {
            return;
        }
        let n = sketch.pencils.len();
        let mut lambda = vec![0.0; n];
        fill(&mut lambda, ROWS, self.threads, &|i| {
            species.states[rows.pa[i]].apical_control
        });
        let (mut lit, mut presence, mut psi) = (vec![0.0; n], vec![0.0; n], vec![0.0; n]);
        for apex in self.live.iter().filter(|a| a.axis < n) {
            let p = &sketch.pencils[apex.axis];
            // Its presence this cycle includes its subtrees' survival of
            // shedding (`balance.rs`), so a bud about to be shed weighs
            // nothing as it goes.
            presence[apex.axis] = p.base_scale * p.running * p.kept;
            lit[apex.axis] = presence[apex.axis] * p.light;
            psi[apex.axis] = species.states[rows.pa[apex.axis]].shade_size;
        }
        let fine = self.fine_presence(sketch);
        for &(i, p, l) in &fine {
            (presence[i], lit[i]) = (p, l);
        }
        let base = self.fine.as_ref().map(|r| {
            let v = r.records.vigour(cycle);
            match (r.split, r.records.q(cycle)) {
                (Split::Recorded, _) => v,
                (Split::PerLight, q) if q > 0.0 => v / q * lit.iter().sum::<f64>(),
                (Split::PerLight, _) => 0.0,
            }
        });
        let got = vigours_from(&rows.links, &lit, &lambda, base);
        for &(i, ..) in &fine {
            presence[i] = 0.0;
        }
        let sized = match (&self.couple, &self.fine) {
            (None, None) => match self
                .oracle
                .as_ref()
                .and_then(|o| o.grouping.filter(|g| g.sizes))
            {
                Some(g) => {
                    let sums = self.grouped_sums(&got, (&presence, &psi, &rows.pa), &g);
                    let factor = |p: usize| (sums[p][1] > 0.0).then_some(sums[p]);
                    read_sizes(&got, (&presence, &psi, &rows.pa), &factor)
                }
                None => sizes(&got, &presence, &psi, &rows.pa),
            },
            (_, Some(reads)) => {
                let means = reads.means.get(cycle as usize);
                let mean = |g: usize| means.map_or(1.0, |m| m[g]);
                read_sizes(&got, (&presence, &psi, &rows.pa), &|g| Some([1.0, mean(g)]))
            }
            (Some(_), None) => {
                let sums = self.fine_means(cycle, &got, (&presence, &psi, &rows.pa));
                let factor = |g: usize| (sums[g][1] > 0.0).then_some(sums[g]);
                read_sizes(&got, (&presence, &psi, &rows.pa), &factor)
            }
        };
        let sketch = self.sketch.as_mut().expect("sketching");
        let growth = match (self.couple.as_mut(), self.fine.as_mut()) {
            (Some(pass), _) => Some(&mut pass.growth),
            (_, Some(reads)) => Some(&mut reads.growth),
            _ => None,
        };
        let mut growth = growth;
        for apex in self.live.iter().filter(|a| a.axis < n) {
            let i = apex.axis;
            sketch.pencils[i].size = sized[i];
            if let (Some(g), true) = (growth.as_deref_mut(), presence[i] > 0.0) {
                tally(g, cycle, rows.pa[i], presence[i], sized[i]);
            }
        }
        if let Some(oracle) = self.oracle.as_mut() {
            let live = self.live.iter().filter(|a| a.axis < n);
            oracle.tally(
                cycle,
                live.map(|a| (rows.pa[a.axis], presence[a.axis], sized[a.axis])),
            );
        }
    }

    fn tip(&self, i: usize) -> Vec3 {
        let axis = &self.axes[i];
        axis.phytomers.last().map_or(axis.base, |p| p.tip)
    }
}

/// Each growing bud's size, (v/p)^ψ against its PA's mean as `[total,
/// weighted]` (none, and every bud of the PA is whole), in the
/// arithmetic `allocation::sizes` takes it.
fn read_sizes(
    got: &[f64],
    (presence, psi, pa): (&[f64], &[f64], &[usize]),
    factor: &dyn Fn(usize) -> Option<[f64; 2]>,
) -> Vec<f64> {
    (0..got.len())
        .map(|i| match (presence[i] > 0.0, factor(pa[i])) {
            (true, Some([t, w])) => (got[i] / presence[i]).powf(psi[i]) * t / w,
            _ => 1.0,
        })
        .collect()
}
