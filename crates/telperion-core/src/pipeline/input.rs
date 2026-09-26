//! What each stage reads, derived once from the family: an owned copy of the
//! rows a stage reads and the family-level values derived from them, with no
//! reference back to the family. A derivation that can fail is kept as its
//! result and answered by the stage that reads it, so a request meets its
//! errors in stage order. Values that live at a branch or a surface sample
//! stay derived inside their stage, where their inputs exist.
use crate::{
    branching::SkeletonParams,
    envelope::Envelope,
    foliage::{CanopyParams, ElementParams, Reference, TwigPlacement},
    presets::Family,
    radius::RadiusParams,
    surface::SurfaceParams,
    twigs::TwigParams,
    Result,
};

/// Every stage's input, read off one family.
#[derive(Debug, Clone)]
pub(crate) struct Inputs {
    pub(crate) grow: GrowInput,
    pub(crate) plan: PlanInput,
    pub(crate) surface: SurfaceInput,
    pub(crate) leaves: LeafInput,
}

/// The skeleton: the solve, and the canopy's frond crown and shed bases.
#[derive(Debug, Clone)]
pub(crate) struct GrowInput {
    pub(crate) skeleton: SkeletonParams,
    pub(crate) radii: RadiusParams,
    pub(crate) canopy: CanopyParams,
}

/// The element, the leaf plan and the box leaves are quantised against.
#[derive(Debug, Clone)]
pub(crate) struct PlanInput {
    pub(crate) element: ElementParams,
    pub(crate) envelope: Envelope,
    pub(crate) canopy: CanopyParams,
    pub(crate) surface: SurfaceParams,
    pub(crate) radii: RadiusParams,
    pub(crate) seed: u32,
    /// The twig rows resolved, once.
    pub(crate) twigs: Result<TwigParams>,
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
        Self {
            grow: GrowInput {
                skeleton: skeleton.clone(),
                radii: family.radii,
                canopy: family.canopy,
            },
            plan: PlanInput {
                element: family.element,
                envelope,
                canopy: family.canopy,
                surface: family.surface,
                radii: family.radii,
                seed: skeleton.seed,
                twigs: skeleton.twigs.resolved(),
            },
            surface: SurfaceInput {
                height: envelope.height,
                params: family.surface,
            },
            leaves: LeafInput {
                envelope,
                canopy: family.canopy,
                seed: skeleton.seed,
                shell_depth: family.shell_depth,
            },
        }
    }
}

impl PlanInput {
    /// Where the family's twig rows put a leaf's stations.
    pub(crate) fn twig(&self) -> Result<TwigPlacement> {
        let twig = self.twigs.clone()?.twig;
        Ok(TwigPlacement {
            internode_length: twig.internode_length,
            stations_per_internode: twig.stations_per_internode,
        })
    }

    /// The box every leaf of this family is quantised against.
    pub(crate) fn reference(&self) -> Result<Reference> {
        let twigs = self.twigs.clone()?;
        Ok(Reference::from_params(
            self.envelope,
            &twigs,
            self.radii,
            &self.surface,
            self.canopy,
        ))
    }
}
