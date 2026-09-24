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
mod samples;
#[cfg(feature = "geometry")]
mod section;
#[cfg(feature = "geometry")]
pub(crate) use attachment::AttachmentSurface;
#[cfg(feature = "geometry")]
use build::*;
#[cfg(feature = "geometry")]
pub use build::{build, extent};
#[cfg(feature = "geometry")]
pub(crate) use dependencies::affected as affected_contacts;
pub(crate) use paths::straightest;
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

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct SurfaceParams {
    /// How many sides each piece of wood is drawn with. Raising it makes
    /// the wood rounder and smoother, and costs triangles.
    pub radial_segments: u32,
    /// How many ridges run up around the trunk. Raising it gives the
    /// bark more flutes; zero is a plain round bole.
    pub lobes: u32,
    /// How deep the flutes between those ridges cut, as a share of the
    /// wood's own radius. Raising it makes the fluting more pronounced. At
    /// zero the bole is plainly round whatever the ridge count says, and any
    /// rise starts cutting the flutes.
    pub lobe_depth: f64,
    /// How many turns those ridges make over the tree's height. Raising
    /// it winds them more tightly around the trunk.
    pub twist_rate: f64,
    /// How much wider the trunk is where it meets the ground, as a
    /// multiple of its own radius. Raising it gives a broader buttress.
    pub flare_radius: f64,
    /// How far up the trunk that flare reaches, as a share of the
    /// height. Raising it carries the swelling further up the bole.
    pub flare_falloff: f64,
    /// How deep the trunk's base is sunk below the ground, as a share
    /// of the height. Raising it buries more of the flare.
    pub flare_depth: f64,
    /// How deeply a child branch is set into its parent at a fork.
    /// Raising it sinks the junction further in, so the two read as one
    /// piece of wood rather than two tubes meeting.
    pub fork_socket: f64,
    /// Fraction of the parent's inscribed radius available for a socket.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_socket_containment")
    )]
    pub socket_containment: f64,
    /// How much wood thickens at a fork. Raising it leaves a more
    /// pronounced collar where a branch leaves its parent.
    pub fork_swell: f64,
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
        use crate::ranges;
        ranges::UNIT.check(self.socket_containment, "socketContainment")?;
        if ![
            (self.radial_segments as f64, ranges::SURFACE_RADIAL_SEGMENTS),
            (self.lobes as f64, ranges::SURFACE_LOBES),
            (self.lobe_depth, ranges::SURFACE_LOBE_DEPTH),
            (self.twist_rate, ranges::SURFACE_TWIST_RATE),
            (self.flare_radius, ranges::SURFACE_FLARE_RADIUS),
            (self.flare_falloff, ranges::SURFACE_FLARE_FALLOFF),
            (self.flare_depth, ranges::UNIT),
            (self.fork_socket, ranges::SURFACE_FORK_SOCKET),
            (self.fork_swell, ranges::SURFACE_FORK_SWELL),
        ]
        .iter()
        .all(|&(v, range)| v.is_finite() && v >= range.0 && v <= range.1)
        {
            return Err(Error::InvalidInput("surface parameters"));
        }
        Ok(())
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
