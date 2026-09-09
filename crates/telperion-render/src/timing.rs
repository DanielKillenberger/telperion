//! What the vegetation costs the GPU, and whether that number may be believed.
//!
//! One timestamp pair is written around the vegetation pass - the base
//! timestamp feature only, never the native-only inside-pass ones, so the same
//! session runs in a browser. The readback is callback-driven for the same
//! reason: a browser's queue advances on its own and `poll` does nothing there.
use crate::device::{Gpu, RenderError, Result};
#[cfg(not(target_arch = "wasm32"))]
use crate::{Camera, Renderer};

/// Frames drawn before the timer is started at all: long enough for the driver
/// to have finished allocating the targets and for the clocks to have settled.
pub const CONDITIONING: usize = 8;
/// Timed frames thrown away, so the first sample kept is not the one that
/// filled the pipeline.
pub const WARMUP: usize = 8;
/// Timed frames the percentiles are taken over. Nothing else is counted.
pub const MEASURED: usize = 120;
/// How far above the median the tail may sit before the measurement is another
/// process's, not the tree's.
pub const CONTENTION_RATIO: f64 = 2.0;

/// Bytes of one timestamp pair.
const PAIR: u64 = 2 * wgpu::QUERY_SIZE as u64;

/// Whether a session's numbers may be read as a measurement. Every variant but
/// `Valid` says why, and a report carrying one has no percentile in it at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Every sample was a duration and the tail sits inside the ratio.
    Valid,
    /// There was nothing to measure with: no timestamp feature, or an adapter
    /// that does not draw.
    Unavailable(String),
    /// A sample was not a duration: non-finite, or a clock that ran backwards.
    Disjoint(String),
    /// The tail ran away from the median; something else had the GPU.
    Contended(String),
}

impl Verdict {
    /// The one word a record is filed under.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::Unavailable(_) => "unavailable",
            Self::Disjoint(_) => "disjoint",
            Self::Contended(_) => "contended",
        }
    }

    /// Why it is not valid, when it is not.
    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::Valid => None,
            Self::Unavailable(why) | Self::Disjoint(why) | Self::Contended(why) => Some(why),
        }
    }

    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid)
    }
}

/// The nearest-rank percentile of an already sorted, non-empty slice.
fn percentile(sorted: &[f64], fraction: f64) -> f64 {
    let rank = (fraction * sorted.len() as f64).ceil().max(1.0) as usize;
    sorted[rank.min(sorted.len()) - 1]
}

/// Judges a set of measured frame durations in milliseconds. This is the only
/// place a measurement is allowed to become valid.
pub fn judge(samples: &[f64]) -> Verdict {
    if samples.is_empty() {
        return Verdict::Disjoint("the session recorded no sample".into());
    }
    if let Some((index, sample)) = samples
        .iter()
        .enumerate()
        .find(|(_, sample)| !sample.is_finite() || **sample <= 0.0)
    {
        return Verdict::Disjoint(format!(
            "sample {index} read {sample}, which is not a duration"
        ));
    }
    let mut sorted = samples.to_vec();
    sorted.sort_by(f64::total_cmp);
    let (median, tail) = (percentile(&sorted, 0.5), percentile(&sorted, 0.95));
    if tail > median * CONTENTION_RATIO {
        return Verdict::Contended(format!(
            "the p95 frame took {:.1} times the median, above the limit of {CONTENTION_RATIO:.0}",
            tail / median
        ));
    }
    Verdict::Valid
}

/// Who did the measuring, in the adapter's own words for itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hardware {
    pub adapter: String,
    pub driver: String,
    pub backend: &'static str,
}

impl From<&wgpu::AdapterInfo> for Hardware {
    fn from(info: &wgpu::AdapterInfo) -> Self {
        Self {
            adapter: info.name.clone(),
            driver: format!("{} {}", info.driver, info.driver_info)
                .trim()
                .to_owned(),
            backend: info.backend.to_str(),
        }
    }
}

/// One measured session, in the terms a reader of the evidence needs. The
/// percentiles exist only on a valid verdict, so nothing in an invalid record
/// can be mistaken for a number that passed.
#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    pub hardware: Hardware,
    pub conditioning: usize,
    pub warmup: usize,
    pub measured: usize,
    /// Samples actually kept, which is `measured` unless the session stopped.
    pub samples: usize,
    verdict: Verdict,
    p50_ms: Option<f64>,
    p95_ms: Option<f64>,
}

