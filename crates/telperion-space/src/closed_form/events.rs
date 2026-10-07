//! One bud's development as the recursion meets it (`closed_form.rs`):
//! its growth units, laterals, relays, continuation and sleeping buds,
//! each weighted by its expectation, and where each stands along the
//! bud's axis (`along.rs`), which the envelope reads (fn-209) and the
//! counts and lengths do not.
use super::along::{dominance, Along, Count};
use super::steps::{first_step, scaled, steps, zones, Relay, Step};
use crate::dormant::{First, Living, Piece};
use crate::schedule::Carried;
use crate::species::{Species, Zone};

/// Where buds of one event stand and what share of a whole place's size
/// each grows at (σ), averaged over them: E[σ^p] for p = 1 to 3, E[σ x]
/// and E[σ² x] for x their distance along the axis, E[σ x²]; their angle
/// to the axis, and the dominance moments of their base scale.
#[derive(Debug, Clone, Copy)]
pub(super) struct Stand {
    pub shares: [f64; 3],
    pub along: [f64; 2],
    pub square: f64,
    pub angle: f64,
    pub dominance: [f64; 3],
}

impl Stand {
    /// A whole bud at `at`, its angle `angle`, its base its bearer's.
    fn whole(at: Along, angle: f64) -> Self {
        Self {
            shares: [1.0; 3],
            along: [at.mean; 2],
            square: at.square,
            angle,
            dominance: [1.0; 3],
        }
    }
}

/// Places summed: (count, share) at a place along, in `Stand`'s moments.
#[derive(Default)]
struct Sum {
    count: f64,
    shares: [f64; 3],
    along: [f64; 2],
    square: f64,
}

impl Sum {
    fn add(&mut self, count: f64, share: f64, at: Along) {
        self.count += count;
        for p in 0..3 {
            self.shares[p] += count * share.powi(p as i32 + 1);
        }
        self.along[0] += count * share * at.mean;
        self.along[1] += count * share * share * at.mean;
        self.square += count * share * at.square;
    }

    fn stand(&self, angle: f64, dominance: [f64; 3]) -> Stand {
        let n = if self.count > 0.0 { self.count } else { 1.0 };
        Stand {
            shares: self.shares.map(|s| s / n),
            along: self.along.map(|s| s / n),
            square: self.square / n,
            angle,
            dominance,
        }
    }
}

/// One step of a bud's development, as the recursion meets it.
#[derive(Clone)]
pub(super) enum Event {
    /// Phytomers of the bud's own PA grown in cycle `at`, each grown at
    /// `size` of a whole unit's length (`schedule.rs`); and their wood's
    /// moments along the axis, [E Σ l, E Σ l x, E Σ l x²].
    Nodes {
        at: usize,
        count: f64,
        size: f64,
        wood: [f64; 3],
    },
    /// A bud of `pa` whose first unit grows in cycle `at + 1`, carrying
    /// `carried`, `weight` of them, born at `size` (a lateral of a partial
    /// unit's node); a woken one with the law of its partial first unit;
    /// `unit` the size of the unit it stands on, and where it stands.
    Bud {
        at: usize,
        pa: usize,
        carried: Carried,
        weight: f64,
        size: f64,
        first: Option<First>,
        unit: f64,
        stand: Stand,
    },
}

/// Every event of a bud of PA `k` carrying `carried`, `m` cycles after it
/// was made, from its steps (`steps.rs`): each unit's failed relay, its
/// zones' nodes, laterals and woken buds, and its abortion's relay or its
/// age's continuation and relay, in that order.
pub(super) fn events(
    species: &Species,
    laterals: &[Vec<f64>],
    living: &mut Living,
    (k, m, carried): (usize, usize, Carried),
    first: Option<First>,
) -> Vec<Event> {
    let steps = steps(species, living, (k, m, carried), first);
    events_of(species, laterals, k, &steps)
}

/// The events of a bud's first unit, and the chance its axis carries on
/// into the rest, the same PA's bud one cycle younger carrying
/// `steps::rest` (fn-209, P1).
pub(super) fn first_events(
    species: &Species,
    laterals: &[Vec<f64>],
    living: &mut Living,
    (k, m, carried): (usize, usize, Carried),
    first: Option<First>,
) -> (Vec<Event>, f64) {
    let Some(step) = first_step(species, living, (k, m, carried), first) else {
        return (Vec::new(), 0.0);
    };
    let on = if step.end.is_none() && m > 1 {
        step.after
    } else {
        0.0
    };
    (events_of(species, laterals, k, &[step]), on)
}

