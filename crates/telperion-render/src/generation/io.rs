use crate::{Gpu, RenderError, Result};

pub(super) fn buffer(
    gpu: &Gpu,
    label: &'static str,
    bytes: u64,
    usage: wgpu::BufferUsages,
    initial: &[u8],
) -> Result<wgpu::Buffer> {
    let size = bytes.max(16).next_multiple_of(4);
    let limits = gpu.device.limits();
    let limit = if usage.contains(wgpu::BufferUsages::STORAGE) {
        limits
            .max_buffer_size
            .min(limits.max_storage_buffer_binding_size)
    } else {
        limits.max_buffer_size
    };
    if size > limit {
        return Err(RenderError::Oversize {
            buffer: label,
            bytes: size,
            limit,
        });
    }
    let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    if !initial.is_empty() {
        gpu.queue.write_buffer(&buffer, 0, initial);
    }
    Ok(buffer)
}

pub(super) struct Pass {
    pub layout: wgpu::BindGroupLayout,
    pub pipelines: Vec<wgpu::ComputePipeline>,
    maximum: u32,
}
impl Pass {
    pub fn new(gpu: &Gpu, shader: String, read_only: &[bool], entries: &[&str]) -> Self {
        let mut bindings = vec![wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }];
        bindings.extend(read_only.iter().enumerate().map(|(i, &read_only)| {
            wgpu::BindGroupLayoutEntry {
                binding: i as u32 + 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }
        }));
        let layout = gpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("experimental generation"),
                entries: &bindings,
            });
        let module = gpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("experimental generation"),
                source: wgpu::ShaderSource::Wgsl(shader.into()),
            });
        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("experimental generation"),
                bind_group_layouts: &[Some(&layout)],
                immediate_size: 0,
            });
        let pipelines = entries
            .iter()
            .map(|&entry| {
                gpu.device
                    .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                        label: Some(entry),
                        layout: Some(&pipeline_layout),
                        module: &module,
                        entry_point: Some(entry),
                        compilation_options: Default::default(),
                        cache: None,
                    })
            })
            .collect();
        Self {
            layout,
            pipelines,
            maximum: gpu.device.limits().max_compute_workgroups_per_dimension,
        }
    }
    pub fn bind(&self, gpu: &Gpu, buffers: &[&wgpu::Buffer]) -> wgpu::BindGroup {
        gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("experimental generation"),
            layout: &self.layout,
            entries: &buffers
                .iter()
                .enumerate()
                .map(|(i, b)| wgpu::BindGroupEntry {
                    binding: i as u32,
                    resource: b.as_entire_binding(),
                })
                .collect::<Vec<_>>(),
        })
    }
    pub fn run(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        bind: &wgpu::BindGroup,
        entry: usize,
        groups: u32,
    ) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("experimental generation"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.pipelines[entry]);
        pass.set_bind_group(0, bind, &[]);
        pass.dispatch_workgroups(groups.min(self.maximum), groups.div_ceil(self.maximum), 1);
    }
}

pub(super) struct PendingRead {
    target: wgpu::Buffer,
    receive: futures_channel::oneshot::Receiver<std::result::Result<(), wgpu::BufferAsyncError>>,
    bytes: u64,
}

pub(super) fn begin_read(gpu: &Gpu, source: &wgpu::Buffer, bytes: u64) -> Result<PendingRead> {
    let target = buffer(
        gpu,
        "generation readback",
        bytes,
        wgpu::BufferUsages::MAP_READ,
        &[],
    )?;
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(source, 0, &target, 0, bytes);
    gpu.queue.submit([encoder.finish()]);
    let (send, receive) = futures_channel::oneshot::channel();
    target
        .slice(..bytes)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = send.send(result);
        });
    Ok(PendingRead {
        target,
        receive,
        bytes,
    })
}

impl PendingRead {
    pub(super) async fn complete(self, _gpu: &Gpu) -> Result<Vec<u8>> {
        #[cfg(not(target_arch = "wasm32"))]
        _gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| error(e.to_string()))?;
        self.receive
            .await
            .map_err(|e| error(e.to_string()))?
            .map_err(|e| error(e.to_string()))?;
        let view = self
            .target
            .slice(..self.bytes)
            .get_mapped_range()
            .map_err(|e| error(e.to_string()))?;
        let result = view.to_vec();
        drop(view);
        self.target.unmap();
        Ok(result)
    }
}

pub(super) async fn read_async(gpu: &Gpu, source: &wgpu::Buffer, bytes: u64) -> Result<Vec<u8>> {
    if bytes == 0 {
        return Ok(Vec::new());
    }
    begin_read(gpu, source, bytes)?.complete(gpu).await
}

pub(super) async fn read_leaves_async(
    gpu: &Gpu,
    source: &wgpu::Buffer,
    count: u32,
) -> Result<Vec<[u32; 3]>> {
    read_leaves_chunked(gpu, source, count as usize, 4 * 1024 * 1024).await
}

pub(super) fn reserve_leaves(count: usize) -> Result<Vec<[u32; 3]>> {
    let mut leaves = Vec::new();
    leaves
        .try_reserve_exact(count)
        .map_err(|e| error(e.to_string()))?;
    Ok(leaves)
}

