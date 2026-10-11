//! The scans of a recorded cycle (`plan.rs`), each over a count only the
//! device knows: `hier_scan.wgsl`'s levels, recorded for the most rows
//! the buffer holds, a level past the count's own top dispatched empty.
use super::super::io;
use super::gate::{Cond, Scan};
use super::plan::{Builder, PassId, MAXX, SLOT_BYTES};
use crate::buffer::Owned;
use super::vm::E;

const SPAN: u32 = 256;

/// The level sizes a column of `rows` holds at most, from the first.
fn levels(rows: u32) -> Vec<u32> {
    let mut out = vec![rows];
    while *out.last().expect("a level") > SPAN {
        out.push(out.last().expect("a level").div_ceil(SPAN));
    }
    out
}

/// The levels `rows` takes, as the device counts them: the first, and
/// each above it while the one below passes a span (`gate::levels_of`).
pub(super) fn levels_needed(rows: &E, most: usize) -> E {
    (1..most.max(1)).fold(E::C(1), |a, l| a.add(rows.clone().div(E::C(SPAN.pow(l as u32 - 1))).gt(E::C(SPAN))))
}

/// The words a scan's levels take in its sums: `fields` of each level
/// above the first of a column of `rows`.
pub(super) fn sums_words(rows: u32, fields: u32) -> u64 {
    levels(rows)[1..].iter().map(|&n| u64::from(n * fields)).sum::<u64>() + 4
}

/// The shape of a scan's first level: its records `stride` words apart,
/// their first `fields` scanned.
#[derive(Clone, Copy)]
pub(super) struct Shape {
    pub stride: u32,
    pub fields: u32,
}

/// One word a record.
pub(super) const COLUMN: Shape = Shape { stride: 1, fields: 1 };

/// `hier_scan.wgsl` over `rows` records (at most `cap`) of `data` from
/// word `at`, its levels in `sums`, its totals to `tally` from word `to`;
/// level `l` recorded where the cycle's limit for scans of `kind` allows.
pub(super) fn levelled(
    b: &mut Builder,
    (pass, kind): (&io::Pass, Scan),
    (data, at, cap, shape): (&Owned, u64, u32, Shape),
    sums: &Owned,
    (tally, to): (&wgpu::Buffer, u64),
    rows: E,
) {
    let most = levels(cap);
    let mut starts = vec![0u32; most.len() + 1];
    for l in 2..=most.len() {
        starts[l] = starts[l - 1] + shape.fields * most[l - 1];
    }
    // The tally bound from a 64-word boundary, its first total past it.
    let (base, out) = (to / 64 * 64, (to % 64) as u32);
    let span = |l: usize| E::C(SPAN.pow(l as u32));
    // Level `l` is the count's own while the level below is past a span.
    let exists = |l: usize| match l {
        0 => E::C(1),
        _ => rows.clone().div(span(l - 1)).gt(E::C(SPAN)),
    };
    let size = |l: usize| exists(l).sel(rows.clone().div(span(l)), E::C(0));
    let top = |l: usize| exists(l).sel(E::C(1).sub(size(l).gt(E::C(SPAN))), E::C(0));
    let mut binds = Vec::new();
    for l in 0..most.len() {
        let words = [0, starts[l], starts[l + 1], u32::from(l == 0), 0, MAXX, shape.stride, shape.fields, out, 0, 0, 0];
        let slot = b.slot(&words);
        b.set(slot, 0, size(l));
        b.set(slot, 4, top(l));
        let up = b.groups(exists(l).sel(size(l).div(E::C(SPAN)).max(E::C(1)), E::C(0)));
        let down = b.groups(top(l).sel(E::C(0), size(l).div(E::C(64))));
        let data_bytes = (4 * u64::from(cap.max(1)) * u64::from(shape.stride)).min(data.size() - 4 * at);
        let rest = [whole(sums), Slice(tally, 4 * base, Some(256))];
        binds.push((slice_bind(b, pass, slot, (data, 4 * at, data_bytes), &rest), up, down));
    }
    for (l, (bind, up, _)) in binds.iter().enumerate() {
        b.enter(Cond::Scan(kind, l as u32));
        b.dispatch((PassId::HierScan, 0), bind.clone(), *up);
        b.leave();
    }
    for (l, (bind, _, down)) in binds.iter().enumerate().rev() {
        b.enter(Cond::Scan(kind, l as u32));
        b.dispatch((PassId::HierScan, 1), bind.clone(), *down);
        b.leave();
    }
}

/// A buffer bound from a byte offset for a size, or to its end.
struct Slice<'a>(&'a wgpu::Buffer, u64, Option<u64>);

fn whole(b: &wgpu::Buffer) -> Slice<'_> {
    Slice(b, 0, None)
}

/// A scan's bind: its slot, a slice of the data, then `rest`.
fn slice_bind(b: &Builder, pass: &io::Pass, slot: u32, (data, at, bytes): (&Owned, u64, u64), rest: &[Slice]) -> wgpu::BindGroup {
    fn slice(binding: u32, buffer: &wgpu::Buffer, offset: u64, size: u64) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding { buffer, offset, size: std::num::NonZeroU64::new(size) }),
        }
    }
    let mut entries = vec![slice(0, &b.plan.uniforms, u64::from(slot) * SLOT_BYTES, SLOT_BYTES), slice(1, data, at, bytes)];
    for (k, s) in rest.iter().enumerate() {
        entries.push(slice(k as u32 + 2, s.0, s.1, s.2.unwrap_or(s.0.size() - s.1)));
    }
    b.gpu.device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("plan scan"), layout: &pass.layout, entries: &entries })
}
