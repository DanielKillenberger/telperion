//! fn-257 R2: the field grown on the device against the CPU's reference
//! field at the same seed, every shipped preset at its age: each cell of
//! a grid over the reference's box at 24, 36 and 48 cells a side, its
//! wood, its thickest radius, its foliage, its leaves and its limb. The
//! limbs are compared as a partition: the device names a system by its
//! own node ids, so each device system is read as the reference system
//! it shares the most cells with. `FIELD_PRESETS` names a comma list,
//! `FIELD_SEEDS` the seeds, `FIELD_SIDES` the grids.
use super::super::{Clock, Generator};
use telperion_core::math::Vec3;
use telperion_core::pipeline::{self, Request};
use std::collections::HashMap;

/// One comparison's measures (`R2`).
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct Measures {
    pub wood: f64,
    pub radius: f64,
    pub foliage: f64,
    pub leaves: f64,
    pub total: f64,
    pub limbs: f64,
}

fn jaccard(a: impl Iterator<Item = (bool, bool)>) -> f64 {
    let (mut both, mut either) = (0u64, 0u64);
    for (x, y) in a {
        both += u64::from(x && y);
        either += u64::from(x || y);
    }
    if either == 0 {
        1.0
    } else {
        both as f64 / either as f64
    }
}

pub(super) fn compare(
    cpu: &[telperion_core::field::Occupancy],
    dev: &[super::Answer],
) -> Measures {
    let pairs = || cpu.iter().zip(dev);
    let wood = jaccard(pairs().map(|(c, d)| (c.wood, d.wood)));
    let foliage = jaccard(pairs().map(|(c, d)| (c.foliage, d.foliage)));
    // The share of the cells both find wood in whose thickest radius
    // differs by more than a percent.
    let (mut off, mut both) = (0u64, 0u64);
    for (c, d) in pairs().filter(|(c, d)| c.wood && d.wood) {
        let e = (c.wood_radius - f64::from(d.wood_radius)).abs() / c.wood_radius.max(1e-6);
        both += 1;
        off += u64::from(e > 0.01);
    }
    let radius = if both == 0 { 0.0 } else { off as f64 / both as f64 };
    let (sum_c, sum_d): (f64, f64) = pairs().fold((0.0, 0.0), |(a, b), (c, d)| {
        (a + c.leaves, b + f64::from(d.leaves))
    });
    let mut diff = 0.0;
    for (c, d) in pairs() {
        diff += (c.leaves - f64::from(d.leaves)).abs();
    }
    let leaves = if sum_c > 0.0 { diff / sum_c } else { 0.0 };
    let total = if sum_c > 0.0 { sum_d / sum_c } else { 1.0 };
    // Each device system read as the reference system it shares the most
    // cells with.
    let mut shared: HashMap<u32, HashMap<u32, u64>> = HashMap::new();
    for (c, d) in pairs() {
        if let (Some(c), Some(d)) = (c.limb, d.limb) {
            *shared.entry(d).or_default().entry(c).or_default() += 1;
        }
    }
    let read: HashMap<u32, u32> = shared
        .iter()
        .map(|(d, m)| (*d, *m.iter().max_by_key(|(c, n)| (**n, std::cmp::Reverse(**c))).unwrap().0))
        .collect();
    let (mut agree, mut held) = (0u64, 0u64);
    for (c, d) in pairs() {
        if c.limb.is_some() || d.limb.is_some() {
            held += 1;
            agree += u64::from(c.limb.is_some() && d.limb.and_then(|d| read.get(&d).copied()) == c.limb);
        }
    }
    let limbs = if held == 0 { 1.0 } else { agree as f64 / held as f64 };
    Measures {
        wood,
        radius,
        foliage,
        leaves,
        total,
        limbs,
    }
}

/// A grid of `n` cells a side over `b`, its cells cubes of the widest
/// side's share.
pub(super) fn grid(b: &telperion_core::foliage::Bounds, n: u32) -> Vec<(Vec3, f64)> {
    let size = (b.max - b.min) * (1.0 / f64::from(n));
    let half = size.x.max(size.y).max(size.z) * 0.5;
    let mut out = Vec::with_capacity((n * n * n) as usize);
    for k in 0..n {
        for j in 0..n {
            for i in 0..n {
                let c = Vec3::new(
                    b.min.x + size.x * (f64::from(i) + 0.5),
                    b.min.y + size.y * (f64::from(j) + 0.5),
                    b.min.z + size.z * (f64::from(k) + 0.5),
                );
                out.push((c, half));
            }
        }
    }
    out
}

