//! A bud's committed boundary per unit base scale (fn-209, option 4): the
//! kept tables read once for every PA and every age a bud can have at the
//! tree's age, so a bud's boundary is a row times powers of its base
//! scale, the same at any zoom and in any order of refinement. Test-only:
//! the rows the predictor's counts are held to (`predictor/tests.rs`).
use super::kept::{Kept, Table, COUNT, LEAVES, LEAVES_ALONG, MASS, MASS_ALONG, SECTION, SECTION2};
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::Species;
use std::sync::{Arc, Mutex, PoisonError};

#[cfg(test)]
mod tests;

/// What a bud of one PA made `m` cycles before the tree's age keeps, per
/// unit base scale: the chance it is kept; its base section's partial
/// moments on that (E[S; kept], E[S²; kept]) in its own exponent `e`;
/// the same before its own fade, which its bearer retains a share of;
/// its mass and that mass's moment along its heading, on being kept; its
/// phytomers kept (E[N; kept], the cut's predictor, fn-209 F1); and its
/// extent bound.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Row {
    pub kept: f64,
    pub s1: f64,
    pub s2: f64,
    pub grown1: f64,
    pub grown2: f64,
    pub mass: f64,
    pub moment: f64,
    pub count: f64,
    pub reach: f64,
    pub e: f64,
}

/// One core's rows, by PA and age.
type Read = Vec<((usize, usize), Row)>;

/// Every row a bud of the species can need at one age.
pub(crate) struct Rows {
    age: usize,
    rows: Vec<Row>,
    seed: Row,
}

impl Rows {
    /// The rows, read from the kept tables: every PA a bud can grow in,
    /// every age 0 to the tree's, on up to `threads` cores. Each core
    /// reads its own share of the rows, interleaved so the costly old
    /// ones are spread, with tables of its own: a row is a value of its
    /// PA and age alone, whatever core read it. Each share runs on a
    /// thread with a stack as deep as the age needs (`cores::deep`). The
    /// first core reads the seed's too, from the tables its rows left.
    pub fn new(species: &Species, age: u32, threads: usize) -> Self {
        Self::read(species, age, threads, true)
    }

    /// The laterals' rows alone, the seed's left at its default: a third
    /// less work at the spruce's 40.
    pub fn laterals(species: &Species, age: u32, threads: usize) -> Self {
        Self::read(species, age, threads, false)
    }

    fn read(species: &Species, age: u32, threads: usize, whole: bool) -> Self {
        let m_max = age as usize;
        let borne = borne(species);
        let mut jobs: Vec<(usize, usize)> = Vec::new();
        for (pa, _) in borne.iter().enumerate().filter(|(_, b)| **b) {
            jobs.extend((0..=m_max).rev().map(|m| (pa, m)));
        }
        jobs.sort_by_key(|&(_, m)| std::cmp::Reverse(m));
        let shares = threads.clamp(1, jobs.len().max(1));
        let done: Vec<Mutex<Read>> = (0..shares).map(|_| Mutex::new(Vec::new())).collect();
        let seed = Mutex::new(Row::default());
        crate::cores::deep(shares, &|k| {
            let windows = Windows::new(species, age);
            let mut kept = Kept::new(species, &windows);
            let mine = jobs.iter().skip(k).step_by(shares);
            *done[k].lock().unwrap() = mine
                .map(|&(pa, m)| ((pa, m), read(&mut kept, species, pa, m)))
                .collect();
            if k == 0 && whole {
                let whole = kept.of(0, m_max, Carried::default(), None);
                let reach = kept.reach_bound(0, m_max, Carried::default(), None);
                *seed.lock().unwrap() = row(&whole, &whole, m_max, reach);
            }
        });
        let mut rows = vec![Row::default(); species.states.len() * (m_max + 1)];
        for ((pa, m), row) in done.into_iter().flat_map(|d| d.into_inner().unwrap()) {
            rows[pa * (m_max + 1) + m] = row;
        }
        Self {
            age: m_max,
            rows,
            seed: seed.into_inner().unwrap(),
        }
    }

