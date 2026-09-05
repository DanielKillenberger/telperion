//! Deterministic space colonization. Attractors keep their nearest node as the tree grows.
pub mod fill;
mod grid;

use crate::{
    bias::GrowthBias,
    envelope::Envelope,
    math::Vec3,
    tree::{Node, Tree},
    Error, Result,
};
use grid::AttractorGrid;

#[derive(Debug, Clone, Copy)]
pub struct GrowthConfig {
    pub influence_radius: f64,
    pub kill_distance: f64,
    pub step_distance: f64,
    pub trunk_height: f64,
    pub max_nodes: usize,
    /// Degrees per step.
    pub max_turn_per_step: f64,
    pub shell: Option<Envelope>,
}
impl Default for GrowthConfig {
    fn default() -> Self {
        Self {
            influence_radius: 4.5,
            kill_distance: 1.0,
            step_distance: 0.5,
            trunk_height: 6.0,
            max_nodes: 4000,
            max_turn_per_step: 35.0,
            shell: None,
        }
    }
}
impl GrowthConfig {
    pub fn validate(&self) -> Result<()> {
        if ![
            self.influence_radius,
            self.kill_distance,
            self.trunk_height,
            self.max_turn_per_step,
        ]
        .iter()
        .all(|x| x.is_finite() && *x >= 0.0)
            || !self.step_distance.is_finite()
            || self.step_distance <= 0.0
            || !self.influence_radius.powi(2).is_finite()
            || !self.kill_distance.powi(2).is_finite()
            || !self.step_distance.powi(2).is_finite()
            || self.max_nodes > u32::MAX as usize
        {
            return Err(Error::InvalidInput("colonization configuration"));
        }
        if let Some(shell) = self.shell {
            shell.validate()?;
        }
        Ok(())
    }
    fn allows_edge(&self, from: Vec3, to: Vec3) -> bool {
        self.shell.is_none_or(|shell| {
            if from.y < self.trunk_height {
                to.y <= shell.height
            } else {
                !shell.contains(from, 0.0) || shell.contains(to, 0.0)
            }
        })
    }
}

/// Great-circle turn limit; arguments are unit directions and a nonnegative angle.
pub fn limit_turn(from: Option<Vec3>, wanted: Vec3, max_radians: f64) -> Vec3 {
    let Some(from) = from else { return wanted };
    if max_radians >= std::f64::consts::PI {
        return wanted;
    }
    let angle = from.dot(wanted).clamp(-1.0, 1.0).acos();
    if angle <= max_radians {
        return wanted;
    }
    let sine = angle.sin();
    if sine < 1e-9 {
        return from * max_radians.cos() + from.perpendicular() * max_radians.sin();
    }
    (from * ((angle - max_radians).sin() / sine) + wanted * (max_radians.sin() / sine)).normalized()
}
fn arrival(tree: &Tree, index: usize) -> Option<Vec3> {
    tree.nodes[index]
        .parent
        .map(|p| (tree.nodes[index].position - tree.nodes[p as usize].position).normalized())
}
fn append(tree: &mut Tree, parent: usize, position: Vec3) -> Result<()> {
    if !position.is_finite() {
        return Err(Error::InvalidInput("colonization step overflow"));
    }
    tree.nodes.push(Node {
        position,
        parent: Some(parent as u32),
        branch: tree.nodes.len() as u32,
        ..Node::root()
    });
    Ok(())
}
struct Attraction {
    alive: Vec<bool>,
    nearest: Vec<usize>,
    distance: Vec<f64>,
    scanned: usize,
}
impl Attraction {
    fn new(count: usize) -> Self {
        Self {
            alive: vec![true; count],
            nearest: vec![usize::MAX; count],
            distance: vec![f64::INFINITY; count],
            scanned: 0,
        }
    }
    fn settle(
        &mut self,
        tree: &Tree,
        points: &[Vec3],
        grid: &AttractorGrid,
        reach_sq: f64,
        kill_sq: f64,
    ) {
        for n in self.scanned..tree.nodes.len() {
            let position = tree.nodes[n].position;
            grid.visit(position, |a| {
                if !self.alive[a] {
                    return;
                }
                let d = position.distance_squared(points[a]);
                if d <= reach_sq && d < self.distance[a] {
                    self.distance[a] = d;
                    self.nearest[a] = n;
                }
            });
        }
        self.scanned = tree.nodes.len();
        for a in 0..points.len() {
            if self.distance[a] <= kill_sq {
                self.alive[a] = false;
            }
        }
    }
    fn in_reach(&self, a: usize, influence_sq: f64) -> bool {
        self.alive[a] && self.nearest[a] != usize::MAX && self.distance[a] <= influence_sq
    }
}

