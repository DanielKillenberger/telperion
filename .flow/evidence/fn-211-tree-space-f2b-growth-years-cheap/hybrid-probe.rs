//! fn-211.6 probe: the stepped executor's phases per cycle (HYBRID=1),
//! for the lit oak. Not product code; removed after the runs.
//!
//!   HYBRID=1 cargo run --profile ci -p telperion-space --example hybrid -- age seed repeats
use std::time::Instant;
use telperion_space::{grow_stepped, oak, Expected, Grouping, Light, PartitionOrder, Request};

fn main() {
    let a: Vec<u64> = std::env::args()
        .skip(1)
        .map(|v| v.parse().expect("a number"))
        .collect();
    let (age, seed, repeats) = (a[0] as u32, a[1], a[2]);
    let species = oak();
    let expected = Expected::new(&species, age, 0);
    let grouping = Grouping {
        order: PartitionOrder::Forward,
        chunk: 1,
        sizes: true,
        leaves: true,
        girth: true,
    };
    let request = Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light {
            extinction: 0.5,
            sky: 0.5,
        },
    };
    for rep in 0..repeats {
        println!("REP {rep}");
        let started = Instant::now();
        let s = grow_stepped(&species, request, &expected, 1024.0, grouping).expect("grows");
        println!(
            "END {{\"rep\":{rep},\"build\":{:.4},\"years\":{:.4},\"settle\":{:.4},\"roots\":{},\"fine\":{},\"coarse\":{}}}",
            started.elapsed().as_secs_f64(),
            s.years.iter().sum::<f64>(),
            s.settle,
            s.roots,
            s.fine_grown,
            s.coarse_grown
        );
    }
}