impl Report {
    /// The report of a session that ran, judged by its own samples.
    pub fn measured(hardware: Hardware, samples: &[f64]) -> Self {
        let verdict = judge(samples);
        let percentiles = verdict.is_valid().then(|| {
            let mut sorted = samples.to_vec();
            sorted.sort_by(f64::total_cmp);
            (percentile(&sorted, 0.5), percentile(&sorted, 0.95))
        });
        Self {
            samples: samples.len(),
            p50_ms: percentiles.map(|(median, _)| median),
            p95_ms: percentiles.map(|(_, tail)| tail),
            verdict,
            ..Self::blank(hardware)
        }
    }

    /// The report of a session that could not run at all.
    pub fn unavailable(hardware: Hardware, reason: impl Into<String>) -> Self {
        Self {
            verdict: Verdict::Unavailable(reason.into()),
            ..Self::blank(hardware)
        }
    }

    fn blank(hardware: Hardware) -> Self {
        Self {
            hardware,
            conditioning: CONDITIONING,
            warmup: WARMUP,
            measured: MEASURED,
            samples: 0,
            verdict: Verdict::Valid,
            p50_ms: None,
            p95_ms: None,
        }
    }

    pub fn verdict(&self) -> &Verdict {
        &self.verdict
    }

    /// The median measured frame, milliseconds, on a valid session only.
    pub fn p50_ms(&self) -> Option<f64> {
        self.p50_ms
    }

    /// The p95 measured frame, milliseconds, on a valid session only.
    pub fn p95_ms(&self) -> Option<f64> {
        self.p95_ms
    }

    /// The record as it is committed to the evidence. Absent percentiles are
    /// absent keys, never zeroes.
    pub fn to_json(&self) -> String {
        let mut fields = vec![
            format!("\"adapter\": {}", quote(&self.hardware.adapter)),
            format!("\"driver\": {}", quote(&self.hardware.driver)),
            format!("\"backend\": {}", quote(self.hardware.backend)),
            format!("\"conditioning\": {}", self.conditioning),
            format!("\"warmup\": {}", self.warmup),
            format!("\"measured\": {}", self.measured),
            format!("\"samples\": {}", self.samples),
            format!("\"verdict\": {}", quote(self.verdict.name())),
        ];
        if let Some(reason) = self.verdict.reason() {
            fields.push(format!("\"reason\": {}", quote(reason)));
        }
        if let (Some(median), Some(tail)) = (self.p50_ms, self.p95_ms) {
            fields.push(format!("\"p50_ms\": {median:.4}"));
            fields.push(format!("\"p95_ms\": {tail:.4}"));
        }
        format!("{{\n  {}\n}}\n", fields.join(",\n  "))
    }
}

/// A JSON string literal. Adapter and driver names are the platform's words,
/// so they are escaped rather than trusted.
fn quote(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            control if control < ' ' => out.push_str(&format!("\\u{:04x}", control as u32)),
            ordinary => out.push(ordinary),
        }
    }
    out.push('"');
    out
}

/// The timestamp pair and the buffers that bring it back to the host.
pub struct Session {
    queries: wgpu::QuerySet,
    resolved: wgpu::Buffer,
    readback: wgpu::Buffer,
    /// Nanoseconds per timestamp tick, as the queue reports it.
    period: f32,
}

impl Session {
    /// A session on this device, or the reason there can be none.
    pub fn new(gpu: &Gpu) -> std::result::Result<Self, String> {
        if gpu.adapter.device_type == wgpu::DeviceType::Cpu {
            return Err(format!(
                "\"{}\" is a software adapter; its frame times are not a GPU's",
                gpu.adapter.name
            ));
        }
        if !gpu.timestamps {
            return Err("the adapter does not offer timestamp queries".into());
        }
        let queries = gpu.device.create_query_set(&wgpu::QuerySetDescriptor {
            label: Some("vegetation pass"),
            ty: wgpu::QueryType::Timestamp,
            count: 2,
        });
        // Resolving writes at a 256-byte aligned offset, so the destination is
        // taken at that granularity rather than at the pair's own size.
        let size = wgpu::QUERY_RESOLVE_BUFFER_ALIGNMENT;
        Ok(Self {
            queries,
            resolved: gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("timestamps"),
                size,
                usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            }),
            readback: gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("timestamps readback"),
                size: PAIR,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
            period: gpu.queue.get_timestamp_period(),
        })
    }

    /// The pair to write around the pass being measured.
    pub fn writes(&self) -> wgpu::RenderPassTimestampWrites<'_> {
        wgpu::RenderPassTimestampWrites {
            query_set: &self.queries,
            beginning_of_pass_write_index: Some(0),
            end_of_pass_write_index: Some(1),
        }
    }

    /// Queues the copy that brings the last pair back. Callback-driven from
    /// here on, which is what lets a browser await the same mapping.
    pub fn resolve(&self, gpu: &Gpu) {
        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("timestamps"),
            });
        encoder.resolve_query_set(&self.queries, 0..2, &self.resolved, 0);
        encoder.copy_buffer_to_buffer(&self.resolved, 0, &self.readback, 0, PAIR);
        gpu.queue.submit([encoder.finish()]);
    }

    /// The pair as a duration in milliseconds. A clock that ran backwards
    /// arrives here as a negative number and is refused by the verdict, not
    /// here: judging is one place, not two.
    fn duration_ms(&self, ticks: [u64; 2]) -> f64 {
        (ticks[1] as f64 - ticks[0] as f64) * f64::from(self.period) / 1e6
    }
}

