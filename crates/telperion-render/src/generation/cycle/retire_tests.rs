//! The device's retirement held to the CPU's (`grow_capture_retired`):
//! every verdict a young tree's growth takes, its root, cycle, waited,
//! kept or retired, and living, word for word; the trees settled after
//! it to the ruled standard (`ruled.rs`).
use super::build::{Build, Kit, Tables};
use super::build_grow::{grow_now as grow, Compact, Sizes};
use super::columns::Cap;
use super::grown_tests::{built_with, device, grown, Grown, YOUNG};
use super::settle_device::Kernels;
use super::settle_read::cpu_settled;
use super::settle_run::settle_now as settle;
use super::settle_tests::{constants, held};
use telperion_space::SettleTo;
use super::hier_scan::read_words;
use super::retire_device::{ENTRY, L_AT};
use telperion_core::presets::Preset;

/// The log's verdicts, by cycle then root, each its root's lineage key,
/// cycle, verdict and living's two words: a root by its lineage, as the
/// device's slots move as its arena compacts (fn-256 task 5).
fn sorted(entries: impl Iterator<Item = [u32; 6]>) -> Vec<[u32; 6]> {
    let mut v: Vec<[u32; 6]> = entries.collect();
    v.sort_by_key(|e| (e[2], u64::from(e[0]) | u64::from(e[1]) << 32));
    v
}

#[test]
fn young_trees_retire_on_the_device_as_on_the_cpu() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device() else {
        return;
    };
    let mut differ = Vec::new();
    // Older trees that wake buds, so that subtrees wait on them.
    let waking = [(Preset::NorwaySpruce, "norway-spruce", 18), (Preset::Ordinary, "ordinary", 16)];
    for (preset, name, age) in YOUNG.into_iter().chain(waking) {
        let g = grown(preset, age, 42);
        let capture = telperion_space::grow_capture_retired(&g.species, g.request).expect("the CPU's tree");
        let (log, keys) = (Cap(&capture).u32s("r_log"), Cap(&capture).u32s("r_lineage"));
        let cpu = sorted(log.chunks(5).map(|w| [keys[2 * w[0] as usize], keys[2 * w[0] as usize + 1], w[1], w[2], w[3], w[4]]));
        let (b, _) = built_with(&gpu, &kit, &g, (age, true));
        let r = &b.retire;
        let head = read_words(&gpu, &r.verdicts, 0, L_AT).expect("a readback");
        assert_eq!(head[1], 0, "{name}: the log held every verdict");
        assert!(head[0] <= r.log_cap, "{name}: the log's count");
        let words = match head[0] {
            0 => Vec::new(),
            n => read_words(&gpu, &r.verdicts, L_AT, ENTRY * u64::from(n)).expect("a readback"),
        };
        let dev = sorted(words.chunks(6).map(|w| [w[0], w[1], w[2], w[3], w[4], w[5]]));
        let count = |v: &[[u32; 6]], k: u32| v.iter().filter(|e| e[3] == k).count();
        println!(
            "{name} {age}: device {} verdicts ({} waited, {} kept, {} retired), cpu {} ({}, {}, {})",
            dev.len(), count(&dev, 0), count(&dev, 1), count(&dev, 2), cpu.len(), count(&cpu, 0), count(&cpu, 1), count(&cpu, 2)
        );
        // Each verdict's root, cycle, verdict and kind exact, its living
        // (an f32's bits) to the ruled standard.
        let flat = |v: &[[u32; 6]]| v.iter().flat_map(|e| [u64::from(e[0]) | u64::from(e[1]) << 32, u64::from(e[2]), u64::from(e[3]), f64::from(f32::from_bits(e[4])).to_bits(), u64::from(e[5])]).collect::<Vec<u64>>();
        let mut a = super::compare::Agreement::default();
        a.records("verdicts", &flat(&dev), &flat(&cpu), (5, &[3]));
        if !a.exact() {
            println!("{name}: {}", a.report());
            differ.push(name);
        }
    }
    assert!(differ.is_empty(), "the device's verdicts are not the CPU's: {differ:?}");
}

/// `g` grown on the device from the least tables, its arena compacted in
/// place after every cycle where `compact` (`compact.rs`); and its tables
/// and rows held.
fn compacted(gpu: &crate::Gpu, kit: &Kit, g: &Grown, compact: bool) -> (Build, Sizes) {
    let t = Tables::new(gpu, &g.tables, &g.chain).expect("tables");
    let mut sizes = Sizes::least();
    sizes.compact = if compact { Compact::Every } else { Compact::Off };
    let (b, sizes, _) = grow(gpu, kit, &t, &g.tables, (sizes, g.request.age)).expect("a build");
    (b, sizes)
}

