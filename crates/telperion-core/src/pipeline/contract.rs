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
        GrowthOverrides, HabitParams, LeafBase, SkeletonParams, DEFAULT_STEP, MAX_LEAF_BASES,
    };
}

pub mod colonization {
    pub use crate::pipeline::colonization::GrowthConfig;
    #[cfg(test)]
    pub use crate::pipeline::colonization::*;
}

pub mod field {
    #[cfg(all(test, feature = "geometry"))]
    pub use crate::pipeline::field::{bounds, read};
    pub use crate::pipeline::field::{florets_into, seat_bounds, seats, Frame, Grid, Occupancy};
}

pub mod foliage {
    #[cfg(test)]
    pub use crate::pipeline::foliage::*;
    pub use crate::pipeline::foliage::{
        blade, transform_point, withered, AnatomyGeometry, BloomParams, Bounds, CanopyParams,
        Crown, Element, ElementParams, FoliageUnit, Instances, Leaf, Level, Reference, Rosette,
        TwigPlacement, Wedge, MAX_FRONDS, MAX_LEAFLETS, WITHERED, WORDS,
    };
    #[cfg(feature = "geometry")]
    pub use crate::pipeline::foliage::{bloom_box, rachis_low};
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
    #[cfg(all(test, feature = "geometry"))]
    pub use crate::pipeline::surface::*;
    pub use crate::pipeline::surface::{
        Bounds, FlatWood, SurfaceChunk, SurfaceMesh, SurfaceParams, SurfaceRun, WoodExtent,
        WoodRun, CHUNK_VERTICES,
    };
    #[cfg(feature = "geometry")]
    pub use crate::pipeline::surface::{
        Budget, Curve, CurveCluster, CurvePoint, CurveRun, Demand, PackedPoint, Tessellation,
        Viewer, CLUSTER, CLUSTER_WORDS, LEVELS, POINT_WORDS, RIBBON, SCALES, SECTION_FLOATS,
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
    #[cfg(test)]
    pub use crate::pipeline::twigs::*;
    pub use crate::pipeline::twigs::{TwigAnatomy, TwigParams};
}
