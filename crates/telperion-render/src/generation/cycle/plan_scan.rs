//! The scans of a recorded cycle (`plan.rs`), each over a count only the
//! device knows: `hier_scan.wgsl`'s two dispatches, the second empty
//! where the count fits one block.
use super::super::io;
use super::hier_scan::SPAN;
use super::plan::{Builder, PassId, MAXX, SLOT_BYTES};
use super::vm::E;
use crate::buffer::Owned;

/// The shape of a scan's column: its records `stride` words apart, their
/// first `fields` scanned.
#[derive(Clone, Copy)]
pub(super) struct Shape {
    pub stride: u32,
    pub fields: u32,
}

/// One word a record.
pub(super) const COLUMN: Shape = Shape {
    stride: 1,
    fields: 1,
};

/// `hier_scan.wgsl` over `rows` records (at most `cap`) of `data` from
/// word `at`, the blocks' totals in `sums`, its totals to `tally` from
/// word `to`.
pub(super) fn scanned(
    b: &mut Builder,
    pass: &io::Pass,
    (data, at, cap, shape): (&Owned, u64, u32, Shape),
    sums: &Owned,
    (tally, to): (&wgpu::Buffer, u64),
    rows: E,
) {
    // The tally bound from a 64-word boundary, its first total past it.
    let (base, out) = (to / 64 * 64, (to % 64) as u32);
    let slot = b.slot(&[0, 0, MAXX, shape.stride, shape.fields, out, 0, 0]);
    b.set(slot, 0, rows.clone());
    b.set(slot, 6, E::C(1));
    let up = b.groups(rows.clone().div(E::C(SPAN)).max(E::C(1)));
    let down = b.groups(
        rows.clone()
            .gt(E::C(SPAN))
            .sel(rows.div(E::C(SPAN)), E::C(0)),
    );
    let data_bytes =
        (4 * u64::from(cap.max(1)) * u64::from(shape.stride)).min(data.size() - 4 * at);
    let rest = [whole(sums), Slice(tally, 4 * base, Some(256))];
    let bind = slice_bind(b, pass, slot, (data, 4 * at, data_bytes), &rest);
    b.dispatch((PassId::HierScan, 0), bind.clone(), up);
    b.dispatch((PassId::HierScan, 1), bind, down);
}

/// A buffer bound from a byte offset for a size, or to its end.
struct Slice<'a>(&'a wgpu::Buffer, u64, Option<u64>);

fn whole(b: &wgpu::Buffer) -> Slice<'_> {
    Slice(b, 0, None)
}

/// A scan's bind: its slot, a slice of the data, then `rest`.
fn slice_bind(
    b: &Builder,
    pass: &io::Pass,
    slot: u32,
    (data, at, bytes): (&Owned, u64, u64),
    rest: &[Slice],
) -> wgpu::BindGroup {
    fn slice(
        binding: u32,
        buffer: &wgpu::Buffer,
        offset: u64,
        size: u64,
    ) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer,
                offset,
                size: std::num::NonZeroU64::new(size),
            }),
        }
    }
    let mut entries = vec![
        slice(
            0,
            &b.plan.uniforms,
            u64::from(slot) * SLOT_BYTES,
            SLOT_BYTES,
        ),
        slice(1, data, at, bytes),
    ];
    for (k, s) in rest.iter().enumerate() {
        entries.push(slice(
            k as u32 + 2,
            s.0,
            s.1,
            s.2.unwrap_or(s.0.size() - s.1),
        ));
    }
    b.gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("plan scan"),
        layout: &pass.layout,
        entries: &entries,
    })
}
