//! An apex's stop and its move to its next age, and the relay either may
//! leave. Where a stop all but happened, the apex carries on and also
//! grows the relay it would have had, the two at shares that sum, so no
//! stop near its bound fades what the axis carries on (host decision 11).
use super::shoot::Shoot;
use super::{stop_stake, Apex};
use crate::contract::{self, presence};
use crate::error::Result;
use crate::lineage::{threshold, Key, ABORTION, MOVE, RELAY};
use crate::presence::Unit;
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

impl Shoot<'_, '_> {
    /// The apex has spent its age in the growth unit keyed `unit`: it
    /// moves on to the next age it grows in, past any it passes through,
    /// by its continuation, or it stops; in this cycle at the rest of it,
    /// `used`, where the age ended within it; its draws keyed `unit`, the
    /// age's end.
    pub(super) fn move_on(&mut self, apex: Apex, unit: Key, cycle: u32, used: f64) -> Result<()> {
        let (species, windows) = (self.species(), self.windows());
        let pa = self.axis(apex.axis).pa;
        let state = &species.states[pa];
        let Some((next, go)) = species.successor(pa) else {
            return self.stop(
                apex,
                (cycle, used),
                contract::INF,
                unit,
                apex.spent,
                (Stop::Ended, true, 0),
            );
        };
        let d = unit.child(MOVE).draw();
        if !d.under(go) {
            let stopped = presence::over(threshold(go), d.0);
            let why = (Stop::Ended, true, 0);
            return self.stop(apex, (cycle, used), stopped, unit, apex.spent, why);
        }
        // Its presence decides whether it also grows its relay, so it is
        // the contract's (FINE-CONTRACT.md D10).
        let stake = stop_stake(state.relay_ended);
        let made = contract::decide::moved(windows, (next, cycle, used), go, d, stake);
        let key = Key(self.root(apex.axis)).onto(next);
        let origin = Origin::Continuation { parent: apex.axis };
        let at = self.len();
        self.successor_mut(apex.axis).push(at);
        self.sprout(key, next, cycle, origin, [presence::real(made), 1.0]);
        self.carry_on(cycle, used)?;
        // An apex that all but stopped also grows the relay it would have
        // had, at the share its continuation lacks (host decision 11),
        // where its relay draw makes one (D9's integer test).
        let relay = state.relay_ended;
        if made < presence::ONE && relay > 0.0 && unit.child(RELAY).draw().under(relay) {
            let relayed = self.relayed(relay, unit, (pa, cycle, Some(used)));
            *self.units_mut(apex.axis) = apex.spent;
            let made = [1.0 - presence::real(made), presence::real(relayed)];
            self.relay_bud(
                apex,
                (cycle, used),
                apex.spent,
                made,
                (0.0, Stop::Ended, true, 0),
            )?;
        }
        Ok(())
    }

    /// The apex's unit keyed `unit` failed `stopped` log-odds past its
    /// bound, having lived `lived` of its cycle from `gone`, and its relay
    /// draw makes one, of presence `relayed` (fn-199 R1, host 2026-10-07):
    /// the unit grows the share it lived, at the relay's presence, and
    /// the relay grows the rest of the cycle, whole, so a draw at the
    /// bound grows what an apex that all but failed does.
    pub(super) fn fail(
        &mut self,
        mut apex: Apex,
        (cycle, gone, whole): (u32, f64, bool),
        (lived, below, relayed, stopped): (f64, u64, f32, f32),
        (pa, unit, decide): (usize, Key, Key),
        after: std::result::Result<f64, f32>,
    ) -> Result<()> {
        let survive = presence::times(relayed, presence::q(self.kept(apex.axis)));
        let (size, light, reads) = (self.size(apex.axis), self.light(apex.axis), self.reads);
        let draws = self.draws_mut(apex.axis);
        draws.units.push(Unit {
            survive: presence::real(survive),
            persist: after.unwrap_or(1.0),
            share: lived,
            size: lived,
        });
        if reads.sketching {
            draws
                .shade
                .push([size, if reads.lights { light } else { 1.0 }]);
        }
        draws.failed = true;
        self.grow_unit(apex, pa, unit, (cycle, None))?;
        apex.spent += lived;
        self.axis_mut(apex.axis).apex_end = Some(cycle);
        *self.units_mut(apex.axis) = apex.spent;
        let blend = presence::real(presence::of(stopped, SPAN as f32));
        let used = (cycle, gone + lived);
        let made = (blend, Stop::Failed, whole, below);
        let Err(lead) = after else {
            return self.relay_bud(apex, used, apex.spent, [1.0; 2], made);
        };
        // The lived part aborted (fn-199, host decision C1): its relay
        // stops at once, as an apex that aborted, and the abortion's relay
        // stands on it, so on the failed axis (`geometry::carried_point`).
        self.make_relay((apex, used, apex.spent, [1.0; 2]), made);
        let relay = self.next.pop().expect("sprouted");
        self.axis_mut(relay.axis).apex_end = Some(cycle);
        let why = (Stop::Aborted, whole, below);
        self.stop(relay, used, lead, decide, relay.spent, why)
    }

