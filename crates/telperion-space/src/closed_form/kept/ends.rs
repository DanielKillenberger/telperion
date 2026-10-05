//! How a bud's own axis ends, weighed in (`path.rs`): a failed unit or an
//! abortion may leave a relay; an age that ends moves on by its
//! continuation, stops and may relay, or, all but stopped, does both.
//! Each end is a factor (the chance what it leaves has stopped living by
//! the cycle) and its quantities, which a relay and a continuation hand to
//! the axis at its exponent (`section.rs`).
use super::section::handed;
use super::{Kept, Placing, SECTION, SECTION2, WIDTH};
use crate::closed_form::along::Along;
use crate::closed_form::steps::{End, Relay};
use crate::schedule::Carried;

/// An end's factor, its quantities on it, and the part of its section a
/// relay hands over at its node rather than at the tip (E[S], E[S²]).
pub(super) type Ended = (f64, [f64; WIDTH], [f64; 2]);

/// No end: nothing left.
pub(super) const NONE: Ended = (1.0, [0.0; WIDTH], [0.0; 2]);

/// Two independent ends at once: their factors multiply, and the square
/// of their summed section gains its cross term.
pub(super) fn both((a1, b1, r1): Ended, (a2, b2, r2): Ended) -> Ended {
    let mut b: [f64; WIDTH] = std::array::from_fn(|q| a2 * b1[q] + a1 * b2[q]);
    b[SECTION2] += 2.0 * b1[SECTION] * b2[SECTION];
    (a1 * a2, b, std::array::from_fn(|q| a2 * r1[q] + a1 * r2[q]))
}

/// `share` of an end, and the rest none.
fn maybe(share: f64, (a, b, r): Ended) -> Ended {
    (
        1.0 - share + share * a,
        b.map(|v| share * v),
        r.map(|v| share * v),
    )
}

impl Kept<'_> {
    /// The end a relay `relay` makes of an end of chance `p`, the relay
    /// standing at `at`.
    pub(super) fn relay_end(
        &mut self,
        (k, m, t): (usize, usize, i64),
        relay: Option<Relay>,
        p: f64,
        at: Along,
    ) -> Ended {
        let Some(r) = relay.filter(|_| p > 0.0) else {
            return NONE;
        };
        let child = self.relay((k, m, t), (r.at, r.carried), at);
        maybe((r.weight / p).min(1.0), child)
    }

    /// A relay of PA `k` made at `born`, standing at `at`: shed by its
    /// PA's delay, its section entering the axis's at the tip.
    fn relay(
        &mut self,
        (k, m, t): (usize, usize, i64),
        (born, carried): (usize, Carried),
        at: Along,
    ) -> Ended {
        let state = &self.species.states[k];
        let (insertion, delay, e) = (state.insertion, state.shedding, state.form.exponent);
        let table = self.shed((k, m - born, carried, None), delay);
        let (a, b, _) = self.whole(&table, t - born as i64, (at, insertion.cos()), e);
        (a, b, [b[SECTION], b[SECTION2]])
    }

    /// A whole child's factor and its quantities at the axis's exponent
    /// `e`, placed at `at` turned by `turn`.
    fn whole(&self, table: &super::Table, t: i64, (at, turn): (Along, f64), e: f64) -> Ended {
        let row = table.at(t);
        let mut out = [0.0; WIDTH];
        if let (_, _, Some(b)) = row {
            Placing::whole(at, turn).place(b, &mut out);
        }
        (out[SECTION], out[SECTION2]) = handed(row, table.e, e);
        (row.0, out, [0.0; 2])
    }

    /// The end an age makes that ends with chance `p`: its continuation at
    /// the tip `tip`, its relay at `at`.
    pub(super) fn age_end(
        &mut self,
        (k, m, t): (usize, usize, i64),
        stop: &End,
        p: f64,
        (tip, at): (Along, Along),
    ) -> Ended {
        if p <= 0.0 {
            return NONE;
        }
        let relay = stop.relay.map_or(0.0, |r| r.weight / p);
        let left = match stop.relay {
            Some(r) => self.relay((k, m, t), (r.at, r.carried), at),
            None => NONE,
        };
        let Some((born, pa, onward, weight)) = stop.continuation else {
            return maybe(relay.min(1.0), left);
        };
        let go = (weight / p).min(1.0);
        let e = self.species.states[k].form.exponent;
        let table = self.of(pa, m - born, onward, None);
        let on = self.whole(&table, t - born as i64, (tip, 1.0), e);
        // Given it moves on, its relay comes as the borderline's share;
        // given it stops, as the rest.
        let with = if go > 0.0 {
            (stop.both / go).min(1.0)
        } else {
            0.0
        };
        let without = if go < 1.0 {
            ((relay - stop.both).max(0.0) / (1.0 - go)).min(1.0)
        } else {
            0.0
        };
        let (a1, b1, r1) = both(on, maybe(with, left));
        let (a0, b0, r0) = maybe(without, left);
        (
            go * a1 + (1.0 - go) * a0,
            std::array::from_fn(|q| go * b1[q] + (1.0 - go) * b0[q]),
            std::array::from_fn(|q| go * r1[q] + (1.0 - go) * r0[q]),
        )
    }
}