fn events_of(species: &Species, laterals: &[Vec<f64>], k: usize, steps: &[Step]) -> Vec<Event> {
    let state = &species.states[k];
    let counts: Vec<Count> = state.zones.iter().map(|z| Count::of(z.nodes)).collect();
    let (_, whole) = zones(&counts);
    let mut out = Vec::new();
    // Where a relay stands: its PA's share of the way along the unit it
    // stands on (`geometry::relay_point`), or the axis's base.
    let on = |unit: usize| {
        unit.checked_sub(1).map_or(Along::default(), |u| {
            steps[u].start.into(steps[u].length, state.relay_at)
        })
    };
    let relay = |out: &mut Vec<Event>, r: Option<Relay>| {
        if let Some(r) = r {
            out.push(Event::Bud {
                at: r.at,
                pa: k,
                carried: r.carried,
                weight: r.weight,
                size: 1.0,
                first: None,
                unit: 1.0,
                stand: Stand::whole(on(r.on), state.insertion),
            });
        }
    };
    for (n, step) in steps.iter().enumerate() {
        // A failed unit's lived parts, each with its relay standing on it,
        // blended back towards the unit before by its extent (fn-199 R1,
        // rule 2'), its nodes taken at their mean.
        for l in &step.lived {
            let grown = (step.i, l.weight, l.size, step.start);
            unit_events(&mut out, species, (k, laterals), grown, &l.sleeping);
            let length = scaled(whole, state.internode * l.size);
            let extent = lived_extent(state, l.size);
            let (r, back) = (
                state.relay_at,
                state.relay_at + extent * (1.0 - state.relay_at),
            );
            let at = match n.checked_sub(1).map(|u| &steps[u]) {
                Some(before) => before.start.into(before.length, back),
                None => step.start,
            }
            .into(length, extent * r);
            out.push(Event::Bud {
                at: l.relay.at,
                pa: k,
                carried: l.relay.carried,
                weight: l.relay.weight,
                size: 1.0,
                first: None,
                unit: 1.0,
                stand: Stand::whole(at, state.insertion),
            });
        }
        let grown = (step.i, step.grown, step.size, step.start);
        unit_events(&mut out, species, (k, laterals), grown, &step.sleeping);
        relay(&mut out, step.aborted);
        if let Some(end) = step.end {
            if let Some((at, pa, onward, weight)) = end.continuation {
                out.push(Event::Bud {
                    at,
                    pa,
                    carried: onward,
                    weight,
                    size: 1.0,
                    first: None,
                    unit: 1.0,
                    stand: Stand::whole(step.start.then(step.length), 0.0),
                });
            }
            relay(&mut out, end.relay);
        }
    }
    out
}

/// A failed unit's lived part's extent, `size` of its cycle grown: its
/// nodes at their mean times that share, at most one (rule 2',
/// `geometry::stand`).
pub(super) fn lived_extent(state: &crate::species::PaState, size: f64) -> f64 {
    let nodes: f64 = state.zones.iter().map(|z| z.nodes.mean()).sum();
    (nodes * size).min(1.0)
}

/// A unit's events: each zone's nodes, grown `weight` of the time at
/// `size` of a whole unit's length from `start`, the laterals they bear
/// and the sleeping buds that wake on them.
fn unit_events(
    out: &mut Vec<Event>,
    species: &Species,
    (k, laterals): (usize, &[Vec<f64>]),
    (i, weight, size, start): (usize, f64, f64, Along),
    sleeping: &[Vec<Event>],
) {
    let state = &species.states[k];
    let counts: Vec<Count> = state.zones.iter().map(|z| Count::of(z.nodes)).collect();
    let (starts, _) = zones(&counts);
    let internode = state.internode * size;
    for (z, (zone, lateral)) in state.zones.iter().zip(laterals).enumerate() {
        let nodes = weight * zone.nodes.mean();
        let start = start.then(scaled(starts[z], internode));
        let wood = counts[z].wood(start, internode).map(|w| weight * w);
        out.push(Event::Nodes {
            at: i,
            count: nodes,
            size,
            wood,
        });
        // Every bud place grows, the last a fraction's at its share of
        // the size: counted whole, sized by the mean share.
        let (places, shared) = zone_places(zone);
        let mut low = 0.0;
        for (j, &p) in lateral.iter().enumerate() {
            let band = (low, low + p);
            low += p;
            let buds = nodes * places * p;
            if buds > 0.0 {
                let placed = (start, internode);
                let stand = lateral_stand(species, (zone, counts[z]), placed, (j, p, band));
                out.push(Event::Bud {
                    at: i,
                    pa: j,
                    carried: Carried::default(),
                    weight: buds,
                    size: size * shared,
                    first: None,
                    unit: size,
                    stand,
                });
            }
        }
        out.extend(sleeping[z].iter().cloned());
    }
}

