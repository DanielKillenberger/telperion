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
/// lateral run of its own: off the stem, starting at its own girth, and
/// parting as a fork where it is nearly as thick as the stem.
fn aside(nodes: &mut [Node], from: &[u32]) {
    let Some(&first) = from.iter().find(|&&n| n != from[0]) else {
        return;
    };
    for &n in from.iter().filter(|&&n| n >= first) {
        let node = &mut nodes[n as usize];
        node.branch = first;
        node.stem = false;
    }
    let carrier = nodes[from[0] as usize].radius;
    // A head that lies wholly inside the wood of its joint is hidden by the
    // stem: drawn thinner than a fork parts, it neither flares a socket
    // nor forks through the stem's surface.
    let joint = nodes[from[0] as usize].position;
    let inside = from[1..]
        .iter()
        .all(|&n| (nodes[n as usize].position - joint).length() < carrier);
    let thickest = nodes[first as usize].radius;
    if inside && thickest > FORK_FROM * carrier {
        let scale = FORK_FROM * carrier / thickest;
        for &n in from.iter().filter(|&&n| n >= first) {
            nodes[n as usize].radius *= scale;
        }
    }
    let head = &mut nodes[first as usize];
    head.shoot = ShootState {
        bud_fate: BudFate::Lateral,
    };
    head.start_radius = head.radius;
    // As thick as the stem it leaves, it parts as a fork does, with no
    // socket flange.
    head.codominant = codominance(head.radius, carrier);
}

/// A lateral nearly as thick as the wood it leaves parts from it as a fork
/// does: its weight rises from none at `FORK_FROM` of the parent's radius to
/// whole at `FORK_AT`.
fn codominance(radius: f64, parent: f64) -> Option<f64> {
    let weight = ((radius / parent - FORK_FROM) / (FORK_AT - FORK_FROM)).clamp(0.0, 1.0);
    (weight > 0.0).then_some(weight)
}

/// The node of `parent` nearest a relay's base, from its bud's node on: it
/// moves from the bud's node to the tip as the relay blends into the
/// continuation it replaces.
fn joint(
    structure: &Structure,
    relay: &telperion_space::Axis,
    parent: usize,
    node: usize,
) -> usize {
    let phytomers = &structure.axes[parent].phytomers;
    let d = |i: usize| (phytomers[i].tip - relay.base).length();
    (node..phytomers.len())
        .min_by(|&a, &b| d(a).total_cmp(&d(b)))
        .unwrap_or(node)
}

/// Each species' trunk-level physiological ages, the trunk of its chain:
/// only an axis of one of these, carried on from the trunk, is stem
/// (fn-195 round 6, host decision 31). The oak's young stem, fork and
/// leader (`oak.rs`); the beech's seedling stem, leader and fork
/// (`beech.rs`); the spruce's seedling, sapling, trunk and crown leader
/// (`spruce.rs`); the palm's one stem (`palm.rs`).
#[allow(dead_code)] // each still example reads its own species's
pub const OAK_TRUNK: [usize; 3] = [0, 1, 2];
#[allow(dead_code)] // each still example reads its own species's
pub const BEECH_TRUNK: [usize; 3] = [0, 1, 2];
#[allow(dead_code)] // each still example reads its own species's
pub const SPRUCE_TRUNK: [usize; 4] = [0, 1, 2, 3];
#[allow(dead_code)] // each still example reads its own species's
pub const PALM_TRUNK: [usize; 1] = [0];

/// The pipeline tree of `structure`. An axis is stem where it carries the
/// trunk on at one of the species' `trunk` ages: a leader that turns into
/// a limb stops being stem there, and a limb's relays never are, so the
/// canopy measures its slender wood against the trunk, not a fork of
/// limbs deep in the crown. Two trunk-level axes leaving one node, a
/// codominant fork of the trunk, are both stem.
pub fn convert(structure: &Structure, trunk: &[usize]) -> Tree {
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
            // the node nearest its base, which moves from its bud's node to
            // the tip as the relay blends into the continuation it
            // replaces, and the stopped module's head beyond turns aside as
            // a lateral of its own.
            Origin::Relay { parent, node }
                if joint(structure, axis, parent, node) + 1
                    < structure.axes[parent].phytomers.len() =>
            {
                let bud = node;
                let node = joint(structure, axis, parent, node);
                match drawn[parent].get(node..) {
                    Some(from) if !from.is_empty() => {
                        // The stem between the bud's node and the joint
                        // carries the relay too: as thick as the bud's node.
                        let girth = nodes[drawn[parent][bud] as usize].radius;
                        for &n in &drawn[parent][bud + 1..=node] {
                            let n = &mut nodes[n as usize];
                            n.radius = n.radius.max(girth);
                            n.start_radius = n.start_radius.max(girth);
                        }
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
        let stem = stem && trunk.contains(&axis.pa);
        let run = if lateral || parent == 0 {
            nodes.len() as u32
        } else {
            nodes[parent as usize].branch
        };
        // A relay's base stands inside its parent's span, below the node
        // it is drawn from: its first internodes that still lie behind that
        // node along the stem merge into it, so the stem never folds back.
        let along = match axis.origin {
            Origin::Relay { .. } if parent > 0 => {
                let p = &nodes[parent as usize];
                let span = p.position - nodes[p.parent.unwrap() as usize].position;
                span * (1.0 / span.length().max(1e-12))
            }
            _ => Vec3::ZERO,
        };
        let mut made = false;
        for p in axis.phytomers.iter().take_while(|p| p.radius >= FINEST) {
            let from = nodes[parent as usize].position;
            // ... and those still inside the wood of the joint, where a
            // turn over less than the stem's radius would fold its surface.
            let inside =
                along != Vec3::ZERO && (up(p.tip) - from).length() < nodes[parent as usize].radius;
            let behind = !made && ((up(p.tip) - from).dot(along) < 0.0 || inside);
            if behind || (up(p.tip) - from).length() < SHORTEST {
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
