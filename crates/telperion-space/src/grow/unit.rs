//! One growth unit of an apex: its zones' nodes, in the order their roles
//! stand along the unit, and the buds of each node, laterals that sprout
//! and sleeping buds put to sleep.
use super::shoot::{Rules, Shoot};
use super::{Apex, Certain};
use crate::error::{Error, Result};
use crate::fixed::{self, presence};
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
            let lateral = &rules.laterals[pa][z];
            let mut sleeping = std::mem::take(&mut self.sleeping);
            let node_wood = node_wood(rules, (pa, z), cycle, &mut sleeping);
            // Each node's presence, Q32 (D18 reads it).
            let window = fixed::decide::node_window(node_wood);
            let draw = zone_key.draw();
            lineage::node_presences(zone.nodes, draw, MAX_NODES_PER_ZONE, window, &mut leads);
            nodes.clear();
            for (i, &node_presence) in leads.iter().enumerate() {
                let node_key = zone_key.child(i as u64);
                let mut buds = [None; MAX_BUDS as usize];
                for (slot, bud) in buds[..zone.places()].iter_mut().enumerate() {
                    let place_key = node_key.child(slot as u64);
                    let d = place_key.draw();
                    let stands = lineage::placed(place_key, zone.share(slot));
                    *bud = stands.then(|| lineage::bud(lateral, d)).flatten().map(|pa| (pa, d));
                }
                // Ordered by the first place's draw, which every node has, so
                // a place a fraction grows moves no node; by its bits, and
                // equal bits by the node's index (FINE-CONTRACT.md D4).
                let order = node_key.child(0).draw();
                nodes.push((order, i as u64, node_presence, buds));
            }
            // Acrotony: the nodes stand in order of their draws, so the
            // youngest lateral PA is on top, bare nodes at the base, and a
            // bud a setting makes or unmakes moves no node.
            nodes.sort_by_key(|n| std::cmp::Reverse(n.0));
            // Counted a zone at a time; the shares of a cycle are summed
            // once they are merged (`Grower::step`).
            self.grown += nodes.len() as u64;
            if rules.grown + self.grown > u64::from(rules.budget) {
                return Err(Error::Budget {
                    limit: rules.budget,
                });
            }
            for &(_, drawn, node_presence, buds) in &nodes {
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
                    grown: 0.0,
                });
                let node_presence = fixed::real(node_presence);
                self.draws_mut(apex.axis).nodes.push(node_presence);
                for (slot, bud) in buds[..zone.places()].iter().enumerate() {
                    let Some((drawn_pa, d)) = *bud else {
                        continue;
                    };
                    // A bud of an age it passes through grows in the next
                    // it lives in, as far as it reaches it, or not at all.
                    let Some((lateral_pa, reach)) = rules.species.lived(drawn_pa) else {
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
                    // Host decision 8: the made presences, Q32.
                    let window = windows.window_q((lateral_pa, cycle + 1, None), 1.0);
                    let lead = lineage::bud_presence(lateral, drawn_pa, d, window);
                    let placed = lineage::place_presence(key, zone.share(slot), window);
                    let made = [
                        fixed::real(presence::times(lead, placed)),
                        fixed::real(presence::q(reach)),
                    ];
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

/// The wood a node of zone `z` of a unit of `pa` grown in `cycle` decides,
/// as a share of the tree's: its internode and the buds it is expected to
/// bear, a PA its buds cannot carry taking no part, and the sleeping buds
/// it is expected to bear, each PA's into `sleeping`.
pub(crate) fn node_wood(
    rules: &Rules,
    (pa, z): (usize, usize),
    cycle: u32,
    sleeping: &mut Vec<f64>,
) -> f64 {
    let (windows, state) = (&rules.windows, &rules.species.states[pa]);
    let zone = &state.zones[z];
    let expected: f64 = rules.laterals[pa][z]
        .iter()
        .enumerate()
        .filter(|&(_, &p)| p > 0.0)
        .map(|(j, p)| p * windows.wood(j, cycle + 1))
        .sum();
    let expected = expected + windows.sleeping(zone, cycle, sleeping);
    windows.share(state.internode) + zone.buds * expected
}
