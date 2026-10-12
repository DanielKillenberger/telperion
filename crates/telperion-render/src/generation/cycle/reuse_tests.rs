//! fn-256 task 10 (COUNCIL.md round 5, step 10), measured: what changes
//! in each cycle's inputs to the upward sums and retirement, and what
//! they take (`census::REUSE`, counted where `Tables::counted`), for
//! Telperion 82, the oak 40 and the beech 40 at seed 42, grown as a build
//! grows them. Each cycle's words go to `REUSE_OUT`; `REUSE_PRESETS`
//! names a comma list. `cargo test --profile ci -p telperion-render
//! reuse_is_counted -- --ignored --nocapture`, under the GPU lock.
use super::build::Tables;
use super::build_arena::hint;
use super::build_grow::{grow_now, Sizes};
use super::census::{self as c, Reused};
use super::grown_tests::{device, grown_from};
use std::fmt::Write as _;
use std::path::PathBuf;
use telperion_core::presets::Preset;

const CASES: [(&str, u32, u64); 3] = [("telperion", 82, 42), ("oregon-white-oak", 40, 42), ("european-beech", 40, 42)];

/// The census words beside the count: the laid axes, the living before
/// and after the balance, the axes, the woken buds and the units at the
/// cycle's start and end, and the hierarchy's levels.
const BESIDE: [(&str, usize); 10] = [
    ("laid", c::LAID0),
    ("live0", c::LIVE0),
    ("live", c::LIVE),
    ("axes0", c::AXES0),
    ("axes", c::AXES),
    ("woken0", c::WOKEN0),
    ("woken", c::WOKEN),
    ("sleepers0", c::SLEEPERS0),
    ("sleepers", c::SLEEPERS),
    ("levels", c::LEVELS),
];

#[test]
#[ignore = "a measurement"]
fn reuse_is_counted() {
    let out = PathBuf::from(std::env::var("REUSE_OUT").expect("REUSE_OUT names a directory"));
    std::fs::create_dir_all(&out).expect("the output directory");
    let Some((gpu, kit)) = device() else {
        return;
    };
    let only = std::env::var("REUSE_PRESETS").ok();
    for (id, age, seed) in CASES.into_iter().filter(|c| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == c.0))) {
        let growth = telperion_core::pipeline::executor::growth(&Preset::from_id(id).expect("a preset").parameters()).expect("a growth");
        let g = grown_from(growth.species, growth.request.light, (age, seed));
        let mut t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
        t.counted = true;
        let hinted = Sizes::hinted(hint(&g.species, age).expect("a hint")).expect("sized");
        let (b, _, grew) = grow_now(&gpu, &kit, &t, &g.tables, (hinted, age)).expect("a build");
        drop(b);
        gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("freed");
        let mut csv = String::from("cycle");
        BESIDE.iter().for_each(|(name, _)| write!(csv, ",{name}").unwrap());
        csv.push_str(",blocks");
        (0..c::REUSED).for_each(|w| write!(csv, ",{:?}", word(w)).unwrap());
        let mut total = [0u64; c::REUSED];
        for (k, read) in grew.censuses.iter().enumerate() {
            write!(csv, "\n{}", k + 1).unwrap();
            BESIDE.iter().for_each(|(_, w)| write!(csv, ",{}", read[*w]).unwrap());
            write!(csv, ",{}", read[c::LVL + read[c::LEVELS] as usize]).unwrap();
            for (w, t) in total.iter_mut().enumerate() {
                write!(csv, ",{}", read[c::REUSE + w]).unwrap();
                *t += u64::from(read[c::REUSE + w]);
            }
        }
        std::fs::write(out.join(format!("{id}-reuse.csv")), csv).expect("written");
        let sum: Vec<String> = total.iter().enumerate().map(|(w, t)| format!("{:?} {t}", word(w))).collect();
        println!("{id} {age}: {}", sum.join(", "));
    }
}

/// The count's word `w`, named.
fn word(w: usize) -> Reused {
    const ALL: [Reused; c::REUSED] = [
        Reused::TotalsNew,
        Reused::TotalsLight,
        Reused::TotalsWood,
        Reused::TotalsFrozen,
        Reused::TotalsTook,
        Reused::TotalsBlocks,
        Reused::SubNew,
        Reused::SubOwn,
        Reused::SubTook,
        Reused::SubBlocks,
        Reused::Lit,
        Reused::Growing,
        Reused::GrowingHeld,
        Reused::Closed,
        Reused::ClosedNow,
        Reused::RetireBlocks,
    ];
    assert_eq!(ALL[w] as usize, w, "the words in order");
    ALL[w]
}
