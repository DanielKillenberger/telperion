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
//! A bud's table is its first unit's with the rest of its axis, the same
//! PA's bud one cycle younger carrying the time spent, set at the unit's
//! tip (fn-209, P1): each table one unit's work, and buds that grow alike
//! one table (`Carried::canonical`).
//!
//! Approximations, each measured (`tests.rs`): presences whole, as the
//! counts have them; times whole cycles; axes straight, azimuths even;
//! the delta method's second order; a unit's own phytomers bearing, for
//! sag, the section beyond its middle, and the section beyond a unit
//! the rest of its axis's as one mixture over how it ends; a relay's
//! section entering at its axis's tip, a failed unit's relay at the
//! rest's base; sleeping buds independent of the axis that bears them.
use super::along::Along;
use super::steps::{first_step, rest};
use super::{sub, Sub};
use crate::dormant::{First, Living};
use crate::memo::Memo;
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::Species;
use crate::transcendental::Transcendental;
use std::rc::Rc;

mod bound;
mod ends;
mod mass;
mod path;
pub(crate) mod section;
pub(in crate::closed_form) use path::tilts;
#[cfg(test)]
mod tests;

/// The quantities, per unit base scale: wood (Σ l), its first moment along
/// the heading, its second moment about the base, its second moment along
/// the heading; the pipes' mass Σ l r² and its moment along the heading;
/// the leaves' mass Σ l o² and its moment; the base section S in the
/// bud's own exponent, and S²; and the phytomers (fn-209 F1, the cut's
/// predictor), which no base scale changes.
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
pub(crate) const COUNT: usize = 10;
pub(crate) const WIDTH: usize = 11;

/// Each quantity's power of the base scale, the section's at exponent `e`.
fn powers(e: f64) -> [f64; WIDTH] {
    [1.0, 2.0, 3.0, 3.0, 3.0, 4.0, 3.0, 4.0, e, 2.0 * e, 0.0]
}

/// A bud's kept quantities by cycle t from `lo` on: `a` the chance its
/// subtree's last living cycle is at most t, `k` the chance of that and
/// its being kept, `b` each quantity's expectation on it.
#[derive(Debug)]
pub(crate) struct Table {
    pub lo: usize,
    pub a: Vec<f64>,
    pub k: Vec<f64>,
    pub b: Vec<[f64; WIDTH]>,
    /// The exponent its section is carried in.
    pub e: f64,
    /// How far its wood can reach (`bound.rs`).
    pub reach: f64,
}

/// A table's row at a cycle: (a, k, b), none before its birth.
pub(crate) type Row<'t> = (f64, f64, Option<&'t [f64; WIDTH]>);

impl Table {
    pub fn at(&self, t: i64) -> Row<'_> {
        if t < 0 {
            return (0.0, 0.0, None);
        }
        let t = (t as usize).min(self.lo + self.a.len() - 1);
        let i = t.checked_sub(self.lo).expect("a cycle the window holds");
        (self.a[i], self.k[i], Some(&self.b[i]))
    }
}

/// How a borne bud stands: its base scale's moments E[s^p] (p = 1 to 4),
/// E[s^p x] (p = 1 to 3) and E[s^p x²] (p = 1, 2) for x its base's
/// distance along the axis, summed over its buds; the buds; and the
/// cosine of its angle to the axis and that cosine's square, each in
/// expectation over its insertion draw (`lineage::turn`).
#[derive(Debug, Clone, Default)]
pub(crate) struct Placing {
    n: f64,
    s: [f64; 4],
    x: [f64; 3],
    x2: [f64; 2],
    turn: [f64; 2],
}

impl Placing {
    pub fn turned(turn: [f64; 2]) -> Self {
        Self {
            turn,
            ..Self::default()
        }
    }

