//! fn-209 F1 task 6: the CPU reference executor under realized mechanics
//! against today's whole-tree engine, by lineage and to the bit, at each
//! cut, in four orders and chunkings; and both timed, cold (the first
//! build in the process) and warm (the second).
//!
//!   cargo run --profile ci -p telperion-space --example realized -- [ages] [seeds] [cuts]
//!
//! Run under `RAYON_NUM_THREADS=1` for one core.
use std::time::Instant;
use telperion_space::{
    beech, digest, grow_fine, grow_whole, spruce, Expected, Fine, Light, Order, Plan, Request,
    Species,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let list = |i: usize, d: &str| -> Vec<f64> {
        let text = args.get(i).map_or(d, String::as_str);
        text.split(',')
            .map(|v| v.parse().expect("a number"))
            .collect()
    };
    let (ages, seeds, cuts) = (list(0, "40,80"), list(1, "1,7"), list(2, "256,1024,4096"));
    println!(
        "| Tree | Cut | Roots | Above (phytomers) | Phytomers | Rows ms | Whole cold / warm ms | Fine cold / warm ms | Phases warm ms | Trips | Equal |"
    );
    println!("|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|");
    for (name, species) in [("beech", beech()), ("spruce", spruce())] {
        for &age in &ages {
            for &seed in &seeds {
                let request = Request {
                    age: age as u32,
                    seed: seed as u64,
                    budget: 20_000_000,
                    light: Light::NEUTRAL,
                };
                let label = format!("{name} {age} {seed}");
                tree(&label, &species, request, &cuts);
            }
        }
    }
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1e3
}

fn tree(label: &str, species: &Species, request: Request, cuts: &[f64]) {
    let t = Instant::now();
    let expected = Expected::new(species, request.age, 0);
    let rows = ms(t);
    let whole = || {
        let t = Instant::now();
        let tree = grow_whole(species, request).expect("grows whole");
        (digest(&tree), tree.phytomer_count(), ms(t))
    };
    let (reference, phytomers, cold) = whole();
    let (_, _, warm) = whole();
    for &cut in cuts {
        let fine = |order, chunk| -> (Fine, f64) {
            let plan = Plan {
                cut,
                order,
                chunk,
                threads: 0,
            };
            let t = Instant::now();
            let fine = grow_fine(species, request, &expected, plan).expect("grows fine");
            (fine, ms(t))
        };
        let (first, fine_cold) = fine(Order::Forward, 16);
        let (second, fine_warm) = fine(Order::Forward, 16);
        let mut equal = [&first, &second]
            .iter()
            .all(|f| digest(&f.structure) == reference);
        for (order, chunk) in [
            (Order::Reverse, 1),
            (Order::Shuffled(3), 7),
            (Order::Forward, 64),
        ] {
            equal &= digest(&fine(order, chunk).0.structure) == reference;
        }
        let above: usize = second.structure.axes[..second.above]
            .iter()
            .map(|a| a.phytomers.len())
            .sum::<usize>()
            - second
                .roots
                .iter()
                .filter_map(|r| r.at)
                .map(|i| second.structure.axes[i].phytomers.len())
                .sum::<usize>();
        let phases: Vec<String> = second
            .phases
            .iter()
            .map(|s| format!("{:.0}", s * 1e3))
            .collect();
        println!(
            "| {label} | {cut} | {} | {above} | {phytomers} | {rows:.0} | {cold:.0} / {warm:.0} | {fine_cold:.0} / {fine_warm:.0} | {} | {} | {} |",
            second.roots.len(),
            phases.join(" / "),
            second.trips,
            if equal { "yes" } else { "NO" },
        );
    }
}
