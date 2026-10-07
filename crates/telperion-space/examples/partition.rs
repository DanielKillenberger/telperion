//! fn-211.4, probe (a): today's tree against the same tree with every
//! whole-tree reduction grouped as a partition at a cut takes it, the
//! state compared after every phase of every cycle; varied over cut, the
//! order fine roots' partials are combined in, and their chunking.
//!
//!   cargo run --profile ci -p telperion-space --example partition -- [ages] [seeds] [species]
use telperion_space::{
    beech, digest, grow, grow_partitioned, oak, spruce, Expected, Grouping, Light, PartitionOrder,
    PartitionRun, Request, Species,
};

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

/// Every run of the matrix: three cuts, each sum alone, then all three
/// in every order and chunking.
fn matrix() -> Vec<(String, PartitionRun)> {
    let every = |order, chunk| Grouping {
        order,
        chunk,
        sizes: true,
        leaves: true,
        girth: true,
    };
    let one = |sizes, leaves, girth| Grouping {
        order: PartitionOrder::Forward,
        chunk: 1,
        sizes,
        leaves,
        girth,
    };
    let mut runs = Vec::new();
    for cut in [256.0, 1024.0, 4096.0] {
        let mut add = |label: String, grouping| runs.push((label, PartitionRun { cut, grouping }));
        add(format!("cut {cut} sizes alone"), one(true, false, false));
        add(format!("cut {cut} leaves alone"), one(false, true, false));
        add(format!("cut {cut} girth alone"), one(false, false, true));
        for order in [
            PartitionOrder::Forward,
            PartitionOrder::Reverse,
            PartitionOrder::Shuffled(3),
        ] {
            for chunk in [1, 7, 64] {
                add(
                    format!("cut {cut} all, {order:?}, chunk {chunk}"),
                    every(order, chunk),
                );
            }
        }
    }
    runs
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let list = |i: usize, d: &str| -> Vec<u64> {
        let text = args.get(i).map_or(d, String::as_str);
        text.split(',')
            .map(|v| v.parse().expect("a number"))
            .collect()
    };
    let (ages, seeds) = (list(0, "40,80"), list(1, "1,7"));
    let only = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| "oak,beech,spruce".into());
    let all: [(&str, fn() -> Species); 3] = [("oak", oak), ("beech", beech), ("spruce", spruce)];
    let (labels, runs): (Vec<String>, Vec<PartitionRun>) = matrix().into_iter().unzip();
    for (name, make) in all.into_iter().filter(|s| only.contains(s.0)) {
        let species = make();
        for &age in &ages {
            let expected = Expected::new(&species, age as u32, 0);
            for &seed in &seeds {
                let request = Request {
                    age: age as u32,
                    seed,
                    budget: 20_000_000,
                    light: LIT,
                };
                let started = std::time::Instant::now();
                let probed = grow_partitioned(&species, request, &expected, &runs).expect("grows");
                let took = started.elapsed().as_secs_f64();
                let today = digest(&grow(&species, request).expect("grows"));
                println!(
                    "{name} {age} s{seed}: the reference is today's tree to the bit: {}; {} runs in one pass, {took:.1} s",
                    digest(&probed.structure) == today,
                    runs.len()
                );
                for (label, first) in labels.iter().zip(&probed.first) {
                    println!(
                        "{name} {age} s{seed} {label}: {}",
                        first.map_or("every sum the tree's to the bit".into(), |(c, ph)| {
                            format!("first sum difference cycle {c}, {ph}")
                        })
                    );
                }
            }
        }
    }
}
