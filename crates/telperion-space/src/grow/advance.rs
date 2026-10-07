//! One living apex's growth unit (`grow.rs`), on its share of the cycle
//! (`shoot.rs`).
use super::shoot::Shoot;
use super::stop::Stop;
use super::{stop_stake, Apex};
use crate::error::Result;
use crate::fixed::{self, presence};
use crate::lineage::{Key, ABORTION, END, ENDED, FAILED, RELAY, VIABILITY};
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
        };
        let step = carried.unit(state.life(), 1);
        let share = step.share;
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
        let survives = fixed::decide::survival(viability, share);
        let d = decide.child(VIABILITY).draw();
        // A failure's relay, where its draw makes one (D9's integer test),
        // and its presence over the unit's cycle (host decision 8).
        let relay = state.relay_failed;
        let relays = relay > 0.0 && decide.child(RELAY).draw().under(relay);
        let relayed = || self.relayed(relay, decide, (pa, cycle, Some(step.used)));
        if d.0 >= survives && relays {
            let relayed = relayed();
            let lived = fixed::real(fixed::decide::lived(viability, share, d) << 12);
            let stopped = presence::over(survives, d.0);
            let pa_unit = (pa, unit);
            return self.fail(
                apex,
                (cycle, carried.gone),
                (lived, relayed, stopped),
                pa_unit,
            );
        }
        if d.0 >= survives {
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
                Stop::Failed,
            )?;
            return Ok(());
        }
        // Its presence on its survival's own threshold, Q32 (host decision
        // 8), and its subtrees' survival of shedding on their balance.
        let stake = stop_stake(state.relay_failed);
        let window = self.windows().window_q((pa, cycle, None), stake);
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
        let survive = fixed::real(survive);
        let (size, reads) = (self.size(apex.axis), self.reads);
        let draws = self.draws_mut(apex.axis);
        draws.units.push(Unit {
            survive,
            persist: 1.0,
            share,
        });
        if reads.sketching {
            draws
                .shade
                .push([size, if reads.lights { light } else { 1.0 }]);
        }
        self.grow_unit(apex, pa, unit, cycle)?;
        apex.spent += share;
        // An apex that has spent its PA's lifespan moves on; it does not
        // also abort. Its hazard counts the time its axis has grown, and
        // runs over the share this unit and the next leave it exposed
        // (`schedule.rs`).
        let draws = self.draws(apex.axis);
        let grown = draws.aged + draws.grown();
        let exposure = carried.exposure(state.life(), 1);
        let aborts = fixed::decide::abortion(state, grown, exposure);
        if aborts > 0 && !step.ends {
            let d = decide.child(ABORTION).draw();
            if d.0 < aborts {
                self.axis_mut(apex.axis).apex_end = Some(cycle);
                self.stop(
                    apex,
                    (cycle, 1.0),
                    presence::under(aborts, d.0),
                    decide,
                    apex.spent,
                    Stop::Aborted,
                )?;
                return Ok(());
            }
            // An apex that all but aborted carries on as itself and as the
            // relay it would have had, which at the bound is laid as its
            // continuation and draws what it would: the two sum, so no
            // stop near its bound fades the stem (host decision 11), each
            // a Q32 presence (host decision 8).
            let at = (pa, cycle + 1, None);
            let window = self.windows().window_q(at, stop_stake(state.relay));
            let faded = presence::of_over(aborts, d.0, window);
            let relayed = self.relayed(state.relay, decide, at);
            let persist = relayed + presence::times(presence::ONE - relayed, faded);
            let units = &mut self.draws_mut(apex.axis).units;
            units.last_mut().unwrap().persist = fixed::real(persist);
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
