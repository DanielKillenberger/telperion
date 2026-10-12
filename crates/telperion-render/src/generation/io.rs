use crate::buffer::Owned;
use crate::programs::Program;
use crate::{Gpu, RenderError, Result};

pub(super) fn buffer(
    gpu: &Gpu,
    label: &'static str,
    bytes: u64,
    usage: wgpu::BufferUsages,
    initial: &[u8],
) -> Result<Owned> {
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
    if let Err((after, cap)) = gpu.ledger.reserve(size, label) {
        let mib = |b: u64| b >> 20;
        return Err(error(format!(
            "the device budget: the build's {label} would hold {} MiB, past the {} MiB it may hold; grow it younger",
            mib(after),
            mib(cap)
        )));
    }
    let descriptor = wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    };
    let buffer = match made(gpu, &descriptor) {
        Ok(buffer) => Owned::reserved(gpu, buffer, label),
        Err(refused) => {
            gpu.ledger.unreserve(size, label);
            return Err(error(format!("the device refused the build's {label} of {} MiB: {refused}", size >> 20)));
        }
    };
    if !initial.is_empty() {
        gpu.queue.write_buffer(&buffer, 0, initial);
    }
    Ok(buffer)
}

/// A buffer made natively under scopes of its own, so a device that
/// cannot hold it (the heap past its share, `device::HEAP_SHARE`) refuses
/// it here, before any pass binds it; a browser reports it at the build's
/// scopes.
fn made(gpu: &Gpu, descriptor: &wgpu::BufferDescriptor<'_>) -> std::result::Result<wgpu::Buffer, String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let [validation, memory] = scope(gpu);
        let buffer = gpu.device.create_buffer(descriptor);
        let memory = pollster::block_on(memory.pop());
        match memory.or(pollster::block_on(validation.pop())) {
            Some(refused) => Err(refused.to_string()),
            None => Ok(buffer),
        }
    }
    #[cfg(target_arch = "wasm32")]
    Ok(gpu.device.create_buffer(descriptor))
}

/// `a` and `b` made side by side: natively `b` on a thread of its own, so
/// a set of modules compiles cold in its slowest's time, not their sum
/// (fn-256 task 4: 3.3 s in turn, 1.45 s side by side); in a browser in
/// turn, where every pipeline compiles async already. Error scopes are a
/// thread's own, so `b` runs under scopes of its own, and what they catch
/// is kept for `programs::ready` to report, as a refused program is.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn pair<A: Send, B: Send>(gpu: &Gpu, a: impl FnOnce() -> A + Send, b: impl FnOnce() -> B + Send) -> (A, B) {
    let scoped = || {
        let [validation, memory] = scope(gpu);
        let made = b();
        let memory = pollster::block_on(memory.pop());
        if let Some(error) = memory.or(pollster::block_on(validation.pop())) {
            let mut refused = gpu.refused.lock().expect("the refusals");
            refused.push(("a module made beside another".into(), error.to_string()));
        }
        made
    };
    std::thread::scope(|s| {
        let b = s.spawn(scoped);
        let a = a();
        (a, b.join().expect("a module made beside another"))
    })
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn pair<A, B>(_: &Gpu, a: impl FnOnce() -> A, b: impl FnOnce() -> B) -> (A, B) {
    (a(), b())
}

