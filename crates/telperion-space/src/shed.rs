//! Shedding. A lateral axis whose subtree has held no living apex for more
//! than its PA's delay is dropped, with all it bears. Which axes go is
//! decided on whole cycles, as GreenLab does; how big a kept one is fades
//! with its subtree's living time weighted by presence, so a branch whose
//! last growth a setting is unmaking shrinks away before it is shed.
use crate::species::Species;
use crate::structure::{Axis, Origin};

/// Which axes the shedding rule keeps at `age`: a lateral or relay whose
/// subtree has held no living apex for more than its PA's delay goes,
/// with all it bears.
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
                let idle = age.saturating_sub(live_until[i]);
                kept[parent] && !delay.is_some_and(|d| idle > d)
            }
        };
    }
    kept
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
    let standing = standing(&axes, species, age);
    let mut index = vec![usize::MAX; axes.len()];
    let mut kept = Vec::with_capacity(axes.len());
    for (i, mut axis) in axes.drain(..).enumerate() {
        if !standing[i] {
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
        Origin::Lateral { .. } | Origin::Relay { .. } => species.states[a.pa].shedding,
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
        let mut lived = grown[i][axis.units.len()];
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
            lived = grown[parent][units] + weight * (gap + lived);
            apex *= weight;
            at = parent;
        }
    }
    // A living subtree is never drawn smaller than its living apex; an apex
    // that light grew past whole (`allocation.rs`) keeps it whole.
    axes.iter()
        .zip(living.iter().zip(&alive))
        .map(|(axis, (&until, &apex))| match delay(axis) {
            Some(d) => (f64::from(d) + 1.0 - (f64::from(age) - until)).clamp(apex.min(1.0), 1.0),
            None => 1.0,
        })
        .collect()
}