    /// `count` buds whose base scale's moments are `powers` (p = 1 to 4),
    /// standing at `at`.
    pub fn add(&mut self, count: f64, powers: &[f64; 4], at: Along) {
        self.n += count;
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
    pub fn whole(at: Along, turn: [f64; 2]) -> Self {
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
        let [cos, cos2] = self.turn;
        let sin2 = 1.0 - cos2;
        out[WOOD] += s1 * c[WOOD];
        out[ALONG] += x1 * c[WOOD] + cos * s2 * c[ALONG];
        let square = q1 * c[WOOD] + 2.0 * cos * x2 * c[ALONG];
        out[SQUARE] += square + s3 * c[SQUARE];
        let across = c[SQUARE] - c[PARALLEL];
        out[PARALLEL] += square + s3 * (cos2 * c[PARALLEL] + sin2 * across / 2.0);
        out[MASS] += s3 * c[MASS];
        out[MASS_ALONG] += x3 * c[MASS] + cos * s4 * c[MASS_ALONG];
        out[LEAVES] += s3 * c[LEAVES];
        out[LEAVES_ALONG] += x3 * c[LEAVES] + cos * s4 * c[LEAVES_ALONG];
        out[COUNT] += self.n * c[COUNT];
    }
}

/// The species' kept quantities at a tree's age, worked out once per bud.
pub(crate) struct Kept<'a> {
    species: &'a Species,
    living: Living<'a>,
    laterals: Rc<[Vec<Vec<f64>>]>,
    axes: Rc<[path::Axis]>,
    memo: Memo<Sub, Rc<Table>>,
    faded: Memo<(Sub, u64), Rc<Table>>,
    /// The cycles before a bud's last that its table holds (`window`).
    lags: usize,
    tape: Tape,
}

/// The tables one cycle of a bud's window reads, in order. Every cycle
/// reads the same ones, so the first records them and the rest replay
/// them, one lookup a table for the whole window (fn-209, P1).
#[derive(Default)]
enum Tape {
    #[default]
    Off,
    Record(Vec<Rc<Table>>),
    Replay(Vec<Rc<Table>>, usize),
}

/// The cycles before its last that every table needs: a row reads a bud
/// at its last cycle, a bearer reads what it bears as many cycles before
/// their last as it is before its own, and a fade of delay d reads the
/// cycles while it is under way and the one before them.
pub(in crate::closed_form) fn window(species: &Species) -> usize {
    let delays = species.states.iter().map(|s| s.shedding);
    let finite = delays.filter(|d| d.is_finite());
    finite.map(|d| (d + 1.0).ceil() as usize).max().unwrap_or(0)
}

impl<'a> Kept<'a> {
    pub fn new(species: &'a Species, windows: &'a Windows) -> Self {
        Self {
            species,
            living: Living::new(species, Some(windows)),
            laterals: species.states.iter().map(|s| s.laterals()).collect(),
            axes: (0..species.states.len())
                .map(|k| path::Axis::of(species, k))
                .collect(),
            memo: Memo::default(),
            faded: Memo::default(),
            lags: window(species),
            tape: Tape::Off,
        }
    }

    /// Tables over every cycle, for the tests that read a whole curve.
    #[cfg(test)]
    pub fn every_cycle(species: &'a Species, windows: &'a Windows) -> Self {
        Self {
            lags: usize::MAX,
            ..Self::new(species, windows)
        }
    }

