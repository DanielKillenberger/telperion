//! What a bud's subtree is expected to keep at the tree's age (fn-209,
//! host decisions 9 and 12): its wood and that wood's moments, its pipe's
//! section, its mass for sag and the mass's moment, with shedding taken.
//!
//! Shedding fades a lateral or relay by its subtree's last living cycle T
//! against its PA's delay (`shed.rs`), and T is the latest of its own
//! axis's last unit and every borne subtree's T. So each bud's table holds,
//! for every cycle t, the chance T <= t (the extinction curve) and each
//! quantity's expectation on that event: a borne subtree enters its
//! bearer's as a factor P(T_c <= t) and, in one of them, its quantity,
//! which over a zone's random node count is the node count's generating
//! function and its derivative (`along::Count::tilted`). A bud's own axis
//! is a path of growth units that ends by failing, aborting, ending its
//! age or living on (`steps.rs`); each end weighs the units grown before.
//!
//! Approximations, each measured (`tests.rs`): presences whole, as the
//! counts have them; times whole cycles; axes straight, azimuths even;
//! every section summed at one exponent and every mass at the pipe model's
//! square; sleeping buds independent of the axis that bears them.
use super::along::Along;
use super::steps::steps;
use super::{sub, Sub};
use crate::dormant::{First, Living};
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::Species;
use std::collections::HashMap;
use std::rc::Rc;

/// The quantities, per unit base scale: wood (Σ l), its first moment along
/// the heading, its second moment about the base, its second moment along
/// the heading; Σ o² by the path to it and by the path's moment along the
/// heading (the mass the pipe model hangs on it, and its moment); Σ l o²
/// and its moment (the leaves' mass); then Σ o^e per exponent.
pub(crate) const WOOD: usize = 0;
pub(crate) const ALONG: usize = 1;
pub(crate) const SQUARE: usize = 2;
pub(crate) const PARALLEL: usize = 3;
pub(crate) const PATH: usize = 4;
pub(crate) const MOMENT: usize = 5;
pub(crate) const LEAVES: usize = 6;
pub(crate) const LEAVES_ALONG: usize = 7;
pub(crate) const SECTIONS: usize = 8;

/// A bud's kept quantities by cycle: `a[t]` the chance its subtree's last
/// living cycle is at most t, `b[t]` each quantity's expectation on it.
#[derive(Debug)]
pub(crate) struct Table {
    pub a: Vec<f64>,
    pub b: Vec<Vec<f64>>,
}

impl Table {
    fn at(&self, t: i64) -> (f64, &[f64]) {
        if t < 0 {
            return (0.0, &[]);
        }
        let t = (t as usize).min(self.a.len() - 1);
        (self.a[t], &self.b[t])
    }
}

/// How a borne bud stands: its quantity's scale moments E[s^p] (p = 1 to
/// 4), E[s^e] per exponent, E[s^p x] (p = 1 to 3) and E[s^p x²] (p = 1, 2)
/// for x its base's distance along the axis, summed over its buds; and
/// the cosine of its angle to the axis.
#[derive(Debug, Clone, Default)]
struct Placing {
    s: [f64; 4],
    e: Vec<f64>,
    x: [f64; 3],
    x2: [f64; 2],
    turn: f64,
}

impl Placing {
    /// `count` buds of scale `scale` (its moments `powers`, by power
    /// p = 1 to 4 then per exponent) standing at `at`.
    fn add(&mut self, count: f64, powers: &[f64], at: Along) {
        for (s, &w) in self.s.iter_mut().zip(powers) {
            *s += count * w;
        }
        for (e, &w) in self.e.iter_mut().zip(&powers[4..]) {
            *e += count * w;
        }
        for (x, &w) in self.x.iter_mut().zip(powers) {
            *x += count * w * at.mean;
        }
        for (x, &w) in self.x2.iter_mut().zip(powers) {
            *x += count * w * at.square;
        }
    }

    /// A borne subtree's quantities `c`, placed.
    fn place(&self, c: &[f64], out: &mut [f64], two: usize) {
        let [s1, s2, s3, s4] = self.s;
        let [x1, x2, x3] = self.x;
        let [q1, q2] = self.x2;
        let (cos, sin2) = (self.turn, 1.0 - self.turn * self.turn);
        let section = c[SECTIONS + two];
        out[WOOD] += s1 * c[WOOD];
        out[ALONG] += x1 * c[WOOD] + cos * s2 * c[ALONG];
        let square = q1 * c[WOOD] + 2.0 * cos * x2 * c[ALONG];
        out[SQUARE] += square + s3 * c[SQUARE];
        let across = c[SQUARE] - c[PARALLEL];
        out[PARALLEL] += square + s3 * (cos * cos * c[PARALLEL] + sin2 * across / 2.0);
        out[PATH] += s3 * c[PATH] + x2 * section;
        out[MOMENT] += 0.5 * q2 * section + x3 * c[PATH] + cos * s4 * c[MOMENT];
        out[LEAVES] += s3 * c[LEAVES];
        out[LEAVES_ALONG] += x3 * c[LEAVES] + cos * s4 * c[LEAVES_ALONG];
        for (k, &w) in self.e.iter().enumerate() {
            out[SECTIONS + k] += w * c[SECTIONS + k];
        }
    }
}

