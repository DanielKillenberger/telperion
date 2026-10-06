//! One growth unit of an apex: its zones' nodes, in the order their roles
//! stand along the unit, and the buds of each node, laterals that sprout
//! and sleeping buds put to sleep.
use super::shoot::Shoot;
use super::{Apex, Certain};
use crate::error::{Error, Result};
use crate::lineage::{self, Key, ZONE};
use crate::species::{ALONG, MAX_BUDS, MAX_NODES_PER_ZONE};
use crate::structure::{Origin, Phytomer, Vec3};

impl Shoot<'_, '_> {
    pub(super) fn grow_unit(&mut self, apex: Apex, pa: usize, unit: Key, cycle: u32) -> Result<()> {
        let rules = self.rules;
        let windows = &rules.windows;
        let state = &rules.species.states[pa];
        let mut nodes = std::mem::take(&mut self.node_buds);
        let mut leads = std::mem::take(&mut self.leads);
        let along = ALONG.iter().filter(|&&z| z < state.zones.len());
        for (z, zone) in along.map(|&z| (z, &state.zones[z])) {
            let zone_key = unit.child(ZONE + z as u64);
            lineage::nodes(zone.nodes, zone_key.unit(), MAX_NODES_PER_ZONE, &mut leads);
            let lateral = &rules.laterals[pa][z];
            // A node decides its internode and the buds it is expected to bear;
            // a PA its buds cannot carry takes no part.
            let expected: f64 = lateral
                .iter()
                .enumerate()
                .filter(|&(_, &p)| p > 0.0)
                .map(|(j, p)| p * windows.wood(j, cycle + 1))
                .sum();
            // And the sleeping buds it is expected to bear.
            let mut sleeping = std::mem::take(&mut self.sleeping);
            let expected = expected + windows.sleeping(zone, cycle, &mut sleeping);
            let node_wood = windows.share(state.internode) + zone.buds * expected;
            nodes.clear();
            for (i, &node_lead) in leads.iter().enumerate() {
                let node_key = zone_key.child(i as u64);
                let mut buds = [None; MAX_BUDS as usize];
                for (slot, bud) in buds[..zone.places()].iter_mut().enumerate() {
                    let place_key = node_key.child(slot as u64);
                    let placed = zone.share(slot);
                    *bud = lineage::bud(lateral, place_key.unit())
                        .map(|(pa, lead)| (pa, lead, placed));
                }
                // Ordered by the first place's draw, which every node has, so
                // a place a fraction grows moves no node.
                let order = node_key.child(0).unit();
                nodes.push((order, i as u64, node_lead, buds));
            }
            // Acrotony: the nodes stand in order of their draws, so the
            // youngest lateral PA is on top, bare nodes at the base, and a
            // bud a setting makes or unmakes moves no node.
            nodes.sort_by(|a, b| b.0.total_cmp(&a.0));
            // Counted a zone at a time; the shares of a cycle are summed
            // once they are merged (`Grower::step`).
            self.grown += nodes.len() as u64;
            if rules.grown + self.grown > u64::from(rules.budget) {
                return Err(Error::Budget {
                    limit: rules.budget,
                });
            }
            for &(_, drawn, node_lead, buds) in &nodes {
                let axis = self.axis_mut(apex.axis);
                let node = axis.phytomers.len();
                axis.phytomers.push(Phytomer {
                    cycle,
                    tip: Vec3::default(),
                    heading: Vec3::default(),
                    side: Vec3::default(),
                    radius: 0.0,
                    scale: 1.0,
                    key: zone_key.child(drawn).0,
                    rank: 0.0,
                    size: 1.0,
                    light: 1.0,
                });
                let node_presence = windows.presence(node_lead, node_wood);
                self.draws_mut(apex.axis).nodes.push(node_presence);
                for (slot, bud) in buds[..zone.places()].iter().enumerate() {
                    let Some((lateral_pa, lead, placed)) = *bud else {
                        continue;
                    };
                    // A bud of an age it passes through grows in the next
                    // it lives in, as far as it reaches it, or not at all.
                    let Some((lateral_pa, reach)) = rules.species.lived(lateral_pa) else {
                        continue;
                    };
                    // Nor where it is certain to be shed by the tree's age
                    // and nothing reads it while it lives (fn-210).
                    if self.shed_certain(lateral_pa, cycle, apex.axis) {
                        if rules.certain == Certain::Skip {
                            continue;
                        }
                        let key = zone_key.child(drawn).child(slot as u64);
                        self.marked.push(key.onto(lateral_pa).0);
                    }
                    let origin = Origin::Lateral {
                        parent: apex.axis,
                        node,
                        slot: slot as u8,
                        whorl: zone.buds,
                        woken: false,
                    };
                    let key = zone_key.child(drawn).child(slot as u64);
                    let wood = windows.wood(lateral_pa, cycle + 1);
                    let made = [windows.presence(lead, wood) * placed, reach];
                    self.sprout(key, lateral_pa, cycle, origin, made);
                }
                if sleeping.iter().any(|&wood| wood > 0.0) {
                    let at = (apex.axis, node, self.draws(apex.axis).units.len() - 1);
                    let node_key = zone_key.child(drawn);
                    self.sleep(zone, &sleeping, node_key, at, cycle);
                }
            }
            self.sleeping = sleeping;
        }
        self.node_buds = nodes;
        self.leads = leads;
        Ok(())
    }
}
