//! The cut's predictor (fn-209 F1, host decision 9): a bud's expected kept
//! phytomers, E[N; kept], and nothing else the kept tables hold.
//!
//! It is the kept recursion (`kept.rs`) cut down to the two columns the
//! count reads: the chance a subtree's last living cycle is at most t,
//! and the expected phytomers on that event. Every operation on those two
//! columns is the kept tables' own, in their order, so each count equals
//! the closed form's row `count` to the bit (`tests.rs`); the sections,
//! masses, moments and reach bounds no cut reads are left out.
//!
//! The recursion is taken with a stack of its own rather than the
//! thread's: a bud's table waits on the tables of the buds it bears and
//! of the rest of its axis, each younger, so the work goes as deep as
//! the tree's age on the heap and never on the call stack.
use super::along::Count;
use super::events::Event;
use super::kept::window;
use super::steps::{first_step, rest, Relay, Step};
use super::{sub, Sub};
use crate::dormant::{First, Living};
use crate::memo::Memo;
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::{Species, ALONG};
use std::rc::Rc;

#[cfg(test)]
mod tests;

/// Every bud's expected kept phytomers at one age, by PA and the cycles
/// from its birth to the tree's age: what the cut reads. A PA no zone
/// draws has no bud, and so no count.
pub(crate) struct Predictor {
    age: usize,
    counts: Vec<Option<f64>>,
}

impl Predictor {
    /// The counts of `species` grown to `age` under `windows`, the tree's,
    /// for every PA a bud can grow in and every age 0 to the tree's.
    pub(crate) fn new(species: &Species, windows: &Windows, age: u32) -> Self {
        let m_max = age as usize;
        let borne = super::boundary::borne(species);
        let mut counts = vec![None; species.states.len() * (m_max + 1)];
        let mut walk = Walk::new(species, windows);
        for m in 0..=m_max {
            for pa in (0..borne.len()).filter(|&pa| borne[pa]) {
                let delay = species.states[pa].shedding;
                let table = walk.child((pa, m, Carried::default(), None), delay);
                counts[pa * (m_max + 1) + m] = Some(table.at(m as i64).1.expect("born"));
            }
        }
        Self { age: m_max, counts }
    }

    /// A lateral of `pa` made in cycle `birth`: its expected kept
    /// phytomers, none for a PA no zone draws.
    pub(crate) fn count(&self, pa: usize, birth: u32) -> Option<f64> {
        let m = self.age.saturating_sub(birth as usize);
        self.counts[pa * (self.age + 1) + m]
    }
}

/// The whole tree's expected kept phytomers at `age`: the seed's table
/// at the tree's age, which no bearer fades (fn-198 host decision 15:
/// the browser's ceiling read before the tree grows).
pub(crate) fn tree(species: &Species, windows: &Windows, age: u32) -> f64 {
    let m = age as usize;
    let mut walk = Walk::new(species, windows);
    let seed = walk.child((0, m, Carried::default(), None), f64::INFINITY);
    seed.at(m as i64).1.expect("born")
}

/// A bud's table by cycle t from `lo` on: `a` the chance its subtree's
/// last living cycle is at most t, `n` its expected phytomers on that.
struct Table {
    lo: usize,
    a: Vec<f64>,
    n: Vec<f64>,
}

impl Table {
    /// (a, n) at cycle `t`, none before its birth.
    fn at(&self, t: i64) -> (f64, Option<f64>) {
        if t < 0 {
            return (0.0, None);
        }
        let t = (t as usize).min(self.lo + self.a.len() - 1);
        let i = t.checked_sub(self.lo).expect("a cycle the window holds");
        (self.a[i], Some(self.n[i]))
    }
}

/// A bud: its PA, the cycles it grows, what it carries, its first unit's
/// law.
type Bud = (usize, usize, Carried, Option<First>);

/// The tables a bud's own axis reads, each a borne bud's, faded.
#[derive(Default)]
struct Parts {
    rest: Option<Rc<Table>>,
    /// Each lived part's relay (fn-199 R1).
    lived: Vec<Option<Rc<Table>>>,
    zones: Vec<ZoneParts>,
    aborted: Option<Rc<Table>>,
    relay: Option<Rc<Table>>,
    onward: Option<Rc<Table>>,
}

