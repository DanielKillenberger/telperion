//! Retained generator frontiers with birth/death stamps and stable timeline slots.
use super::*;
use crate::tree::{NodeIdentity, NodeKey};
use slotmap::{DenseSlotMap, Key};
mod changes;
mod chronicle;
mod contacts;
mod crown;
mod events;
mod history;
#[cfg(feature = "geometry")]
mod interval;
mod keyframes;
mod retention;
mod shared;
#[cfg(feature = "json")]
mod snapshot;
pub use changes::{ChangeRecord, Run, RunNode, SpecimenBuffers};
pub use history::SpecimenRead;
pub use shared::{PackedNode, PackedRead};
#[cfg(test)]
mod measurement;
mod storage;
mod survival;
#[cfg(test)]
mod thickening_tests;
mod timeline;
mod widths;

#[derive(Clone)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct Specimen {
    #[cfg(test)]
    #[cfg_attr(feature = "json", serde(skip))]
    cost: measurement::Cost,
    pub(super) tree: Tree,
    #[cfg_attr(feature = "json", serde(skip))]
    read: std::cell::OnceCell<storage::Read>,
    #[cfg_attr(feature = "json", serde(skip))]
    shared: std::cell::RefCell<Option<(crate::growth::Age, PackedRead)>>,
    #[cfg_attr(feature = "json", serde(skip))]
    read_active: bool,
    #[cfg_attr(feature = "json", serde(skip))]
    read_updates: Vec<usize>,
    pub(super) shed: usize,
    params: SkeletonParams,
    radii: RadiusParams,
    config: GrowthConfig,
    #[cfg_attr(feature = "json", serde(skip, default = "snapshot::empty_bias"))]
    bias: GrowthBias,
    scaffold: scaffold::Frontier,
    local: local::Frontier,
    next_identity: u64,
    timeline: Option<timeline::Timeline>,
    #[cfg_attr(feature = "json", serde(with = "crate::specimen::portable::indices"))]
    identities: DenseSlotMap<NodeKey, usize>,
    links: slotmap::SecondaryMap<NodeKey, chronicle::Links>,
    keyframes: keyframes::Keyframes,
    retention: retention::Retention,
    births: events::Events,
}
/// A stem's own root node: the structural node a stem leaves the root on. It
/// is the base of a trunk rather than a shoot, so the chronicle never sheds
/// it - the tree would be standing on nothing.
///
/// It is NOT stamped with the root's own birth year. A read of the tree at any
/// age is the tree a fresh build of that age grows, and a fresh build at year
/// zero has grown nothing at all: a stem stamped with year zero would appear
/// in the read and not in the build. Whether a clump's stems should instead be
/// born with the root, before the first slice runs, is the owner's call.
fn stem_root(node: &Node) -> bool {
    node.parent == Some(0) && node.kind == NodeKind::Structural
}

impl Specimen {
    pub fn new(params: &SkeletonParams, radii: RadiusParams) -> Result<Self> {
        crate::ranges::POSITIVE_COUNT.check(
            params.sampling_attempts_per_attractor as f64,
            "samplingAttemptsPerAttractor",
        )?;
        if params.attractors > crate::ranges::MAX_ATTRACTORS {
            return Err(Error::InvalidInput("attractors"));
        }
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
        scaffold::stems_placed(params, &config)?;

        let scaffold = scaffold::Frontier::new(params, &config, points);
        Ok(Self {
            #[cfg(test)]
            cost: measurement::Cost::default(),
            tree: Tree::default(),
            read: std::cell::OnceCell::new(),
            shared: Default::default(),
            read_active: false,
            read_updates: Vec::new(),
            shed: 0,
            params: params.clone(),
            radii,
            config,
            bias,
            scaffold,
            local: local::Frontier::default(),
            next_identity: 0,
            timeline: None,
            identities: DenseSlotMap::with_key(),
            links: slotmap::SecondaryMap::new(),
            keyframes: keyframes::Keyframes::default(),
            retention: retention::Retention::default(),
            births: events::Events::default(),
        })
    }
    pub fn tree(&self) -> &Tree {
        self.packed_read().map_or(&self.tree, |read| &read.tree)
    }
    pub fn identities(&self) -> impl ExactSizeIterator<Item = NodeIdentity> + '_ {
        self.tree().nodes.iter().map(|n| n.identity)
    }
    /// Resolve a living identity in the packed read; dead or stale keys fail.
    pub fn node(&self, identity: NodeIdentity) -> Result<&Node> {
        self.identities
            .get(identity.key)
            .filter(|&&i| i != usize::MAX)
            .and_then(|&i| match self.packed_read() {
                Some(read) => read.tree.nodes.get(read.indices[i]),
                None => self.tree.nodes.get(i),
            })
            .filter(|node| node.identity == identity && node.shoot.death_year.is_none())
            .ok_or(Error::InvalidInput("stale node identity"))
    }
    fn finished(&self) -> bool {
        self.scaffold.finished() && self.local.finished()
    }
    fn identify(&mut self) {
        self.identify_range(0..self.tree.nodes.len());
    }
    fn identify_range(&mut self, range: std::ops::Range<usize>) {
        #[cfg(test)]
        {
            self.cost.identities += range.len();
        }
        let mut born = Vec::new();
        let mut linked = Vec::new();
        for (i, node) in self
            .tree
            .nodes
            .iter_mut()
            .enumerate()
            .take(range.end)
            .skip(range.start)
        {
            if node.identity.key.is_null() {
                node.identity = NodeIdentity {
                    birth: self.next_identity,
                    key: self.identities.insert(i),
                };
                self.next_identity += 1;
                if let Some(t) = &self.timeline {
                    if t.foliage.slender(1.0) > 0.0 {
                        self.keyframes.track_eligibility(node.identity, node.kind);
                    }
                    linked.push(i);
                    node.shoot.birth_year = if i == 0 {
                        0.0
                    } else {
                        (t.age.slice + 1) as f64
                    };
                    if node.kind != NodeKind::Structural {
                        born.push(i);
                    }
                }
            } else {
                self.identities[node.identity.key] = i;
            }
        }
        self.link_births(&linked);
        if let Some(t) = &mut self.timeline {
            for i in born {
                t.widths.born(&mut self.tree, &t.pipes, i);
            }
        }
    }
    // Legacy envelope path only. Annual deaths never remove or reuse a slot.
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
            None,
        );
        if !self.tree.diagnostics.node_capped {
            self.local.advance(
                &mut self.tree,
                local::Planner {
                    clock: None,
                    widths: None,
                    growing_envelope: false,
                    planning: None,
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
        s.step(0, usize::MAX)?;
        debug_assert!(s.finished() || !s.tree.diagnostics.complete());
        s.shed = finish(&mut s.tree, params, radii)?;
        s.remap_after_shedding();
        Ok(s)
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod monthly_tests;

#[cfg(test)]
mod cost_tests;

#[cfg(test)]
mod read_tests;

#[cfg(test)]
mod foliage_tests;

#[cfg(test)]
mod change_tests;

#[cfg(test)]
mod chronicle_tests;

#[cfg(test)]
mod cohort_tests;
