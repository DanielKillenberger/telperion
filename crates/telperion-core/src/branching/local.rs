use super::*;
use std::{f64::consts::TAU, rc::Rc};
#[derive(Clone)]
struct Run {
    positions: Vec<Vec3>,
    fractions: Vec<f64>,
    length: f64,
}
struct Shoot {
    at: usize,
    direction: Vec3,
    normal: Vec3,
    phase: f64,
    radius: f64,
    length: f64,
    branch: Option<u32>,
    completed: usize,
    generation: usize,
    internodes: usize,
    key: u32,
    run: Option<Rc<Run>>,
}
fn rejected(config: &GrowthConfig, p: Vec3) -> bool {
    p.y < config.trunk_height
        || config
            .shell
            .is_some_and(|s| p.y > s.height || p.x.hypot(p.z) > s.radius_at(p.y))
}
struct Planner<'a> {
    config: &'a GrowthConfig,
    bias: Option<&'a GrowthBias>,
    twigs: TwigParams,
}
impl Planner<'_> {
    fn heading(&self, at: Vec3, from: Vec3, wanted: Vec3, distance: f64) -> Vec3 {
        let c = self.config;
        let wanted = self.bias.map_or(wanted.normalized(), |b| {
            b.apply(at, wanted, c.step_distance)
        });
        colonization::limit_turn(
            Some(from),
            wanted,
            c.max_turn_per_step.to_radians() * (distance / c.step_distance).min(1.0),
        )
    }
    fn run(
        &self,
        start: Vec3,
        first: Vec3,
        length: f64,
        internodes: usize,
        bearing: bool,
    ) -> Option<Rc<Run>> {
        let count = internodes.max(if bearing {
            1
        } else {
            self.twigs.laterals as usize + 1
        });
        let mut stations: Vec<_> = (1..=count).map(|k| k as f64 / count as f64).collect();
        if !bearing {
            for j in 0..self.twigs.laterals {
                let station = ((j + 1) as f64 * count as f64 / (self.twigs.laterals + 1) as f64)
                    .round()
                    .max(1.0) as usize;
                stations[station - 1] = (j + 1) as f64 / (self.twigs.laterals + 1) as f64;
            }
        }
        let mut points = vec![start];
        let mut along = vec![0.0];
        let mut heading = first;
        for k in 0..count {
            let stride = length * (stations[k] - if k == 0 { 0.0 } else { stations[k - 1] });
            let at = *points.last().unwrap();
            heading = self.heading(at, heading, heading, stride);
            let end = at + heading * stride;
            if rejected(self.config, end) {
                let mut low = 0.0;
                let mut high = stride;
                for _ in 0..40 {
                    let mid = (low + high) / 2.0;
                    if rejected(self.config, at + heading * mid) {
                        high = mid
                    } else {
                        low = mid
                    }
                }
                if low > 1e-9 {
                    points.push(at + heading * low);
                    along.push(along.last().unwrap() + low)
                }
                break;
            }
            points.push(end);
            along.push(along.last().unwrap() + stride);
        }
        let mut actual = *along.last().unwrap();
        if actual < length - 1e-9 {
            actual = (actual - self.twigs.twig.length).max(0.0)
        }
        if actual <= 1e-9 {
            return None;
        }
        while along.len() > 2 && along[along.len() - 2] >= actual {
            along.pop();
            points.pop();
        }
        let last = along.len() - 1;
        points[last] = points[last - 1].lerp(
            points[last],
            (actual - along[last - 1]) / (along[last] - along[last - 1]),
        );
        along[last] = actual;
        Some(Rc::new(Run {
            positions: points.into_iter().skip(1).collect(),
            fractions: along.into_iter().skip(1).map(|d| d / actual).collect(),
            length: actual,
        }))
    }
}
/// Append local branches to a solved structural crown. Cap diagnostics survive shedding.
pub fn append(
    tree: &mut Tree,
    config: &GrowthConfig,
    params: TwigParams,
    seed: u32,
    bias: Option<&GrowthBias>,
) -> Result<()> {
    tree.validate_solved()?;
    config.validate()?;
    let t = params.resolved()?;
    if tree.crossover != tree.nodes.len() {
        return Err(Error::InvalidInput("branching requires a structural crown"));
    }
    if tree.nodes.len() < 2 {
        return Ok(());
    }
    let crossover = tree.crossover;
    let root_radius = tree.nodes[0].radius;
    let mut children = vec![0; crossover];
    for n in tree.nodes.iter().skip(1) {
        children[n.parent.unwrap() as usize] += 1;
    }
    let divergence = t.divergence.to_radians();
    let tilt = t.angle.to_radians();
    let separation = (tilt.min(config.max_turn_per_step.to_radians()) / 2.0)
        .max(1e-6)
        .cos();
    let twig_radius = t.twig.diameter / 2.0;
    let mut frontier = Vec::new();
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        if n.position.y < config.trunk_height
            || (children[i] != 0 && n.radius >= t.limb_radius * root_radius)
        {
            continue;
        }
        let direction = (n.position - tree.nodes[n.parent.unwrap() as usize].position).normalized();
        if direction.length_squared() == 0.0 {
            continue;
        }
        let length = branch_length(n.radius);
        frontier.push(Shoot {
            at: i,
            direction,
            normal: direction.perpendicular(),
            phase: (i as f64 * divergence) % TAU,
            radius: n.radius,
            length,
            branch: None,
            completed: 0,
            generation: 0,
            internodes: t.internodes(n.radius, length),
            key: i as u32,
            run: None,
        });
    }
    let planner = Planner {
        config,
        bias,
        twigs: t,
    };
    while !frontier.is_empty() {
        let mut next = Vec::new();
        for s in frontier {
            let from = s.direction;
            let position = tree.nodes[s.at].position;
            let phase = s.phase + divergence;
            let binormal = from.cross(s.normal);
            let mut accepted: Vec<Vec3> = Vec::with_capacity(7);
            let origin = s.at < crossover;
            let bearing = !origin && s.radius <= t.twig.bearing_diameter / 2.0;
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
                    tree.nodes[s.at].radius
                };
                let radius = if lateral {
                    child_radius(supporting, ratio, t.ratio_power)
                } else {
                    s.radius
                };
                let length = if lateral { s.length * ratio } else { s.length };
                let generation = s.generation + usize::from(lateral);
                let terminal = !lateral && s.completed == s.internodes;
                let is_twig = terminal
                    || (lateral && bearing)
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
                    let across = s.normal * azimuth.cos() + binormal * azimuth.sin();
                    from * departure.cos() + across * departure.sin()
                };
                let mut run = s.run.clone();
                let (candidate, heading) = if is_twig {
                    let heading = planner.heading(
                        position,
                        if lateral { wanted } else { from },
                        wanted,
                        t.twig.length,
                    );
                    let p = position + heading * t.twig.length;
                    if rejected(config, p) {
                        continue;
                    }
                    (p, heading)
                } else {
                    if starts {
                        run = planner.run(
                            position,
                            if lateral { wanted } else { from },
                            length,
                            internodes,
                            radius <= t.twig.bearing_diameter / 2.0,
                        )
                    }
                    let Some(r) = &run else { continue };
                    internodes = r.positions.len();
                    let p = r.positions[completed];
                    (p, (p - position).normalized())
                };
                if !candidate.is_finite() {
                    return Err(Error::ResourceLimit("branch position overflow"));
                }
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
                    twig_radius
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
                tree.nodes.push(Node {
                    position: candidate,
                    parent: Some(s.at as u32),
                    radius: distal,
                    start_radius: if starts {
                        base
                    } else {
                        tree.nodes[s.at].radius
                    },
                    base_radius: base,
                    branch,
                    kind: if is_twig {
                        NodeKind::Twig
                    } else {
                        NodeKind::Branch
                    },
                });
                if !is_twig {
                    let projected = s.normal - heading * s.normal.dot(heading);
                    let normal = if projected.length_squared() > 1e-12 {
                        projected.normalized()
                    } else {
                        (binormal - heading * binormal.dot(heading)).normalized()
                    };
                    next.push(Shoot {
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
                    });
                }
            }
        }
        frontier = next;
    }
    tree.validate_solved()
}
