//! The comparison route, the resident build's oracle in tests: `family`
//! grown on the CPU through the one cycle (`executor::grow`), its leaves
//! placed on the device from the CPU's stations, and the readback the
//! tests compare by.
use super::*;
use telperion_core::{
    foliage::{self, Instances},
    mesh,
    pipeline::executor::{self, Expansion},
    surface,
};

impl Generator {
    /// A tree with no leaves: the buffers a placement of none holds.
    pub(super) fn empty(&self) -> Result<Resident> {
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

    /// The placed leaves `prepared` holds, read back from the device.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn read_instances(&self, prepared: &Prepared) -> Result<Instances> {
        pollster::block_on(self.read_instances_async(prepared))
    }

    async fn read_instances_async(&self, prepared: &Prepared) -> Result<Instances> {
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

    /// The reference mesh whole, where no stations were prepared.
    fn on_cpu(&self, x: Expansion, mut metrics: Metrics, total: Clock) -> Result<Prepared> {
        let mesh = x.mesh()?;
        metrics.wood_cpu_bytes = wood::cpu_bytes(&mesh.wood);
        metrics.instances = mesh.foliage_instances() as u32;
        metrics.total_ms = total.elapsed_ms();
        Ok(Prepared {
            identity: self.identity.clone(),
            mesh,
            resident: None,
            organ: None,
            wood: None,
            backend: Backend::Gpu,
            metrics,
            colouring: 1.0,
        })
    }

    /// The prepared tree: `x`'s element and flowers beside its wood and
    /// placed leaves, bounded by all of them.
    fn assemble(
        &self,
        x: Expansion,
        (curve, wood, full): (surface::Curve, surface::SurfaceMesh, Option<Bounds>),
        instances: Instances,
        resident: Option<Resident>,
        mut metrics: Metrics,
        total: Clock,
    ) -> Result<Prepared> {
        metrics.wood_cpu_bytes = wood::cpu_bytes(&wood);
        // The flowers are the CPU step's on both executors (fn-216).
        let organ = x.organ()?;
        let flowers = match &organ {
            Some(o) => o.instances.bounds(&o.element)?.map(Bounds::from),
            None => None,
        };
        let bounds = union(union(curve.bounds(), full), flowers)
            .ok_or(telperion_core::Error::InvalidInput("mesh has no geometry"))?;
        let mesh = TreeMesh {
            wood,
            curve,
            foliage: mesh::Foliage {
                element: x.into_element(),
                instances,
            },
            organ,
            bounds,
        };
        crate::submit::fits_count(&self.gpu.device.limits(), &mesh, metrics.instances as usize)?;
        metrics.total_ms = total.elapsed_ms();
        Ok(Prepared {
            identity: self.identity.clone(),
            mesh,
            resident,
            organ: None,
            wood: None,
            backend: Backend::Gpu,
            metrics,
            colouring: 1.0,
        })
    }
}

/// `family` grown through the one cycle.
fn grow(family: &Family, metrics: &mut Metrics) -> Result<executor::Grown> {
    let started = Clock::now();
    let grown = executor::grow(family)?;
    metrics.skeleton_ms = started.elapsed_ms();
    Ok(grown)
}

fn base_cpu_bytes(x: &Expansion) -> u64 {
    let element = x.element();
    (x.tree().nodes.capacity() * size_of::<telperion_core::tree::Node>()
        + element.positions.capacity() * size_of::<telperion_core::math::Vec3>()
        + (element.indices.capacity()
            + element.level_indices.capacity()
            + element.coords.capacity())
            * 4
        + element.levels.capacity() * size_of::<foliage::Level>()) as u64
}

fn retained_bytes(r: &Resident) -> u64 {
    r.leaves.region().capacity() + r.masses.region().capacity()
}

/// The comparison route: `family` grown on the CPU, its ring positions
/// made by the position programme's every entry (or uploaded from the
/// CPU's), the leaves seated on them and left on the device.
impl Generator {
    pub(super) async fn prepare_whole_resident(&self, family: &Family) -> Result<Prepared> {
        let total = Clock::now();
        let mut metrics = Metrics::default();
        let grown = grow(family, &mut metrics)?;
        let started = Clock::now();
        let x = grown.expansion()?;
        let mut uploaded_wood = None;
        let mut shared_stations = None;
        let mut early_wood_ms = 0.0;
        // The curve the compact rings were read from, kept for the drawn wood.
        let mut swept = None;
        // Ring positions have one reader, the seated leaves' contacts
        // (`pack.wgsl`): a family that seats nothing builds none (fn-212 R2).
        if x.seats() && x.supports_stations() && x.round_section() {
            let begin = Clock::now();
            let mut candidate_stations = None;
            let mut station_ms = 0.0;
            let mut station_bytes = 0;
            let compact = x.curve_with_contacts().and_then(|(curve, shared)| {
                swept = Some(curve);
                metrics.shared_contact_cpu_bytes = shared.contact_bytes() as u64;
                if shared.surface().qualified() {
                    let station_start = Clock::now();
                    candidate_stations = x
                        .compact_stations(&shared)?
                        .map(|p| (p.segments, p.count, p.ring_size));
                    station_ms = station_start.elapsed_ms();
                    station_bytes = candidate_stations.as_ref().map_or(0, |(s, _, _)| {
                        (s.capacity() * size_of::<foliage::prepared::StationSegment>()) as u64
                    });
                }
                metrics.shared_prepare_cpu_bytes = wood::compact_cpu_bytes(shared.surface())
                    + metrics.shared_contact_cpu_bytes
                    + station_bytes;
                Ok(shared.into_surface())
            });
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
                metrics.shared_prepare_cpu_bytes = metrics
                    .shared_prepare_cpu_bytes
                    .max(wood::compact_cpu_bytes(&p) + station_bytes);
                if candidate_stations.is_some() || !p.qualified() {
                    let scopes = io::scope(&self.gpu);
                    let result = self.emit_positions(p, &mut metrics).await;
                    let errors = io::errors(&self.gpu, scopes).await;
                    uploaded_wood = result?;
                    errors?;
                    metrics.shared_prepare_cpu_bytes = metrics
                        .shared_prepare_cpu_bytes
                        .max(metrics.position_cpu_bytes + station_bytes);
                } else {
                    metrics.position_fallback = Some("station capability");
                }
                if let Some(wood) = &uploaded_wood {
                    shared_stations = candidate_stations;
                    metrics.shared_metadata_cpu_bytes = wood.metadata_bytes();
                }
            }
            early_wood_ms = begin.elapsed_ms() - station_ms;
        } else if x.seats() {
            metrics.position_fallback = Some("profile or station capability");
        }
        if uploaded_wood.is_none() && x.seats() && x.supports_stations() {
            let prepare_start = Clock::now();
            let shared = x.prepared_with_contacts()?;
            metrics.wood_prepare_ms = prepare_start.elapsed_ms();
            early_wood_ms += metrics.wood_prepare_ms;
            if let Some(shared) = shared {
                metrics.shared_contact_cpu_bytes = shared.contact_bytes() as u64;
                if let Some(p) = x.shared_stations(&shared)? {
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
                    if let Some(wood) = &uploaded_wood {
                        metrics.shared_metadata_cpu_bytes = wood.metadata_bytes();
                        shared_stations = Some(station_data);
                    }
                }
            } else {
                metrics.wood_fallback = Some("CPU triangle admission");
            }
        }
        let stations = if shared_stations.is_some() {
            None
        } else {
            x.stations()?
        };
        if stations.is_none() && shared_stations.is_none() {
            drop(uploaded_wood);
            metrics.gpu_positions = false;
            metrics.position_retained_metadata_bytes = 0;
            return self.on_cpu(x, metrics, total);
        };
        metrics.base_cpu_bytes = base_cpu_bytes(&x);
        metrics.descriptors_ms = started.elapsed_ms() - early_wood_ms;
        let element = x.element();
        let scopes = io::scope(&self.gpu);
        let shared_positions = shared_stations
            .as_ref()
            .is_some_and(|(_, count, _)| *count > 0);
        let computed = if let Some((segments, count, ring_size)) = shared_stations {
            if count == 0 {
                self.empty()
            } else {
                let rings = &*uploaded_wood.as_ref().unwrap().positions;
                let p = foliage::prepared::PreparedStations {
                    segments,
                    count,
                    ring_size,
                    rings,
                };
                let (leaves, twig) = (x.leaves(), x.twig());
                let (lent, reference) = (rings.size(), x.reference());
                self.compute_buffer_async(p, lent, &leaves, twig, element, reference, &mut metrics)
                    .await
            }
        } else {
            let stations = stations.unwrap();
            if stations.count == 0 {
                self.empty()
            } else {
                let (leaves, twig) = (x.leaves(), x.twig());
                self.compute_async(
                    stations,
                    &leaves,
                    twig,
                    element,
                    x.reference(),
                    &mut metrics,
                )
                .await
            }
        };
        let errors = io::errors(&self.gpu, scopes).await;
        let resident = computed?;
        errors?;
        if let Some(wood) = &uploaded_wood {
            metrics.gpu_compute_peak_bytes += wood.gpu_metadata_bytes()
                + if shared_positions {
                    0
                } else {
                    wood.positions.size()
                };
        }
        let full = resident.full;
        metrics.retained_gpu_bytes = retained_bytes(&resident);
        let instances = Instances::new(x.reference());
        // The ring positions served the stations; the wood itself is drawn
        // from its curve (fn-208, host decision 20).
        drop(uploaded_wood);
        let started = Clock::now();
        let curve = match swept {
            Some(curve) => curve,
            None => x.curve()?,
        };
        metrics.wood_backend = Some(Backend::Gpu);
        metrics.wood_ms = started.elapsed_ms() + early_wood_ms;
        let wood = surface::SurfaceMesh::default();
        let parts = (curve, wood, full);
        self.assemble(x, parts, instances, Some(resident), metrics, total)
    }
}

/// The box over both bounds, or the one there is.
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
