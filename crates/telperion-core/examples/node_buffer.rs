//! Native half of the native/wasm node-buffer parity test. Wire order is slots 6/7.
use std::io::{self, Read, Write};
use telperion_core::{params, pipeline, specimen::SpecimenStore};

fn main() {
    let id = std::env::args().nth(1).expect("family identity");
    let mut f = params::by_identity(&id).expect("known family");
    let age = std::env::args()
        .nth(2)
        .map(|age| age.parse::<f64>().expect("age"));
    let mode = std::env::args().nth(3).unwrap_or_default();
    let mut placements = Vec::new();
    let tree = if let Some(age) = age {
        f.age = age;
        // The growth path's own handle, at its default history.
        let mut store = SpecimenStore::default();
        let handle = if mode == "--import" {
            let mut bytes = Vec::new();
            io::stdin().read_to_end(&mut bytes).unwrap();
            store.import(&bytes).unwrap()
        } else {
            store.build(&f, 10_000.0).unwrap()
        };
        if mode == "--export" {
            io::stdout()
                .write_all(&store.snapshot(handle).unwrap())
                .unwrap();
            return;
        }
        let frontier = store.specimen(handle).unwrap().age();
        if age > frontier {
            store.advance(handle, age - frontier).unwrap();
        }
        let read = store.read(handle, Some(age)).unwrap();
        placements = read.placements;
        read.tree
    } else {
        // A request for no output runs the skeleton stage alone.
        pipeline::build(&f, pipeline::Request::default())
            .unwrap()
            .skeleton
            .tree
    };
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
    for placement in placements {
        for word in placement.leaf {
            out.write_all(&word.to_le_bytes()).unwrap();
        }
    }
}
