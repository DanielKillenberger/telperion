//! The data contracts the stages hand on, public as types: the parameter
//! groups a family is made of, and the artifacts a build returns. A function
//! that builds one is a stage and stays inside the pipeline; the crate's own
//! tests, and only they, reach the whole stage through the same path.

pub mod bias {
    pub use crate::pipeline::bias::{BiasParams, GrowthBias, SupernaturalParams};
}

pub mod branching {
    #[cfg(test)]
    pub use crate::pipeline::branching::*;
    pub use crate::pipeline::branching::{
        ChangeRecord, GrowthOverrides, GrowthReport, HabitParams, LeafBase, PackedNode, PackedRead,
        Run, RunNode, SkeletonParams, Specimen, SpecimenBuffers, SpecimenRead, DEFAULT_STEP,
        MAX_LEAF_BASES,
    };
}

pub mod colonization {
    pub use crate::pipeline::colonization::GrowthConfig;
    #[cfg(test)]
    pub use crate::pipeline::colonization::*;
}

pub mod field {
    pub use crate::pipeline::field::{Field, FieldSnapshot, IndexSnapshot, Occupancy};
}

pub mod foliage {
    #[cfg(feature = "geometry")]
    pub use crate::pipeline::foliage::ShortShoot;
    #[cfg(test)]
    pub use crate::pipeline::foliage::*;
    pub use crate::pipeline::foliage::{
        transform_point, withered, AnatomyGeometry, Bounds, CanopyParams, Element, ElementParams,
        FoliageUnit, Instances, Leaf, Level, Placement, PlacementIdentity, Reference, Rosette,
        TwigPlacement, MAX_FRONDS, MAX_LEAFLETS, MAX_SHORT_SHOOT_LEAVES, SHORT_SHOOT_SPACING,
        WITHERED, WORDS,
    };
    pub mod plan {
        #[cfg(test)]
        pub use crate::pipeline::foliage::plan::*;
        pub use crate::pipeline::foliage::plan::{Descriptor, Plan, Run};
    }
    #[cfg(feature = "geometry")]
    pub mod prepared {
        #[cfg(test)]
        pub use crate::pipeline::foliage::prepared::*;
        pub use crate::pipeline::foliage::prepared::{PreparedStations, StationSegment};
    }
}

#[cfg(all(test, feature = "geometry"))]
pub mod footprint {
    pub use crate::pipeline::footprint::*;
}

pub mod radius {
    pub use crate::pipeline::radius::RadiusParams;
    #[cfg(test)]
    pub use crate::pipeline::radius::*;
}

pub mod surface {
    #[cfg(test)]
    pub use crate::pipeline::surface::*;
    pub use crate::pipeline::surface::{
        Bounds, SurfaceMesh, SurfaceParams, SurfaceRun, WoodExtent,
    };
    #[cfg(feature = "geometry")]
    pub mod compact {
        #[cfg(test)]
        pub use crate::pipeline::surface::compact::*;
        pub use crate::pipeline::surface::compact::{CompactSurface, CompactWithContacts};
    }
    #[cfg(feature = "geometry")]
    pub mod prepared {
        #[cfg(test)]
        pub use crate::pipeline::surface::prepared::*;
        pub use crate::pipeline::surface::prepared::{
            ring_radius, PreparedSurface, PreparedWithContacts, Run,
        };
    }
}

pub mod twigs {
    pub use crate::pipeline::twigs::{
        branch_length, child_radius, TwigAnatomy, TwigParams, MAX_GENERATIONS,
    };
}
