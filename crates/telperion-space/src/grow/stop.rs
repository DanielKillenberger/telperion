//! An apex's stop and its move to its next age, and the relay either may
//! leave. Where a stop all but happened, the apex carries on and also
//! grows the relay it would have had, the two at shares that sum, so no
//! stop near its bound fades what the axis carries on (host decision 11).
use super::{stop_stake, Apex, Grower};
use crate::error::Result;
use crate::lineage::{above, below, Key, MOVE, RELAY};
use crate::presence::SPAN;
use crate::species::PaState;
use crate::structure::Origin;

/// Why an apex stopped, which says which relay share takes it over.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum Stop {
    /// It aborted after a growth unit, its age going on.
    Aborted,
    /// Its age ended and it did not move on.
    Ended,
    /// Its growth unit failed.
    Failed,
}

impl Stop {
    /// The share of such a stop a relay takes over (host decision 12).
    fn relay(self, state: &PaState) -> f64 {
        match self {
            Stop::Aborted => state.relay,
            Stop::Ended => state.relay_ended,
            Stop::Failed => state.relay_failed,
        }
    }
}

impl Grower<'_> {
    /// The apex has spent its age in the growth unit keyed `unit`: it
    /// moves on to the next age it grows in, past any it passes through,
    /// by its continuation, or it stops; in this cycle at the rest of it,
    /// `used`, where the age ended within it; its draws keyed `unit`, the
    /// age's end.
    pub(super) fn move_on(&mut self, apex: Apex, unit: Key, cycle: u32, used: f64) -> Result<()> {
        let pa = self.axes[apex.axis].pa;
        let state = &self.species.states[pa];
        let Some((next, go)) = self.species.successor(pa) else {
            return self.stop(
                apex,
                (cycle, used),
                f64::INFINITY,
                unit,
                apex.spent,
                Stop::Ended,
            );
        };
        let u = unit.child(MOVE).unit();
        if u >= go {
            let stopped = above(u, go);
            return self.stop(apex, (cycle, used), stopped, unit, apex.spent, Stop::Ended);
        }
        let wood = self.windows.wood_at(next, cycle, used);
        let made = self
            .windows
            .decided(below(u, go), wood, stop_stake(state.relay_ended));
        let key = Key(self.roots[apex.axis]).onto(next);
        let origin = Origin::Continuation { parent: apex.axis };
        self.successor[apex.axis].push(self.axes.len());
        self.sprout(key, next, cycle, origin, [made, 1.0]);
        self.carry_on(cycle, used)?;
        // An apex that all but stopped also grows the relay it would have
        // had, at the share its continuation lacks (host decision 11).
        let relayed = self.relayed(
            state.relay_ended,
            unit,
            self.windows.wood_at(pa, cycle, used),
        );
        if made < 1.0 && relayed > 0.0 {
            self.units[apex.axis] = apex.spent;
            let made = [1.0 - made, relayed];
            self.relay_bud(apex, (cycle, used), apex.spent, made, (0.0, Stop::Ended))?;
        }
        Ok(())
    }

    /// The relay of the apex that stopped having spent `spent` of its PA,
    /// made by draws of presences `made` and `blend` of the way from its
    /// axis's continuation to its own bud. It carries on the axis's
    /// lineage, its time in its PA and the time the axis has grown, which
    /// its abortion hazard counts, so a relay laid as the continuation
    /// grows what the apex would have (host decision 11). The relay of an
    /// age that `ended` grows a whole first unit (`schedule.rs`).
    fn relay_bud(
        &mut self,
        apex: Apex,
        (cycle, used): (u32, f64),
        spent: f64,
        made: [f64; 2],
        (blend, why): (f64, Stop),
    ) -> Result<()> {
        let (pa, key) = (self.axes[apex.axis].pa, Key(self.axes[apex.axis].lineage));
        let draws = &self.draws[apex.axis];
        let grown = draws.aged + draws.shares.iter().sum::<f64>() + spent - apex.spent;
        let ended_relays = draws.ended_relays + u32::from(why == Stop::Ended);
        // The relay of an age that ended, or of a unit that failed, grows
        // a whole first unit: neither is the apex carried on.
        let ended = why != Stop::Aborted;
        let origin = Origin::Relay {
            parent: apex.axis,
            node: 0,
        };
        self.successor[apex.axis].push(self.axes.len());
        self.sprout(key, pa, cycle, origin, made);
        self.axes.last_mut().unwrap().blend = blend;
        let draws = self.draws.last_mut().unwrap();
        (draws.aged, draws.ended, draws.ended_relays) = (grown, ended, ended_relays);
        self.carry_on(cycle, used)
    }

    /// The presence of the relay a stop in the growth unit keyed `unit`
    /// would have: none where its PA relays none or the draw fails.
    pub(super) fn relayed(&self, relay: f64, unit: Key, wood: f64) -> f64 {
        if relay <= 0.0 {
            return 0.0;
        }
        let u = unit.child(RELAY).unit();
        if u < relay {
            self.windows.presence(below(u, relay), wood)
        } else {
            0.0
        }
    }

    /// The apex has stopped in the growth unit keyed `unit`, `stopped`
    /// log-odds past its last draw, having spent `spent` of its PA, with
    /// `used` of the cycle gone; a relay bud
    /// of its PA may take over. The relay is the axis's continuation: it
    /// carries its lineage and the units spent, so its growth units draw
    /// what the apex's would have, and it stands between the axis's tip
    /// and its PA's `relay_at` along it by the stop's presence
    /// (`geometry.rs`).
    pub(super) fn stop(
        &mut self,
        apex: Apex,
        (cycle, used): (u32, f64),
        stopped: f64,
        unit: Key,
        spent: f64,
        why: Stop,
    ) -> Result<()> {
        self.units[apex.axis] = spent;
        if why == Stop::Ended {
            self.draws[apex.axis].outlived = Some(used);
        }
        let axis = &self.axes[apex.axis];
        let relay = why.relay(&self.species.states[axis.pa]);
        if relay <= 0.0 {
            return Ok(());
        }
        let u = unit.child(RELAY).unit();
        if u < relay {
            let wood = self.windows.wood_at(axis.pa, cycle, used);
            // A stop by abortion or a move is whole: the apex that all
            // but stopped grew its relay as well (host decision 11). A
            // failed unit's relay misses that unit, so it grows in from
            // nothing past the bound.
            let stop = if why == Stop::Failed {
                self.windows.decided(stopped, wood, stop_stake(relay))
            } else {
                1.0
            };
            let made = [stop, self.windows.presence(below(u, relay), wood)];
            // The relay moves over the widest window: its move is a growth
            // unit's length whatever wood it carries.
            let blend = (stopped / SPAN).clamp(0.0, 1.0);
            return self.relay_bud(apex, (cycle, used), spent, made, (blend, why));
        }
        Ok(())
    }
}
