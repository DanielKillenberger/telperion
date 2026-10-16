//! fn-291 task 5: the chunked wood per shipped preset at its own seed, laid
//! end to end by `flattened()` and digested exactly as `flatdigest-base.rs`
//! digests the base commit's flat wood (FNV-1a over every position, normal
//! and coord's bits, then every absolute index in draw order), with the
//! chunk count and the largest chunk. Built as `woodsize.rs` is, against
//! `flatdigest.Cargo.toml`; equal digest lines are the equality.
use telperion_core::{params, pipeline};

fn main() {
    for &(_, id, _, _) in params::CATALOGUE {
        let f = params::by_identity(id).unwrap();
        let req = pipeline::Request { wood: true, ..Default::default() };
        let built = match pipeline::build(&f, req) {
            Ok(b) => b,
            Err(e) => { println!("{id}: {e}"); continue; }
        };
        let chunked = built.outputs.wood.unwrap();
        let chunks = chunked.chunks.len();
        let largest = chunked.chunks.iter().map(|c| c.positions.len() / 3).max().unwrap_or(0);
        let w = chunked.flattened();
        let run_table: Vec<_> = chunked.run_table.iter().map(|r| r.span).collect();
        let mut hash = 14695981039346656037_u64;
        let mut take = |bytes: &[u8]| {
            for byte in bytes {
                hash = (hash ^ u64::from(*byte)).wrapping_mul(1099511628211);
            }
        };
        for floats in [&w.positions, &w.normals, &w.coords] {
            for v in floats { take(&v.to_bits().to_le_bytes()); }
        }
        for i in &w.indices { take(&i.to_le_bytes()); }
        let segments = f.surface.radial_segments.max(f.surface.lobes * 4) as usize;
        let longest = run_table.iter().map(|r| r.index_count as usize / (segments * 6) * segments + 2).max().unwrap_or(0);
        println!("{id} seed {}: digest {hash:016x} vertices {} indices {} runs {} dropped {} longest-run-vertices {longest}",
            f.skeleton.seed, w.positions.len() / 3, w.indices.len(), run_table.len(), chunked.dropped);
        println!("{id}: chunks {chunks} largest-chunk-vertices {largest}");
    }
}
