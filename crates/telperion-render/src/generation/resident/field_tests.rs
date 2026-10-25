//! fn-257 R8a: the field read out on the device against the CPU's at the
//! same seed, every shipped preset at its age: each cell of the CPU's grid
//! at 24, 36 and 48 cells a side, its wood, its thickest radius, its
//! foliage, its leaves and its limb. Both are asked over the CPU's box,
//! since a frond's leaflets fall on each executor's own draws. The limbs
//! are compared as a partition, each device system read as the reference
//! system it shares the most cells with, and by the names both package
//! entries give them, the cell where each holds the most leaves (fn-317
//! R4c).
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
    /// The share of the cells the partition pairs whose limb carries the
    /// same name on both (fn-317 R4c).
    pub ids: f64,
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
    let best = |m: &HashMap<u32, u64>| {
        *m.iter()
            .max_by_key(|(t, n)| (**n, std::cmp::Reverse(**t)))
            .unwrap()
            .0
    };
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
    let (mut agree, mut held, mut named) = (0u64, 0u64, 0u64);
    for (c, d) in pairs() {
        if c.limb.is_some() || d.limb.is_some() {
            held += 1;
            let paired = c.limb.is_some() && d.limb.and_then(|d| read.get(&d).copied()) == c.limb;
            agree += u64::from(paired);
            named += u64::from(paired && d.limb == c.limb);
        }
    }
    let share = |n: u64, of: u64| if of == 0 { 1.0 } else { n as f64 / of as f64 };
    let (limbs, ids) = (share(agree, held), share(named, agree));
    Measures {
        wood,
        radius,
        foliage,
        leaves,
        total,
        limbs,
        ids,
    }
}

/// The tolerances R2 holds the device's field to, from the measured runs
/// (`FIELD.md`, `PROBE-R8.md`): every cell's wood and foliage, its thickest
/// radius within a percent, the leaves, the limbs as the same partition. A
/// frond's leaflets are drawn on each executor's own draws, as placement's
/// are (`every_rosette_is_placed_as_the_cpu_places_it`), so a crown of
/// fronds is held to where its draws allow.
pub(super) fn within(m: &Measures, fronds: bool) -> Result<(), String> {
    // A frond crown's wood holds its frond axes, which turn with the
    // crown's spiral: both executors take its reference from vertical
    // squared off a crown axis 4e-3 to 7e-3 rad off vertical
    // (`rosette::frame`), which magnifies the axes' 1.2e-6 rad lineage gap
    // to 1.2e-4 to 3.0e-4 rad (fn-283 .3, raw/frond-azimuth.log); the
    // leaflets and bases share it. Measured: wood 0.9976 on the palm at
    // seed 5678 on 24 cells; held at about twice its miss.
    let (wood, foliage, leaves, total, limbs) = match fronds {
        false => (0.999, 0.999, 0.002, 0.001, 0.95),
        true => (0.995, 0.70, 0.40, 0.02, 0.70),
    };
    let held = m.wood >= wood
        && m.radius <= 0.005
        && m.foliage >= foliage
        && m.leaves <= leaves
        && (m.total - 1.0).abs() <= total
        && m.limbs >= limbs;
    held.then_some(()).ok_or_else(|| format!("{m:?}"))
}

