//! A bud's own axis at one cycle t (`kept.rs`): unit by unit, the chance
//! that every subtree it bears has stopped living by t and the quantities
//! on that event, then each way the axis may end weighed in.
//!
//! A zone's subtrees enter as a factor f a node, so its node count N
//! enters as E[f^N]; its own wood, whose place and size grow with N, as
//! that tilted by f^N; and a borne subtree's quantity, in one node, as
//! E[N f^(N-1)] with the node's rank. Units and zones are independent, so
//! the tilts multiply, and the places of later wood are the tilted
//! lengths of the wood before it.
use super::{Kept, Placing, LEAVES, LEAVES_ALONG, MOMENT, PARALLEL, PATH, SECTIONS, WOOD};
use crate::closed_form::along::{dominance_power, Along, Count};
use crate::closed_form::events::Event;
use crate::closed_form::steps::{Relay, Step};
use crate::girth::ripe;
use crate::schedule::Carried;
use crate::species::{Species, ALONG};

/// What a bud's own PA lays down, the same at every cycle: each zone's
/// node-count law, in the order the zones stand along a unit.
pub(super) struct Axis {
    pmfs: Vec<Vec<f64>>,
}

impl Axis {
    pub fn of(species: &Species, k: usize) -> Self {
        let pmfs = species.states[k]
            .zones
            .iter()
            .map(|z| Count::pmf(z.nodes))
            .collect();
        Self { pmfs }
    }
}

/// The running product over the units grown: E[Π f] and E[Π f · Y].
struct Running {
    a: f64,
    b: Vec<f64>,
}

impl Running {
    /// Another independent factor `g` with its quantities `h` (E[g · Y]).
    fn times(&mut self, g: f64, h: &[f64]) {
        for (b, &h) in self.b.iter_mut().zip(h) {
            *b = *b * g + self.a * h;
        }
        self.a *= g;
    }
}

