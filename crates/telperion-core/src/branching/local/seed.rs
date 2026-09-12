use super::*;
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
        let crossover = tree.crossover;
        let root_radius = tree.nodes[0].radius;
        let mut children = vec![0; crossover];
        for n in tree.nodes[..crossover].iter().skip(1) {
            children[n.parent.unwrap() as usize] += 1;
        }
        let mut pendant_floor = vec![None; crossover];
        if habit.rise_secondary < 0.0 {
            let mut continuation = vec![None; crossover];
            for (i, n) in tree.nodes[..crossover].iter().enumerate().skip(1) {
                continuation[n.parent.unwrap() as usize].get_or_insert(i);
            }
            for (i, n) in tree.nodes[..crossover].iter().enumerate().skip(1) {
                let parent = n.parent.unwrap() as usize;
                pendant_floor[i] = pendant_floor[parent];
                if pendant_floor[i].is_none()
                    && (n.position - tree.nodes[parent].position).normalized().y < -0.5
                {
                    let mut end = i;
                    while let Some(next) = continuation[end] {
                        end = next;
                    }
                    pendant_floor[i] = Some(tree.nodes[end].position.y);
                }
            }
        }
        let divergence = t.divergence.to_radians();
        let mut frontier = Vec::new();
        for (i, n) in tree.nodes[..crossover].iter().enumerate().skip(1) {
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
            let pendant = pendant_floor[i].is_some();
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
                pendant_floor: pendant_floor[i],
            });
        }
        self.queue.extend(frontier);
    }
}
