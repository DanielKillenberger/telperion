//! fn-257 R8a: the field read out on the device against the CPU's at the
//! same seed, every shipped preset at its age: each cell of the CPU's grid
//! at 24, 36 and 48 cells a side, its wood, its thickest radius, its
//! foliage, its leaves and its limb. Both are asked over the CPU's box,
//! since a frond's leaflets fall on each executor's own draws. The limbs
//! are compared as a partition: each executor names a system by its own
//! node ids, so each device system is read as the reference system it
//! shares the most cells with.
use super::super::{Clock, Generator};
use std::collections::HashMap;
use telperion_core::field::{Grid, Occupancy};
use telperion_core::pipeline::{self, FieldRequest, Request};
use telperion_core::presets::Family;

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

/// The device's name for each of `from`'s limb systems: the one of `to`'s
/// it shares the most cells with, the lower on a tie.
fn names(from: &[Occupancy], to: &[Occupancy]) -> HashMap<u32, u32> {
    let mut shared: HashMap<u32, HashMap<u32, u64>> = HashMap::new();
    for (f, t) in from.iter().zip(to) {
        if let (Some(f), Some(t)) = (f.limb, t.limb) {
            *shared.entry(f).or_default().entry(t).or_default() += 1;
        }
    }
    let best = |m: &HashMap<u32, u64>| *m.iter().max_by_key(|(t, n)| (**n, std::cmp::Reverse(**t))).unwrap().0;
    shared.iter().map(|(f, m)| (*f, best(m))).collect()
}

pub(super) fn compare(cpu: &[Occupancy], dev: &[Occupancy]) -> Measures {
    let pairs = || cpu.iter().zip(dev);
    let wood = jaccard(pairs().map(|(c, d)| (c.wood, d.wood)));
    let foliage = jaccard(pairs().map(|(c, d)| (c.foliage, d.foliage)));
    // The share of the cells both find wood in whose thickest radius
    // differs by more than a percent.
    let (mut off, mut both) = (0u64, 0u64);
    for (c, d) in pairs().filter(|(c, d)| c.wood && d.wood) {
        let e = (c.wood_radius - d.wood_radius).abs() / c.wood_radius.max(1e-6);
        both += 1;
        off += u64::from(e > 0.01);
    }
    let radius = if both == 0 {
        0.0
    } else {
        off as f64 / both as f64
    };
    let (sum_c, sum_d) = pairs().fold((0.0, 0.0), |(a, b), (c, d)| (a + c.leaves, b + d.leaves));
    let diff: f64 = pairs().map(|(c, d)| (c.leaves - d.leaves).abs()).sum();
    let leaves = if sum_c > 0.0 { diff / sum_c } else { 0.0 };
    let total = if sum_c > 0.0 { sum_d / sum_c } else { 1.0 };
    let read = names(dev, cpu);
    let (mut agree, mut held) = (0u64, 0u64);
    for (c, d) in pairs() {
        if c.limb.is_some() || d.limb.is_some() {
            held += 1;
            agree +=
                u64::from(c.limb.is_some() && d.limb.and_then(|d| read.get(&d).copied()) == c.limb);
        }
    }
    let limbs = if held == 0 {
        1.0
    } else {
        agree as f64 / held as f64
    };
    Measures {
        wood,
        radius,
        foliage,
        leaves,
        total,
        limbs,
    }
}

/// The tolerances R2 holds the device's field to, from the measured runs
/// (`FIELD.md`, `PROBE-R8.md`): every cell's wood and foliage, its thickest
/// radius within a percent, the leaves, the limbs as the same partition. A
/// frond's leaflets are drawn on each executor's own draws, as placement's
/// are (`every_rosette_is_placed_as_the_cpu_places_it`), so a crown of
/// fronds is held to where its draws allow.
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

fn generator() -> Option<Generator> {
    let gpu = super::super::test_gpu()?;
    Some(pollster::block_on(Generator::for_fields_async(gpu)).expect("a generator"))
}

pub(super) fn family(id: &str, seed: u32) -> Family {
    let mut family = telperion_core::presets::by_identity(id).expect("a preset");
    family.skeleton.seed = seed;
    family
}

fn ask(n: u32, limb_order: Option<u32>) -> Request {
    Request {
        field: Some(FieldRequest { n, limb_order }),
        ..Request::default()
    }
}

/// The CPU's field of `family` at each of `sides`, built side by side.
fn references(family: &Family, sides: &[u32], limb_order: Option<u32>) -> Vec<Grid> {
    std::thread::scope(|s| {
        let built: Vec<_> = (sides.iter())
            .map(|&n| s.spawn(move || pipeline::build(family, ask(n, limb_order))))
            .collect();
        let field = |b: std::thread::ScopedJoinHandle<_>| {
            let built: telperion_core::Result<pipeline::Built> = b.join().unwrap();
            built.expect("the reference").outputs.field.expect("a field")
        };
        built.into_iter().map(field).collect()
    })
}

/// The device's field of `family` over the CPU's grid `cpu`.
fn device(gen: &Generator, family: &Family, cpu: &Grid, limb_order: Option<u32>) -> Grid {
    pollster::block_on(gen.grow_field_in(family, cpu.frame, limb_order)).expect("read out")
}

