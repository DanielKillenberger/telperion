//! Sweep dependencies queried through stable identity links, without geometry.
use super::*;
use crate::growth::Age;
use std::collections::BTreeSet;
impl Specimen {
    /// Complete queried sweep paths plus their ancestor and junction context.
    /// Sibling stubs preserve leader choice without projecting their subtrees.
    pub(super) fn contact_wood(
        &self,
        selected: &BTreeSet<NodeIdentity>,
        age: Age,
    ) -> BTreeSet<NodeIdentity> {
        let mut query = Query {
            s: self,
            age,
            stands: Default::default(),
            outgoing: Default::default(),
            leaders: Default::default(),
        };
        let mut ids = selected.clone();
        for &id in selected {
            if query.alive(id) && self.links[id.key].parent.is_some() && query.stand(id) == id {
                ids.extend(query.path(id));
            }
        }
        for id in ids.clone() {
            let mut at = self.links[id.key].parent;
            while let Some(parent) = at {
                if !ids.insert(parent) {
                    break;
                }
                at = self.links[parent.key].parent;
            }
        }
        let mut context = Vec::new();
        for &id in &ids {
            context.extend(&self.links[id.key].children);
        }
        while let Some(id) = context.pop() {
            if query.alive(id) && ids.insert(id) && query.stand(id) != id {
                context.extend(&self.links[id.key].children);
            }
        }
        ids
    }

