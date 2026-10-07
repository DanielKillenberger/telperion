//! Per-pass GPU timestamps for measurement (fn-209 F1 task 8): while a
//! profile is on, every compute pass `io::Pass::run` records writes its
//! start and end to a query set, labelled by its entry point. Test-only;
//! no production pass is touched.
use crate::Gpu;
use std::cell::RefCell;

/// Queries in one set (wgpu's `QUERY_SET_MAX_QUERIES`).
const SET: u32 = wgpu::QUERY_SET_MAX_QUERIES;

struct Profile {
    device: wgpu::Device,
    sets: Vec<wgpu::QuerySet>,
    labels: Vec<String>,
}

thread_local! {
    static ON: RefCell<Option<Profile>> = const { RefCell::new(None) };
}

/// Starts recording every pass on this thread; false where the device
/// has no timestamps.
pub(crate) fn start(gpu: &Gpu) -> bool {
    let on = gpu
        .device
        .features()
        .contains(wgpu::Features::TIMESTAMP_QUERY);
    if on {
        ON.with_borrow_mut(|p| {
            *p = Some(Profile {
                device: gpu.device.clone(),
                sets: Vec::new(),
                labels: Vec::new(),
            })
        });
    }
    on
}

/// The query set and first index a pass labelled `label` writes, while a
/// profile is on.
pub(super) fn slot(label: &str) -> Option<(wgpu::QuerySet, u32)> {
    ON.with_borrow_mut(|p| {
        let p = p.as_mut()?;
        let at = 2 * p.labels.len() as u32;
        if at.is_multiple_of(SET) {
            p.sets
                .push(p.device.create_query_set(&wgpu::QuerySetDescriptor {
                    label: Some("profile"),
                    ty: wgpu::QueryType::Timestamp,
                    count: SET,
                }));
        }
        p.labels.push(label.to_string());
        Some((p.sets.last().expect("a set").clone(), at % SET))
    })
}

/// Ends the profile: each pass's label, start and end in nanoseconds, in
/// the order recorded.
pub(crate) fn finish(gpu: &Gpu) -> Vec<(String, f64, f64)> {
    let Some(p) = ON.with_borrow_mut(Option::take) else {
        return Vec::new();
    };
    let period = f64::from(gpu.queue.get_timestamp_period());
    let mut ticks = Vec::new();
    for (k, set) in p.sets.iter().enumerate() {
        let used = (2 * p.labels.len() as u32 - k as u32 * SET).min(SET);
        let bytes = u64::from(used) * 8;
        let usage = wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC;
        let resolved = super::io::buffer(gpu, "profile", bytes, usage, &[]).unwrap();
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        encoder.resolve_query_set(set, 0..used, &resolved, 0);
        gpu.queue.submit([encoder.finish()]);
        let raw = super::io::read(gpu, &resolved, bytes).unwrap();
        ticks.extend_from_slice(bytemuck::cast_slice::<u8, u64>(&raw));
    }
    p.labels
        .into_iter()
        .zip(ticks.as_chunks::<2>().0)
        .map(|(l, t)| (l, t[0] as f64 * period, t[1] as f64 * period))
        .collect()
}
