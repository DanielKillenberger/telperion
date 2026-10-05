//! One growth unit of an apex: its zones' nodes, each with its internode,
//! its lateral buds and its sleeping buds.
use super::{Apex, Grower};
use crate::error::{Error, Result};
use crate::lineage::{self, Key, ZONE};
use crate::species::{MAX_BUDS, MAX_NODES_PER_ZONE};
use crate::structure::{Origin, Phytomer, Vec3};

impl Grower<'_> {
    pub(super) fn grow_unit(&mut self, apex: Apex, pa: usize, unit: Key, cycle: u32) -> Result<()> {
        let state = &self.species.states[pa];
        let mut nodes = std::mem::take(&mut self.node_buds);
        let mut leads = std::mem::take(&mut self.leads);
        for (z, zone) in state.zones.iter().enumerate() {
            let zone_key = unit.child(ZONE + z as u64);
            lineage::nodes(zone.nodes, zone_key.unit(), MAX_NODES_PER_ZONE, &mut leads);
            let lateral = &self.laterals[pa][z];
            // A node decides its internode and the buds it is expected to bear;
            // a PA its buds cannot carry takes no part.
            let expected: f64 = lateral
                .iter()
                .enumerate()
                .filter(|&(_, &p)| p > 0.0)
                .map(|(j, p)| p * self.windows.wood(j, cycle + 1))
                .sum();
            // And the sleeping buds it is expected to bear.
            let mut sleeping = std::mem::take(&mut self.sleeping);
            let expected = expected + self.windows.sleeping(zone, cycle, &mut sleeping);
            let node_wood = self.windows.share(state.internode) + f64::from(zone.buds) * expected;
            nodes.clear();
            for (i, &node_lead) in leads.iter().enumerate() {
                let node_key = zone_key.child(i as u64);
                let mut buds = [None; MAX_BUDS as usize];
                let mut order = 1.0f64;
                for (slot, bud) in buds[..zone.buds as usize].iter_mut().enumerate() {
                    let u = node_key.child(slot as u64).unit();
                    order = order.min(u);
                    *bud = lineage::bud(lateral, u);
                }
                nodes.push((order, i as u64, node_lead, buds));
            }
            // Acrotony: the nodes stand in order of their draws, so the
            // youngest lateral PA is on top, bare nodes at the base, and a
            // bud a setting makes or unmakes moves no node.
            nodes.sort_by(|a, b| b.0.total_cmp(&a.0));
            for &(_, drawn, node_lead, buds) in &nodes {
                self.grown += 1;
                if self.grown > self.budget {
                    return Err(Error::Budget { limit: self.budget });
                }
                let axis = &mut self.axes[apex.axis];
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
                let node_presence = self.windows.presence(node_lead, node_wood);
                self.draws[apex.axis].nodes.push(node_presence);
                for (slot, bud) in buds[..zone.buds as usize].iter().enumerate() {
                    let Some((lateral_pa, lead)) = *bud else {
                        continue;
                    };
                    let origin = Origin::Lateral {
                        parent: apex.axis,
                        node,
                        slot: slot as u8,
                        whorl: zone.buds,
                        woken: false,
                    };
                    let key = zone_key.child(drawn).child(slot as u64);
                    let wood = self.windows.wood(lateral_pa, cycle + 1);
                    let made = [self.windows.presence(lead, wood), 1.0];
                    self.sprout(key, lateral_pa, cycle, origin, made);
                }
                if sleeping.iter().any(|&wood| wood > 0.0) {
                    let at = (apex.axis, node, self.draws[apex.axis].units.len() - 1);
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
