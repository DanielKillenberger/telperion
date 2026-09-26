//! Independent, closed swept shells over solved tree paths. Buffers are caller-owned.
//!
//! The value table and the buffer types are always built; the builder and
//! its helpers sit behind the `geometry` feature, so a plan-only build of the
//! crate carries no sweep.
use crate::{math::Vec3, Error, Result};
#[cfg(feature = "geometry")]
mod angular;
#[cfg(feature = "geometry")]
mod attachment;
#[cfg(feature = "geometry")]
mod build;
#[cfg(feature = "geometry")]
#[doc(hidden)]
pub mod compact;
#[cfg(feature = "geometry")]
mod dependencies;
#[cfg(feature = "geometry")]
mod frames;
#[cfg(feature = "geometry")]
mod normals;
#[cfg(all(feature = "geometry", target_os = "linux", target_arch = "x86_64"))]
mod parallel;
mod paths;
#[cfg(feature = "geometry")]
pub mod prepared;
#[cfg(feature = "geometry")]
mod rings;
#[cfg(feature = "geometry")]
mod samples;
#[cfg(feature = "geometry")]
mod section;
use crate::catalogue::{bounded, input, value, Blend, Bounds as Rail, Site};
#[cfg(feature = "geometry")]
pub(crate) use attachment::AttachmentSurface;
#[cfg(feature = "geometry")]
use build::*;
#[cfg(feature = "geometry")]
pub use build::{build, extent};
#[cfg(feature = "geometry")]
pub(crate) use build::{faces, Faces};
#[cfg(feature = "geometry")]
pub(crate) use dependencies::affected as affected_contacts;
pub(crate) use paths::straightest;
#[cfg(feature = "geometry")]
pub(crate) use rings::{rings, Rings, Sweep};
#[cfg(feature = "geometry")]
use rings::{Resweep, Swept};
#[cfg(feature = "geometry")]
use {
    crate::{math::Transcendental, tree::Tree},
    frames::frames,
    paths::paths,
    samples::sample_path,
};
fn reserved<T>(n: usize) -> Result<Vec<T>> {
    let mut out = Vec::new();
    out.try_reserve_exact(n)
        .map_err(|_| Error::ResourceLimit("surface allocation"))?;
    Ok(out)
}
fn filled<T: Clone>(n: usize, value: T) -> Result<Vec<T>> {
    let mut out = reserved(n)?;
    out.resize(n, value);
    Ok(out)
}
/// One complete surface run, in descending order of its largest sample radius.
/// The spans tile the wood index buffer; a caster can draw a single prefix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceRun {
    pub first_index: u32,
    pub index_count: u32,
    pub largest_radius: f64,
}

/// The elements a tree's wood buffers hold once it is swept. `build` reserves
/// by it and a prediction sizes the specimen by it, so the ring arithmetic is
/// written once and read twice.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WoodExtent {
    /// Floats in `positions`, and the same count again in `normals`.
    pub positions: usize,
    /// Floats in `coords`: two a vertex.
    pub coords: usize,
    /// Indices in `indices`.
    pub indices: usize,
}
impl WoodExtent {
    /// Bytes the four buffers keep, each count times the size of the type
    /// that holds it. `positions` is counted twice: `normals` is its equal.
    pub fn bytes(&self) -> usize {
        (self.positions * 2 + self.coords) * size_of::<f32>() + self.indices * size_of::<u32>()
    }
}

