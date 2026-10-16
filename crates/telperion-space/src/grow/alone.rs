//! The rules a chain's units draw by (fn-209 probe P2), and the buds a
//! chain bears: the laterals it makes and the sleeping buds that wake on
//! it, made, keyed and recorded, never grown. The site's coarse settle
//! hands its fine roots on as these buds (`grow/realized.rs`), and the
//! device's unit tables are read from these rules (`ChainTables::cycle`).
//! The chain grower that grew a fine root alone went with fn-256.
use super::shoot::Rules;
use super::{bud, Grower};
use crate::contract::presence::{q, real, times};
use crate::dormant::{self, Sleeper};
use crate::error::{refuse, Result};
use crate::lineage::Key;
use crate::presence::{Draws, Windows};
use crate::shed::lasting;
use crate::species::{PaState, Species};
use crate::structure::{Axis, Origin};

/// A sleeping bud that woke on a chain grown alone, before its chain is
/// grown: what `woke` needs, and the axis that carried it.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Pending {
    pub sleeper: Sleeper,
    pub aged: (usize, f64, f64),
    pub carried: f64,
    pub cycle: u32,
    pub bearer: usize,
    pub next: usize,
}

/// A bud a chain bears, as its bearer made it: all its own chain grows
/// from. `axis` is the bud's first axis as made (its lineage, PA, birth,
/// origin in its bearer's chain and its vigour as made); a woken bud
/// carries what waking it reads.
#[derive(Debug, Clone)]
pub(crate) struct Bud {
    pub axis: Axis,
    pub draws: Draws,
    pub wake: Option<(Pending, f64)>,
}

/// What every chain of a tree reads: grown once per tree.
pub(crate) fn rules<'a>(species: &'a Species, age: u32, budget: u32) -> Result<Rules<'a>> {
    species.validate()?;
    if age == 0 {
        return refuse("age", "a tree grows at least one cycle");
    }
    Ok(drawn(species, age, budget))
}

/// The rules a tree's units draw by, its light unchecked: the alone
/// rules' laterals, lasting and windows.
pub(crate) fn drawn(species: &Species, age: u32, budget: u32) -> Rules<'_> {
    let laterals: Vec<_> = species.states.iter().map(PaState::laterals).collect();
    let lasting = lasting(species, &laterals);
    Rules {
        species,
        laterals: laterals.into(),
        lasting: lasting.into(),
        certain: super::Certain::Skip,
        windows: Windows::new(species, age),
        age,
        budget,
        grown: 0,
        alone: true,
        hands: None,
        one: false,
    }
}

/// The bud a sleeping bud makes as it wakes (`p`), keeping `kept` of its
/// size.
pub(super) fn woken_bud(rules: &Rules, p: Pending, kept: f64) -> Bud {
    let s = p.sleeper;
    let (first, _) = rules.species.lived(s.pa).expect("aged");
    let origin = Origin::Lateral {
        parent: s.parent,
        node: s.node,
        slot: s.slot,
        whorl: s.whorl,
        woken: true,
    };
    let mut axis = bud_axis(Key(s.key).onto(first), first, p.cycle - 1, origin);
    let [own, carried] = s.made(p.carried, p.aged.2).map(q);
    axis.vigour = real(times(times(own, carried), q(kept)));
    Bud {
        axis,
        draws: Draws::default(),
        wake: Some((p, kept)),
    }
}

fn bud_axis(key: Key, pa: usize, birth: u32, origin: Origin) -> Axis {
    bud(key, pa, birth, origin)
}

impl Grower<'_> {
    /// The sleeping buds that wake in `cycle` on a chain grown alone:
    /// buds it bears, one on every axis that carries their bearer on.
    pub(super) fn borne_wakes(&mut self, asleep: Vec<Sleeper>, cycle: u32) -> Result<()> {
        for sleeper in asleep {
            let slept = f64::from(sleeper.slept) + sleeper.sleep;
            let Some(aged) = dormant::aged(self.rules.species, sleeper.pa, slept) else {
                continue;
            };
            for (carried, bearer) in
                dormant::carried(&self.axes, &self.draws, &self.successor, &sleeper, cycle)
            {
                let next = self.draws[bearer].units.len();
                self.wakes.push(Pending {
                    sleeper,
                    aged,
                    carried,
                    cycle,
                    bearer,
                    next,
                });
            }
        }
        Ok(())
    }
}
