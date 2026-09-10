//! One scaffold builder. Every axis grows in growth units from the numeric
//! habit traits, and every unit of every axis, leader included, takes its
//! heading from one sum of the rule heading, the attractor pull and the bias
//! field.
use super::*;
use std::{
    collections::VecDeque,
    f64::consts::{FRAC_PI_2, PI, TAU},
};

/// Divergence of a scattered phyllotaxis, in radians.
const GOLDEN_ANGLE: f64 = 2.399_963_229_728_653;
/// Room for the accumulated error of a run of growth units, in metres.
const TOLERANCE: f64 = 1e-9;
/// Growth units one axis may spend. The node ceiling is the real bound; this
/// only keeps a vanishing step from spinning.
const MAX_UNITS: usize = 4096;

/// A stream per axis, hashed from the family seed, the parent axis and the
/// child index, so a trait step perturbs one subtree and not the whole crown.
fn axis_key(parent: u32, station: usize, member: usize) -> u32 {
    Rng::new(
        parent
            ^ (station as u32).wrapping_mul(0x9e37_79b9)
            ^ (member as u32).wrapping_mul(0x85eb_ca6b),
    )
    .next_u32()
}

struct Axis {
    at: usize,
    heading: Vec3,
    length: f64,
    order: u32,
    key: u32,
}
struct Builder<'a> {
    tree: Tree,
    envelope: Envelope,
    planning: Envelope,
    config: &'a GrowthConfig,
    bias: &'a GrowthBias,
    habit: HabitParams,
    points: &'a [Vec3],
    alive: Vec<bool>,
    influence_sq: f64,
    kill_sq: f64,
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
            return Err(Error::ResourceLimit("scaffold position overflow"));
        }
        let start = self.tree.nodes[parent].position;
        if start.distance(position) <= 1e-9 {
            return Ok(None);
        }
        // The envelope has no width below the crown base, so it constrains the
        // bole's height and nothing else there; a crown axis never enters that
        // region at all. An axis that already stands outside the silhouette,
        // as a leaning bole does where it meets the crown, may close on it.
        let held = self.envelope.contains(start, TOLERANCE);
        if (1..=8).any(|k| {
            let p = start.lerp(position, k as f64 / 8.0);
            let bole = p.y < self.config.trunk_height;
            p.y < -TOLERANCE
                || p.y > self.envelope.height + TOLERANCE
                || (crown && bole)
                || (held && !bole && !self.envelope.contains(p, TOLERANCE))
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
            .map_err(|_| Error::ResourceLimit("scaffold allocation"))?;
        self.tree.nodes.push(Node {
            position,
            parent: Some(parent as u32),
            branch: id as u32,
            ..Node::root()
        });
        Ok(Some(id))
    }
    /// Sum of the directions to every attractor still in reach, or None where
    /// the cloud is spent. An empty cloud never pulls and never stops an axis.
    fn pull(&self, position: Vec3) -> Option<Vec3> {
        let mut sum = Vec3::ZERO;
        let mut found = false;
        for (a, point) in self.points.iter().enumerate() {
            if !self.alive[a] || position.distance_squared(*point) > self.influence_sq {
                continue;
            }
            let delta = *point - position;
            if delta.length_squared() > 0.0 {
                sum += delta.normalized();
                found = true;
            }
        }
        (found && sum.length_squared() > 0.0).then(|| sum.normalized())
    }
    /// An attractor is spent when an axis passes through it: within the kill
    /// distance, but never further than the unit that just landed, or one long
    /// axis would empty the cloud its siblings have yet to reach.
    fn consume(&mut self, position: Vec3, unit: f64) {
        let reached = self.kill_sq.min(unit * unit);
        for (a, point) in self.points.iter().enumerate() {
            if self.alive[a] && position.distance_squared(*point) <= reached {
                self.alive[a] = false;
            }
        }
    }
    /// The one heading sum. Nothing else sets a heading; fn-4's occupancy term
    /// lands here as one more term.
    fn heading(&self, position: Vec3, rule: Vec3, pull: Option<Vec3>, from: Vec3) -> Vec3 {
        // The pull is a term beside the rule heading, never a replacement for
        // it: at any weight the axis still knows its own architecture.
        let wanted = match pull {
            Some(pull) => rule + pull * self.habit.attractor_weight,
            None => rule,
        };
        let wanted = if wanted.length_squared() > 1e-18 {
            wanted
        } else {
            rule
        };
        let wanted = self.bias.apply(position, wanted, self.config.step_distance);
        colonization::limit_turn(
            Some(from),
            wanted,
            self.config.max_turn_per_step.to_radians(),
        )
    }
    /// Straight-line room for a first-order axis, measured against the
    /// envelope the local layer is left to fill.
    fn reach(&self, position: Vec3, direction: Vec3) -> f64 {
        let probe = (self.envelope.height / 64.0).max(1e-9);
        let mut length = 0.0;
        for _ in 0..96 {
            let next = position + direction * (length + probe);
            if !self.planning.contains(next, 0.0) || next.y < self.config.trunk_height {
                break;
            }
            length += probe;
        }
        length
    }
    /// The growth unit divides the axis's own internode, so a station always
    /// lands on a node at exactly the spacing the trait asks for.
    fn unit(&self, order: u32) -> f64 {
        let spacing = if order == 0 {
            self.habit.leader_internode
        } else {
            self.habit.lateral_spacing
        }
        .max(1e-6);
        let steps = (spacing / self.config.step_distance).ceil().max(1.0);
        (spacing / steps).max(1e-9)
    }
    /// Laterals borne at one station: the whorl on the leader, one alternating
    /// bud on every axis below it.
    fn station(&mut self, axis: &Axis, at: usize, heading: Vec3, index: usize) -> Vec<Axis> {
        let position = self.tree.nodes[at].position;
        if position.y < self.config.trunk_height
            || (!self.points.is_empty() && self.pull(position).is_none())
        {
            return Vec::new();
        }
        let members = if axis.order == 0 {
            self.habit.laterals_per_station.max(1) as usize
        } else {
            1
        };
        let advance =
            self.habit.whorl_strength * PI + (1.0 - self.habit.whorl_strength) * GOLDEN_ANGLE;
        let tangent = Vec3::Y.cross(heading);
        let tangent = if tangent.length_squared() > 1e-12 {
            tangent.normalized()
        } else {
            heading.perpendicular()
        };
        let normal = heading.cross(tangent);
        let phase = Rng::new(axis.key ^ 0x5f35_6495).range(0.0, TAU);
        let unit = self.unit(axis.order + 1);
        let mut out = Vec::new();
        for member in 0..members {
            let key = axis_key(axis.key, index, member);
            let mut rng = Rng::new(key);
            let azimuth = phase + index as f64 * advance + member as f64 * TAU / members as f64;
            let across = tangent * azimuth.cos() + normal * azimuth.sin();
            let pitch = (self.habit.lateral_pitch
                + self.habit.pitch_variation * (2.0 * rng.next_f64() - 1.0))
                .to_radians()
                .clamp(0.0, PI);
            let direction = (heading * pitch.cos() + across * pitch.sin()).normalized();
            let length = if axis.order == 0 {
                self.reach(position, direction)
            } else {
                axis.length * self.habit.lateral_length_ratio
            };
            if length <= unit * 0.5 {
                continue;
            }
            out.push(Axis {
                at,
                heading: direction,
                length,
                order: axis.order + 1,
                key,
            });
        }
        out
    }
    fn grow(&mut self, axis: &Axis) -> Result<Vec<Axis>> {
        let unit = self.unit(axis.order);
        let units = (axis.length / unit).ceil().clamp(1.0, MAX_UNITS as f64) as usize;
        let spacing = if axis.order == 0 {
            self.habit.leader_internode
        } else {
            self.habit.lateral_spacing
        };
        let rise = if axis.order <= 1 {
            self.habit.rise_primary
        } else {
            self.habit.rise_secondary
        };
        // Crookedness is a wave about the axis's own intent, never a random
        // walk: an axis wanders and still arrives where it set out for.
        let phase = Rng::new(axis.key ^ 0x1d8e_4fc3).range(0.0, TAU);
        let crookedness = self.habit.crookedness.to_radians();
        let side = axis.heading.perpendicular();
        let across = axis.heading.cross(side);
        let up = Vec3::Y - axis.heading * axis.heading.y;
        let up = (up.length_squared() > 1e-12).then(|| up.normalized());
        let mut at = axis.at;
        let mut heading = axis.heading;
        let mut children = Vec::new();
        let mut since = 0.0;
        let mut index = 0;
        let mut stationed = false;
        for k in 0..units {
            let position = self.tree.nodes[at].position;
            let pull = self.pull(position);
            if !self.points.is_empty() && pull.is_none() && position.y >= self.config.trunk_height {
                break;
            }
            let t = (k + 1) as f64 / units as f64;
            let turn = rise * FRAC_PI_2 * t;
            let mut rule = match up {
                Some(up) => axis.heading * turn.cos() + up * turn.sin(),
                None => axis.heading,
            };
            if crookedness > 0.0 && position.y >= self.config.trunk_height {
                let angle = t * TAU * 2.0 + phase;
                rule += (side * angle.sin() + across * (angle * 0.7).cos()) * crookedness;
            }
            let next = self.heading(position, rule.normalized(), pull, heading);
            let stride = unit.min(axis.length - unit * k as f64).max(1e-9);
            let Some(id) = self.edge(at, position + next * stride, axis.order > 0)? else {
                break;
            };
            heading = next;
            at = id;
            since += stride;
            self.consume(self.tree.nodes[at].position, stride);
            stationed = false;
            if since + 1e-9 >= spacing && axis.order < self.habit.lateral_orders {
                since = 0.0;
                children.append(&mut self.station(axis, at, heading, index));
                index += 1;
                stationed = true;
            }
        }
        // The apex bears its own station, so a leader that yields early still
        // hands the crown to its forks.
        if !stationed && at != axis.at && axis.order < self.habit.lateral_orders {
            children.append(&mut self.station(axis, at, heading, index));
        }
        Ok(children)
    }
}

