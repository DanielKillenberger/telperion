//! A bud's expected envelope (fn-209, the stand-in's shape): the wood its
//! subtree is expected to grow, by length, and that wood's first and
//! second moments about the bud's base, the first along the bud's heading.
//! The same recursion as the counts: each lateral, relay and continuation
//! stands where `along.rs` puts it, turned from the axis by its insertion
//! angle, its own envelope scaled by its base scale (a length by it, a
//! moment by its square, a second moment by its cube).
//!
//! The axes are taken straight and every bud's azimuth as even: what a
//! lateral's subtree leans off its own heading averages out about its
//! bearer. A tree that bends (tropism, wander, sag, straightening) or
//! turns its relays by its phyllotaxis departs from it; how far is
//! measured, not assumed (`tests.rs`).
use super::events::{events, Event};
use super::{sub, Sub};
use crate::dormant::{First, Living};
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::Species;
use std::collections::HashMap;

/// A subtree's wood about its bud's base, per unit of the bud's base
/// scale: Σ l, Σ l (x · heading) and Σ l |x|², over its internodes'
/// lengths l and middles x.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Envelope {
    pub wood: f64,
    pub along: f64,
    pub square: f64,
}

impl Envelope {
    /// The envelope of the same subtree grown at base scale `s`.
    pub fn scaled(self, s: f64) -> Self {
        Self {
            wood: self.wood * s,
            along: self.along * s * s,
            square: self.square * s * s * s,
        }
    }

    /// How far along the heading its wood's centre stands.
    #[allow(dead_code)]
    pub fn centre(&self) -> f64 {
        if self.wood > 0.0 {
            self.along / self.wood
        } else {
            0.0
        }
    }

    /// The root mean square distance of its wood from its base: a mean,
    /// not a bound on how far any of it reaches.
    #[allow(dead_code)]
    pub fn reach(&self) -> f64 {
        if self.wood > 0.0 {
            (self.square / self.wood).sqrt()
        } else {
            0.0
        }
    }
}

/// The species' stand-ins at a tree's age: each bud's envelope by its PA,
/// the cycles left and what it carries, worked out once.
pub(crate) struct StandIns<'a> {
    species: &'a Species,
    laterals: Vec<Vec<Vec<f64>>>,
    living: Living<'a>,
    memo: HashMap<Sub, Envelope>,
}

impl<'a> StandIns<'a> {
    /// The stand-ins of a tree whose windows are `windows`: an apex that
    /// all but stopped moving on grows its relay as well (host decision
    /// 13), as the counts have it.
    pub fn new(species: &'a Species, windows: &'a Windows) -> Self {
        Self {
            species,
            laterals: species.states.iter().map(|s| s.laterals()).collect(),
            living: Living::new(species, Some(windows)),
            memo: HashMap::new(),
        }
    }

    /// The envelope of a bud of PA `k` made `m` cycles before the tree's
    /// age, carrying `carried`; a woken one with its first unit's law. A
    /// bud of an age it passes through grows at the share of its size that
    /// reaches the next it lives in.
    pub fn of(&mut self, k: usize, m: usize, carried: Carried, first: Option<First>) -> Envelope {
        let Some((k, reach)) = self.species.lived(k) else {
            return Envelope::default();
        };
        self.lived(k, m, carried, first).scaled(reach)
    }

    fn lived(&mut self, k: usize, m: usize, carried: Carried, first: Option<First>) -> Envelope {
        if m == 0 {
            return Envelope::default();
        }
        let key = sub(self.species, k, m, carried, first);
        if let Some(&envelope) = self.memo.get(&key) {
            return envelope;
        }
        let mut sum = Envelope::default();
        let laterals = &self.laterals[k];
        for event in events(
            self.species,
            laterals,
            &mut self.living,
            (k, m, carried),
            first,
        ) {
            match event {
                Event::Nodes { wood, .. } => {
                    sum.wood += wood[0];
                    sum.along += wood[1];
                    sum.square += wood[2];
                }
                Event::Bud {
                    at,
                    pa,
                    carried,
                    weight,
                    first,
                    unit,
                    stand,
                    ..
                } => {
                    let c = self.of(pa, m - at, carried, first);
                    let turn = stand.turn[0];
                    let [d1, d2, d3] = stand.dominance;
                    let [s1, s2, s3] = stand.shares;
                    let (k1, k2, k3) = (unit * d1, unit * unit * d2, unit.powi(3) * d3);
                    // Its base at x along the axis, its wood turned by its
                    // angle: |x h + R y|² = x² + 2 x cos(angle) (y · h') + |y|².
                    sum.wood += weight * k1 * s1 * c.wood;
                    sum.along += weight * (k1 * stand.along[0] * c.wood + turn * k2 * s2 * c.along);
                    sum.square += weight
                        * (k1 * stand.square * c.wood
                            + 2.0 * turn * k2 * stand.along[1] * c.along
                            + k3 * s3 * c.square);
                }
            }
        }
        self.memo.insert(key, sum);
        sum
    }
}

#[cfg(test)]
mod tests;