impl Kept<'_> {
    pub(super) fn threshold(
        &mut self,
        (k, m): (usize, usize),
        axis: &Axis,
        steps: &[Step],
        t: i64,
    ) -> (f64, Vec<f64>) {
        let width = self.width();
        let state = &self.species.states[k];
        let mut run = Running {
            a: 1.0,
            b: vec![0.0; width],
        };
        let (mut a, mut b) = (0.0, vec![0.0; width]);
        // Weighs in one way the axis ends: chance `p`, its last unit `last`,
        // and its end's factor and quantities.
        let mut end = |p: f64, last: usize, run: &Running, (ea, eb): (f64, Vec<f64>)| {
            if p <= 0.0 || last as i64 > t {
                return;
            }
            a += p * run.a * ea;
            for q in 0..width {
                b[q] += p * (run.b[q] * ea + run.a * eb[q]);
            }
        };
        // The start of the next unit, and each unit's start and length,
        // as tilted by what the units bear.
        let mut path = Along::default();
        let mut units: Vec<(Along, Along)> = Vec::new();
        for step in steps {
            let on = |u: usize, units: &[(Along, Along)]| {
                u.checked_sub(1).map_or(Along::default(), |u| {
                    units[u].0.into(units[u].1, state.relay_at)
                })
            };
            let failed = step.before - step.grown;
            let at = on(step.i - 1, &units);
            let e = self.relay_end(k, m, step.failed, failed, at, t);
            end(failed, step.i - 1, &run, e);
            let (g, h, length) = self.unit((k, m), axis, step, path, t);
            run.times(g, &h);
            units.push((path, length));
            path = path.then(length);
            match &step.end {
                None => {
                    let aborted = step.grown - step.after;
                    let at = on(step.i, &units);
                    let e = self.relay_end(k, m, step.aborted, aborted, at, t);
                    end(aborted, step.i, &run, e);
                    if step.i == m {
                        end(step.after, m, &run, (1.0, vec![0.0; width]));
                    }
                }
                Some(stop) => {
                    let at = on(step.i, &units);
                    let e = self.age_end(k, m, stop, step.grown, (path, at), t);
                    end(step.grown, step.i, &run, e);
                }
            }
        }
        if steps.is_empty() {
            a = 1.0;
        }
        (a, b)
    }

    /// The factor and quantities of a relay `relay` an end of chance `p`
    /// may leave, standing at `at`.
    fn relay_end(
        &mut self,
        k: usize,
        m: usize,
        relay: Option<Relay>,
        p: f64,
        at: Along,
        t: i64,
    ) -> (f64, Vec<f64>) {
        let width = self.width();
        let Some(r) = relay.filter(|_| p > 0.0) else {
            return (1.0, vec![0.0; width]);
        };
        let q = (r.weight / p).min(1.0);
        let (ra, rb) = self.relay_child(k, m, (r.at, r.carried), at, t);
        (1.0 - q + q * ra, rb.iter().map(|v| q * v).collect())
    }

    /// A relay's factor and quantities, placed at `at`.
    fn relay_child(
        &mut self,
        k: usize,
        m: usize,
        (born, carried): (usize, Carried),
        at: Along,
        t: i64,
    ) -> (f64, Vec<f64>) {
        let state = &self.species.states[k];
        let table = self.shed((k, m - born, carried, None), state.shedding);
        let (ra, rb) = table.at(t - born as i64);
        let mut out = vec![0.0; self.width()];
        if !rb.is_empty() {
            self.whole(at, state.insertion.cos()).place(rb, &mut out, 0);
        }
        (ra, out)
    }

    /// The factor and quantities of an age's end of chance `p`: its
    /// continuation at the tip `tip`, its relay at `at`.
    fn age_end(
        &mut self,
        k: usize,
        m: usize,
        stop: &crate::closed_form::steps::End,
        p: f64,
        (tip, at): (Along, Along),
        t: i64,
    ) -> (f64, Vec<f64>) {
        let width = self.width();
        if p <= 0.0 {
            return (1.0, vec![0.0; width]);
        }
        let (mut ra, mut rb) = (1.0, vec![0.0; width]);
        let relay = stop.relay.map(|r| r.weight / p).unwrap_or(0.0);
        if let Some(r) = stop.relay {
            (ra, rb) = self.relay_child(k, m, (r.at, r.carried), at, t);
        }
        let Some((born, pa, onward, weight)) = stop.continuation else {
            let q = relay.min(1.0);
            return (1.0 - q + q * ra, rb.iter().map(|v| q * v).collect());
        };
        let go = (weight / p).min(1.0);
        let table = self.of(pa, m - born, onward, None);
        let (ca, cb) = table.at(t - born as i64);
        let mut c = vec![0.0; width];
        if !cb.is_empty() {
            self.whole(tip, 1.0).place(cb, &mut c, 0);
        }
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
        let r1 = 1.0 - with + with * ra;
        let r0 = 1.0 - without + without * ra;
        let ea = go * ca * r1 + (1.0 - go) * r0;
        let eb = (0..width)
            .map(|q| go * (c[q] * r1 + ca * with * rb[q]) + (1.0 - go) * without * rb[q])
            .collect();
        (ea, eb)
    }

    /// One whole bud of its bearer's base scale at `at`, at angle cosine `turn`.
    fn whole(&self, at: Along, turn: f64) -> Placing {
        let mut p = Placing {
            e: vec![0.0; self.exponents.len()],
            turn,
            ..Placing::default()
        };
        p.add(1.0, &vec![1.0; 4 + self.exponents.len()], at);
        p
    }

    /// Unit `step` from `start`: its factor, its quantities on it, and its
    /// length as tilted.
    fn unit(
        &mut self,
        (k, m): (usize, usize),
        axis: &Axis,
        step: &Step,
        start: Along,
        t: i64,
    ) -> (f64, Vec<f64>, Along) {
        let width = self.width();
        let species = self.species;
        let state = &species.states[k];
        let (i, size) = (step.i, step.size);
        let l = state.internode * size;
        let years = (m - i + 1) as f64;
        let own = state.form.pipe * size * ripe(years, state.form.ripening);
        let mut run = Running {
            a: 1.0,
            b: vec![0.0; width],
        };
        let mut at = start;
        let mut length = Along::default();
        for &z in ALONG.iter().filter(|&&z| z < state.zones.len()) {
            let zone = &state.zones[z];
            let lateral = self.laterals[k][z].clone();
            // Each place's factor: bare, or a lateral whose subtree has
            // stopped living by t.
            let mut tables = Vec::new();
            let mut phi = 1.0;
            for (j, &p) in lateral.iter().enumerate().filter(|(_, &p)| p > 0.0) {
                let delay = species
                    .lived(j)
                    .map_or(f64::INFINITY, |(q, _)| species.states[q].shedding);
                let table = self.shed((j, m - i, Carried::default(), None), delay);
                let (ca, _) = table.at(t - i as i64);
                phi += p * (ca - 1.0);
                tables.push((j, p, table));
            }
            let places = zone.places() as i32;
            let f = phi.powi(places);
            let (mut g, mut mu, mut nu) = (0.0, [0.0; 3], [0.0; 3]);
            for (n, &pn) in axis.pmfs[z].iter().enumerate() {
                let nf = n as f64;
                let tilt = pn * f.powi(n as i32);
                g += tilt;
                if n > 0 {
                    let one = pn * f.powi(n as i32 - 1);
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
            let mut h = self.own(&tilted, at, l, own);
            h.iter_mut().for_each(|v| *v *= g);
            // One node's place holds the subtree; its other places, and
            // every other node, their factors.
            let replaced = Count::from_moments(nu[0], nu[1], nu[2]);
            let others = phi.powi(places - 1);
            let mut low = 0.0;
            for (j, &p) in lateral.iter().enumerate() {
                let band = (low, low + p);
                low += p;
                let Some((_, _, table)) = tables.iter().find(|x| x.0 == j) else {
                    continue;
                };
                let (_, cb) = table.at(t - i as i64);
                if cb.is_empty() {
                    continue;
                }
                let placing = self.laterals_at(j, (zone, &replaced), (at, l, size), (p, band));
                let mut y = vec![0.0; width];
                placing.place(cb, &mut y, 0);
                for q in 0..width {
                    h[q] += others * y[q];
                }
            }
            run.times(g, &h);
            let zl = Along {
                mean: tilted.length(l).mean,
                square: tilted.length(l).square,
            };
            at = at.then(zl);
            length = length.then(zl);
            for event in &step.sleeping[z] {
                let (fa, fb) = self.woken(m, step.grown, event, t);
                run.times(fa, &fb);
            }
        }
        (run.a, run.b, length)
    }

    /// The quantities of a zone's own phytomers, its node count `n` (as
    /// tilted), from `x`, each `l` long and its pipe `o`.
    fn own(&self, n: &Count, x: Along, l: f64, o: f64) -> Vec<f64> {
        let mut y = vec![0.0; self.width()];
        let [w, a, q] = n.wood(x, l);
        let (n1, n2, n3) = n.moments();
        let o2 = o * o;
        y[WOOD] = w;
        y[super::ALONG] = a;
        y[super::SQUARE] = q;
        y[PARALLEL] = q;
        // Each node's tip stands at x + n l along the path.
        y[PATH] = o2 * (n1 * x.mean + l * (n2 + n1) / 2.0);
        y[MOMENT] = o2 / 2.0
            * (n1 * x.square + x.mean * l * (n2 + n1) + l * l * (2.0 * n3 + 3.0 * n2 + n1) / 6.0);
        y[LEAVES] = n1 * l * o2;
        y[LEAVES_ALONG] = o2 * a;
        for (s, e) in self.exponents.iter().enumerate() {
            y[SECTIONS + s] = n1 * o.powf(*e);
        }
        y
    }

    /// How laterals of PA `j` stand on a zone whose one node is replaced
    /// (`replaced`: the node count's moments so weighted) from `x`, each
    /// node `l` long on a unit of `size`.
    fn laterals_at(
        &self,
        j: usize,
        (zone, replaced): (&crate::species::Zone, &Count),
        (x, l, size): (Along, f64, f64),
        (p, band): (f64, (f64, f64)),
    ) -> Placing {
        let species = self.species;
        let (lived, reach) = species.lived(j).unwrap_or((j, 0.0));
        let lived = &species.states[lived];
        let d = lived.form.dominance;
        let mut placing = Placing {
            e: vec![0.0; self.exponents.len()],
            turn: lived.insertion.cos(),
            ..Placing::default()
        };
        let powers = |share: f64| -> Vec<f64> {
            let base = size * share * reach;
            let mut v: Vec<f64> = (1..=4)
                .map(|p| base.powi(p) * dominance_power(d, f64::from(p)))
                .collect();
            v.extend(
                self.exponents
                    .iter()
                    .map(|&e| base.powf(e) * dominance_power(d, e)),
            );
            v
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
    /// over the unit's chance (`grown`), each independent of the axis.
    fn woken(&mut self, m: usize, grown: f64, event: &Event, t: i64) -> (f64, Vec<f64>) {
        let width = self.width();
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
            return (1.0, vec![0.0; width]);
        };
        if grown <= 0.0 {
            return (1.0, vec![0.0; width]);
        }
        let species = self.species;
        let delay = species
            .lived(*pa)
            .map_or(f64::INFINITY, |(q, _)| species.states[q].shedding);
        let table = self.shed((*pa, m - at, *carried, *first), delay);
        let (ca, cb) = table.at(t - *at as i64);
        let lambda = weight / grown;
        let factor = (lambda * (ca - 1.0)).exp();
        let mut y = vec![0.0; width];
        if !cb.is_empty() {
            let mut placing = Placing {
                e: vec![0.0; self.exponents.len()],
                turn: stand.angle.cos(),
                ..Placing::default()
            };
            let share = stand.shares[0] * unit;
            let mut powers: Vec<f64> = (1..=4).map(|p| share.powi(p)).collect();
            powers.extend(self.exponents.iter().map(|&e| share.powf(e)));
            let x = stand.along[0] / stand.shares[0].max(f64::MIN_POSITIVE);
            let x2 = stand.square / stand.shares[0].max(f64::MIN_POSITIVE);
            placing.add(
                1.0,
                &powers,
                Along {
                    mean: x,
                    square: x2,
                },
            );
            placing.place(cb, &mut y, 0);
        }
        (factor, y.iter().map(|v| lambda * factor * v).collect())
    }
}