/// Where the laterals of PA `j`, drawn at `p` within `band` of place 0's
/// draw, stand on a zone's nodes from `start`, and at what share.
pub(super) fn lateral_stand(
    species: &Species,
    (zone, count): (&Zone, Count),
    (start, internode): (Along, f64),
    (j, p, band): (usize, f64, (f64, f64)),
) -> Stand {
    let mut sum = Sum::default();
    // Place 0's draw also orders the nodes (`grow/unit.rs`).
    let (n, at) = count.ranked(start, internode, band);
    sum.add(n, zone.share(0), at);
    for slot in 1..zone.places() {
        let (n, at) = count.spread(start, internode, false);
        sum.add(n * p, zone.share(slot), at);
    }
    let lived = species.lived(j).map(|(q, _)| &species.states[q]);
    let (angle, d) = lived.map_or((0.0, 0.0), |s| (s.insertion, s.form.dominance));
    sum.stand(angle, dominance(d))
}

/// The sleeping buds of one zone's `nodes` on a growth unit of the bearer
/// (its PA, units carried, unit, and the chance it aborts after the
/// unit): each wakes `s` years on and grows from cycle i + 1 + s in the
/// stage it has aged into, while its bearer is still carried on. Each
/// stands half an internode below its node (`placed`: the zone's count,
/// start and internode).
pub(super) fn sleeping(
    living: &mut Living,
    (zone, z): (&Zone, usize),
    (nodes, size): (f64, f64),
    (bearer, i, abortion): ((usize, Carried, usize), usize, f64),
    (m, placed): (usize, (Count, Along, f64)),
    out: &mut Vec<Event>,
) {
    let (places, shared) = zone_places(zone);
    let (count, start, internode) = placed;
    for (j, &p) in zone.dormant.iter().enumerate() {
        let buds = nodes * places * p;
        if buds <= 0.0 {
            continue;
        }
        let mut sum = Sum::default();
        for slot in 0..zone.places() {
            let (n, at) = count.spread(start, internode, true);
            sum.add(n * p, zone.share(slot), at);
        }
        let species = living.species();
        let lived = species.lived(j).map(|(q, _)| &species.states[q]);
        let (angle, d) = lived.map_or((0.0, 0.0), |s| (s.insertion, s.form.dominance));
        let stand = sum.stand(angle, dominance(d));
        for s in 0..(m - i) as u32 {
            let pieces = living.woken((bearer.0, z), j, s);
            if pieces.is_empty() {
                continue;
            }
            // Its bearer carried on through the cycle it wakes in.
            let carried = living.after(bearer, i, i + 2 + s as usize, abortion);
            // Each stage it may wake in over the year, by the part of the
            // year that wakes it there. It ages its slept years in the
            // stage, and its abortion hazard counts them; its first unit's
            // time is a whole cycle's.
            for &(
                Piece {
                    wakes,
                    pa,
                    slept,
                    reach,
                    ..
                },
                first,
            ) in pieces.iter()
            {
                let woken = Carried {
                    spent: slept,
                    aged: slept,
                    ..Carried::default()
                };
                out.push(Event::Bud {
                    at: i + s as usize,
                    pa,
                    carried: woken,
                    // It reaches its stage at a share of its size, as the
                    // grower grows it; it is counted whole.
                    weight: buds * wakes * carried,
                    size: size * shared * reach,
                    first: Some(first),
                    unit: size * reach,
                    stand,
                });
            }
        }
    }
}

/// A zone's bud places per node, every one grown, and their mean share
/// of a whole place's size.
fn zone_places(zone: &Zone) -> (f64, f64) {
    let places = zone.places() as f64;
    if places > 0.0 {
        (places, zone.buds / places)
    } else {
        (0.0, 0.0)
    }
}