pub(super) fn generator() -> Option<Generator> {
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
pub(super) fn references(family: &Family, sides: &[u32], limb_order: Option<u32>) -> Vec<Grid> {
    std::thread::scope(|s| {
        let built: Vec<_> = (sides.iter())
            .map(|&n| s.spawn(move || pipeline::build(family, ask(n, limb_order))))
            .collect();
        let field = |b: std::thread::ScopedJoinHandle<_>| {
            let built: telperion_core::Result<pipeline::Built> = b.join().unwrap();
            built
                .expect("the reference")
                .outputs
                .field
                .expect("a field")
        };
        built.into_iter().map(field).collect()
    })
}

/// The device's field of `family` over the CPU's grid `cpu`.
pub(super) fn device(
    gen: &Generator,
    family: &Family,
    cpu: &Grid,
    limb_order: Option<u32>,
) -> Grid {
    pollster::block_on(gen.grow_field_in(family, cpu.frame, limb_order)).expect("read out")
}

/// R8a's comparison of `id` at `seed` on each of `sides`: each side's
/// measures and the device's milliseconds.
fn agreement(
    gen: &Generator,
    id: &str,
    seed: u32,
    sides: &[u32],
    named: bool,
) -> Vec<(u32, Measures, f64)> {
    let family = family(id, seed);
    let mut out = Vec::new();
    for mut cpu in references(&family, sides, None) {
        let t = Clock::now();
        let mut dev = device(gen, &family, &cpu, None);
        if named {
            cpu.name_limbs();
            dev.name_limbs();
        }
        let ms = t.elapsed_ms();
        assert_eq!(dev.frame, cpu.frame);
        out.push((cpu.frame.n, compare(&cpu.cells, &dev.cells), ms));
    }
    out
}

fn fronds(id: &str) -> bool {
    family(id, 1).canopy.rosette_fronds > 0.0
}

/// R8a on every shipped preset at seeds 1 and 5678, on a grid of 36 cells
/// a side: the device's read-out against the CPU's. The grids of 24 and 48
/// are the ignored test below (host, fn-304 ruling 4).
#[test]
fn every_preset_s_device_field_answers_within_its_tolerance() {
    every_preset_answers_within_its_tolerance(&[36]);
}

/// R8a on grids of 24, 36 and 48 cells a side:
/// `cargo test --profile ci -p telperion-render --lib -- --ignored every_preset_s_device_field_on_every_grid`
#[test]
#[ignore = "fn-304: every grid size, outside the gate; about 90 s"]
fn every_preset_s_device_field_on_every_grid_answers_within_its_tolerance() {
    every_preset_answers_within_its_tolerance(&[24, 36, 48]);
}

fn every_preset_answers_within_its_tolerance(sides: &[u32]) {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    for &(_, id, ..) in telperion_core::presets::CATALOGUE {
        for seed in [1, 5678] {
            for (n, m, _) in agreement(&gen, id, seed, sides, false) {
                within(&m, fronds(id)).unwrap_or_else(|m| panic!("{id} {seed} at {n}: {m}"));
            }
        }
    }
}

/// fn-317 R4c: every shipped preset's limb systems carry the CPU's names
/// on the device wherever the partition pairs them, at seeds 1 and 5678 on
/// grids of 24 and 36. Prints each preset's remainder.
#[test]
fn every_preset_s_limbs_carry_the_cpu_s_names() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    for &(_, id, ..) in telperion_core::presets::CATALOGUE {
        for seed in [1, 5678] {
            for (n, m, _) in agreement(&gen, id, seed, &[24, 36], true) {
                eprintln!(
                    "names {id} {seed} {n}: {:.6} of paired, partition {:.6}",
                    m.ids, m.limbs
                );
                assert!(m.ids >= NAMED, "{id} {seed} at {n}: {m:?}");
            }
        }
    }
}
/// Measured (fn-317, raw/names.log): 0.985 to 1 of paired cells, the
/// least the beech at 1 on 24; held at about twice its miss. A system
/// whose densest cell the executors' leaf estimates, within a few
/// thousandths, put in another cell takes another name whole.
const NAMED: f64 = 0.97;

/// fn-301 R2: the young palm rests its fronds' leaflets on the ground, and
/// the device's field boxes them in the frame they rest in, as the CPU's
/// plan does (`field_fronds.rs`'s `a_rested_leaflet_lies_in_its_planned_box`):
/// the palm at 12 answers within its tolerance on grids of 24, 36 and 48.
#[test]
fn the_young_palm_s_rested_leaflets_are_boxed_as_the_cpu_boxes_them() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    let mut palm = family("date-palm", 1);
    palm.age = 12.0;
    for cpu in references(&palm, &[24, 36, 48], None) {
        let dev = device(&gen, &palm, &cpu, None);
        let m = compare(&cpu.cells, &dev.cells);
        within(&m, true).unwrap_or_else(|m| panic!("the palm at 12 on {}: {m}", cpu.frame.n));
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
        let core = pipeline::build(&beech, ask(n, None))
            .err()
            .expect("refused");
        let refused = pollster::block_on(gen.grow_field(&beech, n, None)).expect_err("refused");
        assert!(refused.to_string().contains(&core.to_string()), "{refused}");
    }
    let mut clumped = beech;
    clumped.canopy.limb_clumping = 0.5;
    let refused = pollster::block_on(gen.grow_field(&clumped, 24, None)).expect_err("refused");
    assert!(
        refused.to_string().contains("leaves the device places"),
        "{refused}"
    );
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
    let [fine, whole] = <[Grid; 2]>::try_from(references(&birch, &[24, 3], None))
        .ok()
        .unwrap();
    let systems = |g: &Grid| {
        g.cells
            .iter()
            .filter_map(|c| c.limb)
            .collect::<std::collections::HashSet<_>>()
            .len()
    };
    assert!(systems(&fine) > 128, "{} systems", systems(&fine));
    let named = names(&fine.cells, &device(&gen, &birch, &fine, None).cells);
    let mine = device(&gen, &birch, &whole, None).cells[13];
    let crown = whole.cells[13];
    assert_eq!(mine.limb, named.get(&crown.limb.expect("a limb")).copied());
    assert!((mine.leaves / crown.leaves - 1.0).abs() < 1e-3);
}

