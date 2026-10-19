//! fn-291 task 5: the base commit's flat wood per shipped preset at its own
//! seed, as one FNV-1a digest, and the longest run in vertices. It reads the
//! flat fields the base commit's `SurfaceMesh` had; `flatdigest.rs` is the
//! same digest over the chunks.
use std::process::ExitCode;
use telperion_core::{params, pipeline};

/// `<seed,seed,...>` (the preset's own seed where `own`), then for the
/// chunked tool `--expect <base log>`: every digest line must be in it.
fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let seeds: Vec<Option<u32>> = args.first().map_or(vec![None], |s| {
        s.split(',').map(|s| s.parse().ok()).collect()
    });
    let expected = args.iter().position(|a| a == "--expect").map(|i| {
        std::fs::read_to_string(&args[i + 1]).expect("the base log")
    });
    let mut failed = false;
    for &(_, id, _, _) in params::CATALOGUE {
        for &seed in &seeds {
            let mut f = params::by_identity(id).unwrap();
            if let Some(seed) = seed {
                f.skeleton.seed = seed;
            }
            let req = pipeline::Request { wood: true, ..Default::default() };
            let built = match pipeline::build(&f, req) {
                Ok(b) => b,
                Err(e) => {
                    println!("{id} seed {}: FAILED {e}", f.skeleton.seed);
                    failed = true;
                    continue;
                }
            };
            let w = built.outputs.wood.unwrap();
            let mut hash = 14695981039346656037_u64;
            let mut take = |bytes: &[u8]| {
                for byte in bytes {
                    hash = (hash ^ u64::from(*byte)).wrapping_mul(1099511628211);
                }
            };
            for floats in [&w.positions, &w.normals, &w.coords] {
                for v in floats {
                    take(&v.to_bits().to_le_bytes());
                }
            }
            for i in &w.indices {
                take(&i.to_le_bytes());
            }
            let line = format!(
                "{id} seed {}: digest {hash:016x} vertices {} indices {}",
                f.skeleton.seed,
                w.positions.len() / 3,
                w.indices.len()
            );
            let matched = expected.as_ref().is_none_or(|e| e.lines().any(|l| l == line));
            println!("{line}{}", if matched { "" } else { " MISMATCH" });
            failed |= !matched;
        }
    }
    if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}