    pub(super) fn contact_candidates(
        &self,
        changed: &BTreeSet<NodeIdentity>,
        age: Age,
    ) -> BTreeSet<NodeIdentity> {
        let mut query = Query {
            s: self,
            age,
            stands: Default::default(),
            outgoing: Default::default(),
            leaders: Default::default(),
        };
        let mut touched = BTreeSet::new();
        for &id in changed {
            if !query.alive(id) {
                continue;
            }
            touched.insert(id);
            if let Some(parent) = self.links[id.key].parent {
                touched.insert(query.stand(parent));
            }
        }
        let mut out = BTreeSet::new();
        for id in touched {
            // Zero-length forks have no sweep segment of their own.
            if query.stand(id) != id {
                continue;
            }
            if self.links[id.key].parent.is_some() {
                out.extend(query.path(id));
            }
            for child in query.children(id) {
                out.extend(query.path(child));
            }
        }
        out
    }
}
struct Query<'a> {
    s: &'a Specimen,
    age: Age,
    stands: std::collections::BTreeMap<NodeIdentity, NodeIdentity>,
    outgoing: std::collections::BTreeMap<NodeIdentity, Vec<NodeIdentity>>,
    leaders: std::collections::BTreeMap<NodeIdentity, Option<NodeIdentity>>,
}
impl Query<'_> {
    fn alive(&self, id: NodeIdentity) -> bool {
        let n = self.node(id);
        n.shoot.birth_year <= self.age.slice as f64
            && n.shoot.death_year.is_none_or(|year| self.age.slice < year)
    }
    fn stand(&mut self, id: NodeIdentity) -> NodeIdentity {
        if let Some(&stand) = self.stands.get(&id) {
            return stand;
        }
        let stand = self.s.links[id.key].parent.map_or(id, |parent| {
            let stand = self.stand(parent);
            let nodes = &self.s.tree.nodes;
            if nodes[self.s.identities[id.key]]
                .position
                .distance(nodes[self.s.identities[stand.key]].position)
                > 1e-9
            {
                id
            } else {
                stand
            }
        });
        self.stands.insert(id, stand);
        stand
    }
    fn children(&mut self, id: NodeIdentity) -> Vec<NodeIdentity> {
        if let Some(children) = self.outgoing.get(&id) {
            return children.clone();
        }
        let mut pending = self.s.links[id.key].children.clone();
        let mut children = Vec::new();
        while let Some(child) = pending.pop() {
            if !self.alive(child) {
                continue;
            }
            if self.stand(child) == id {
                pending.extend(&self.s.links[child.key].children);
            } else {
                children.push(child);
            }
        }
        // The packed surface breaks equal-radius ties in structural/local,
        // then birth order. Late structural growth must keep that same tie.
        children.sort_unstable_by_key(|child| {
            (
                self.s.tree.nodes[self.s.identities[child.key]].kind != NodeKind::Structural,
                *child,
            )
        });
        self.outgoing.insert(id, children.clone());
        children
    }
    fn node(&self, id: NodeIdentity) -> &Node {
        &self.s.tree.nodes[self.s.identities[id.key]]
    }
    /// The surface's own choice: the straighter stem where stems part above
    /// the root, and the widest child everywhere else.
    fn leader(&mut self, id: NodeIdentity) -> Option<NodeIdentity> {
        if let Some(&leader) = self.leaders.get(&id) {
            return leader;
        }
        let children = self.children(id);
        let leader = self
            .straightest(id, &children)
            .or_else(|| self.widest(&children));
        self.leaders.insert(id, leader);
        leader
    }
    fn straightest(&mut self, id: NodeIdentity, children: &[NodeIdentity]) -> Option<NodeIdentity> {
        let parent = self.s.links[id.key].parent.filter(|_| self.node(id).stem)?;
        let below = self.stand(parent);
        let before = self.node(below).position;
        let stems = children
            .iter()
            .filter(|&&child| self.node(child).stem)
            .map(|&child| (child, self.node(child).position));
        crate::pipeline::surface::straightest(before, self.node(id).position, stems)
    }
    fn widest(&self, children: &[NodeIdentity]) -> Option<NodeIdentity> {
        let mut leader = None;
        let mut radius = f64::NEG_INFINITY;
        for &child in children {
            let current = self.s.keyframes.at(child, self.age.slice).unwrap()[1];
            if current > radius {
                leader = Some(child);
                radius = current;
            }
        }
        leader
    }
    fn path(&mut self, mut edge: NodeIdentity) -> Vec<NodeIdentity> {
        let mut attach = self.stand(self.s.links[edge.key].parent.unwrap());
        while self.leader(attach) == Some(edge) {
            let Some(parent) = self.s.links[attach.key].parent else {
                break;
            };
            edge = attach;
            attach = self.stand(parent);
        }
        let mut out = vec![edge];
        while let Some(next) = self.leader(edge) {
            out.push(next);
            edge = next;
        }
        out
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contacts_carry_on_into_the_stem_the_surface_does() {
        // Where a clump parts above the ground the query follows the
        // straighter stem as the surface does, so every node's dependencies,
        // the fork's and its stems' among them, are the sweep's own.
        let mut f = crate::presets::Preset::OregonWhiteOak.parameters();
        f.skeleton.seed = 7;
        f.skeleton.habit.stems = 2;
        f.skeleton.habit.stem_lean = 24.0;
        f.skeleton.habit.stem_lean_spread = 1.0;
        f.skeleton.habit.stem_fork_height = 0.4;
        f.age = 16.0;
        let s = Specimen::build(&f).unwrap();
        for year in [8, 12, 16] {
            let age = Age::from_years(year as f64).unwrap();
            let (tree, _) = s.wood_at(age, false).unwrap();
            let step = (tree.nodes.len() / 16).max(1);
            let stems = tree.nodes.iter().filter(|n| n.stem);
            for n in stems.chain(tree.nodes.iter().step_by(step)) {
                let changed = BTreeSet::from([n.identity]);
                assert_eq!(
                    s.contact_candidates(&changed, age),
                    crate::pipeline::surface::affected_contacts(&tree, &changed).unwrap(),
                    "year {year} node {:?}",
                    n.identity
                );
            }
        }
    }
    #[test]
    fn indexed_contacts_match_both_endpoint_sweep_dependencies() {
        let (_, mut s, shoot) = super::super::foliage_tests::fixture(1.0);
        let sibling = super::super::foliage_tests::sibling(&mut s);
        super::super::interval::tests::stamp(&mut s, 1);
        s.stamp_deaths(&[sibling], 2);
        super::super::interval::tests::stamp(&mut s, 2);
        for year in [1, 2] {
            let age = Age::from_years(year as f64).unwrap();
            let (tree, _) = s.wood_at(age, false).unwrap();
            let changed = BTreeSet::from([shoot, sibling]);
            assert_eq!(
                s.contact_candidates(&changed, age),
                crate::pipeline::surface::affected_contacts(&tree, &changed).unwrap()
            );
        }
        for preset in [
            crate::presets::Preset::Ordinary,
            crate::presets::Preset::OregonWhiteOak,
            crate::presets::Preset::NorwaySpruce,
            crate::presets::Preset::EuropeanBeech,
            crate::presets::Preset::SilverBirch,
            crate::presets::Preset::Telperion,
            crate::presets::Preset::Laurelin,
        ] {
            let mut family = preset.parameters();
            family.age = 12.0;
            let s = Specimen::build(&family).unwrap();
            for year in [4, 8, 12] {
                let age = Age::from_years(year as f64).unwrap();
                let (tree, _) = s.wood_at(age, false).unwrap();
                for n in tree.nodes.iter().step_by((tree.nodes.len() / 7).max(1)) {
                    let changed = BTreeSet::from([n.identity]);
                    assert_eq!(
                        s.contact_candidates(&changed, age),
                        crate::pipeline::surface::affected_contacts(&tree, &changed).unwrap(),
                        "{preset:?} year {year} node {:?}",
                        n.identity
                    );
                }
            }
        }
    }
}
