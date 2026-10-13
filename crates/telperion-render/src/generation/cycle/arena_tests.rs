//! fn-256 task 5 (the host's ruling 2 of 2026-10-12): what each table
//! holds at most against the closed form's hint, every shipped preset at
//! its default age and seeds 1, 7 and 42, compacted after every cycle: the
//! ratios `build_size::ARENA` states. `cargo test --profile ci -p
//! telperion-render arena_ratios -- --ignored --nocapture`, under the GPU
//! lock; `ARENA_PRESETS` names a comma list, `ARENA_AGE` an age for them
//! all (the oak at 80, fn-256 task 6).
use super::build::Tables;
use super::build_grow::{grow_now, Compact, Sizes};
use super::build_arena::{hint, TABLES};
use super::growth::tests::digest;
use super::grown_tests::{device, grown_from};
use super::settle_run::settle_now;
use telperion_space::SettleTo;
use super::plan_units::rounds_used;
use super::census as c;
use telperion_core::presets::Preset;
use telperion_space::NodeLaw;

const NAMES: [&str; TABLES] = ["axes", "units", "phytomers", "sleepers", "living", "woken", "leaves", "nodes"];

#[test]
#[ignore = "a measurement"]
fn arena_ratios() {
    let Some((gpu, kit)) = device() else {
        return;
    };
    // Held to the native default (`ARENA_BUDGET_MIB` to measure a tree
    // past it) and what the card leaves, as a build is.
    let free = gpu.free_device_memory().unwrap_or(u64::MAX);
    let budget = std::env::var("ARENA_BUDGET_MIB").ok().and_then(|m| m.parse::<u64>().ok()).map_or(4u64 << 30, |m| m << 20);
    let _capped = gpu.ledger.capped(budget.min(gpu.ledger.live().saturating_add(free / 100 * 85)));
    let only = std::env::var("ARENA_PRESETS").ok();
    let mut most = [0f64; TABLES];
    println!("| Preset | Seed | Hint | {} |", NAMES.join(" | "));
    let ids = telperion_core::presets::CATALOGUE.iter().map(|c| c.1);
    for id in ids.filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id))) {
        let preset = Preset::from_id(id).expect("a shipped preset");
        let family = preset.parameters();
        let age = aged(family.age as u32);
        // The gate's seeds, and the family's own (`skeleton.seed`).
        for seed in [1, 7, 42, u64::from(family.skeleton.seed)] {
            // The species and light a build grows by (`executor::growth`):
            // the supernatural rows change the Two Trees'.
            let growth = telperion_core::pipeline::executor::growth(&family).expect("a growth");
            let g = grown_from(growth.species, growth.request.light, (age, seed));
            let hinted = hint(&g.species, age).expect("a hint");
            let h = hinted.kept.max(1.0);
            let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
            let off = std::env::var("ARENA_COMPACT").is_ok_and(|c| c == "off");
            // `ARENA_LEAST`: grown from the least tables by moves, for a tree
            // the hinted arena does not fit (what it holds is the same).
            let least = std::env::var("ARENA_LEAST").is_ok();
            let sized = if least { Sizes::least() } else { Sizes::hinted(hinted).expect("sized") };
            let sizes = Sizes { compact: if off { Compact::Off } else { Compact::Every }, ..sized };
            println!("{id} {seed}: {hinted:?}, {:?}", sizes.caps);
            let (b, _, grew) = grow_now(&gpu, &kit, &t, &g.tables, (sizes, age)).expect("a build");
            drop(b);
            gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("freed");
            // The axes against the expected axes, the rest the kept phytomers.
            let of = |k: usize| if k == 0 { hinted.axes.max(1.0) } else { h };
            let ratios: Vec<f64> = grew.most.iter().enumerate().map(|(k, &n)| f64::from(n) / of(k)).collect();
            for (m, r) in most.iter_mut().zip(&ratios) {
                *m = m.max(*r);
            }
            let cells: Vec<String> = ratios.iter().map(|r| format!("{r:.4}")).collect();
            println!("| {id} | {seed} | {h:.0} | {}|", cells.join(" | "));
        }
    }
    let cells: Vec<String> = most.iter().map(|r| format!("{r:.4}")).collect();
    println!("| most | | | {} |", cells.join(" | "));
}