    /// The row of a lateral of `pa` `m` cycles from the tree's age, none
    /// where no bud of the PA is borne (the predictor's proof).
    pub(crate) fn row_at(&self, pa: usize, m: usize) -> Option<&Row> {
        self.rows.get(pa * (self.age + 1) + m).filter(|r| r.e > 0.0)
    }

    /// The seed: the whole tree.
    pub fn seed(&self) -> &Row {
        &self.seed
    }
}

/// What keys a shelf's rows: the species as its `Debug` text, which prints
/// every parameter and each f64 to the bit, its age, and the seed's mode.
type Key = (String, u32, bool);

/// The `room` most recently used rows, the most recent last.
pub(super) struct Shelf {
    held: Mutex<Vec<(Key, Arc<Rows>)>>,
    room: usize,
}

impl Shelf {
    pub(super) const fn new(room: usize) -> Self {
        Self {
            held: Mutex::new(Vec::new()),
            room,
        }
    }

    /// The rows `Rows::new` (`whole`) or `Rows::laterals` reads, read once.
    /// The lock is not held while they are read: two trees reading the
    /// same rows at once read them twice and keep one.
    pub(super) fn rows(
        &self,
        species: &Species,
        age: u32,
        threads: usize,
        whole: bool,
    ) -> Arc<Rows> {
        let key = (format!("{species:?}"), age, whole);
        if let Some(rows) = self.take(&key) {
            return rows;
        }
        let rows = Arc::new(Rows::read(species, age, threads, whole));
        let mut held = self.held.lock().unwrap_or_else(PoisonError::into_inner);
        held.retain(|(k, _)| *k != key);
        if held.len() == self.room {
            held.remove(0);
        }
        held.push((key, rows.clone()));
        rows
    }

    /// The rows held at `key`, made the most recently used.
    fn take(&self, key: &Key) -> Option<Arc<Rows>> {
        let mut held = self.held.lock().unwrap_or_else(PoisonError::into_inner);
        let at = held.iter().position(|(k, _)| k == key)?;
        let entry = held.remove(at);
        let rows = entry.1.clone();
        held.push(entry);
        Some(rows)
    }
}

/// The PAs a lateral or sleeping bud can grow in: every one a living
/// age's zone draws, as far as it passes through ages that do not live.
pub(super) fn borne(species: &Species) -> Vec<bool> {
    let mut borne = vec![false; species.states.len()];
    for state in species.states.iter().filter(|s| s.lifespan > 0.0) {
        for zone in &state.zones {
            let drawn = zone.lateral.iter().zip(&zone.dormant).enumerate();
            for (j, _) in drawn.filter(|(_, (l, d))| **l > 0.0 || **d > 0.0) {
                if let Some((pa, _)) = species.lived(j) {
                    borne[pa] = true;
                }
            }
        }
    }
    borne
}

/// A lateral of `pa` made `m` cycles before the tree's age.
fn read(kept: &mut Kept, species: &Species, pa: usize, m: usize) -> Row {
    let delay = species.states[pa].shedding;
    let grown = kept.of(pa, m, Carried::default(), None);
    let faded = kept.shed((pa, m, Carried::default(), None), delay);
    let reach = kept.reach_bound(pa, m, Carried::default(), None);
    row(&faded, &grown, m, reach)
}

fn row(faded: &Table, grown: &Table, m: usize, reach: f64) -> Row {
    let (_, kept, b) = faded.at(m as i64);
    let (_, _, g) = grown.at(m as i64);
    let (b, g) = (b.expect("born"), g.expect("born"));
    Row {
        kept,
        s1: b[SECTION],
        s2: b[SECTION2],
        grown1: g[SECTION],
        grown2: g[SECTION2],
        mass: b[MASS] + b[LEAVES],
        moment: b[MASS_ALONG] + b[LEAVES_ALONG],
        count: b[COUNT],
        reach,
        e: faded.e,
    }
}