pub(super) async fn read_leaves_chunked(
    gpu: &Gpu,
    source: &wgpu::Buffer,
    count: usize,
    chunk_limit: u64,
) -> Result<Vec<[u32; 3]>> {
    if count == 0 {
        return Ok(Vec::new());
    }
    let bytes = u64::try_from(count)
        .ok()
        .and_then(|n| n.checked_mul(12))
        .ok_or_else(|| error("leaf readback size overflow".into()))?;
    if bytes > source.size() || !source.usage().contains(wgpu::BufferUsages::COPY_SRC) {
        return Err(error("invalid leaf readback source".into()));
    }
    let chunk_bytes = bytes
        .min(chunk_limit)
        .min(4 * 1024 * 1024)
        .min(gpu.device.limits().max_buffer_size)
        / 12
        * 12;
    if chunk_bytes == 0 {
        return Err(error(
            "leaf readback staging limit is smaller than one leaf".into(),
        ));
    }
    if let Some(error) = gpu.lost() {
        return Err(error);
    }
    let mut leaves = reserve_leaves(count)?;
    let scopes = scope(gpu);
    let result = async {
        let target = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bounded leaf readback"),
            size: chunk_bytes,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut offset = 0;
        while offset < bytes {
            let length = chunk_bytes.min(bytes - offset);
            let mut encoder = gpu.device.create_command_encoder(&Default::default());
            encoder.copy_buffer_to_buffer(source, offset, &target, 0, length);
            gpu.queue.submit([encoder.finish()]);
            let (send, receive) = futures_channel::oneshot::channel();
            target
                .slice(..length)
                .map_async(wgpu::MapMode::Read, move |result| {
                    let _ = send.send(result);
                });
            #[cfg(not(target_arch = "wasm32"))]
            gpu.device
                .poll(wgpu::PollType::wait_indefinitely())
                .map_err(|e| error(e.to_string()))?;
            receive
                .await
                .map_err(|e| error(e.to_string()))?
                .map_err(|e| error(e.to_string()))?;
            let view = target
                .slice(..length)
                .get_mapped_range()
                .map_err(|e| error(e.to_string()))?;
            leaves.extend(view.chunks_exact(12).map(|b| {
                [
                    u32::from_ne_bytes(b[0..4].try_into().unwrap()),
                    u32::from_ne_bytes(b[4..8].try_into().unwrap()),
                    u32::from_ne_bytes(b[8..12].try_into().unwrap()),
                ]
            }));
            drop(view);
            target.unmap();
            offset += length;
        }
        Ok(leaves)
    }
    .await;
    let checked = errors(gpu, scopes).await;
    checked?;
    result
}

pub(super) async fn errors(gpu: &Gpu, scopes: [wgpu::ErrorScopeGuard; 2]) -> Result<()> {
    let [validation, memory] = scopes;
    let memory = memory.pop().await;
    let validation = validation.pop().await;
    if let Some(error) = memory.or(validation) {
        return Err(self::error(error.to_string()));
    }
    if let Some(error) = gpu.lost() {
        return Err(error);
    }
    Ok(())
}
pub(super) fn scope(gpu: &Gpu) -> [wgpu::ErrorScopeGuard; 2] {
    [
        gpu.device.push_error_scope(wgpu::ErrorFilter::Validation),
        gpu.device.push_error_scope(wgpu::ErrorFilter::OutOfMemory),
    ]
}
pub(super) fn error(detail: String) -> RenderError {
    RenderError::DeviceRefused {
        requirement: "experimental generation".into(),
        detail,
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
pub(super) fn read(gpu: &Gpu, source: &wgpu::Buffer, bytes: u64) -> Result<Vec<u8>> {
    pollster::block_on(read_async(gpu, source, bytes))
}
pub(super) async fn complete(gpu: &Gpu) -> Result<()> {
    let (send, receive) = futures_channel::oneshot::channel();
    gpu.queue.on_submitted_work_done(move || {
        let _ = send.send(());
    });
    #[cfg(not(target_arch = "wasm32"))]
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .map_err(|e| error(e.to_string()))?;
    receive.await.map_err(|e| error(e.to_string()))?;
    if let Some(error) = gpu.lost() {
        return Err(error);
    }
    Ok(())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod pending_tests {
    use super::*;

    #[test]
    fn read_is_submitted_and_mapped_before_completion_is_polled() {
        let gpu = pollster::block_on(Gpu::request(None)).unwrap();
        let source = buffer(
            &gpu,
            "pending fixture",
            32,
            wgpu::BufferUsages::COPY_SRC,
            &[7; 32],
        )
        .unwrap();
        let mut pending = begin_read(&gpu, &source, 32).unwrap();
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();
        assert!(matches!(pending.receive.try_recv(), Ok(Some(Ok(())))));
        let view = pending.target.slice(..32).get_mapped_range().unwrap();
        assert_eq!(&*view, &[7; 32]);
        drop(view);
        pending.target.unmap();
    }
}
