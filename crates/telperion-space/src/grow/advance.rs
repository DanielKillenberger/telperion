//! One living apex's growth unit (`grow.rs`), on its share of the cycle
//! (`shoot.rs`).
use super::shoot::Shoot;
use super::stop::Stop;
use super::{shared, stop_stake, Apex};
use crate::error::Result;
use crate::lineage::{above, below, Key, ABORTION, END, ENDED, VIABILITY};
use crate::schedule::Carried;

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
        let step = carried.unit(state.lifespan, 1);
        let share = step.share;
        // Shade raises the apex's death hazard, ln survival = ln
        // viability x light^-phi (host, 2026-10-05): near-certain survival
        // moves gently as light does, and certain survival not at all.
        let light = self.light(apex.axis);
        let viability = if state.shade_hazard > 0.0 && light < 1.0 {
            state.viability.powf(light.powf(-state.shade_hazard))
        } else {
            state.viability
        };
        let viability = shared(viability, share);
        let u = decide.child(VIABILITY).unit();
        if u >= viability {
            self.axis_mut(apex.axis).apex_end = Some(cycle - 1);
            self.draws_mut(apex.axis).failed = true;
            // The unit that failed is spent: a relay's first unit draws
            // anew.
            let spent = apex.spent + share;
            self.stop(
                apex,
                (cycle, 1.0),
                above(u, viability),
                decide,
                spent,
                Stop::Failed,
            )?;
            return Ok(());
        }
        let survive = below(u, viability);
        let wood = self.windows().wood(pa, cycle);
        let survive = self
            .windows()
            .decided(survive, wood, stop_stake(state.relay_failed));
        // And its subtrees' survival of shedding on their balance.
        let survive = survive * self.kept(apex.axis);
        let (size, reads) = (self.size(apex.axis), self.reads);
        let draws = self.draws_mut(apex.axis);
        draws.units.push([survive, 1.0]);
        draws.shares.push(share);
        if reads.sketching {
            draws.sizes.push(size);
            if reads.lights {
                draws.lights.push(light);
            }
        }
        self.grow_unit(apex, pa, unit, cycle)?;
        apex.spent += share;
        // An apex that has spent its PA's lifespan moves on; it does not
        // also abort. Its hazard counts the time its axis has grown, and
        // runs over the share this unit and the next leave it exposed
        // (`schedule.rs`).
        let draws = self.draws(apex.axis);
        let grown = draws.aged + draws.shares.iter().sum::<f64>();
        let exposure = carried.exposure(state.lifespan, 1);
        let abortion = 1.0 - shared(1.0 - state.abortion_at(grown), exposure);
        if abortion > 0.0 && !step.ends {
            let u = decide.child(ABORTION).unit();
            if u < abortion {
                self.axis_mut(apex.axis).apex_end = Some(cycle);
                self.stop(
                    apex,
                    (cycle, 1.0),
                    below(u, abortion),
                    decide,
                    apex.spent,
                    Stop::Aborted,
                )?;
                return Ok(());
            }
            // An apex that all but aborted carries on as itself and as the
            // relay it would have had, which at the bound is laid as its
            // continuation and draws what it would: the two sum, so no
            // stop near its bound fades the stem (host decision 11).
            let wood = self.windows().wood(pa, cycle + 1);
            let faded = self
                .windows()
                .decided(above(u, abortion), wood, stop_stake(state.relay));
            let relayed = self.relayed(state.relay, decide, wood);
            let units = &mut self.draws_mut(apex.axis).units;
            units.last_mut().unwrap()[1] = relayed + (1.0 - relayed) * faded;
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
