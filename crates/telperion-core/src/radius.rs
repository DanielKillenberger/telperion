//! Structural fork solve and the separate branch-local taper contract.
mod history;
mod incremental;
use crate::catalogue::{bounded, tuned, value, Bounds, Site};
use crate::math::Transcendental;
use crate::{envelope::Envelope, tree::Tree, Error, Result};
pub(crate) use incremental::Pipes;
crate::catalogue::rows! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct RadiusParams in "/radii" {
        /// The trunk's radius at the ground as a share of the tree's height,
        /// so raising it thickens every piece of wood in proportion.
        pub trunk_radius: f64 = "trunkRadius" "share of height"
            Bounds::closed(4e-6, f64::MAX) => [Grow, Plan] {
            check: value(Site::Radius, 1, "trunkRadius"),
            note: "Also sizes the leaf box where `canopy.shootRadius` is above zero or short \
                shoots grow.",
            dial: tuned("trunk_radius", "the trunk's radius at the ground as a share of the \
                height", [0.011, 0.023], [0.002, 0.004], "preset span"),
        },
        /// How wood divides at a fork. The parent's area is the sum of the
        /// children's radii raised to this power, so raising it leaves the
        /// children thicker for the same parent.
        pub fork_exponent: f64 = "forkExponent" "-" Bounds::closed(1.0, 8.0) => [Grow] {
            check: value(Site::Radius, 2, "forkExponent"),
            dial: bounded("fork_exponent", "how wood divides at a fork; higher leaves the \
                children thicker", [1.0, 2.0]),
        },
        /// How fast wood thins along its own length. Raising it makes a
        /// branch narrow more sharply from its base to its tip.
        pub length_taper: f64 = "lengthTaper" "per height" Bounds::closed(0.0, f64::MAX) => [Grow] {
            check: value(Site::Radius, 3, "lengthTaper"),
            dial: tuned("length_taper", "how fast wood thins along its own length",
                [0.0, 0.8], [0.1, 0.2], "preset span"),
        },
        /// The ceiling on accumulated taper, so no single long branch can
        /// thin away to nothing. Raising it lets long branches taper further.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_max_taper_exponent"))]
        pub max_taper_exponent: f64 = "maxTaperExponent" "-" Bounds::closed(0.0, 64.0) => [Grow] {
            check: value(Site::Radius, 0, "maxTaperExponent"),
            applies: "`lengthTaper` zero",
            dial: tuned("max_taper_exponent", "the ceiling on accumulated taper along one long \
                branch", [6.0, 18.0], [2.0, 4.0], "preset span"),
        },
    }
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
        crate::catalogue::check(Self::ROWS, &self, Site::Radius)?;
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