/// A zone's laterals (PA, probability, table) and its woken buds' tables.
struct ZoneParts {
    z: usize,
    laterals: Vec<(usize, f64, Rc<Table>)>,
    woken: Vec<Option<Rc<Table>>>,
}

/// The recursion's memo and what every table reads.
struct Walk<'a> {
    species: &'a Species,
    living: Living<'a>,
    laterals: Vec<Vec<Vec<f64>>>,
    pmfs: Vec<Vec<Vec<f64>>>,
    lived: Memo<Sub, Rc<Table>>,
    faded: Memo<(Sub, u64), Rc<Table>>,
    lags: usize,
}

impl<'a> Walk<'a> {
    fn new(species: &'a Species, windows: &'a Windows) -> Self {
        Self {
            species,
            living: Living::new(species, Some(windows)),
            laterals: species.states.iter().map(|s| s.laterals()).collect(),
            pmfs: species
                .states
                .iter()
                .map(|s| s.zones.iter().map(|z| Count::pmf(z.nodes)).collect())
                .collect(),
            lived: Memo::default(),
            faded: Memo::default(),
            lags: window(species),
        }
    }

    /// A borne bud's table, faded as shedding fades a bud of a PA with
    /// the delay `delay`, its own table and every one it waits on built.
    fn child(&mut self, bud: Bud, delay: f64) -> Rc<Table> {
        if let Some(lived) = self.lived_bud(bud) {
            self.build(lived);
        }
        self.ready(bud, delay).expect("built")
    }

    /// The bud of the age a bud grows in, carrying what changes its
    /// growth (`Carried::canonical`); none where it grows in none.
    fn lived_bud(&self, (k, m, carried, first): Bud) -> Option<Bud> {
        let (k, _) = self.species.lived(k)?;
        let state = &self.species.states[k];
        Some((
            k,
            m,
            carried.canonical(state.life(), state.rises(), m),
            first,
        ))
    }

    fn key(&self, (k, m, carried, first): Bud) -> Sub {
        sub(self.species, k, m, carried, first)
    }

    /// `child`'s table where the lived table it reads is built; else the
    /// bud whose table is missing.
    fn ready(&mut self, bud: Bud, delay: f64) -> Result<Rc<Table>, Bud> {
        let (_, m, carried, first) = bud;
        let Some(lived) = self.lived_bud(bud) else {
            // An age it only passes through: nothing grows, nothing lives.
            return Ok(Rc::new(Table {
                lo: 0,
                a: vec![1.0; m + 1],
                n: vec![0.0; m + 1],
            }));
        };
        let table = self.lived.get(&self.key(lived)).ok_or(lived)?.clone();
        if !delay.is_finite() {
            return Ok(table);
        }
        let key = (
            sub(self.species, lived.0, m, carried, first),
            delay.to_bits(),
        );
        if let Some(t) = self.faded.get(&key) {
            return Ok(t.clone());
        }
        let faded = Rc::new(fade(&table, m, delay));
        self.faded.insert(key, faded.clone());
        Ok(faded)
    }

    /// Builds the table of the lived bud `bud` and every table it waits
    /// on, the youngest first, on a stack of its own.
    fn build(&mut self, bud: Bud) {
        let mut stack = vec![bud];
        let mut steps: Memo<Sub, Option<Step>> = Memo::default();
        while let Some(&top) = stack.last() {
            let key = self.key(top);
            if self.lived.contains_key(&key) {
                stack.pop();
                continue;
            }
            let (k, m, carried, first) = top;
            let step = match steps.remove(&key) {
                Some(step) => step,
                None => first_step(self.species, &mut self.living, (k, m, carried), first),
            };
            match self.parts(top, step.as_ref()) {
                Ok(parts) => {
                    let table = self.table(top, step.as_ref(), &parts);
                    self.lived.insert(key, Rc::new(table));
                    stack.pop();
                }
                Err(missing) => {
                    steps.insert(key, step);
                    stack.extend(missing);
                }
            }
        }
    }