/// The tolerances R2 holds the device's field to, from the measured runs
/// (`FIELD.md`): every cell's wood and foliage, its thickest radius within
/// a percent, the leaves, the limbs as the same partition. A frond's
/// leaflets are drawn on each executor's own draws, as placement's are
/// (`every_rosette_is_placed_as_the_cpu_places_it`), so a crown of fronds
/// is held to where its draws allow.
pub(super) fn within(m: &Measures, fronds: bool) -> Result<(), String> {
    let (foliage, leaves, total, limbs) = match fronds {
        false => (0.999, 0.002, 0.001, 0.95),
        true => (0.70, 0.40, 0.02, 0.70),
    };
    let held = m.wood >= 0.999
        && m.radius <= 0.005
        && m.foliage >= foliage
        && m.leaves <= leaves
        && (m.total - 1.0).abs() <= total
        && m.limbs >= limbs;
    held.then_some(()).ok_or_else(|| format!("{m:?}"))
}

/// R2 on every shipped preset at seed 1, on a grid of 24 cells a side:
/// the device's field against the CPU's (the gate's; the measurement
/// below runs seeds, sides and timings).
#[test]
fn every_preset_s_device_field_answers_within_its_tolerance() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let gen = pollster::block_on(Generator::for_fields_async(gpu.clone())).expect("a generator");
    for &(_, id, ..) in telperion_core::presets::CATALOGUE {
        let mut family = telperion_core::presets::by_identity(id).expect("a preset");
        family.skeleton.seed = 1;
        let request = Request {
            field: Some(None),
            ..Request::default()
        };
        let reference = pipeline::build(&family, request).expect("the reference");
        let reference = reference.outputs.field.expect("a field");
        let field = pollster::block_on(gen.grow_field(&family, None)).expect("grown");
        let cells = grid(&reference.bounds().expect("bounds"), 24);
        let cpu: Vec<_> = cells.iter().map(|(c, h)| reference.query(*c, *h).expect("answered")).collect();
        let words: Vec<f32> = cells
            .iter()
            .flat_map(|(c, h)| [c.x as f32, c.y as f32, c.z as f32, *h as f32])
            .collect();
        let dev = pollster::block_on(gen.query_field(&field, &words)).expect("answered");
        let fronds = family.canopy.rosette_fronds > 0.0;
        within(&compare(&cpu, &dev), fronds).unwrap_or_else(|m| panic!("{id}: {m}"));
    }
}

/// A query the CPU's field refuses is refused whole, and an id the
/// catalogue does not serve is refused by name.
#[test]
fn the_device_field_refuses_what_the_core_refuses() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let gen = pollster::block_on(Generator::for_fields_async(gpu.clone())).expect("a generator");
    let family = telperion_core::presets::by_identity("european-beech").expect("a preset");
    let field = pollster::block_on(gen.grow_field(&family, None)).expect("grown");
    for cells in [&[0.0, 1.0, 0.0, -0.1][..], &[f32::NAN, 1.0, 0.0, 0.1], &[0.0, 1.0, 0.0]] {
        let refused = pollster::block_on(gen.query_field(&field, cells)).expect_err("refused");
        assert!(refused.to_string().contains("field query"), "{refused}");
    }
    let one = pollster::block_on(gen.query_field(&field, &[0.0, 1.0, 0.0, 0.5])).expect("answered");
    assert_eq!(one.len(), 1);
    assert!(telperion_core::presets::by_identity("no-such-tree").is_err());
}

fn list<T: std::str::FromStr>(var: &str, or: &str) -> Vec<T> {
    std::env::var(var)
        .unwrap_or_else(|_| or.into())
        .split(',')
        .filter_map(|v| v.parse().ok())
        .collect()
}

