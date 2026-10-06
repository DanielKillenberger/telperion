//! A bud's own axis at one cycle t (`kept.rs`): unit by unit, the chance
//! that every subtree it bears has stopped living by t and the quantities
//! on that event, then each way the axis may end weighed in (`ends.rs`).
//!
//! A zone's subtrees enter as a factor f a node, so its node count N
//! enters as E[f^N]; its own wood, whose place and size grow with N, as
//! that tilted by f^N; and a borne subtree's quantity, in one node, as
//! E[N f^(N-1)] with the node's rank (and, for the section's square, two
//! of them in two nodes, E[N (N-1) f^(N-2)]). Units and zones are
//! independent, so the tilts multiply, and the places of later wood are
//! the tilted lengths of the wood before it.
use super::ends::{Ended, NONE};
use super::mass::Units;
use super::section::{handed, zoned};
use super::{Kept, Placing, Table, LEAVES, LEAVES_ALONG, PARALLEL, SECTION, SECTION2, WIDTH, WOOD};
use crate::closed_form::along::{dominance_power, Along, Count};
use crate::closed_form::events::Event;
use crate::closed_form::steps::Step;
use crate::girth::ripe;
use crate::schedule::Carried;
use crate::species::{Species, Zone, ALONG};

/// What a bud's own PA lays down, the same at every cycle: each zone's
/// node-count law.
pub(super) struct Axis {
    pmfs: Vec<Vec<f64>>,
}

impl Axis {
    pub fn of(species: &Species, k: usize) -> Self {
        let pmfs = species.states[k].zones.iter().map(|z| Count::pmf(z.nodes));
        Self {
            pmfs: pmfs.collect(),
        }
    }
}

/// The running product over the parts grown: E[Π f] and E[Π f · Y].
pub(super) struct Running {
    pub a: f64,
    pub b: [f64; WIDTH],
}

impl Running {
    fn new() -> Self {
        Self {
            a: 1.0,
            b: [0.0; WIDTH],
        }
    }

    /// Another independent factor `g` with its quantities `h` (E[g · Y]);
    /// the section's square gains the cross term of the two.
    fn times(&mut self, g: f64, h: &[f64; WIDTH]) {
        let section = self.b[SECTION];
        for (b, &h) in self.b.iter_mut().zip(h) {
            *b = *b * g + self.a * h;
        }
        self.b[SECTION2] += 2.0 * section * h[SECTION];
        self.a *= g;
    }
}

