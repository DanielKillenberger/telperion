//! fn-255 R1: one cycle for every species (DESIGN.md section 1). The
//! whole-tree grower runs every phase every cycle for every species and
//! every light: the rough layout and the light read, the balance and the
//! allotment, each zero setting a neutral term in them, never another way
//! of building. The light evaluator is the one the build is created with:
//! today's lattice, the oracle, or fn-255's foliage volumes (R3's
//! candidate). Allocation is `allocation::sizes` throughout (R2).
//!
//! An experiment behind its own entry point until the owner approves R3's
//! candidate: `grow` never builds one.
mod probe;

pub use probe::{Measure, Probe};

use super::{lineage_order, start, Grower, Options, Request};
use crate::error::Result;
use crate::light::volumes::{Candidate, Volumes};
use crate::light::Field;
use crate::species::Species;
use crate::structure::{Structure, Vec3};
use super::sketch::Sketch;

/// How a cycle's buds read the light of the last cycle's leaves.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Model {
    /// Today's lattice: deposited, swept and sampled.
    #[default]
    Lattice,
    /// The leaves as a hierarchy of foliage volumes.
    Volumes(Candidate),
}

/// A tree grown through the one cycle: the tree, the phytomers grown for
/// it (the shed ones included), the seconds the build and its light took,
/// and the probe's measures, where candidates were read beside it.
pub struct Cycled {
    pub structure: Structure,
    pub grown: u64,
    pub seconds: f64,
    pub light_seconds: f64,
    pub probe: Option<Probe>,
}

/// Grows `species` through the one cycle, its buds reading `model`'s
/// light; each of `beside` read on the same leaves and buds every cycle
/// and measured against the lattice, which then drives growth.
pub fn grow_cycle(species: &Species, request: Request, model: Model, beside: &[Candidate]) -> Result<Cycled> {
    let started = crate::clock::seconds();
    let (mut g, keeps) = start(species, request, true, Options::default())?;
    g.one = true;
    g.model = model;
    g.sketch.get_or_insert_with(|| Sketch::new(request.light));
    // Every lateral is grown: while it lives its leaves shade.
    g.rules.lasting = vec![None; species.states.len()].into();
    if !beside.is_empty() {
        g.model = Model::Lattice;
        g.probe = Some(Box::new(Probe::new(beside)));
    }
    let mut advanced = Vec::new();
    for cycle in 1..=request.age {
        g.year(cycle, cycle < request.age, &mut advanced, &mut |_| {})?;
    }
    let (grown, light_seconds, probe) = (g.rules.grown, g.light_seconds, g.probe.take());
    let settled = g.settle((request, Options::default()), (true, keeps), &mut |_| {})?;
    Ok(Cycled {
        structure: lineage_order(settled.structure),
        grown,
        seconds: crate::clock::seconds() - started,
        light_seconds,
        probe: probe.map(|p| *p),
    })
}

/// Each cycle's laid axes after its light is read: lineage, the subtree's
/// light the next balance sums (Q), its remembered balance, and whether
/// its apex lives. fn-254's amplification measures, re-taken on the one
/// cycle (DESIGN.md section 4 (c)).
pub type Traced = Vec<Vec<(u64, f64, f64, bool)>>;

/// `grow_cycle`'s cycles, traced, without the final settle.
pub fn trace_cycle(species: &Species, request: Request, model: Model) -> Result<Traced> {
    let (mut g, _) = start(species, request, true, Options::default())?;
    (g.one, g.model) = (true, model);
    g.sketch.get_or_insert_with(|| Sketch::new(request.light));
    g.rules.lasting = vec![None; species.states.len()].into();
    let (mut advanced, mut out) = (Vec::new(), Vec::new());
    for cycle in 1..=request.age {
        g.year(cycle, true, &mut advanced, &mut |_| {})?;
        let pencils = &g.sketch.as_ref().expect("sketching").pencils;
        let mut q = vec![0.0; pencils.len()];
        let mut living = vec![false; pencils.len()];
        for a in g.live.iter().filter(|a| a.axis < pencils.len()) {
            let p = &pencils[a.axis];
            (q[a.axis], living[a.axis]) = (p.base_scale * p.running * p.light, true);
        }
        for i in (1..pencils.len()).rev() {
            if let Some(parent) = g.axes[i].origin.parent() {
                q[parent] += q[i];
            }
        }
        out.push((0..pencils.len()).map(|i| (g.axes[i].lineage, q[i], pencils[i].memory, living[i])).collect());
    }
    Ok(out)
}

impl Grower<'_> {
    /// The light at each of `tips` from the cycle's leaves, by the
    /// grower's model; with a probe, every candidate read beside the
    /// lattice and measured against it.
    pub(super) fn read_light(&mut self, tips: &[Vec3]) -> Vec<f64> {
        let light = self.sketch.as_ref().expect("sketching").light;
        let started = crate::clock::seconds();
        let read = match self.model {
            Model::Lattice => {
                let field = Field::on(&light, &self.leaves, tips, self.threads);
                let mut read = vec![0.0; tips.len()];
                crate::cores::fill(&mut read, 1 << 10, self.threads, &|k| field.at(tips[k]));
                read
            }
            Model::Volumes(candidate) => {
                let volumes = self.volumes(candidate);
                let directions = lit(&light);
                let mut read = vec![0.0; tips.len()];
                let k = light.extinction;
                crate::cores::fill(&mut read, 1 << 8, self.threads, &|i| volumes.read(tips[i], &directions, k).0);
                read
            }
        };
        self.light_seconds += crate::clock::seconds() - started;
        if let Some(mut probe) = self.probe.take() {
            probe.measure(self, tips, &read, started);
            self.probe = Some(probe);
        }
        read
    }

    /// The cycle's leaves as volumes: each leaf its axis's, its own
    /// variance from its PA's leaf size.
    pub(super) fn volumes(&self, candidate: Candidate) -> Volumes {
        let states = &self.species.states;
        let leaves: Vec<_> = (self.leaves.iter().zip(&self.owners))
            .map(|(&(at, area), &i)| {
                let s2 = candidate.leaf * candidate.leaf * states[self.axes[i].pa].leaf_area / 7.0;
                (at, area, i, s2)
            })
            .collect();
        let parents: Vec<Option<usize>> = self.axes.iter().map(|a| a.origin.parent()).collect();
        Volumes::build(candidate, &leaves, &parents)
    }
}

/// The sky's directions that carry light.
pub(super) fn lit(light: &crate::light::Light) -> Vec<(Vec3, f64)> {
    light.directions().into_iter().filter(|d| d.1 > 0.0).collect()
}

#[cfg(test)]
mod tests;
