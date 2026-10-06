//! The mass sag hangs on a bud's own axis at its true exponent (fn-209,
//! host decision 15). A phytomer weighs its length times its radius
//! squared (`sag.rs`), its radius the section it carries to the power
//! 1 / e, blended with the radius it was established at by the PA's
//! secondary growth (`girth.rs`). The section a phytomer carries is its
//! own node's and every node's beyond it, and everything after them: each
//! zone's nodes in turn, every zone after it, and what the axis's end
//! leaves, each independent, so their means and spreads add. Within a
//! zone of N nodes the n-th phytomer carries N − n + 1 nodes' worth, an
//! average of (N + 1) / 2, weighed by length and by length and place;
//! the radius's square is taken by the delta method (`section.rs`), so
//! where it is linear (exponent 2, whole secondary growth) the mass is
//! exact.
//!
//! A relay's section enters at its node, its PA's `relay_at` share along
//! the last unit (`geometry::relay_point`): the phytomers before it carry
//! it, those after do not, taken by that share of the unit's length (its
//! node count's ceiling where the nodes are whole).
use super::section::delta;
use super::{MASS, MASS_ALONG, SECTION, SECTION2, WIDTH};
use crate::closed_form::along::{Along, Count};
use crate::species::Species;

/// One zone's nodes along the axis, as tilted: each `l` long from
/// `start`, their count's moments, their section's mean and spread, and
/// the pipe their phytomers were established with.
struct Segment {
    l: f64,
    start: f64,
    count: (f64, f64, f64),
    mean: f64,
    spread: f64,
    established: f64,
}

/// The units grown so far: where each starts and how long it is, and
/// their zones' segments, with where each unit's end.
pub(super) struct Units {
    exponent: f64,
    secondary: f64,
    relay_at: f64,
    places: Vec<(Along, Along)>,
    segments: Vec<Segment>,
    ends: Vec<usize>,
}

impl Units {
    pub fn new(species: &Species, k: usize) -> Self {
        let form = &species.states[k].form;
        Self {
            exponent: form.exponent,
            secondary: form.secondary,
            relay_at: species.states[k].relay_at,
            places: Vec::new(),
            segments: Vec::new(),
            ends: Vec::new(),
        }
    }

    /// A zone of the unit growing now: its nodes `l` long from `start`,
    /// their count as tilted, its section on its factor `g` (`s1`, `s2`),
    /// and the pipe its phytomers were established with.
    pub fn zone(
        &mut self,
        (l, start): (f64, Along),
        count: &Count,
        (g, s1, s2): (f64, f64, f64),
        established: f64,
    ) {
        let (mean, spread) = moments(g, s1, s2);
        self.segments.push(Segment {
            l,
            start: start.mean,
            count: count.moments(),
            mean,
            spread,
            established,
        });
    }

    /// A woken bud's section in the zone last added.
    pub fn woken(&mut self, g: f64, h: &[f64; WIDTH]) {
        let (mean, spread) = moments(g, h[SECTION], h[SECTION2]);
        if let Some(s) = self.segments.last_mut() {
            s.mean += mean;
            s.spread += spread;
        }
    }

    /// The unit growing now has ended at `start`, `length` long.
    pub fn close(&mut self, start: Along, length: Along) {
        self.places.push((start, length));
        self.ends.push(self.segments.len());
    }

    /// Where unit `u` (from 0) starts, and its length.
    pub fn place(&self, u: usize) -> (Along, Along) {
        self.places[u]
    }

    /// Adds to `add` the mass of the first `last` units' own phytomers,
    /// the axis ending with `end` (its relay's part apart), on the units'
    /// factor `a`.
    pub fn mass(
        &self,
        last: usize,
        (ea, eb, relay): (f64, &[f64; WIDTH], [f64; 2]),
        a: f64,
        add: &mut [f64; WIDTH],
    ) {
        if last == 0 || ea <= 0.0 {
            return;
        }
        let (e, s) = (self.exponent, self.secondary);
        // Whole secondary growth leaves the established pipe no share.
        let square = |mean: f64, spread: f64, est: f64| {
            let power = |r: f64| delta(1.0, mean, spread + mean * mean, r);
            if s == 1.0 {
                return power(2.0 / e);
            }
            s * s * power(2.0 / e)
                + 2.0 * s * (1.0 - s) * est * power(1.0 / e)
                + (1.0 - s) * (1.0 - s) * est * est
        };
        let (mut after, mut spread) = moments(ea, eb[SECTION], eb[SECTION2]);
        let (relay, relay_spread) = moments(ea, relay[0], relay[1]);
        let (first, end) = (
            if last > 1 { self.ends[last - 2] } else { 0 },
            self.ends[last - 1],
        );
        // The last unit's length, and where along it the relay stands.
        let length: f64 = self.segments[first..end]
            .iter()
            .map(|g| g.l * g.count.0)
            .sum();
        let mut from = length;
        for (i, seg) in self.segments[..end].iter().enumerate().rev() {
            let (n1, n2, n3) = seg.count;
            let (l, x) = (seg.l, seg.start);
            // The share of its nodes before the relay's node: all of them
            // before the last unit.
            let before = if i >= first && length > 0.0 {
                from -= l * n1;
                ((self.relay_at * length - from) / (l * n1).max(f64::MIN_POSITIVE)).clamp(0.0, 1.0)
            } else {
                1.0
            };
            if n1 > 0.0 {
                let node = seg.mean / n1;
                // Σ_n (N − n + 1) = N (N + 1) / 2, and by place,
                // Σ_n' n' (x + l n' / 2) over the nodes beyond.
                let carried = node * (n2 + n1) / 2.0;
                let wood = l * n1;
                let placed = l * (n1 * x + l * n2 / 2.0);
                let by_place =
                    node * l * (x * (n2 + n1) / 2.0 + l * (2.0 * n3 + 3.0 * n2 + n1) / 12.0);
                let placed_before = l * (before * n1 * x + l * before * before * n2 / 2.0);
                let within = seg.spread / 2.0;
                let est = seg.established;
                let (on, off) = (
                    (after, spread),
                    (after - relay, (spread - relay_spread).max(0.0)),
                );
                // A share of no nodes weighs nothing.
                for ((mean, sp), w, p) in [
                    (on, before * wood, placed_before),
                    (off, (1.0 - before) * wood, placed - placed_before),
                ] {
                    if w != 0.0 {
                        add[MASS] += a * ea * w * square(mean + carried / n1, sp + within, est);
                    }
                    if placed > 0.0 && p != 0.0 {
                        add[MASS_ALONG] +=
                            a * ea * p * square(mean + by_place / placed, sp + within, est);
                    }
                }
            }
            after += seg.mean;
            spread += seg.spread;
        }
    }
}

/// A part's section mean and spread on its factor `g`.
fn moments(g: f64, s1: f64, s2: f64) -> (f64, f64) {
    if g <= 0.0 {
        return (0.0, 0.0);
    }
    let mean = s1 / g;
    (mean, (s2 / g - mean * mean).max(0.0))
}
