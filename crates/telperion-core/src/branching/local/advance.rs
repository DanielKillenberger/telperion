use super::*;
impl Frontier {
    pub(in crate::branching) fn advance(
        &mut self,
        tree: &mut Tree,
        planner: Planner<'_>,
        habit: HabitParams,
        budget: usize,
    ) -> Result<()> {
        self.visited.clear();
        self.wake(&planner, tree);
        #[cfg(test)]
        {
            self.retries = [0; 4];
        }
        if budget == 0 || self.queue.is_empty() {
            return Ok(());
        }
        let first = tree.nodes.len();
        let t = planner.twigs;
        let config = planner.config;
        let seed = planner.seed;
        let root_radius = if tree.nodes.is_empty() {
            0.0
        } else {
            planner.width(tree, 0)[0]
        };
        self.stations.sync(tree);
        let children = &self.stations.children;
        let divergence = t.divergence.to_radians();
        let tilt = t.angle.to_radians();
        let separation = (tilt.min(config.max_turn_per_step.to_radians()) / 2.0)
            .max(1e-6)
            .cos_fixed();
        let twig_radius = t.twig.diameter / 2.0;
        let visits = if planner.growing_envelope {
            self.queue.len()
        } else {
            budget
        };
        let mut remaining = budget;
        for _ in 0..visits {
            if remaining == 0 {
                break;
            }
            let Some(mut s) = self.queue.pop_front() else {
                break;
            };
            if planner.clock.is_some() && waiting::below_reach(&s, tree, &planner) {
                self.sleeping.entry(u64::MAX).or_default().push(s);
                continue;
            }
            if planner.growing_envelope {
                self.visited.push(s.at);
                if tree.nodes[s.at].shoot.vigour() < habit.shedding_threshold {
                    self.queue.push_back(s);
                    continue;
                }
                s.radius = s.branch.map_or_else(
                    || planner.width(tree, s.at)[0],
                    |b| planner.width(tree, b as usize)[2],
                );
            }
            #[cfg(test)]
            {
                self.retries[0] += 1;
            }
            let before = tree.nodes.len();
            let from = s.direction;
            let position = tree.nodes[s.at].position;
            let phase = s.phase + divergence;
            let binormal = from.cross(s.normal);
            let mut accepted = std::mem::take(&mut s.accepted);
            let mut deferred = false;
            let mut next_wake = u64::MAX;
            let origin = tree.nodes[s.at].kind == NodeKind::Structural;
            let bearing = !origin && s.radius <= t.twig.bearing_diameter / 2.0;
            // A switch out of leaf-bearing wood can release different laterals.
            // Such shoots remain awake until a radius wake condition is available.
            let can_sleep = !origin && (t.laterals == 0 || !bearing);
            let immediate = planner.clock.map_or(0, |clock| clock.slice + 1);
            let mut laterals = 0;
            let mut first_lateral = 0;
            if origin {
                if s.radius < t.limb_radius * root_radius {
                    laterals = t.laterals as usize
                }
            } else if bearing {
                if s.completed > 0 && s.completed < s.internodes {
                    laterals = 1;
                    first_lateral = s.completed
                }
            } else {
                for j in 0..t.laterals as usize {
                    let station = ((j + 1) as f64 * s.internodes as f64 / (t.laterals + 1) as f64)
                        .round()
                        .max(1.0) as usize;
                    if station == s.completed {
                        if laterals == 0 {
                            first_lateral = j
                        }
                        laterals += 1;
                    }
                }
            }
            for c in 0..=laterals {
                let mask = 1 << c;
                if s.flushed & mask != 0 {
                    continue;
                }
                s.flushed |= mask;
                let lateral = c > 0;
                if !lateral && origin && children[s.at] != 0 {
                    continue;
                }
                let bud = first_lateral + c;
                let key = if lateral {
                    Rng::new(s.key ^ (bud as u32).wrapping_mul(0x9e3779b9)).next_u32()
                } else {
                    s.key
                };
                let draw = |salt| 2.0 * Rng::new(key ^ seed ^ salt).next_f64() - 1.0;
                let ratio = if lateral && t.vigour_variation != 0.0 {
                    (t.length_ratio * (1.0 + t.vigour_variation * draw(0x68bc21eb)))
                        .clamp(0.05, 1.0)
                } else {
                    t.length_ratio
                };
                let departure = if lateral && t.angle_variation != 0.0 {
                    (t.angle + t.angle_variation * draw(0x02e5be93))
                        .clamp(0.0, 90.0)
                        .to_radians()
                } else {
                    tilt
                };
                let supporting = if origin {
                    s.radius
                } else {
                    planner.width(tree, s.at)[0]
                };
                let radius = if lateral {
                    child_radius(supporting, ratio, t.ratio_power)
                } else {
                    s.radius
                };
                let length = if lateral { s.length * ratio } else { s.length };
                let length = s.curtain.length(length, t);
                let generation = s.generation + usize::from(lateral);
                let terminal = !lateral && s.completed == s.internodes;
                let is_twig = terminal
                    || (lateral && bearing)
                    || generation >= t.generations as usize
                    || radius <= twig_radius
                    || length < t.twig.internode_length;
                let starts = lateral || s.branch.is_none() || terminal;
                let completed = if starts { 0 } else { s.completed };
                let mut internodes = if lateral {
                    t.internodes(radius, length)
                } else {
                    s.internodes
                };
                if !is_twig && generation >= MAX_LEVELS {
                    tree.diagnostics.level_capped = true;
                    continue;
                }
                let wanted = if !lateral {
                    from
                } else {
                    let azimuth = if origin {
                        phase + (c - 1) as f64 * TAU / laterals as f64
                    } else {
                        phase + (first_lateral + c - 1) as f64 * divergence
                    };
                    let across = s.normal * azimuth.cos_fixed() + binormal * azimuth.sin_fixed();
                    let upright = from * departure.cos_fixed() + across * departure.sin_fixed();
                    if s.curtain.hangs() {
                        let side = if (first_lateral + c) % 2 == 0 {
                            -1.0
                        } else {
                            1.0
                        };
                        s.curtain.direction(upright, position.y, side, t)
                    } else {
                        upright
                    }
                };
                let mut run = s.run.clone();
                let (candidate, heading) = if is_twig {
                    let heading = planner.heading(
                        position,
                        if lateral { wanted } else { from },
                        wanted,
                        t.twig.length,
                    );
                    let twig_length = s.curtain.clear(position.y, -heading.y, t.twig.length);
                    if twig_length <= 1e-9 {
                        continue;
                    }
                    let p = position + heading * twig_length;
                    if rejected(config, p) || s.curtain.below(p.y) {
                        #[cfg(test)]
                        {
                            self.retries[1] += 1;
                        }
                        if planner.growing_envelope {
                            s.flushed &= !mask;
                            deferred = true;
                            let fixed = (terminal || length < t.twig.internode_length)
                                && planner.bias.is_none_or(GrowthBias::height_independent);
                            next_wake = next_wake.min(if can_sleep && fixed {
                                planner.clock.map_or(immediate, |clock| {
                                    if s.curtain.below(p.y) {
                                        u64::MAX
                                    } else {
                                        clock.next(p, config.trunk_height)
                                    }
                                })
                            } else {
                                immediate
                            });
                        }
                        continue;
                    }
                    (p, heading)
                } else {
                    if starts {
                        let length = s.curtain.clear(position.y, -wanted.normalized().y, length);
                        run = planner.run(
                            position,
                            if lateral { wanted } else { from },
                            length,
                            internodes,
                            radius <= t.twig.bearing_diameter / 2.0,
                            key,
                        )
                    }
                    let Some(r) = &run else {
                        next_wake = immediate;
                        #[cfg(test)]
                        {
                            self.retries[2] += 1;
                        }
                        if planner.growing_envelope {
                            s.flushed &= !mask;
                            deferred = true;
                        }
                        continue;
                    };
                    internodes = r.positions.len();
                    let p = r.positions[completed];
                    if planner.growing_envelope && rejected(config, p) {
                        #[cfg(test)]
                        {
                            self.retries[3] += 1;
                        }
                        s.flushed &= !mask;
                        deferred = true;
                        next_wake = next_wake.min(if can_sleep && !starts {
                            planner
                                .clock
                                .map_or(immediate, |clock| clock.next(p, config.trunk_height))
                        } else {
                            immediate
                        });
                        continue;
                    }
                    (p, (p - position).normalized())
                };
                if !candidate.is_finite() {
                    return Err(Error::ResourceLimit("branch position overflow"));
                }
                let separation = s.curtain.separation(separation, t);
                if lateral
                    && (from.dot(heading) >= separation
                        || accepted.iter().any(|a| a.dot(heading) >= separation))
                {
                    continue;
                }
                if tree.nodes.len() >= config.max_nodes {
                    tree.diagnostics.node_capped = true;
                    return Ok(());
                }
                if lateral {
                    accepted.push(heading)
                }
                let id = tree.nodes.len() as u32;
                let branch = if starts { id } else { s.branch.unwrap() };
                let distal = if is_twig {
                    twig_radius * habit.twig_tip_taper
                } else {
                    twig_radius
                        + (radius - twig_radius)
                            * (1.0 - run.as_ref().unwrap().fractions[completed])
                                .max(0.0)
                                .sqrt()
                };
                let base = if is_twig { twig_radius } else { radius };
                tree.nodes
                    .try_reserve(1)
                    .map_err(|_| Error::ResourceLimit("branch allocation"))?;
                // Births can stop a visit early or append younger shoots. A
                // full visit without births preserves the existing identity order.
                self.ordered = false;
                tree.nodes.push(Node {
                    position: candidate,
                    parent: Some(s.at as u32),
                    radius: distal,
                    start_radius: if starts {
                        base
                    } else {
                        planner.width(tree, s.at)[0]
                    },
                    base_radius: base,
                    branch,
                    kind: if is_twig {
                        NodeKind::Twig
                    } else {
                        NodeKind::Branch
                    },
                    ..Node::root()
                });
                tree.nodes[id as usize].shoot.bud_fate = if lateral {
                    crate::tree::BudFate::Lateral
                } else {
                    crate::tree::BudFate::Terminal
                };
                if !is_twig {
                    let projected = s.normal - heading * s.normal.dot(heading);
                    let normal = if projected.length_squared() > 1e-12 {
                        projected.normalized()
                    } else {
                        (binormal - heading * binormal.dot(heading)).normalized()
                    };
                    self.queue.push_back(Shoot {
                        flushed: 0,
                        accepted: Vec::new(),
                        at: id as usize,
                        direction: heading,
                        normal,
                        phase,
                        radius,
                        length: run.as_ref().unwrap().length,
                        branch: Some(branch),
                        completed: completed + 1,
                        generation,
                        internodes,
                        key,
                        run,
                        curtain: s.curtain,
                    });
                }
            }
            if deferred {
                s.accepted = accepted;
                if planner.clock.is_some() && next_wake > immediate {
                    self.sleeping.entry(next_wake).or_default().push(s);
                } else {
                    self.queue.push_back(s);
                }
            }
            // Waiting consumes no growth unit; younger eligible shoots can use it.
            if !planner.growing_envelope || tree.nodes.len() > before {
                remaining -= 1;
            }
        }
        tree.validate_range(first..tree.nodes.len(), true)
    }
}