/// Produces parent-before-child structural nodes, with crossover at the end.
/// Empty attractors leave a root; a zero node budget returns an explicitly capped empty tree.
pub fn colonize(
    points: &[Vec3],
    start: Vec3,
    config: &GrowthConfig,
    bias: Option<&GrowthBias>,
) -> Result<Tree> {
    config.validate()?;
    if !start.is_finite()
        || points
            .iter()
            .any(|p| !p.is_finite() || !p.distance_squared(start).is_finite())
    {
        return Err(Error::InvalidInput("colonization positions"));
    }
    let mut tree = Tree::default();
    if config.max_nodes == 0 {
        tree.diagnostics.node_capped = true;
        return Ok(tree);
    }
    tree.nodes.push(Node {
        position: start,
        ..Node::root()
    });
    let step = config.step_distance;
    let influence_sq = config.influence_radius * config.influence_radius;
    let kill_sq = config.kill_distance * config.kill_distance;
    let reach = config.influence_radius.max(config.kill_distance);
    let grid = AttractorGrid::new(points, reach);
    let mut attraction = Attraction::new(points.len());
    attraction.settle(&tree, points, &grid, reach * reach, kill_sq);
    let max_turn = config.max_turn_per_step.to_radians();
    let bend = |position, direction| {
        bias.map_or(direction, |field| field.apply(position, direction, step))
    };
    let ceiling = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
    while tree.nodes.len() < config.max_nodes {
        let tip = tree.nodes.len() - 1;
        let position = tree.nodes[tip].position;
        if position.y > ceiling
            || (position.y >= config.trunk_height
                && (0..points.len()).any(|a| attraction.in_reach(a, influence_sq)))
        {
            break;
        }
        let direction = limit_turn(arrival(&tree, tip), bend(position, Vec3::Y), max_turn);
        let candidate = position + direction * step;
        if !candidate.is_finite() {
            return Err(Error::InvalidInput("colonization step overflow"));
        }
        if !config.allows_edge(position, candidate) {
            break;
        }
        append(&mut tree, tip, candidate)?;
        attraction.settle(&tree, points, &grid, reach * reach, kill_sq);
    }
    let mut stopped = Vec::new();
    let mut directions: Vec<Vec<Vec3>> = Vec::new();
    while tree.nodes.len() < config.max_nodes {
        let before = tree.nodes.len();
        stopped.resize(before, false);
        directions.resize_with(before, Vec::new);
        let mut pull = vec![Vec3::ZERO; before];
        let mut active = vec![false; before];
        for (a, point) in points.iter().enumerate() {
            if !attraction.in_reach(a, influence_sq) {
                continue;
            }
            let p = attraction.nearest[a];
            if stopped[p] || tree.nodes[p].position.y < config.trunk_height {
                continue;
            }
            let delta = *point - tree.nodes[p].position;
            if delta.length_squared() == 0.0 {
                continue;
            }
            pull[p] += delta.normalized();
            active[p] = true;
        }
        let mut heading = vec![None; before];
        for p in 0..before {
            if active[p] && pull[p].length_squared() > 0.0 {
                let direction = limit_turn(
                    arrival(&tree, p),
                    bend(tree.nodes[p].position, pull[p].normalized()),
                    max_turn,
                );
                if !(tree.nodes[p].position + direction * step).is_finite() {
                    return Err(Error::InvalidInput("colonization step overflow"));
                }
                heading[p] = Some(direction);
            }
        }
        let mut closing = vec![false; before];
        for (a, point) in points.iter().enumerate() {
            if !attraction.in_reach(a, influence_sq) {
                continue;
            }
            let p = attraction.nearest[a];
            if closing[p] {
                continue;
            }
            if let Some(direction) = heading[p] {
                let position = tree.nodes[p].position;
                let candidate = position + direction * step;
                if candidate.y >= config.trunk_height
                    && config.allows_edge(position, candidate)
                    && candidate.distance_squared(*point) < attraction.distance[a]
                {
                    closing[p] = true;
                }
            }
        }
        for p in 0..before {
            if tree.nodes.len() >= config.max_nodes {
                break;
            }
            let Some(direction) = heading[p] else {
                continue;
            };
            if !closing[p] {
                stopped[p] = true;
                continue;
            }
            if directions[p]
                .iter()
                .any(|prior| prior.dot(direction) > 1.0 - 1e-9)
            {
                continue;
            }
            directions[p].push(direction);
            let candidate = tree.nodes[p].position + direction * step;
            append(&mut tree, p, candidate)?;
        }
        if tree.nodes.len() == before {
            break;
        }
        attraction.settle(&tree, points, &grid, reach * reach, kill_sq);
    }
    tree.crossover = tree.nodes.len();
    tree.diagnostics.node_capped = !points.is_empty() && tree.nodes.len() >= config.max_nodes;
    Ok(tree)
}