impl Kept<'_> {
    pub(super) fn threshold(
        &mut self,
        (k, m): (usize, usize),
        axis: &Axis,
        (steps, rest): (&[Step], Option<&Table>),
        t: i64,
    ) -> (f64, [f64; WIDTH]) {
        let relay_at = self.species.states[k].relay_at;
        let mut run = Running::new();
        let mut units = Units::new(self.species, k);
        let (mut a, mut b) = (0.0, [0.0; WIDTH]);
        // Weighs in one way the axis ends: chance `p`, its last unit
        // `last`, and the end's factor and quantities.
        let mut end = |p: f64, last: usize, run: &Running, units: &Units, (ea, eb, er): Ended| {
            if p <= 0.0 || last as i64 > t {
                return;
            }
            a += p * run.a * ea;
            let mut add: [f64; WIDTH] = std::array::from_fn(|q| run.b[q] * ea + run.a * eb[q]);
            add[SECTION2] += 2.0 * run.b[SECTION] * eb[SECTION];
            units.mass(last, (ea, &eb, er), run.a, &mut add);
            b.iter_mut().zip(add).for_each(|(b, v)| *b += p * v);
        };
        // The start of the next unit, as tilted by what the units bear.
        let mut path = Along::default();
        for step in steps {
            let on = |u: usize, units: &Units| {
                u.checked_sub(1).map_or(Along::default(), |u| {
                    let (start, length) = units.place(u);
                    start.into(length, relay_at)
                })
            };
            let failed = step.before - step.grown;
            let e = self.relay_end((k, m, t), step.failed, failed, on(step.i - 1, &units));
            end(failed, step.i - 1, &run, &units, e);
            let (g, h, length) = self.unit((k, m, t), axis, step, (path, &mut units));
            run.times(g, &h);
            units.close(path, length);
            path = path.then(length);
            match &step.end {
                None => {
                    let aborted = step.grown - step.after;
                    let e = self.relay_end((k, m, t), step.aborted, aborted, on(step.i, &units));
                    end(aborted, step.i, &run, &units, e);
                    if step.i == m {
                        end(step.after, m, &run, &units, NONE);
                    } else if let Some(rest) = rest {
                        // The rest of the axis, one cycle younger, from
                        // the unit's tip.
                        let e = self.species.states[k].form.exponent;
                        let at = t - step.i as i64;
                        let rest = self.whole(rest, at, (path, 1.0), e);
                        end(step.after, step.i, &run, &units, rest);
                    }
                }
                Some(stop) => {
                    let at = on(step.i, &units);
                    let e = self.age_end((k, m, t), stop, step.grown, (path, at));
                    end(step.grown, step.i, &run, &units, e);
                }
            }
        }
        if steps.is_empty() {
            a = 1.0;
        }
        (a, b)
    }

    /// Unit `step` from `start`: its factor, its quantities on it, and its
    /// length as tilted; its zones handed to `units` for sag.
    fn unit(
        &mut self,
        (k, m, t): (usize, usize, i64),
        axis: &Axis,
        step: &Step,
        (start, units): (Along, &mut Units),
    ) -> (f64, [f64; WIDTH], Along) {
        let species = self.species;
        let state = &species.states[k];
        let (i, size, e) = (step.i, step.size, state.form.exponent);
        let l = state.internode * size;
        let form = &state.form;
        let own = form.pipe * size * ripe((m - i + 1) as f64, form.ripening);
        let mut run = Running::new();
        let established = form.pipe * size * ripe(1.0, form.ripening);
        let mut at = start;
        let mut length = Along::default();
        for &z in ALONG.iter().filter(|&&z| z < state.zones.len()) {
            let zone = &state.zones[z];
            let laterals = self.laterals.clone();
            let lateral = &laterals[k][z];
            // Each place's factor: bare, or a lateral whose subtree has
            // stopped living by t; and each lateral's table.
            let mut tables = Vec::new();
            let mut phi = 1.0;
            for (j, &p) in lateral.iter().enumerate().filter(|(_, &p)| p > 0.0) {
                let lived = species.lived(j).map(|(q, _)| &species.states[q]);
                let delay = lived.map_or(f64::INFINITY, |s| s.shedding);
                let table = self.child((j, m - i, Carried::default(), None), delay);
                phi += p * (table.at(t - i as i64).0 - 1.0);
                tables.push((j, p, table));
            }
            let places = zone.places() as i32;
            let f = phi.powi(places);
            let (g, tilted, replaced) = tilts(&axis.pmfs[z], f, zone);
            let mut h = self.own(&tilted, at, l, own).map(|v| v * g);
            // One node's place holds the subtree; its other places, and
            // every other node, their factors.
            let others = phi.powi(places - 1);
            let mut slots = vec![(0.0, 0.0); zone.places()];
            let mut low = 0.0;
            for (j, &p) in lateral.iter().enumerate() {
                let band = (low, low + p);
                low += p;
                let Some((_, _, table)) = tables.iter().find(|x| x.0 == j) else {
                    continue;
                };
                let row = table.at(t - i as i64);
                let Some(cb) = row.2 else {
                    continue;
                };
                let placing = self.laterals_at(j, (zone, &replaced), (at, l, size), (p, band));
                let mut y = [0.0; WIDTH];
                placing.place(cb, &mut y);
                for q in 0..WIDTH {
                    h[q] += others * y[q];
                }
                // Its section, handed over in its place's share.
                let (x1, x2) = handed(row, table.e, e);
                let lived = species.lived(j).unwrap_or((j, 0.0));
                let d = species.states[lived.0].form.dominance;
                for (slot, s) in slots.iter_mut().enumerate() {
                    let base = size * zone.share(slot) * lived.1;
                    s.0 += p * base.powf(e) * dominance_power(d, e) * x1;
                    s.1 += p * base.powf(2.0 * e) * dominance_power(d, 2.0 * e) * x2;
                }
            }
            let (s1, s2) = zoned(
                &axis.pmfs[z],
                (f, phi, places),
                (own.powf(e), own.powf(2.0 * e)),
                &slots,
            );
            (h[SECTION], h[SECTION2]) = (s1, s2);
            run.times(g, &h);
            units.zone((l, at), &tilted, (g, s1, s2), established);
            let zone_length = tilted.length(l);
            at = at.then(zone_length);
            length = length.then(zone_length);
            for event in &step.sleeping[z] {
                let (fa, fb, _) = self.woken((m, t), step.grown, event, e);
                run.times(fa, &fb);
                units.woken(fa, &fb);
            }
        }
        (run.a, run.b, length)
    }

    /// The quantities of a zone's own phytomers, its node count `n` (as
    /// tilted), from `x`, each `l` long and its pipe `o`; their section
    /// is the zone's (`section::zone`).
    fn own(&self, n: &Count, x: Along, l: f64, o: f64) -> [f64; WIDTH] {
        let mut y = [0.0; WIDTH];
        let [w, a, q] = n.wood(x, l);
        y[WOOD] = w;
        y[super::ALONG] = a;
        y[super::SQUARE] = q;
        y[PARALLEL] = q;
        y[LEAVES] = w * o * o;
        y[LEAVES_ALONG] = a * o * o;
        y
    }

    /// How laterals of PA `j` stand on a zone whose one node is replaced
    /// (`replaced`: the node count's moments so weighted) from `x`, each
    /// node `l` long on a unit of `size`.
    fn laterals_at(
        &self,
        j: usize,
        (zone, replaced): (&Zone, &Count),
        (x, l, size): (Along, f64, f64),
        (p, band): (f64, (f64, f64)),
    ) -> Placing {
        let species = self.species;
        let (lived, reach) = species.lived(j).unwrap_or((j, 0.0));
        let lived = &species.states[lived];
        let d = lived.form.dominance;
        let mut placing = Placing::turned(lived.insertion.cos());
        let powers = |share: f64| -> [f64; 4] {
            let base = size * share * reach;
            std::array::from_fn(|p| base.powi(p as i32 + 1) * dominance_power(d, p as f64 + 1.0))
        };
        // Place 0's draw also orders the nodes (`grow/unit.rs`).
        let (n, at) = replaced.ranked(x, l, band);
        placing.add(n, &powers(zone.share(0)), at);
        for slot in 1..zone.places() {
            let (n, at) = replaced.spread(x, l, false);
            placing.add(n * p, &powers(zone.share(slot)), at);
        }
        placing
    }

    /// A woken bud's factor and quantities: as many as its expectation
    /// over the unit's chance (`grown`), each independent of the axis,
    /// handed to it at its exponent `e`.
    fn woken(&mut self, (m, t): (usize, i64), grown: f64, event: &Event, e: f64) -> Ended {
        let Event::Bud {
            at,
            pa,
            carried,
            weight,
            first,
            unit,
            stand,
            ..
        } = event
        else {
            return NONE;
        };
        if grown <= 0.0 {
            return NONE;
        }
        let species = self.species;
        let lived = species.lived(*pa).map(|(q, _)| &species.states[q]);
        let delay = lived.map_or(f64::INFINITY, |s| s.shedding);
        let table = self.child((*pa, m - at, *carried, *first), delay);
        let row = table.at(t - *at as i64);
        let lambda = weight / grown;
        let factor = (lambda * (row.0 - 1.0)).exp();
        let mut y = [0.0; WIDTH];
        if let Some(cb) = row.2 {
            let share = stand.shares[0] * unit;
            let mut placing = Placing::turned(stand.angle.cos());
            let x = stand.along[0] / stand.shares[0].max(f64::MIN_POSITIVE);
            let x2 = stand.square / stand.shares[0].max(f64::MIN_POSITIVE);
            let powers = std::array::from_fn(|p| share.powi(p as i32 + 1));
            placing.add(
                1.0,
                &powers,
                Along {
                    mean: x,
                    square: x2,
                },
            );
            placing.place(cb, &mut y);
        }
        let (x1, x2) = handed(row, table.e, e);
        let share = (stand.shares[0] * unit).powf(e);
        // A Poisson count of them: E[Σ X²] gains the pairs, λ² E[X]².
        y[SECTION] = share * x1;
        y[SECTION2] = share * share * (x2 + lambda * x1 * x1);
        (factor, y.map(|v| lambda * factor * v), [0.0; 2])
    }
}

/// A zone's node count under its subtrees' factor `f` a node: E[f^N], the
/// count's moments tilted by f^N (normalized), and its moments with one
/// node replaced, E[N^p f^(N-1)].
fn tilts(pmf: &[f64], f: f64, zone: &Zone) -> (f64, Count, Count) {
    let (mut g, mut mu, mut nu) = (0.0, [0.0; 3], [0.0; 3]);
    for (n, &pn) in pmf.iter().enumerate() {
        let tilt = pn * f.powi(n as i32);
        g += tilt;
        if n > 0 {
            let (nf, one) = (n as f64, pn * f.powi(n as i32 - 1));
            for p in 0..3 {
                mu[p] += tilt * nf.powi(p as i32 + 1);
                nu[p] += one * nf.powi(p as i32 + 1);
            }
        }
    }
    let tilted = if g > 0.0 {
        Count::from_moments(mu[0] / g, mu[1] / g, mu[2] / g)
    } else {
        Count::of(zone.nodes)
    };
    (g, tilted, Count::from_moments(nu[0], nu[1], nu[2]))
}
