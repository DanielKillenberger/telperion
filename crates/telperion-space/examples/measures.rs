//! The engine's measures for the passed species (fn-197).
//!
//!   cargo run --release -p telperion-space --example measures -- <what> [age] [seeds] [species]
//!
//! - `hash`: each tree's hash over every public field, to the bit, without
//!   light: the proof that a change left every species as it was.
//! - `stages`: each tree's time per stage (`telperion_space::Stage`), without
//!   light and under `LIT`.
//! - `gap`: under `LIT`, the oak's and the beech's rough layout grown with
//!   the tree against their final lay: crown extent, how far each phytomer
//!   stands from its final place, and the light at each living bud.
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::Instant;
use telperion_space::{
    beech, bud_light, grow, grow_staged, oak, sketch, spruce, Light, Request, Species, Stage,
    Structure, Vec3,
};

const BUDGET: u32 = 20_000_000;
/// The light the measures are taken under: leaves at every angle, under
/// the standard overcast sky, and from overhead.
const LIT: [Light; 2] = [
    Light {
        extinction: 0.5,
        sky: 0.5,
    },
    Light {
        extinction: 0.5,
        sky: 0.0,
    },
];
const SPECIES: [(&str, fn() -> Species); 3] = [("beech", beech), ("spruce", spruce), ("oak", oak)];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let what = args.first().map_or("hash", String::as_str);
    let age: u32 = args.get(1).map_or(80, |a| a.parse().expect("age"));
    let seeds: Vec<u64> = args.get(2).map_or(vec![1, 7], |s| {
        s.split(',').map(|v| v.parse().expect("seed")).collect()
    });
    let request = |seed, light| Request {
        age,
        seed,
        budget: BUDGET,
        light,
    };
    let only = args.get(3);
    for (name, make) in SPECIES {
        if only.is_some_and(|o| o != name) {
            continue;
        }
        let species = make();
        for &seed in &seeds {
            match what {
                "hash" => {
                    let started = Instant::now();
                    let tree = grow(&species, request(seed, Light::NEUTRAL)).expect("grows");
                    println!(
                        "{name} age {age} seed {seed}: hash {:016x}, {} phytomers, grown in {:.2} s",
                        hash(&tree),
                        tree.phytomer_count(),
                        started.elapsed().as_secs_f64()
                    );
                }
                "stages" => {
                    for light in [Light::NEUTRAL, LIT[0]] {
                        stages(name, &species, request(seed, light));
                    }
                }
                "gap" if name != "spruce" => {
                    // And with every sag at 0: the share of the gap that
                    // is sag's.
                    let mut unsagged = species.clone();
                    unsagged.states.iter_mut().for_each(|s| s.form.sag = 0.0);
                    for light in LIT {
                        gap(name, &species, request(seed, light));
                        if unsagged != species {
                            gap(
                                &format!("{name} without sag"),
                                &unsagged,
                                request(seed, light),
                            );
                        }
                    }
                }
                "gap" => {}
                _ => panic!("measures hash | stages | gap [age] [seeds]"),
            }
        }
    }
}

/// The time each stage took, summed over the cycles.
fn stages(name: &str, species: &Species, request: Request) {
    let order = [
        Stage::Grown,
        Stage::Sketched,
        Stage::Relaid,
        Stage::Lit,
        Stage::Settled,
        Stage::Laid,
    ];
    let mut spent = [0.0; 6];
    let mut last = Instant::now();
    let started = last;
    grow_staged(species, request, &mut |stage| {
        let now = Instant::now();
        let i = order.iter().position(|&s| s == stage).unwrap();
        spent[i] += (now - last).as_secs_f64();
        last = now;
    })
    .expect("grows");
    let total = started.elapsed().as_secs_f64();
    println!(
        "{name} age {} seed {} extinction {} sky {}: total {total:.2} s; growth {:.2}, rough layout {:.2}, full re-lays {:.2}, light {:.2}, settle {:.2}, final lay {:.2}; peak memory so far {}",
        request.age, request.seed, request.light.extinction, request.light.sky,
        spent[0], spent[1], spent[2], spent[3], spent[4], spent[5], peak()
    );
}

