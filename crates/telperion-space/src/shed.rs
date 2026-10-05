//! Shedding. A lateral axis whose subtree has held no living apex for more
//! than its PA's delay is dropped, with all it bears. How big a kept one is
//! fades with its subtree's living time, each growth unit's time weighted
//! by its presence, so a branch whose last growth a setting is unmaking
//! shrinks away before it is shed; it is dropped once its fade reaches
//! nothing, which for whole presences and delays is the cycle GreenLab
//! drops it in (fn-206).
use crate::species::Species;
use crate::structure::{Axis, Origin};

/// Which axes may stand at `age` while the tree grows, before its
/// presences are known (`grow/relay.rs`): a lateral or relay whose subtree
/// has held no living apex for more than a cycle past its PA's delay goes,
/// with all it bears. Its fade has then reached nothing whatever its
/// presences, so `shed` drops no axis this keeps less than it would; the
/// rest `shed` decides by the fade (Codex on fn-206's port).
pub(crate) fn standing(axes: &[Axis], species: &Species, age: u32) -> Vec<bool> {
    let mut live_until: Vec<u32> = axes
        .iter()
        .map(|a| a.apex_end.unwrap_or(u32::MAX))
        .collect();
    // Parents precede children, so one backward pass carries each subtree's
    // last living cycle up to its root.
    for i in (1..axes.len()).rev() {
        let parent = axes[i].origin.parent().unwrap_or(0);
        live_until[parent] = live_until[parent].max(live_until[i]);
    }
    let mut kept = vec![false; axes.len()];
    for (i, axis) in axes.iter().enumerate() {
        kept[i] = match axis.origin {
            Origin::Seed => true,
            Origin::Continuation { parent } => kept[parent],
            Origin::Lateral { parent, .. } | Origin::Relay { parent, .. } => {
                let delay = species.states[axis.pa].shedding;
                let idle = f64::from(age.saturating_sub(live_until[i]));
                kept[parent] && !(delay.is_finite() && idle > delay + 1.0)
            }
        };
    }
    kept
}

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
        last[pa] = after.map(|a| state.lifespan.ceil() + a);
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

pub(crate) fn shed(mut axes: Vec<Axis>, species: &Species, age: u32) -> Shed {
    let fade = fades(&axes, species, age);
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
    Shed {
        axes: kept,
        index,
        fade,
    }
}

/// Each axis's size factor from shedding: 1 for a living subtree, falling
/// to 0 as its idle time, measured in presence-weighted cycles, passes the
/// delay, but never below its most present living apex. Where every
/// presence is whole it is 1 for every kept axis.
fn fades(axes: &[Axis], species: &Species, age: u32) -> Vec<f64> {
    let delay = |a: &Axis| match a.origin {
        Origin::Lateral { .. } | Origin::Relay { .. } => {
            Some(species.states[a.pa].shedding).filter(|d| d.is_finite())
        }
        _ => None,
    };
    if !axes.iter().any(|a| delay(a).is_some()) {
        return vec![1.0; axes.len()];
    }
    // grown[a][k]: the presence of axis a's first k growth units.
    let grown: Vec<Vec<f64>> = axes
        .iter()
        .map(|a| {
            std::iter::once(0.0)
                .chain(a.units.iter().scan(0.0, |sum, p| {
                    *sum += p;
                    Some(*sum)
                }))
                .collect()
        })
        .collect();
    // living[s]: the last cycle anything in s's subtree lived, each growth
    // unit weighted by its presence relative to s. An axis lives from the
    // time its parent had reached when it was made, plus the cycles from
    // there to its birth: whole, its birth cycle. alive[s]: the presence,
    // relative to s, of the most present apex still living in its subtree.
    let mut living = vec![f64::NEG_INFINITY; axes.len()];
    let mut alive = vec![0.0f64; axes.len()];
    for (i, axis) in axes.iter().enumerate() {
        // From the share of its first cycle gone before it grew.
        let mut lived = axis.sleep + grown[i][axis.units.len()];
        let mut apex = axis.alive;
        let mut at = i;
        loop {
            let link = &axes[at];
            living[at] = living[at].max(f64::from(link.birth) + lived);
            alive[at] = alive[at].max(apex);
            let Some(parent) = link.origin.parent() else {
                break;
            };
            let mut weight = link.vigour;
            if let Origin::Lateral { node, .. } = link.origin {
                weight *= axes[parent].phytomers[node].scale;
            }
            let p = &axes[parent];
            let units = (link.birth.saturating_sub(p.birth) as usize).min(p.units.len());
            // A relay made the cycle its parent died starts a cycle after
            // the parent's last growth unit.
            let gap = f64::from(link.birth - p.birth) - units as f64;
            // The parent's own time to the child's birth: from its share of
            // its first cycle gone, through its units before the child; none
            // where the child carries it on in the cycle it began, whose own
            // share gone already holds the parent's (Codex on fn-206's port).
            let before = if units == 0 {
                0.0
            } else {
                p.sleep + grown[parent][units]
            };
            lived = before + weight * (gap + lived);
            apex *= weight;
            at = parent;
        }
    }
    // A living subtree is never drawn smaller than its living apex; an apex
    // that light grew past whole (`allocation.rs`) keeps it whole.
    axes.iter()
        .zip(living.iter().zip(&alive))
        .map(|(axis, (&until, &apex))| match delay(axis) {
            Some(d) => (d + 1.0 - (f64::from(age) - until)).clamp(apex.min(1.0), 1.0),
            None => 1.0,
        })
        .collect()
}
