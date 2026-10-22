//! What a view's wood asked at its finest scale, read back a frame late,
//! and the capacity its budget grows to from it (fn-258, host decision
//! 2026-10-10): sized from the demand of the view, a quarter above it, and
//! grown, never shrunk, when a later view asks more, the way a growable
//! vector is. A frame past the capacity draws at the finest scale that
//! fits (`choose`), and the next frame has the room.
use crate::buffer::Owned;
use crate::device::Gpu;
use std::sync::{Arc, Mutex};

/// The headroom above what the view asked.
const HEADROOM: f64 = 1.25;

/// Where a copy of the totals' first four words stands.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Copy {
    /// Nothing in flight: the next surfacing may copy.
    #[default]
    Idle,
    /// Copied into the frame's encoder; to be mapped once it is submitted.
    Queued,
    /// Mapping; the flag turns once the words can be read.
    Mapping,
}

pub(super) struct Feedback {
    staging: Owned,
    copy: Copy,
    mapped: Arc<Mutex<Option<bool>>>,
    /// Rings, vertices, tube and ribbon indices the budget holds at most;
    /// a kind no view has asked for yet is held to nothing less than the
    /// whole budget.
    capacity: [Option<u64>; 4],
}

impl Feedback {
    pub(super) fn new(gpu: &Gpu) -> Self {
        Self {
            staging: Owned::create(
                gpu,
                &wgpu::BufferDescriptor {
                    label: Some("curve demand"),
                    size: 16,
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                    mapped_at_creation: false,
                },
            ),
            copy: Copy::Idle,
            mapped: Arc::default(),
            capacity: [None; 4],
        }
    }

    /// The bytes its readback holds on the device.
    pub(super) fn bytes(&self) -> u64 {
        self.staging.size()
    }

    /// The most of each kind the budget is held to: the whole budget for
    /// a kind no reading has seen.
    pub(super) fn capacity(&self) -> [u64; 4] {
        self.capacity.map(|c| c.unwrap_or(u64::MAX))
    }

    /// Copies the finest scale's counts out of `totals` after the passes,
    /// unless a copy is still in flight.
    pub(super) fn copy(&mut self, encoder: &mut wgpu::CommandEncoder, totals: &wgpu::Buffer) {
        if self.copy == Copy::Idle {
            encoder.copy_buffer_to_buffer(totals, 0, &self.staging, 0, 16);
            self.copy = Copy::Queued;
        }
    }

    /// Maps the copy the frame just submitted.
    pub(super) fn map(&mut self) {
        if self.copy != Copy::Queued {
            return;
        }
        self.copy = Copy::Mapping;
        let mapped = self.mapped.clone();
        self.staging
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                *mapped.lock().unwrap_or_else(|e| e.into_inner()) = Some(result.is_ok());
            });
    }

    /// Takes a reading where one arrived, growing the capacity to it.
    pub(super) fn read(&mut self) {
        let done = self.mapped.lock().unwrap_or_else(|e| e.into_inner()).take();
        let Some(ok) = done else { return };
        self.copy = Copy::Idle;
        if !ok {
            return;
        }
        let words: [u32; 4] = {
            let view = self.staging.slice(..).get_mapped_range();
            match view {
                Ok(view) => bytemuck::pod_read_unaligned(&view[..16]),
                Err(_) => return,
            }
        };
        self.staging.unmap();
        self.capacity = grow(self.capacity, words.map(u64::from));
    }
}

/// `capacity` grown to hold `demand` with headroom, kind by kind: a kind
/// is set by the first reading that asks for any of it and grows wherever
/// a later one asks past it. A kind no view has asked for says nothing of
/// what the next view will ask, so it stays unset and its whole budget
/// stands: a far view of ribbons alone leaves the tubes their room, and a
/// tree opened off screen its wood (Codex and Astra, fn-258).
fn grow(capacity: [Option<u64>; 4], demand: [u64; 4]) -> [Option<u64>; 4] {
    let roomy = |d: u64| (d as f64 * HEADROOM).ceil() as u64;
    std::array::from_fn(|k| match capacity[k] {
        _ if demand[k] == 0 => capacity[k],
        Some(c) if demand[k] <= c => Some(c),
        _ => Some(roomy(demand[k])),
    })
}

#[cfg(test)]
mod tests {
    use super::grow;

    #[test]
    fn the_capacity_opens_a_quarter_above_the_view_and_only_grows() {
        let opened = grow([None; 4], [100, 400, 8, 80]);
        assert_eq!(opened, [Some(125), Some(500), Some(10), Some(100)]);
        assert_eq!(grow(opened, [10, 40, 1, 8]), opened, "it shrank");
        let grown = [Some(125), Some(750), Some(10), Some(100)];
        assert_eq!(grow(opened, [120, 600, 4, 80]), grown);
    }

    /// Astra and Codex (fn-258): a kind no view had asked for learned a
    /// capacity of nothing, and the view that first asked for it left its
    /// wood out until the budget grew.
    #[test]
    fn a_kind_no_view_asked_for_keeps_its_whole_budget() {
        assert_eq!(grow([None; 4], [0; 4]), [None; 4], "an empty view set it");
        let far = grow([None; 4], [100, 400, 0, 80]);
        assert_eq!(far[2], None, "ribbons alone took the tubes' room");
        assert_eq!(grow(far, [100, 400, 30, 80])[2], Some(38));
    }
}
