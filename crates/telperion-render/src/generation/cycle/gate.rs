//! How much of the recorded cycle the host records each cycle (DESIGN.md
//! section 5: "the bounds start from the last readback's counts plus a
//! margin"). The plan holds every dispatch up to its bounds; a cycle
//! records a dispatch only where its conditions hold under the cycle's
//! limits, which the host sets from the census it read a cycle late and
//! writes into the census, so a count past a limit raises its flag on the
//! device.
use super::build_plan::Bounds;
use super::census as c;

/// The scans whose levels are limited, by the rows they take.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Scan {
    /// The hierarchy step's children (the laid axes), its blocks' joiners
    /// and members (blocks and new axes), its openers (new axes), its
    /// levels; the sums' living flags; the layout's grown axes; the units'
    /// living apexes and sleeping buds; the layout's new axes.
    Kids,
    Blocks,
    Open,
    Levels,
    Live,
    Grew,
    Units,
    Sleep,
    Lay,
}

pub(super) const SCANS: usize = 9;

/// A condition on a recorded step: its range, round, pass, level,
/// generation or scan level against the cycle's limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Cond {
    Units(u32),
    Draw(u32),
    Deposit(u32),
    Light(u32),
    Sweep(u32),
    Round(u32),
    /// A units round of the living apexes (0) or the wake (1), DESIGN.md
    /// section 10.
    UnitRound(u32, u32),
    Pass(u32),
    Level(u32),
    LayGen(u32),
    Scan(Scan, u32),
}

/// The cycle's limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Limits {
    pub units: u32,
    pub draws: u32,
    pub deposits: u32,
    pub lights: u32,
    pub sweeps: u32,
    pub rounds: u32,
    pub passes: u32,
    pub levels: u32,
    pub laygens: u32,
    pub scans: [u32; SCANS],
    pub unit_rounds: [u32; 2],
}

/// The scans' levels a plan holds at most.
pub(super) const SCAN_LEVELS: u32 = 8;

/// Where the limits stand in the census, in `Limits::words`' order.
pub(super) const REC: usize = c::REC;

impl Limits {
    /// Every dispatch the plan holds.
    pub(super) fn full(b: &Bounds) -> Self {
        Self {
            units: b.units,
            draws: b.lay.draws,
            deposits: b.lay.deposits,
            lights: b.lay.lights,
            sweeps: b.lay.sweeps,
            rounds: b.hier.gens,
            passes: b.hier.passes,
            levels: b.hier.levels,
            laygens: b.lay.generations,
            scans: [SCAN_LEVELS; SCANS],
            unit_rounds: [b.rounds; 2],
        }
    }

    /// The limits no greater than the plan's bounds.
    pub(super) fn within(self, b: &Bounds) -> Self {
        let f = Self::full(b);
        let s = self.scans;
        Self {
            units: self.units.clamp(1, f.units),
            draws: self.draws.clamp(1, f.draws),
            deposits: self.deposits.clamp(1, f.deposits),
            lights: self.lights.clamp(1, f.lights),
            sweeps: self.sweeps.clamp(1, f.sweeps),
            rounds: self.rounds.clamp(1, f.rounds),
            passes: self.passes.clamp(1, f.passes),
            levels: self.levels.clamp(1, f.levels),
            laygens: self.laygens.clamp(1, f.laygens),
            scans: std::array::from_fn(|k| s[k].clamp(1, SCAN_LEVELS)),
            unit_rounds: self.unit_rounds.map(|r| r.clamp(1, b.rounds)),
        }
    }

    pub(super) fn allows(&self, c: &Cond) -> bool {
        match *c {
            Cond::Units(r) => r < self.units,
            Cond::Draw(r) => r < self.draws,
            Cond::Deposit(r) => r < self.deposits,
            Cond::Light(r) => r < self.lights,
            Cond::Sweep(k) => k < self.sweeps,
            Cond::Round(r) => r < self.rounds,
            Cond::UnitRound(w, r) => r < self.unit_rounds[w as usize],
            Cond::Pass(p) => p < self.passes,
            Cond::Level(l) => l < self.levels,
            Cond::LayGen(g) => g < self.laygens,
            Cond::Scan(s, l) => l < self.scans[s as usize],
        }
    }

    /// The census's words the device's flags read.
    pub(super) fn words(&self) -> [u32; 11 + SCANS] {
        let mut out = [0; 11 + SCANS];
        let head = [self.units, self.draws, self.deposits, self.lights, self.sweeps, self.rounds, self.passes, self.levels, self.laygens];
        out[..9].copy_from_slice(&head);
        out[9..9 + SCANS].copy_from_slice(&self.scans);
        out[9 + SCANS..].copy_from_slice(&self.unit_rounds);
        out
    }
}

/// The census word holding a limit.
pub(super) mod rec {
    use super::REC;
    pub(in super::super) const UNITS: usize = REC;
    pub(in super::super) const DRAWS: usize = REC + 1;
    pub(in super::super) const DEPOSITS: usize = REC + 2;
    pub(in super::super) const LIGHTS: usize = REC + 3;
    pub(in super::super) const SWEEPS: usize = REC + 4;
    pub(in super::super) const ROUNDS: usize = REC + 5;
    pub(in super::super) const LEVELS: usize = REC + 7;
    pub(in super::super) const LAYGENS: usize = REC + 8;
    pub(in super::super) fn scan(s: super::Scan) -> usize {
        REC + 9 + s as usize
    }
}

/// The levels `hier_scan.wgsl` takes over `rows` (the first's and each
/// above it while the one below passes a span).
pub(super) fn levels_of(rows: u32) -> u32 {
    let (mut n, mut l) = (rows, 1);
    while n > 256 {
        n = n.div_ceil(256);
        l += 1;
    }
    l
}
