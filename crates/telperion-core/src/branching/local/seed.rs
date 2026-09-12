use super::*;
use std::collections::BTreeSet;

#[derive(Clone, Default)]
pub(super) struct Stations {
    parents: Vec<Option<usize>>,
    pub children: Vec<usize>,
    continuation: Vec<Option<usize>>,
    pending: BTreeSet<usize>,
    inserted: Vec<usize>,
}
impl Stations {
    pub(super) fn sync(&mut self, tree: &Tree) {
        self.children.resize(tree.nodes.len(), 0);
        self.continuation.resize(tree.nodes.len(), None);
        let first = self.parents.len();
        self.parents.resize(tree.nodes.len(), None);
        for i in std::mem::take(&mut self.inserted)
            .into_iter()
            .chain(first..tree.nodes.len())
        {
            if tree.nodes[i].kind != NodeKind::Structural {
                continue;
            }
            let parent = tree.nodes[i].parent.map(|p| p as usize);
            self.parents[i] = parent;
            if let Some(p) = parent {
                self.children[p] += 1;
                self.continuation[p].get_or_insert(i);
                self.pending.insert(i);
            }
        }
    }
    // Only a station allocating buds needs a pendant floor. Follow its first
    // descending ancestor, then the retained first-child continuation to its tip.
    fn floor(&self, tree: &Tree, mut i: usize) -> Option<f64> {
        let mut origin = None;
        while let Some(parent) = self.parents[i] {
            if (tree.nodes[i].position - tree.nodes[parent].position)
                .normalized()
                .y
                < -0.5
            {
                origin = Some(i);
            }
            i = parent;
        }
        origin.map(|mut end| {
            while let Some(next) = self.continuation[end] {
                end = next;
            }
            tree.nodes[end].position.y
        })
    }
    pub fn remap(&mut self, map: &[Option<u32>]) {
        self.pending = self
            .pending
            .iter()
            .filter_map(|&i| map[i].map(|v| v as usize))
            .collect();
        for (i, p) in self.parents.iter().enumerate() {
            if map[i].is_none() {
                if let Some(p) = p.and_then(|p| map[p]) {
                    self.pending.insert(p as usize);
                }
            }
        }
        let count = map.iter().flatten().max().map_or(0, |i| *i as usize + 1);
        let mut mapped = vec![false; count];
        for &i in map.iter().flatten() {
            mapped[i as usize] = true;
        }
        self.inserted = mapped
            .iter()
            .enumerate()
            .filter_map(|(i, present)| (!present).then_some(i))
            .collect();
        let mut parents = Vec::with_capacity(self.parents.capacity().max(count));
        parents.resize(count, None);
        for (i, p) in self.parents.iter().enumerate() {
            if let Some(new) = map[i] {
                parents[new as usize] = p.and_then(|p| map[p].map(|p| p as usize));
            }
        }
        self.parents = parents;
        self.children.fill(0);
        self.children.resize(count, 0);
        self.continuation.fill(None);
        self.continuation.resize(count, None);
        for (i, p) in self.parents.iter().enumerate() {
            if let Some(p) = p {
                self.children[*p] += 1;
                self.continuation[*p].get_or_insert(i);
            }
        }
    }
}

impl Frontier {
    pub(in crate::branching) fn seed(
        &mut self,
        tree: &Tree,
        config: &GrowthConfig,
        t: TwigParams,
        habit: HabitParams,
    ) {
        if tree.nodes.len() < 2 {
            return;
        }
        let root_radius = tree.nodes[0].radius;
        self.stations.sync(tree);
        let children = &self.stations.children;
        if self.stations.pending.is_empty() {
            return;
        }
        let divergence = t.divergence.to_radians();
        let mut frontier = Vec::new();
        let mut completed = Vec::new();
        for &i in &self.stations.pending {
            let n = &tree.nodes[i];
            if n.position.y < config.trunk_height {
                continue;
            }
            let terminal = u16::from(children[i] == 0);
            let laterals = if n.radius < t.limb_radius * root_radius {
                ((1_u16 << t.laterals) - 1) << 1
            } else {
                0
            };
            let allocated = self.seeded.entry(n.identity.birth_order()).or_default();
            let buds = (terminal | laterals) & !*allocated;
            let possible = terminal | (((1_u16 << t.laterals) - 1) << 1);
            if (*allocated | buds) & possible == possible {
                completed.push(i);
            }
            if buds == 0 {
                continue;
            }
            let direction =
                (n.position - tree.nodes[n.parent.unwrap() as usize].position).normalized();
            if direction.length_squared() == 0.0 {
                continue;
            }
            // Terminal and lateral buds become eligible independently as the
            // scaffold extends and its trunk/branch radius ratio changes.
            *allocated |= buds;
            let floor = if habit.rise_secondary < 0.0 {
                self.stations.floor(tree, i)
            } else {
                None
            };
            let pendant = floor.is_some();
            let length = branch_length(n.radius);
            frontier.push(Shoot {
                flushed: !buds,
                accepted: Vec::new(),
                at: i,
                direction,
                normal: direction.perpendicular(),
                phase: (n.identity.birth_order() as f64 * divergence) % TAU,
                radius: n.radius,
                length,
                branch: None,
                completed: 0,
                generation: 0,
                internodes: t.internodes(n.radius, length),
                key: n.identity.birth_order() as u32,
                run: None,
                pendant,
                curtain_across: Vec3::new(-n.position.z, 0.0, n.position.x).normalized(),
                pendant_floor: floor,
            });
        }
        for i in completed {
            self.stations.pending.remove(&i);
        }
        self.queue.extend(frontier);
    }
}
