//! The size of a point between species (fn-230 R1, host 2026-10-08):
//! averaging growth rates of differently built species compounds over
//! decades, so the blended rows are shrunk by one factor on every lateral
//! probability until the tree's light-neutral expected phytomers at its age,
//! kept and grown alike, are at most the ends' weighted geometric mean: each
//! count's own factor is solved and the smaller taken. Kept phytomers are the
//! tree's size; grown ones, the shed included, are what the engine's budget
//! counts, and a blend can shed a larger share than either end. Only shrunk:
//! a blend predicted at or below both means keeps its rows, so a walk to the
//! date palm, whose size is in fronds and not laterals, can come out smaller
//! than either end. Light only removes growth, so the light-neutral counts
//! bound a lit tree's from above.
use std::sync::Mutex;
use telperion_space::{expected_grown, expected_kept, Species};

use crate::{Error, Result};

/// Steps of the bracketed solve: its tolerance is reached in under ten on
/// every shipped pair; the rest bound a pathological one.
const STEPS: usize = 64;
/// The solve stops when the prediction is the target to this share.
const TOLERANCE: f64 = 1e-12;

/// The light-neutral predicted counts the size is matched on.
#[derive(Clone, Copy, PartialEq)]
enum Count {
    Kept,
    Grown,
}

impl Count {
    fn of(self, species: &Species, age: u32) -> Result<f64> {
        match self {
            Count::Kept => kept(species, age),
            Count::Grown => grown(species, age),
        }
    }

    /// `of` an end of a walk: the same at every frame, so it is taken from
    /// the ends a recent frame predicted, keyed by the species' every bit
    /// (its `Debug`, which writes each number to round trip) and the age.
    fn of_end(self, end: &Species, age: u32) -> Result<f64> {
        let key = (self, age, format!("{end:?}"));
        let lock = || ENDS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(&(.., n)) = lock()
            .iter()
            .find(|(c, a, k, _)| (*c, *a, k) == (key.0, key.1, &key.2))
        {
            return Ok(n);
        }
        let n = self.of(end, age)?;
        let mut ends = lock();
        if ends.len() == ENDS_HELD {
            ends.remove(0);
        }
        ends.push((key.0, key.1, key.2, n));
        Ok(n)
    }
}

/// Ends' predictions held: both counts of a walk's two ends, and of the
/// walk before it.
const ENDS_HELD: usize = 8;

/// Recently predicted ends, oldest first.
static ENDS: Mutex<Vec<(Count, u32, String, f64)>> = Mutex::new(Vec::new());

/// `shape` shrunk so neither its light-neutral expected kept nor grown
/// phytomers at `age` pass the geometric mean of `ends`' weighted by their
/// shares; unchanged when it is predicted at or below both, or when its age
/// grows nothing. The grown count is solved from the kept count's factor
/// down, so it costs one prediction where the kept count binds.
pub fn sized(shape: Species, ends: &[(&Species, f64)], age: u32) -> Result<Species> {
    if age == 0 {
        return Ok(shape);
    }
    let kept = factor(&shape, ends, age, Count::Kept, 1.0)?;
    let factor = factor(&shape, ends, age, Count::Grown, kept)?;
    Ok(if factor < 1.0 {
        scaled(&shape, factor)
    } else {
        shape
    })
}

/// The factor, at most `ceiling`, at which `count` of `shape` at `age` is the
/// ends' weighted geometric mean of it, or 0 where no factor brings it down
/// that far.
fn factor(
    shape: &Species,
    ends: &[(&Species, f64)],
    age: u32,
    count: Count,
    ceiling: f64,
) -> Result<f64> {
    if ceiling == 0.0 {
        return Ok(0.0);
    }
    let (mut logs, mut total) = (0.0, 0.0);
    for &(end, share) in ends.iter().filter(|(_, share)| *share > 0.0) {
        logs += share * count.of_end(end, age)?.ln();
        total += share;
    }
    let target = logs / total;
    let at = |x: f64| -> Result<f64> { Ok(count.of(&scaled(shape, x.exp()), age)?.ln() - target) };
    let top = ceiling.ln();
    let high = at(top)?;
    if high <= 0.0 {
        return Ok(ceiling);
    }
    solve(at, top, high)
}

/// The factor below `exp(top)` at which `at`, the log of the prediction over
/// the target as a function of the factor's log, is zero: the Illinois
/// variant of regula falsi on a bracket doubled down from `top`, so every
/// step keeps a root between its ends and a rerun lands on the same factor
/// to the bit.
fn solve(at: impl Fn(f64) -> Result<f64>, top: f64, high: f64) -> Result<f64> {
    let (mut b, mut gb) = (top, high);
    let (mut a, mut ga) = (top - 0.25, at(top - 0.25)?);
    let mut width = 0.25;
    while ga > 0.0 {
        if width > 64.0 {
            // What the tree grows without laterals is already above the
            // target (a first year grows no laterals): none comes nearest.
            return Ok(0.0);
        }
        (b, gb) = (a, ga);
        width *= 2.0;
        a = b - width;
        ga = at(a)?;
    }
    let mut side = 0;
    for _ in 0..STEPS {
        let x = (a * gb - b * ga) / (gb - ga);
        let gx = at(x)?;
        if gx.abs() <= TOLERANCE || x <= a || x >= b {
            return Ok(x.exp());
        }
        if gx > 0.0 {
            (b, gb) = (x, gx);
            if side == 1 {
                ga /= 2.0;
            }
            side = 1;
        } else {
            (a, ga) = (x, gx);
            if side == -1 {
                gb /= 2.0;
            }
            side = -1;
        }
    }
    Ok(a.exp())
}

/// The species' expected kept phytomers at `age` with its light rows at 0.
fn kept(species: &Species, age: u32) -> Result<f64> {
    let kept = expected_kept(&neutral(species), age).map_err(refused)?;
    kept.ok_or(REFUSED)
}

/// The species' expected grown phytomers at `age`, the shed ones included,
/// with its light rows at 0.
fn grown(species: &Species, age: u32) -> Result<f64> {
    expected_grown(&neutral(species), age).map_err(refused)
}

const REFUSED: Error = Error::InvalidInput("the predicted size of a blend");

fn refused(_: telperion_space::Error) -> Error {
    REFUSED
}

/// `species` with its light rows at 0.
fn neutral(species: &Species) -> Species {
    let mut neutral = species.clone();
    for state in &mut neutral.states {
        state.shade_hazard = 0.0;
        state.shade_size = 0.0;
        state.balance_hazard = 0.0;
        state.leaf_girth = 0.0;
    }
    neutral
}

/// `species` with every age's lateral probabilities multiplied by `factor`.
fn scaled(species: &Species, factor: f64) -> Species {
    let mut species = species.clone();
    for zone in species.states.iter_mut().flat_map(|s| &mut s.zones) {
        zone.lateral.iter_mut().for_each(|p| *p *= factor);
    }
    species
}
