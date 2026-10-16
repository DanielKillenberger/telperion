//! fn-256 task 4 review: a build's cap on the device ledger counts a
//! buffer let go until it is destroyed, so a move's new tables are held
//! to the cap beside its old while the copy encoder is open; and (task 5)
//! until the device has done the work submitted before its destroy, so a
//! dropped build's tables count beside the next's until it is polled free.
use super::io;

#[test]
fn a_buffer_let_go_counts_against_the_cap_until_it_is_destroyed() {
    let Some(gpu) = super::test_gpu() else {
        return;
    };
    let usage = wgpu::BufferUsages::STORAGE;
    let mib = 1 << 20;
    let _capped = gpu.ledger.capped(gpu.ledger.live() + 3 * mib);
    let old = io::buffer(&gpu, "old table", 2 * mib, usage, &[]).expect("under the cap");
    let encoder = gpu.encoder(&Default::default());
    drop(old);
    let refused =
        io::buffer(&gpu, "new table", 2 * mib, usage, &[]).expect_err("the old one still counts");
    assert!(refused.to_string().contains("device budget"), "{refused}");
    drop(encoder);
    let held =
        io::buffer(&gpu, "new table", 2 * mib, usage, &[]).expect_err("destroyed, not yet freed");
    assert!(held.to_string().contains("device budget"), "{held}");
    pollster::block_on(io::complete(&gpu)).expect("the device's work done");
    io::buffer(&gpu, "new table", 2 * mib, usage, &[]).expect("the old one freed");
}

/// fn-256 task 5 review: a buffer let go while one encoder is open stays
/// charged until that encoder closes, and is freed then, however many
/// other encoders open and close meanwhile.
#[test]
fn a_buffer_let_go_beside_other_encoders_is_freed_once_its_own_closes() {
    let Some(gpu) = super::test_gpu() else {
        return;
    };
    let usage = wgpu::BufferUsages::STORAGE;
    let mib = 1 << 20;
    let _capped = gpu.ledger.capped(gpu.ledger.live() + 3 * mib);
    let old = io::buffer(&gpu, "old table", 2 * mib, usage, &[]).expect("under the cap");
    let first = gpu.encoder(&Default::default());
    drop(old);
    let other = gpu.encoder(&Default::default());
    drop(other);
    drop(first);
    pollster::block_on(io::complete(&gpu)).expect("the device's work done");
    io::buffer(&gpu, "new table", 2 * mib, usage, &[]).expect("the old one freed");
}

/// fn-256 task 5 review: two builds on one device each keep their cap;
/// one ending lifts only its own.
#[test]
fn a_build_ending_leaves_another_builds_cap() {
    let Some(gpu) = super::test_gpu() else {
        return;
    };
    let usage = wgpu::BufferUsages::STORAGE;
    let mib = 1 << 20;
    let long = gpu.ledger.capped(gpu.ledger.live() + 3 * mib);
    let short = gpu.ledger.capped(gpu.ledger.live() + mib);
    drop(short);
    let refused = io::buffer(&gpu, "past the long build's cap", 4 * mib, usage, &[])
        .expect_err("the long build's cap holds");
    assert!(refused.to_string().contains("device budget"), "{refused}");
    drop(long);
    io::buffer(&gpu, "uncapped", 4 * mib, usage, &[]).expect("no cap");
}