/// The host's gate (ruling 2 of 2026-10-12, as fn-270 R4 rules it):
/// every shipped preset at its default age, seeds 1, 7 and 42, grown from
/// the tables its hint sizes, its tables grown by observed counts, is
/// the tree grown in its final tables from the first cycle, word for
/// word, and never begins again. `ARENA_PRESETS` names a comma list, and
/// `ARENA_AGE` an age for them all.
#[test]
#[ignore = "device limits: every preset's whole tree, three seeds"]
fn every_preset_grows_from_its_hint_as_in_its_final_tables() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let k = super::tree::Kernels::new(&gpu);
    let free = gpu.free_device_memory().unwrap_or(u64::MAX);
    let _capped = gpu.ledger.capped((4u64 << 30).min(gpu.ledger.live().saturating_add(free / 100 * 85)));
    let only = std::env::var("ARENA_PRESETS").ok();
    let mut apart = Vec::new();
    let ids = telperion_core::presets::CATALOGUE.iter().map(|c| c.1);
    for id in ids.filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id))) {
        let family = Preset::from_id(id).expect("a shipped preset").parameters();
        let age = aged(family.age as u32);
        let growth = telperion_core::pipeline::executor::growth(&family).expect("a growth");
        for seed in [1, 7, 42] {
            let g = grown_from(growth.species.clone(), growth.request.light, (age, seed));
            let sizes = Sizes::hinted(hint(&g.species, age).expect("a hint")).expect("sized");
            let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
            let drawn = |sizes: Sizes| {
                gpu.ledger.take_peak();
                let clock = std::time::Instant::now();
                let (b, grown, grew) = grow_now(&gpu, &k.build, &t, &g.tables, (sizes, age)).unwrap_or_else(|e| panic!("{id} {seed}: {e}"));
                let secs = clock.elapsed().as_secs_f64();
                let b = pollster::block_on(b.settling(&gpu, (&t, &g.tables))).expect("settling");
                let s = settle_now(&gpu, (&k.build, &k.settle), (&b, &t), (&t.settle_pa, SettleTo::Laid)).expect("a settle");
                let d = digest(&gpu, &s);
                drop((s, b));
                gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("freed");
                println!("{id} {seed}: {secs:.2} s, {} moves, {} compactions, peak {} MB; {}", grew.moves, grew.compactions, gpu.ledger.take_peak() >> 20, rounds(&grew.censuses));
                (d, grown)
            };
            let (hinted, grown) = drawn(sizes);
            let (fixed, _) = drawn(Sizes { caps: grown.caps, points: grown.points, tips: grown.tips, nodes: grown.nodes, bounds: grown.bounds, ..sizes });
            if hinted != fixed {
                apart.push(format!("{id} {seed}"));
            }
        }
    }
    assert!(apart.is_empty(), "grown by moves, the tree differs: {apart:?}");
}

/// fn-256 task 15: a pole, one straight stem of 450 m growth units with
/// no laterals, stands a lattice of about 900 layers two nodes wide. Its
/// hint sizes the least arena's 4,096 nodes, which the cube they stood as
/// swept 640 layers of; it builds.
#[test]
fn a_tall_narrow_tree_builds() {
    let Some((gpu, kit)) = device() else {
        return;
    };
    let family = Preset::from_id("ordinary").expect("a shipped preset").parameters();
    let growth = telperion_core::pipeline::executor::growth(&family).expect("a growth");
    let mut species = growth.species;
    for s in &mut species.states {
        (s.viability, s.abortion, s.insertion) = (1.0, 0.0, 0.0);
        (s.form.wander, s.form.tropism) = (0.0, 0.0);
        for z in &mut s.zones {
            z.nodes = NodeLaw::Uniform { min: 20.0, max: 20.0 };
            z.lateral.iter_mut().chain(&mut z.dormant).for_each(|p| *p = 0.0);
        }
        s.internode = 450.0 / 20.0 / s.zones.len() as f64;
    }
    let age = 3;
    let g = grown_from(species, growth.request.light, (age, 1));
    let sizes = Sizes::hinted(hint(&g.species, age).expect("a hint")).expect("sized");
    let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
    let (_, _, grew) = grow_now(&gpu, &kit, &t, &g.tables, (sizes, age)).unwrap_or_else(|e| panic!("the pole: {e}"));
    let most = |w: usize| grew.censuses.iter().map(|c| c[w]).max().unwrap_or(0);
    let (wide, swept) = (most(c::GRID + 3).max(most(c::GRID + 4)), most(c::SWEPT));
    println!("the pole: {} nodes, {wide} wide, {swept} layers swept", sizes.nodes);
    assert!(sizes.nodes == 4096 && wide == 2 && swept > 640, "{} nodes, {wide} wide, {swept} layers", sizes.nodes);
}

/// The units rounds a build's cycles used, the living apexes' and the
/// wake's: the most, and the most any rose over the two cycles a limit
/// looks ahead (`build_size::limits`).
fn rounds(censuses: &[Vec<u32>]) -> String {
    let used = |w: usize| censuses.iter().map(|c| rounds_used(c, w)).collect::<Vec<_>>();
    let [live, wake] = [used(0), used(1)];
    let rise = |u: &[u32]| u.windows(3).map(|w| w[2].saturating_sub(w[0])).max().unwrap_or(0);
    let most = |u: &[u32]| u.iter().copied().max().unwrap_or(0);
    format!("units rounds used at most {} and {}, rising {} and {} over two cycles", most(&live), most(&wake), rise(&live), rise(&wake))
}

/// `ARENA_AGE` where it is set, else the preset's own age.
fn aged(own: u32) -> u32 {
    std::env::var("ARENA_AGE").ok().and_then(|a| a.parse().ok()).unwrap_or(own)
}

