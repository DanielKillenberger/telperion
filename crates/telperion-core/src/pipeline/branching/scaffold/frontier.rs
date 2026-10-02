use super::*;

#[derive(Clone)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub(in crate::pipeline::branching) struct Frontier {
    queue: VecDeque<Axis>,
    points: Vec<Vec3>,
    consumed: Vec<Option<u64>>,
    pub(in crate::pipeline::branching) year: u64,
    visited: Vec<usize>,
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
            consumed: vec![None; points.len()],
            year: 0,
            visited: Vec::new(),
            points,
            limbs: Limbs::default(),
        }
    }
    /// The bound of every limb system stopped short so far.
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
            height: params.envelope.height,
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
            height: f.skeleton.envelope.height,
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
            limbs: &mut Limbs::default(),
        };
        builder.consume(Vec3::ZERO, 1.0);
        builder.year = 8;
        builder.consume(Vec3::ZERO, 1.0);
        builder.consume(Vec3::Y, 1.0);
        assert_eq!(consumed, vec![Some(3), Some(8), None]);
    }


}
