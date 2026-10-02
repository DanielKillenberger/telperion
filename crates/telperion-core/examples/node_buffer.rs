//! Native half of the native/wasm node-buffer parity test. Wire order is slots 6/7.
use std::io::{self, Write};
use telperion_core::{params, pipeline};

fn main() {
    let id = std::env::args().nth(1).expect("family identity");
    let f = params::by_identity(&id).expect("known family");
    // A request for no output runs the skeleton stage alone.
    let tree = pipeline::build(&f, pipeline::Request::default())
        .unwrap()
        .skeleton
        .tree;
    let mut out = io::BufWriter::new(io::stdout().lock());
    for n in &tree.nodes {
        for value in [
            n.position.x,
            n.position.y,
            n.position.z,
            n.radius,
            n.start_radius,
            n.base_radius,
        ] {
            out.write_all(&value.to_le_bytes()).unwrap();
        }
    }
    for n in &tree.nodes {
        for value in [n.parent.unwrap_or(u32::MAX), n.branch, n.kind as u32] {
            out.write_all(&value.to_le_bytes()).unwrap();
        }
    }
}
