use super::*;

#[derive(Clone)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub(in crate::branching) struct Frontier {
    queue: VecDeque<Axis>,
    points: Vec<Vec3>,
    consumed: Vec<Option<u64>>,
    pub(in crate::branching) year: u64,
    visited: Vec<usize>,
}
impl Frontier {
    pub(in crate::branching) fn visited(&self) -> impl Iterator<Item = usize> + '_ {
        self.visited.iter().copied()
    }
    #[cfg(test)]
    pub(in crate::branching) fn reverse_for_test(&mut self) {
        self.queue.make_contiguous().reverse();
    }
    pub(in crate::branching) fn new(
        params: &SkeletonParams,
        config: &GrowthConfig,
        points: Vec<Vec3>,
    ) -> Self {
        let base = config
            .trunk_height
            .max(params.envelope.height * params.envelope.crown_base);
        let top = base + (params.envelope.height - base) * params.habit.apical_dominance;
        let queue = if params.envelope.height > 0.0 && top > 0.0 {
            VecDeque::from([Axis::new(0, Vec3::Y, top, 0, params.seed ^ 0x742b_e831)])
        } else {
            VecDeque::new()
        };
        Self {
            queue,
            consumed: vec![None; points.len()],
            year: 0,
            visited: Vec::new(),
            points,
        }
    }
    pub(in crate::branching) fn remap(&mut self, map: &[Option<u32>]) {
        fn remap_axis(a: &mut Axis, map: &[Option<u32>]) -> bool {
            let (Some(at), Some(tip)) = (map[a.at], map[a.tip]) else {
                return false;
            };
            a.at = at as usize;
            a.tip = tip as usize;
            a.children.retain_mut(|child| remap_axis(child, map));
            true
        }
        self.queue.retain_mut(|a| remap_axis(a, map));
    }
    pub(in crate::branching) fn remove_dead(&mut self, tree: &Tree) {
        fn living(axis: &mut Axis, tree: &Tree) -> bool {
            if tree.nodes[axis.at].shoot.death_year.is_some()
                || tree.nodes[axis.tip].shoot.death_year.is_some()
            {
                return false;
            }
            axis.children.retain_mut(|a| living(a, tree));
            true
        }
        self.queue.retain_mut(|a| living(a, tree));
    }
    pub(in crate::branching) fn finished(&self) -> bool {
        self.queue.is_empty()
    }
    /// Each pending axis is visited in its origin's birth order, at most once
    /// per slice. A boundary-limited axis remains in the frontier for expansion.
    pub(in crate::branching) fn slice(
        &mut self,
        tree: &mut Tree,
        params: &SkeletonParams,
        config: &GrowthConfig,
        bias: &GrowthBias,
        budget: usize,
        fraction: f64,
    ) -> Result<usize> {
        self.visited.clear();
        let mut b = Builder {
            tree,
            envelope: Envelope {
                height: params.envelope.height * fraction,
                ..params.envelope
            },
            planning: inner_envelope(
                Envelope {
                    crown_base: params.envelope.crown_base * fraction,
                    ..params.envelope
                },
                params.twigs.reach,
            ),
            config,
            bias,
            habit: params.habit,
            points: &self.points,
            consumed: &mut self.consumed,
            year: self.year,
            influence_sq: config.influence_radius.powi(2),
            kill_sq: config.kill_distance.powi(2),
            point_scale: fraction,
            growing_envelope: true,
            paused: false,
        };
        self.queue
            .make_contiguous()
            .sort_by_key(|axis| (b.tree.nodes[axis.at].identity.birth_order(), axis.key));
        let mut remaining = budget;
        for _ in 0..self.queue.len() {
            if remaining == 0 || b.tree.diagnostics.node_capped {
                break;
            }
            let mut axis = self.queue.pop_front().unwrap();
            self.visited.push(axis.tip);
            if axis.order > 0
                && b.tree.nodes[axis.tip].shoot.vigour() < params.habit.shedding_threshold
            {
                self.queue.push_back(axis);
                continue;
            }
            b.paused = false;
            let finished = b.grow(&mut axis, &mut remaining)?;
            // Stations already reached bear buds now, even while the parent
            // waits for the envelope to expand. Children retain their keyed
            // streams and join the next slice's identity-ordered frontier.
            self.queue.extend(std::mem::take(&mut axis.children));
            if !finished {
                self.queue.push_back(axis);
            }
        }
        b.tree.crossover = b.tree.nodes.len();
        Ok(budget - remaining)
    }
    pub(in crate::branching) fn advance(
        &mut self,
        tree: &mut Tree,
        params: &SkeletonParams,
        config: &GrowthConfig,
        bias: &GrowthBias,
        mut budget: usize,
    ) -> Result<()> {
        let mut b = Builder {
            tree,
            envelope: params.envelope,
            planning: inner_envelope(params.envelope, params.twigs.reach),
            config,
            bias,
            habit: params.habit,
            points: &self.points,
            consumed: &mut self.consumed,
            year: self.year,
            influence_sq: config.influence_radius.powi(2),
            kill_sq: config.kill_distance.powi(2),
            point_scale: 1.0,
            growing_envelope: false,
            paused: false,
        };
        if b.tree.nodes.is_empty() && !b.capped() {
            b.tree.nodes.push(Node::root());
        }
        while budget > 0 && !b.tree.diagnostics.node_capped {
            let Some(mut axis) = self.queue.pop_front() else {
                break;
            };
            if b.grow(&mut axis, &mut budget)? {
                self.queue.extend(axis.children);
            } else {
                self.queue.push_front(axis);
            }
        }
        b.tree.crossover = b.tree.nodes.len();
        b.tree.validate()
    }
}

