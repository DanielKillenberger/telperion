//! A buffer its owner lets go is destroyed after the submission that
//! binds it, never by another submission on the device (fn-212 task 11,
//! Astra's finding 1).
use super::{io, test_gpu};

#[test]
fn a_buffer_let_go_waits_for_its_own_encoder_not_another_submit() {
    let Some(gpu) = test_gpu() else {
        return;
    };
    let usage = wgpu::BufferUsages::COPY_SRC;
    let source = io::buffer(&gpu, "let go", 16, usage, &[7; 16]).unwrap();
    let usage = wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE;
    let target = io::buffer(&gpu, "kept", 16, usage, &[]).unwrap();
    let mut work = gpu.encoder(&Default::default());
    work.copy_buffer_to_buffer(&source, 0, &target, 0, 16);
    drop(source);
    // Another thread's frame, submitted while the work is still recorded.
    let other = gpu.encoder(&Default::default());
    gpu.submit([other.finish()]);
    let scopes = io::scope(&gpu);
    gpu.submit([work.finish()]);
    pollster::block_on(io::errors(&gpu, scopes)).expect("the work's own buffer was destroyed");
    assert_eq!(io::read(&gpu, &target, 16).unwrap(), [7; 16]);
}
