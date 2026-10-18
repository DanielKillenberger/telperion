//! fn-291 task 4: the base commit's flat wood for one preset and seed,
//! built natively with wood and leaves requested as the CPU entry asks, and
//! written to stdout for `still.mjs native`: vertex and index counts as u32,
//! positions and normals as f32, indices as u32, all little-endian. Built
//! against a worktree of the pre-chunk base (2db376e1e).
//!   node still.mjs native <this binary> <preset> <seed> <out.png> <camera.json>
use std::io::Write;
use telperion_core::{params, pipeline};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut family = params::by_identity(&args[0]).expect("a shipped preset");
    family.skeleton.seed = args[1].parse().expect("a seed");
    let wood = {
        let request = pipeline::Request { wood: true, leaves: true, ..Default::default() };
        let built = pipeline::build(&family, request).expect("the base builds it natively");
        built.outputs.wood.expect("the wood was asked for")
    };
    let mut out = std::io::BufWriter::with_capacity(1 << 20, std::io::stdout().lock());
    let count = |n: usize| u32::try_from(n).expect("a count in u32").to_le_bytes();
    out.write_all(&count(wood.positions.len() / 3))?;
    out.write_all(&count(wood.indices.len()))?;
    for floats in [&wood.positions, &wood.normals] {
        for v in floats {
            out.write_all(&v.to_le_bytes())?;
        }
    }
    for i in &wood.indices {
        out.write_all(&i.to_le_bytes())?;
    }
    out.flush()
}
