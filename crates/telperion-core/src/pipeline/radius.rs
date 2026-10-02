//! Structural fork solve and the separate branch-local taper contract.
mod hold;
use crate::catalogue::{bounded, tuned, value, Bounds, Growth, Site};
use crate::math::Transcendental;
use crate::{
    envelope::Envelope,
    tree::{BudFate, Tree},
    Error, Result,
};
pub(crate) use hold::hold;
crate::catalogue::rows! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct RadiusParams in "/radii" {
        /// The trunk's radius at the ground as a share of the tree's height,
        /// so raising it thickens every piece of wood in proportion.
        pub trunk_radius: f64 = "trunkRadius" "share of height"
            Bounds::closed(4e-6, f64::MAX) => [Grow, Plan] {
            wire: 74,
            check: value(Site::Radius, 1, "trunkRadius"),
            note: "Also sizes the leaf box where `canopy.shootRadius` is above zero or short \
                shoots grow.",
            dial: tuned("trunk_radius", "the trunk's radius at the ground as a share of the \
                height",
                [4e-06, 0.023], [0.002, 0.004], "capped").span([0.011, 0.023])
                .cap("ceiling capped: the generator validates only a floor here (4e-6), so there \
                    is no validated ceiling to widen to; the ceiling stays at the preset span \
                    until the generator authors one"),
        },
        /// How wood divides at a fork. The parent's area is the sum of the
        /// children's radii raised to this power, so raising it leaves the
        /// children thicker for the same parent.
        pub fork_exponent: f64 = "forkExponent" "-" Bounds::closed(1.0, 8.0) => [Grow] {
            wire: 75,
            check: value(Site::Radius, 2, "forkExponent"),
            dial: bounded("fork_exponent", "how wood divides at a fork; higher leaves the \
                children thicker", [1.0, 2.0]),
        },
        /// How fast wood thins along its own length. Raising it makes a
        /// branch narrow more sharply from its base to its tip.
        pub length_taper: f64 = "lengthTaper" "per height" Bounds::closed(0.0, f64::MAX) => [Grow] {
            wire: 76,
            check: value(Site::Radius, 3, "lengthTaper"),
            dial: tuned("length_taper", "how fast wood thins along its own length",
                [0.0, 0.8], [0.1, 0.2], "capped").span([0.0, 0.8])
                .cap("ceiling capped: the generator validates only a floor here (0), so there is \
                    no validated ceiling to widen to; the ceiling stays at the preset span until \
                    the generator authors one"),
        },
        /// The ceiling on accumulated taper, so no single long branch can
        /// thin away to nothing. Raising it lets long branches taper further.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_max_taper_exponent"))]
        pub max_taper_exponent: f64 = "maxTaperExponent" "-" Bounds::closed(0.0, 64.0) => [Grow] {
            wire: 77,
            check: value(Site::Radius, 0, "maxTaperExponent"),
            applies: "`lengthTaper` zero",
            dial: bounded("max_taper_exponent", "the ceiling on accumulated taper along one long \
                branch", [2.0, 4.0]).span([6.0, 18.0]),
        },
        /// The wood a lateral takes at a fork against what its own subtree
        /// asks, 0 to 1: the parent carries a lateral's pipe at this share and
        /// the lateral's wood thins by its root. At one every fork divides by
        /// the pipe model alone.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_share"))]
        pub lateral_share: f64 = "lateralShare" "share" Bounds::closed(0.01, 1.0) => [Grow] {
            wire: 246,
            check: value(Site::Radius, 4, "lateralShare"),
            note: "Below one a lateral leaves thinner than a continuation carrying as many \
                tips. A lateral is the wood the scaffold marks `BudFate::Lateral`.",
            dial: bounded("lateral_share", "how thin a limb leaves the axis it grows from, \
                against its own reach; lower keeps the leaders' girth", [0.1, 0.2]),
        },
        /// The wood a codominant sibling takes at a fork against its own
        /// subtree's, 0 to 1, beside the primary that carries the axis on. At
        /// one the parts divide by the pipe model alone.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_share"))]
        pub fork_balance: f64 = "forkBalance" "share" Bounds::closed(0.01, 1.0) => [Grow] {
            wire: 247,
            check: value(Site::Radius, 5, "forkBalance"),
            applies: "`skeleton.habit.codominance` zero",
            note: "A sibling is the wood the scaffold marks `codominant`; its share is this \
                times the weight it has grown in by. At one a sibling takes its own pipe as \
                the primary does.",
            dial: bounded("fork_balance", "how much thinner the lesser parts of a codominant \
                fork leave than the part carrying the axis on", [0.1, 0.2]),
        },
        /// The share of each structural axis's reach over which it holds the
        /// girth it starts with, 0 to 0.9: past it the wood falls to the pipe
        /// model's radius by the axis's tip. At zero wood is the pipe model's.
        #[cfg_attr(feature = "json", serde(default))]
        pub girth_hold: f64 = "girthHold" "share of reach" Bounds::closed(0.0, 0.9) => [Grow] {
            wire: 248,
            check: value(Site::Radius, 6, "girthHold"),
            growth: Growth::Ignored,
            note: "Every part leaving the root starts an axis, as do a lateral and a \
                codominant sibling; a fork's primary above the root carries its axis on. An \
                axis holds the pipe model's radius where it leaves its parent. A node's share \
                of its axis's reach is its path from there against that plus its path on \
                along the axis to the axis's tip. Its radius is the larger of the pipe model's \
                and the held girth, so a held fork's parts carry more wood than their parent: \
                conservation at forks is given up over the hold, and a part's start never \
                exceeds its parent's radius. Read once the twigs have grown, and which wood \
                bears leaves is decided on the pipe model's radii, so no twig or leaf is \
                added or lost; the ceiling leaves every axis a tenth of its reach to \
                fall, so no tip ends blunt.",
            dial: bounded("girth_hold", "how far along its reach a limb keeps the girth it \
                starts with before it breaks into fine wood", [0.05, 0.15]),
        },
        /// How short the fall after `girthHold` is: the fall lasts the hold's
        /// share divided by this, so raising it breaks the wood into twigs
        /// over a shorter distance.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_girth_fall"))]
        pub girth_fall: f64 = "girthFall" "-" Bounds::closed(0.5, 8.0) => [Grow] {
            wire: 249,
            check: value(Site::Radius, 7, "girthFall"),
            growth: Growth::Ignored,
            applies: "`girthHold` zero",
            note: "Ends at the tip where the hold leaves too little reach. At 0.5 the fall is \
                twice the hold, close to the pipe model's steady thinning; at 8 it is an eighth \
                of it, a break over a few stations of a long limb.",
            dial: bounded("girth_fall", "how abruptly a held limb breaks into fine wood after \
                its hold", [0.5, 2.0]),
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
            lateral_share: 1.0,
            fork_balance: 1.0,
            girth_hold: 0.0,
            girth_fall: crate::ranges::default_girth_fall(),
        }
    }
}
impl RadiusParams {
    pub fn resolved(self) -> Result<Self> {
        crate::catalogue::check(Self::CHECKS, &self, Site::Radius)?;
        Ok(self)
    }
}
/// The share of its own pipe node `j` takes at its fork, which its whole
/// subtree thins by the root of: a lateral `lateralShare`, a codominant
/// sibling `forkBalance` of its weight, and a primary all of it. The one
/// allocation both solves read, so the parent's wood is always the sum of
/// the wood its children leave with.
pub(crate) fn share(tree: &Tree, j: usize, p: RadiusParams) -> f64 {
    let n = &tree.nodes[j];
    match (n.shoot.bud_fate, n.codominant) {
        (BudFate::Lateral, _) => p.lateral_share,
        (BudFate::Terminal, Some(weight)) => p.fork_balance * weight,
        (BudFate::Terminal, None) => 1.0,
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
    for i in 1..count {
        let parent = tree.nodes[i].parent.unwrap() as usize;
        shed[i] = (shed[parent]
            + p.length_taper * tree.nodes[parent].position.distance(tree.nodes[i].position)
                / height)
            .min(p.max_taper_exponent);
    }
    // Each node's children, latest first: the order the pipes were always
    // summed in, so a neutral share divides the wood to the bit as it did.
    let mut first = vec![usize::MAX; count];
    let mut next = vec![usize::MAX; count];
    for (i, later) in next.iter_mut().enumerate().skip(1) {
        let parent = tree.nodes[i].parent.unwrap() as usize;
        *later = first[parent];
        first[parent] = i;
    }
    let children = |i: usize| {
        std::iter::successors(Some(first[i]).filter(|&k| k != usize::MAX), |&k| {
            Some(next[k]).filter(|&k| k != usize::MAX)
        })
    };
    // Bottom up, each node's own pipe and the share of it its parent carries.
    let mut own = vec![(1.0, 1.0); count];
    for i in (0..count).rev() {
        let carried: f64 = children(i)
            .map(|j| share(tree, j, p) * own[j].1.powf_fixed(p.fork_exponent))
            .sum();
        let r = if carried > 0.0 {
            carried.powf_fixed(1.0 / p.fork_exponent)
        } else {
            1.0
        };
        let start = match tree.nodes[i].parent {
            Some(parent) => r * (shed[i] - shed[parent as usize]).exp_fixed(),
            None => r,
        };
        own[i] = (r, start);
    }
    // Top down, a lateral's or a sibling's whole subtree thins by the root of
    // the share its fork gave it.
    let mut factor = vec![1.0; count];
    for i in 0..count {
        if let Some(parent) = tree.nodes[i].parent {
            factor[i] =
                factor[parent as usize] * share(tree, i, p).powf_fixed(1.0 / p.fork_exponent);
        }
        tree.nodes[i].radius = own[i].0 * factor[i];
        tree.nodes[i].start_radius = own[i].1 * factor[i];
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
#[cfg(test)]
mod hold_tests;
#[cfg(test)]
mod share_tests;
