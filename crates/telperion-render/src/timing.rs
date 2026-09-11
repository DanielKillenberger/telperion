//! What the vegetation costs the GPU, and whether that number may be believed.
//!
//! Three timestamp pairs are written per timed frame: one around the vegetation
//! render pass, one around the selection compute pass that decides what it
//! draws, and one around the depth pass the sun writes its map in. All three
//! are pass-boundary writes under the base timestamp feature - never the
//! native-only inside-encoder ones - so the same session runs in a browser. The
//! readback is callback-driven for the same reason: a browser's queue advances
//! on its own and `poll` does nothing there.
mod report;

pub use report::{Hardware, LevelCount, Report};

// The targets a session draws into are the native half's: a page draws into
// its own canvas and the browser half never sees them.
#[cfg(not(target_arch = "wasm32"))]
use crate::{camera, Camera, Renderer, Target};
use crate::{
    device::{Gpu, RenderError, Result},
    Timed,
};

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
/// How many pairs a timed frame writes: vegetation, selection, shadow. The
/// vegetation pass stays first and the shadow pass is added after the two that
/// were already there, so a reader of the first sixteen bytes reads what
/// fn-22's records already meant by them.
const PASSES: u32 = 3;
const PAIRS: u64 = PASSES as u64 * PAIR;

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
pub(super) fn percentile(sorted: &[f64], fraction: f64) -> f64 {
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

/// The timestamp pairs and the buffers that bring them back to the host.
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
            label: Some("vegetation, selection and shadow passes"),
            ty: wgpu::QueryType::Timestamp,
            count: 2 * PASSES,
        });
        // Resolving writes at a 256-byte aligned offset, so the destination is
        // taken at that granularity rather than at the pairs' own size.
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
                size: PAIRS,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
            period: gpu.queue.get_timestamp_period(),
        })
    }

    /// The three pairs a timed frame writes, in the order the record reads
    /// them. A timed frame opens all three passes even when a view leaves one
    /// of them with nothing to do, so no query a resolve waits on is left
    /// unwritten.
    pub fn timed(&self) -> Timed<'_> {
        Timed {
            vegetation: wgpu::RenderPassTimestampWrites {
                query_set: &self.queries,
                beginning_of_pass_write_index: Some(0),
                end_of_pass_write_index: Some(1),
            },
            selection: wgpu::ComputePassTimestampWrites {
                query_set: &self.queries,
                beginning_of_pass_write_index: Some(2),
                end_of_pass_write_index: Some(3),
            },
            shadow: wgpu::RenderPassTimestampWrites {
                query_set: &self.queries,
                beginning_of_pass_write_index: Some(4),
                end_of_pass_write_index: Some(5),
            },
        }
    }

    /// Queues the copy that brings the last pairs back. Callback-driven from
    /// here on, which is what lets a browser await the same mapping.
    pub fn resolve(&self, gpu: &Gpu) {
        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("timestamps"),
            });
        encoder.resolve_query_set(&self.queries, 0..2 * PASSES, &self.resolved, 0);
        encoder.copy_buffer_to_buffer(&self.resolved, 0, &self.readback, 0, PAIRS);
        gpu.queue.submit([encoder.finish()]);
    }

    /// A pair as a duration in milliseconds. A clock that ran backwards
    /// arrives here as a negative number and is refused by the verdict, not
    /// here: judging is one place, not two.
    fn duration_ms(&self, ticks: [u64; 2]) -> f64 {
        (ticks[1] as f64 - ticks[0] as f64) * f64::from(self.period) / 1e6
    }

    /// One frame's resolved pairs as the three durations they stand for, in
    /// the order the query set writes them.
    fn durations(&self, ticks: [u64; 2 * PASSES as usize]) -> (f64, f64, f64) {
        (
            self.duration_ms([ticks[0], ticks[1]]),
            self.duration_ms([ticks[2], ticks[3]]),
            self.duration_ms([ticks[4], ticks[5]]),
        )
    }
}

/// The browser half of the readback. The same mapping callback the blocking
/// half waits on, awaited instead: a browser's queue advances on its own and
/// `poll` does nothing there.
#[cfg(target_arch = "wasm32")]
impl Session {
    /// What the last resolved frame's three passes cost, vegetation first,
    /// then selection, then the shadow, in milliseconds - the same three the
    /// blocking half returns, awaited instead of polled.
    pub async fn sample_ms(&self) -> Result<(f64, f64, f64)> {
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
        let ticks =
            bytemuck::pod_read_unaligned::<[u64; 2 * PASSES as usize]>(&view[..PAIRS as usize]);
        drop(view);
        self.readback.unmap();
        Ok(self.durations(ticks))
    }
}

