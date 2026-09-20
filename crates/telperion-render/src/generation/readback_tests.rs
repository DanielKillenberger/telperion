use super::io;

#[test]
fn leaf_readback_preserves_words_across_chunk_boundaries() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let words: Vec<[u32; 3]> = (0..11).map(|i| [i, 0x80000000 | i, u32::MAX - i]).collect();
    let bytes: Vec<u8> = words
        .iter()
        .flatten()
        .flat_map(|w| w.to_ne_bytes())
        .collect();
    let source = io::buffer(
        &gpu,
        "readback fixture",
        bytes.len() as u64,
        wgpu::BufferUsages::COPY_SRC,
        &bytes,
    )
    .unwrap();
    for count in [0, 1, 4, 8, 11] {
        let actual = pollster::block_on(io::read_leaves_chunked(&gpu, &source, count, 48)).unwrap();
        assert_eq!(actual, words[..count]);
    }
}

#[test]
fn leaf_readback_rejects_invalid_buffers_and_allocation() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let source = io::buffer(
        &gpu,
        "invalid readback",
        24,
        wgpu::BufferUsages::COPY_SRC,
        &[],
    )
    .unwrap();
    assert!(pollster::block_on(io::read_leaves_chunked(&gpu, &source, 3, 48)).is_err());
    assert!(io::reserve_leaves(usize::MAX).is_err());
    assert!(pollster::block_on(io::read_leaves_chunked(&gpu, &source, 1, 11)).is_err());
    let wrong_usage =
        io::buffer(&gpu, "wrong usage", 24, wgpu::BufferUsages::UNIFORM, &[]).unwrap();
    assert!(pollster::block_on(io::read_leaves_chunked(&gpu, &wrong_usage, 1, 48)).is_err());
    source.destroy();
    assert!(
        pollster::block_on(io::read_leaves_chunked(&gpu, &source, 0, 0))
            .unwrap()
            .is_empty()
    );
    assert!(pollster::block_on(io::read_leaves_chunked(&gpu, &source, 1, 48)).is_err());
}

#[test]
fn leaf_readback_device_loss_returns_no_output() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let source = io::buffer(
        &gpu,
        "lost device readback",
        24,
        wgpu::BufferUsages::COPY_SRC,
        &[],
    )
    .unwrap();
    gpu.device.destroy();
    assert!(pollster::block_on(io::read_leaves_chunked(&gpu, &source, 1, 48)).is_err());
}
