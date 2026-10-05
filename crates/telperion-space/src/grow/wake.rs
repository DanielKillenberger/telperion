//! Sleeping buds in the grower (`dormant.rs`): each node's sleeping buds
//! put to sleep, and woken in their cycle.
use super::{bud, Apex, Grower};
use crate::dormant::{self, Woken};
use crate::error::Result;
use crate::lineage::{Key, CONTINUATION};
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
            let Some((pa, spent)) = dormant::aged(self.species, sleeper.pa, sleeper.slept) else {
                continue;
            };
            let Some((carried, bearer)) =
                dormant::carried(&self.axes, &self.draws, &self.successor, &sleeper)
            else {
                continue;
            };
            let mut origin = Origin::Lateral {
                parent: sleeper.parent,
                node: sleeper.node,
                slot: sleeper.slot,
                whorl: sleeper.whorl,
                woken: true,
            };
            let mut key = Key(sleeper.key);
            let mut made = [self.windows.presence(sleeper.lead, sleeper.wood), carried];
            self.woken.push(Woken {
                axis: self.axes.len(),
                bearer,
                next: self.draws[bearer].units.len(),
                cycle,
                share: 1.0 - sleeper.sleep,
            });
            let mut stage = sleeper.pa;
            while stage != pa {
                let at = self.axes.len();
                self.slept(key, stage, cycle - 1, origin, made);
                stage = self.species.states[stage].next.expect("aged along next");
                (origin, key, made) = (
                    Origin::Continuation { parent: at },
                    key.child(CONTINUATION),
                    [1.0; 2],
                );
                self.successor[at] = Some(at + 1);
            }
            self.sprout(key, pa, cycle - 1, origin, made);
            let draws = self.draws.last_mut().unwrap();
            (draws.sleep, draws.aged) = (sleeper.sleep, spent);
            *self.units.last_mut().unwrap() = spent;
            let apex = self.next.pop().expect("sprouted");
            self.advance(
                Apex {
                    units: spent,
                    ..apex
                },
                cycle,
            )?;
        }
        Ok(())
    }

    /// A stage a sleeping bud slept through: an axis of no length.
    fn slept(&mut self, key: Key, pa: usize, cycle: u32, origin: Origin, made: [f64; 2]) {
        self.units.push(self.species.states[pa].lifespan);
        self.successor.push(None);
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
        for slot in 0..zone.buds {
            let place = node_key.child(u64::from(slot));
            let drawn = dormant::draw(zone, place, (at, slot), woods, (cycle, self.age));
            if let Some((wakes, sleeper)) = drawn {
                self.asleep[wakes].push(sleeper);
            }
        }
    }
}
