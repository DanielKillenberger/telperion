//! The scans of a recorded cycle (`plan.rs`), each over a count only the
//! device knows: `fine/scan.wgsl`'s three dispatches, its totals copied
//! where its readers look; and `hier_scan.wgsl`'s levels, recorded for
//! the most rows the buffer holds, a level past the count's own top
//! dispatched empty.
use super::super::io;
use super::plan::{Builder, PassId, Size, Step, MAXX, SLOT_BYTES};
use super::vm::E;
use crate::buffer::Owned;
use crate::Result;

const SPAN: u32 = 256;

fn zeroed(b: &Builder, label: &'static str, words: u64) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    io::buffer(b.gpu, label, words.max(4) * 4, usage, &[])
}

/// A fine scan's buffers, kept as long as the plan.
pub(super) struct Fine {
    _sums: Owned,
    _totals: Owned,
}

/// `fine/scan.wgsl` over `n` records `stride` words apart (at most `cap`
/// of them), their first `fields`, its totals copied to `to` at word
/// `w`; `pass` its module, `id` its plan name.
#[allow(clippy::too_many_arguments)]
pub(super) fn fine(
    b: &mut Builder,
    (pass, id): (&io::Pass, PassId),
    data: &Owned,
    (n, cap, stride, fields): (E, u32, u32, u32),
    (to, w): (&wgpu::Buffer, u64),
) -> Result<Fine> {
    let sums = zeroed(b, "plan scan sums", 4 * u64::from(cap.div_ceil(SPAN).max(1)))?;
    let totals = zeroed(b, "plan scan totals", 4)?;
    let slot = b.slot(&[0, stride, fields, 0, MAXX * 64, 0, 0, 0]);
    b.set(slot, 0, n.clone());
    b.set(slot, 3, n.clone().div(E::C(SPAN)));
    let blocks = b.groups(n.clone().div(E::C(SPAN)).div(E::C(64)));
    let spread = b.groups(n.div(E::C(64)));
    let bind = |b: &Builder| b.bind(pass, slot, &[data, &sums, &totals]);
    for (entry, size) in [(0, blocks), (1, Size::Direct(1)), (2, spread)] {
        let bound = bind(b);
        b.dispatch((id, entry), bound, size);
    }
    b.step(Step::Copy { from: (*totals).clone(), at: 0, to: to.clone(), to_at: 4 * w, bytes: 16 });
    Ok(Fine { _sums: sums, _totals: totals })
}

/// The level sizes a column of `rows` holds at most, from the first.
fn levels(rows: u32) -> Vec<u32> {
    let mut out = vec![rows];
    while *out.last().expect("a level") > SPAN {
        out.push(out.last().expect("a level").div_ceil(SPAN));
    }
    out
}

/// `hier_scan.wgsl` over `rows` words (at most `cap`) of `data` from word
/// `at`, its levels in `sums`, its total to `tally`'s word `to`.
pub(super) fn levelled(
    b: &mut Builder,
    pass: &io::Pass,
    (data, at, cap): (&Owned, u64, u32),
    sums: &Owned,
    (tally, to): (&Owned, u64),
    rows: E,
) {
    let most = levels(cap);
    let mut starts = vec![0u32; most.len() + 1];
    for l in 2..=most.len() {
        starts[l] = starts[l - 1] + most[l - 1];
    }
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
        let slot = b.slot(&[0, starts[l], starts[l + 1], u32::from(l == 0), 0, MAXX, 0, 0]);
        b.set(slot, 0, size(l));
        b.set(slot, 4, top(l));
        let up = b.groups(exists(l).sel(size(l).div(E::C(SPAN)).max(E::C(1)), E::C(0)));
        let down = b.groups(top(l).sel(E::C(0), size(l).div(E::C(64))));
        let data_bytes = 4 * u64::from(cap.max(1));
        binds.push((slice_bind(b, pass, slot, (data, 4 * at, data_bytes), sums, (tally, 4 * to)), up, down));
    }
    for (bind, up, _) in &binds {
        b.dispatch((PassId::HierScan, 0), bind.clone(), *up);
    }
    for (bind, _, down) in binds.iter().rev() {
        b.dispatch((PassId::HierScan, 1), bind.clone(), *down);
    }
}

/// A scan level's bind: its slot, a slice of the data, the sums and the
/// tally's 256 bytes from its total.
fn slice_bind(b: &Builder, pass: &io::Pass, slot: u32, (data, at, bytes): (&Owned, u64, u64), sums: &Owned, (tally, to): (&Owned, u64)) -> wgpu::BindGroup {
    fn slice(binding: u32, buffer: &wgpu::Buffer, offset: u64, size: u64) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding { buffer, offset, size: std::num::NonZeroU64::new(size) }),
        }
    }
    let uniform = &b.plan.uniforms;
    let entries = [
        slice(0, uniform, u64::from(slot) * SLOT_BYTES, SLOT_BYTES),
        slice(1, data, at, bytes),
        slice(2, sums, 0, sums.size()),
        slice(3, tally, to, 256),
    ];
    b.gpu.device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("plan scan"), layout: &pass.layout, entries: &entries })
}