/// The blocking half of the readback. A browser awaits the same callback
/// instead; `poll` is a no-op there, which is why it lives apart from the
/// queueing above.
#[cfg(not(target_arch = "wasm32"))]
impl Session {
    /// Draws one timed frame and returns what its vegetation pass, its
    /// selection pass and its shadow pass cost, in that order.
    fn sample(
        &self,
        renderer: &mut Renderer,
        camera: &Camera,
        viewport: (u32, u32),
        target: Target<'_>,
    ) -> Result<(f64, f64, f64)> {
        renderer.draw_timed(camera, viewport, target, self.timed());
        self.resolve(renderer.gpu());
        let ticks = self.read(renderer.gpu())?;
        Ok(self.durations(ticks))
    }

    fn read(&self, gpu: &Gpu) -> Result<[u64; 2 * PASSES as usize]> {
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
        let ticks =
            bytemuck::pod_read_unaligned::<[u64; 2 * PASSES as usize]>(&view[..PAIRS as usize]);
        drop(view);
        self.readback.unmap();
        Ok(ticks)
    }
}

/// Runs the whole protocol on what is already submitted and reports it, with
/// the camera held still. An adapter that cannot be timed is an unavailable
/// report, never an error: a missing measurement is a result, and the caller
/// still has a record to file.
#[cfg(not(target_arch = "wasm32"))]
pub fn run(
    renderer: &mut Renderer,
    camera: &Camera,
    viewport: (u32, u32),
    target: Target<'_>,
) -> Result<Report> {
    collect(renderer, viewport, target, |_| *camera, false)
}

/// The same protocol while the camera makes one full turn around the hero
/// pose, at its own elevation and distance. Conditioning and warmup run at the
/// start of the turn; the measured frames divide it evenly between them, and
/// the host's wait from one to the next is reported beside the GPU numbers.
#[cfg(not(target_arch = "wasm32"))]
pub fn orbit(
    renderer: &mut Renderer,
    hero: &Camera,
    viewport: (u32, u32),
    target: Target<'_>,
) -> Result<Report> {
    collect(
        renderer,
        viewport,
        target,
        |turn| camera::orbit_pose(hero, turn),
        true,
    )
}

/// One session: conditioning, warmup, then the measured frames, each posed by
/// `pose` at its own fraction of the way through. What comes back is the two
/// passes' costs, what the crown drew at each level, and - when the pose
/// moves - how long the host waited between frames.
#[cfg(not(target_arch = "wasm32"))]
fn collect(
    renderer: &mut Renderer,
    viewport: (u32, u32),
    target: Target<'_>,
    pose: impl Fn(f64) -> Camera,
    walls: bool,
) -> Result<Report> {
    let hardware = Hardware::from(&renderer.gpu().adapter);
    // What the frame was drawn at belongs to every record, measured or not: a
    // number is only comparable with another taken at the same count.
    let samples = renderer.samples();
    let session = match Session::new(renderer.gpu()) {
        Ok(session) => session,
        Err(reason) => return Ok(Report::unavailable(hardware, reason).with_multisample(samples)),
    };
    let start = pose(0.0);
    for _ in 0..CONDITIONING {
        renderer.draw(&start, viewport, target);
    }
    for _ in 0..WARMUP {
        session.sample(renderer, &start, viewport, target)?;
    }

    // Only a view that draws the crown runs selection, so only it has counters
    // worth reading; a bare or leaf session records the passes and no levels.
    let crown = renderer.view().selects();
    let deviations = renderer.level_deviations().to_vec();
    let mut vegetation = Vec::with_capacity(MEASURED);
    let mut selection = Vec::with_capacity(MEASURED);
    let mut shadow = Vec::with_capacity(MEASURED);
    let mut counted: Vec<Vec<u32>> = Vec::with_capacity(MEASURED);
    let mut wall = Vec::with_capacity(MEASURED);
    let mut previous: Option<std::time::Instant> = None;
    for frame in 0..MEASURED {
        let now = std::time::Instant::now();
        if let Some(last) = previous.replace(now) {
            wall.push((now - last).as_secs_f64() * 1e3);
        }
        let camera = pose(frame as f64 / MEASURED as f64);
        let (pass, select, sun) = session.sample(renderer, &camera, viewport, target)?;
        vegetation.push(pass);
        selection.push(select);
        shadow.push(sun);
        if let Some(counts) = crown.then(|| renderer.level_counts()).flatten() {
            counted.push(counts);
        }
    }

    let report = Report::measured(hardware, &vegetation)
        .with_multisample(samples)
        .with_passes(&vegetation, &selection, &shadow)
        .with_levels(&deviations, &counted);
    Ok(if walls {
        report.with_wall(&wall)
    } else {
        report
    })
}
