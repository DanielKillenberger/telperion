//! Shedding. A lateral axis whose subtree has held no living apex for more
//! than its PA's delay is dropped, with all it bears. How big a kept one is
//! fades with its subtree's living time, each growth unit's time weighted
//! by its presence, so a branch whose last growth a setting is unmaking
//! shrinks away before it is shed; it is dropped once its fade reaches
//! nothing, which for whole presences and delays is the cycle GreenLab
//! drops it in (fn-206).
use crate::contract::presence::{carry, fade, q, real, times};
use crate::species::Species;
use crate::structure::{Axis, Origin};

/// Per PA, the cycles past a fresh bud's birth by which every apex its
/// subtree can grow has stopped, from the settings alone; none where no
/// bound holds under every draw (fn-210, lever 1). An apex grows its age's
/// units in its lifespan's whole cycles and hands on at once; its
/// laterals are made by then. Viability, abortion and the balance only
/// shorten life, and an abortion's relay carries the age's time on. Life
/// is unbounded where a lateral bears its own age, where sleeping buds
/// wake, where an age that ends relays (each relay grows a whole unit
/// past it, and may relay again), and where a failed unit relays.
pub(crate) fn lasting(species: &Species, laterals: &[Vec<Vec<f64>>]) -> Vec<Option<f64>> {
    let n = species.states.len();
    let mut last: Vec<Option<f64>> = vec![None; n];
    // A lateral is never younger than its bearer and an age moves on only
    // to older ones, so the oldest are bounded first.
    for pa in (0..n).rev() {
        let state = &species.states[pa];
        let wakes = state
            .zones
            .iter()
            .any(|z| z.rate > 0.0 && z.dormant.iter().any(|&p| p > 0.0));
        let relays = state.relay_ended > 0.0 || (state.relay_failed > 0.0 && state.viability < 1.0);
        if state.lifespan <= 0.0 || wakes || relays {
            continue;
        }
        let mut after = Some(0.0f64);
        let mut bears = |q: Option<usize>| {
            after = match (after, q) {
                (Some(a), Some(q)) if q > pa => last[q].map(|b| a.max(b)),
                (_, Some(_)) => None,
                (a, None) => a,
            };
        };
        if let Some((next, _)) = species.successor(pa) {
            bears(Some(next));
        }
        for zone in &laterals[pa] {
            for (j, _) in zone.iter().enumerate().filter(|&(_, &p)| p > 0.0) {
                bears(species.lived(j).map(|(q, _)| q));
            }
        }
        last[pa] = after.map(|a| state.life().ceil() + a);
    }
    last
}

/// What shedding leaves: the kept axes, each grown axis's index among
/// them (`usize::MAX` where it was shed) and each grown axis's fade.
pub(crate) struct Shed {
    pub axes: Vec<Axis>,
    pub index: Vec<usize>,
    pub fade: Vec<f64>,
}

pub(crate) fn shed(axes: Vec<Axis>, species: &Species, age: u32) -> Shed {
    shed_given(axes, species, age, &|_| None, None).0
}

/// A subtree's living as its bearer's fade reads it (fn-209 F1, realized
/// mechanics): the last time anything in it lived, from its birth, in
/// presence-weighted cycles, and the presence of its most present living
/// apex (host decision 8).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Living {
    pub lived: f32,
    pub alive: f32,
}

/// What stands in for an axis whose subtree is grown elsewhere: its
/// subtree's living, by index (none: the axis's own).
pub(crate) type Given<'g, T> = &'g (dyn Fn(usize) -> Option<T> + Sync);

/// Which axes a pass leaves out, by index.
pub(crate) type Gone<'g> = &'g (dyn Fn(usize) -> bool + Sync);

/// `shed`, each axis `given` names standing for a subtree grown elsewhere
/// whose living it gives; and the living of axis `report`'s subtree.
pub(crate) fn shed_given(
    axes: Vec<Axis>,
    species: &Species,
    age: u32,
    given: Given<Living>,
    report: Option<usize>,
) -> (Shed, Living) {
    shed_past(axes, species, age, (given, &|_| false), report)
}

/// `shed_given`, every axis `gone` names left out of the fades: one inside
/// a subtree retired as it grew, whose root `given` stands for (fn-255).
pub(crate) fn shed_past(
    mut axes: Vec<Axis>,
    species: &Species,
    age: u32,
    (given, gone): (Given<Living>, Gone),
    report: Option<usize>,
) -> (Shed, Living) {
    let (fade, reported) = fades(&axes, species, age, (given, gone), report);
    let mut index = vec![usize::MAX; axes.len()];
    let mut kept = Vec::with_capacity(axes.len());
    for (i, mut axis) in axes.drain(..).enumerate() {
        // Gone with its parent, or once its fade has reached nothing: its
        // subtree idle past its delay by a whole cycle, in the time its
        // growth units lived (fn-206).
        let lost = match axis.origin {
            Origin::Seed => false,
            Origin::Continuation { parent } => index[parent] == usize::MAX,
            Origin::Lateral { parent, .. } | Origin::Relay { parent, .. } => {
                index[parent] == usize::MAX || fade[i] <= 0.0
            }
        };
        if lost {
            continue;
        }
        axis.vigour *= fade[i];
        match &mut axis.origin {
            Origin::Seed => {}
            Origin::Continuation { parent }
            | Origin::Lateral { parent, .. }
            | Origin::Relay { parent, .. } => *parent = index[*parent],
        }
        index[i] = kept.len();
        kept.push(axis);
    }
    let shed = Shed {
        axes: kept,
        index,
        fade,
    };
    (shed, reported)
}

