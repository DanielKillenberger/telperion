//! fn-256 task 4 review: a build's cap on the device ledger counts a
//! buffer let go until it is destroyed, so a move's new tables are held
//! to the cap beside its old while the copy encoder is open.
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
    let refused = io::buffer(&gpu, "new table", 2 * mib, usage, &[]).expect_err("the old one still counts");
    assert!(refused.to_string().contains("device budget"), "{refused}");
    drop(encoder);
    io::buffer(&gpu, "new table", 2 * mib, usage, &[]).expect("the old one destroyed");
}
