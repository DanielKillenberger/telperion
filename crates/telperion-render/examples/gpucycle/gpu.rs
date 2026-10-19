//! The spike's device: one adapter, kernels compiled and timed, passes
//! recorded one dispatch a pass with timestamps, and readbacks.
use std::time::Instant;
use wgpu::util::DeviceExt;

pub struct Gpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub name: String,
    pub align: u64,
}

pub fn gpu() -> Gpu {
    let instance =
        wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        ..Default::default()
    }))
    .expect("an adapter");
    let info = adapter.get_info();
    assert!(
        adapter.features().contains(wgpu::Features::TIMESTAMP_QUERY),
        "timestamps"
    );
    let mut limits = adapter.limits();
    limits.max_storage_buffers_per_shader_stage =
        limits.max_storage_buffers_per_shader_stage.min(32);
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("fn-256 spike"),
        required_features: wgpu::Features::TIMESTAMP_QUERY,
        required_limits: limits.clone(),
        ..Default::default()
    }))
    .expect("a device");
    device.set_device_lost_callback(|reason, message| panic!("device lost: {reason:?} {message}"));
    let name = format!("{} ({:?}, {})", info.name, info.backend, info.driver_info);
    let align = u64::from(limits.min_uniform_buffer_offset_alignment).max(64);
    Gpu {
        device,
        queue,
        name,
        align,
    }
}

/// A shader's kernels: binding 0 a uniform of `Params` at a dynamic
/// offset, bindings 1..=n storage buffers; the seconds they took to make.
pub struct Kit {
    layout: wgpu::BindGroupLayout,
    pipelines: Vec<(String, wgpu::ComputePipeline)>,
    pub compile: f64,
}

/// One dispatch's parameters.
#[repr(C)]
#[derive(Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Params {
    pub a: [u32; 8],
    pub f: [f32; 8],
}

impl Kit {
    pub fn pipeline(&self, entry: &str) -> &wgpu::ComputePipeline {
        &self
            .pipelines
            .iter()
            .find(|q| q.0 == entry)
            .expect("an entry")
            .1
    }
}

impl Gpu {
    /// The uniform every recording writes its dispatches' parameters to.
    pub fn uniform(&self, max_dispatches: usize) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (max_dispatches as u64 + 1) * self.align,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    pub fn kit(&self, source: &str, entries: &[&str], buffers: u32) -> Kit {
        let started = Instant::now();
        let mut entries_l = vec![wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: true,
                min_binding_size: None,
            },
            count: None,
        }];
        for b in 1..=buffers {
            entries_l.push(wgpu::BindGroupLayoutEntry {
                binding: b,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            });
        }
        let layout = self
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &entries_l,
            });
        let module = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(source.into()),
            });
        let pl = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[Some(&layout)],
                immediate_size: 0,
            });
        let pipelines = entries
            .iter()
            .map(|&e| {
                let p = self
                    .device
                    .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                        label: Some(e),
                        layout: Some(&pl),
                        module: &module,
                        entry_point: Some(e),
                        compilation_options: Default::default(),
                        cache: None,
                    });
                (e.to_string(), p)
            })
            .collect();
        let _ = self.device.poll(wgpu::PollType::wait_indefinitely());
        Kit {
            layout,
            pipelines,
            compile: started.elapsed().as_secs_f64(),
        }
    }

    pub fn bind(
        &self,
        kit: &Kit,
        uniform: &wgpu::Buffer,
        buffers: &[&wgpu::Buffer],
    ) -> wgpu::BindGroup {
        let mut entries = vec![wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: uniform,
                offset: 0,
                size: wgpu::BufferSize::new(std::mem::size_of::<Params>() as u64),
            }),
        }];
        for (i, b) in buffers.iter().enumerate() {
            entries.push(wgpu::BindGroupEntry {
                binding: i as u32 + 1,
                resource: b.as_entire_binding(),
            });
        }
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &kit.layout,
            entries: &entries,
        })
    }

    pub fn storage<T: bytemuck::Pod>(&self, data: &[T]) -> wgpu::Buffer {
        let bytes: &[u8] = bytemuck::cast_slice(data);
        let pad = [0u8; 4];
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: if bytes.is_empty() { &pad } else { bytes },
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_SRC
                    | wgpu::BufferUsages::COPY_DST,
            })
    }

    pub fn zeroed(&self, bytes: u64) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: bytes.max(4).next_multiple_of(4),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::INDIRECT,
            mapped_at_creation: false,
        })
    }

    pub fn read<T: bytemuck::Pod>(&self, source: &wgpu::Buffer, count: usize) -> Vec<T> {
        let bytes = (count * std::mem::size_of::<T>()) as u64;
        if bytes == 0 {
            return Vec::new();
        }
        let staging = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: bytes.next_multiple_of(4),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut e = self.device.create_command_encoder(&Default::default());
        e.copy_buffer_to_buffer(source, 0, &staging, 0, bytes.next_multiple_of(4));
        self.queue.submit([e.finish()]);
        staging
            .slice(..)
            .map_async(wgpu::MapMode::Read, |r| r.expect("mapped"));
        self.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("polled");
        let view = staging
            .slice(..)
            .get_mapped_range()
            .expect("a mapped range");
        bytemuck::cast_slice::<u8, T>(&view[..bytes as usize]).to_vec()
    }

    /// Submits and waits: the fenced wall clock in seconds.
    pub fn finish(&self, encoder: wgpu::CommandEncoder) -> f64 {
        let started = Instant::now();
        self.queue.submit([encoder.finish()]);
        self.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("polled");
        started.elapsed().as_secs_f64()
    }
}

