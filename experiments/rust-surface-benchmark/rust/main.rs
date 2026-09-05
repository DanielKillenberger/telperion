use std::{fs, hint::black_box, time::Instant};
use surface_benchmark::{build, Mesh};
fn decode(bytes: &[u8]) -> Vec<f64> {
    bytes
        .chunks_exact(8)
        .map(|b| f64::from_le_bytes(b.try_into().unwrap()))
        .collect()
}
fn consume(mesh: Mesh) {
    black_box(mesh);
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3, "usage: surface-native INPUT.bin OUTPUT.bin");
    let bytes = fs::read(&args[1]).expect("read input");
    let input = decode(&bytes);
    for _ in 0..3 {
        consume(build(&input));
    }
    let mut compute = vec![];
    let mut caller = vec![];
    for _ in 0..10 {
        let t = Instant::now();
        let mesh = build(black_box(&input));
        compute.push(t.elapsed().as_secs_f64() * 1000.0);
        consume(mesh);
        let t = Instant::now();
        let owned = decode(black_box(&bytes));
        let mesh = build(&owned);
        caller.push(t.elapsed().as_secs_f64() * 1000.0);
        consume(mesh);
    }
    let mesh = build(&input);
    let mut output = Vec::with_capacity(8 + (mesh.positions.len() + mesh.indices.len()) * 4);
    output.extend((mesh.positions.len() as u32).to_le_bytes());
    output.extend((mesh.indices.len() as u32).to_le_bytes());
    for v in &mesh.positions {
        output.extend(v.to_le_bytes());
    }
    for v in &mesh.indices {
        output.extend(v.to_le_bytes());
    }
    fs::write(&args[2], output).unwrap();
    let status = fs::read_to_string("/proc/self/status").unwrap_or_default();
    let peak = status
        .lines()
        .find(|l| l.starts_with("VmHWM:"))
        .unwrap_or("unavailable");
    println!("{{\"computeMs\":{compute:?},\"callerMs\":{caller:?},\"peakRss\":{peak:?},\"outputBytes\":{}}}",(mesh.positions.len()+mesh.indices.len())*4);
}
