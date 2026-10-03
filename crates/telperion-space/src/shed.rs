//! Shedding. A lateral axis whose subtree has held no living apex for more
//! than its PA's delay is dropped, with all it bears. Which axes go is
//! decided on whole cycles, as GreenLab does; how big a kept one is fades
//! with its subtree's living time weighted by presence, so a branch whose
//! last growth a setting is unmaking shrinks away before it is shed.
use crate::species::Species;
use crate::structure::{Axis, Origin};

pub(crate) fn shed(mut axes: Vec<Axis>, species: &Species, age: u32) -> Vec<Axis> {
    let fade = fades(&axes, species, age);
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
    let mut index = vec![usize::MAX; axes.len()];
    let mut kept = Vec::with_capacity(axes.len());
    for (i, mut axis) in axes.drain(..).enumerate() {
        let lost = match axis.origin {
            Origin::Seed => false,
            Origin::Continuation { parent } => index[parent] == usize::MAX,
            Origin::Lateral { parent, .. } | Origin::Relay { parent } => {
                let delay = species.states[axis.pa].shedding;
                let idle = age.saturating_sub(live_until[i]);
                index[parent] == usize::MAX || delay.is_some_and(|d| idle > d)
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
            | Origin::Relay { parent } => *parent = index[*parent],
        }
        index[i] = kept.len();
        kept.push(axis);
    }
    kept
}

/// Each axis's size factor from shedding: 1 for a living subtree, falling
/// to 0 as its idle time, measured in presence-weighted cycles, passes the
/// delay. Where every presence is whole it is 1 for every kept axis.
fn fades(axes: &[Axis], species: &Species, age: u32) -> Vec<f64> {
    let delay = |a: &Axis| match a.origin {
        Origin::Lateral { .. } | Origin::Relay { .. } => species.states[a.pa].shedding,
        _ => None,
    };
    if !axes.iter().any(|a| delay(a).is_some()) {
        return vec![1.0; axes.len()];
    }
    // living[s]: the last cycle anything in s's subtree lived, each growth
    // unit weighted by its presence relative to s. An axis lives from the
    // time its parent had reached when it was made: whole, its birth cycle.
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
    let mut living = vec![f64::NEG_INFINITY; axes.len()];
    for (i, axis) in axes.iter().enumerate() {
        let mut lived = grown[i][axis.units.len()];
        let mut at = i;
        loop {
            let link = &axes[at];
            living[at] = living[at].max(f64::from(link.birth) + lived);
            let Some(parent) = link.origin.parent() else {
                break;
            };
            let mut weight = link.vigour;
            if let Origin::Lateral { node, .. } = link.origin {
                weight *= axes[parent].phytomers[node].scale;
            }
            let p = &axes[parent];
            let units = (link.birth.saturating_sub(p.birth) as usize).min(p.units.len());
            lived = grown[parent][units] + weight * lived;
            at = parent;
        }
    }
    axes.iter()
        .zip(&living)
        .map(|(axis, &until)| match delay(axis) {
            Some(d) => (f64::from(d) + 1.0 - (f64::from(age) - until)).clamp(0.0, 1.0),
            None => 1.0,
        })
        .collect()
}