/// The browser half of the readback. The same mapping callback the blocking
/// half waits on, awaited instead: a browser's queue advances on its own and
/// `poll` does nothing there.
#[cfg(target_arch = "wasm32")]
impl Session {
    /// What the last resolved pair cost, in milliseconds.
    pub async fn sample_ms(&self) -> Result<f64> {
        let lost = |reason: String| RenderError::DeviceLost { reason };
        let (sender, receiver) = futures_channel::oneshot::channel();
        self.readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                let _ = sender.send(result);
            });
        receiver
            .await
            .map_err(|_| lost("the timestamp readback never completed".into()))?
            .map_err(|error| lost(error.to_string()))?;

        let view = self
            .readback
            .slice(..)
            .get_mapped_range()
            .map_err(|error| lost(error.to_string()))?;
        let ticks = bytemuck::pod_read_unaligned::<[u64; 2]>(&view[..PAIR as usize]);
        drop(view);
        self.readback.unmap();
        Ok(self.duration_ms(ticks))
    }
}

/// The blocking half of the readback. A browser awaits the same callback
/// instead; `poll` is a no-op there, which is why it lives apart from the
/// queueing above.
#[cfg(not(target_arch = "wasm32"))]
impl Session {
    /// Draws one timed frame and returns what its vegetation pass cost.
    fn sample(
        &self,
        renderer: &mut Renderer,
        camera: &Camera,
        viewport: (u32, u32),
        colour: &wgpu::TextureView,
        depth: &wgpu::TextureView,
    ) -> Result<f64> {
        renderer.draw(camera, viewport, colour, depth, Some(self.writes()));
        self.resolve(renderer.gpu());
        Ok(self.duration_ms(self.read(renderer.gpu())?))
    }

    fn read(&self, gpu: &Gpu) -> Result<[u64; 2]> {
        let lost = |reason: String| RenderError::DeviceLost { reason };
        let (sender, receiver) = std::sync::mpsc::channel();
        self.readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                let _ = sender.send(result);
            });
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|error| gpu.lost().unwrap_or_else(|| lost(error.to_string())))?;
        receiver
            .recv()
            .map_err(|_| lost("the timestamp readback never completed".into()))?
            .map_err(|error| lost(error.to_string()))?;

        let view = self
            .readback
            .slice(..)
            .get_mapped_range()
            .map_err(|error| lost(error.to_string()))?;
        let ticks = bytemuck::pod_read_unaligned::<[u64; 2]>(&view[..PAIR as usize]);
        drop(view);
        self.readback.unmap();
        Ok(ticks)
    }
}

/// Runs the whole protocol on what is already submitted and reports it. An
/// adapter that cannot be timed is an unavailable report, never an error: a
/// missing measurement is a result, and the caller still has a record to file.
#[cfg(not(target_arch = "wasm32"))]
pub fn run(
    renderer: &mut Renderer,
    camera: &Camera,
    viewport: (u32, u32),
    colour: &wgpu::TextureView,
    depth: &wgpu::TextureView,
) -> Result<Report> {
    let hardware = Hardware::from(&renderer.gpu().adapter);
    let session = match Session::new(renderer.gpu()) {
        Ok(session) => session,
        Err(reason) => return Ok(Report::unavailable(hardware, reason)),
    };
    for _ in 0..CONDITIONING {
        renderer.draw(camera, viewport, colour, depth, None);
    }
    for _ in 0..WARMUP {
        session.sample(renderer, camera, viewport, colour, depth)?;
    }
    let mut samples = Vec::with_capacity(MEASURED);
    for _ in 0..MEASURED {
        samples.push(session.sample(renderer, camera, viewport, colour, depth)?);
    }
    Ok(Report::measured(hardware, &samples))
}