/// The species' kept quantities at a tree's age, worked out once per bud.
pub(crate) struct Kept<'a> {
    species: &'a Species,
    living: Living<'a>,
    laterals: Vec<Vec<Vec<f64>>>,
    /// The exponents sections are summed at, 2 first.
    exponents: Vec<f64>,
    memo: HashMap<Sub, Rc<Table>>,
    faded: HashMap<(Sub, u64), Rc<Table>>,
}

impl<'a> Kept<'a> {
    pub fn new(species: &'a Species, windows: &'a Windows) -> Self {
        let mut exponents = vec![2.0];
        for s in &species.states {
            if !exponents.contains(&s.form.exponent) {
                exponents.push(s.form.exponent);
            }
        }
        Self {
            species,
            living: Living::new(species, Some(windows)),
            laterals: species.states.iter().map(|s| s.laterals()).collect(),
            exponents,
            memo: HashMap::new(),
            faded: HashMap::new(),
        }
    }

    /// The width of a quantity vector.
    fn width(&self) -> usize {
        SECTIONS + self.exponents.len()
    }

    /// The index of exponent `e` among the sections.
    pub fn section(&self, e: f64) -> usize {
        SECTIONS + self.exponents.iter().position(|&x| x == e).unwrap_or(0)
    }

    /// Each quantity's power of the base scale.
    fn powers(&self) -> Vec<f64> {
        let mut p = vec![1.0, 2.0, 3.0, 3.0, 3.0, 4.0, 3.0, 4.0];
        p.extend(&self.exponents);
        p
    }

    /// A bud of PA `k` made `m` cycles before the tree's age, carrying
    /// `carried`, its first unit's law `first`: what it keeps by its last
    /// living cycle, its own fade not taken. A bud of an age it passes
    /// through grows at the share of its size that reaches the next.
    pub fn of(&mut self, k: usize, m: usize, carried: Carried, first: Option<First>) -> Rc<Table> {
        let Some((k, reach)) = self.species.lived(k) else {
            let width = self.width();
            return Rc::new(Table {
                a: vec![1.0; m + 1],
                b: vec![vec![0.0; width]; m + 1],
            });
        };
        let table = self.lived(k, m, carried, first);
        if reach == 1.0 {
            return table;
        }
        let powers = self.powers();
        Rc::new(Table {
            a: table.a.clone(),
            b: table
                .b
                .iter()
                .map(|b| {
                    b.iter()
                        .zip(&powers)
                        .map(|(v, p)| v * reach.powf(*p))
                        .collect()
                })
                .collect(),
        })
    }

    /// `of`, then faded as shedding fades a lateral or relay of a PA with
    /// the delay `delay` (infinite: never shed).
    pub fn shed(
        &mut self,
        (k, m, carried, first): (usize, usize, Carried, Option<First>),
        delay: f64,
    ) -> Rc<Table> {
        let table = self.of(k, m, carried, first);
        if !delay.is_finite() {
            return table;
        }
        let lived = self.species.lived(k).map_or(k, |l| l.0);
        let key = (sub(self.species, lived, m, carried, first), delay.to_bits());
        if let Some(t) = self.faded.get(&key) {
            return t.clone();
        }
        let powers = self.powers();
        let mut b = vec![vec![0.0; self.width()]; table.a.len()];
        let mut sum = vec![0.0; self.width()];
        let zero = vec![0.0; self.width()];
        for (s, row) in b.iter_mut().enumerate() {
            // Its last unit s cycles in, m - s before the tree's age.
            let fade = (delay + 1.0 - (m - s) as f64).clamp(0.0, 1.0);
            let before = if s > 0 { &table.b[s - 1] } else { &zero };
            for (q, total) in sum.iter_mut().enumerate() {
                *total += fade.powf(powers[q]) * (table.b[s][q] - before[q]);
            }
            row.copy_from_slice(&sum);
        }
        let t = Rc::new(Table {
            a: table.a.clone(),
            b,
        });
        self.faded.insert(key, t.clone());
        t
    }

    fn lived(&mut self, k: usize, m: usize, carried: Carried, first: Option<First>) -> Rc<Table> {
        let key = sub(self.species, k, m, carried, first);
        if let Some(t) = self.memo.get(&key) {
            return t.clone();
        }
        let steps = steps(self.species, &mut self.living, (k, m, carried), first);
        let axis = path::Axis::of(self.species, k);
        let mut a = Vec::with_capacity(m + 1);
        let mut b = Vec::with_capacity(m + 1);
        for t in 0..=m {
            let (at, bt) = self.threshold((k, m), &axis, &steps, t as i64);
            a.push(at);
            b.push(bt);
        }
        let table = Rc::new(Table { a, b });
        self.memo.insert(key, table.clone());
        table
    }
}

mod path;

#[cfg(test)]
mod tests;
