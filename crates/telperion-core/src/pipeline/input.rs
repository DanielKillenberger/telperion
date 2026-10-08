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
    math::Transcendental,
    pipeline::foliage::{CanopyParams, ElementParams, TwigPlacement},
    pipeline::surface::SurfaceParams,
    pipeline::twigs::TwigParams,
    presets::{space::Space, Family},
    tree::Tree,
    Result,
};

mod shoulder;

/// The input of every stage past the skeleton, read off one family.
#[derive(Debug, Clone)]
pub(crate) struct Inputs {
    pub(crate) plan: PlanInput,
    #[cfg(feature = "geometry")]
    pub(crate) surface: SurfaceInput,
    #[cfg(feature = "geometry")]
    pub(crate) leaves: LeafInput,
}

/// The skeleton: the engine's species, the age it grows to, the seed and
/// the cut below which a fine root stands as its expectation (fn-214; 0
/// grows every one). It lends the family's species for the one stage that
/// reads it.
#[derive(Debug, Clone, Copy)]
pub(crate) struct GrowInput<'f> {
    pub(crate) space: &'f Space,
    pub(crate) age: f64,
    pub(crate) seed: u32,
    pub(crate) cut: u32,
}

impl<'f> GrowInput<'f> {
    pub(crate) fn of(family: &'f Family) -> Self {
        Self {
            space: &family.space,
            age: family.age,
            seed: family.skeleton.seed,
            cut: 0,
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
    let height = nodes.iter().map(|n| n.position.y).fold(0.0, f64::max);
    let reach = |n: &crate::tree::Node| n.position.x.hypot_fixed(n.position.z);
    let base = nodes
        .iter()
        .filter(|n| !n.stem && n.parent.is_some())
        .map(|n| n.position.y)
        .fold(height, f64::min);
    let widest = nodes.iter().max_by(|a, b| reach(a).total_cmp(&reach(b)));
    let (radius, at) = widest.map_or((0.0, height), |n| (reach(n), n.position.y));
    let share = |v: f64, of: f64| {
        if of > 0.0 {
            (v / of).clamp(0.0, 1.0)
        } else {
            0.0
        }
    };
    let fullness = share(at - base, height - base);
    Envelope {
        height,
        crown_base: share(base, height),
        spread: if height > 0.0 { radius / height } else { 0.0 },
        fullness,
        shoulder: shoulder::contained(nodes, [base, height, radius, fullness]),
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

/// Placement and the cull: the rows an executor places leaves by.
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
    /// family's own.
    pub(crate) fn of(family: &Family, envelope: Envelope) -> Self {
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