/// R8a's comparison of `id` at `seed` on each of `sides`: each side's
/// measures and the device's milliseconds.
fn agreement(gen: &Generator, id: &str, seed: u32, sides: &[u32]) -> Vec<(u32, Measures, f64)> {
    let family = family(id, seed);
    let mut out = Vec::new();
    for cpu in references(&family, sides, None) {
        let t = Clock::now();
        let dev = device(gen, &family, &cpu, None);
        let ms = t.elapsed_ms();
        assert_eq!(dev.frame, cpu.frame);
        out.push((cpu.frame.n, compare(&cpu.cells, &dev.cells), ms));
    }
    out
}

fn fronds(id: &str) -> bool {
    family(id, 1).canopy.rosette_fronds > 0.0
}

/// R8a on every shipped preset at seeds 1 and 5678, on grids of 24, 36
/// and 48 cells a side: the device's read-out against the CPU's.
#[test]
fn every_preset_s_device_field_answers_within_its_tolerance() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    for &(_, id, ..) in telperion_core::presets::CATALOGUE {
        for seed in [1, 5678] {
            for (n, m, _) in agreement(&gen, id, seed, &[24, 36, 48]) {
                within(&m, fronds(id)).unwrap_or_else(|m| panic!("{id} {seed} at {n}: {m}"));
            }
        }
    }
}

/// A grid side outside 3 to 256 is refused before any work as the core
/// refuses it, a family the device cannot draw by name, and an id the
/// catalogue does not serve by name.
#[test]
fn the_device_field_refuses_what_the_core_refuses() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    let beech = family("european-beech", 1);
    for n in [0, 2, 257] {
        let core = pipeline::build(&beech, ask(n, None)).err().expect("refused");
        let refused = pollster::block_on(gen.grow_field(&beech, n, None)).expect_err("refused");
        assert!(refused.to_string().contains(&core.to_string()), "{refused}");
    }
    let mut clumped = beech;
    clumped.canopy.limb_clumping = 0.5;
    let refused = pollster::block_on(gen.grow_field(&clumped, 24, None)).expect_err("refused");
    assert!(refused.to_string().contains("leaves the device places"), "{refused}");
    assert!(telperion_core::presets::by_identity("no-such-tree").is_err());
}

/// A cell crowded with limbs names the CPU's limb: the birch's whole crown
/// in the middle cell of a grid of three, its 387 systems at its own order,
/// each device system read as the CPU's it shares the most cells with on
/// a grid of 24.
#[test]
fn a_cell_crowded_with_limbs_names_the_cpu_s_limb() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    let birch = family("silver-birch", 1);
    let [fine, whole] = <[Grid; 2]>::try_from(references(&birch, &[24, 3], None)).ok().unwrap();
    let systems = |g: &Grid| g.cells.iter().filter_map(|c| c.limb).collect::<std::collections::HashSet<_>>().len();
    assert!(systems(&fine) > 128, "{} systems", systems(&fine));
    let named = names(&fine.cells, &device(&gen, &birch, &fine, None).cells);
    let mine = device(&gen, &birch, &whole, None).cells[13];
    let crown = whole.cells[13];
    assert_eq!(mine.limb, named.get(&crown.limb.expect("a limb")).copied());
    assert!((mine.leaves / crown.leaves - 1.0).abs() < 1e-3);
}

/// The same tree's field reads out the same, to the bit, however the
/// device fills its table: read out twice.
#[test]
fn the_device_field_answers_the_same_every_time() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    let birch = family("silver-birch", 1);
    let once = pollster::block_on(gen.grow_field(&birch, 24, None)).expect("read out");
    let twice = pollster::block_on(gen.grow_field(&birch, 24, None)).expect("read out");
    assert!(once == twice, "the grid moved");
}

/// A field on a lost device is refused by name, never read out.
#[test]
fn a_field_on_a_lost_device_is_refused_by_name() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    let palm = family("date-palm", 1);
    pollster::block_on(gen.grow_field(&palm, 24, None)).expect("read out");
    gen.gpu.device.destroy();
    let refused = pollster::block_on(gen.grow_field(&palm, 24, None)).expect_err("refused");
    assert!(refused.to_string().contains("lost"), "{refused}");
}

fn list<T: std::str::FromStr>(var: &str, or: &str) -> Vec<T> {
    std::env::var(var)
        .unwrap_or_else(|_| or.into())
        .split(',')
        .filter_map(|v| v.parse().ok())
        .collect()
}

/// R8a measured: every preset (`FIELD_PRESETS`), its seeds (`FIELD_SEEDS`)
/// and grids (`FIELD_SIDES`), each held to its tolerance, with the device's
/// read-out time over the CPU's grid.
#[test]
#[ignore = "a measurement: every preset's field on the device against the CPU's"]
fn the_device_field_answers_as_the_cpu_field() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    let only = std::env::var("FIELD_PRESETS").ok();
    let seeds: Vec<u32> = list("FIELD_SEEDS", "1");
    let sides: Vec<u32> = list("FIELD_SIDES", "24,36,48");
    let mut past = Vec::new();
    println!("| Preset | Seed | Side | wood J | radius off | foliage J | leaves err | total | limbs | read-out ms |");
    println!("|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    let ids = telperion_core::presets::CATALOGUE.iter().map(|c| c.1);
    for id in ids.filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id))) {
        for &seed in &seeds {
            for (n, m, ms) in agreement(&gen, id, seed, &sides) {
                if let Err(m) = within(&m, fronds(id)) {
                    past.push(format!("{id} {seed} {n}: {m}"));
                }
                println!(
                    "| {id} | {seed} | {n} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {ms:.0} |",
                    m.wood, m.radius, m.foliage, m.leaves, m.total, m.limbs
                );
            }
        }
    }
    assert!(past.is_empty(), "past their tolerances: {past:#?}");
}
