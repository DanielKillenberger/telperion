use super::*;

#[derive(Clone)]
pub(in crate::branching) struct Frontier {
    queue: VecDeque<Axis>,
    points: Vec<Vec3>,
    alive: Vec<bool>,
}
impl Frontier {
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
            alive: vec![true; points.len()],
            points,
        }
    }
    pub(in crate::branching) fn finished(&self) -> bool {
        self.queue.is_empty()
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
            alive: &mut self.alive,
            influence_sq: config.influence_radius.powi(2),
            kill_sq: config.kill_distance.powi(2),
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
