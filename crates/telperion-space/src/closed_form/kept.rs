//! What a bud's subtree is expected to keep at the tree's age (fn-209,
//! host decisions 9, 15 and 16): its wood and that wood's moments, its
//! base section's mean and mean square, its mass for sag and the mass's
//! moment, with shedding taken; and how far it can reach.
//!
//! Shedding fades a lateral or relay by its subtree's last living cycle T
//! against its PA's delay (`shed.rs`), and T is the latest of its own
//! axis's last unit and every borne subtree's T. So each bud's table holds,
//! for every cycle t, the chance T <= t (the extinction curve), the chance
//! it is kept and T <= t, and each quantity's expectation on that event: a
//! borne subtree enters its bearer's as a factor P(T_c <= t) and, in one
//! of them, its quantity, which over a zone's random node count is the
//! count's generating function and its derivatives (`kept/path.rs`).
//!
//! A section is carried in its own PA's exponent and handed to a bearer of
//! another through E[S^(e_p / e_c)], taken by the delta method from its
//! mean and mean square among the subtrees kept (`kept/section.rs`); the
//! mass's radius likewise, at the axis's own exponent (host decision 15).
//!
//! Approximations, each measured (`tests.rs`): presences whole, as the
//! counts have them; times whole cycles; axes straight, azimuths even;
//! the delta method's second order; a unit's own phytomers bearing, for
//! sag, the section beyond its middle; a relay's section entering at its
//! axis's tip; sleeping buds independent of the axis that bears them.
use super::along::Along;
use super::steps::steps;
use super::{sub, Sub};
use crate::dormant::{First, Living};
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::Species;
use std::collections::HashMap;
use std::rc::Rc;

mod bound;
mod ends;
mod mass;
mod path;
mod section;
#[cfg(test)]
mod tests;

/// The quantities, per unit base scale: wood (Σ l), its first moment along
/// the heading, its second moment about the base, its second moment along
/// the heading; the pipes' mass Σ l r² and its moment along the heading;
/// the leaves' mass Σ l o² and its moment; the base section S in the
/// bud's own exponent, and S².
pub(crate) const WOOD: usize = 0;
pub(crate) const ALONG: usize = 1;
pub(crate) const SQUARE: usize = 2;
pub(crate) const PARALLEL: usize = 3;
pub(crate) const MASS: usize = 4;
pub(crate) const MASS_ALONG: usize = 5;
pub(crate) const LEAVES: usize = 6;
pub(crate) const LEAVES_ALONG: usize = 7;
pub(crate) const SECTION: usize = 8;
pub(crate) const SECTION2: usize = 9;
pub(crate) const WIDTH: usize = 10;

/// Each quantity's power of the base scale, the section's at exponent `e`.
fn powers(e: f64) -> [f64; WIDTH] {
    [1.0, 2.0, 3.0, 3.0, 3.0, 4.0, 3.0, 4.0, e, 2.0 * e]
}

/// A bud's kept quantities by cycle: `a[t]` the chance its subtree's last
/// living cycle is at most t, `k[t]` the chance of that and its being
/// kept, `b[t]` each quantity's expectation on it.
#[derive(Debug)]
pub(crate) struct Table {
    pub a: Vec<f64>,
    pub k: Vec<f64>,
    pub b: Vec<[f64; WIDTH]>,
    /// The exponent its section is carried in.
    pub e: f64,
}

/// A table's row at a cycle: (a, k, b), none before its birth.
pub(crate) type Row<'t> = (f64, f64, Option<&'t [f64; WIDTH]>);

impl Table {
    pub fn at(&self, t: i64) -> Row<'_> {
        if t < 0 {
            return (0.0, 0.0, None);
        }
        let t = (t as usize).min(self.a.len() - 1);
        (self.a[t], self.k[t], Some(&self.b[t]))
    }
}

/// How a borne bud stands: its base scale's moments E[s^p] (p = 1 to 4),
/// E[s^p x] (p = 1 to 3) and E[s^p x²] (p = 1, 2) for x its base's
/// distance along the axis, summed over its buds; and the cosine of its
/// angle to the axis.
#[derive(Debug, Clone, Default)]
pub(crate) struct Placing {
    s: [f64; 4],
    x: [f64; 3],
    x2: [f64; 2],
    turn: f64,
}

impl Placing {
    pub fn turned(turn: f64) -> Self {
        Self {
            turn,
            ..Self::default()
        }
    }

    /// `count` buds whose base scale's moments are `powers` (p = 1 to 4),
    /// standing at `at`.
    pub fn add(&mut self, count: f64, powers: &[f64; 4], at: Along) {
        for (s, &w) in self.s.iter_mut().zip(powers) {
            *s += count * w;
        }
        for (x, &w) in self.x.iter_mut().zip(powers) {
            *x += count * w * at.mean;
        }
        for (x, &w) in self.x2.iter_mut().zip(powers) {
            *x += count * w * at.square;
        }
    }

