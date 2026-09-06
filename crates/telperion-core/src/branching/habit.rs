//! Structural habits evidenced by spreading broadleaf limbs and tiered conifer crowns.
use super::*;
use std::f64::consts::TAU;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum BranchHabit {
    #[default]
    Colonizing,
    Spreading(SpreadingHabit),
    Tiered(TieredHabit),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpreadingHabit {
    pub scaffold_limbs: u32,
    pub subdivisions: u32,
    /// Maximum natural change of direction at successive growth units, in degrees.
    pub crookedness: f64,
}
impl Default for SpreadingHabit {
    fn default() -> Self {
        Self {
            scaffold_limbs: 5,
            subdivisions: 3,
            crookedness: 24.0,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TieredHabit {
    pub tiers: u32,
    pub branches_per_tier: u32,
    /// Distance between hanging secondary axes, in metres.
    pub secondary_spacing: f64,
    /// Secondary length as a fraction of its supporting primary's length.
    pub secondary_length: f64,
    /// Rise at a primary's distal end, as a fraction of primary length.
    pub upturn: f64,
}
impl Default for TieredHabit {
    fn default() -> Self {
        Self {
            tiers: 16,
            branches_per_tier: 5,
            secondary_spacing: 0.35,
            secondary_length: 0.3,
            upturn: 0.12,
        }
    }
}
impl BranchHabit {
    pub fn validate(self) -> Result<()> {
        let valid = match self {
            Self::Colonizing => true,
            Self::Spreading(p) => {
                (2..=12).contains(&p.scaffold_limbs)
                    && (1..=6).contains(&p.subdivisions)
                    && p.crookedness.is_finite()
                    && (0.0..=60.0).contains(&p.crookedness)
            }
            Self::Tiered(p) => {
                (1..=64).contains(&p.tiers)
                    && (2..=12).contains(&p.branches_per_tier)
                    && p.secondary_spacing.is_finite()
                    && p.secondary_spacing > 0.0
                    && p.secondary_length.is_finite()
                    && (0.0..=1.0).contains(&p.secondary_length)
                    && p.upturn.is_finite()
                    && (0.0..=0.5).contains(&p.upturn)
            }
        };
        if valid {
            Ok(())
        } else {
            Err(Error::InvalidInput("branch habit"))
        }
    }
}

struct Builder<'a> {
    tree: Tree,
    envelope: Envelope,
    config: &'a GrowthConfig,
    bias: &'a GrowthBias,
    rng: Rng,
    repaired: bool,
}
impl Builder<'_> {
    fn capped(&mut self) -> bool {
        if self.tree.nodes.len() >= self.config.max_nodes.min(NODE_CEILING) {
            self.tree.diagnostics.node_capped = true;
            true
        } else {
            false
        }
    }
    fn edge(&mut self, parent: usize, position: Vec3, crown: bool) -> Result<Option<usize>> {
        if !position.is_finite() {
            return Err(Error::ResourceLimit("habit position overflow"));
        }
        let start = self.tree.nodes[parent].position;
        if start.distance(position) <= 1e-9 {
            return Ok(None);
        }
        if (1..=8).any(|k| {
            let p = start.lerp(position, k as f64 / 8.0);
            !self.envelope.contains(p, 0.0) || (crown && p.y < self.config.trunk_height)
        }) {
            return Ok(None);
        }
        if self.capped() {
            return Ok(None);
        }
        let id = self.tree.nodes.len();
        self.tree
            .nodes
            .try_reserve(1)
            .map_err(|_| Error::ResourceLimit("habit allocation"))?;
        self.tree.nodes.push(Node {
            position,
            parent: Some(parent as u32),
            branch: id as u32,
            ..Node::root()
        });
        Ok(Some(id))
    }
    fn trunk(&mut self, height: f64, min_segments: usize) -> Result<Vec<usize>> {
        let mut ids = vec![0];
        let count = (height / self.config.step_distance)
            .ceil()
            .max(min_segments as f64)
            .min(NODE_CEILING as f64) as usize;
        for k in 1..=count {
            let Some(id) = self.edge(
                *ids.last().unwrap(),
                Vec3::Y * (height * k as f64 / count as f64),
                false,
            )?
            else {
                break;
            };
            ids.push(id);
        }
        Ok(ids)
    }
    fn crooked_axis(
        &mut self,
        at: usize,
        direction: Vec3,
        length: f64,
        bend: f64,
    ) -> Result<(usize, Vec3)> {
        let mut at = at;
        let mut heading = direction;
        let units = (length / self.config.step_distance).ceil().clamp(3.0, 16.0) as usize;
        for _ in 0..units {
            let normal = heading.perpendicular();
            let phase = self.rng.range(0.0, TAU);
            let angle = self.rng.range(-bend, bend).to_radians();
            let wanted = heading * angle.cos()
                + (normal * phase.cos() + heading.cross(normal) * phase.sin()) * angle.sin();
            let start = self.tree.nodes[at].position;
            let wanted = self.bias.apply(start, wanted, self.config.step_distance);
            heading = colonization::limit_turn(
                Some(heading),
                wanted,
                self.config.max_turn_per_step.to_radians(),
            );
            let Some(next) = self.edge(at, start + heading * (length / units as f64), true)? else {
                break;
            };
            at = next;
        }
        Ok((at, heading))
    }
    fn spreading(&mut self, p: SpreadingHabit) -> Result<()> {
        let base = self
            .config
            .trunk_height
            .max(self.envelope.height * self.envelope.crown_base);
        let span = self.envelope.height - base;
        let bole = base + span * 0.08;
        let trunk = self.trunk(bole, 3)?;
        if self.tree.diagnostics.node_capped || span <= 0.0 {
            return Ok(());
        }
        let mut shoots = Vec::new();
        let mut infill = Vec::new();
        let phase = self.rng.range(0.0, TAU);
        for j in 0..p.scaffold_limbs {
            let azimuth =
                phase + TAU * j as f64 / p.scaffold_limbs as f64 + self.rng.range(-0.2, 0.2);
            let slope = self.rng.range(0.5, 1.8);
            let direction = Vec3::new(azimuth.cos(), slope, azimuth.sin()).normalized();
            shoots.push((
                *trunk.last().unwrap(),
                direction,
                span * self.rng.range(0.25, 0.34),
                0,
            ));
        }
        while !shoots.is_empty() && !self.tree.diagnostics.node_capped {
            let mut next = Vec::new();
            for (start, direction, length, order) in shoots {
                let first = self.tree.nodes.len();
                let (end, heading) = self.crooked_axis(start, direction, length, p.crookedness)?;
                // Lateral systems arise along the scaffold, not only at its end.
                // Defer them so the established scaffold keeps its random stream.
                if order + 2 <= p.subdivisions && end != start {
                    let count = self.tree.nodes.len() - first;
                    for fraction in [0.4, 0.7] {
                        let at = first + ((count - 1) as f64 * fraction) as usize;
                        infill.push((at, length * 0.45));
                    }
                }
                if end == start || order == p.subdivisions {
                    continue;
                }
                let normal = heading.perpendicular();
                let phase = self.rng.range(0.0, TAU);
                let across = normal * phase.cos() + heading.cross(normal) * phase.sin();
                for side in [-1.0, 1.0] {
                    let angle = self.rng.range(22.0_f64, 40.0).to_radians();
                    let direction =
                        (heading * angle.cos() + across * (side * angle.sin()) + Vec3::Y * 0.15)
                            .normalized();
                    next.push((
                        end,
                        direction,
                        length * self.rng.range(0.58, 0.72),
                        order + 1,
                    ));
                }
            }
            shoots = next;
        }
        for (at, length) in infill {
            let parent = self.tree.nodes[at].parent.unwrap() as usize;
            let heading =
                (self.tree.nodes[at].position - self.tree.nodes[parent].position).normalized();
            let normal = heading.perpendicular();
            let phase = self.rng.range(0.0, TAU);
            let across = normal * phase.cos() + heading.cross(normal) * phase.sin();
            let direction = (heading * 0.45 + across * 0.85 + Vec3::Y * 0.1).normalized();
            let (end, tip) = self.crooked_axis(at, direction, length, p.crookedness)?;
            if end != at {
                for side in [-1.0, 1.0] {
                    self.crooked_axis(
                        end,
                        (tip + across * side * 0.65).normalized(),
                        length * 0.55,
                        p.crookedness,
                    )?;
                }
            }
            if self.tree.diagnostics.node_capped {
                break;
            }
        }
        self.space_crowded_scaffolds();
        Ok(())
    }
    /// Repair a displaced scaffold only when it duplicates a neighbour's sector.
    /// Central ascending systems are excluded: their azimuth is ill-conditioned.
    fn space_crowded_scaffolds(&mut self) {
        let nodes = &mut self.tree.nodes;
        let mut roots = Vec::new();
        let mut owner = vec![usize::MAX; nodes.len()];
        let mut children = vec![0; nodes.len()];
        for i in 1..nodes.len() {
            children[nodes[i].parent.unwrap() as usize] += 1;
        }
        for i in 1..nodes.len() {
            let parent = nodes[i].parent.unwrap() as usize;
            let p = nodes[parent].position;
            let q = nodes[i].position;
            if p.x.hypot(p.z) < 1e-9 && q.x.hypot(q.z) > 1e-9 {
                owner[i] = roots.len();
                roots.push(i);
            } else {
                owner[i] = owner[parent];
            }
        }
        let mut centres = vec![Vec3::ZERO; roots.len()];
        let mut counts = vec![0; roots.len()];
        let mut highest = vec![0.0_f64; roots.len()];
        for i in 1..nodes.len() {
            let o = owner[i];
            if o == usize::MAX {
                continue;
            }
            highest[o] = highest[o].max(nodes[i].position.y);
            if children[i] == 0 {
                centres[o] += nodes[i].position;
                counts[o] += 1;
            }
        }
        for (c, n) in centres.iter_mut().zip(counts) {
            if n > 0 {
                *c = *c / n as f64;
            }
        }
        let angles: Vec<_> = centres.iter().map(|c| c.z.atan2(c.x)).collect();
        let wrap = |a: f64| (a + std::f64::consts::PI).rem_euclid(TAU) - std::f64::consts::PI;
        let mut sorted = angles.clone();
        sorted.sort_by(f64::total_cmp);
        let gap = (0..sorted.len())
            .map(|i| {
                let width = (sorted[(i + 1) % sorted.len()] - sorted[i]).rem_euclid(TAU);
                (width, sorted[i] + width * 0.5)
            })
            .max_by(|a, b| a.0.total_cmp(&b.0));
        for (o, &root) in roots.iter().enumerate() {
            let base = nodes[nodes[root].parent.unwrap() as usize].position;
            let first = nodes[root].position - base;
            let displacement = wrap(angles[o] - first.z.atan2(first.x));
            let crowded = angles
                .iter()
                .enumerate()
                .any(|(j, a)| j != o && wrap(angles[o] - a).abs() < 10.0_f64.to_radians());
            let yaw = if centres[o].x.hypot(centres[o].z) > self.envelope.height * 0.175
                && displacement.abs() > 70.0_f64.to_radians()
                && crowded
            {
                gap.filter(|g| g.0 > std::f64::consts::FRAC_PI_2)
                    .map_or(0.0, |g| wrap(g.1 - angles[o]))
            } else {
                0.0
            };
            // Only a scaffold extending into the reserved upper growth margin
            // is lowered; rotate its connected system, never clip its endpoints.
            let tilt = if highest[o] > self.envelope.height * 0.9 {
                12.0_f64.to_radians()
            } else {
                0.0
            };
            if yaw == 0.0 && tilt == 0.0 {
                continue;
            }
            self.repaired = true;
            let radial = Vec3::new(centres[o].x, 0.0, centres[o].z).normalized();
            let hinge = Vec3::new(-radial.z, 0.0, radial.x);
            for i in 1..nodes.len() {
                if owner[i] == o {
                    let delta = (nodes[i].position - base).rotate(hinge, -tilt);
                    nodes[i].position = base + delta.rotate(Vec3::Y, -yaw);
                }
            }
        }
    }
    fn tiered(&mut self, p: TieredHabit) -> Result<()> {
        let height = self.envelope.height;
        let base = self
            .config
            .trunk_height
            .max(height * self.envelope.crown_base);
        let span = height - base;
        let leader = self.trunk(height, p.tiers as usize * 2)?;
        if self.tree.diagnostics.node_capped || span <= 0.0 {
            return Ok(());
        }
        let mut primaries = Vec::new();
        for tier in 0..p.tiers {
            let fraction = (tier as f64 + 0.55) / (p.tiers as f64 + 0.5);
            let y = base + span * fraction;
            let at = *leader
                .iter()
                .min_by(|&&a, &&b| {
                    (self.tree.nodes[a].position.y - y)
                        .abs()
                        .total_cmp(&(self.tree.nodes[b].position.y - y).abs())
                })
                .unwrap();
            let start = self.tree.nodes[at].position;
            let phase = self.rng.range(0.0, TAU);
            for j in 0..p.branches_per_tier {
                let azimuth = phase
                    + TAU * j as f64 / p.branches_per_tier as f64
                    + self.rng.range(-0.18, 0.18);
                let radial = Vec3::new(azimuth.cos(), 0.0, azimuth.sin());
                let length = self.envelope.radius_at(start.y) * self.rng.range(0.78, 0.94);
                let mut parent = at;
                let units = (length / p.secondary_spacing)
                    .ceil()
                    .clamp(2.0, NODE_CEILING as f64) as usize;
                for k in 1..=units {
                    let t = k as f64 / units as f64;
                    let rise = length * p.upturn * (t.powi(4) - 0.25 * t);
                    let target = start + radial * (length * t) + Vec3::Y * rise;
                    let from = self.tree.nodes[parent].position;
                    let delta = target - from;
                    let candidate = from
                        + self
                            .bias
                            .apply(from, delta.normalized(), self.config.step_distance)
                            * delta.length();
                    let Some(id) = self.edge(parent, candidate, true)? else {
                        break;
                    };
                    if k < units {
                        primaries.push((
                            id,
                            radial,
                            length
                                * p.secondary_length
                                * (1.0 - 0.6 * t)
                                * self.rng.range(0.7, 1.2),
                        ));
                    }
                    parent = id;
                }
                if self.tree.diagnostics.node_capped {
                    return Ok(());
                }
            }
        }
        let mut pendants = Vec::new();
        for (at, radial, length) in primaries {
            let tangent = Vec3::new(-radial.z, 0.0, radial.x);
            let direction =
                (radial * 0.15 + tangent * self.rng.range(-0.3, 0.3) - Vec3::Y).normalized();
            let first = self.tree.nodes.len();
            self.crooked_axis(at, direction, length, 4.0)?;
            let count = self.tree.nodes.len() - first;
            // Alternating fine axes along each secondary form a hanging fan.
            // Needle density is unchanged; the supporting architecture supplies mass.
            for k in 0..count.saturating_sub(1) {
                let fraction = (k + 1) as f64 / count as f64;
                let side = if k % 2 == 0 { -1.0 } else { 1.0 };
                pendants.push((
                    first + k,
                    (tangent * side * 0.35 - Vec3::Y).normalized(),
                    // Long overlapping pendants remain below the secondary;
                    // their narrower fan concentrates branchlet mass into a curtain.
                    length * (0.95 - 0.45 * fraction),
                ));
            }
            if self.tree.diagnostics.node_capped {
                break;
            }
        }
        for (at, direction, length) in pendants {
            self.crooked_axis(at, direction, length, 4.0)?;
            if self.tree.diagnostics.node_capped {
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
pub(super) fn generate(
    params: &SkeletonParams,
    config: &GrowthConfig,
    bias: &GrowthBias,
) -> Result<Tree> {
    generate_with_repairs(params, config, bias).map(|(tree, _)| tree)
}
pub(super) fn generate_with_repairs(
    params: &SkeletonParams,
    config: &GrowthConfig,
    bias: &GrowthBias,
) -> Result<(Tree, bool)> {
    let mut b = Builder {
        tree: Tree::default(),
        envelope: if matches!(params.habit, BranchHabit::Spreading(_)) {
            inner_envelope(params.envelope, params.twigs.reach)
        } else {
            params.envelope
        },
        config,
        bias,
        rng: Rng::new(params.seed ^ 0x742be831),
        repaired: false,
    };
    if !b.capped() {
        b.tree.nodes.push(Node::root());
        if params.envelope.height > 0.0 {
            match params.habit {
                BranchHabit::Spreading(p) => b.spreading(p)?,
                BranchHabit::Tiered(p) => b.tiered(p)?,
                BranchHabit::Colonizing => unreachable!(),
            }
        }
    }
    b.tree.crossover = b.tree.nodes.len();
    b.tree.validate()?;
    Ok((b.tree, b.repaired))
}
