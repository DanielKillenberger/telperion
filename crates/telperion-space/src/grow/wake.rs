//! Sleeping buds in the grower (`dormant.rs`): each node's sleeping buds
//! put to sleep, and woken in their cycle.
use super::shoot::Shoot;
use super::{Apex, Grower};
use crate::dormant::{self, Sleeper, Woken};
use crate::error::Result;
use crate::lineage::Key;
use crate::presence::Draws;
use crate::species::Zone;
use crate::structure::Origin;

impl Grower<'_> {
    /// The sleeping buds that wake in `cycle` sprout and grow their
    /// partial first unit, each on an axis something still carries on
    /// past the cycle, by the presence of every draw that kept it alive. A bud that slept through a stage wakes in the
    /// next, carried there by axes of no length, as its continuations; one
    /// that slept past its last stage never wakes.
    pub(super) fn wake(&mut self, cycle: u32) -> Result<()> {
        let Some(asleep) = self.asleep.get_mut(cycle as usize) else {
            return Ok(());
        };
        let asleep = std::mem::take(asleep);
        if asleep.is_empty() {
            return Ok(());
        }
        // The buds that wake make axes of their own and touch none the
        // tree has: one share, numbered from the tree's count.
        let mut shoot = Shoot::new(&self.rules, self.reads(), (&self.roots, &self.horizon));
        for sleeper in asleep {
            // It ages through the whole of the time it slept, the share of
            // its waking cycle included, so it never wakes in a stage it
            // has outlived.
            let slept = f64::from(sleeper.slept) + sleeper.sleep;
            let Some((pa, spent, reach)) = dormant::aged(self.rules.species, sleeper.pa, slept)
            else {
                continue;
            };
            // It wakes on every axis that carries its bearer on, each at
            // the presence that carries it there.
            for (carried, bearer) in
                dormant::carried(&self.axes, &self.draws, &self.successor, &sleeper, cycle)
            {
                let next = self.draws[bearer].units.len();
                shoot.woke(&sleeper, (pa, spent, reach), carried, (bearer, next), cycle)?;
            }
        }
        let share = shoot.finish(Vec::new());
        self.absorb(share);
        Ok(())
    }
}

impl Shoot<'_, '_> {
    /// The sleeping bud `sleeper` wakes on `bearer`, aged to `pa` with
    /// `spent` of it and `reach` of it reached, carried there by `carried`;
    /// `next` is the bearer's next growth unit.
    fn woke(
        &mut self,
        sleeper: &Sleeper,
        (pa, spent, reach): (usize, f64, f64),
        carried: f64,
        (bearer, next): (usize, usize),
        cycle: u32,
    ) -> Result<()> {
        let mut origin = Origin::Lateral {
            parent: sleeper.parent,
            node: sleeper.node,
            slot: sleeper.slot,
            whorl: sleeper.whorl,
            woken: true,
        };
        let mut key = Key(sleeper.key);
        let lead = self.windows().presence(sleeper.lead, sleeper.wood) * sleeper.placed;
        let mut made = [lead, carried * reach];
        self.woken.push(Woken {
            axis: self.len(),
            bearer,
            next,
            cycle,
            share: 1.0 - sleeper.sleep,
        });
        // The stages it slept through, past any it passes through,
        // each keyed by its place on the reference axis.
        let root = Key(sleeper.key);
        let species = self.species();
        let (mut stage, _) = species.lived(sleeper.pa).expect("aged");
        while stage != pa {
            let at = self.len();
            // The bud's own first stage, keyed as `sprout` keys a bud.
            let lineage = match origin {
                Origin::Lateral { .. } => root.onto(stage),
                _ => key,
            };
            self.slept(lineage, root, stage, (cycle - 1, origin), made);
            (stage, _) = species.successor(stage).expect("aged along the axis");
            (origin, key, made) = (
                Origin::Continuation { parent: at },
                root.onto(stage),
                [1.0; 2],
            );
            self.successor_mut(at).push(at + 1);
        }
        self.sprout(key, pa, cycle - 1, origin, made);
        // Its time in its stage holds the share of the cycle it slept,
        // which its first unit lacks (`schedule.rs`).
        let line = self.made.last_mut().unwrap();
        (line.draws.sleep, line.draws.aged) = (sleeper.sleep, spent);
        line.units = spent;
        let apex = self.next.pop().expect("sprouted");
        self.advance(Apex { spent, ..apex }, cycle)?;
        Ok(())
    }

    /// A stage a sleeping bud slept through: an axis of no length.
    fn slept(
        &mut self,
        key: Key,
        root: Key,
        pa: usize,
        (cycle, origin): (u32, Origin),
        made: [f64; 2],
    ) {
        let draws = Draws {
            birth: made,
            ..Draws::default()
        };
        let lifespan = self.species().states[pa].lifespan;
        self.push(key, pa, cycle, origin, (lifespan, root.0, draws));
        self.made.last_mut().unwrap().axis.apex_end = Some(cycle);
    }

    /// The sleeping buds of the node keyed `node_key`, at `at` (its axis,
    /// node and growth unit), that wake within the tree's age.
    pub(super) fn sleep(
        &mut self,
        zone: &Zone,
        woods: &[f64],
        node_key: Key,
        at: (usize, usize, usize),
        cycle: u32,
    ) {
        for slot in 0..zone.places() {
            let place = node_key.child(slot as u64);
            let placed = zone.share(slot);
            let age = self.rules.age;
            let drawn = dormant::draw(zone, place, (at, slot as u8), woods, (cycle, age));
            let drawn = drawn.map(|(wakes, sleeper)| (wakes, Sleeper { placed, ..sleeper }));
            if let Some((wakes, sleeper)) = drawn {
                self.asleep.push((wakes, sleeper));
            }
        }
    }
}
