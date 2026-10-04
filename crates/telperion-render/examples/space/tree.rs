//! The tree space's structure as a pipeline tree (`telperion_core::tree`),
//! so the pipeline's own expansion dresses it in wood and leaves: one node
//! per phytomer at its tip, z up turned to the pipeline's y up. A lateral
//! starts a branch run; a continuation or relay carries its parent's on.
//! Wood under `STRUCTURAL` of the root's radius is fine wood: a twig where
//! nothing grows from it, a branch otherwise. Wood thinner than `FINEST`
//! (a phytomer growing in from its draw, and all it bears) is not drawn,
//! and an internode shorter than `SHORTEST` is merged into its node, so
//! the sweep never meets a ring it cannot resolve in float32.
use telperion_core::math::Vec3;
use telperion_core::tree::{BudFate, Node, NodeKind, ShootState, Tree};
use telperion_space::{Origin, Structure};

pub const STRUCTURAL: f64 = 0.05;
const FINEST: f64 = 2e-4;
const SHORTEST: f64 = 1e-3;
const FORK_FROM: f64 = 0.4;
const FORK_AT: f64 = 0.7;

fn up(v: telperion_space::Vec3) -> Vec3 {
    Vec3::new(v.x, v.z, -v.y)
}

/// Turns the head of a stopped module, the nodes after `from[0]`, into a
/// lateral run of its own: off the stem, starting at its own girth.
fn aside(nodes: &mut [Node], from: &[u32]) {
    let Some(&first) = from.iter().find(|&&n| n != from[0]) else {
        return;
    };
    for &n in from.iter().filter(|&&n| n >= first) {
        let node = &mut nodes[n as usize];
        node.branch = first;
        node.stem = false;
    }
    let head = &mut nodes[first as usize];
    head.shoot = ShootState {
        bud_fate: BudFate::Lateral,
    };
    head.start_radius = head.radius;
}

/// A lateral nearly as thick as the wood it leaves parts from it as a fork
/// does: its weight rises from none at `FORK_FROM` of the parent's radius to
/// whole at `FORK_AT`.
fn codominance(radius: f64, parent: f64) -> Option<f64> {
    let weight = ((radius / parent - FORK_FROM) / (FORK_AT - FORK_FROM)).clamp(0.0, 1.0);
    (weight > 0.0).then_some(weight)
}

pub fn convert(structure: &Structure) -> Tree {
    let mut nodes = vec![Node::root()];
    // The node each axis ends on, for what continues it.
    let mut ends: Vec<Option<u32>> = vec![None; structure.axes.len()];
    // Each axis's drawn nodes, base first.
    let mut drawn: Vec<Vec<u32>> = vec![Vec::new(); structure.axes.len()];
    for (i, axis) in structure.axes.iter().enumerate() {
        let start = match axis.origin {
            Origin::Seed => Some((0, false, true)),
            Origin::Lateral { parent, node, .. } => {
                drawn[parent].get(node).map(|&n| (n, true, false))
            }
            // A relay from inside its parent takes the parent's run on from
            // its node, and the stopped module's head beyond turns aside as
            // a lateral of its own.
            Origin::Relay { parent, node } if node + 1 < structure.axes[parent].phytomers.len() => {
                match drawn[parent].get(node..) {
                    Some(from) if !from.is_empty() => {
                        aside(&mut nodes, from);
                        Some((from[0], false, nodes[from[0] as usize].stem))
                    }
                    _ => None,
                }
            }
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                ends[parent].map(|end| (end, false, nodes[end as usize].stem))
            }
        };
        let Some((mut parent, lateral, stem)) = start else {
            continue;
        };
        let run = if lateral || parent == 0 {
            nodes.len() as u32
        } else {
            nodes[parent as usize].branch
        };
        let mut made = false;
        for p in axis.phytomers.iter().take_while(|p| p.radius >= FINEST) {
            if (up(p.tip) - nodes[parent as usize].position).length() < SHORTEST {
                drawn[i].push(parent);
                continue;
            }
            let mut node = Node::root();
            let inherited = nodes[parent as usize].radius;
            let first = !made;
            node.shoot = ShootState {
                bud_fate: if lateral && first {
                    BudFate::Lateral
                } else {
                    BudFate::Terminal
                },
            };
            node.position = up(p.tip);
            node.parent = Some(parent);
            node.radius = p.radius;
            node.start_radius = if lateral && first {
                p.radius
            } else {
                inherited.max(p.radius)
            };
            node.branch = run;
            node.stem = stem;
            if lateral && first {
                node.codominant = codominance(p.radius, inherited);
            }
            parent = nodes.len() as u32;
            made = true;
            drawn[i].push(parent);
            nodes.push(node);
        }
        // An axis with no drawn phytomers ends where it starts, and only a
        // whole axis is carried on.
        if drawn[i].len() == axis.phytomers.len() {
            ends[i] = Some(parent);
        }
    }
    let root = nodes.get(1).map_or(0.0, |n| n.start_radius);
    nodes[0].radius = root;
    nodes[0].start_radius = root;
    let mut children = vec![0u32; nodes.len()];
    for n in &nodes[1..] {
        children[n.parent.unwrap() as usize] += 1;
    }
    for i in 0..nodes.len() {
        let run = nodes[i].branch as usize;
        nodes[i].base_radius = nodes[run].start_radius;
        nodes[i].kind = if i == 0 || nodes[i].radius >= STRUCTURAL * root {
            NodeKind::Structural
        } else if children[i] == 0 {
            NodeKind::Twig
        } else {
            NodeKind::Branch
        };
    }
    let crossover = nodes.len();
    Tree {
        nodes,
        crossover,
        ..Tree::default()
    }
}
