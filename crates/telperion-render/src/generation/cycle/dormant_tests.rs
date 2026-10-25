//! fn-256 task 9 (COUNCIL.md round 5, step 9): the lattice is dormant
//! where nothing shades. Every shipped preset's settled node table,
//! hashed, to hold before and after a change that keeps its words
//! (`node_tables_are_hashed`), and the dormant light's reads held to the
//! swept one's.
use super::build::Tables;
use super::build_arena::hint;
use super::build_grow::{grow_now, Sizes};
use super::grown_tests::{device, grown_from};
use super::hier_scan::read_words;
use super::settle_device::Kernels;
use super::settle_run::settle_now;
use telperion_core::presets::Preset;
use telperion_space::SettleTo;

/// FNV-1a over the words.
fn hash(words: &[u32]) -> u64 {
    words
        .iter()
        .flat_map(|w| w.to_le_bytes())
        .fold(0xcbf29ce484222325, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
        })
}

/// Each shipped preset at its default age and seed, grown as a build
/// grows it (`executor::growth`, the hinted arena) and laid: its node
/// table's words and their hash. `NODES_PRESETS` names a comma list,
/// `NODES_CAPS` the hierarchy's block caps (the build's, else), and
/// `NODES_CONTEXTS` how many fresh device contexts run them all (one,
/// else). `cargo test --profile ci -p telperion-render
/// node_tables_are_hashed -- --ignored --nocapture`, under the GPU lock.
#[test]
#[ignore = "a measurement: every preset's whole tree"]
fn node_tables_are_hashed() {
    let only = std::env::var("NODES_PRESETS").ok();
    let list = |key: &str| {
        std::env::var(key).ok().map(|v| {
            v.split(',')
                .map(|x| x.parse::<u32>().expect("a number"))
                .collect::<Vec<_>>()
        })
    };
    let caps = list("NODES_CAPS");
    let contexts = list("NODES_CONTEXTS").map_or(1, |c| c[0]);
    for context in 1..=contexts {
        let Some((gpu, kit)) = device() else {
            return;
        };
        let free = gpu.free_device_memory().unwrap_or(u64::MAX);
        // What the card leaves: the Two Trees' settle and readback pass 4 GiB.
        let _capped = gpu
            .ledger
            .capped(gpu.ledger.live().saturating_add(free / 100 * 85));
        let k = Kernels::new(&gpu);
        let ids = telperion_core::presets::CATALOGUE.iter().map(|c| c.1);
        for id in ids.filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id))) {
            let family = Preset::from_id(id).expect("a shipped preset").parameters();
            let (age, seed) = (family.age as u32, u64::from(family.skeleton.seed));
            let growth = telperion_core::pipeline::executor::growth(&family).expect("a growth");
            let g = grown_from(growth.species, growth.request.light, (age, seed));
            let hinted = Sizes::hinted(hint(&g.species, age).expect("a hint")).expect("sized");
            let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
            for cap in caps.clone().unwrap_or(vec![hinted.block_cap]) {
                let sizes = Sizes {
                    block_cap: cap,
                    ..hinted
                };
                let (b, ..) = grow_now(&gpu, &kit, &t, &g.tables, (sizes, age))
                    .unwrap_or_else(|e| panic!("{id}: {e}"));
                let s = settle_now(
                    &gpu,
                    (&kit, &k),
                    (&b, &t),
                    (&super::settle_tests::constants(&g), SettleTo::Laid),
                )
                .expect("a settle");
                let mut words = s.error.to_vec();
                words.extend(read_words(&gpu, &s.nt, 0, s.table.words).expect("a readback"));
                println!("nodes {id} {age} {seed}: {} words, hash {:016x} (block cap {cap}, context {context})", words.len(), hash(&words));
                drop((s, b));
                gpu.device
                    .poll(wgpu::PollType::wait_indefinitely())
                    .expect("freed");
            }
        }
    }
}