    /// A bud of PA `k` made `m` cycles before the tree's age, carrying
    /// `carried`, its first unit's law `first`: what it keeps by its last
    /// living cycle, its own fade not taken. A bud of an age it passes
    /// through grows at the share of its size that reaches the next.
    pub fn of(&mut self, k: usize, m: usize, carried: Carried, first: Option<First>) -> Rc<Table> {
        let Some((k, reach)) = self.species.lived(k) else {
            return Rc::new(Table {
                lo: 0,
                a: vec![1.0; m + 1],
                k: vec![1.0; m + 1],
                b: vec![[0.0; WIDTH]; m + 1],
                e: 2.0,
                reach: 0.0,
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
            .map(|b| std::array::from_fn(|q| b[q] * reach.powf_fixed(p[q])));
        Rc::new(Table {
            lo: table.lo,
            a: table.a.clone(),
            k: table.k.clone(),
            b: b.collect(),
            e: table.e,
            reach: reach * table.reach,
        })
    }

    /// `shed`, read once for a bud's whole window (`Tape`).
    pub(super) fn child(
        &mut self,
        bud: (usize, usize, Carried, Option<First>),
        delay: f64,
    ) -> Rc<Table> {
        if let Tape::Replay(read, i) = &mut self.tape {
            *i += 1;
            return read[*i - 1].clone();
        }
        let table = self.shed(bud, delay);
        if let Tape::Record(read) = &mut self.tape {
            read.push(table.clone());
        }
        table
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
        for (i, s) in (table.lo..=m).enumerate() {
            // Its last unit s cycles in, m - s before the tree's age; a
            // cycle it has faded by adds nothing.
            let fade = (delay + 1.0 - (m - s) as f64).clamp(0.0, 1.0);
            if fade > 0.0 {
                let (a0, b0) = match s {
                    0 => (0.0, [0.0; WIDTH]),
                    _ => (table.a[i - 1], table.b[i - 1]),
                };
                // Its count in expectation over the fade, as the executors
                // fade it by degree (fn-231); its quantities at their powers.
                for q in 0..WIDTH {
                    let f = if q == COUNT { fade } else { fade.powf_fixed(p[q]) };
                    sum[q] += f * (table.b[i][q] - b0[q]);
                }
                k_sum += table.a[i] - a0;
            }
            b.push(sum);
            kept.push(k_sum);
        }
        let t = Rc::new(Table {
            lo: table.lo,
            a: table.a.clone(),
            k: kept,
            b,
            e: table.e,
            reach: table.reach,
        });
        self.faded.insert(key, t.clone());
        t
    }

    fn lived(&mut self, k: usize, m: usize, carried: Carried, first: Option<First>) -> Rc<Table> {
        let state = &self.species.states[k];
        let carried = carried.canonical(state.life(), state.rises(), m);
        let key = sub(self.species, k, m, carried, first);
        if let Some(t) = self.memo.get(&key) {
            return t.clone();
        }
        let step = first_step(self.species, &mut self.living, (k, m, carried), first);
        // The rest of its axis: the bud one cycle younger (fn-209, P1).
        let rest = match &step {
            Some(s) if s.end.is_none() && s.after > 0.0 && m > 1 => {
                let lifespan = self.species.states[k].life();
                Some(self.lived(k, m - 1, rest(carried, lifespan), None))
            }
            _ => None,
        };
        let reach = self.bound((k, m), step.as_ref(), rest.as_deref());
        let steps: Vec<_> = step.into_iter().collect();
        let axes = self.axes.clone();
        let axis = &axes[k];
        let lo = m.saturating_sub(self.lags);
        let (mut a, mut b) = (
            Vec::with_capacity(m + 1 - lo),
            Vec::with_capacity(m + 1 - lo),
        );
        let outer = std::mem::take(&mut self.tape);
        for t in lo..=m {
            self.tape = match std::mem::take(&mut self.tape) {
                Tape::Record(read) | Tape::Replay(read, _) => Tape::Replay(read, 0),
                Tape::Off => Tape::Record(Vec::new()),
            };
            let (at, bt) = self.threshold((k, m), axis, (&steps, rest.as_deref()), t as i64);
            a.push(at);
            b.push(bt);
        }
        self.tape = outer;
        let e = self.species.states[k].form.exponent;
        let table = Rc::new(Table {
            lo,
            k: a.clone(),
            a,
            b,
            e,
            reach,
        });
        self.memo.insert(key, table.clone());
        table
    }
}
