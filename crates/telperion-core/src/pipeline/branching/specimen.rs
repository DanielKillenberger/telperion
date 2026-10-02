//! The direct build's grower: the scaffold and twig frontiers drained in
//! botanical order, every node given a stable birth identity.
use super::*;
use crate::tree::{NodeIdentity, NodeKey};
use slotmap::{DenseSlotMap, Key};
mod budget;

pub(crate) struct Specimen {
    pub(super) tree: Tree,
    pub(super) shed: usize,
    params: SkeletonParams,
    radii: RadiusParams,
    config: GrowthConfig,
    bias: GrowthBias,
    scaffold: scaffold::Frontier,
    local: local::Frontier,
    next_identity: u64,
    identities: DenseSlotMap<NodeKey, usize>,
}
/// The rows `Specimen::new` judges before it scatters an attractor, each
/// refused by its own name; the twig rows come back resolved.
fn rows(params: &SkeletonParams, radii: RadiusParams) -> Result<TwigParams> {
    crate::catalogue::check(SkeletonParams::CHECKS, params, Site::Sampling)?;
    params.envelope.validate()?;
    params.bias.validate()?;
    params.habit.validate()?;
    radii.resolved()?;
    let twigs = params.twigs.resolved()?;
    crate::catalogue::check(SkeletonParams::CHECKS, params, Site::Step)?;
    if params.habit.attractor_weight > 0.0 && params.attractors == 0 {
        return Err(Error::InvalidInput("attractor weight and attractor count"));
    }
    Ok(twigs)
}

/// Every refusal `Specimen::new` can make of the rows, with no attractor
/// scattered and no node grown. The scatter's count stands in for the points:
/// a scatter that succeeds places exactly that many, or none in a crown with
/// no volume, and either way the growth configuration it resolves is the same.
pub(crate) fn validate(params: &SkeletonParams, radii: RadiusParams) -> Result<()> {
    let twigs = rows(params, radii)?;
    inner_envelope(params.envelope, twigs.reach).validate()?;
    let scattered = if params.habit.attractor_weight > 0.0 {
        params.attractors
    } else {
        0
    };
    params.resolved_growth(scattered)?;
    scaffold::forks_placed(params)
}

impl Specimen {
    pub(crate) fn new(params: &SkeletonParams, radii: RadiusParams) -> Result<Self> {
        let twigs = rows(params, radii)?;
        let inner = inner_envelope(params.envelope, twigs.reach);
        let points = if params.habit.attractor_weight > 0.0 {
            let _asks = crate::envelope::queries::during(Purpose::ScaffoldContainment);
            inner.sample_with_attempts(
                params.attractors,
                &mut Rng::new(params.seed),
                params.seed,
                params.sampling_attempts_per_attractor,
            )?
        } else {
            Vec::new()
        };
        let config = params.resolved_growth(points.len())?;
        let bias = GrowthBias::new(params.envelope, params.seed, params.bias)?;
        scaffold::forks_placed(params)?;

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
            identities: DenseSlotMap::with_key(),
        })
    }
    #[cfg(test)]
    fn identities(&self) -> impl ExactSizeIterator<Item = NodeIdentity> + '_ {
        self.tree.nodes.iter().map(|n| n.identity)
    }
    /// Resolve a living identity; dead or stale keys fail.
    #[cfg(test)]
    fn node(&self, identity: NodeIdentity) -> Result<&Node> {
        self.identities
            .get(identity.key)
            .and_then(|&i| self.tree.nodes.get(i))
            .filter(|node| node.identity == identity)
            .ok_or(Error::InvalidInput("stale node identity"))
    }
    fn finished(&self) -> bool {
        self.scaffold.finished() && self.local.finished()
    }
    fn identify(&mut self) {
        for (i, node) in self.tree.nodes.iter_mut().enumerate() {
            if node.identity.key.is_null() {
                node.identity = NodeIdentity {
                    birth: self.next_identity,
                    key: self.identities.insert(i),
                };
                self.next_identity += 1;
            } else {
                self.identities[node.identity.key] = i;
            }
        }
    }
    // Shedding removes nodes; their identities retire and are never reused.
    fn remap_after_shedding(&mut self) {
        let mut map = vec![None; self.identities.len()];
        for (i, node) in self.tree.nodes.iter().enumerate() {
            let previous = self.identities[node.identity.key];
            map[previous] = Some(i as u32);
        }
        self.local.remap(&map);
        self.identities
            .retain(|_, previous| map[*previous].is_some());
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
    /// Drain the scaffold, then the twig layer.
    /// `crowned` says every stem apex bears a rosette and so no twig layer.
    pub(crate) fn grow(
        params: &SkeletonParams,
        radii: RadiusParams,
        crowned: bool,
    ) -> Result<Self> {
        let mut s = Self::new(params, radii)?;
        s.local.crowned = crowned;
        s.step(usize::MAX, 0)?;
        let scaffold_fits = !s.tree.diagnostics.node_capped;
        s.step(0, usize::MAX)?;
        if scaffold_fits && s.tree.diagnostics.node_capped {
            s = Self::within_budget(params, radii, crowned)?;
        }
        debug_assert!(s.finished() || !s.tree.diagnostics.complete());
        s.shed = finish(&mut s.tree, params, radii, crowned)?;
        s.remap_after_shedding();
        Ok(s)
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod limb_tests;
