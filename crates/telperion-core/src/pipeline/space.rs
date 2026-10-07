//! Stage 2: the tree-space engine grows the family's species to its age
//! (fn-198 host decision 2), and its structure becomes the pipeline's tree
//! here, privately (host decision 3), so the pipeline's own expansion dresses
//! it in wood and leaves: one node per phytomer at its tip, z up turned to
//! the pipeline's y up. A lateral starts a branch run; a continuation or
//! relay carries its parent's on. Wood under `STRUCTURAL` of the root's
//! radius is fine wood: a twig where nothing grows from it, a branch
//! otherwise. Wood thinner than `FINEST` (a phytomer growing in from its
//! draw, and all it bears) is not drawn, and a phytomer is merged into the
//! last node drawn while the wood grown since that node is shorter than
//! `SHORTEST`, so the sweep never meets a ring it cannot resolve in float32.
use super::GrowInput;
use crate::math::Vec3;
use crate::tree::{Attachment, BudFate, Node, NodeKind, ShootState, Tree};
use crate::{Error, Result};
use telperion_space::{Origin, Request, Structure};

/// The most phytomers a tree may grow, the shed ones included: a safety
/// bound no shipped species reaches at the harness's ages (the tree-space
/// examples' still runner, fn-193; the spruce at 80 grows 4.8M).
const BUDGET: u32 = 20_000_000;
/// The most phytomers a tree may keep where the pipeline runs in 32-bit
/// Wasm, so its CPU preparation fits the 4 GB Wasm memory and a tree too
/// large is refused by name, never a trap (fn-198 host decision 5). Measured
/// in the harness at seed 42 (task 3, BROWSER.md): the spruce at 73 keeps
/// 4.27M and builds, at 75 it keeps 4.48M and traps; the bound leaves the
/// slack a page's earlier trees and other seeds take. fn-209's fine path
/// lifts it.
const WASM_KEPT: usize = 4_000_000;
const KEPT: Option<usize> = if cfg!(target_arch = "wasm32") {
    Some(WASM_KEPT)
} else {
    None
};
/// Where fine wood begins, as a share of the root's radius: the fine-wood
/// measure of the pinned baseline (fn-190, BASELINE.md).
pub(super) const STRUCTURAL: f64 = 0.05;
/// The thinnest wood drawn, in metres, and the shortest internode kept, in
/// whole micrometres of wood grown since the last node drawn: a float32
/// ring stays resolved above them (host, fn-193 C, the beech). The length
/// is the recorded grown wood (`Tree::grown`) counted as the stations count
/// it (`plan::micrometres`), so both executors take the merge alike (host,
/// fn-212 task 8).
const FINEST: f64 = 2e-4;
const SHORTEST: f64 = 1000.0;
/// A lateral of `FORK_FROM` to `FORK_AT` of its parent's radius grows in as
/// a codominant fork, so a fork has no collar (host, fn-193 RESULT.md).
const FORK_FROM: f64 = 0.4;
const FORK_AT: f64 = 0.7;

/// The family's species grown to its age, as the pipeline's tree. A family
/// with no engine species is refused by name, never grown another way
/// (host decision 9).
pub(super) fn grow(input: GrowInput) -> Result<Tree> {
    let (species, request) = asked(input)?;
    foreseen(species, request.age, KEPT)?;
    // The engine returns its tree in its canonical lineage order, parents
    // first and siblings by lineage, whichever path built it (FINE-CONTRACT
    // decision C; fn-198 host decision 17): the conversion's node numbering
    // and a relay's girth never depend on how the tree was assembled.
    let (structure, fine, bases) =
        telperion_space::grow_marked(species, request).map_err(engine)?;
    fits(&structure, KEPT)?;
    let rows = (&input.space.trunk[..], &internodes(species)[..]);
    Ok(convert(&structure, (&fine, &bases), rows))
}

/// The family's species settled with its fine roots grown and laid by
/// `roots` (fn-212 task 7, host decision 1), and the rows its conversion
/// reads: the trunk's PAs and each PA's internode. Never on the browser's
/// ceiling: the executor holding the roots is what lifts it.
pub(super) fn settled(
    input: GrowInput,
    roots: &mut dyn telperion_space::Roots,
) -> Result<(telperion_space::Settled, Vec<usize>, Vec<f64>)> {
    let (species, request) = asked(input)?;
    let plan = telperion_space::Plan::default();
    let (settled, _) = telperion_space::settle(species, request, plan, roots).map_err(engine)?;
    Ok((settled, input.space.trunk.clone(), internodes(species)))
}

