//! fn-256 task 5 (the host's ruling 2 of 2026-10-12): what each table
//! holds at most against the closed form's hint, every shipped preset at
//! its default age and seeds 1, 7 and 42, compacted after every cycle: the
//! ratios `build_size::ARENA` states. `cargo test --profile ci -p
//! telperion-render arena_ratios -- --ignored --nocapture`, under the GPU
//! lock; `ARENA_PRESETS` names a comma list.
use super::build::Tables;
use super::build_grow::{grow_now, Compact, Sizes};
use super::build_arena::{hint, TABLES};
use super::grown_tests::{device, grown_from};
use telperion_core::presets::Preset;

const NAMES: [&str; TABLES] = ["axes", "units", "phytomers", "sleepers", "living", "woken", "leaves", "nodes", "columns"];

#[test]
#[ignore = "a measurement"]
fn arena_ratios() {
    let Some((gpu, kit)) = device() else {
        return;
    };
    // Held to the native default and what the card leaves, as a build is.
    let free = gpu.free_device_memory().unwrap_or(u64::MAX);
    let _capped = gpu.ledger.capped((4u64 << 30).min(gpu.ledger.live().saturating_add(free / 100 * 85)));
    let only = std::env::var("ARENA_PRESETS").ok();
    let mut most = [0f64; TABLES];
    println!("| Preset | Seed | Hint | {} |", NAMES.join(" | "));
    let ids = telperion_core::presets::CATALOGUE.iter().map(|c| c.1);
    for id in ids.filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id))) {
        let preset = Preset::from_id(id).expect("a shipped preset");
        let family = preset.parameters();
        let age = family.age as u32;
        // The gate's seeds, and the family's own (`skeleton.seed`).
        for seed in [1, 7, 42, u64::from(family.skeleton.seed)] {
            // The species and light a build grows by (`executor::growth`):
            // the supernatural rows change the Two Trees'.
            let growth = telperion_core::pipeline::executor::growth(&family).expect("a growth");
            let g = grown_from(growth.species, growth.request.light, (age, seed));
            let h = hint(&g.species, age).expect("a hint").max(1.0);
            let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
            let off = std::env::var("ARENA_COMPACT").is_ok_and(|c| c == "off");
            let sizes = Sizes { compact: if off { Compact::Off } else { Compact::Every }, ..Sizes::least() };
            let (b, _, grew) = grow_now(&gpu, &kit, &t, &g.tables, (sizes, age)).expect("a build");
            drop(b);
            gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("freed");
            let ratios: Vec<f64> = grew.most.iter().map(|&n| f64::from(n) / h).collect();
            for (m, r) in most.iter_mut().zip(&ratios) {
                *m = m.max(*r);
            }
            let cells: Vec<String> = ratios.iter().map(|r| format!("{r:.4}")).collect();
            println!("| {id} | {seed} | {h:.0} | {} |", cells.join(" | "));
        }
    }
    let cells: Vec<String> = most.iter().map(|r| format!("{r:.4}")).collect();
    println!("| most | | | {} |", cells.join(" | "));
}

/// The host's gate (ruling 2): every shipped preset at its default age,
/// seeds 1, 7 and 42, grown from the tables its hint sizes, makes no move
/// and never begins again. `ARENA_PRESETS` names a comma list.
#[test]
#[ignore = "device limits: every preset's whole tree, three seeds"]
fn every_preset_grows_in_its_hinted_arena() {
    let Some((gpu, kit)) = device() else {
        return;
    };
    let free = gpu.free_device_memory().unwrap_or(u64::MAX);
    let _capped = gpu.ledger.capped((4u64 << 30).min(gpu.ledger.live().saturating_add(free / 100 * 85)));
    let only = std::env::var("ARENA_PRESETS").ok();
    let mut apart = Vec::new();
    let ids = telperion_core::presets::CATALOGUE.iter().map(|c| c.1);
    for id in ids.filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id))) {
        let family = Preset::from_id(id).expect("a shipped preset").parameters();
        let age = family.age as u32;
        let growth = telperion_core::pipeline::executor::growth(&family).expect("a growth");
        for seed in [1, 7, 42] {
            let g = grown_from(growth.species.clone(), growth.request.light, (age, seed));
            let sizes = Sizes::hinted(hint(&g.species, age).expect("a hint"));
            let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
            gpu.ledger.take_peak();
            let clock = std::time::Instant::now();
            let (b, grown, grew) = grow_now(&gpu, &kit, &t, &g.tables, (sizes, age)).unwrap_or_else(|e| panic!("{id} {seed}: {e}"));
            println!("{id} {seed}: bounds {:?}", grown.bounds);
            let secs = clock.elapsed().as_secs_f64();
            drop(b);
            gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("freed");
            let caps = sizes.caps;
            let held = [caps.axes, caps.units, caps.phyts, caps.sleepers, caps.next, caps.woken];
            let fill: Vec<String> = grew.most[..6].iter().zip(held).map(|(&m, c)| format!("{:.2}", f64::from(m) / f64::from(c))).collect();
            println!(
                "{id} {seed}: {:.2} s, {} moves, {} compactions, peak {} MB, most of each table {}",
                secs, grew.moves, grew.compactions, gpu.ledger.take_peak() >> 20, fill.join(" ")
            );
            if grew.moves > 0 {
                apart.push(format!("{id} {seed}"));
            }
        }
    }
    assert!(apart.is_empty(), "moved or begun again: {apart:?}");
}
