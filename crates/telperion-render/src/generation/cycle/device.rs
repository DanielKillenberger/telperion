//! The cycle's units and wake on the device (DESIGN.md sections 2.1, 2.2
//! and 10): the units module's entries, a job's phases in a round of the
//! recorded cycle (`plan_units.rs`), and the places its tally and its
//! jobs' state take.
use super::super::io;
use super::state::Caps;
use crate::{Gpu, RenderError, Result};
use telperion_core::Error;

pub(super) const INIT: usize = 0;
/// A round's phases (DESIGN.md section 10, re-fused in fn-256 task 6), each
/// cut where its cold compile stays small: a waking bud woken on its next
/// axis (the wake's rounds after the first); its unit decided and what
/// follows the decision, a job each; its zones laid and counted, and,
/// emitted, written, then their presences placed; what follows each unit.
pub(super) const WOKE: usize = 1;
pub(super) const DECIDE: usize = 2;
pub(super) const ZONES: usize = 3;
pub(super) const WRITTEN: [usize; 3] = [4, 11, 5];
pub(super) const AFTER: usize = 6;
pub(super) const CLOSE: usize = 7;
pub(super) const FLAG: usize = 8;
pub(super) const GATHER: usize = 9;
pub(super) const SEED: usize = 10;

pub(super) const ENTRIES: [&str; 12] = [
    "init_main",
    "woke_main",
    "decide_main",
    "zones_main",
    "written_main",
    "placed_main",
    "after_main",
    "close_main",
    "flag",
    "gather",
    "seed_main",
    "sprouted_main",
];

/// The tally's places (`arena.wgsl`'s `T_` words).
pub(super) const T_UNITS: usize = 0;
pub(super) const T_UNITS_B: usize = 4;
pub(super) const T_LIST: usize = 8;
pub(super) const T_WAKE: usize = 12;
pub(super) const T_WAKE_B: usize = 16;
pub(super) const T_ROUNDS: usize = 21;
/// The rounds the cycle's units used, the living apexes' then the wake's.
pub(super) const T_USED: usize = 22;

/// A job's words (`job.wgsl`'s `JW`), and a unit zone's (`grow.wgsl`'s
/// `Z_` columns).
pub(super) const JW: u32 = 130 + FRAMES * 35 + (FRAMES - 1) * 6;
/// The axes a units job holds at once (`job.wgsl`'s `FRAMES`): the most
/// units it grows in a cycle where none fails. 11, the most floor
/// (`telperion_space::most_units`) of every preset at every age to 80 and
/// the film's walk frames (fn-276, host 2026-10-14: palm to Laurelin at
/// t = 0.01 chains eleven sliver ages); a species past it is refused by
/// name before its build.
pub(super) const FRAMES: u32 = 11;
/// A zone: six words, then its order along the unit, a byte a node.
pub(super) const ZW: u32 = 6 + 64 / 4;

pub(super) struct Kernels {
    pub(super) cycle: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let mut reads = vec![true; 3];
        reads.extend([false; 5]);
        Self {
            cycle: io::Pass::new(gpu, super::source(), &reads, &ENTRIES),
        }
    }
}

/// The living apexes a range of the units' dispatches takes
/// (`Ranges::units`), its jobs at once: every range a units round
/// dispatches again, and the work a dispatch takes (fn-256 task 6: at
/// 262,144 Telperion's `written_main` took 5.9 ms, past the 2 ms bound;
/// at 65,536, 1.7 ms).
pub(super) const RANGE: u32 = 65_536;

/// Where `counts` holds what, for the state's `caps` and units of `zones`
/// zones at most: the units' second count records (a multiple of 64
/// words, so a scan binds them at 256 bytes), the jobs' state, the jobs a
/// range takes (a range of living apexes, or every sleeping bud), the
/// zones', and its words.
#[derive(Debug, Clone, Copy)]
pub(super) struct Counts {
    pub b_at: u32,
    pub job_at: u32,
    pub jobs: u32,
    pub zone_at: u32,
    pub words: u64,
}

/// The most bytes a storage buffer of the device holds.
pub(super) fn limit(gpu: &Gpu) -> u64 {
    let l = gpu.device.limits();
    l.max_buffer_size.min(l.max_storage_buffer_binding_size)
}

/// `counts` sized in u64 and refused by name, before any buffer is made,
/// where it would pass `limit` bytes or the words a u32 index reaches.
pub(super) fn counts(caps: Caps, zones: u32, limit: u64) -> Result<Counts> {
    let refused = || RenderError::from(Error::ResourceLimit("the cycle's job counts"));
    let b_at = (4 * u64::from(caps.next.max(caps.sleepers).max(1))).next_multiple_of(64);
    // A tree of fewer living than a range holds its jobs alone.
    let jobs = RANGE.min(caps.next).max(caps.sleepers);
    let job_at = 2 * b_at;
    let zone_at = (job_at + u64::from(JW) * u64::from(jobs)).next_multiple_of(64);
    let words = (u64::from(ZW) * u64::from(jobs))
        .checked_mul(u64::from(zones.max(1)))
        .and_then(|zoned| zoned.checked_add(zone_at))
        .filter(|&words| words <= (limit / 4).min(1 << 32))
        .ok_or_else(refused)?;
    let word = |at: u64| u32::try_from(at).map_err(|_| refused());
    Ok(Counts {
        b_at: word(b_at)?,
        job_at: word(job_at)?,
        jobs,
        zone_at: word(zone_at)?,
        words,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn caps(sleepers: u32) -> Caps {
        Caps {
            axes: 64,
            units: 64,
            phyts: 64,
            sleepers,
            next: 64,
            woken: 64,
        }
    }

    fn refused(c: &Result<Counts>) -> bool {
        matches!(
            c,
            Err(RenderError::Generation(Error::ResourceLimit(
                "the cycle's job counts"
            )))
        )
    }

    /// At 7.47 million sleepers the jobs' state alone passes 2^32 words:
    /// sized in u64, it is refused by name whatever the device holds,
    /// never sized wrapped. A million sleepers are sized whole.
    #[test]
    fn the_jobs_past_u32_words_are_refused_not_sized_wrapped() {
        assert!(u64::from(JW) * 7_470_000 > u64::from(u32::MAX));
        assert!(refused(&counts(caps(7_470_000), 4, u64::MAX)));
        let s: u64 = 1_000_000;
        let jobs = (2 * (4 * s).next_multiple_of(64) + u64::from(JW) * s).next_multiple_of(64);
        let c = counts(caps(s as u32), 4, u64::MAX).expect("sized");
        assert_eq!(c.words, jobs + u64::from(ZW) * s * 4);
        assert_eq!(u64::from(c.zone_at), jobs);
    }

    /// The counts' bytes against the device's limit: one below it and at
    /// it they are sized (a buffer of the limit is one the device makes),
    /// one byte past it they are refused by name.
    #[test]
    fn job_counts_past_the_device_limit_are_refused_by_name() {
        let bytes = 4 * counts(caps(1_000_000), 4, u64::MAX).expect("sized").words;
        for (limit, fits) in [(bytes + 1, true), (bytes, true), (bytes - 1, false)] {
            let c = counts(caps(1_000_000), 4, limit);
            assert_eq!(!refused(&c), fits, "{bytes} bytes under a limit of {limit}");
            assert!(fits == c.is_ok());
        }
    }
}