crate::catalogue::rows! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct SurfaceParams in "/surface" {
        /// How many sides each piece of wood is drawn with. Raising it makes
        /// the wood rounder and smoother, and costs triangles.
        pub radial_segments: u32 = "radialSegments" "sides" Rail::closed(3.0, 64.0) => [Expand] {
            wire: 85,
            check: input(Site::Surface, 1, "surface parameters"),
            note: "Wood is drawn with `max(radialSegments, 4·lobes)` sides.",
            blend: Blend::Count,
            dial: bounded("radial_segments", "how many sides each piece of wood is drawn with",
                [2.0, 4.0]).span([6.0, 18.0]),
        },
        /// How many ridges run up around the trunk. Raising it gives the
        /// bark more flutes; zero is a plain round bole.
        pub lobes: u32 = "lobes" "lobes" Rail::closed(0.0, 16.0) => [Expand] {
            wire: 86,
            check: input(Site::Surface, 2, "surface parameters"),
            note: "Raises the side count even at `lobeDepth` zero; the GPU executor draws only \
                unlobed wood itself.",
            blend: Blend::Count,
            dial: bounded("surface_lobes", "how many ridges run up around the trunk", [2.0, 4.0]),
        },
        /// How deep the flutes between those ridges cut, as a share of the
        /// wood's own radius. Raising it makes the fluting more pronounced. At
        /// zero the bole is plainly round whatever the ridge count says, and any
        /// rise starts cutting the flutes.
        pub lobe_depth: f64 = "lobeDepth" "share of radius"
            Rail::closed(0.0, 0.9) => [Plan, Expand] {
            wire: 87,
            check: input(Site::Surface, 3, "surface parameters"),
            applies: "`lobes` zero",
            note: "With `surfaceContact` above zero, leaf seating and the leaf box scale by `1 + \
                lobeDepth`; the socket's inscribed radius shrinks by it.",
            dial: bounded("surface_lobe_depth", "how deep the flutes between those ridges cut, \
                as a share of the radius; at zero the bole is plainly round whatever the ridge \
                count says, and any rise starts cutting the flutes", [0.15, 0.3]),
        },
        /// How many turns those ridges make over the tree's height. Raising
        /// it winds them more tightly around the trunk.
        pub twist_rate: f64 = "twistRate" "turns over the height"
            Rail::closed(-64.0, 64.0) => [Expand] {
            wire: 88,
            check: input(Site::Surface, 4, "surface parameters"),
            applies: "unless `lobes` and `lobeDepth` are both above zero",
            dial: bounded("twist_rate", "how many turns those ridges make over the tree's height",
                [1.0, 2.0]).span([-1.2, 3.6]),
        },
        /// How much wider the trunk is where it meets the ground, as a
        /// multiple of its own radius. Raising it gives a broader buttress.
        pub flare_radius: f64 = "flareRadius" "multiple of radius"
            Rail::closed(1.0, 8.0) => [Plan, Expand] {
            wire: 89,
            check: input(Site::Surface, 5, "surface parameters"),
            note: "Applies by height, to branches near the ground too; seats leaves where \
                surface contact is on.",
            dial: bounded("flare_radius", "how much wider the trunk is at the ground, as a \
                multiple of its radius", [0.2, 0.4]).span([1.2, 2.4]),
        },
        /// How far up the trunk that flare reaches, as a share of the
        /// height. Raising it carries the swelling further up the bole.
        pub flare_falloff: f64 = "flareFalloff" "share of height"
            Rail::closed(1e-4, 1.0) => [Expand] {
            wire: 90,
            check: input(Site::Surface, 6, "surface parameters"),
            applies: "`flareRadius` one",
            dial: bounded("flare_falloff", "how far up the bole that flare reaches, as a share \
                of the height", [0.15, 0.3]),
        },
        /// How deep the trunk's base is sunk below the ground, as a share
        /// of the height. Raising it buries more of the flare.
        pub flare_depth: f64 = "flareDepth" "share of height" Rail::closed(0.0, 1.0) => [Expand] {
            wire: 91,
            check: input(Site::Surface, 7, "surface parameters"),
            note: "Buries one ring per trunk run even with no flare.",
            dial: bounded("flare_depth", "how deep the trunk's base is sunk below the ground, as \
                a share of the height", [0.15, 0.3]),
        },
        /// How deeply a child branch is set into its parent at a fork.
        /// Raising it sinks the junction further in, so the two read as one
        /// piece of wood rather than two tubes meeting.
        pub fork_socket: f64 = "forkSocket" "share" Rail::closed(0.0, 0.9) => [Expand] {
            wire: 92,
            check: input(Site::Surface, 8, "surface parameters"),
            dial: bounded("fork_socket", "how deeply a child branch is set into its parent at a \
                fork", [0.15, 0.3]),
        },
        /// Fraction of the parent's inscribed radius available for a socket.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_socket_containment"))]
        pub socket_containment: f64 = "socketContainment" "share"
            Rail::closed(0.0, 1.0) => [Expand] {
            wire: 93,
            check: value(Site::Surface, 0, "socketContainment"),
            applies: "`forkSocket` zero",
            dial: bounded("socket_containment", "the share of the parent's inscribed radius a \
                socket may use", [0.15, 0.3]),
        },
        /// How much wood thickens at a fork. Raising it leaves a more
        /// pronounced collar where a branch leaves its parent.
        pub fork_swell: f64 = "forkSwell" "multiple of radius"
            Rail::closed(1.0, 4.0) => [Plan, Expand] {
            wire: 94,
            check: input(Site::Surface, 9, "surface parameters"),
            note: "Seats leaves and sizes the leaf box where surface contact is on.",
            dial: bounded("fork_swell", "how much wood thickens at a fork", [0.5, 1.0]),
        },
    }
}
impl Default for SurfaceParams {
    fn default() -> Self {
        Self {
            radial_segments: 12,
            lobes: 5,
            lobe_depth: 0.16,
            twist_rate: 1.5,
            flare_radius: 2.1,
            flare_falloff: 0.022,
            flare_depth: 0.004,
            fork_socket: 0.5,
            socket_containment: crate::ranges::default_socket_containment(),
            fork_swell: 1.35,
        }
    }
}
/// The height a surface is swept against: a tree with no height has no wood.
pub(crate) fn height(height: f64) -> Result<()> {
    if !height.is_finite() || height <= 0.0 {
        return Err(Error::InvalidInput("surface height"));
    }
    Ok(())
}
impl SurfaceParams {
    pub fn validate(&self) -> Result<()> {
        crate::catalogue::check(Self::CHECKS, self, Site::Surface)
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub min: Vec3,
    pub max: Vec3,
}
#[derive(Debug, Default, PartialEq)]
pub struct SurfaceMesh {
    pub positions: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,
    /// Two floats per vertex: metres along the branch from the root, and the
    /// angle around it in radians. Bark is drawn along them; no geometry here
    /// reads them back.
    pub coords: Vec<f32>,
    pub bounds: Option<Bounds>,
    pub runs: usize,
    pub run_table: Vec<SurfaceRun>,
    /// Triangles dropped because their float32 corners span no area; each
    /// run's span in the index buffer already leaves them out.
    pub dropped: usize,
}
#[cfg(all(test, feature = "geometry"))]
mod fork_tests;

#[cfg(test)]
mod tests {
    #[test]
    fn allocation_failure_is_explicit() {
        assert!(matches!(
            super::reserved::<f32>(usize::MAX),
            Err(crate::Error::ResourceLimit(_))
        ));
    }
}
