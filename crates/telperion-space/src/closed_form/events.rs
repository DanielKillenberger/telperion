//! One bud's development as the recursion meets it (`closed_form.rs`):
//! its growth units, laterals, relays, continuation and sleeping buds,
//! each weighted by its expectation, and where each stands along the
//! bud's axis (`along.rs`), which the envelope reads (fn-209) and the
//! counts and lengths do not.
use super::along::{dominance, Along, Count};
use crate::dormant::{First, Living, Piece};
use crate::schedule::Carried;
use crate::species::{Species, Zone, ALONG};

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
/// was made: its growth units, laterals, relays and continuation, each
/// weighted by its expectation. A relay carries on the time spent; a unit
/// that ends its age mid-cycle hands the rest of its cycle on (`schedule`).
pub(super) fn events(
    species: &Species,
    laterals: &[Vec<f64>],
    living: &mut Living,
    (k, m, carried): (usize, usize, Carried),
    first: Option<First>,
) -> Vec<Event> {
    let state = &species.states[k];
    let mut out = Vec::new();
    let insertion = |pa: usize| {
        species
            .lived(pa)
            .map_or(0.0, |(q, _)| species.states[q].insertion)
    };
    let spread = |pa: usize| {
        let d = species
            .lived(pa)
            .map_or(0.0, |(q, _)| species.states[q].form.dominance);
        dominance(d)
    };
    // Each zone's node count, and where it starts along a unit one
    // internode a node long, after the zones before it (`ALONG`).
    let counts: Vec<Count> = state.zones.iter().map(|z| Count::of(z.nodes)).collect();
    let mut starts = vec![Along::default(); state.zones.len()];
    let mut whole = Along::default();
    for &z in ALONG.iter().filter(|&&z| z < state.zones.len()) {
        starts[z] = whole;
        whole = whole.then(counts[z].length(1.0));
    }
    let scaled = |a: Along, l: f64| Along {
        mean: a.mean * l,
        square: a.square * l * l,
    };
    // A relay carries on the time its axis has grown, which its abortion
    // hazard counts (host decision 11).
    // The relay of an age that `ended` grows a whole first unit.
    let relay = |out: &mut Vec<Event>,
                 (at, spent, gone, ended): (usize, f64, f64, bool),
                 weight: f64,
                 on: Along| {
        if weight > 0.0 {
            out.push(Event::Bud {
                at,
                pa: k,
                carried: Carried {
                    spent,
                    aged: carried.aged + spent - carried.spent,
                    gone,
                    relay: ended,
                },
                weight,
                size: 1.0,
                first: None,
                unit: 1.0,
                stand: Stand::whole(on, state.insertion),
            });
        }
    };
    // Where a relay stands on a unit from `start`, `length` long: its PA's
    // share of the way along it (`geometry::relay_point`).
    let relay_at = |(start, length): (Along, Along)| start.into(length, state.relay_at);
    // The start of the next unit along the axis, and the last unit grown.
    let mut path = Along::default();
    let mut last = (Along::default(), Along::default());
    let mut survival = 1.0;
    for i in 1..=m {
        let unit = carried.unit(state.lifespan, i);
        let spent = unit.before + unit.share;
        // A woken bud's partial first unit runs its share of the year's
        // risks (`First`); a unit that shares its cycle, its share.
        let partial = first.filter(|_| i == 1);
        let before = survival;
        let survive = partial.map_or(shared(state.viability, unit.share), |f| f.survive);
        // A unit that failed is spent; its relay stands on the unit before.
        relay(
            &mut out,
            (i, spent, 0.0, true),
            survival * (1.0 - survive) * state.relay_failed,
            relay_at(last),
        );
        survival *= survive;
        // The chance it then aborts, its hazard counting the time grown.
        let grown = carried.aged + spent - carried.spent;
        let abortion = match partial {
            Some(f) if !unit.ends && f.survive > 0.0 => 1.0 - f.persist / f.survive,
            Some(_) => 0.0,
            None if !unit.ends => {
                // Over the share of the age's next unit.
                let exposure = carried.exposure(state.lifespan, i);
                1.0 - shared(1.0 - state.abortion_at(grown), exposure)
            }
            None => 0.0,
        };
        // A partial unit counts its phytomers whole, at its share of
        // their length; a woken bud's first unit as before, whole.
        let size = if partial.is_some() { 1.0 } else { unit.share };
        let internode = state.internode * size;
        for (z, (zone, lateral)) in state.zones.iter().zip(laterals).enumerate() {
            let nodes = survival * zone.nodes.mean();
            let start = path.then(scaled(starts[z], internode));
            let wood = counts[z].wood(start, internode).map(|w| survival * w);
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
                    let mut sum = Sum::default();
                    // Place 0's draw also orders the nodes (`grow/unit.rs`).
                    let (n, at) = counts[z].ranked(start, internode, band);
                    sum.add(n, zone.share(0), at);
                    for slot in 1..zone.places() {
                        let (n, at) = counts[z].spread(start, internode, false);
                        sum.add(n * p, zone.share(slot), at);
                    }
                    out.push(Event::Bud {
                        at: i,
                        pa: j,
                        carried: Carried::default(),
                        weight: buds,
                        size: size * shared,
                        first: None,
                        unit: size,
                        stand: sum.stand(insertion(j), spread(j)),
                    });
                }
            }
            let bearer = ((k, carried, m), i, abortion);
            let placed = (counts[z], start, internode);
            sleeping(
                living,
                (zone, z),
                (nodes, size),
                bearer,
                (m, placed),
                &mut out,
            );
        }
        let length = scaled(whole, internode);
        last = (path, length);
        path = path.then(length);
        if !unit.ends {
            match partial {
                None => {
                    relay(
                        &mut out,
                        (i, spent, 0.0, false),
                        survival * abortion * state.relay,
                        relay_at(last),
                    );
                    survival *= 1.0 - abortion;
                }
                Some(f) => {
                    let stopped = before * (f.survive - f.persist);
                    relay(
                        &mut out,
                        (i, spent, 0.0, false),
                        stopped * state.relay,
                        relay_at(last),
                    );
                    survival = before * f.persist;
                }
            }
            continue;
        }
        // It moves on by its continuation, or stops and may relay, in this
        // cycle where the age ended within it.
        let (next, go) = species.successor(k).unwrap_or((k, 0.0));
        let (onward, at) = unit.onward(i);
        if m > at && go > 0.0 {
            out.push(Event::Bud {
                at,
                pa: next,
                carried: onward,
                weight: survival * go,
                size: 1.0,
                first: None,
                unit: 1.0,
                stand: Stand::whole(path, 0.0),
            });
        }
        if m > at {
            // And where it all but stopped, it grows its relay as well.
            let both = living.borderline(k, m.saturating_sub(i), unit.used);
            let ended = survival * ((1.0 - go) * state.relay_ended + both);
            relay(
                &mut out,
                (at, spent, onward.gone, true),
                ended,
                relay_at(last),
            );
        }
        break;
    }
    out
}

/// A survival over a share of a cycle: to the share, itself whole.
fn shared(p: f64, share: f64) -> f64 {
    if share < 1.0 {
        p.powf(share)
    } else {
        p
    }
}

/// The sleeping buds of one zone's `nodes` on a growth unit of the bearer
/// (its PA, units carried, unit, and the chance it aborts after the
/// unit): each wakes `s` years on and grows from cycle i + 1 + s in the
/// stage it has aged into, while its bearer is still carried on. Each
/// stands half an internode below its node (`placed`: the zone's count,
/// start and internode).
fn sleeping(
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
