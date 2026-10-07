//! A bud's committed boundary per unit base scale (fn-209, option 4): the
//! kept tables read once for every PA and every age a bud can have at the
//! tree's age, so a bud's boundary is a row times powers of its base
//! scale, the same at any zoom and in any order of refinement.
use super::kept::{Kept, Table, COUNT, LEAVES, LEAVES_ALONG, MASS, MASS_ALONG, SECTION, SECTION2};
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::Species;
use std::sync::Mutex;

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

/// One core's rows, by PA and age, each with its extinction curve.
type Read = Vec<((usize, usize), (Row, Vec<f64>))>;

/// Every row a bud of the species can need at one age.
pub(crate) struct Rows {
    age: usize,
    rows: Vec<Row>,
    /// Each row's extinction curve: the chance its subtree's last living
    /// cycle is at most its birth plus t, for t = 0 to its age (fn-209
    /// probe P2: what a bearer's shedding reads of the buds it bears).
    curves: Vec<Vec<f64>>,
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
            if k == 0 {
                let whole = kept.of(0, m_max, Carried::default(), None);
                let reach = kept.reach_bound(0, m_max, Carried::default(), None);
                *seed.lock().unwrap() = row(&whole, &whole, m_max, reach);
            }
        });
        let mut rows = vec![Row::default(); species.states.len() * (m_max + 1)];
        let mut curves = vec![Vec::new(); rows.len()];
        for ((pa, m), (row, curve)) in done.into_iter().flat_map(|d| d.into_inner().unwrap()) {
            rows[pa * (m_max + 1) + m] = row;
            curves[pa * (m_max + 1) + m] = curve;
        }
        Self {
            age: m_max,
            rows,
            curves,
            seed: seed.into_inner().unwrap(),
        }
    }

    /// A lateral of `pa` made in cycle `birth`: a PA no zone draws has
    /// no row.
    pub fn lateral(&self, pa: usize, birth: u32) -> &Row {
        let m = self.age.saturating_sub(birth as usize);
        let row = &self.rows[pa * (self.age + 1) + m];
        debug_assert!(row.e > 0.0, "a row for PA {pa}");
        row
    }

    /// The row of a lateral of `pa` `m` cycles from the tree's age, none
    /// where no bud of the PA is borne (the predictor's proof).
    #[cfg(test)]
    pub(crate) fn row_at(&self, pa: usize, m: usize) -> Option<&Row> {
        self.rows.get(pa * (self.age + 1) + m).filter(|r| r.e > 0.0)
    }

    /// The chance that a lateral of `pa` made in cycle `birth` has no
    /// living apex past cycle `t`: its extinction curve, 0 before its
    /// birth.
    pub fn dead_by(&self, pa: usize, birth: u32, t: i64) -> f64 {
        let m = self.age.saturating_sub(birth as usize);
        let curve = &self.curves[pa * (self.age + 1) + m];
        let at = t - i64::from(birth);
        if at < 0 {
            return 0.0;
        }
        curve[(at as usize).min(curve.len() - 1)]
    }

    /// The seed: the whole tree.
    pub fn seed(&self) -> &Row {
        &self.seed
    }

    /// The rows held, for the memory they take.
    pub fn len(&self) -> usize {
        self.rows.iter().filter(|r| r.e > 0.0).count() + 1
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
fn read(kept: &mut Kept, species: &Species, pa: usize, m: usize) -> (Row, Vec<f64>) {
    let delay = species.states[pa].shedding;
    let grown = kept.of(pa, m, Carried::default(), None);
    let faded = kept.shed((pa, m, Carried::default(), None), delay);
    let reach = kept.reach_bound(pa, m, Carried::default(), None);
    (
        row(&faded, &grown, m, reach),
        grown.a[..=m.min(grown.a.len() - 1)].to_vec(),
    )
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
