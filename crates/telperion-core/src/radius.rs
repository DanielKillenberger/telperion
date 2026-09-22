//! Structural fork solve and the separate branch-local taper contract.
mod history;
mod incremental;
use crate::math::Transcendental;
use crate::{envelope::Envelope, tree::Tree, Error, Result};
pub(crate) use incremental::Pipes;
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct RadiusParams {
    /// The trunk's radius at the ground as a share of the tree's height,
    /// so raising it thickens every piece of wood in proportion.
    pub trunk_radius: f64,
    /// How wood divides at a fork. The parent's area is the sum of the
    /// children's radii raised to this power, so raising it leaves the
    /// children thicker for the same parent.
    pub fork_exponent: f64,
    /// How fast wood thins along its own length. Raising it makes a
    /// branch narrow more sharply from its base to its tip.
    pub length_taper: f64,
    /// The ceiling on accumulated taper, so no single long branch can
    /// thin away to nothing. Raising it lets long branches taper further.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_max_taper_exponent")
    )]
    pub max_taper_exponent: f64,
}
impl Default for RadiusParams {
    fn default() -> Self {
        Self {
            trunk_radius: 0.02,
            fork_exponent: 2.0,
            length_taper: 0.6,
            max_taper_exponent: crate::ranges::default_max_taper_exponent(),
        }
    }
}
impl RadiusParams {
    pub fn resolved(self) -> Result<Self> {
        crate::ranges::MAX_TAPER.check(self.max_taper_exponent, "maxTaperExponent")?;
        crate::ranges::TRUNK_RADIUS.check(self.trunk_radius, "trunkRadius")?;
        crate::ranges::FORK_EXPONENT.check(self.fork_exponent, "forkExponent")?;
        crate::ranges::LENGTH_TAPER.check(self.length_taper, "lengthTaper")?;
        Ok(self)
    }
}
pub fn solve(tree: &mut Tree, envelope: Envelope, params: RadiusParams) -> Result<()> {
    tree.validate()?;
    envelope.validate()?;
    let p = params.resolved()?;
    if tree.nodes.is_empty() {
        return Ok(());
    }
    let count = tree.crossover;
    if count == 0 {
        return Err(Error::InvalidInput("structural crossover"));
    }
    let height = envelope.height.max(1e-6);
    let trunk = p.trunk_radius * height;
    let mut shed = vec![0.0; count];
    let mut carried = vec![0.0_f64; count];
    for i in 1..count {
        let parent = tree.nodes[i].parent.unwrap() as usize;
        shed[i] = (shed[parent]
            + p.length_taper * tree.nodes[parent].position.distance(tree.nodes[i].position)
                / height)
            .min(p.max_taper_exponent);
    }
    for i in (0..count).rev() {
        let r = if carried[i] > 0.0 {
            carried[i].powf_fixed(1.0 / p.fork_exponent)
        } else {
            1.0
        };
        let start = if let Some(parent) = tree.nodes[i].parent {
            let parent = parent as usize;
            let start = r * (shed[i] - shed[parent]).exp_fixed();
            carried[parent] += start.powf_fixed(p.fork_exponent);
            start
        } else {
            r
        };
        tree.nodes[i].radius = r;
        tree.nodes[i].start_radius = start;
    }
    let scale = trunk / tree.nodes[0].radius;
    for n in &mut tree.nodes[..count] {
        n.radius *= scale;
        n.start_radius *= scale;
    }
    for i in count..tree.nodes.len() {
        tree.nodes[i].start_radius = if tree.nodes[i].branch as usize == i {
            tree.nodes[i].base_radius
        } else {
            tree.nodes[tree.nodes[i].parent.unwrap() as usize].radius
        };
    }
    tree.validate_solved()
}
