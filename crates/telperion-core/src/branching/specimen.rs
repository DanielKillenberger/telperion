//! Persistent generator frontiers and stable identities beside compact storage.
use super::*;

#[derive(Clone)]
pub struct Specimen {
    pub(super) tree: Tree,
    pub(super) shed: usize,
    params: SkeletonParams,
    radii: RadiusParams,
    config: GrowthConfig,
    bias: GrowthBias,
    scaffold: scaffold::Frontier,
    local: local::Frontier,
    next_identity: u64,
    identities: Vec<u64>,
}
impl Specimen {
    pub fn new(params: &SkeletonParams, radii: RadiusParams) -> Result<Self> {
        params.envelope.validate()?;
        params.bias.validate()?;
        params.habit.validate()?;
        radii.resolved()?;
        let twigs = params.twigs.resolved()?;
        if !params.step.is_finite() || params.step <= 0.0 {
            return Err(Error::InvalidInput("growth step"));
        }
        if params.habit.attractor_weight > 0.0 && params.attractors == 0 {
            return Err(Error::InvalidInput("attractor weight and attractor count"));
        }
        let inner = inner_envelope(params.envelope, twigs.reach);
        let points = if params.habit.attractor_weight > 0.0 {
            inner.sample(params.attractors, &mut Rng::new(params.seed))?
        } else {
            Vec::new()
        };
        let config = params.resolved_growth(points.len())?;
        let bias = GrowthBias::new(params.envelope, params.seed, params.bias)?;

        let scaffold = scaffold::Frontier::new(params, &config, points);
        Ok(Self {
            tree: Tree::default(),
            shed: 0,
            params: params.clone(),
            radii,
            config,
            bias,
            scaffold,
            local: local::Frontier::default(),
            next_identity: 0,
            identities: Vec::new(),
        })
    }
    pub fn tree(&self) -> &Tree {
        &self.tree
    }
    pub fn identities(&self) -> &[u64] {
        &self.identities
    }
    fn finished(&self) -> bool {
        self.scaffold.finished() && self.local.finished()
    }
    fn identify(&mut self) {
        for node in &mut self.tree.nodes {
            if node.identity == u64::MAX {
                node.identity = self.next_identity;
                self.next_identity += 1;
            }
        }
        self.identities = self.tree.nodes.iter().map(|n| n.identity).collect();
    }
    fn remap_after_shedding(&mut self) {
        let live: std::collections::HashMap<_, _> = self
            .tree
            .nodes
            .iter()
            .enumerate()
            .map(|(i, n)| (n.identity, i as u32))
            .collect();
        let map: Vec<_> = self
            .identities
            .iter()
            .map(|id| live.get(id).copied())
            .collect();
        self.local.remap(&map);
        self.identify();
    }
    /// Structural insertion shifts local storage, but never its birth identities.
    fn step(&mut self, structural: usize, local: usize) -> Result<()> {
        let first = self.tree.crossover;
        let mut tail = self.tree.nodes.split_off(first);
        let config = GrowthConfig {
            max_nodes: self.config.max_nodes.saturating_sub(tail.len()),
            ..self.config
        };
        let result = self.scaffold.advance(
            &mut self.tree,
            &self.params,
            &config,
            &self.bias,
            structural,
        );
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
        result?;
        radius::solve(&mut self.tree, self.params.envelope, self.radii)?;
        self.identify();
        self.local.seed(
            &self.tree,
            &self.config,
            self.params.twigs.resolved()?,
            self.params.habit,
        );
        if !self.tree.diagnostics.node_capped {
            self.local.advance(
                &mut self.tree,
                local::Planner {
                    config: &self.config,
                    bias: Some(&self.bias),
                    twigs: self.params.twigs.resolved()?,
                    crookedness: self.params.habit.crookedness,
                    seed: self.params.seed,
                },
                self.params.habit,
                local,
            )?;
        }
        self.identify();
        Ok(())
    }
    /// Drain the same retained builders used by incremental growth.
    pub fn grow(params: &SkeletonParams, radii: RadiusParams) -> Result<Self> {
        let mut s = Self::new(params, radii)?;
        s.step(usize::MAX, 0)?;
        let twigs = params.twigs.resolved()?;
        s.config.max_nodes = s
            .config
            .max_nodes
            .min(NODE_CEILING)
            .min(s.tree.nodes.len() + headroom(&s.tree, &s.config, twigs));
        s.step(0, usize::MAX)?;
        debug_assert!(s.finished() || !s.tree.diagnostics.complete());
        s.shed = finish(&mut s.tree, params, radii)?;
        s.remap_after_shedding();
        Ok(s)
    }
}

#[cfg(test)]
mod tests;