/// A cell holding more of one limb's leaves than one word sums names the
/// CPU's limb and its leaves: the spruce's whole crown, some four million
/// needles, in the middle cell of a grid of three on the trunk's system.
#[test]
fn a_cell_past_a_word_of_one_limb_s_leaves_reads_as_the_cpu_s() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    let spruce = family("norway-spruce", 1);
    let [cpu] = <[Grid; 1]>::try_from(references(&spruce, &[3], Some(0)))
        .ok()
        .unwrap();
    let crown = cpu.cells[13];
    assert!(
        crown.leaves > f64::from(1u32 << 20),
        "{} leaves",
        crown.leaves
    );
    let mine = device(&gen, &spruce, &cpu, Some(0)).cells[13];
    assert!(mine.limb.is_some() && crown.limb.is_some());
    assert!(
        (mine.leaves / crown.leaves - 1.0).abs() < 1e-3,
        "{mine:?} {crown:?}"
    );
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
            for (n, m, ms) in agreement(&gen, id, seed, &sides, false) {
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

/// fn-316: the device's field counts the florets the CPU's counts on the
/// trees that flower, the same total to the floret and each in the CPU's
/// cell but those whose seat the f32 tree moves across a cell's face; a
/// tree that flowers nowhere answers none.
#[test]
fn the_device_field_counts_the_flowers_the_cpu_counts() {
    flowers_agree(
        &[
            ("telperion", 1),
            ("laurelin", 5678),
            ("oregon-white-oak", 1),
        ],
        &[24],
    );
}

/// fn-316 measured: both flowering trees at seeds 1 and 5678 on grids of
/// 24, 36 and 48 cells a side.
#[test]
#[ignore = "a measurement: the flowering trees' fields on every grid"]
fn the_device_field_counts_the_flowers_on_every_grid() {
    let trees = [
        ("telperion", 1),
        ("telperion", 5678),
        ("laurelin", 1),
        ("laurelin", 5678),
    ];
    flowers_agree(&trees, &[24, 36, 48]);
}

/// The share of florets the device may count in another cell than the
/// CPU's, held at about twice its measured most: 4.0e-4, Laurelin at 5678
/// on 48 cells (`the_device_field_counts_the_flowers_on_every_grid`,
/// 2026-10-25). A seat the f32 tree moves across a cell's face moves.
const FLOWERS_MOVED: f64 = 1e-3;
/// How far the device's total may stand from the CPU's, as a share, held
/// at about three times its measured most: Telperion at 5678 hangs 10 of
/// 157,417 florets more on the device (6.4e-5), every other tree the same
/// total.
const FLOWERS_TOTAL: f64 = 2e-4;

fn flowers_agree(trees: &[(&str, u32)], sides: &[u32]) {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    let flowers = |g: &Grid| g.cells.iter().map(|c| c.flowers).sum::<f64>();
    let mut past = 0;
    for &(id, seed) in trees {
        let family = family(id, seed);
        for cpu in references(&family, sides, None) {
            let dev = device(&gen, &family, &cpu, None);
            let moved: f64 = (cpu.cells.iter().zip(&dev.cells))
                .map(|(c, d)| (c.flowers - d.flowers).abs())
                .sum::<f64>()
                / 2.0;
            let (total, n) = (flowers(&cpu), cpu.frame.n);
            let share = if total > 0.0 { moved / total } else { 0.0 };
            println!(
                "{id} {seed} {n}: {total} florets, device {}, moved {moved} ({share:.2e})",
                flowers(&dev)
            );
            // A tree the CPU finds no floret on has none on the device.
            let gap = if total > 0.0 {
                (flowers(&dev) / total - 1.0).abs()
            } else {
                flowers(&dev)
            };
            println!("  total off {gap:.2e}");
            past += usize::from(gap > FLOWERS_TOTAL || share > FLOWERS_MOVED);
            assert_eq!(total > 0.0, id != "oregon-white-oak", "{id} flowers");
        }
    }
    assert_eq!(past, 0, "trees past their tolerances");
}
