//! Optional native GPU foliage experiment. The pure core and browser build paths
//! remain available; every request explicitly states its delivery mode.
mod compute;
mod data;
mod io;
use crate::{buffer::Held, Gpu, Renderer, Result, Submitted};
use std::{sync::Arc, time::Instant};
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
    Gpu,
    CpuFallback,
}
#[derive(Debug, Default)]
pub struct Metrics {
    pub skeleton_ms: f64,
    pub descriptors_ms: f64,
    pub upload_dispatch_ms: f64,
    pub placement_wait_ms: f64,
    pub compact_ms: f64,
    pub mass_ms: f64,
    pub readback_ms: f64,
    pub wood_ms: f64,
    pub total_ms: f64,
    pub base_cpu_bytes: u64,
    pub wood_cpu_bytes: u64,
    pub retained_gpu_bytes: u64,
    pub input_instances: u32,
    pub instances: u32,
    pub descriptor_cpu_bytes: u64,
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
    pub backend: Backend,
    pub metrics: Metrics,
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
}
impl Generator {
    /// Live tree buffers only; excludes textures, pipelines and driver allocations.
    pub fn tree_buffer_bytes(renderer: &Renderer) -> u64 {
        renderer.wood.allocated_bytes() + renderer.foliage.allocated_bytes()
    }
    pub fn new(renderer: &Renderer) -> Result<Self> {
        Self::create(renderer.gpu.clone(), renderer.identity.clone(), true)
    }
    /// Experimental owned CPU output without renderer resources. Resident delivery
    /// is rejected before generation; use `new` for renderer-bound results.
    pub fn for_cpu_output(gpu: Gpu) -> Result<Self> {
        Self::create(gpu, Arc::new(()), false)
    }
    fn create(gpu: Gpu, identity: Arc<()>, resident_allowed: bool) -> Result<Self> {
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
        io::errors(&gpu, scopes)?;
        Ok(Self {
            gpu,
            identity,
            resident_allowed,
            place,
            compact,
            mass,
        })
    }
    pub fn prepare(&self, family: &Family, delivery: Delivery) -> Result<Prepared> {
        if delivery == Delivery::Resident && !self.resident_allowed {
            return Err(telperion_core::Error::InvalidInput(
                "standalone generation requires CPU delivery",
            )
            .into());
        }
        let total = Instant::now();
        let mut metrics = Metrics::default();
        let started = Instant::now();
        let tree = branching::generate(&family.skeleton, family.radii)?.tree;
        metrics.skeleton_ms = started.elapsed().as_secs_f64() * 1000.0;
        let started = Instant::now();
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
        let Some(stations) = foliage::prepared::prepare_stations(
            &tree,
            family.skeleton.envelope,
            family.canopy,
            Some(twig),
            &family.surface,
        )?
        else {
            let mesh = mesh::assemble(&tree, family)?;
            metrics.total_ms = total.elapsed().as_secs_f64() * 1000.0;
            return Ok(Prepared {
                identity: self.identity.clone(),
                mesh,
                resident: None,
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
        metrics.descriptors_ms = started.elapsed().as_secs_f64() * 1000.0;
        let scopes = io::scope(&self.gpu);
        let computed = if stations.count == 0 {
            self.empty()
        } else {
            self.compute(stations, family, twig, &element, reference, &mut metrics)
        };
        let errors = io::errors(&self.gpu, scopes);
        let mut resident = Some(computed?);
        errors?;
        let full = resident.as_ref().unwrap().full;
        metrics.retained_gpu_bytes = resident.as_ref().map_or(0, |r| {
            r.leaves.region().capacity() + r.masses.region().capacity()
        });
        let mut instances = Instances::new(reference);
        if delivery == Delivery::Cpu {
            let start = Instant::now();
            let output = resident.take().unwrap();
            let bytes = io::read(
                &self.gpu,
                output.leaves.buffer(),
                u64::from(output.count) * 12,
            )?;
            instances.leaves = bytes
                .chunks_exact(12)
                .map(|b| {
                    [
                        u32::from_ne_bytes(b[0..4].try_into().unwrap()),
                        u32::from_ne_bytes(b[4..8].try_into().unwrap()),
                        u32::from_ne_bytes(b[8..12].try_into().unwrap()),
                    ]
                })
                .collect();
            drop(bytes);
            drop(output);
            instances.validate()?;
            metrics.readback_ms = start.elapsed().as_secs_f64() * 1000.0;
        }
        // No descriptor, contact, rank or raw GPU buffer crosses into wood construction.
        let started = Instant::now();
        let wood = surface::build(&tree, family.skeleton.envelope.height, &family.surface)?;
        metrics.wood_ms = started.elapsed().as_secs_f64() * 1000.0;
        metrics.wood_cpu_bytes = ((wood.positions.capacity()
            + wood.normals.capacity()
            + wood.coords.capacity()
            + wood.indices.capacity())
            * 4
            + wood.run_table.capacity() * size_of::<surface::SurfaceRun>())
            as u64;
        let bounds = union(wood.bounds, full)
            .ok_or(telperion_core::Error::InvalidInput("mesh has no geometry"))?;
        let mesh = TreeMesh {
            wood,
            foliage: mesh::Foliage { element, instances },
            bounds,
        };
        crate::submit::fits_count(&self.gpu.device.limits(), &mesh, metrics.instances as usize)?;
        metrics.total_ms = total.elapsed().as_secs_f64() * 1000.0;
        Ok(Prepared {
            identity: self.identity.clone(),
            mesh,
            resident,
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
    pub fn read_instances(&self, prepared: &Prepared) -> Result<Instances> {
        if !Arc::ptr_eq(&self.identity, &prepared.identity) {
            return Err(telperion_core::Error::InvalidInput("generation renderer mismatch").into());
        }
        if let Some(r) = &prepared.resident {
            let bytes = io::read(&self.gpu, r.leaves.buffer(), u64::from(r.count) * 12)?;
            let mut out = Instances::new(prepared.mesh.foliage.instances.reference);
            out.leaves = bytes
                .chunks_exact(12)
                .map(|b| {
                    [
                        u32::from_ne_bytes(b[0..4].try_into().unwrap()),
                        u32::from_ne_bytes(b[4..8].try_into().unwrap()),
                        u32::from_ne_bytes(b[8..12].try_into().unwrap()),
                    ]
                })
                .collect();
            Ok(out)
        } else {
            Ok(prepared.mesh.foliage.instances.clone())
        }
    }
}
impl Renderer {
    pub fn submit_prepared(&mut self, prepared: Prepared) -> Result<Submitted> {
        if !Arc::ptr_eq(&self.identity, &prepared.identity) {
            return Err(telperion_core::Error::InvalidInput("generation renderer mismatch").into());
        }
        if let Some(error) = self.gpu.lost() {
            return Err(error);
        }
        let count = prepared.count();
        crate::submit::fits_count(&self.gpu.device.limits(), &prepared.mesh, count)?;
        let Some(resident) = prepared.resident else {
            return self.submit(&prepared.mesh);
        };
        let mesh = prepared.mesh;
        self.wood.submit(&self.gpu, &mesh.wood);
        self.foliage.submit_resident(
            &self.gpu,
            &mesh.foliage.element,
            mesh.foliage.instances.reference,
            resident.count,
            resident.leaves,
            resident.masses,
        );
        self.scene
            .place_figure(&self.gpu, mesh.bounds.max.y - mesh.bounds.min.y);
        self.scene.set_crown(resident.crown);
        self.scene
            .set_leaf_reference(mesh.foliage.instances.reference);
        self.scene.section_roundness = mesh.foliage.element.section_roundness;
        self.bounds = Some(mesh.bounds);
        self.set_casters();
        self.level_deviations = mesh
            .foliage
            .element
            .levels
            .iter()
            .map(|l| l.deviation)
            .collect();
        Ok(Submitted {
            wood_vertices: mesh.wood_vertices(),
            wood_triangles: mesh.wood_triangles(),
            foliage_instances: count,
            bounds: mesh.bounds,
        })
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