pub(super) fn generate(
    params: &SkeletonParams,
    config: &GrowthConfig,
    bias: &GrowthBias,
    points: &[Vec3],
) -> Result<Tree> {
    let habit = params.habit;
    let mut b = Builder {
        tree: Tree::default(),
        envelope: params.envelope,
        planning: inner_envelope(params.envelope, params.twigs.reach),
        config,
        bias,
        habit,
        points,
        alive: vec![true; points.len()],
        influence_sq: config.influence_radius * config.influence_radius,
        kill_sq: config.kill_distance * config.kill_distance,
    };
    if !b.capped() {
        b.tree.nodes.push(Node::root());
        let base = config
            .trunk_height
            .max(params.envelope.height * params.envelope.crown_base);
        let top = base + (params.envelope.height - base) * habit.apical_dominance;
        if params.envelope.height > 0.0 && top > 0.0 {
            let mut queue = VecDeque::from([Axis {
                at: 0,
                heading: Vec3::Y,
                length: top,
                order: 0,
                key: params.seed ^ 0x742b_e831,
            }]);
            while let Some(axis) = queue.pop_front() {
                if b.tree.diagnostics.node_capped {
                    break;
                }
                queue.extend(b.grow(&axis)?);
            }
        }
    }
    b.tree.crossover = b.tree.nodes.len();
    b.tree.validate()?;
    Ok(b.tree)
}
