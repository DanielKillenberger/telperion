//! What each stage reads, derived once from the family: an owned copy of the
//! rows a stage reads and the family-level values derived from them, with no
//! reference back to the family. A derivation that can fail is kept as its
//! result and answered by the stage that reads it, so a request meets its
//! errors in stage order. Values that live at a branch or a surface sample
//! stay derived inside their stage, where their inputs exist.
#[cfg(feature = "geometry")]
use crate::pipeline::foliage::Reference;
use crate::{
    envelope::Envelope,
    pipeline::branching::SkeletonParams,
    pipeline::foliage::{CanopyParams, ElementParams, TwigPlacement},
    pipeline::radius::RadiusParams,
    pipeline::surface::SurfaceParams,
    pipeline::twigs::TwigParams,
    presets::Family,
    Result,
};

/// The input of every stage past the skeleton, read off one family.
#[derive(Debug, Clone)]
pub(crate) struct Inputs {
    pub(crate) plan: PlanInput,
    #[cfg(feature = "geometry")]
    pub(crate) surface: SurfaceInput,
    #[cfg(feature = "geometry")]
    pub(crate) leaves: LeafInput,
}

/// The skeleton: the solve, and the canopy's frond crown and shed bases.
/// It lends the family's groups for the one stage that reads them.
#[derive(Debug, Clone, Copy)]
pub(crate) struct GrowInput<'f> {
    pub(crate) skeleton: &'f SkeletonParams,
    pub(crate) radii: RadiusParams,
    pub(crate) canopy: &'f CanopyParams,
}

impl<'f> GrowInput<'f> {
    pub(crate) fn of(family: &'f Family) -> Self {
        Self {
            skeleton: &family.skeleton,
            radii: family.radii,
            canopy: &family.canopy,
        }
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
    /// The box every leaf of this family is quantised against.
    #[cfg(feature = "geometry")]
    pub(crate) reference: Result<Reference>,
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
    pub(crate) fn of(family: &Family) -> Self {
        let skeleton = &family.skeleton;
        let envelope = skeleton.envelope;
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
                reference: twigs.map(|twigs| {
                    let (radii, canopy) = (family.radii, family.canopy);
                    Reference::from_params(envelope, &twigs, radii, &family.surface, canopy)
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