/// The process's peak resident memory, where Linux reports it.
fn peak() -> String {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM"))
                .map(|l| l[6..].trim().to_string())
        })
        .unwrap_or_else(|| "unknown".into())
}

/// The rough layout against the final lay.
fn gap(name: &str, species: &Species, request: Request) {
    let laid = grow(species, request).expect("grows");
    let rough = sketch(species, request).expect("grows");
    let (l, r) = (extent(&laid), extent(&rough));
    let mut moved: Vec<f64> = laid
        .axes
        .iter()
        .zip(&rough.axes)
        .flat_map(|(a, b)| a.phytomers.iter().zip(&b.phytomers))
        .map(|(p, q)| (p.tip - q.tip).length())
        .collect();
    moved.sort_by(f64::total_cmp);
    let q = |f: f64| moved[((moved.len() - 1) as f64 * f) as usize];
    let final_light = bud_light(&laid, species, &request.light);
    let rough_light = bud_light(&rough, species, &request.light);
    let n = final_light.len() as f64;
    let mean = |v: &[(usize, f64)]| v.iter().map(|x| x.1).sum::<f64>() / n;
    let (mf, mr) = (mean(&final_light), mean(&rough_light));
    let (mut cov, mut vf, mut vr, mut diff, mut far) = (0.0, 0.0, 0.0, 0.0, 0usize);
    for (a, b) in final_light.iter().zip(&rough_light) {
        let (x, y) = (a.1 - mf, b.1 - mr);
        cov += x * y;
        vf += x * x;
        vr += y * y;
        diff += (a.1 - b.1).abs();
        far += usize::from((a.1 - b.1).abs() > 0.1);
    }
    println!(
        "{name} age {} seed {} sky {}: extent final {:.1} x {:.1} x {:.1} m, rough {:.1} x {:.1} x {:.1} m; \
         phytomers from their final place: median {:.2} m, 90th {:.2} m, 99th {:.2} m; \
         {} living buds, light final {mf:.3}, rough {mr:.3}, mean |difference| {:.3}, \
         correlation {:.3}, {:.1}% differ by more than 0.1",
        request.age, request.seed, request.light.sky,
        l.x, l.y, l.z, r.x, r.y, r.z,
        q(0.5), q(0.9), q(0.99),
        final_light.len(),
        diff / n,
        cov / (vf * vr).sqrt(),
        100.0 * far as f64 / n
    );
}

/// The crown's extent: x and y across, and height.
fn extent(tree: &Structure) -> Vec3 {
    let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
    for p in tree.axes.iter().flat_map(|a| &a.phytomers) {
        for (i, c) in [p.tip.x, p.tip.y, p.tip.z].into_iter().enumerate() {
            lo[i] = lo[i].min(c);
            hi[i] = hi[i].max(c);
        }
    }
    Vec3::new(hi[0] - lo[0], hi[1] - lo[1], hi[2])
}

/// Every public field of the tree, to the bit.
fn hash(tree: &Structure) -> u64 {
    let mut h = DefaultHasher::new();
    let v = |h: &mut DefaultHasher, v: Vec3| [v.x, v.y, v.z].map(|c| c.to_bits()).hash(h);
    (tree.age, tree.pas).hash(&mut h);
    for axis in &tree.axes {
        (axis.lineage, axis.pa, axis.birth, axis.apex_end).hash(&mut h);
        format!("{:?}", axis.origin).hash(&mut h);
        axis.vigour.to_bits().hash(&mut h);
        v(&mut h, axis.base);
        v(&mut h, axis.heading);
        v(&mut h, axis.side);
        for p in &axis.phytomers {
            (p.cycle, p.radius.to_bits(), p.scale.to_bits()).hash(&mut h);
            v(&mut h, p.tip);
            v(&mut h, p.heading);
            v(&mut h, p.side);
        }
    }
    h.finish()
}