/// The young trees settled after growing with their retired subtrees let
/// go hold to the CPU's laid tree to the ruled standard.
#[test]
fn young_trees_settle_as_on_the_cpu_with_retired_subtrees_let_go() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let mut differ = Vec::new();
    for (preset, name, age) in YOUNG {
        let g = grown(preset, age, 42);
        let capture = telperion_space::grow_capture_settled_at(&g.species, g.request, SettleTo::Laid).expect("the CPU's tree");
        let cpu = cpu_settled(&Cap(&capture), SettleTo::Laid);
        let (plain, plain_sizes) = compacted(&gpu, &kit, &g, false);
        let (rows, cap) = (plain.held.phyts, plain_sizes.caps.phyts);
        drop(plain);
        let (b, sizes) = compacted(&gpu, &kit, &g, true);
        let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
        let s = settle(&gpu, (&kit, &k), (&b, &t), (&constants(&g), SettleTo::Laid)).expect("a settle");
        let dev = s.snapshot(&gpu, &b.state, SettleTo::Laid).expect("a readback");
        println!(
            "{name} {age}: arena rows {} of {} without letting go, {} of {} with",
            rows, cap, b.held.phyts, sizes.caps.phyts
        );
        let found = held((&dev.0, &dev.1), (&cpu.0, &cpu.1));
        differ.extend(found.into_iter().map(|f| format!("{name}: {f}")));
    }
    assert!(differ.is_empty(), "the settled trees with retirement are not the CPU's: {differ:?}");
}

/// The laid node table of every young preset, its retired subtrees let
/// go, is the same on ten contexts and every block cap.
#[test]
fn the_node_table_with_retirement_is_the_same_on_every_context() {
    let _held = super::super::whole_tree();
    let mut differ = Vec::new();
    for (preset, name, age) in YOUNG {
        let g = grown(preset, age, 42);
        let table = |cap: u32| -> Option<Vec<u32>> {
            let (gpu, kit) = device()?;
            let k = Kernels::new(&gpu);
            let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
            let mut sizes = Sizes::least();
            (sizes.compact, sizes.block_cap) = (Compact::Every, cap);
            let (b, ..) = grow(&gpu, &kit, &t, &g.tables, (sizes, age)).expect("a build");
            let s = settle(&gpu, (&kit, &k), (&b, &t), (&constants(&g), SettleTo::Laid)).expect("a settle");
            let mut words = s.error.to_vec();
            words.extend(read_words(&gpu, &s.nt, 0, s.table.words).expect("a readback"));
            Some(words)
        };
        let Some(first) = table(super::hier_build::CAP) else {
            return;
        };
        let runs = (2..=10).map(|k| (format!("context {k}"), super::hier_build::CAP)).chain([64, 128, 1024].map(|c| (format!("block cap {c}"), c)));
        for (label, cap) in runs {
            let again = table(cap).expect("a device");
            let parted = again.iter().zip(&first).filter(|(a, b)| a != b).count() + again.len().abs_diff(first.len());
            println!("{name} {age}, {label}: {} words, {parted} differ", first.len());
            if parted > 0 {
                differ.push(format!("{name} {label}"));
            }
        }
    }
    assert!(differ.is_empty(), "the node table differs: {differ:?}");
}

/// A compaction after the last cycle would leave the build holding the
/// last census's rows. The oak at 10 and the ordinary at 8, which record
/// retired rows, compacted after every cycle but the last: each holds the
/// rows its unit records reach, and each settles as the CPU's laid tree
/// to the ruled standard.
#[test]
fn a_build_holds_its_compacted_rows_after_its_last_cycle() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let mut differ = Vec::new();
    for (preset, name, age) in [(Preset::OregonWhiteOak, "oregon-white-oak", 10), (Preset::Ordinary, "ordinary", 8)] {
        let g = grown(preset, age, 42);
        let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
        let mut sizes = Sizes::least();
        sizes.compact = Compact::Every;
        let (b, _, grew) = grow(&gpu, &kit, &t, &g.tables, (sizes, age)).expect("a build");
        assert!(grew.moved_at.iter().all(|&m| m < age), "{name} {age}: moved after its last cycle: {:?}", grew.moved_at);
        let caps = b.state.caps;
        let units = read_words(&gpu, &b.state.units, 0, super::state::UNITS * u64::from(caps.units)).expect("a readback");
        let at = |c: u64, u: usize| units[c as usize * caps.units as usize + u];
        let reached = (0..b.held.units as usize)
            .filter(|&u| at(super::state::U_FIRST, u) & 0x8000_0000 == 0)
            .map(|u| at(super::state::U_FIRST, u) + at(super::state::U_COUNT, u))
            .max()
            .unwrap_or(0);
        println!("{name} {age}: the units reach {reached} arena rows, the build holds {}", b.held.phyts);
        assert_eq!(b.held.phyts, reached, "{name} {age}: the build holds the rows the compaction kept");
        let s = settle(&gpu, (&kit, &k), (&b, &t), (&constants(&g), SettleTo::Laid)).expect("a settle");
        let dev = s.snapshot(&gpu, &b.state, SettleTo::Laid).expect("a readback");
        let capture = telperion_space::grow_capture_settled_at(&g.species, g.request, SettleTo::Laid).expect("the CPU's tree");
        let cpu = cpu_settled(&Cap(&capture), SettleTo::Laid);
        let found = held((&dev.0, &dev.1), (&cpu.0, &cpu.1));
        differ.extend(found.into_iter().map(|f| format!("{name}: {f}")));
    }
    assert!(differ.is_empty(), "the settled trees are not the CPU's: {differ:?}");
}