    /// The tables `bud`'s first unit and the rest of its axis read, as
    /// `kept.rs` reads them, or the buds whose tables are missing.
    fn parts(&mut self, (k, m, carried, _): Bud, step: Option<&Step>) -> Result<Parts, Vec<Bud>> {
        let mut missing = Vec::new();
        let mut get = |walk: &mut Self, bud: Bud, delay: f64| match walk.ready(bud, delay) {
            Ok(t) => Some(t),
            Err(b) => {
                missing.push(b);
                None
            }
        };
        let Some(step) = step else {
            return Ok(Parts::default());
        };
        let species = self.species;
        let state = &species.states[k];
        let shedding = state.shedding;
        let delay_of = |j: usize| {
            species
                .lived(j)
                .map_or(f64::INFINITY, |(q, _)| species.states[q].shedding)
        };
        let mut parts = Parts::default();
        if step.end.is_none() && step.after > 0.0 && m > 1 {
            let bud = (k, m - 1, rest(carried, state.life()), None);
            parts.rest = get(self, bud, f64::INFINITY);
        }
        let relay = |r: Option<Relay>, p: f64| r.filter(|_| p > 0.0);
        for l in &step.lived {
            let r = l.relay;
            parts
                .lived
                .push(get(self, (k, m - r.at, r.carried, None), shedding));
        }
        for &z in ALONG.iter().filter(|&&z| z < state.zones.len()) {
            let mut laterals = Vec::new();
            for (j, &p) in self.laterals[k][z].clone().iter().enumerate() {
                if p > 0.0 {
                    let bud = (j, m - step.i, Carried::default(), None);
                    if let Some(t) = get(self, bud, delay_of(j)) {
                        laterals.push((j, p, t));
                    }
                }
            }
            // A failed unit's lived parts wake the same buds as the unit
            // (`steps.rs`), so one list of tables serves them all.
            let (sleeping, grown) = match step.lived.first() {
                Some(l) if step.grown <= 0.0 => (&l.sleeping[z], l.weight),
                _ => (&step.sleeping[z], step.grown),
            };
            let mut woken = Vec::new();
            for event in sleeping {
                woken.push(match event {
                    Event::Bud {
                        at,
                        pa,
                        carried,
                        first,
                        ..
                    } if grown > 0.0 => get(self, (*pa, m - at, *carried, *first), delay_of(*pa)),
                    _ => None,
                });
            }
            parts.zones.push(ZoneParts { z, laterals, woken });
        }
        match &step.end {
            None => {
                if let Some(r) = relay(step.aborted, step.grown - step.after) {
                    parts.aborted = get(self, (k, m - r.at, r.carried, None), shedding);
                }
            }
            Some(stop) if step.grown > 0.0 => {
                if let Some(r) = stop.relay {
                    parts.relay = get(self, (k, m - r.at, r.carried, None), shedding);
                }
                if let Some((born, pa, onward, _)) = stop.continuation {
                    parts.onward = get(self, (pa, m - born, onward, None), f64::INFINITY);
                }
            }
            Some(_) => {}
        }
        if missing.is_empty() {
            Ok(parts)
        } else {
            Err(missing)
        }
    }
}

/// `table`, of a bud `m` cycles before the tree's age, faded by a delay
/// `delay` as `Kept::shed` fades it: the count by the fade (fn-231).
fn fade(table: &Table, m: usize, delay: f64) -> Table {
    let mut n = Vec::with_capacity(m + 1);
    let mut sum = 0.0;
    for (i, s) in (table.lo..=m).enumerate() {
        let fade = (delay + 1.0 - (m - s) as f64).clamp(0.0, 1.0);
        if fade > 0.0 {
            let n0 = if s == 0 { 0.0 } else { table.n[i - 1] };
            sum += fade * (table.n[i] - n0);
        }
        n.push(sum);
    }
    Table {
        lo: table.lo,
        a: table.a.clone(),
        n,
    }
}

/// A part's factor and its expected phytomers on it.
type Ended = (f64, f64);

/// No end: nothing left.
const NONE: Ended = (1.0, 0.0);

/// Another independent factor `g` with its phytomers `h` (E[g · N]).
fn times((a, n): Ended, (g, h): Ended) -> Ended {
    (a * g, n * g + a * h)
}

/// Two independent ends at once.
fn both((a1, n1): Ended, (a2, n2): Ended) -> Ended {
    (a1 * a2, a2 * n1 + a1 * n2)
}

/// `share` of an end, and the rest none.
fn maybe(share: f64, (a, n): Ended) -> Ended {
    (1.0 - share + share * a, share * n)
}

/// A whole child at cycle `t`.
fn whole(table: &Table, t: i64) -> Ended {
    let (a, n) = table.at(t);
    (a, n.unwrap_or(0.0))
}

mod axis;
