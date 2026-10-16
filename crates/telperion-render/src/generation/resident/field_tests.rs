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

/// R2 on every shipped preset at seed 1, on grids of 24, 36 and 48 cells
/// a side: the device's field, its primitives written compact (R7),
/// against the CPU's (the gate's; the measurement below runs seeds and
/// timings).
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
        let fronds = family.canopy.rosette_fronds > 0.0;
        for n in [24, 36, 48] {
            let cells = grid(&reference.bounds().expect("bounds"), n);
            let cpu: Vec<_> = cells.iter().map(|(c, h)| reference.query(*c, *h).expect("answered")).collect();
            let dev = pollster::block_on(gen.query_field(&field, &words(&cells))).expect("answered");
            within(&compare(&cpu, &dev), fronds).unwrap_or_else(|m| panic!("{id} at {n}: {m}"));
        }
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

/// `family`'s reference field and the device's, asked of the same cells.
fn both(
    gen: &Generator,
    family: &telperion_core::presets::Family,
    limb_order: Option<u32>,
) -> (telperion_core::field::Field, super::DeviceField) {
    let request = Request {
        field: Some(limb_order),
        ..Request::default()
    };
    let reference = pipeline::build(family, request).expect("the reference");
    let reference = reference.outputs.field.expect("a field");
    let field = pollster::block_on(gen.grow_field(family, limb_order)).expect("grown");
    (reference, field)
}

fn words(cells: &[(Vec3, f64)]) -> Vec<f32> {
    cells
        .iter()
        .flat_map(|(c, h)| [c.x as f32, c.y as f32, c.z as f32, *h as f32])
        .collect()
}

/// The device's name for each reference limb system: the one it shares
/// the most cells with.
fn names(cpu: &[telperion_core::field::Occupancy], dev: &[super::Answer]) -> HashMap<u32, u32> {
    let mut shared: HashMap<u32, HashMap<u32, u64>> = HashMap::new();
    for (c, d) in cpu.iter().zip(dev) {
        if let (Some(c), Some(d)) = (c.limb, d.limb) {
            *shared.entry(c).or_default().entry(d).or_default() += 1;
        }
    }
    (shared.iter())
        .map(|(c, m)| (*c, *m.iter().max_by_key(|(d, n)| (**n, std::cmp::Reverse(**d))).unwrap().0))
        .collect()
}

/// A cell that meets more limb systems than one tally holds is answered,
/// a window of systems at a time, as the CPU answers it: the birch's
/// whole crown in one cube, its 387 systems at its own order.
#[test]
fn a_cell_crowded_with_limbs_names_the_cpu_s_limb() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let gen = pollster::block_on(Generator::for_fields_async(gpu.clone())).expect("a generator");
    let family = telperion_core::presets::by_identity("silver-birch").expect("a preset");
    let (reference, field) = both(&gen, &family, None);
    assert!(field.named() > super::field::TALLY, "{}", field.named());
    let b = reference.bounds().expect("bounds");
    let cells = grid(&b, 24);
    let cpu: Vec<_> = cells.iter().map(|(c, h)| reference.query(*c, *h).expect("answered")).collect();
    let dev = pollster::block_on(gen.query_field(&field, &words(&cells))).expect("answered");
    let named = names(&cpu, &dev);
    let centre = (b.min + b.max) * 0.5;
    let half = (b.max - b.min).x.max((b.max - b.min).y).max((b.max - b.min).z) * 0.5;
    let whole = reference.query(centre, half).expect("answered");
    let mine = pollster::block_on(gen.query_field(&field, &words(&[(centre, half)]))).expect("answered");
    let limb = whole.limb.expect("a limb");
    assert_eq!(mine[0].limb, named.get(&limb).copied(), "the whole crown's limb");
    assert!((f64::from(mine[0].leaves) / whole.leaves - 1.0).abs() < 1e-3);
}

/// The same tree's field answers the same, to the bit, however the
/// device fills its buckets: grown twice, asked twice.
#[test]
fn the_device_field_answers_the_same_every_time() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let gen = pollster::block_on(Generator::for_fields_async(gpu.clone())).expect("a generator");
    let family = telperion_core::presets::by_identity("silver-birch").expect("a preset");
    let (reference, first) = both(&gen, &family, None);
    let cells = words(&grid(&reference.bounds().expect("bounds"), 24));
    let once = pollster::block_on(gen.query_field(&first, &cells)).expect("answered");
    drop(first);
    let again = pollster::block_on(gen.grow_field(&family, None)).expect("grown");
    let twice = pollster::block_on(gen.query_field(&again, &cells)).expect("answered");
    assert!(once == twice, "the answers moved");
}

/// A field on a lost device is refused by name, never answered.
#[test]
fn a_field_on_a_lost_device_is_refused_by_name() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let gen = pollster::block_on(Generator::for_fields_async(gpu.clone())).expect("a generator");
    let family = telperion_core::presets::by_identity("date-palm").expect("a preset");
    let field = pollster::block_on(gen.grow_field(&family, None)).expect("grown");
    gpu.device.destroy();
    for cells in [&[0.0, 1.0, 0.0, 0.5][..], &[]] {
        let refused = pollster::block_on(gen.query_field(&field, cells)).expect_err("refused");
        assert!(refused.to_string().contains("lost"), "{refused}");
    }
    let refused = pollster::block_on(gen.grow_field(&family, None)).err().expect("refused");
    assert!(refused.to_string().contains("lost"), "{refused}");
}

/// R2 measured: every preset (`FIELD_PRESETS`), its seeds (`FIELD_SEEDS`)
/// and grids (`FIELD_SIDES`, 24, 36 and 48 cells a side), each held to
/// its tolerance, with each field's warm growth, its query and its bytes.
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
    let mut past = Vec::new();
    println!("| Preset | Seed | Side | wood J | radius off | foliage J | leaves err | total | limbs | grow ms | query ms | field MB |");
    println!("|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    let ids = telperion_core::presets::CATALOGUE.iter().map(|c| c.1);
    for id in ids.filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id))) {
        for &seed in &seeds {
            let mut family = telperion_core::presets::by_identity(id).expect("a preset");
            family.skeleton.seed = seed;
            // Warm: the second growth of two.
            let (reference, field) = both(&gen, &family, None);
            drop(field);
            let t = Clock::now();
            let field = pollster::block_on(gen.grow_field(&family, None)).expect("grown");
            let grow_ms = t.elapsed_ms();
            let fronds = family.canopy.rosette_fronds > 0.0;
            for &n in &sides {
                let cells = grid(&reference.bounds().expect("bounds"), n);
                let cpu: Vec<_> = cells.iter().map(|(c, h)| reference.query(*c, *h).expect("answered")).collect();
                let t = Clock::now();
                let dev = pollster::block_on(gen.query_field(&field, &words(&cells))).expect("answered");
                let query_ms = t.elapsed_ms();
                let m = compare(&cpu, &dev);
                if let Err(m) = within(&m, fronds) {
                    past.push(format!("{id} {seed} {n}: {m}"));
                }
                println!(
                    "| {id} | {seed} | {n} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {grow_ms:.0} | {query_ms:.1} | {:.1} |",
                    m.wood, m.radius, m.foliage, m.leaves, m.total, m.limbs,
                    field.bytes() as f64 / 1e6
                );
            }
        }
    }
    assert!(past.is_empty(), "past their tolerances: {past:#?}");
}
