//! fn-211.4's cost: today's whole oak against the stepped executor (cut
//! 1,024), per build and per growth year. One core is `taskset -c 0`.
//!
//!   cargo run --profile ci -p telperion-space --example stepcost -- [ages] [seeds] [repeats]
use std::time::Instant;
use telperion_space::{
    digest, grow_staged, grow_stepped, oak, Expected, Grouping, Light, PartitionOrder, Request,
    Stage,
};

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

/// Today's tree: its build, each year's seconds and the settle's.
fn today(request: Request) -> (u64, f64, Vec<f64>, f64) {
    let species = oak();
    let started = Instant::now();
    let mut ends: Vec<Instant> = Vec::new();
    let mut last = started;
    let mut grown = 0;
    let tree = grow_staged(&species, request, &mut |s| {
        if s == Stage::Grown {
            if grown > 0 {
                ends.push(last);
            }
            grown += 1;
        }
        last = Instant::now();
    })
    .expect("grows");
    let build = started.elapsed().as_secs_f64();
    // The last year ends at its growth; the settle runs from there.
    let mut years = Vec::new();
    let mut from = started;
    for &end in &ends {
        years.push((end - from).as_secs_f64());
        from = end;
    }
    let settled = build - (from - started).as_secs_f64();
    (digest(&tree), build, years, settled)
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
    let repeats = list(2, "1")[0];
    let grouping = Grouping {
        order: PartitionOrder::Forward,
        chunk: 1,
        sizes: true,
        leaves: true,
        girth: true,
    };
    let species = oak();
    for &age in &ages {
        let expected = Expected::new(&species, age as u32, 0);
        for &seed in &seeds {
            let request = Request {
                age: age as u32,
                seed,
                budget: 20_000_000,
                light: LIT,
            };
            for _ in 0..repeats {
                let (want, build, years, settled) = today(request);
                let started = Instant::now();
                let s =
                    grow_stepped(&species, request, &expected, 1024.0, grouping).expect("grows");
                let stepped = started.elapsed().as_secs_f64();
                let same = digest(&s.structure) == want;
                let worst = |y: &[f64]| {
                    let (k, t) =
                        y.iter()
                            .copied()
                            .enumerate()
                            .fold((0, 0.0), |a, b| if b.1 > a.1 { b } else { a });
                    format!("{:.0} ms (year {})", 1e3 * t, k + 1)
                };
                let median = |y: &[f64]| {
                    let mut v = y.to_vec();
                    v.sort_by(f64::total_cmp);
                    1e3 * v[v.len() / 2]
                };
                println!(
                    "oak {age} s{seed} on {} cores: today {build:.2} s (years {:.2} s, median {:.0} ms, worst {}; settle {settled:.2} s) | stepped {stepped:.2} s (years {:.2} s, median {:.0} ms, worst {}; assemble and settle {:.2} s; {} roots) | same tree: {same}",
                    std::thread::available_parallelism().map_or(1, |n| n.get()),
                    years.iter().sum::<f64>(),
                    median(&years),
                    worst(&years),
                    s.years.iter().sum::<f64>(),
                    median(&s.years),
                    worst(&s.years),
                    s.settle,
                    s.roots,
                );
            }
        }
    }
}