#[cfg(test)]
pub(in crate::branching) fn generate(
    params: &SkeletonParams,
    config: &GrowthConfig,
    bias: &GrowthBias,
    points: &[Vec3],
) -> Result<Tree> {
    let mut tree = Tree::default();
    Frontier::new(params, config, points.to_vec()).advance(
        &mut tree,
        params,
        config,
        bias,
        usize::MAX,
    )?;
    Ok(tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attractor_consumption_keeps_the_first_year() {
        let f = crate::presets::Preset::Ordinary.parameters();
        let config = f.skeleton.resolved_growth(3).unwrap();
        let bias = GrowthBias::new(f.skeleton.envelope, f.skeleton.seed, f.skeleton.bias).unwrap();
        let points = vec![Vec3::ZERO, Vec3::Y, Vec3::Y * 2.0];
        let mut consumed = vec![None; points.len()];
        let mut tree = Tree::default();
        let mut builder = Builder {
            tree: &mut tree,
            envelope: f.skeleton.envelope,
            planning: f.skeleton.envelope,
            config: &config,
            bias: &bias,
            habit: f.skeleton.habit,
            points: &points,
            consumed: &mut consumed,
            year: 3,
            influence_sq: 1.0,
            kill_sq: 0.01,
            point_scale: 1.0,
            growing_envelope: true,
            paused: false,
        };
        builder.consume(Vec3::ZERO, 1.0);
        builder.year = 8;
        builder.consume(Vec3::ZERO, 1.0);
        builder.consume(Vec3::Y, 1.0);
        assert_eq!(consumed, vec![Some(3), Some(8), None]);
    }

    #[test]
    fn annual_blocked_axes_leave_the_budget_for_live_shoots() {
        let f = crate::presets::Preset::NorwaySpruce.parameters();
        let p = &f.skeleton;
        let config = p.resolved_growth(0).unwrap();
        let bias = GrowthBias::new(p.envelope, p.seed, p.bias).unwrap();
        let mut frontier = Frontier::new(p, &config, Vec::new());
        let mut tree = Tree {
            nodes: vec![Node::root()],
            crossover: 1,
            ..Tree::default()
        };
        let spent = frontier
            .slice(&mut tree, p, &config, &bias, 100, 0.1)
            .unwrap();
        assert_eq!(
            spent,
            tree.nodes.len() - 1,
            "waiting for the envelope is not growth work"
        );
    }

    #[test]
    fn annual_laterals_extend_before_the_leader_finishes() {
        let f = crate::presets::Preset::NorwaySpruce.parameters();
        let p = &f.skeleton;
        let config = p.resolved_growth(0).unwrap();
        let bias = GrowthBias::new(p.envelope, p.seed, p.bias).unwrap();
        let mut frontier = Frontier::new(p, &config, Vec::new());
        let mut tree = Tree {
            nodes: vec![Node::root()],
            crossover: 1,
            ..Tree::default()
        };
        let config = GrowthConfig {
            trunk_height: config.trunk_height * 0.5,
            ..config
        };
        for _ in 0..4 {
            frontier
                .slice(&mut tree, p, &config, &bias, 10_000, 0.5)
                .unwrap();
        }
        assert!(
            frontier.queue.iter().any(|a| a.order == 0),
            "leader must still be growing"
        );
        assert!(
            tree.nodes
                .iter()
                .any(|n| n.position.x.abs() + n.position.z.abs() > 0.1),
            "a reached lateral station must grow before the parent axis completes"
        );
    }
}
