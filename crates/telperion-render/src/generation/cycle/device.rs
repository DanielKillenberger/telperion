//! The cycle's units and wake on the device (DESIGN.md sections 2.1, 2.2
//! and 10): the units module's entries, a job's phases in a round of the
//! recorded cycle (`plan_units.rs`), and the places its tally and its
//! jobs' state take.
use super::super::io;
use crate::Gpu;

pub(super) const INIT: usize = 0;
pub(super) const WOKE: usize = 1;
pub(super) const DECIDE: usize = 2;
pub(super) const ZONES: usize = 3;
pub(super) const NODES: [usize; 2] = [4, 26];
/// The zones' records written: their nodes ordered, then their
/// phytomers, laterals, the laterals' presences and sleeping buds.
pub(super) const WRITTEN: [usize; 7] = [20, 21, 14, 22, 23, 15, 24];
/// A unit after its decision: its stop where it fails with no relay, or
/// the unit appended, where it lives or where it dies.
pub(super) const UNIT: [usize; 4] = [16, 17, 18, 19];
pub(super) const CLOSE: usize = 5;
pub(super) const FLAG: usize = 6;
pub(super) const GATHER: usize = 7;
pub(super) const SEED: usize = 8;
/// The operations after a unit, an entry each, in a round's order.
pub(super) const AFTER: [usize; 6] = [9, 10, 25, 11, 12, 13];

pub(super) const ENTRIES: [&str; 27] = [
    "init_main",
    "woke_main",
    "decide_main",
    "zone_main",
    "nodes_main",
    "close_main",
    "flag",
    "gather",
    "seed_main",
    "grown_main",
    "move_main",
    "stop_main",
    "relay_main",
    "ended_main",
    "laterals_main",
    "sleepers_main",
    "fails_main",
    "unit_main",
    "dies_main",
    "lived_main",
    "order_main",
    "phyts_main",
    "made_main",
    "placed_main",
    "asleep_main",
    "moved_main",
    "dormant_main",
];

/// The tally's places (`arena.wgsl`'s `T_` words).
pub(super) const T_UNITS: usize = 0;
pub(super) const T_UNITS_B: usize = 4;
pub(super) const T_LIST: usize = 8;
pub(super) const T_WAKE: usize = 12;
pub(super) const T_WAKE_B: usize = 16;
pub(super) const T_ROUNDS: usize = 21;

/// A job's words (`job.wgsl`'s `JW`), and a unit zone's (`grow.wgsl`'s
/// `Z_` columns).
pub(super) const JW: u32 = 178 + 6 * 35;
pub(super) const ZW: u32 = 6 + 64;

pub(super) struct Kernels {
    pub(super) cycle: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let mut reads = vec![true; 3];
        reads.extend([false; 5]);
        Self { cycle: io::Pass::new(gpu, super::source(), &reads, &ENTRIES) }
    }
}

/// Where the counts' second records start for `n` invocations: a multiple
/// of 64 words, so a scan binds them at 256 bytes.
pub(super) fn b_at(n: u32) -> u32 {
    (4 * n.max(1)).next_multiple_of(64)
}

/// The living apexes a range of the units' dispatches takes
/// (`Ranges::units`), its jobs at once.
pub(super) const RANGE: u32 = 65536;

/// Where `counts` holds what, for the state's `caps` and units of `zones`
/// zones at most: the jobs' state, the jobs a range takes (a range of
/// living apexes, or every sleeping bud), the zones', and its words.
#[derive(Debug, Clone, Copy)]
pub(super) struct Counts {
    pub job_at: u32,
    pub jobs: u32,
    pub zone_at: u32,
    pub words: u64,
}

pub(super) fn counts(caps: super::state::Caps, zones: u32) -> Counts {
    let b_at = b_at(caps.next.max(caps.sleepers));
    let jobs = RANGE.max(caps.sleepers);
    let job_at = 2 * b_at;
    let zone_at = (u64::from(job_at) + u64::from(JW) * u64::from(jobs)).next_multiple_of(64) as u32;
    let words = u64::from(zone_at) + u64::from(ZW * jobs * zones.max(1));
    Counts { job_at, jobs, zone_at, words }
}
