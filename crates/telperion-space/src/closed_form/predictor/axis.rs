//! A bud's own axis at one cycle t, its count alone (`kept/path.rs` and
//! `kept/ends.rs`, the same operations in the same order on the two
//! columns the count reads).
use super::{both, maybe, times, whole, Bud, Ended, Parts, Table, Walk, ZoneParts, NONE};
use crate::closed_form::events::Event;
use crate::closed_form::kept::tilts;
use crate::closed_form::steps::{End, Relay, Step};
use crate::transcendental::Transcendental;

impl Walk<'_> {
    /// `bud`'s table over its window, every table it reads in `parts`.
    pub(super) fn table(&self, (k, m, _, _): Bud, step: Option<&Step>, parts: &Parts) -> Table {
        let lo = m.saturating_sub(self.lags);
        let (mut a, mut n) = (
            Vec::with_capacity(m + 1 - lo),
            Vec::with_capacity(m + 1 - lo),
        );
        for t in lo..=m {
            let (at, nt) = match step {
                Some(step) => self.threshold((k, m), step, parts, t as i64),
                None => (1.0, 0.0),
            };
            a.push(at);
            n.push(nt);
        }
        Table { lo, a, n }
    }

    fn threshold(&self, (k, m): (usize, usize), step: &Step, parts: &Parts, t: i64) -> Ended {
        let (mut a, mut n) = (0.0, 0.0);
        let mut end = |p: f64, last: usize, (ra, rn): Ended, (ea, en): Ended| {
            if p <= 0.0 || last as i64 > t {
                return;
            }
            a += p * ra * ea;
            n += p * (rn * ea + ra * en);
        };
        let failed = step.before - step.grown;
        let e = relay_end(step.failed, failed, parts.failed.as_deref(), t);
        end(failed, step.i - 1, NONE_RUN, e);
        let run = times(NONE_RUN, self.unit(k, step, &parts.zones, t));
        match &step.end {
            None => {
                let aborted = step.grown - step.after;
                let e = relay_end(step.aborted, aborted, parts.aborted.as_deref(), t);
                end(aborted, step.i, run, e);
                if step.i == m {
                    end(step.after, m, run, NONE);
                } else if let Some(rest) = &parts.rest {
                    end(step.after, step.i, run, whole(rest, t - step.i as i64));
                }
            }
            Some(stop) => {
                let e = age_end(stop, step.grown, parts, t);
                end(step.grown, step.i, run, e);
            }
        }
        (a, n)
    }

    /// The unit `step`: its factor and its phytomers on it.
    fn unit(&self, k: usize, step: &Step, zones: &[ZoneParts], t: i64) -> Ended {
        let state = &self.species.states[k];
        let i = step.i;
        let mut run = NONE_RUN;
        for zp in zones {
            let zone = &state.zones[zp.z];
            let mut phi = 1.0;
            for (_, p, table) in &zp.laterals {
                phi += p * (table.at(t - i as i64).0 - 1.0);
            }
            let places = zone.places() as i32;
            let f = phi.powi(places);
            let (g, tilted, replaced) = tilts(&self.pmfs[k][zp.z], f, zone);
            let mut h = tilted.moments().0 * g;
            let others = phi.powi(places - 1);
            let n1 = replaced.moments().0;
            let mut low = 0.0;
            for (j, &p) in self.laterals[k][zp.z].iter().enumerate() {
                let band = (low, low + p);
                low += p;
                let Some((_, _, table)) = zp.laterals.iter().find(|x| x.0 == j) else {
                    continue;
                };
                let Some(c) = table.at(t - i as i64).1 else {
                    continue;
                };
                // Place 0's buds, then one a node on every other place.
                let mut buds = n1 * (band.1 - band.0);
                for _ in 1..zone.places() {
                    buds += n1 * p;
                }
                h += others * (buds * c);
            }
            run = times(run, (g, h));
            for (event, table) in step.sleeping[zp.z].iter().zip(&zp.woken) {
                run = times(run, woken(t, step.grown, event, table.as_deref()));
            }
        }
        run
    }
}

/// The running product before any part: E[1], E[0].
const NONE_RUN: Ended = (1.0, 0.0);

/// A woken bud's factor and phytomers: Poisson many over the unit's
/// chance `grown`.
fn woken(t: i64, grown: f64, event: &Event, table: Option<&Table>) -> Ended {
    let (Event::Bud { at, weight, .. }, Some(table)) = (event, table) else {
        return NONE;
    };
    if grown <= 0.0 {
        return NONE;
    }
    let (a, n) = table.at(t - *at as i64);
    let lambda = weight / grown;
    let factor = (lambda * (a - 1.0)).exp_fixed();
    (factor, lambda * factor * n.unwrap_or(0.0))
}

/// The end a relay makes of an end of chance `p`.
fn relay_end(relay: Option<Relay>, p: f64, table: Option<&Table>, t: i64) -> Ended {
    let Some(r) = relay.filter(|_| p > 0.0) else {
        return NONE;
    };
    let child = whole(table.expect("a relay's table"), t - r.at as i64);
    maybe((r.weight / p).min(1.0), child)
}

/// The end an age makes that ends with chance `p`: its continuation and
/// its relay.
fn age_end(stop: &End, p: f64, parts: &Parts, t: i64) -> Ended {
    if p <= 0.0 {
        return NONE;
    }
    let relay = stop.relay.map_or(0.0, |r| r.weight / p);
    let left = match stop.relay {
        Some(r) => whole(parts.relay.as_deref().expect("built"), t - r.at as i64),
        None => NONE,
    };
    let Some((born, _, _, weight)) = stop.continuation else {
        return maybe(relay.min(1.0), left);
    };
    let go = (weight / p).min(1.0);
    let on = whole(parts.onward.as_deref().expect("built"), t - born as i64);
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
    let (a1, n1) = both(on, maybe(with, left));
    let (a0, n0) = maybe(without, left);
    (go * a1 + (1.0 - go) * a0, go * n1 + (1.0 - go) * n0)
}