pub(super) struct Pass {
    pub layout: wgpu::BindGroupLayout,
    /// In a browser, compiling until `programs::ready` (fn-243).
    pipelines: Vec<Program>,
    maximum: u32,
    #[cfg(test)]
    entries: Vec<String>,
    /// Which bindings the shader writes, the uniform first: WebGPU
    /// refuses a dispatch whose writable binding shares its buffer with
    /// any other binding, which native Vulkan lets pass (fn-212 task 10).
    #[cfg(test)]
    writable: Vec<bool>,
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
        #[cfg(test)]
        let started = std::time::Instant::now();
        let module = gpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("experimental generation"),
                source: wgpu::ShaderSource::Wgsl(shader.into()),
            });
        #[cfg(test)]
        crate::trace::compiled(&format!("module of {}", entries[0]), started.elapsed());
        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("experimental generation"),
                bind_group_layouts: &[Some(&layout)],
                immediate_size: 0,
            });
        let descs: Vec<_> = (entries.iter())
            .map(|&entry| wgpu::ComputePipelineDescriptor {
                label: Some(entry),
                layout: Some(&pipeline_layout),
                module: &module,
                entry_point: Some(entry),
                compilation_options: Default::default(),
                cache: None,
            })
            .collect();
        let made = Program::make_many(gpu, &descs);
        #[cfg(test)]
        for (entry, (_, took)) in entries.iter().zip(&made) {
            crate::trace::compiled(entry, *took);
        }
        let pipelines = made.into_iter().map(|(p, _)| p).collect();
        Self {
            layout,
            pipelines,
            maximum: gpu.device.limits().max_compute_workgroups_per_dimension,
            #[cfg(test)]
            entries: entries.iter().map(|e| e.to_string()).collect(),
            #[cfg(test)]
            writable: std::iter::once(false)
                .chain(read_only.iter().map(|&r| !r))
                .collect(),
        }
    }
    pub fn bind(&self, gpu: &Gpu, buffers: &[&wgpu::Buffer]) -> wgpu::BindGroup {
        #[cfg(test)]
        for (i, a) in buffers.iter().enumerate() {
            for (j, b) in buffers.iter().enumerate().skip(i + 1) {
                let written = self.writable[i] || self.writable[j];
                assert!(
                    !(written && a == b),
                    "{:?}: bindings {i} and {j} share a buffer one of them writes, \
                     which WebGPU refuses",
                    self.entries
                );
            }
        }
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
        #[cfg(test)]
        crate::trace::dispatch(&self.entries[entry], groups);
        let mut pass = self.begin(encoder, bind, entry);
        pass.dispatch_workgroups(groups.min(self.maximum), groups.div_ceil(self.maximum), 1);
    }

    /// `entry` dispatched inside `pass`, a compute pass it shares with
    /// the dispatches beside it (unstamped): `groups` workgroups.
    pub fn encode(&self, pass: &mut wgpu::ComputePass<'_>, (bind, entry): (&wgpu::BindGroup, usize), groups: u32) {
        pass.set_pipeline(self.pipelines[entry].get());
        pass.set_bind_group(0, bind, &[]);
        pass.dispatch_workgroups(groups.min(self.maximum), groups.div_ceil(self.maximum), 1);
    }

    /// A stamped compute pass with `entry`'s pipeline and `bind` set.
    fn begin<'e>(
        &self,
        encoder: &'e mut wgpu::CommandEncoder,
        bind: &wgpu::BindGroup,
        entry: usize,
    ) -> wgpu::ComputePass<'e> {
        #[cfg(test)]
        let stamp = super::profile::slot(&self.entries[entry]);
        #[cfg(not(test))]
        let stamp: Option<(wgpu::QuerySet, u32)> = None;
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("experimental generation"),
            timestamp_writes: stamp
                .as_ref()
                .map(|(set, at)| wgpu::ComputePassTimestampWrites {
                    query_set: set,
                    beginning_of_pass_write_index: Some(*at),
                    end_of_pass_write_index: Some(at + 1),
                }),
        });
        pass.set_pipeline(self.pipelines[entry].get());
        pass.set_bind_group(0, bind, &[]);
        pass
    }
}

pub(super) struct PendingRead {
    target: crate::buffer::Owned,
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
    let mut encoder = gpu.encoder(&Default::default());
    encoder.copy_buffer_to_buffer(source, 0, &target, 0, bytes);
    gpu.submit([encoder.finish()]);
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
        #[cfg(test)]
        crate::trace::waited("readback");
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
        let target = Owned::create(
            gpu,
            &wgpu::BufferDescriptor {
                label: Some("bounded leaf readback"),
                size: chunk_bytes,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            },
        );
        let mut offset = 0;
        while offset < bytes {
            let length = chunk_bytes.min(bytes - offset);
            let mut encoder = gpu.encoder(&Default::default());
            encoder.copy_buffer_to_buffer(source, offset, &target, 0, length);
            gpu.submit([encoder.finish()]);
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
            leaves.extend(view.as_chunks::<12>().0.iter().map(|b| {
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
    // A lost device is the cause of every error it raised (fn-235).
    if let Some(error) = gpu.lost() {
        return Err(error);
    }
    if let Some(error) = memory.or(validation) {
        return Err(self::error(error.to_string()));
    }
    Ok(())
}
/// Every program asked of `gpu` settled and `scopes` popped, the cause
/// reported as `programs::first_cause` orders it (fn-243).
pub(super) async fn programs_ready(gpu: &Gpu, scopes: [wgpu::ErrorScopeGuard; 2]) -> Result<()> {
    let ready = crate::programs::ready(gpu).await;
    let scoped = errors(gpu, scopes).await;
    crate::programs::first_cause(gpu.lost(), ready, scoped)
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

#[cfg(all(test, not(target_arch = "wasm32")))]
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
    #[cfg(test)]
    crate::trace::waited("stage");
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
        let Some(gpu) = crate::generation::test_gpu() else {
            return;
        };
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
