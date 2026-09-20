//! Optional shared GPU foliage experiment. The existing pure-core build paths
//! remain available; every request explicitly states its delivery mode.
mod clock;
mod compute;
use clock::Clock;
mod data;
mod io;
mod positions;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    Resident,
    Cpu,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Cpu,
    Gpu,
    CpuFallback,
}
#[derive(Debug, Default)]
pub struct Metrics {
    pub position_prepare_ms: f64,
    pub position_upload_ms: f64,
    pub position_wait_ms: f64,
    pub position_cpu_bytes: u64,
    pub position_gpu_peak_bytes: u64,
    pub position_retained_metadata_bytes: u64,
    pub position_fallback: Option<&'static str>,
    pub gpu_positions: bool,
    pub skeleton_ms: f64,
    pub descriptors_ms: f64,
    pub upload_dispatch_ms: f64,
    pub placement_wait_ms: f64,
    pub compact_ms: f64,
    pub mass_ms: f64,
    pub readback_ms: f64,
    pub wood_ms: f64,
    pub wood_prepare_ms: f64,
    pub wood_upload_dispatch_ms: f64,
    pub wood_wait_ms: f64,
    pub wood_prepared_cpu_bytes: u64,
    pub wood_metadata_cpu_bytes: u64,
    pub wood_gpu_peak_bytes: u64,
    pub wood_backend: Option<Backend>,
    pub wood_fallback: Option<&'static str>,
    pub total_ms: f64,
    pub base_cpu_bytes: u64,
    pub wood_cpu_bytes: u64,
    pub retained_gpu_bytes: u64,
    pub input_instances: u32,
    pub instances: u32,
    pub descriptor_cpu_bytes: u64,
    pub shared_contact_cpu_bytes: u64,
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
    pub async fn prepare_async(&self, family: &Family, delivery: Delivery) -> Result<Prepared> {
        if delivery == Delivery::Resident && !self.resident_allowed {
            return Err(telperion_core::Error::InvalidInput(
                "standalone generation requires CPU delivery",
            )
            .into());
        }
        let total = Clock::now();
        let mut metrics = Metrics::default();
        let started = Clock::now();
        let tree = branching::generate(&family.skeleton, family.radii)?.tree;
        metrics.skeleton_ms = started.elapsed_ms();
        let started = Clock::now();
        let element = foliage::build_element(family.element)?;
        element.validate()?;
        let twig = family.skeleton.twigs.resolved()?.twig;
        let twig = TwigPlacement {
            internode_length: twig.internode_length,
            stations_per_internode: twig.stations_per_internode,
        };
        if !family.shell_depth.is_finite() || !(0.0..=1.0).contains(&family.shell_depth) {
            return Err(telperion_core::Error::InvalidInput("shell depth").into());
        }
        let reference = Reference::of(family)?;
        let mut uploaded_wood = None;
        let mut shared_stations = None;
        let mut wood_attempted = false;
        let mut early_wood_ms = 0.0;
        let mut station_unsupported = false;
        if delivery == Delivery::Resident
            && foliage::prepared::supports_stations(family.canopy, Some(twig))
            && (family.surface.lobes == 0 || family.surface.lobe_depth == 0.0)
        {
            let begin = Clock::now();
            let needs_contacts = family.canopy.surface_contact > 0.0 && family.canopy.size != 0.0;
            let mut candidate_stations = None;
            let mut station_ms = 0.0;
            let mut station_bytes = 0;
            if !needs_contacts {
                let station_start = Clock::now();
                candidate_stations = foliage::prepared::prepare_stations(
                    &tree,
                    family.skeleton.envelope,
                    family.canopy,
                    Some(twig),
                    &family.surface,
                )?
                .map(|p| (p.segments, p.count, p.ring_size));
                station_ms = station_start.elapsed_ms();
                station_unsupported = candidate_stations.is_none();
                station_bytes = candidate_stations.as_ref().map_or(0, |(s, _, _)| {
                    (s.capacity() * size_of::<foliage::prepared::StationSegment>()) as u64
                });
            }
            let compact = if needs_contacts {
                surface::compact::prepare_with_contacts(
                    &tree,
                    family.skeleton.envelope.height,
                    &family.surface,
                )
                .and_then(|shared| {
                    metrics.shared_contact_cpu_bytes = shared.contact_bytes() as u64;
                    if shared.surface().qualified() {
                        let station_start = Clock::now();
                        candidate_stations = foliage::prepared::prepare_compact_stations(
                            &shared,
                            family.skeleton.envelope,
                            family.canopy,
                            Some(twig),
                        )?
                        .map(|p| (p.segments, p.count, p.ring_size));
                        station_ms = station_start.elapsed_ms();
                        station_bytes = candidate_stations.as_ref().map_or(0, |(s, _, _)| {
                            (s.capacity() * size_of::<foliage::prepared::StationSegment>()) as u64
                        });
                    }
                    metrics.shared_prepare_cpu_bytes = positions::cpu_bytes(shared.surface())
                        + metrics.shared_contact_cpu_bytes
                        + station_bytes;
                    Ok(shared.into_surface())
                })
            } else {
                surface::compact::prepare(&tree, family.skeleton.envelope.height, &family.surface)
            };
            metrics.position_prepare_ms = begin.elapsed_ms() - station_ms;
            let compact = match compact {
                Ok(p) => Some(p),
                Err(telperion_core::Error::InvalidInput("compact surface float32 overflow")) => {
                    metrics.position_fallback = Some("compact float32 range");
                    None
                }
                Err(error) => return Err(error.into()),
            };
            if let Some(p) = compact {
                if candidate_stations.is_some() || !p.qualified() {
                    let scopes = io::scope(&self.gpu);
                    let result = self.emit_positions(p, &mut metrics).await;
                    let errors = io::errors(&self.gpu, scopes).await;
                    uploaded_wood = result?;
                    errors?;
                    metrics.shared_prepare_cpu_bytes = metrics
                        .shared_prepare_cpu_bytes
                        .max(metrics.position_cpu_bytes + station_bytes);
                    if let Some(wood) = &uploaded_wood {
                        shared_stations = candidate_stations;
                        metrics.shared_metadata_cpu_bytes = wood.metadata_bytes();
                        wood_attempted = true;
                    }
                } else {
                    metrics.position_fallback = Some("station capability");
                }
            }
            early_wood_ms = begin.elapsed_ms() - station_ms;
        } else if delivery == Delivery::Resident {
            metrics.position_fallback = Some("profile or station capability");
        }
        if uploaded_wood.is_none()
            && delivery == Delivery::Resident
            && family.canopy.surface_contact > 0.0
            && family.canopy.size != 0.0
            && foliage::prepared::supports_stations(family.canopy, Some(twig))
        {
            let prepare_start = Clock::now();
            let shared = surface::prepared::prepare_with_contacts(
                &tree,
                family.skeleton.envelope.height,
                &family.surface,
            )?;
            metrics.wood_prepare_ms = prepare_start.elapsed_ms();
            early_wood_ms += metrics.wood_prepare_ms;
            if let Some(shared) = shared {
                metrics.shared_contact_cpu_bytes = shared.contact_bytes() as u64;
                if let Some(p) = foliage::prepared::prepare_shared_stations(
                    &shared,
                    family.skeleton.envelope,
                    family.canopy,
                    Some(twig),
                )? {
                    let station_cpu_bytes = (p.segments.capacity()
                        * size_of::<foliage::prepared::StationSegment>())
                        as u64;
                    metrics.shared_prepare_cpu_bytes = metrics.shared_prepare_cpu_bytes.max(
                        wood::prepared_cpu_bytes(shared.surface())
                            + metrics.shared_contact_cpu_bytes
                            + station_cpu_bytes,
                    );
                    let station_data = (p.segments, p.count, p.ring_size);
                    let upload_start = Clock::now();
                    let scopes = io::scope(&self.gpu);
                    let result = self.upload_wood(shared.into_surface(), &mut metrics);
                    let errors = io::errors(&self.gpu, scopes).await;
                    uploaded_wood = result?;
                    errors?;
                    early_wood_ms += upload_start.elapsed_ms();
                    metrics.shared_prepare_cpu_bytes = metrics.shared_prepare_cpu_bytes.max(
                        metrics.wood_prepared_cpu_bytes
                            + metrics.wood_metadata_cpu_bytes
                            + station_cpu_bytes,
                    );
                    wood_attempted = true;
                    if let Some(wood) = &uploaded_wood {
                        metrics.shared_metadata_cpu_bytes = wood.metadata_bytes();
                        shared_stations = Some(station_data);
                    }
                }
            } else {
                wood_attempted = true;
                metrics.wood_fallback = Some("CPU triangle admission");
            }
        }
        let stations = if shared_stations.is_some() || station_unsupported {
            None
        } else {
            foliage::prepared::prepare_stations(
                &tree,
                family.skeleton.envelope,
                family.canopy,
                Some(twig),
                &family.surface,
            )?
        };
        if stations.is_none() && shared_stations.is_none() {
            drop(uploaded_wood);
            metrics.gpu_positions = false;
            metrics.position_retained_metadata_bytes = 0;
            let mesh = mesh::assemble(&tree, family)?;
            metrics.wood_backend = Some(Backend::CpuFallback);
            metrics.wood_fallback = Some("CPU foliage preparation fallback");
            metrics.wood_cpu_bytes = wood::cpu_bytes(&mesh.wood);
            metrics.instances = mesh.foliage_instances() as u32;
            metrics.total_ms = total.elapsed_ms();
            return Ok(Prepared {
                identity: self.identity.clone(),
                mesh,
                resident: None,
                wood: None,
                backend: Backend::CpuFallback,
                metrics,
            });
        };
        metrics.base_cpu_bytes = (tree.nodes.capacity() * size_of::<telperion_core::tree::Node>()
            + element.positions.capacity() * size_of::<telperion_core::math::Vec3>()
            + (element.indices.capacity()
                + element.level_indices.capacity()
                + element.coords.capacity())
                * 4
            + element.levels.capacity() * size_of::<foliage::Level>())
            as u64;
        metrics.descriptors_ms = started.elapsed_ms() - early_wood_ms;
        let scopes = io::scope(&self.gpu);
        let shared_positions = shared_stations
            .as_ref()
            .is_some_and(|(_, count, _)| *count > 0);
        let computed = if let Some((segments, count, ring_size)) = shared_stations {
            if count == 0 {
                self.empty()
            } else {
                let p = foliage::prepared::PreparedStations {
                    segments,
                    count,
                    ring_size,
                    rings: std::borrow::Cow::Borrowed(&uploaded_wood.as_ref().unwrap().positions),
                };
                self.compute_buffer_async(p, family, twig, &element, reference, &mut metrics)
                    .await
            }
        } else {
            let stations = stations.unwrap();
            if stations.count == 0 {
                self.empty()
            } else {
                self.compute_async(stations, family, twig, &element, reference, &mut metrics)
                    .await
            }
        };
        let errors = io::errors(&self.gpu, scopes).await;
        let mut resident = Some(computed?);
        errors?;
        if let Some(wood) = &uploaded_wood {
            metrics.gpu_compute_peak_bytes += wood.gpu_metadata_bytes()
                + if shared_positions {
                    0
                } else {
                    wood.positions.size()
                };
        }
        let full = resident.as_ref().unwrap().full;
        metrics.retained_gpu_bytes = resident.as_ref().map_or(0, |r| {
            r.leaves.region().capacity() + r.masses.region().capacity()
        });
        let mut instances = Instances::new(reference);
        if delivery == Delivery::Cpu {
            let start = Clock::now();
            let output = resident.take().unwrap();
            instances.leaves =
                io::read_leaves_async(&self.gpu, output.leaves.buffer(), output.count).await?;
            drop(output);
            instances.validate()?;
            metrics.readback_ms = start.elapsed_ms();
        }
        let started = Clock::now();
        let mut resident_wood = None;
        if let Some(uploaded) = uploaded_wood {
            let scopes = io::scope(&self.gpu);
            let result = self.expand_uploaded_wood(uploaded, &mut metrics).await;
            let errors = io::errors(&self.gpu, scopes).await;
            resident_wood = result?;
            errors?;
        } else if delivery == Delivery::Resident && !wood_attempted {
            let compact = surface::prepared::prepare(
                &tree,
                family.skeleton.envelope.height,
                &family.surface,
            )?;
            metrics.wood_prepare_ms = started.elapsed_ms();
            if let Some(compact) = compact {
                let scopes = io::scope(&self.gpu);
                let result = self.expand_wood(compact, &mut metrics).await;
                let errors = io::errors(&self.gpu, scopes).await;
                resident_wood = result?;
                errors?;
            } else {
                metrics.wood_fallback = Some("CPU triangle admission");
            }
        }
        metrics.wood_backend = Some(if delivery == Delivery::Cpu {
            Backend::Cpu
        } else if resident_wood.is_some() {
            Backend::Gpu
        } else {
            Backend::CpuFallback
        });
        let wood = if resident_wood.is_some() {
            surface::SurfaceMesh::default()
        } else {
            surface::build(&tree, family.skeleton.envelope.height, &family.surface)?
        };
        metrics.wood_ms = started.elapsed_ms() + early_wood_ms;
        metrics.wood_cpu_bytes = wood::cpu_bytes(&wood);
        if let Some(w) = &resident_wood {
            metrics.retained_gpu_bytes += w.bytes();
        }
        let bounds = union(
            resident_wood.as_ref().map_or(wood.bounds, |w| w.bounds),
            full,
        )
        .ok_or(telperion_core::Error::InvalidInput("mesh has no geometry"))?;
        let mesh = TreeMesh {
            wood,
            foliage: mesh::Foliage { element, instances },
            bounds,
        };
        crate::submit::fits_wood_counts(
            &self.gpu.device.limits(),
            &mesh,
            metrics.instances as usize,
            resident_wood
                .as_ref()
                .map(|w| (w.vertices, w.index_count, w.runs.as_slice())),
        )?;
        metrics.total_ms = total.elapsed_ms();
        Ok(Prepared {
            identity: self.identity.clone(),
            mesh,
            resident,
            wood: resident_wood,
            backend: Backend::Gpu,
            metrics,
        })
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