    /// The abortion draw keyed `decide` after a unit of PA `pa` in `cycle`
    /// with threshold `aborts`: the lead past it where the apex aborts,
    /// else its persistence, the relay it would have had included, as a
    /// presence (host decision 11).
    pub(super) fn persists(
        &self,
        decide: Key,
        (pa, cycle): (usize, u32),
        aborts: u64,
    ) -> std::result::Result<f64, f32> {
        if aborts == 0 {
            return Ok(1.0);
        }
        let d = decide.child(ABORTION).draw();
        if d.0 < aborts {
            return Err(presence::under(aborts, d.0));
        }
        // An apex that all but aborted carries on as itself and as the
        // relay it would have had, which at the bound is laid as its
        // continuation and draws what it would: the two sum, so no stop
        // near its bound fades the stem (host decision 11), each a
        // presence (host decision 8).
        let state = &self.species().states[pa];
        let at = (pa, cycle + 1, None);
        let window = self.windows().window(at, stop_stake(state.relay));
        let faded = presence::of_over(aborts, d.0, window);
        let relayed = self.relayed(state.relay, decide, at);
        Ok(presence::real(
            relayed + presence::times(presence::ONE - relayed, faded),
        ))
    }

    /// The relay of the apex that stopped having spent `spent` of its PA,
    /// made by draws of presences `made` and `blend` of the way from its
    /// axis's continuation to its own bud. It carries on the axis's
    /// lineage, its time in its PA and the time the axis has grown, which
    /// its abortion hazard counts, so a relay laid as the continuation
    /// grows what the apex would have (host decision 11). A relay made
    /// `whole` grows a whole first unit (`schedule.rs`).
    fn relay_bud(
        &mut self,
        apex: Apex,
        (cycle, used): (u32, f64),
        spent: f64,
        made: [f64; 2],
        (blend, why, whole, below): (f64, Stop, bool, u64),
    ) -> Result<()> {
        let at = (apex, (cycle, used), spent, made);
        self.make_relay(at, (blend, why, whole, below));
        self.carry_on(cycle, used)
    }

    /// `relay_bud`'s relay, made and not yet grown: the next apex.
    fn make_relay(
        &mut self,
        (apex, (cycle, used), spent, made): (Apex, (u32, f64), f64, [f64; 2]),
        (blend, why, whole, below): (f64, Stop, bool, u64),
    ) {
        let (pa, key) = (self.axis(apex.axis).pa, Key(self.axis(apex.axis).lineage));
        let draws = self.draws(apex.axis);
        let grown = draws.aged + draws.grown() + spent - apex.spent;
        let ended_relays = draws.ended_relays + u32::from(why == Stop::Ended);
        // The relay of an age that ended grows a whole first unit: it is
        // not the apex carried on. A failed unit's relay carries the
        // apex's time on from the moment it failed (fn-199 R1), the rest
        // of a whole unit whole, and an aborted one's grows what the apex
        // would have.
        let origin = Origin::Relay {
            parent: apex.axis,
            node: 0,
        };
        let at = self.len();
        self.successor_mut(apex.axis).push(at);
        self.sprout(key, pa, cycle, origin, made);
        let line = self.made.last_mut().unwrap();
        line.axis.blend = blend;
        let draws = &mut line.draws;
        (draws.aged, draws.ended, draws.ended_relays) = (grown, whole, ended_relays);
        // A relay that grows in the cycle its axis stopped in, other than
        // an ended age's, grows the rest of a failed unit (fn-199 R1, C1);
        // one whose unit lived it whole grows on from the next, as the
        // apex would have.
        let rest = why != Stop::Ended && used < 1.0;
        (draws.rest, draws.below) = (rest, if rest { below } else { 0 });
    }

    /// The presence of the relay a stop in the growth unit keyed `unit`
    /// would have (host decision 8): none where its PA relays none
    /// or the draw fails.
    pub(super) fn relayed(&self, relay: f64, unit: Key, at: (usize, u32, Option<f64>)) -> f32 {
        if relay <= 0.0 {
            return 0.0;
        }
        let d = unit.child(RELAY).draw();
        if d.under(relay) {
            presence::of_under(threshold(relay), d.0, self.windows().window(at, 1.0))
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
        stopped: f32,
        unit: Key,
        spent: f64,
        (why, whole, below): (Stop, bool, u64),
    ) -> Result<()> {
        *self.units_mut(apex.axis) = spent;
        if why == Stop::Ended {
            self.draws_mut(apex.axis).outlived = Some(used);
        }
        let windows = self.windows();
        let axis = self.axis(apex.axis);
        let relay = why.relay(&self.species().states[axis.pa]);
        if relay <= 0.0 {
            return Ok(());
        }
        let d = unit.child(RELAY).draw();
        if d.under(relay) {
            let at = (axis.pa, cycle, Some(used));
            // A stop is whole: the apex that all but stopped grew its relay
            // as well (host decision 11), and a failed unit's relay is
            // grown by `fail`, never here. A presence (host decision 8).
            debug_assert!(why != Stop::Failed, "a failed unit's relay grows in `fail`");
            let made = presence::of_under(threshold(relay), d.0, windows.window(at, 1.0));
            let made = [presence::ONE, made].map(presence::real);
            // The relay moves over the widest window: its move is a growth
            // unit's length whatever wood it carries.
            let blend = presence::real(presence::of(stopped, SPAN as f32));
            return self.relay_bud(
                apex,
                (cycle, used),
                spent,
                made,
                (blend, why, whole || why == Stop::Ended, below),
            );
        }
        Ok(())
    }
}