/// Each axis's size factor from shedding: 1 for a living subtree, falling
/// to 0 as its idle time, measured in presence-weighted cycles, passes the
/// delay, but never below its most present living apex. Where every
/// presence is whole it is 1 for every kept axis.
fn fades(
    axes: &[Axis],
    species: &Species,
    age: u32,
    (given, gone): (Given<Living>, Gone),
    report: Option<usize>,
) -> (Vec<f64>, Living) {
    let delay = |a: &Axis| match a.origin {
        Origin::Lateral { .. } | Origin::Relay { .. } => {
            Some(species.states[a.pa].shedding).filter(|d| d.is_finite())
        }
        _ => None,
    };
    if report.is_none() && !axes.iter().any(|a| delay(a).is_some()) {
        return (vec![1.0; axes.len()], Living::default());
    }
    // grown[at[a] + k]: the presence of axis a's first k growth units,
    // summed in order (host decision 8).
    let mut at = Vec::with_capacity(axes.len());
    let mut grown: Vec<f32> = Vec::new();
    for a in axes {
        at.push(grown.len());
        grown.push(0.0);
        let mut sum = 0.0;
        for &p in &a.units {
            sum += q(p);
            grown.push(sum);
        }
    }
    // living[s]: the last cycle anything in s's subtree lived, each growth
    // unit weighted by its presence relative to s. An axis lives from the
    // time its parent had reached when it was made, plus the cycles from
    // there to its birth: whole, its birth cycle. alive[s]: the presence,
    // relative to s, of the most present apex still living in its subtree.
    //
    // Each subtree's is its own, or the most of what its children's carry
    // up: their parent's own time to their birth plus their weight times
    // theirs. That carry never lowers a larger time below a smaller, its
    // weights being no less than nothing, so the most carried up is the
    // carry of the most, and children follow their parents: one backward
    // pass (fn-211). Its maxima are order-free; each sum is taken in the
    // order the axes stand.
    let mut lived: Vec<f32> = axes
        .iter()
        .enumerate()
        .map(|(i, a)| q(a.sleep) + grown[at[i] + a.units.len()])
        .collect();
    let mut alive: Vec<f32> = axes.iter().map(|a| q(a.alive)).collect();
    for (i, (lived, alive)) in lived.iter_mut().zip(&mut alive).enumerate() {
        if let Some(g) = given(i) {
            (*lived, *alive) = (g.lived, g.alive);
        }
    }
    for (i, link) in axes.iter().enumerate().rev() {
        let Some(parent) = link.origin.parent().filter(|_| !gone(i)) else {
            continue;
        };
        let mut weight = q(link.vigour);
        if let Origin::Lateral { node, .. } = link.origin {
            weight = times(weight, q(axes[parent].phytomers[node].scale));
        }
        let p = &axes[parent];
        let units = (link.birth.saturating_sub(p.birth) as usize).min(p.units.len());
        // A relay made the cycle its parent died starts a cycle after
        // the parent's last growth unit.
        let gap = link.birth - p.birth - units as u32;
        // The parent's own time to the child's birth: from its share of
        // its first cycle gone, through its units before the child; none
        // where the child carries it on in the cycle it began, whose own
        // share gone already holds the parent's (Codex on fn-206's port).
        let before = if units == 0 {
            0.0
        } else {
            q(p.sleep) + grown[at[parent] + units]
        };
        let carried = carry(before, weight, gap, lived[i]);
        lived[parent] = lived[parent].max(carried);
        alive[parent] = alive[parent].max(times(alive[i], weight));
    }
    let reported = report.map_or_else(Living::default, |r| Living {
        lived: lived[r],
        alive: alive[r],
    });
    // A living subtree is never drawn smaller than its living apex; an apex
    // that light grew past whole (`allocation.rs`) keeps it whole. Shed
    // where the fade is none (D17).
    let faded = axes
        .iter()
        .zip(lived.iter().zip(&alive))
        .map(|(axis, (&lived, &apex))| match delay(axis) {
            Some(d) => faded(d, axis.birth, Living { lived, alive: apex }, age),
            None => 1.0,
        })
        .collect();
    (faded, reported)
}

/// The fade of a lateral or relay of shedding delay `d`, born in `birth`,
/// whose subtree lived as `living`, at the tree's `age`: shed at none.
pub(crate) fn faded(d: f64, birth: u32, living: Living, age: u32) -> f64 {
    real(fade(q(d + 1.0), birth, (living.lived, living.alive), age))
}

/// The living of the subtree `axes` stands for, axis 0 its root, each
/// axis `given` names standing for a subtree whose living it gives: what
/// the whole tree's fades read of it (fn-255's retirement).
pub(crate) fn subtree_living(
    axes: &[Axis],
    species: &Species,
    age: u32,
    given: Given<Living>,
) -> Living {
    fades(axes, species, age, (given, &|_| false), Some(0)).1
}
