//! The device's retirement held to the CPU's (`grow_capture_retired`):
//! every verdict a young tree's growth takes, its root, cycle, waited,
//! kept or retired, and living, word for word where built exact.
use super::build::{Build, Kit, Tables};
use super::build_grow::{grow, Sizes};
use super::columns::Cap;
use super::grown_compare::exact;
use super::grown_tests::{built, device, grown, Grown, YOUNG};
use super::settle_device::Kernels;
use super::settle_read::{beside_agree, cpu_settled};
use super::settle_run::settle;
use super::settle_tests::constants;
use telperion_space::SettleTo;
use super::hier_scan::read_words;
use super::retire_device::{Retire, ENTRY, L_AT};
use super::Arith;
use telperion_core::presets::Preset;

/// The log's verdicts, by cycle then root, each its five words.
fn sorted(words: &[u32]) -> Vec<[u32; 5]> {
    let mut v: Vec<[u32; 5]> = words.chunks(5).map(|w| [w[0], w[1], w[2], w[3], w[4]]).collect();
    v.sort_by_key(|e| (e[1], e[0]));
    v
}

#[test]
fn young_trees_retire_on_the_device_as_on_the_cpu() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device(Arith::Exact) else {
        return;
    };
    let mut differ = Vec::new();
    // Older trees that wake buds, so that subtrees wait on them.
    let waking = [(Preset::NorwaySpruce, "norway-spruce", 18), (Preset::Ordinary, "ordinary", 16)];
    for (preset, name, age) in YOUNG.into_iter().chain(waking) {
        let g = grown(preset, age, 42);
        let cpu = sorted(&Cap(&telperion_space::grow_capture_retired(&g.species, g.request).expect("the CPU's tree")).u32s("r_log"));
        let (b, _) = built(&gpu, &kit, &g, age);
        let r = &b.retire;
        let head = read_words(&gpu, &r.verdicts, 0, L_AT).expect("a readback");
        assert_eq!(head[1], 0, "{name}: the log held every verdict");
        assert!(head[0] <= Retire::log_cap(r.stride), "{name}: the log's count");
        let words = match head[0] {
            0 => Vec::new(),
            n => read_words(&gpu, &r.verdicts, L_AT, ENTRY * u64::from(n)).expect("a readback"),
        };
        let dev = sorted(&words);
        let count = |v: &[[u32; 5]], k: u32| v.iter().filter(|e| e[2] == k).count();
        println!(
            "{name} {age}: device {} verdicts ({} waited, {} kept, {} retired), cpu {} ({}, {}, {})",
            dev.len(), count(&dev, 0), count(&dev, 1), count(&dev, 2), cpu.len(), count(&cpu, 0), count(&cpu, 1), count(&cpu, 2)
        );
        if dev != cpu {
            let first = dev.iter().zip(&cpu).position(|(a, b)| a != b);
            println!("{name}: first parting {first:?}: device {:?} cpu {:?}", first.map(|k| dev[k]), first.map(|k| cpu[k]));
            differ.push(name);
        }
    }
    assert!(differ.is_empty(), "the device's verdicts are not the CPU's: {differ:?}");
}

/// `g` grown on the device with its retired subtrees' phytomers let go at
/// each move into larger phytomer tables, from tables small enough that
/// the young trees move several times; and its tables and rows held.
fn compacted(gpu: &crate::Gpu, kit: &Kit, g: &Grown, compact: bool) -> (Build, Sizes) {
    let t = Tables::new(gpu, &g.tables, &g.chain).expect("tables");
    let mut sizes = Sizes::least();
    (sizes.compact, sizes.caps.phyts) = (compact, 512);
    let (b, sizes, _) = grow(gpu, kit, &t, &g.tables, (sizes, g.request.age)).expect("a build");
    (b, sizes)
}

/// The young trees settled after growing with their retired subtrees let
/// go equal the CPU's laid tree in every word, exact.
#[test]
fn young_trees_settle_as_on_the_cpu_with_retired_subtrees_let_go() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device(Arith::Exact) else {
        return;
    };
    let k = Kernels::new(&gpu, Arith::Exact);
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
        let mut a = exact(&dev.0, &cpu.0);
        beside_agree(&mut a, &dev.1, &cpu.1);
        println!(
            "{name} {age}: arena rows {} of {} without letting go, {} of {} with; {} records",
            rows, cap, b.held.phyts, sizes.caps.phyts, b.retire.rec_n
        );
        if !a.exact() {
            println!("{}", a.report());
            differ.push(name);
        }
    }
    assert!(differ.is_empty(), "the settled trees with retirement are not the CPU's: {differ:?}");
}

/// The laid node table with retired subtrees let go is the same on every
/// context and block cap.
#[test]
fn the_node_table_with_retirement_is_the_same_on_every_context() {
    let _held = super::super::whole_tree();
    let mut differ = Vec::new();
    for (preset, name, age) in [(Preset::OregonWhiteOak, "oregon-white-oak", 9), (Preset::NorwaySpruce, "norway-spruce", 12)] {
        let g = grown(preset, age, 42);
        let table = |cap: u32| -> Option<Vec<u32>> {
            let (gpu, kit) = device(Arith::Plain)?;
            let k = Kernels::new(&gpu, Arith::Plain);
            let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
            let mut sizes = Sizes::least();
            (sizes.compact, sizes.caps.phyts, sizes.block_cap) = (true, 512, cap);
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
