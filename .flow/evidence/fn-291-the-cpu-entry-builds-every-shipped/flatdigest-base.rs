//! fn-291 task 5: the base commit's flat wood per shipped preset at its own
//! seed, as one FNV-1a digest, and the longest run in vertices. It reads the
//! flat fields the base commit's `SurfaceMesh` had; `flatdigest.rs` is the
//! same digest over the chunks.
use telperion_core::{params, pipeline};

fn main() {
    for &(_, id, _, _) in params::CATALOGUE {
        let f = params::by_identity(id).unwrap();
        let req = pipeline::Request { wood: true, ..Default::default() };
        let built = match pipeline::build(&f, req) {
            Ok(b) => b,
            Err(e) => { println!("{id}: {e}"); continue; }
        };
        let w = built.outputs.wood.unwrap();
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
        let longest = w.run_table.iter().map(|r| r.index_count as usize / (segments * 6) * segments + 2).max().unwrap_or(0);
        println!("{id} seed {}: digest {hash:016x} vertices {} indices {} runs {} dropped {} longest-run-vertices {longest}",
            f.skeleton.seed, w.positions.len() / 3, w.indices.len(), w.run_table.len(), w.dropped);
    }
}
