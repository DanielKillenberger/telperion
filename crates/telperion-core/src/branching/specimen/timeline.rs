use super::*;
use crate::{
    growth::{Age, GrowthTraits},
    presets::Family,
};

#[derive(Clone)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub(super) struct Timeline {
    pub age: Age,
    pub years: Vec<history::Year>,
    pub(super) unpacked: bool,
    pub(super) traits: GrowthTraits,
    mature_slice: u64,
    pub(super) envelope: Envelope,
    pub(super) pipes: radius::Pipes,
    pub(super) widths: widths::Widths,
    #[cfg_attr(feature = "json", serde(skip))]
    pub(super) crown: crown::Crown,
    pub(super) foliage: crate::foliage::timeline::Foliage,
}
impl Specimen {
    /// Build from a seedling through the exact annual path used by `advance`.
    /// The legacy envelope builder remains available through `grow`.
    pub fn build(family: &Family) -> Result<Self> {
        Self::build_with_history_cap(family, retention::DEFAULT_HISTORY_CAP)
    }
    pub fn build_with_history_cap(family: &Family, cap: f64) -> Result<Self> {
        let cap = retention::checked_cap(cap)?;
        Age::from_years(family.age)?;
        family.growth.validate()?;
        let mut specimen = Self::new(&family.skeleton, family.radii)?;
        specimen.retention.cap = cap;
        specimen.config.max_nodes = specimen.config.max_nodes.min(NODE_CEILING);
        specimen.timeline = Some(Timeline {
            age: Age::default(),
            years: Vec::new(),
            unpacked: false,
            traits: family.growth,
            mature_slice: family.growth.mature_slice(),
            envelope: Envelope {
                height: 0.0,
                ..family.skeleton.envelope
            },
            pipes: radius::Pipes::default(),
            widths: widths::Widths::default(),
            crown: crown::Crown::default(),
            foliage: crate::foliage::timeline::Foliage::new(family)?,
        });
        if specimen.config.max_nodes == 0 {
            specimen.tree.diagnostics.node_capped = true;
        } else {
            specimen.advance_growth(family.age)?;
        }
        specimen.compact_history();
        Ok(specimen)
    }
    /// Leaf stations in identity order, before optional canopy shell culling.
    pub fn placements(&self) -> Result<Vec<crate::foliage::Placement>> {
        let timeline = self
            .timeline
            .as_ref()
            .ok_or(Error::InvalidInput("specimen has no age"))?;
        timeline
            .foliage
            .read(self.tree(), self.envelope(), timeline.age)
    }
    pub fn age(&self) -> f64 {
        self.timeline.as_ref().map_or(0.0, |t| t.age.years())
    }
    /// Current growth envelope; the family keeps its authored asymptote.
    pub fn envelope(&self) -> Envelope {
        self.timeline
            .as_ref()
            .map_or(self.params.envelope, |t| t.envelope)
    }
    /// Negative/non-finite requests are refused before mutating any state.
    /// On a cap, commit only complete slices and discard the failed slice's time.
    pub fn advance(&mut self, years: f64) -> Result<ChangeRecord> {
        let timeline = self
            .timeline
            .as_ref()
            .ok_or(Error::InvalidInput("specimen has no age"))?;
        let target = timeline.age.advanced(years)?;
        if self.tree.diagnostics.node_capped {
            return Err(Error::ResourceLimit("node ceiling reached"));
        }
        if !self.tree.nodes.is_empty()
            && target.slice.min(timeline.mature_slice) <= timeline.age.slice
        {
            // Unchanged wood can only fill newly reached cohort offsets.
            // Saturated cohorts need no contact surface or run-buffer copy.
            let born_placements = self.interval(timeline.age, target)?.born_placements;
            let from = timeline.age;
            self.timeline.as_mut().unwrap().age = target;
            let record = ChangeRecord {
                born_placements,
                ..ChangeRecord::default()
            };
            self.update_shared(from, &record);
            self.compact_history();
            return Ok(record);
        }
        let from = timeline.age;
        self.advance_growth(years)?;
        #[cfg(test)]
        let clock = std::time::Instant::now();
        let changes = self.interval(from, self.timeline.as_ref().unwrap().age)?;
        #[cfg(test)]
        {
            self.cost.changes = clock.elapsed();
        }
        self.update_shared(from, &changes);
        self.compact_history();
        Ok(changes)
    }
    pub(super) fn advance_growth(&mut self, years: f64) -> Result<()> {
        let timeline = self
            .timeline
            .as_ref()
            .ok_or(Error::InvalidInput("specimen has no age"))?;
        let target = timeline.age.advanced(years)?;
        if self.tree.diagnostics.node_capped {
            return Err(Error::ResourceLimit("node ceiling reached"));
        }
        let end = target.slice.min(timeline.mature_slice);
        let cached_read = self.read.take();
        self.read_active = cached_read.is_some();
        if self.tree.nodes.is_empty() {
            self.tree.nodes.push(Node::root());
            self.tree.crossover = 1;
            self.identify();
            self.tree.nodes[0].radius = self.radii.resolved()?.trunk_radius * 1e-6;
            self.tree.nodes[0].start_radius = self.tree.nodes[0].radius;
            self.seed_widths();
        }
        while self.timeline.as_ref().unwrap().age.slice < end {
            let t = self.timeline.as_ref().unwrap();
            let slice = t.age.slice + 1;
            let budget = t.traits.budget(slice);
            if budget > 0 {
                // One structural unit adds at most one node; a local unit can
                // flush its terminal plus all its lateral buds. Only a slice
                // that can hit the ceiling needs a rollback copy.
                let worst = budget.saturating_mul(self.params.twigs.laterals as usize + 2);
                let checkpoint = (worst
                    >= self.config.max_nodes.saturating_sub(self.tree.nodes.len()))
                .then(|| self.clone());
                let result = self.slice(slice, budget);
                if self.tree.diagnostics.node_capped || result.is_err() {
                    if let Some(previous) = checkpoint {
                        *self = previous;
                    }
                    self.finish_widths()?;
                    self.read_active = false;
                    self.read_updates.clear();
                    result?;
                    self.read.take();
                    self.tree.diagnostics.node_capped = true;
                    return Ok(());
                }
            }
            self.timeline.as_mut().unwrap().age = Age {
                slice,
                remainder: 0,
            };
        }
        self.finish_widths()?;
        self.timeline.as_mut().unwrap().age = target;
        self.update_packed(cached_read);
        Ok(())
    }
    /// Raising a ceiling unblocks the rolled-back frontier; limits are resources,
    /// not growth traits, and do not change the annual budget.
    pub fn set_node_ceiling(&mut self, limit: usize) -> Result<()> {
        if limit > NODE_CEILING || limit < self.tree.nodes.len() + self.retention.dead.len() {
            return Err(Error::InvalidValue {
                field: "node ceiling",
                value: limit.to_string(),
            });
        }
        let retained_limit = limit - self.retention.dead.len();
        self.read.take();
        if retained_limit > self.config.max_nodes {
            self.tree.diagnostics.node_capped = false;
        }
        self.config.max_nodes = retained_limit;
        self.params.growth.max_nodes = Some(limit);
        Ok(())
    }
    pub(super) fn slice(&mut self, slice: u64, budget: usize) -> Result<()> {
        self.read.take();
        #[cfg(test)]
        {
            self.cost = super::measurement::Cost::default();
        }
        #[cfg(test)]
        let mut clock = std::time::Instant::now();
        let shed = self.environment(slice);
        #[cfg(test)]
        self.cost.stamp(0, &mut clock);
        let fraction = self.timeline.as_ref().unwrap().traits.fraction(slice);
        let envelope = Envelope {
            height: self.params.envelope.height * fraction,
            ..self.params.envelope
        };
        let mut params = self.params.clone();
        params.envelope = envelope;
        params.habit.apical_dominance /=
            1.0 + self.timeline.as_ref().unwrap().traits.apical_control_loss * slice as f64;
        // Lost terminal control releases a larger share to laterals, expressed
        // through the existing lateral allocation trait rather than a species rule.
        let released = self.params.habit.apical_dominance - params.habit.apical_dominance;
        params.habit.lateral_length_ratio += (1.0 - params.habit.lateral_length_ratio) * released;
        let config = GrowthConfig {
            trunk_height: self.config.trunk_height * fraction,
            influence_radius: self.config.influence_radius * fraction,
            kill_distance: self.config.kill_distance * fraction,
            shell: Some(envelope),
            ..self.config
        };
        let bias = GrowthBias::new(envelope, params.seed, params.bias)?;
        let first = self.tree.crossover;
        let previous_len = self.tree.nodes.len();
        let structural =
            ((budget as f64 * (0.5 + 0.5 * params.habit.apical_dominance)).round() as usize).max(1);
        let scaffold_params = SkeletonParams {
            envelope: self.params.envelope,
            ..params.clone()
        };
        self.scaffold.year = slice;
        self.scaffold.crown_base_retention =
            self.timeline.as_ref().unwrap().traits.crown_base_retention;
        let spent = self.scaffold.slice(
            &mut self.tree,
            &scaffold_params,
            &config,
            &bias,
            structural,
            fraction,
        )?;
        #[cfg(test)]
        self.cost.stamp(1, &mut clock);
        self.insert_structural(first, previous_len);
        #[cfg(test)]
        self.cost.stamp(2, &mut clock);
        let timeline = self.timeline.as_mut().unwrap();
        timeline.envelope = envelope;
        let radial_height = envelope.height
            * timeline
                .traits
                .radius_fraction(slice, timeline.mature_slice);
        timeline.pipes.record(
            &self.tree,
            radial_height,
            self.params.envelope.height,
            self.radii,
        )?;
        timeline.widths.invalidate();
        #[cfg(test)]
        self.cost.stamp(3, &mut clock);
        let twigs = params.twigs.resolved()?;
        let timeline = self.timeline.as_ref().unwrap();
        let widths = |tree: &Tree, i| timeline.widths.sample(tree, &timeline.pipes, i);
        self.local.reserve_tips(self.scaffold.growing_tips());
        self.local
            .seed(&self.tree, &config, twigs, params.habit, Some(&widths));
        #[cfg(test)]
        self.cost.stamp(4, &mut clock);
        let local_first = self.tree.nodes.len();
        if !self.tree.diagnostics.node_capped {
            self.local.advance(
                &mut self.tree,
                local::Planner {
                    clock: Some(local::waiting::Clock {
                        slice,
                        traits: timeline.traits,
                        envelope: self.params.envelope,
                    }),
                    widths: Some(&widths),
                    growing_envelope: true,
                    planning: Some(Envelope {
                        crown_base: self.params.envelope.crown_base
                            * (fraction + (1.0 - fraction) * timeline.traits.crown_base_retention),
                        ..self.params.envelope
                    }),
                    config: &config,
                    bias: Some(&bias),
                    twigs,
                    crookedness: params.habit.crookedness,
                    seed: params.seed,
                },
                params.habit,
                budget.saturating_sub(spent),
            )?;
        }
        #[cfg(test)]
        self.cost.stamp(5, &mut clock);
        self.identify_range(local_first..self.tree.nodes.len());
        self.sample_frontier(slice);
        #[cfg(test)]
        self.cost.stamp(6, &mut clock);
        self.stamp_deaths(&shed, slice);
        #[cfg(test)]
        self.cost.stamp(7, &mut clock);
        let timeline = self.timeline.as_mut().unwrap();
        timeline.pipes.record(
            &self.tree,
            radial_height,
            self.params.envelope.height,
            self.radii,
        )?;
        timeline.widths.invalidate();
        self.record_widths(slice, previous_len);
        self.timeline.as_mut().unwrap().years.push(history::Year {
            year: slice,
            envelope,
            diagnostics: self.tree.diagnostics,
        });
        #[cfg(test)]
        self.cost.stamp(8, &mut clock);
        Ok(())
    }
}
