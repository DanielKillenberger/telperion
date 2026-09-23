//! Native requested-output and completed-frame comparison, one mode per process.
use serde_json::json;
use std::time::Instant;
use telperion_core::{mesh, presets::Preset};
use telperion_render::{
    generation::{Delivery, Generator},
    hero_pose, Frame, Gpu, Renderer, GROUND_REACH, STILL_FORMAT,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let id = args.get(1).ok_or("preset")?;
    let mode = args
        .get(2)
        .ok_or("cpu-output|cpu-render|gpu-output|gpu-render")?;
    if !["cpu-output", "cpu-render", "gpu-output", "gpu-render"].contains(&mode.as_str()) {
        return Err("invalid mode".into());
    }
    let mut family = Preset::from_id(id).ok_or("unknown preset")?.parameters();
    family.skeleton.seed = std::env::var("GENERATION_SEED")
        .unwrap_or_else(|_| "1".into())
        .parse()?;
    let samples: usize = std::env::var("GENERATION_SAMPLES")
        .unwrap_or_else(|_| "4".into())
        .parse()?;
    let gpu_mode = mode.starts_with("gpu");
    let render_mode = mode.ends_with("render");
    let init = Instant::now();
    let mut renderer = if render_mode {
        Some(Renderer::new(
            pollster::block_on(Gpu::request(None))?,
            STILL_FORMAT,
        ))
    } else {
        None
    };
    let mut adapter = renderer.as_ref().map(|r| r.gpu().adapter.name.clone());
    let generator = if gpu_mode {
        Some(if let Some(renderer) = &renderer {
            Generator::new(renderer)?
        } else {
            let gpu = pollster::block_on(Gpu::request(None))?;
            adapter = Some(gpu.adapter.name.clone());
            Generator::for_cpu_output(gpu)?
        })
    } else {
        None
    };
    let frame = if render_mode {
        Some(Frame::new(
            renderer.as_ref().unwrap(),
            "generation comparison",
            (1280, 720),
        ))
    } else {
        None
    };
    println!(
        "{}",
        json!({"event":"provenance","preset":id,"seed":family.skeleton.seed,"mode":mode,"samples":samples,"initializationMs":init.elapsed().as_secs_f64()*1000.0,"adapter":adapter,"viewport":[1280,720],"camera":"hero","boundary":"GPU queue/device completion for rendering; owned geometry for output","family":telperion_core::params::metadata(&family)})
    );
    for sample in 0..samples {
        let previous_tree_gpu_bytes = renderer.as_ref().map_or(0, Generator::tree_buffer_bytes);
        let start = Instant::now();
        let (count, bounds, stages, prepared, cpu) = if let Some(generator) = &generator {
            let prepared = generator.prepare(
                &family,
                if render_mode {
                    Delivery::Resident
                } else {
                    Delivery::Cpu
                },
            )?;
            let m = &prepared.metrics;
            let stages = json!({"backend":format!("{:?}",prepared.backend),"positionPrepareMs":m.position_prepare_ms,"positionUploadMs":m.position_upload_ms,"positionWaitMs":m.position_wait_ms,"positionCpuBytes":m.position_cpu_bytes,"positionGpuPeakBytes":m.position_gpu_peak_bytes,"positionRetainedMetadataBytes":m.position_retained_metadata_bytes,"positionFallback":m.position_fallback,"gpuPositions":m.gpu_positions,"skeletonMs":m.skeleton_ms,"descriptorsMs":m.descriptors_ms,"uploadDispatchMs":m.upload_dispatch_ms,"placementWaitMs":m.placement_wait_ms,"compactMs":m.compact_ms,"massMs":m.mass_ms,"readbackMs":m.readback_ms,"woodMs":m.wood_ms,"woodPrepareMs":m.wood_prepare_ms,"woodUploadDispatchMs":m.wood_upload_dispatch_ms,"woodWaitMs":m.wood_wait_ms,"woodPreparedCpuBytes":m.wood_prepared_cpu_bytes,"woodMetadataCpuBytes":m.wood_metadata_cpu_bytes,"woodGpuPeakBytes":m.wood_gpu_peak_bytes,"woodBackend":m.wood_backend.map(|b| format!("{b:?}")),"woodFallback":m.wood_fallback,"inputInstances":m.input_instances,"baseCpuBytes":m.base_cpu_bytes,"woodCpuBytes":m.wood_cpu_bytes,"retainedGpuBytes":m.retained_gpu_bytes,"descriptorCpuBytes":m.descriptor_cpu_bytes,"sharedContactCpuBytes":m.shared_contact_cpu_bytes,"sharedPrepareCpuBytes":m.shared_prepare_cpu_bytes,"sharedMetadataCpuBytes":m.shared_metadata_cpu_bytes,"gpuComputePeakBytes":m.gpu_compute_peak_bytes});
            (
                prepared.count(),
                prepared.bounds(),
                stages,
                Some(prepared),
                None,
            )
        } else {
            let mesh = mesh::build(&family)?;
            (
                mesh.foliage_instances(),
                mesh.bounds,
                json!({"backend":"Cpu"}),
                None,
                Some(mesh),
            )
        };
        let prepare_ms = start.elapsed().as_secs_f64() * 1000.0;
        let mut hash = None;
        if std::env::var_os("GENERATION_VERIFY").is_some() {
            let instances = if let Some(p) = &prepared {
                Some(generator.as_ref().unwrap().read_instances(p)?)
            } else {
                None
            };
            let leaves = instances
                .as_ref()
                .map(|i| &i.leaves)
                .unwrap_or_else(|| &cpu.as_ref().unwrap().foliage.instances.leaves);
            let mut value = 14695981039346656037u64;
            for byte in leaves
                .iter()
                .flat_map(|leaf| leaf.iter().flat_map(|w| w.to_le_bytes()))
            {
                value = (value ^ u64::from(byte)).wrapping_mul(1099511628211);
            }
            hash = Some(format!("{value:016x}"));
        }
        let delivery = Instant::now();
        if render_mode {
            let renderer = renderer.as_mut().unwrap();
            renderer.set_material(family.material);
            if let Some(p) = prepared {
                renderer.submit_prepared(p)?;
            } else {
                renderer.submit(cpu.as_ref().unwrap())?;
            }
            let camera = hero_pose(bounds, 1280.0 / 720.0, GROUND_REACH);
            renderer.draw(&camera, (1280, 720), frame.as_ref().unwrap().target());
            renderer
                .gpu()
                .device
                .poll(wgpu::PollType::wait_indefinitely())?;
        }
        let delivery_ms = if render_mode {
            delivery.elapsed().as_secs_f64() * 1000.0
        } else {
            0.0
        };
        if let Some(path) = std::env::var_os("GENERATION_CAPTURE") {
            if render_mode {
                let renderer = renderer.as_mut().unwrap();
                let camera = hero_pose(bounds, 1280.0 / 720.0, GROUND_REACH);
                let still = telperion_render::render(renderer, &camera, 1280, 720)?;
                telperion_render::write_png(std::path::Path::new(&path), &still)?;
            }
        }
        println!(
            "{}",
            json!({"event":"sample","sample":sample,"cold":sample==0,"prepareMs":prepare_ms,"deliveryMs":delivery_ms,"totalMs":prepare_ms+delivery_ms,"instances":count,"bounds":[[bounds.min.x,bounds.min.y,bounds.min.z],[bounds.max.x,bounds.max.y,bounds.max.z]],"hash":hash,"previousTreeGpuBytes":previous_tree_gpu_bytes,"treeGpuBytes":renderer.as_ref().map(Generator::tree_buffer_bytes),"stages":stages})
        );
    }
    Ok(())
}
