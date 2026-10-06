//! fn-211 task 1 probe (temporary): one tree grown `reps` times; per build
//! each pass's wall and spread time per cycle, as JSON lines.
//!   profile <species> <age> <seed> <reps>
use std::time::Instant;
use telperion_space::{beech, grow, oak, palm, prof, spruce, Light, Request};

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (name, age, seed, reps) = (
        a[0].as_str(),
        a[1].parse::<u32>().unwrap(),
        a[2].parse::<u64>().unwrap(),
        a[3].parse::<usize>().unwrap(),
    );
    let lit = Light { extinction: 0.5, sky: 0.5 };
    let (species, light) = match name {
        "beech" => (beech(), Light::NEUTRAL),
        "spruce" => (spruce(), Light::NEUTRAL),
        "oak" => (oak(), lit),
        "palm" => (palm(), Light::NEUTRAL),
        _ => panic!("species"),
    };
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    for rep in 0..reps {
        let load = std::fs::read_to_string("/proc/loadavg").unwrap();
        let load = load.split_whitespace().next().unwrap().to_string();
        let t = Instant::now();
        let tree = grow(&species, Request { age, seed, budget: 20_000_000, light }).expect("grows");
        let total = t.elapsed().as_nanos() as u64;
        let spans = prof::take();
        let kept = tree.phytomer_count();
        drop(tree);
        println!(
            "{{\"species\":\"{name}\",\"age\":{age},\"seed\":{seed},\"threads\":{threads},\"rep\":{rep},\"load\":{load},\"total_ns\":{total},\"kept\":{kept},\"spans\":[{}]}}",
            spans
                .iter()
                .map(|(c, n, w, p)| format!("[{c},\"{n}\",{w},{p}]"))
                .collect::<Vec<_>>()
                .join(",")
        );
    }
}
