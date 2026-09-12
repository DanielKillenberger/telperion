use super::*;
use crate::{
    growth::{Age, GrowthTraits},
    presets::Family,
};

#[derive(Clone)]
pub(super) struct Timeline {
    pub age: Age,
    traits: GrowthTraits,
    mature_month: u64,
    envelope: Envelope,
    pipes: radius::Pipes,
}
impl Specimen {
    /// Build from a seedling through the exact monthly path used by `advance`.
    /// The legacy envelope builder remains available through `grow`.
    pub fn build(family: &Family) -> Result<Self> {
        Age::from_years(family.age)?;
        family.growth.validate()?;
        let mut specimen = Self::new(&family.skeleton, family.radii)?;
        specimen.config.max_nodes = specimen.config.max_nodes.min(NODE_CEILING);
        specimen.timeline = Some(Timeline {
            age: Age::default(),
            traits: family.growth,
            mature_month: family.growth.mature_month(),
            envelope: Envelope {
                height: 0.0,
                ..family.skeleton.envelope
            },
            pipes: radius::Pipes::default(),
        });
        if specimen.config.max_nodes == 0 {
            specimen.tree.diagnostics.node_capped = true;
        } else {
            specimen.advance(family.age)?;
        }
        Ok(specimen)
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
    /// On a cap, commit only complete months and discard the failed month's time.
    pub fn advance(&mut self, years: f64) -> Result<()> {
        let timeline = self
            .timeline
            .as_ref()
            .ok_or(Error::InvalidInput("specimen has no age"))?;
        let target = timeline.age.advanced(years)?;
        if self.tree.diagnostics.node_capped {
            return Err(Error::ResourceLimit("node ceiling reached"));
        }
        let end = target.month.min(timeline.mature_month);
        if self.tree.nodes.is_empty() {
            self.tree.nodes.push(Node::root());
            self.tree.crossover = 1;
            self.identify();
            self.tree.nodes[0].radius = self.radii.resolved()?.trunk_radius * 1e-6;
            self.tree.nodes[0].start_radius = self.tree.nodes[0].radius;
        }
        while self.timeline.as_ref().unwrap().age.month < end {
            let t = self.timeline.as_ref().unwrap();
            let month = t.age.month + 1;
            let budget = t.traits.budget(month);
            if budget > 0 {
                // One structural unit adds at most one node; a local unit can
                // flush its terminal plus all its lateral buds. Only a slice
                // that can hit the ceiling needs a rollback copy.
                let worst = budget.saturating_mul(self.params.twigs.laterals as usize + 2);
                let checkpoint = (worst
                    >= self.config.max_nodes.saturating_sub(self.tree.nodes.len()))
                .then(|| self.clone());
                let result = self.month(month, budget);
                if self.tree.diagnostics.node_capped || result.is_err() {
                    if let Some(previous) = checkpoint {
                        *self = previous;
                    }
                    result?;
                    self.tree.diagnostics.node_capped = true;
                    return Ok(());
                }
            }
            self.timeline.as_mut().unwrap().age = Age {
                month,
                remainder: 0,
            };
        }
        self.timeline.as_mut().unwrap().age = target;
        Ok(())
    }
    /// Raising a ceiling unblocks the rolled-back frontier; limits are resources,
    /// not growth traits, and do not change the monthly budget.
    pub fn set_node_ceiling(&mut self, limit: usize) -> Result<()> {
        if limit > NODE_CEILING || limit < self.tree.nodes.len() {
            return Err(Error::InvalidValue {
                field: "node ceiling",
                value: limit.to_string(),
            });
        }
        if limit > self.config.max_nodes {
            self.tree.diagnostics.node_capped = false;
        }
        self.config.max_nodes = limit;
        self.params.growth.max_nodes = Some(limit);
        Ok(())
    }
    fn month(&mut self, month: u64, budget: usize) -> Result<()> {
        let fraction = self.timeline.as_ref().unwrap().traits.fraction(month);
        let envelope = Envelope {
            height: self.params.envelope.height * fraction,
            ..self.params.envelope
        };
        let mut params = self.params.clone();
        params.envelope = envelope;
        let config = GrowthConfig {
            trunk_height: self.config.trunk_height * fraction,
            influence_radius: self.config.influence_radius * fraction,
            kill_distance: self.config.kill_distance * fraction,
            shell: Some(envelope),
            ..self.config
        };
        let bias = GrowthBias::new(envelope, params.seed, params.bias)?;
        let first = self.tree.crossover;
        let mut tail = self.tree.nodes.split_off(first);
        let structural_config = GrowthConfig {
            max_nodes: config.max_nodes.saturating_sub(tail.len()),
            ..config
        };
        let structural =
            ((budget as f64 * (0.5 + 0.5 * params.habit.apical_dominance)).round() as usize).max(1);
        let spent = self.scaffold.month(
            &mut self.tree,
            &self.params,
            &structural_config,
            &bias,
            structural,
            fraction,
        )?;
        let added = self.tree.nodes.len() - first;
        let map: Vec<_> = (0..first + tail.len())
            .map(|i| Some((if i < first { i } else { i + added }) as u32))
            .collect();
        for n in &mut tail {
            n.parent = n.parent.map(|p| map[p as usize].unwrap());
            n.branch = map[n.branch as usize].unwrap();
        }
        self.tree.nodes.extend(tail);
        self.local.remap(&map);
        self.identify();
        let timeline = self.timeline.as_mut().unwrap();
        timeline.envelope = envelope;
        timeline.pipes.update(
            &mut self.tree,
            envelope.height,
            self.params.envelope.height,
            self.radii,
        )?;
        let twigs = params.twigs.resolved()?;
        self.local.seed(&self.tree, &config, twigs, params.habit);
        self.local.identity_order(&self.tree);
        if !self.tree.diagnostics.node_capped {
            self.local.advance(
                &mut self.tree,
                local::Planner {
                    growing_envelope: true,
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
        self.identify();
        self.tree.validate_solved()
    }
}