/// `settled`'s structure above the cut and its roots laid on the CPU
/// converted by the one conversion, each other root's attachment recorded
/// and none of its wood drawn; each root's first axis's lineage (none:
/// shed), and the roots laid on the CPU.
pub(super) fn part(
    settled: telperion_space::Settled,
    (trunk, internodes): (&[usize], &[f64]),
) -> (Tree, Vec<Option<u64>>, Vec<usize>) {
    let part = telperion_space::partial(settled);
    let tree = convert(
        &part.structure,
        (&part.fine, &part.bases),
        (trunk, internodes),
    );
    (tree, part.lineages, part.grounded)
}

/// The family's engine species and the request it grows by.
fn asked(input: GrowInput<'_>) -> Result<(&telperion_space::Species, Request)> {
    let species = (input.space.species.as_ref()).ok_or(Error::InvalidInput("no engine species"))?;
    let age = input.age;
    if age.fract() != 0.0 || !(0.0..=f64::from(u32::MAX)).contains(&age) {
        let value = format!("{age}");
        return Err(Error::InvalidValue {
            field: "age",
            value,
        });
    }
    let request = Request {
        age: age as u32,
        seed: u64::from(input.seed),
        budget: BUDGET,
        light: input.space.light,
    };
    Ok((species, request))
}

/// Each PA's internode length, which each node's grown wood is measured by.
fn internodes(species: &telperion_space::Species) -> Vec<f64> {
    species.states.iter().map(|s| s.internode).collect()
}

/// The refusal of a tree over the browser's ceiling.
const OVER: Error = Error::ResourceLimit(
    "the tree keeps more than 4,000,000 phytomers, more than a browser \
     prepares in its 4 GB Wasm memory; grow it younger",
);

/// Refuses, before it grows, a tree expected to keep more phytomers than
/// `ceiling` (host decision 15): the cut's predictor, in milliseconds where
/// growth takes seconds. A species whose buds read light has no expected
/// count and is held by `fits` after growth alone.
fn foreseen(species: &telperion_space::Species, age: u32, ceiling: Option<usize>) -> Result<()> {
    let Some(most) = ceiling else {
        return Ok(());
    };
    let expected = telperion_space::expected_kept(species, age).map_err(engine)?;
    match expected {
        Some(kept) if kept > most as f64 => Err(OVER),
        _ => Ok(()),
    }
}

/// Refuses a grown tree that keeps more phytomers than `ceiling`, before
/// any of its preparation is allocated: a tree kept above its expectation.
fn fits(structure: &Structure, ceiling: Option<usize>) -> Result<()> {
    let kept: usize = structure.axes.iter().map(|a| a.phytomers.len()).sum();
    match ceiling {
        Some(most) if kept > most => Err(OVER),
        _ => Ok(()),
    }
}

/// The engine's refusal as the pipeline's.
fn engine(e: telperion_space::Error) -> Error {
    match e {
        telperion_space::Error::Budget { .. } => Error::ResourceLimit("engine phytomer budget"),
        e => Error::InvalidValue {
            field: "engine",
            value: e.to_string(),
        },
    }
}

