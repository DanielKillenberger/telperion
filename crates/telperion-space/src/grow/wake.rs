//! Sleeping buds in the grower (`dormant.rs`): each node's sleeping buds
//! put to sleep, and woken in their cycle.
use super::{bud, Apex, Grower};
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
        for sleeper in std::mem::take(asleep) {
            // It ages through the whole of the time it slept, the share of
            // its waking cycle included, so it never wakes in a stage it
            // has outlived.
            let slept = f64::from(sleeper.slept) + sleeper.sleep;
            let Some((pa, spent, reach)) = dormant::aged(self.species, sleeper.pa, slept) else {
                continue;
            };
            // It wakes on every axis that carries its bearer on, each at
            // the presence that carries it there.
            for (carried, bearer) in
                dormant::carried(&self.axes, &self.draws, &self.successor, &sleeper, cycle)
            {
                self.woke(&sleeper, (pa, spent, reach), carried, bearer, cycle)?;
            }
        }
        Ok(())
    }

    /// The sleeping bud `sleeper` wakes on `bearer`, aged to `pa` with
    /// `spent` of it and `reach` of it reached, carried there by `carried`.
    fn woke(
        &mut self,
        sleeper: &Sleeper,
        (pa, spent, reach): (usize, f64, f64),
        carried: f64,
        bearer: usize,
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
        let lead = self.windows.presence(sleeper.lead, sleeper.wood) * sleeper.placed;
        let mut made = [lead, carried * reach];
        self.woken.push(Woken {
            axis: self.axes.len(),
            bearer,
            next: self.draws[bearer].units.len(),
            cycle,
            share: 1.0 - sleeper.sleep,
        });
        // The stages it slept through, past any it passes through,
        // each keyed by its place on the reference axis.
        let root = Key(sleeper.key);
        let (mut stage, _) = self.species.lived(sleeper.pa).expect("aged");
        while stage != pa {
            let at = self.axes.len();
            // The bud's own first stage, keyed as `sprout` keys a bud.
            let lineage = match origin {
                Origin::Lateral { .. } => root.onto(stage),
                _ => key,
            };
            self.slept(lineage, root, stage, (cycle - 1, origin), made);
            (stage, _) = self.species.successor(stage).expect("aged along the axis");
            (origin, key, made) = (
                Origin::Continuation { parent: at },
                root.onto(stage),
                [1.0; 2],
            );
            self.successor[at].push(at + 1);
        }
        self.sprout(key, pa, cycle - 1, origin, made);
        // Its time in its stage holds the share of the cycle it slept,
        // which its first unit lacks (`schedule.rs`).
        let draws = self.draws.last_mut().unwrap();
        (draws.sleep, draws.aged) = (sleeper.sleep, spent);
        *self.units.last_mut().unwrap() = spent;
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
        self.units.push(self.species.states[pa].lifespan);
        self.successor.push(Vec::new());
        self.roots.push(root.0);
        let mut axis = bud(key, pa, cycle, origin);
        axis.apex_end = Some(cycle);
        self.axes.push(axis);
        self.draws.push(Draws {
            birth: made,
            ..Draws::default()
        });
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
            let drawn = dormant::draw(zone, place, (at, slot as u8), woods, (cycle, self.age));
            let drawn = drawn.map(|(wakes, sleeper)| (wakes, Sleeper { placed, ..sleeper }));
            if let Some((wakes, sleeper)) = drawn {
                self.asleep[wakes].push(sleeper);
            }
        }
    }
}