#[test]
#[ignore = "device limits: every preset's field on the device against the CPU's"]
fn the_device_field_answers_as_the_cpu_field() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let gen = pollster::block_on(Generator::for_fields_async(gpu.clone())).expect("a generator");
    let only = std::env::var("FIELD_PRESETS").ok();
    let seeds: Vec<u32> = list("FIELD_SEEDS", "1");
    let sides: Vec<u32> = list("FIELD_SIDES", "24,36,48");
    println!("| Preset | Seed | Side | wood J | radius off | foliage J | leaves err | total | limbs | grow ms | query ms | field MB |");
    println!("|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    let ids = telperion_core::presets::CATALOGUE.iter().map(|c| c.1);
    for id in ids.filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id))) {
        for &seed in &seeds {
            let mut family = telperion_core::presets::by_identity(id).expect("a preset");
            family.skeleton.seed = seed;
            let request = Request {
                field: Some(None),
                ..Request::default()
            };
            let built = pipeline::build(&family, request).expect("the reference");
            if let Some(plan) = &built.outputs.plan {
                let named: std::collections::BTreeSet<u32> = plan.descriptors.iter().map(|d| d.system).collect();
                let leaves: u64 = plan.descriptors.iter().map(|d| u64::from(d.count)).sum();
                println!("{id} {seed}: the reference plans {} segments, {} leaves, {} systems; {} nodes", plan.descriptors.len(), leaves, named.len(), built.skeleton.tree.nodes.len());
            }
            let reference = built.outputs.field.expect("a field");
            let b = reference.bounds().expect("bounds");
            // Warm: the second of two.
            let mut grow_ms = 0.0;
            let mut field = None;
            for _ in 0..2 {
                drop(field.take());
                let t = Clock::now();
                field = Some(pollster::block_on(gen.grow_field(&family, None)).expect("grown"));
                grow_ms = t.elapsed_ms();
            }
            let field = field.expect("a field");
            if let (Some(plan), true) = (&built.outputs.plan, std::env::var("FIELD_NEAREST").is_ok()) {
                let ours = field.prims(&gpu, u64::from(field.wood), u64::from(field.leaves));
                let ours: Vec<Vec3> = ours
                    .iter()
                    .filter(|p| p[3] >= 0.0)
                    .map(|p| Vec3::new(f64::from(p[0] + p[4]) * 0.5, f64::from(p[1] + p[5]) * 0.5, f64::from(p[2] + p[6]) * 0.5))
                    .collect();
                let mut far = Vec::new();
                for d in &plan.descriptors {
                    let c = (d.endpoints[0] + d.endpoints[1]) * 0.5;
                    let near = ours.iter().map(|o| o.distance(c)).fold(f64::INFINITY, f64::min);
                    far.push(near);
                }
                far.sort_by(f64::total_cmp);
                let q = |f: f64| far[((far.len() - 1) as f64 * f) as usize];
                println!("{id} {seed}: {} reference centres, {} ours; nearest median {:.4} m, 90% {:.4}, most {:.4}", far.len(), ours.len(), q(0.5), q(0.9), q(1.0));
            }
            let systems: std::collections::BTreeSet<u32> = field.systems(&gpu).into_iter().collect();
            println!("{id} {seed}: {} wood, {} leaf segments, {} systems, {} leaves", field.wood, field.leaves, systems.len(), field.leaf_total(&gpu));
            for &n in &sides {
                let cells = grid(&b, n);
                let cpu: Vec<_> = cells
                    .iter()
                    .map(|(c, h)| reference.query(*c, *h).expect("answered"))
                    .collect();
                let words: Vec<f32> = cells
                    .iter()
                    .flat_map(|(c, h)| [c.x as f32, c.y as f32, c.z as f32, *h as f32])
                    .collect();
                let named: std::collections::BTreeSet<u32> = cpu.iter().filter_map(|o| o.limb).collect();
                println!("{id} {seed} {n}: the reference's cells name {} systems", named.len());
                let t = Clock::now();
                let dev = match pollster::block_on(gen.query_field(&field, &words)) {
                    Ok(dev) => dev,
                    Err(e) => {
                        println!("| {id} | {seed} | {n} | refused: {e} |");
                        continue;
                    }
                };
                let query_ms = t.elapsed_ms();
                let m = compare(&cpu, &dev);
                let fronds = family.canopy.rosette_fronds > 0.0;
                if let Err(m) = within(&m, fronds) {
                    println!("{id} {seed} {n}: past its tolerance: {m}");
                }
                println!(
                    "| {id} | {seed} | {n} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {grow_ms:.0} | {query_ms:.1} | {:.1} |",
                    m.wood, m.radius, m.foliage, m.leaves, m.total, m.limbs,
                    field.bytes() as f64 / 1e6
                );
            }
        }
    }
}
