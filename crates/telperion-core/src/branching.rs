//! Crown, local branches, shell shedding, and final radius solve, in botanical order.
use crate::math::Transcendental;
mod local;
mod scaffold;
mod specimen;
mod traits;
use crate::ranges::DEFAULT_MAX_NODES;
use crate::{
    bias::{BiasParams, GrowthBias},
    colonization::{self, GrowthConfig},
    envelope::{distance_to_profile, Envelope},
    math::Vec3,
    radius::{self, RadiusParams},
    rng::Rng,
    tree::{Node, NodeKind, Tree},
    twigs::{branch_length, child_radius, TwigParams},
    Error, Result,
};
pub use local::{append, in_band as in_curtain_band};
pub use specimen::{
    ChangeRecord, PackedNode, PackedRead, Run, RunNode, Specimen, SpecimenBuffers, SpecimenRead,
};
pub use traits::HabitParams;
pub const DEFAULT_STEP: f64 = 0.022;
#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct SkeletonParams {
    pub seed: u32,
    pub habit: HabitParams,
    pub envelope: Envelope,
    /// How many pull points are scattered through the crown for the
    /// branches to grow toward. Raising it fills the crown with more and
    /// finer branching; at an `attractor_weight` of zero none are
    /// scattered and the row does nothing.
    pub attractors: usize,
    /// How many random tries the sampler may spend on each pull point
    /// before it gives up. Raising it lets a narrow or deeply lobed crown
    /// reach its full count of points instead of settling for fewer.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_sampling_attempts_per_attractor")
    )]
    pub sampling_attempts_per_attractor: u32,
    /// How far the crown grows in one step, as a share of the tree's
    /// height; the distance at which a pull point is used up is twice it.
    /// Raising it grows the crown in longer, coarser strides.
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
            sampling_attempts_per_attractor: crate::ranges::default_sampling_attempts_per_attractor(
            ),
            step: DEFAULT_STEP,
            bias: BiasParams::default(),
            twigs: TwigParams::default(),
            growth: GrowthOverrides::default(),
        }
    }
}
/// Metre-valued overrides applied after envelope-derived distances.
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct GrowthOverrides {
    /// Metres a pull point may reach to steer the wood nearest it; unset,
    /// the crown's own volume and the point count decide it. Raising it
    /// lets distant points draw a branch across the crown.
    pub influence_radius: Option<f64>,
    /// Metres within which a pull point counts as reached and stops
    /// pulling; unset, twice the step distance. Raising it uses the points
    /// up sooner, so branches stop shorter and the crown fills coarsely.
    pub kill_distance: Option<f64>,
    /// Metres of wood laid down in one growth step; unset, the tree's
    /// height times `step`. Raising it lays down longer, coarser segments.
    pub step_distance: Option<f64>,
    /// Metres of bare trunk before the crown may start; unset, the
    /// envelope's own crown base. Raising it lifts the whole crown and
    /// leaves a longer clear bole.
    pub trunk_height: Option<f64>,
    /// The ceiling on nodes the crown may grow; unset, the shipped
    /// default. Growth stops at it, so raising it changes only a crown
    /// that reached it.
    pub max_nodes: Option<usize>,
    pub max_turn_per_step: Option<f64>,
}
impl SkeletonParams {
    pub fn resolved_growth(&self, scattered: usize) -> Result<GrowthConfig> {
        let mut c = default_growth(self.envelope, scattered, self.step);
        c.seed = self.seed;
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
    let share = 1.0 - reach;
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
        steps.max(2.0 * (volume / attractors as f64).cbrt_fixed())
    }
}
pub fn default_growth(e: Envelope, attractors: usize, step: f64) -> GrowthConfig {
    let distance = e.height.max(1e-6) * step;
    GrowthConfig {
        step_distance: distance,
        kill_distance: distance * 2.0,
        influence_radius: influence_radius(e, distance, attractors),
        trunk_height: e.height * e.crown_base,
        max_nodes: DEFAULT_MAX_NODES,
        shell: Some(e),
        ..Default::default()
    }
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
        let r = n.position.x.hypot_fixed(n.position.z);
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
/// Drop the local layer standing above every stem apex: the twig wood a
/// childless order-zero axis carries at its tip, and everything borne on it.
///
/// An apex that bears a rosette bears no twig. Suppressing only the foliage
/// there would leave bare twig wood under the fronds, which no tree that
/// carries a frond crown has. Only nodes past the crossover are dropped, so
/// the structural scaffold and the crossover itself are untouched.
pub fn clear_apical_twigs(tree: &mut Tree) -> Result<()> {
    let count = tree.nodes.len();
    let crossover = tree.crossover.min(count);
    let mut apex = vec![false; count];
    for i in tree.stem_apices() {
        apex[i] = true;
    }
    // A parent is always stored before its child, so one forward pass carries
    // the apex's whole local subtree.
    let mut dropped = vec![false; count];
    for i in crossover..count {
        let parent = tree.nodes[i].parent.unwrap() as usize;
        dropped[i] = dropped[parent] || apex[parent];
    }
    if !dropped.iter().any(|&d| d) {
        return Ok(());
    }
    let mut index = vec![0_u32; count];
    let mut next = 0_u32;
    for (i, &drop) in dropped.iter().enumerate() {
        if !drop {
            index[i] = next;
            next += 1;
        }
    }
    let mut old = 0;
    tree.nodes.retain_mut(|n| {
        let i = old;
        old += 1;
        if dropped[i] {
            return false;
        }
        n.parent = n.parent.map(|p| index[p as usize]);
        n.branch = index[n.branch as usize];
        true
    });
    tree.validate()
}

/// Generate solved structure only. Representations are independent borrowed-tree requests.
pub fn generate(params: &SkeletonParams, radii: RadiusParams) -> Result<GrowthReport> {
    let specimen = Specimen::grow(params, radii)?;
    Ok(GrowthReport {
        tree: specimen.tree,
        shed: specimen.shed,
    })
}
fn finish(tree: &mut Tree, params: &SkeletonParams, radii: RadiusParams) -> Result<usize> {
    let twigs = params.twigs.resolved()?;
    let removed = if params.habit.shedding_threshold > 0.0 {
        shed(tree, params.envelope, params.habit.shedding_threshold)?
    } else {
        0
    };
    radius::solve(tree, params.envelope, radii)?;
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
    Ok(removed)
}

#[cfg(test)]
mod audit;
