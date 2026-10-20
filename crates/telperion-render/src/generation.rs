//! Optional shared GPU foliage experiment. The existing pure-core build paths
//! remain available.
mod clock;
mod compute;
use clock::Clock;
// fn-256: the one cycle on the device, which grows, settles and draws
// the whole tree, every readback awaited, natively and in a browser.
mod cycle;
mod data;
// The resident build's stages after the cycle's: the folds, sweep,
// stations, rings and placement (fn-212, fn-256 task 3).
mod resident;
#[cfg(target_arch = "wasm32")]
pub(crate) use resident::BROWSER;
pub use resident::{Admitted, GrowingTree};
mod io;
mod positions;
#[cfg(test)]
mod preparation;
#[cfg(test)]
mod profile;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) mod request;
pub(crate) mod sources;
mod submit;
#[cfg(test)]
mod wood;
use crate::{buffer::Held, Gpu, Renderer, Result, Submitted};
use std::sync::Arc;
use telperion_core::{mesh::TreeMesh, surface::Bounds, Family};

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
    /// Unstable diagnostic text; absent when no fallback reason was recorded.
    pub wood_fallback: Option<&'static str>,
    /// Preparation only: excludes adoption, drawing and completed-frame fencing.
    pub total_ms: f64,
    pub base_cpu_bytes: u64,
    pub wood_cpu_bytes: u64,
    /// GPU output capacity retained by this result, excluding the previous tree.
    pub retained_gpu_bytes: u64,
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
    /// The flowers already on the device, which the submit takes in place
    /// of `mesh.organ`'s florets (fn-229).
    organ: Option<Resident>,
    /// The wood's curve already on the device, which the submit takes in
    /// place of `mesh.curve` (fn-212 task 7).
    wood: Option<crate::curve::CurveGpu>,
    pub metrics: Metrics,
    /// How far the tree's falling leaves have coloured (fn-270 R5c), which
    /// the submit draws them by: whole for a tree built at once.
    pub colouring: f64,
}
impl Prepared {
    pub fn count(&self) -> usize {
        self.resident
            .as_ref()
            .map_or(self.mesh.foliage_instances(), |r| r.count as usize)
    }
    pub fn bounds(&self) -> Bounds {
        self.mesh.bounds
    }
    /// The wood's curve, which the renderer surfaces.
    pub fn curve(&self) -> &telperion_core::surface::Curve {
        &self.mesh.curve
    }
}
/// Device/pipeline ownership survives independently of the renderer. Creating
/// this once warms pipelines, never generated specimens.
pub struct Generator {
    gpu: Gpu,
    identity: Arc<()>,
    place: io::Pass,
    mass: io::Pass,
    positions: io::Pass,
    /// The device's kernels, made on the first tree built resident.
    resident: std::cell::OnceCell<resident::Kit>,
    /// A test's device error, raised inside the next device build.
    #[cfg(test)]
    fail_next: std::cell::Cell<bool>,
    /// A test's device budget, held to as a browser holds to its own.
    #[cfg(test)]
    budget: std::cell::Cell<Option<u64>>,
}
impl Generator {
    /// Live tree buffers only; excludes textures, pipelines and driver allocations.
    pub fn tree_buffer_bytes(renderer: &Renderer) -> u64 {
        renderer.wood.allocated_bytes()
            + renderer.foliage.allocated_bytes()
            + renderer.organ.allocated_bytes()
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
        async move { Self::create(gpu, identity).await }
    }
    /// A generator with no renderer, for fields grown on the device
    /// (fn-257): a worker's or a test's.
    pub async fn for_fields_async(gpu: Gpu) -> Result<Self> {
        Self::create(gpu, Arc::new(())).await
    }
    async fn create(gpu: Gpu, identity: Arc<()>) -> Result<Self> {
        let scopes = io::scope(&gpu);
        let place = io::Pass::new(
            &gpu,
            sources::place(),
            &[true, true, true, false, false],
            &["place", "bloom"],
        );
        let mass = io::Pass::new(
            &gpu,
            sources::mass(),
            &[true, false, false],
            &["occupancy", "depth"],
        );
        let positions = io::Pass::new(
            &gpu,
            sources::positions(),
            &[true, true, false, false, false, false],
            &positions::entries(),
        );
        io::programs_ready(&gpu, scopes).await?;
        Ok(Self {
            gpu,
            identity,
            place,
            mass,
            positions,
            resident: std::cell::OnceCell::new(),
            #[cfg(test)]
            fail_next: Default::default(),
            #[cfg(test)]
            budget: Default::default(),
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn prepare(&self, family: &Family) -> Result<Prepared> {
        pollster::block_on(self.prepare_async(family))
    }
    /// `family` grown, drawn and placed on the device whole (fn-256 R1).
    pub async fn prepare_async(&self, family: &Family) -> Result<Prepared> {
        self.prepare_resident(family).await
    }
}

#[cfg(test)]
mod build_time_tests;
#[cfg(test)]
mod tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
mod unfold_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod cpu_builds_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
mod ledger_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
mod lobed_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
mod program_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
mod readback_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
mod release_tests;

#[cfg(test)]
use crate::whole_tree::whole_tree;

/// Match the integration tests: unavailable hardware is explicit, while a
/// device that exists but fails for another reason still fails the test.
#[cfg(test)]
fn test_gpu() -> Option<crate::Gpu> {
    match pollster::block_on(crate::Gpu::request(None)) {
        Ok(gpu) => Some(gpu),
        Err(
            error @ (crate::RenderError::WebGpuUnavailable(_)
            | crate::RenderError::FallbackOnly { .. }),
        ) => {
            println!("skipped: {error}");
            None
        }
        Err(error) => panic!("the device was there and still refused: {error}"),
    }
}