/// A recording: every dispatch its own compute pass, its start and end
/// stamped, labelled by its phase; the uniform holds each dispatch's
/// parameters at its own aligned offset.
pub struct Recording {
    pub encoder: wgpu::CommandEncoder,
    pub sets: Vec<wgpu::QuerySet>,
    pub labels: Vec<String>,
    params: Vec<u8>,
    pub uniform: wgpu::Buffer,
    align: u64,
    pub dispatches: usize,
}

const SET: u32 = 4096;

pub struct Timed {
    /// Each dispatch's phase and seconds, in order; the first start to the
    /// last end.
    pub passes: Vec<(String, f64)>,
    pub span: f64,
}

impl Recording {
    pub fn new(gpu: &Gpu, uniform: &wgpu::Buffer) -> Self {
        let uniform = uniform.clone();
        Self {
            encoder: gpu.device.create_command_encoder(&Default::default()),
            sets: Vec::new(),
            labels: Vec::new(),
            params: Vec::new(),
            uniform,
            align: gpu.align,
            dispatches: 0,
        }
    }

    pub fn slot(&mut self, gpu: &Gpu, label: &str) -> (usize, u32) {
        let at = 2 * self.labels.len() as u32;
        if at % SET == 0 {
            self.sets
                .push(gpu.device.create_query_set(&wgpu::QuerySetDescriptor {
                    label: None,
                    ty: wgpu::QueryType::Timestamp,
                    count: SET,
                }));
        }
        self.labels.push(label.to_string());
        (self.sets.len() - 1, at % SET)
    }

    pub fn offset(&mut self, p: Params) -> u32 {
        let at = self.params.len() as u64;
        self.params.extend_from_slice(bytemuck::bytes_of(&p));
        self.params.resize((at + self.align) as usize, 0);
        at as u32
    }

    /// One dispatch of `entry` over `groups`, in its own stamped pass.
    pub fn run(
        &mut self,
        gpu: &Gpu,
        (kit, bind): (&Kit, &wgpu::BindGroup),
        entry: &str,
        label: &str,
        groups: [u32; 3],
        p: Params,
    ) {
        let offset = self.offset(p);
        let (set, at) = self.slot(gpu, label);
        let pipeline = &kit
            .pipelines
            .iter()
            .find(|q| q.0 == entry)
            .expect("an entry")
            .1;
        let mut pass = self
            .encoder
            .begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: Some(wgpu::ComputePassTimestampWrites {
                    query_set: &self.sets[set],
                    beginning_of_pass_write_index: Some(at),
                    end_of_pass_write_index: Some(at + 1),
                }),
            });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, bind, &[offset]);
        if groups.iter().all(|&g| g > 0) {
            pass.dispatch_workgroups(groups[0], groups[1], groups[2]);
        }
        self.dispatches += 1;
    }

    /// A dispatch of `entry` with its workgroup count read from `args` at
    /// `offset`, in its own stamped pass, or in `pass` where one is open.
    pub fn indirect(
        &mut self,
        gpu: &Gpu,
        (kit, bind): (&Kit, &wgpu::BindGroup),
        entry: &str,
        label: &str,
        (args, offset): (&wgpu::Buffer, u64),
        p: Params,
    ) {
        let uoff = self.offset(p);
        let (set, at) = self.slot(gpu, label);
        let pipeline = &kit
            .pipelines
            .iter()
            .find(|q| q.0 == entry)
            .expect("an entry")
            .1;
        let mut pass = self
            .encoder
            .begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: Some(wgpu::ComputePassTimestampWrites {
                    query_set: &self.sets[set],
                    beginning_of_pass_write_index: Some(at),
                    end_of_pass_write_index: Some(at + 1),
                }),
            });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, bind, &[uoff]);
        pass.dispatch_workgroups_indirect(args, offset);
        self.dispatches += 1;
    }

    /// Writes the parameters, resolves the stamps, submits and waits: the
    /// encoding seconds (from `begun`), the fenced wall clock and the
    /// stamped passes.
    pub fn submit(mut self, gpu: &Gpu, begun: Instant) -> (f64, f64, Timed) {
        let used = 2 * self.labels.len() as u64;
        let resolve = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (used * 8).max(8),
            usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        for (k, set) in self.sets.iter().enumerate() {
            let n = (used - k as u64 * u64::from(SET)).min(u64::from(SET)) as u32;
            self.encoder
                .resolve_query_set(set, 0..n, &resolve, k as u64 * u64::from(SET) * 8);
        }
        let encoding = begun.elapsed().as_secs_f64();
        if !self.params.is_empty() {
            gpu.queue.write_buffer(&self.uniform, 0, &self.params);
        }
        let wall = gpu.finish(self.encoder);
        let ticks: Vec<u64> = gpu.read(&resolve, used as usize);
        let period = f64::from(gpu.queue.get_timestamp_period()) * 1e-9;
        let passes: Vec<(String, f64)> = self
            .labels
            .iter()
            .enumerate()
            .map(|(k, l)| {
                (
                    l.clone(),
                    ticks[2 * k + 1].saturating_sub(ticks[2 * k]) as f64 * period,
                )
            })
            .collect();
        let span = match ticks.len() {
            0 => 0.0,
            n => ticks[n - 1].saturating_sub(ticks[0]) as f64 * period,
        };
        (encoding, wall, Timed { passes, span })
    }
}
