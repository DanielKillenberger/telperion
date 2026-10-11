//! The device's retirement held to the CPU's (`grow_capture_retired`):
//! every verdict a young tree's growth takes, its root, cycle, waited,
//! kept or retired, and living, word for word where built exact.
use super::columns::Cap;
use super::grown_tests::{built, device, grown, YOUNG};
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