    /// One whole bud of its bearer's base scale at `at`.
    pub fn whole(at: Along, turn: f64) -> Self {
        let mut p = Self::turned(turn);
        p.add(1.0, &[1.0; 4], at);
        p
    }

    /// A borne subtree's quantities `c`, placed; its section is handed
    /// over apart (`section.rs`).
    pub fn place(&self, c: &[f64; WIDTH], out: &mut [f64; WIDTH]) {
        let [s1, s2, s3, s4] = self.s;
        let [x1, x2, x3] = self.x;
        let q1 = self.x2[0];
        let (cos, sin2) = (self.turn, 1.0 - self.turn * self.turn);
        out[WOOD] += s1 * c[WOOD];
        out[ALONG] += x1 * c[WOOD] + cos * s2 * c[ALONG];
        let square = q1 * c[WOOD] + 2.0 * cos * x2 * c[ALONG];
        out[SQUARE] += square + s3 * c[SQUARE];
        let across = c[SQUARE] - c[PARALLEL];
        out[PARALLEL] += square + s3 * (cos * cos * c[PARALLEL] + sin2 * across / 2.0);
        out[MASS] += s3 * c[MASS];
        out[MASS_ALONG] += x3 * c[MASS] + cos * s4 * c[MASS_ALONG];
        out[LEAVES] += s3 * c[LEAVES];
        out[LEAVES_ALONG] += x3 * c[LEAVES] + cos * s4 * c[LEAVES_ALONG];
    }
}

/// The species' kept quantities at a tree's age, worked out once per bud.
pub(crate) struct Kept<'a> {
    species: &'a Species,
    living: Living<'a>,
    laterals: Vec<Vec<Vec<f64>>>,
    memo: HashMap<Sub, Rc<Table>>,
    faded: HashMap<(Sub, u64), Rc<Table>>,
    bounds: HashMap<Sub, f64>,
}

impl<'a> Kept<'a> {
    pub fn new(species: &'a Species, windows: &'a Windows) -> Self {
        Self {
            species,
            living: Living::new(species, Some(windows)),
            laterals: species.states.iter().map(|s| s.laterals()).collect(),
            memo: HashMap::new(),
            faded: HashMap::new(),
            bounds: HashMap::new(),
        }
    }

    /// A bud of PA `k` made `m` cycles before the tree's age, carrying
    /// `carried`, its first unit's law `first`: what it keeps by its last
    /// living cycle, its own fade not taken. A bud of an age it passes
    /// through grows at the share of its size that reaches the next.
    pub fn of(&mut self, k: usize, m: usize, carried: Carried, first: Option<First>) -> Rc<Table> {
        let Some((k, reach)) = self.species.lived(k) else {
            return Rc::new(Table {
                a: vec![1.0; m + 1],
                k: vec![1.0; m + 1],
                b: vec![[0.0; WIDTH]; m + 1],
                e: 2.0,
            });
        };
        let table = self.lived(k, m, carried, first);
        if reach == 1.0 {
            return table;
        }
        let p = powers(table.e);
        let b = table
            .b
            .iter()
            .map(|b| std::array::from_fn(|q| b[q] * reach.powf(p[q])));
        Rc::new(Table {
            a: table.a.clone(),
            k: table.k.clone(),
            b: b.collect(),
            e: table.e,
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
        let p = powers(table.e);
        let (mut b, mut kept) = (Vec::with_capacity(m + 1), Vec::with_capacity(m + 1));
        let (mut sum, mut k_sum) = ([0.0; WIDTH], 0.0);
        for s in 0..table.a.len() {
            // Its last unit s cycles in, m - s before the tree's age.
            let fade = (delay + 1.0 - (m - s) as f64).clamp(0.0, 1.0);
            let (a0, b0) = if s > 0 {
                (table.a[s - 1], table.b[s - 1])
            } else {
                (0.0, [0.0; WIDTH])
            };
            for q in 0..WIDTH {
                sum[q] += fade.powf(p[q]) * (table.b[s][q] - b0[q]);
            }
            if fade > 0.0 {
                k_sum += table.a[s] - a0;
            }
            b.push(sum);
            kept.push(k_sum);
        }
        let t = Rc::new(Table {
            a: table.a.clone(),
            k: kept,
            b,
            e: table.e,
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
        let (mut a, mut b) = (Vec::with_capacity(m + 1), Vec::with_capacity(m + 1));
        for t in 0..=m {
            let (at, bt) = self.threshold((k, m), &axis, &steps, t as i64);
            a.push(at);
            b.push(bt);
        }
        let e = self.species.states[k].form.exponent;
        let table = Rc::new(Table {
            k: a.clone(),
            a,
            b,
            e,
        });
        self.memo.insert(key, table.clone());
        table
    }
}