/// The engine's z up as the pipeline's y up.
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
    let scaled = inside && thickest > FORK_FROM * carrier;
    if scaled {
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
    // socket flange. One drawn down to `FORK_FROM` stands where a fork's
    // weight begins, none, decided by the branch taken and never by the
    // radius it rounds to (fn-212 host decision, 2026-10-07: the
    // executors round it differently).
    head.codominant = if scaled {
        None
    } else {
        codominance(head.radius, carrier)
    };
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

/// The pipeline tree of `structure`. An axis is stem where it carries the
/// trunk on at one of the species' `trunk` ages: a leader that turns into
/// a limb stops being stem there, and a limb's relays never are, so the
/// canopy measures its slender wood against the trunk, not a fork of
/// limbs deep in the crown. Two trunk-level axes leaving one node, a
/// codominant fork of the trunk, are both stem. `internodes` is each
/// PA's internode length, which each node's grown wood is measured by.
fn convert(
    structure: &Structure,
    (fine, bases): (&[bool], &[(u64, f64)]),
    (trunk, internodes): (&[usize], &[f64]),
) -> Tree {
    // Each fine root's node drawn from wood above the cut.
    let mut seams = Vec::new();
    let mut attachments = Vec::new();
    let mut nodes = vec![Node::root()];
    // Whether each node is a fine root's.
    let mut in_root = vec![false];
    let mut frames = vec![[0.0f32; 6]];
    let mut keys = vec![0u32];
    let mut lengths = vec![0.0];
    // Each fine root's axis's root, by its place in `attachments`.
    let mut of_root: Vec<Option<usize>> = vec![None; structure.axes.len()];
    // The node each axis ends on, for what continues it.
    let mut ends: Vec<Option<u32>> = vec![None; structure.axes.len()];
    // Each axis's drawn nodes, base first.
    let mut drawn: Vec<Vec<u32>> = vec![Vec::new(); structure.axes.len()];
    for (i, axis) in structure.axes.iter().enumerate() {
        let root =
            fine.get(i).copied().unwrap_or(false) && !axis.origin.parent().is_some_and(|p| fine[p]);
        if let Some(p) = axis.origin.parent() {
            of_root[i] = of_root[p];
        }
        let start = match axis.origin {
            Origin::Seed => Some((0, false, true)),
            Origin::Lateral { parent, node, .. } => {
                let start = drawn[parent].get(node);
                if let (true, Some(&n)) = (root, start) {
                    of_root[i] = Some(attachments.len());
                    // Its bearer's fork, from its base radius as phase 2
                    // handed it up (host option 3).
                    let base = bases.binary_search_by_key(&axis.lineage, |b| b.0);
                    let base = base.map_or(0.0, |k| bases[k].1);
                    attachments.push(Attachment {
                        lineage: axis.lineage,
                        node: n,
                        seams: Vec::new(),
                        fork: codominance(base, nodes[n as usize].radius).is_some(),
                    });
                }
                start.map(|&n| (n, true, false))
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
        // The wood grown since the axis's last node: the internodes merged
        // into a node are carried on to the next one drawn.
        let internode = internodes[axis.pa];
        let mut pending = 0.0;
        for (place, p) in axis
            .phytomers
            .iter()
            .enumerate()
            .take_while(|(_, p)| p.radius >= FINEST)
        {
            let from = nodes[parent as usize].position;
            // ... and those still inside the wood of the joint, where a
            // turn over less than the stem's radius would fold its surface.
            let inside =
                along != Vec3::ZERO && (up(p.tip) - from).length() < nodes[parent as usize].radius;
            let behind = !made && ((up(p.tip) - from).dot(along) < 0.0 || inside);
            pending += internode * p.scale;
            if behind || super::foliage::plan::micrometres(pending) < SHORTEST {
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
            let held = parent as usize;
            parent = nodes.len() as u32;
            // Every fine root's node drawn from wood above the cut starts a
            // run of its own (D4): its first axis's first node, and a
            // lateral its merged first phytomers hang from its bearer.
            let grown = fine.get(i).copied().unwrap_or(false);
            if grown && !in_root[held] {
                seams.push(parent);
                if let Some(a) = of_root[i] {
                    attachments[a].seams.push(parent);
                }
            }
            in_root.push(grown);
            let (h, s) = (up(p.heading), up(p.side));
            frames.push([h.x, h.y, h.z, s.x, s.y, s.z].map(|v| v as f32));
            keys.push(crate::tree::node_key(axis.lineage, place));
            lengths.push(std::mem::take(&mut pending));
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
    // A bearer counts each fine root it bears as one child, whatever the
    // root draws at its seam (host option 3): its kind never reads wood
    // another executor draws.
    let mut children = vec![0u32; nodes.len()];
    let mut seam = vec![false; nodes.len()];
    for &s in &seams {
        seam[s as usize] = true;
    }
    for (i, n) in nodes.iter().enumerate().skip(1) {
        children[n.parent.unwrap() as usize] += u32::from(!seam[i]);
    }
    for a in &attachments {
        children[a.node as usize] += 1;
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
    seams.sort_unstable();
    Tree {
        nodes,
        crossover,
        seams,
        attachments,
        frames,
        keys,
        grown: lengths,
        ..Tree::default()
    }
}

#[cfg(test)]
mod tests;
