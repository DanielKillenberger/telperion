use super::*;

#[derive(Clone)]
pub(in crate::pipeline::branching) struct Frontier {
    queue: VecDeque<Axis>,
    points: Vec<Vec3>,
    consumed: Vec<bool>,
    limbs: Limbs,
}
impl Frontier {
    pub(in crate::pipeline::branching) fn new(
        params: &SkeletonParams,
        config: &GrowthConfig,
        points: Vec<Vec3>,
    ) -> Self {
        Self {
            queue: super::fork::axes(params, config),
            consumed: vec![false; points.len()],
            points,
            limbs: Limbs::default(),
        }
    }
    /// The bound of every limb system stopped short so far.
    #[cfg(test)]
    pub(in crate::pipeline::branching) fn limbs(&self) -> &Limbs {
        &self.limbs
    }
    pub(in crate::pipeline::branching) fn finished(&self) -> bool {
        self.queue.is_empty()
    }
    pub(in crate::pipeline::branching) fn advance(
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
            influence_sq: config.influence_radius.powi(2),
            kill_sq: config.kill_distance.powi(2),
            limbs: &mut self.limbs,
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
pub(in crate::pipeline::branching) fn generate(
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

