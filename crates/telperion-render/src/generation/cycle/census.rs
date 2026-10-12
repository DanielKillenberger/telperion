//! The device's counts (DESIGN.md section 5): one buffer of words the
//! cycle's points read and write (`vm.wgsl`), the tallies of the phases
//! copied in where they land, and a flag a word for each bound or
//! capacity a count passed. The host reads it a cycle late, never to
//! size a dispatch.

/// The cycle the host is recording.
pub(super) const CYCLE: usize = 0;
/// What the state holds: axes, units, phytomers, sleeping buds, woken
/// buds, the living a cycle leaves, the axes laid, those the hierarchy
/// holds, and the phytomers grown.
pub(super) const AXES: usize = 1;
pub(super) const UNITS: usize = 2;
pub(super) const PHYTS: usize = 3;
pub(super) const SLEEPERS: usize = 4;
pub(super) const WOKEN: usize = 5;
pub(super) const NEXT: usize = 6;
pub(super) const LAID: usize = 7;
pub(super) const BUILT: usize = 8;
pub(super) const GROWN: usize = 9;
/// The same at the cycle's start (`LIVE0` the living before the
/// balance), copied by its first point.
pub(super) const AXES0: usize = 11;
pub(super) const UNITS0: usize = 12;
pub(super) const PHYTS0: usize = 13;
pub(super) const SLEEPERS0: usize = 14;
pub(super) const WOKEN0: usize = 15;
pub(super) const LIVE0: usize = 16;
pub(super) const LAID0: usize = 17;
/// The cycle's: the living after the balance, the hierarchy's
/// generations, its blocks and levels before its step and its levels
/// after, the axes that grew.
pub(super) const LIVE: usize = 18;
pub(super) const GENS: usize = 19;
pub(super) const BLOCKS0: usize = 20;
pub(super) const LEVELS0: usize = 21;
pub(super) const LEVELS: usize = 22;
pub(super) const GREW: usize = 23;
/// The units' tally as the cycle left it (`arena.wgsl`'s `T_`), 24 words.
pub(super) const MADE: usize = 32;
/// Each block level's first block after the hierarchy's step.
pub(super) const LVL: usize = 64;
/// The light's grid (low, size, nodes, standing), its errors, and the
/// layers its sweep takes.
pub(super) const GRID: usize = 128;
pub(super) const LIGHT_ERRORS: usize = 136;
pub(super) const SWEPT: usize = 137;
/// The hierarchy's axes left unplaced after each round's last pass, its
/// faults, and the layout's faults and deepest generations.
pub(super) const UNPLACED: usize = 140;
pub(super) const HIER_FAULT: usize = 172;
pub(super) const LAY_FAULT: usize = 173;
pub(super) const LAY_DEEP: usize = 176;
/// What the cycle's retirement reused (fn-256 task 10, DESIGN.md section
/// 16), counted where a measurement asks (`Tables::counted`), `REUSED`
/// words (`Reused`).
pub(super) const REUSE: usize = 156;
#[cfg_attr(not(test), allow(dead_code))]
pub(super) const REUSED: usize = 3;
/// The words of `REUSE`, in order: the laid axes closed, those closed
/// this cycle (the ones retirement carries up), and the blocks its
/// living's sweep entered.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Reused {
    Closed,
    ClosedNow,
    Blocks,
}
/// A flag a word: set where a count passed its capacity or bound, or a
/// phase faulted.
pub(super) const FLAGS: usize = 192;
/// The cycle's limits (`gate.rs`), which the host writes as it does the
/// cycle.
pub(super) const REC: usize = 224;
/// The hierarchy's axes left unplaced after each pass, a round's
/// `PASSES` words after another's.
pub(super) const PASS_LEFT: usize = 256;
pub(super) const PASSES: usize = 8;
pub(super) const ROUNDS: usize = 16;
pub(super) const WORDS: usize = 512;

/// The flags, by what each names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Flag {
    Axes,
    Units,
    Phyts,
    Sleepers,
    Next,
    Woken,
    Budget,
    UnitFaults,
    HierGens,
    HierPasses,
    HierFault,
    LayGenerations,
    LayFault,
    UnitRanges,
    DrawRanges,
    DepositRanges,
    Sweeps,
    Points,
    LightErrors,
    Collapsed,
    /// A dispatch's true size past the workgroups the host gave it.
    Given,
    /// A units job with operations left past the cycle's last round.
    UnitRounds,
    /// A table holding more rows than the compaction after the cycle
    /// covered (`compact.rs`).
    Compaction,
}

pub(super) const FLAG_NAMES: [Flag; 23] = [
    Flag::Axes,
    Flag::Units,
    Flag::Phyts,
    Flag::Sleepers,
    Flag::Next,
    Flag::Woken,
    Flag::Budget,
    Flag::UnitFaults,
    Flag::HierGens,
    Flag::HierPasses,
    Flag::HierFault,
    Flag::LayGenerations,
    Flag::LayFault,
    Flag::UnitRanges,
    Flag::DrawRanges,
    Flag::DepositRanges,
    Flag::Sweeps,
    Flag::Points,
    Flag::LightErrors,
    Flag::Collapsed,
    Flag::Given,
    Flag::UnitRounds,
    Flag::Compaction,
];

impl Flag {
    pub(super) fn word(self) -> usize {
        FLAGS + FLAG_NAMES.iter().position(|f| *f == self).expect("a flag")
    }
}

/// The flags set in `census`, each with its word's value.
pub(super) fn raised(census: &[u32]) -> Vec<(Flag, u32)> {
    FLAG_NAMES.iter().map(|&f| (f, census[f.word()])).filter(|f| f.1 != 0).collect()
}
