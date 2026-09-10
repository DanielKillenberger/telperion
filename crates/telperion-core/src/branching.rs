//! Crown, local branches, shell shedding, and final radius solve, in botanical order.
mod local;
mod scaffold;
mod traits;
use crate::{
    bias::{BiasParams, GrowthBias},
    colonization::{self, GrowthConfig},
    envelope::{distance_to_profile, Envelope},
    math::Vec3,
    radius::{self, RadiusParams},
    rng::Rng,
    tree::{Node, NodeKind, Tree},
    twigs::{branch_length, child_radius, TwigParams, MAX_LEVELS},
    Error, Result,
};
pub use local::append;
pub use traits::HabitParams;
pub const NODE_CEILING: usize = 250_000;
pub const DEFAULT_STEP: f64 = 0.022;
#[derive(Debug, Clone)]
pub struct SkeletonParams {
    pub seed: u32,
    pub habit: HabitParams,
    pub envelope: Envelope,
    pub attractors: usize,
    pub step: f64,
    pub bias: BiasParams,
    pub twigs: TwigParams,
    pub growth: GrowthOverrides,
}
impl Default for SkeletonParams {
    fn default() -> Self {
        Self {
            seed: 42,
            habit: HabitParams::default(),
            envelope: Envelope::default(),
            attractors: 500,
            step: DEFAULT_STEP,
            bias: BiasParams::default(),
            twigs: TwigParams::default(),
            growth: GrowthOverrides::default(),
        }
    }
}
/// Metre-valued overrides applied after envelope-derived distances.
#[derive(Debug, Clone, Copy, Default)]
pub struct GrowthOverrides {
    pub influence_radius: Option<f64>,
    pub kill_distance: Option<f64>,
    pub step_distance: Option<f64>,
    pub trunk_height: Option<f64>,
    pub max_nodes: Option<usize>,
    pub max_turn_per_step: Option<f64>,
}
impl SkeletonParams {
    pub fn resolved_growth(&self, scattered: usize) -> Result<GrowthConfig> {
        let mut c = default_growth(self.envelope, scattered, self.step);
        let o = self.growth;
        if let Some(v) = o.influence_radius {
            c.influence_radius = v
        }
        if let Some(v) = o.kill_distance {
            c.kill_distance = v
        }
        if let Some(v) = o.step_distance {
            c.step_distance = v
        }
        if let Some(v) = o.trunk_height {
            c.trunk_height = v
        }
        if let Some(v) = o.max_nodes {
            c.max_nodes = v
        }
        if let Some(v) = o.max_turn_per_step {
            c.max_turn_per_step = v
        }
        c.validate()?;
        Ok(c)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GrowthReport {
    pub tree: Tree,
    pub shed: usize,
}
pub fn inner_envelope(e: Envelope, reach: f64) -> Envelope {
    let share = 1.0 - reach.clamp(0.0, 0.9);
    let base = e.height * e.crown_base;
    let height = base + (e.height - base) * share;
    if height <= 0.0 {
        return e;
    }
    Envelope {
        height,
        crown_base: base / height,
        spread: e.spread * share * (e.height / height),
        ..e
    }
}
pub fn influence_radius(e: Envelope, step_distance: f64, attractors: usize) -> f64 {
    let base = e.height * e.crown_base;
    let span = e.height - base;
    let sum: f64 = (0..256)
        .map(|i| e.radius_at(base + (i as f64 + 0.5) / 256.0 * span).powi(2))
        .sum();
    let volume = std::f64::consts::PI * sum * (span / 256.0);
    let steps = 9.0 * step_distance;
    if attractors == 0 || volume <= 0.0 {
        steps
    } else {
        steps.max(2.0 * (volume / attractors as f64).cbrt())
    }
}
pub fn default_growth(e: Envelope, attractors: usize, step: f64) -> GrowthConfig {
    let distance = e.height.max(1e-6) * step;
    GrowthConfig {
        step_distance: distance,
        kill_distance: distance * 2.0,
        influence_radius: influence_radius(e, distance, attractors),
        trunk_height: e.height * e.crown_base,
        max_nodes: NODE_CEILING,
        shell: Some(e),
        ..Default::default()
    }
}
fn headroom(tree: &Tree, c: &GrowthConfig, t: TwigParams) -> usize {
    fn nodes_for(radius: f64, length: f64, generation: usize, t: TwigParams, ratio: f64) -> usize {
        if radius <= t.twig.diameter / 2.0 || length < t.twig.internode_length {
            return 1;
        }
        if generation >= MAX_LEVELS {
            return 0;
        }
        if radius <= t.twig.bearing_diameter / 2.0 {
            return 64;
        }
        let offspring = if t.laterals == 0 {
            0
        } else {
            t.laterals as usize
                * nodes_for(
                    child_radius(radius, ratio, t.ratio_power),
                    length * ratio,
                    generation + 1,
                    t,
                    ratio,
                )
        };
        (33 + offspring).clamp(64, NODE_CEILING)
    }
    let ratio = (t.length_ratio * (1.0 + t.vigour_variation)).min(1.0);
    let mut children = vec![0; tree.nodes.len()];
    for n in tree.nodes.iter().skip(1) {
        children[n.parent.unwrap() as usize] += 1
    }
    let mut estimate = 0;
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        if n.position.y < c.trunk_height {
            continue;
        }
        let length = branch_length(n.radius);
        if children[i] == 0 {
            estimate += nodes_for(n.radius, length, 0, t, ratio)
        }
        if n.radius < t.limb_radius * tree.nodes[0].radius {
            estimate += t.laterals as usize
                * nodes_for(
                    child_radius(n.radius, ratio, t.ratio_power),
                    length * ratio,
                    1,
                    t,
                    ratio,
                )
        }
        if estimate >= NODE_CEILING {
            return NODE_CEILING;
        }
    }
    estimate
}
/// Retain illuminated subtrees and complete leader runs; remap every parent and run ID.
pub fn shed(tree: &mut Tree, envelope: Envelope, shell_depth: f64) -> Result<usize> {
    tree.validate()?;
    envelope.validate()?;
    if !shell_depth.is_finite() || shell_depth < 0.0 {
        return Err(Error::InvalidInput("shedding shell depth"));
    }
    let first = tree.crossover;
    let count = tree.nodes.len();
    if first >= count {
        return Ok(0);
    }
    if first == 0 {
        return Err(Error::InvalidInput("structural crossover"));
    }
    let shell = shell_depth * envelope.max_radius();
    let profile = envelope.profile();
    let mut keep = vec![false; count];
    keep[..first].fill(true);
    for (i, n) in tree.nodes.iter().enumerate().skip(first) {
        let r = n.position.x.hypot(n.position.z);
        keep[i] = envelope.radius_at(n.position.y) - r <= shell
            || distance_to_profile(&profile, r, n.position.y) <= shell;
    }
    for i in (first..count).rev() {
        if keep[i] {
            keep[tree.nodes[i].parent.unwrap() as usize] = true
        }
    }
    let mut live = vec![false; count];
    for i in first..count {
        if keep[i] {
            live[tree.nodes[i].branch as usize] = true
        }
    }
    for i in first..count {
        let n = &tree.nodes[i];
        if live[n.branch as usize] {
            keep[i] = true
        }
        let parent = n.parent.unwrap() as usize;
        if n.kind == NodeKind::Twig
            && parent >= first
            && keep[parent]
            && tree.nodes[parent].radius == n.base_radius
        {
            keep[i] = true
        }
    }
    let mut index = vec![0; count];
    let mut next = 0;
    for i in 0..count {
        if keep[i] {
            index[i] = next;
            next += 1
        }
    }
    let mut old = 0;
    tree.nodes.retain_mut(|n| {
        let i = old;
        old += 1;
        if !keep[i] {
            return false;
        }
        n.parent = n.parent.map(|p| index[p as usize]);
        n.branch = index[n.branch as usize];
        true
    });
    tree.validate()?;
    Ok(count - tree.nodes.len())
}
/// Generate solved structure only. Representations are independent borrowed-tree requests.
pub fn generate(params: &SkeletonParams, radii: RadiusParams) -> Result<GrowthReport> {
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
    let mut tree = scaffold::generate(params, &config, &bias, &points)?;
    radius::solve(&mut tree, params.envelope, radii)?;
    let max_nodes = config
        .max_nodes
        .min(NODE_CEILING)
        .min(tree.nodes.len() + headroom(&tree, &config, twigs));
    local::append(
        &mut tree,
        &GrowthConfig {
            max_nodes,
            ..config
        },
        twigs,
        params.seed,
        Some(&bias),
        params.habit,
    )?;
    let removed = if params.habit.shedding_threshold > 0.0 {
        shed(&mut tree, params.envelope, params.habit.shedding_threshold)?
    } else {
        0
    };
    radius::solve(&mut tree, params.envelope, radii)?;
    // The distal end of a childless structural axis carries no wood the local
    // layer would have thinned; the taper trait says how far it narrows.
    let mut has_children = vec![false; tree.crossover];
    for node in tree.nodes.iter().skip(1) {
        if let Some(parent) = has_children.get_mut(node.parent.unwrap() as usize) {
            *parent = true;
        }
    }
    let tip_radius = twigs.twig.diameter / 2.0 * params.habit.twig_tip_taper;
    for (i, node) in tree
        .nodes
        .iter_mut()
        .take(tree.crossover)
        .enumerate()
        .skip(1)
    {
        if node.kind == NodeKind::Structural && !has_children[i] {
            node.radius = node.radius.min(tip_radius);
        }
    }
    Ok(GrowthReport {
        tree,
        shed: removed,
    })
}

#[cfg(test)]
mod audit;
