//! A bud's own axis at one cycle t, its count alone (`kept/path.rs` and
//! `kept/ends.rs`, the same operations in the same order on the two
//! columns the count reads).
use super::{both, maybe, tilts, times, whole, Bud, Ended, Parts, Table, Walk, ZoneParts, NONE};
use crate::closed_form::along::Count;
use crate::closed_form::events::Event;
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
        // A failure with no relay ends the axis at the unit before; each
        // lived part grows its unit's nodes and ends in its relay (fn-199
        // R1).
        end(step.failed, step.i - 1, NONE_RUN, NONE);
        for (l, table) in step.lived.iter().zip(&parts.lived) {
            let grown = (l.weight, &l.sleeping[..]);
            let run = times(
                NONE_RUN,
                self.unit(k, (step.i, grown), (&parts.zones, 1.0), t),
            );
            let child = whole(table.as_deref().expect("built"), t - l.relay.at as i64);
            end(
                l.weight,
                step.i,
                run,
                maybe(l.relay.weight / l.weight, child),
            );
        }
        let grown = (step.grown, &step.sleeping[..]);
        let zones = (&parts.zones[..], step.laid);
        let run = times(NONE_RUN, self.unit(k, (step.i, grown), zones, t));
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
                // An age a lifespan's fraction ends inside its cycle ends
                // its axis that share into it: in expectation, at its own
                // cycle and the one before (`Step::ends_at`, fn-231).
                let u = step.ends_at;
                if u < 1.0 {
                    end(step.grown * u, step.i, run, e);
                    end(step.grown * (1.0 - u), step.i - 1, run, e);
                } else {
                    end(step.grown, step.i, run, e);
                }
            }
        }
        (a, n)
    }

    /// Unit `i`, grown with chance `grown` and bearing `sleeping`, laying
    /// `laid` of each zone's nodes (`Step::laid`): its factor and its
    /// phytomers on it.
    fn unit(
        &self,
        k: usize,
        (i, (grown, sleeping)): (usize, (f64, &[Vec<Event>])),
        (zones, laid): (&[ZoneParts], f64),
        t: i64,
    ) -> Ended {
        let state = &self.species.states[k];
        let mut run = NONE_RUN;
        for zp in zones {
            let zone = &state.zones[zp.z];
            let mut phi = 1.0;
            for (_, p, table) in &zp.laterals {
                phi += p * (table.at(t - i as i64).0 - 1.0);
            }
            // Each whole place a factor phi; the last, a fraction `s`
            // stands at, 1 + s (phi - 1) (fn-231).
            let (whole, s) = zone.whole_places();
            let psi = 1.0 + s * (phi - 1.0);
            let f = if s > 0.0 {
                phi.powi(whole) * psi
            } else {
                phi.powi(whole)
            };
            let pmf = &self.pmfs[k][zp.z];
            let (g, tilted, replaced) = match laid < 1.0 {
                true => tilts(&Count::laid_pmf(pmf, laid), f, zone),
                false => tilts(pmf, f, zone),
            };
            let mut h = tilted.moments().0 * g;
            let others = phi.powi(whole - 1);
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
                // Place 0's buds, then one a node on every other place;
                // the last, a fraction's, apart.
                let place = |slot: usize| n1 * if slot == 0 { band.1 - band.0 } else { p };
                if whole > 0 {
                    let mut buds = place(0);
                    for slot in 1..whole as usize {
                        buds += place(slot);
                    }
                    h += if s > 0.0 { others * psi } else { others } * (buds * c);
                }
                if s > 0.0 {
                    h += phi.powi(whole) * (s * place(whole as usize) * c);
                }
            }
            run = times(run, (g, h));
            for (event, table) in sleeping[zp.z].iter().zip(&zp.woken) {
                run = times(run, woken(t, grown, event, table.as_deref()));
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
