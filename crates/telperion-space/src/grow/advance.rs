//! One living apex's growth unit (`grow.rs`), on its share of the cycle
//! (`shoot.rs`).
use super::shoot::Shoot;
use super::stop::Stop;
use super::{stop_stake, Apex};
use crate::error::Result;
use crate::contract::{self, presence};
use crate::lineage::{Key, END, ENDED, FAILED, RELAY, VIABILITY};
use crate::presence::Unit;
use crate::schedule::Carried;
use crate::transcendental::Transcendental;

impl Shoot<'_, '_> {
    /// One living apex's growth unit in `cycle`: its survival, the unit,
    /// then its abortion or its move to its next PA. A unit that ends its
    /// age within the cycle hands the rest of it on (`schedule.rs`).
    pub fn advance(&mut self, mut apex: Apex, cycle: u32) -> Result<()> {
        let pa = self.axis(apex.axis).pa;
        let state = &self.species().states[pa];
        // A unit's draws are keyed by the cycle it grows in, so a unit that
        // a lifespan's fraction makes or unmakes renumbers none after it
        // (host decision 7).
        let unit = Key(self.axis(apex.axis).lineage).child(u64::from(cycle));
        // A failed unit's relay grows the rest of that unit's cycle, its
        // first unit keyed apart from the failed one, its nodes and its
        // decisions (fn-199 R1); its later units draw what the apex's
        // would have.
        let first = self.draws(apex.axis).units.is_empty();
        let unit = if first && self.draws(apex.axis).rest {
            unit.child(FAILED)
        } else {
            unit
        };
        // Its stop decisions are drawn apart for each relay of an age that
        // ended, the first of which may grow in the cycle its axis stopped
        // in: apart from that axis's, and the same whether the age ended
        // within the cycle or at its start (Codex rounds 3 and 4 on fn-206).
        let decide = match self.draws(apex.axis).ended_relays {
            0 => unit,
            n => unit.child(ENDED).child(u64::from(n)),
        };
        // The share of the cycle gone before the first unit: asleep, or to
        // the age before; its risks run over the unit's own share.
        let draws = self.draws(apex.axis);
        let first = draws.units.is_empty();
        // The relay of an age that ended or a unit that failed grows a
        // whole first unit.
        let relay = draws.ended;
        let carried = Carried {
            spent: apex.spent,
            aged: 0.0,
            gone: if first { draws.sleep } else { 0.0 },
            relay: first && relay,
            rest: first && draws.rest,
        };
        let step = carried.unit(state.life(), 1);
        let share = step.share;
        // A unit a lifespan's fraction cuts, the age's last or its
        // successor's rest of the cycle, lays that share of each zone's
        // nodes, whole (fn-231); every other unit all of them at its share.
        // What carries the axis on where its age ended inside the cycle,
        // its continuation or the relay of an age that ended; not a failed
        // unit's relay, whose rest a draw made.
        let successor = draws.carried && !draws.rest;
        let laid = step.laid(carried.gone, (first && successor, first && draws.woke));
        // Shade raises the apex's death hazard, ln survival = ln
        // viability x light^-phi (host, 2026-10-05): near-certain survival
        // moves gently as light does, and certain survival not at all.
        let light = self.light(apex.axis);
        let viability = if state.shade_hazard > 0.0 && light < 1.0 {
            state
                .viability
                .powf_fixed(light.powf_fixed(-state.shade_hazard))
        } else {
            state.viability
        };
        // Decided on the contract's threshold (FINE-CONTRACT.md D5); the
        // real survival sizes the unit's presence.
        // A failed unit's relay's first unit holds a part of a tick more
        // than its ticks, which its thresholds read (fn-199 B2).
        let below = if first {
            self.draws(apex.axis).below
        } else {
            0
        };
        let survives = contract::decide::survival(viability, share, below);
        let d = decide.child(VIABILITY).draw();
        // A failure's relay, where its draw makes one (D9's integer test),
        // and its presence over the unit's cycle (host decision 8).
        let relay = state.relay_failed;
        let relays = relay > 0.0 && decide.child(RELAY).draw().under(relay);
        let relayed = || self.relayed(relay, decide, (pa, cycle, Some(step.used)));
        // The first unit of a failed unit's relay, the rest of that cycle,
        // carries on where its own failure relays, at that relay's
        // presence, as an apex that all but failed does: relays do not
        // nest within a cycle (fn-199, host decision A3).
        let nested = first && self.draws(apex.axis).rest && relays;
        if d.0 >= survives && relays && !nested {
            let relayed = relayed();
            let lived = contract::decide::lived(viability, share, d) as f64 / contract::CYCLE;
            let below = contract::decide::lived_rest(viability, share, d);
            let stopped = presence::over(survives, d.0);
            // The lived part takes the unit's abortion draw over the share
            // it lived and the next unit's (fn-199, host decision C1).
            let draws = self.draws(apex.axis);
            let grown = draws.aged + draws.grown() + lived;
            let next = carried.unit(state.life(), 2).share;
            let aborts = if step.ends {
                0
            } else {
                contract::decide::abortion(state, grown, (lived.min(next), 0))
            };
            let after = self.persists(decide, (pa, cycle), aborts);
            return self.fail(
                apex,
                (cycle, carried.gone, step.whole),
                (lived, below, relayed, stopped),
                (pa, unit, decide),
                after,
            );
        }
        if d.0 >= survives && !nested {
            self.axis_mut(apex.axis).apex_end = Some(cycle - 1);
            self.draws_mut(apex.axis).failed = true;
            // The unit that failed is spent: a relay's first unit draws
            // anew.
            let spent = apex.spent + share;
            self.stop(
                apex,
                (cycle, 1.0),
                presence::over(survives, d.0),
                decide,
                spent,
                (Stop::Failed, false, 0),
            )?;
            return Ok(());
        }
        // Its presence on its survival's own threshold (host decision
        // 8), and its subtrees' survival of shedding on their balance.
        let stake = stop_stake(state.relay_failed);
        let window = self.windows().window((pa, cycle, None), stake);
        let survive = presence::of_under(survives, d.0, window);
        // An apex that all but failed carries on as itself and as the
        // relay it would have had, as one that all but aborted does (host
        // decision 11; fn-199 R1): the relay's share is whole at the bound.
        let survive = if relays {
            let relayed = relayed();
            relayed + presence::times(presence::ONE - relayed, survive)
        } else {
            survive
        };
        let survive = presence::times(survive, presence::q(self.kept(apex.axis)));
        let survive = presence::real(survive);
        let (size, reads) = (self.size(apex.axis), self.reads);
        let draws = self.draws_mut(apex.axis);
        draws.units.push(Unit {
            survive,
            persist: 1.0,
            share,
            size: laid.map_or(share, |c| c.of),
        });
        if reads.sketching {
            draws
                .shade
                .push([size, if reads.lights { light } else { 1.0 }]);
        }
        self.grow_unit(apex, pa, unit, (cycle, laid))?;
        apex.spent += share;
        // An apex that has spent its PA's lifespan moves on; it does not
        // also abort. Its hazard counts the time its axis has grown, and
        // runs over the share this unit and the next leave it exposed
        // (`schedule.rs`).
        let draws = self.draws(apex.axis);
        let grown = draws.aged + draws.grown();
        let next = carried.unit(state.life(), 2).share;
        let fine = |s: f64| contract::ticks(s) << 12;
        let exposure = if fine(share) + below <= fine(next) {
            (share, below)
        } else {
            (next, 0)
        };
        let aborts = contract::decide::abortion(state, grown, exposure);
        if !step.ends {
            match self.persists(decide, (pa, cycle), aborts) {
                Err(lead) => {
                    self.axis_mut(apex.axis).apex_end = Some(cycle);
                    let why = (Stop::Aborted, false, 0);
                    return self.stop(apex, (cycle, 1.0), lead, decide, apex.spent, why);
                }
                Ok(persist) => {
                    let units = &mut self.draws_mut(apex.axis).units;
                    units.last_mut().unwrap().persist = persist;
                }
            }
        }
        if !step.ends {
            self.next.push(apex);
            return Ok(());
        }
        self.axis_mut(apex.axis).apex_end = Some(cycle);
        let used = if step.same_cycle() { step.used } else { 1.0 };
        // Its end is drawn once for its age, whatever cycle it falls in,
        // so a lifespan crossing a whole cycle moves no draw (Codex round
        // 7 on fn-206); apart for each relay of an age that ended.
        let ends = Key(self.axis(apex.axis).lineage)
            .child(END)
            .child(u64::from(self.draws(apex.axis).ended_relays));
        self.move_on(apex, ends, cycle, used)
    }
}
