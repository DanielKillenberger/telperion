//! Optional shared GPU foliage experiment. The existing pure-core build paths
//! remain available; every request explicitly states its delivery mode.
mod clock;
mod compute;
use clock::Clock;
mod data;
mod io;
mod positions;
mod preparation;
mod submit;
mod wood;
use wood::ResidentWood;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) mod request;
use crate::{buffer::Held, Gpu, Renderer, Result, Submitted};
use std::sync::Arc;
use telperion_core::{
    branching,
    foliage::{self, Instances, Reference, TwigPlacement},
    mesh::{self, TreeMesh},
    surface::{self, Bounds},
    Family,
};

/// Requested geometry ownership, independent of the compute backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    /// Keep supported output on the renderer's device; fallback may use CPU geometry.
    Resident,
    /// Return owned CPU geometry, reading GPU-generated foliage back when applicable.
    Cpu,
}
/// Backend for the reported stage: [`Prepared::backend`] describes foliage;
/// [`Metrics::wood_backend`] reports wood independently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// CPU execution requested by the delivery path.
    Cpu,
    /// GPU execution, including output subsequently read back to CPU ownership.
    Gpu,
    /// Canonical CPU execution because acceleration was unsupported or rejected.
    CpuFallback,
}
/// Diagnostics for one preparation request. `_ms` fields are host wall-clock
/// milliseconds, not GPU timestamps; `_bytes` fields count selected allocation
/// capacities in bytes. Stage timings can nest or overlap and must not be summed.
/// Buffer snapshots and phase peaks also overlap: use ownership-aware phase maxima,
/// not their sum. They exclude previous renderer trees, some transient scratch,
/// allocator-retained Wasm capacity, upload staging and driver/deferred allocations.
/// Zero/default fields may mean the stage was not used, rather than free execution.
#[derive(Debug, Default)]
pub struct Metrics {
    pub position_prepare_ms: f64,
    pub position_upload_ms: f64,
    /// Remaining admission wait after any overlapping CPU station preparation.
    pub position_wait_ms: f64,
    pub position_cpu_bytes: u64,
    pub position_gpu_peak_bytes: u64,
    pub position_retained_metadata_bytes: u64,
    /// Unstable diagnostic text, not an error code or a string to branch on.
    pub position_fallback: Option<&'static str>,
    /// Whether admitted GPU positions are used by this result.
    pub gpu_positions: bool,
    pub skeleton_ms: f64,
    /// CPU descriptor work, including station preparation; excludes early wood work.
    pub descriptors_ms: f64,
    pub upload_dispatch_ms: f64,
    pub placement_wait_ms: f64,
    pub compact_ms: f64,
    pub mass_ms: f64,
    pub readback_ms: f64,
    /// Wood work including early position preparation; contains wood substage timings.
    pub wood_ms: f64,
    pub wood_prepare_ms: f64,
    pub wood_upload_dispatch_ms: f64,
    pub wood_wait_ms: f64,
    pub wood_prepared_cpu_bytes: u64,
    pub wood_metadata_cpu_bytes: u64,
    pub wood_gpu_peak_bytes: u64,
    pub wood_backend: Option<Backend>,
    /// Unstable diagnostic text; absent when no fallback reason was recorded.
    pub wood_fallback: Option<&'static str>,
    /// Preparation only: excludes adoption, drawing and completed-frame fencing.
    pub total_ms: f64,
    pub base_cpu_bytes: u64,
    pub wood_cpu_bytes: u64,
    /// GPU output capacity retained by this result, excluding the previous tree.
    pub retained_gpu_bytes: u64,
    pub input_instances: u32,
    pub instances: u32,
    pub descriptor_cpu_bytes: u64,
    pub shared_contact_cpu_bytes: u64,
    /// Maximum recorded shared CPU phase snapshot, not a complete process-memory peak.
    pub shared_prepare_cpu_bytes: u64,
    pub shared_metadata_cpu_bytes: u64,
    pub gpu_compute_peak_bytes: u64,
}
struct Resident {
    leaves: Held,
    masses: Held,
    count: u32,
    crown: Option<Bounds>,
    full: Option<Bounds>,
}
/// Owns all output, with no live renderer borrow. Its private identity prevents
/// committing buffers through another renderer, even on the same device.
pub struct Prepared {
    identity: Arc<()>,
    mesh: TreeMesh,
    resident: Option<Resident>,
    wood: Option<ResidentWood>,
    pub backend: Backend,
    pub metrics: Metrics,
}
impl Prepared {
    pub fn count(&self) -> usize {
        self.resident
            .as_ref()
            .map_or(self.mesh.foliage_instances(), |r| r.count as usize)
    }
    pub fn wood_vertices(&self) -> usize {
        self.wood
            .as_ref()
            .map_or(self.mesh.wood_vertices(), |w| w.vertices)
    }
    pub fn wood_triangles(&self) -> usize {
        self.wood
            .as_ref()
            .map_or(self.mesh.wood_triangles(), |w| w.index_count as usize / 3)
    }
    pub fn bounds(&self) -> Bounds {
        self.mesh.bounds
    }
    pub fn cpu_mesh(&self) -> Option<&TreeMesh> {
        self.resident.is_none().then_some(&self.mesh)
    }
}
/// Device/pipeline ownership survives independently of the renderer. Creating
/// this once warms pipelines, never generated specimens.
pub struct Generator {
    resident_allowed: bool,
    gpu: Gpu,
    identity: Arc<()>,
    place: io::Pass,
    compact: io::Pass,
    mass: io::Pass,
    wood: Option<io::Pass>,
    positions: Option<io::Pass>,
}
impl Generator {
    /// Live tree buffers only; excludes textures, pipelines and driver allocations.
    pub fn tree_buffer_bytes(renderer: &Renderer) -> u64 {
        renderer.wood.allocated_bytes() + renderer.foliage.allocated_bytes()
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(renderer: &Renderer) -> Result<Self> {
        pollster::block_on(Self::new_async(renderer))
    }
    /// Experimental constructor whose future owns its device context.
    pub fn new_async(
        renderer: &Renderer,
    ) -> impl std::future::Future<Output = Result<Self>> + 'static {
        let gpu = renderer.gpu.clone();
        let identity = renderer.identity.clone();
        async move { Self::create(gpu, identity, true).await }
    }
    /// Experimental owned CPU output without renderer resources.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn for_cpu_output(gpu: Gpu) -> Result<Self> {
        pollster::block_on(Self::for_cpu_output_async(gpu))
    }
    pub async fn for_cpu_output_async(gpu: Gpu) -> Result<Self> {
        Self::create(gpu, Arc::new(()), false).await
    }
    async fn create(gpu: Gpu, identity: Arc<()>, resident_allowed: bool) -> Result<Self> {
        let scopes = io::scope(&gpu);
        let leaf = include_str!("shaders/leaf.wgsl");
        let place = io::Pass::new(
            &gpu,
            format!(
                "{leaf}\n{}\n{}\n{}\n{}",
                include_str!("generation/order.wgsl"),
                include_str!("generation/place.wgsl"),
                include_str!("generation/contact.wgsl"),
                include_str!("generation/pack.wgsl")
            ),
            &[true, true, true, false, false, false],
            &["place", "prefix"],
        );
        let compact = io::Pass::new(
            &gpu,
            include_str!("generation/scatter.wgsl").into(),
            &[true, true, true, false],
            &["scatter"],
        );
        let mass = io::Pass::new(
            &gpu,
            format!("{leaf}\n{}", include_str!("generation/mass.wgsl")),
            &[true, false, false],
            &["occupancy", "depth"],
        );
        let wood = resident_allowed.then(|| {
            io::Pass::new(
                &gpu,
                format!(
                    "{}\n{}",
                    include_str!("generation/wood.wgsl"),
                    include_str!("generation/wood_geometry.wgsl")
                ),
                &[true, true, false, false, false, false, false],
                &["expand"],
            )
        });
        let positions = resident_allowed.then(|| {
            io::Pass::new(
                &gpu,
                format!(
                    "{}\n{}",
                    include_str!("generation/positions.wgsl"),
                    include_str!("generation/wood_geometry.wgsl")
                ),
                &[true, true, false, false, false, false],
                &["emit", "admit", "radii", "finish"],
            )
        });
        io::errors(&gpu, scopes).await?;
        Ok(Self {
            gpu,
            identity,
            resident_allowed,
            place,
            compact,
            mass,
            wood,
            positions,
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn prepare(&self, family: &Family, delivery: Delivery) -> Result<Prepared> {
        pollster::block_on(self.prepare_async(family, delivery))
    }
    fn empty(&self) -> Result<Resident> {
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        Ok(Resident {
            leaves: Held::resident(
                io::buffer(&self.gpu, "empty generated foliage", 16, usage, &[])?,
                0,
                "empty generated foliage",
            ),
            masses: Held::resident(
                io::buffer(&self.gpu, "empty generated mass", 32, usage, &[0; 32])?,
                32,
                "empty generated mass",
            ),
            count: 0,
            crown: None,
            full: None,
        })
    }
    /// Explicit verification readback; callers keep it outside resident delivery timing.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_instances(&self, prepared: &Prepared) -> Result<Instances> {
        pollster::block_on(self.read_instances_async(prepared))
    }
    pub async fn read_instances_async(&self, prepared: &Prepared) -> Result<Instances> {
        if !Arc::ptr_eq(&self.identity, &prepared.identity) {
            return Err(telperion_core::Error::InvalidInput("generation renderer mismatch").into());
        }
        if let Some(r) = &prepared.resident {
            let mut out = Instances::new(prepared.mesh.foliage.instances.reference);
            out.leaves = io::read_leaves_async(&self.gpu, r.leaves.buffer(), r.count).await?;
            Ok(out)
        } else {
            Ok(prepared.mesh.foliage.instances.clone())
        }
    }
}

fn union(a: Option<Bounds>, b: Option<Bounds>) -> Option<Bounds> {
    match (a, b) {
        (Some(a), Some(b)) => Some(Bounds {
            min: telperion_core::math::Vec3::new(
                a.min.x.min(b.min.x),
                a.min.y.min(b.min.y),
                a.min.z.min(b.min.z),
            ),
            max: telperion_core::math::Vec3::new(
                a.max.x.max(b.max.x),
                a.max.y.max(b.max.y),
                a.max.z.max(b.max.z),
            ),
        }),
        (a, b) => a.or(b),
    }
}
#[cfg(test)]
mod standalone_tests;
#[cfg(test)]
mod tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod readback_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod wood_tests;
