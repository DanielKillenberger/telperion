//! What each stage reads, derived once from the family: an owned copy of the
//! rows a stage reads and the family-level values derived from them, with no
//! reference back to the family. A derivation that can fail is kept as its
//! result and answered by the stage that reads it, so a request meets its
//! errors in stage order. Values that live at a branch or a surface sample
//! stay derived inside their stage, where their inputs exist.
#[cfg(feature = "geometry")]
use crate::pipeline::foliage::Reach;
use crate::{
    envelope::Envelope,
    math::Vec3,
    pipeline::foliage::{BloomParams, CanopyParams, ElementParams, TwigPlacement},
    pipeline::surface::SurfaceParams,
    pipeline::twigs::TwigParams,
    presets::{space::Space, Family},
    supernatural::Supernatural,
    tree::Tree,
    Result,
};
use std::borrow::Cow;

mod folds;
mod shoulder;
use folds::share;
pub use folds::{reach, Folds};
pub use shoulder::{beyond, bins, crown_point, BINS};

/// The input of every stage past the skeleton, read off one family.
#[derive(Debug, Clone)]
pub(crate) struct Inputs {
    pub(crate) plan: PlanInput,
    #[cfg(feature = "geometry")]
    pub(crate) surface: SurfaceInput,
    #[cfg(feature = "geometry")]
    pub(crate) leaves: LeafInput,
    /// What flowers where the tree's ages flower (fn-216).
    #[cfg(feature = "geometry")]
    pub(crate) bloom: BloomParams,
}

/// The skeleton: the engine's species, the age it grows to, the seed and
/// the cut below which a fine root stands as its expectation (fn-214; 0
/// grows every one) and the supernatural group, whose scale the grown tree
/// is drawn at. It lends the family's species for the one stage that reads
/// it, with the group's flowering laid over it while the group is on.
#[derive(Debug, Clone)]
pub(crate) struct GrowInput<'f> {
    pub(crate) space: Cow<'f, Space>,
    pub(crate) age: f64,
    pub(crate) seed: u32,
    pub(crate) cut: u32,
    pub(crate) supernatural: Supernatural,
    /// The crown's rows a rosette's stem establishes by (fn-275 R2a).
    pub(crate) canopy: crate::pipeline::foliage::CanopyParams,
}

impl<'f> GrowInput<'f> {
    pub(crate) fn of(family: &'f Family) -> Self {
        let space = match family.applied() {
            Cow::Borrowed(f) => Cow::Borrowed(&f.space),
            Cow::Owned(f) => Cow::Owned(f.space),
        };
        Self {
            space,
            age: family.age,
            seed: family.skeleton.seed,
            cut: 0,
            supernatural: family.supernatural,
            canopy: family.canopy,
        }
    }
}

/// The envelope of a grown tree (fn-198 host decision 7): its height, its
/// widest reach off the root as a share of it, where its crown starts (the
/// lowest node off the stem), where the crown is widest between that and
/// the top, and the least shoulder whose outline holds the crown (host
/// decision 11a). The outline is smooth: irregularity is 0, and the lobe
/// scale, which nothing on this path reads, is the struct's until R0
/// removes both rows. A tree with no wood off the stem has its crown at
/// its top.
pub(crate) fn grown_envelope(tree: &Tree) -> Envelope {
    let nodes = &tree.nodes;
    let folds = Folds::of(nodes, |_| true);
    envelope(folds, nodes.iter().map(|n| n.position))
}

/// The envelope of a tree whose nodes fold to `folds` and whose crown's
/// shoulder is read off `points`: every node's position, or any set of
/// them that holds every node no other passes (`shoulder.rs`). The folds
/// may be combined from parts of the tree (fn-212 task 4).
pub fn envelope(folds: Folds, points: impl Iterator<Item = Vec3> + Clone) -> Envelope {
    let shape @ [base, height, radius, fullness] = folds.shape();
    Envelope {
        height,
        crown_base: share(base, height),
        spread: if height > 0.0 { radius / height } else { 0.0 },
        fullness,
        shoulder: shoulder::contained(points, shape),
        irregularity: 0.0,
        ..Envelope::default()
    }
}

/// The element, the leaf plan and the box leaves are quantised against.
#[derive(Debug, Clone)]
pub(crate) struct PlanInput {
    pub(crate) element: ElementParams,
    pub(crate) envelope: Envelope,
    pub(crate) canopy: CanopyParams,
    pub(crate) surface: SurfaceParams,
    pub(crate) seed: u32,
    /// Where the family's twig rows put a leaf's stations.
    pub(crate) twig: Result<TwigPlacement>,
    /// How far a station stands from its wood: what the box leaves are
    /// quantised against grows the wood's extent by.
    #[cfg(feature = "geometry")]
    pub(crate) reach: Result<Reach>,
}

/// The rings and the wood swept on them.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SurfaceInput {
    pub(crate) height: f64,
    pub(crate) params: SurfaceParams,
}

/// Placement: the rows an executor places leaves by.
#[derive(Debug, Clone, Copy)]
pub struct LeafInput {
    pub envelope: Envelope,
    pub canopy: CanopyParams,
    pub seed: u32,
    pub shell_depth: f64,
}

impl Inputs {
    /// The inputs of every stage past the skeleton, under `envelope`: the
    /// grown tree's (`grown_envelope`), or for the suite's scaffold trees the
    /// family's own; read off the family as the supernatural group applies it.
    pub(crate) fn of(family: &Family, envelope: Envelope) -> Self {
        let applied = family.applied();
        let family = &*applied;
        let skeleton = &family.skeleton;
        let twigs = skeleton.twigs.resolved();
        Self {
            plan: PlanInput {
                element: family.element,
                envelope,
                canopy: family.canopy,
                surface: family.surface,
                seed: skeleton.seed,
                #[cfg(feature = "geometry")]
                twig: twigs.as_ref().map(placement).map_err(Clone::clone),
                #[cfg(not(feature = "geometry"))]
                twig: twigs.map(|twigs| placement(&twigs)),
                #[cfg(feature = "geometry")]
                reach: twigs.map(|twigs| {
                    let (radii, canopy) = (family.radii, family.canopy);
                    Reach::from_params(envelope, &twigs, radii, &family.surface, canopy)
                }),
            },
            #[cfg(feature = "geometry")]
            surface: SurfaceInput {
                height: envelope.height,
                params: family.surface,
            },
            #[cfg(feature = "geometry")]
            leaves: LeafInput {
                envelope,
                canopy: family.canopy,
                seed: skeleton.seed,
                shell_depth: family.shell_depth,
            },
            #[cfg(feature = "geometry")]
            bloom: family.bloom,
        }
    }
}

/// The stations a family's resolved twig rows place.
fn placement(twigs: &TwigParams) -> TwigPlacement {
    TwigPlacement {
        internode_length: twigs.twig.internode_length,
        stations_per_internode: twigs.twig.stations_per_internode,
    }
}
