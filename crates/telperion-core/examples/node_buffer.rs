//! Native half of the native/wasm node-buffer parity test. Wire order is slots 6/7.
use std::io::{self, Read, Write};
use telperion_core::{branching, params};

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
        let mut specimen = if mode == "--import" {
            let mut bytes = Vec::new();
            io::stdin().read_to_end(&mut bytes).unwrap();
            branching::Specimen::from_snapshot(&bytes).unwrap()
        } else {
            branching::Specimen::build(&f).unwrap()
        };
        if mode == "--export" {
            io::stdout()
                .write_all(&specimen.snapshot().unwrap())
                .unwrap();
            return;
        }
        if age > specimen.age() {
            specimen.advance(age - specimen.age()).unwrap();
        }
        let read = specimen.read_at_age(age).unwrap();
        placements = read.placements;
        read.tree
    } else {
        branching::generate(&f.skeleton, f.radii).unwrap().tree
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
        for value in placement.transform {
            out.write_all(&value.to_le_bytes()).unwrap();
        }
    }
}
