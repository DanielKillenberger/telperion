//! How much of the recorded cycle the host records each cycle (DESIGN.md
//! section 5: "the bounds start from the last readback's counts plus a
//! margin"). The plan holds every dispatch up to its bounds; a cycle
//! records a dispatch only where its conditions hold under the cycle's
//! limits, which the host sets from the census it read a cycle late and
//! writes into the census, so a count past a limit raises its flag on the
//! device.
use super::build_plan::Bounds;
use super::census as c;

/// A condition on a recorded step: its range, round, pass, level or
/// generation against the cycle's limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Cond {
    Units(u32),
    Draw(u32),
    Deposit(u32),
    Sweep(u32),
    Round(u32),
    /// A units round of the living apexes (0) or the wake (1), DESIGN.md
    /// section 10.
    UnitRound(u32, u32),
    Pass(u32),
    Level(u32),
    /// A sweep's folded top levels, where the cycle folds any.
    Top,
    LayGen(u32),
}

/// The cycle's limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Limits {
    pub units: u32,
    pub draws: u32,
    pub deposits: u32,
    pub sweeps: u32,
    pub rounds: u32,
    pub passes: u32,
    /// The block levels a sweep dispatches one by one, `[top, tail)`: the
    /// levels above `top` and from `tail` are folded (`fold.wgsl`).
    pub top: u32,
    pub tail: u32,
    pub laygens: u32,
    pub unit_rounds: [u32; 2],
}

/// Where the limits stand in the census, in `Limits::words`' order.
pub(super) const REC: usize = c::REC;

impl Limits {
    /// Every dispatch the plan holds.
    pub(super) fn full(b: &Bounds) -> Self {
        Self {
            units: b.units,
            draws: b.lay.draws,
            deposits: b.lay.deposits,
            sweeps: b.lay.sweeps,
            rounds: b.hier.gens,
            passes: b.hier.passes,
            top: 0,
            tail: b.hier.levels,
            laygens: b.lay.generations,
            unit_rounds: [b.rounds; 2],
        }
    }

    /// The limits no greater than the plan's bounds.
    pub(super) fn within(self, b: &Bounds) -> Self {
        let f = Self::full(b);
        Self {
            units: self.units.clamp(1, f.units),
            draws: self.draws.clamp(1, f.draws),
            deposits: self.deposits.clamp(1, f.deposits),
            sweeps: self.sweeps.clamp(1, f.sweeps),
            rounds: self.rounds.clamp(1, f.rounds),
            passes: self.passes.clamp(1, f.passes),
            tail: self.tail.min(f.tail),
            top: self.top.min(self.tail.min(f.tail)),
            laygens: self.laygens.clamp(1, f.laygens),
            unit_rounds: self.unit_rounds.map(|r| r.clamp(1, b.rounds)),
        }
    }

    pub(super) fn allows(&self, c: &Cond) -> bool {
        match *c {
            Cond::Units(r) => r < self.units,
            Cond::Draw(r) => r < self.draws,
            Cond::Deposit(r) => r < self.deposits,
            Cond::Sweep(k) => k < self.sweeps,
            Cond::Round(r) => r < self.rounds,
            Cond::UnitRound(w, r) => r < self.unit_rounds[w as usize],
            Cond::Pass(p) => p < self.passes,
            Cond::Level(l) => self.top <= l && l < self.tail,
            Cond::Top => self.top > 0,
            Cond::LayGen(g) => g < self.laygens,
        }
    }

    /// The census's words the device's flags read.
    pub(super) fn words(&self) -> [u32; 11] {
        let [a, b] = self.unit_rounds;
        [
            self.units,
            self.draws,
            self.deposits,
            self.sweeps,
            self.rounds,
            self.passes,
            self.top,
            self.tail,
            self.laygens,
            a,
            b,
        ]
    }
}

/// The census word holding a limit.
pub(super) mod rec {
    use super::REC;
    pub(in super::super) const UNITS: usize = REC;
    pub(in super::super) const DRAWS: usize = REC + 1;
    pub(in super::super) const DEPOSITS: usize = REC + 2;
    pub(in super::super) const SWEEPS: usize = REC + 3;
    pub(in super::super) const ROUNDS: usize = REC + 4;
    pub(in super::super) const TOP: usize = REC + 6;
    pub(in super::super) const TAIL: usize = REC + 7;
    pub(in super::super) const LAYGENS: usize = REC + 8;
}
