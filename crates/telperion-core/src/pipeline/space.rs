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
//! A node's identity is its phytomer's lineage key, so every draw keyed by
//! it is the node's own.
use super::GrowInput;
use crate::math::Vec3;
use crate::tree::{
    Bloom, BudFate, Cluster, Leafing, Node, NodeIdentity, NodeKind,
    ShootState, Tree,
};
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
    let (species, request) = asked(&input)?;
    let rows = pa_rows(species);
    let establishing = Establishing::of(&input.canopy, species, &input.space.trunk);
    let rows = (&input.space.trunk[..], &rows[..]);
    // A coarse tree keeps a structure above its cut, never the tree's
    // expected count: `fits` holds it once grown.
    if input.cut > 0 {
        let cut = f64::from(input.cut);
        let coarse = telperion_space::grow_coarse(species, request, cut).map_err(engine)?;
        fits(&coarse.structure, KEPT)?;
        let coarse = Establishing::coarse(establishing, coarse, rows.0);
        let (mut tree, drawn) = converted(&coarse.structure, rows);
        tree.stands = stands::convert(&coarse, &drawn);
        scaled(&mut tree, input.supernatural.scale());
        return Ok(tree);
    }
    foreseen(species, request.age, KEPT)?;
    // The engine returns its tree in its canonical lineage order, parents
    // first and siblings by lineage, whichever path built it (FINE-CONTRACT
    // decision C; fn-198 host decision 17): the conversion's node numbering
    // and a relay's girth never depend on how the tree was assembled.
    let structure = telperion_space::grow(species, request).map_err(engine)?;
    fits(&structure, KEPT)?;
    let structure = Establishing::apply(establishing, &structure, rows.0);
    let mut tree = convert(&structure, rows);
    scaled(&mut tree, input.supernatural.scale());
    Ok(tree)
}

/// A PA's row as the conversion reads it, for an executor that draws
/// the engine's tree itself (fn-256 task 3): its internode and its
/// spread, its flowering share, the leaves at each of its axes' tips and
/// their fan, its leaf retention's first shedding age and that age's
/// share, and whether it carries the trunk on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConvertRow {
    pub internode: f64,
    pub spread: f64,
    pub flowering: f64,
    pub leaves: u32,
    pub fan: f64,
    pub sheds: u32,
    pub share: u16,
    pub trunk: bool,
    /// Its year on the presentation's clock (fn-270 R5c).
    pub season: telperion_space::unfold::Season,
}

/// What another executor grows and draws the family's tree by: the
/// engine's species, the request, the scale it is drawn at, each PA's
/// conversion row, and the rows its stem establishes by.
pub(super) type Grows = (telperion_space::Species, Request, f64, Vec<ConvertRow>, Option<Establishing>);

pub(super) fn growth(input: GrowInput) -> Result<Grows> {
    let (species, request) = asked(&input)?;
    let rows = pa_rows(species);
    let establishing = Establishing::of(&input.canopy, species, &input.space.trunk);
    let rows = rows.iter().enumerate().map(|(pa, r)| ConvertRow {
        internode: r.internode,
        spread: r.spread,
        flowering: r.flowering,
        leaves: r.cluster.0,
        fan: r.cluster.1,
        sheds: r.retention.0,
        share: r.retention.1,
        trunk: input.space.trunk.contains(&pa),
        season: r.season,
    });
    let rows = rows.collect();
    Ok((species.clone(), request, input.supernatural.scale(), rows, establishing))
}

/// The converted tree drawn at `scale` times its natural size (fn-217
/// S1): every position and length once, and no orientation. The engine
/// grew it, and the conversion took every decision of its topology, at the
/// natural size, so the tree is the natural one enlarged.
fn scaled(tree: &mut Tree, scale: f64) {
    if scale == 1.0 {
        return;
    }
    for n in &mut tree.nodes {
        n.position = n.position * scale;
        n.radius *= scale;
        n.start_radius *= scale;
        n.base_radius *= scale;
    }
    for grown in &mut tree.grown {
        *grown *= scale;
    }
    for leafing in &mut tree.leafing {
        leafing.older *= scale;
    }
    for s in &mut tree.stands {
        (s.base, s.tip) = (s.base * scale, s.tip * scale);
        (s.radius, s.reach) = (s.radius * scale, s.reach * scale);
    }
}

/// The family's engine species and the request it grows by. The
/// supernatural group's rows are judged here, whether or not it is on: the
/// conversion reads its scale.
fn asked<'a>(input: &'a GrowInput<'_>) -> Result<(&'a telperion_space::Species, Request)> {
    input.supernatural.validate()?;
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

/// Each PA's row the conversion reads: its internode length and spread
/// (fn-222), which each node's grown wood is measured by, the share of
/// its nodes that flower (fn-216), the leaves at each of its axes'
/// tips and their fan (fn-223), and how long its wood keeps its leaves
/// (fn-248, `tree::retention`).
#[derive(Debug, Clone, Copy)]
pub(super) struct PaRow {
    internode: f64,
    spread: f64,
    flowering: f64,
    cluster: (u32, f64),
    retention: (u32, u16),
    /// Its year on the presentation's clock (fn-270 R5c).
    season: telperion_space::unfold::Season,
}

/// Each PA's row (`PaRow`).
fn pa_rows(species: &telperion_space::Species) -> Vec<PaRow> {
    (species.states.iter())
        .map(|s| PaRow {
            internode: s.internode,
            spread: s.form.internode_spread,
            flowering: s.flowering,
            // Whole, 0 to 8, by the species' own check.
            cluster: (s.cluster_leaves as u32, s.cluster_spread),
            retention: crate::tree::retention(s.leaf_years),
            season: telperion_space::unfold::Season::of(s),
        })
        .collect()
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
pub(super) fn up(v: telperion_space::Vec3) -> Vec3 {
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

/// The pipeline tree of `structure`. An axis is stem where it carries the
/// trunk on at one of the species' `trunk` ages: a leader that turns into
/// a limb stops being stem there, and a limb's relays never are, so the
/// canopy measures its slender wood against the trunk, not a fork of
/// limbs deep in the crown. Two trunk-level axes leaving one node, a
/// codominant fork of the trunk, are both stem. `rows` is each PA's row:
/// its internode length, which each node's grown wood is measured by, and
/// its flowering share; the tip of an axis of an age that flowers, its last
/// node drawn from its own wood, is kept as a bloom (fn-253). An axis of an age that states a cluster is a
/// short shoot: each node it draws is marked, and its last drawn node, its
/// own tip, keeps the cluster; an axis that draws no node bears none.
fn convert(structure: &Structure, rows: (&[usize], &[PaRow])) -> Tree {
    converted(structure, rows).0
}

/// `convert`'s tree and each axis's drawn nodes, base first: where a coarse
/// tree's stands are borne (fn-214).
fn converted(structure: &Structure, rows: (&[usize], &[PaRow])) -> (Tree, Vec<Vec<u32>>) {
    unfolded(structure, rows, Unfolding::WHOLE)
}

/// `converted`, its leaves drawn by `unfolding`.
fn unfolded(
    structure: &Structure,
    (trunk, rows): (&[usize], &[PaRow]),
    unfolding: Unfolding,
) -> (Tree, Vec<Vec<u32>>) {
    let mut nodes = vec![Node::root()];
    let mut blooms = Vec::new();
    let mut clusters = Vec::new();
    // Each node's mark, kept only where some age states a cluster.
    let clustering = rows.iter().any(|r| r.cluster.0 > 0);
    let mut clustered = vec![false; usize::from(clustering)];
    // Each node's leaf retention, kept only where some age's wood sheds
    // its leaves (fn-248).
    let leafing_on = rows.iter().any(|r| r.retention.0 != u32::MAX);
    let mut leafing = vec![Leafing::ALL; usize::from(leafing_on)];
    let mut frames = vec![[0.0f32; 6]];
    // Each node's leaves within the year, kept only where it is drawn
    // within it (fn-270 R5, R5c).
    let unfolding_on = unfolding.within();
    let mut unfolded = vec![crate::tree::Unfolded::WHOLE; usize::from(unfolding_on)];
    let mut keys = vec![0u32];
    let mut lengths = vec![0.0];
    // The node each axis ends on, for what continues it.
    let mut ends: Vec<Option<u32>> = vec![None; structure.axes.len()];
    // Each axis's drawn nodes, base first.
    let mut drawn: Vec<Vec<u32>> = vec![Vec::new(); structure.axes.len()];
    // Each axis's relays by where they join it, and each relay's join as
    // drawn: the node and its parent's first phytomer beyond it.
    let joins = join::by_parent(structure);
    let mut joined: Vec<Option<(u32, usize)>> = vec![None; structure.axes.len()];
    for (i, axis) in structure.axes.iter().enumerate() {
        let start = match axis.origin {
            Origin::Seed => Some((0, false, true)),
            Origin::Lateral { parent, node, .. } => drawn[parent].get(node).map(|&n| (n, true, false)),
            // A relay takes its parent's run on from its join, the node drawn
            // where it joins (`telperion_space::Join`, fn-244), and the
            // stopped module's head beyond turns aside as a lateral of its
            // own, its stub.
            Origin::Relay { parent, .. } if joined[i].is_some() => {
                let (at, stub) = joined[i].expect("a join");
                let head = drawn[parent].get(stub..).unwrap_or_default();
                let from: Vec<u32> = std::iter::once(at).chain(head.iter().copied()).collect();
                aside(&mut nodes, &from);
                Some((at, false, nodes[at as usize].stem))
            }
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                ends[parent].map(|end| (end, false, nodes[end as usize].stem))
            }
        };
        let Some((mut parent, lateral, stem)) = start else {
            continue;
        };
        let stem = stem && trunk.contains(&axis.pa);
        let share = rows[axis.pa].flowering;
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
        let mut own_tip = false;
        // The wood grown since the axis's last node: the internodes merged
        // into a node are carried on to the next one drawn.
        let PaRow {
            internode,
            spread,
            cluster,
            retention,
            ..
        } = rows[axis.pa];
        let mut pending = 0.0;
        // The retentions of the wood merged into the next node: where the
        // last change falls, the retention before it and the latest.
        let mut wood: Option<(f64, u16, u16)> = None;
        // The same pieces' retentions a year before, before the boundary
        // and the latest.
        let mut was: Option<(u16, u16)> = None;
        for (place, p) in axis
            .phytomers
            .iter()
            .enumerate()
            .take_while(|(_, p)| p.radius >= FINEST)
        {
            // Each internode as it was laid: its scale and its spread's
            // factor (fn-222).
            // The wood's retention, every piece of it the phytomer's own
            // (fn-248).
            let (q, qw) = unfolding.kept(retention, structure.age, p.cycle);
            let piece = internode * p.scale * telperion_space::internode_factor(p.key(), spread);
            let here = joins[i].iter().filter(|(_, j)| j.node == place);
            for &(c, _) in here.clone().filter(|(_, j)| j.t <= 0.0) {
                joined[c] = Some((parent, place));
            }
            let inside: Vec<_> = here.filter(|(_, j)| j.t > 0.0 && j.t < join::ONE).collect();
            // A span a relay joins inside is drawn above its join at the
            // stopped module's own wood (`Join::above`).
            let mut own = join::Draw::of(axis, place, piece);
            for (_, j) in &inside {
                own.radius = own.radius.min(j.above);
            }
            let draws = inside
                .iter()
                .map(|&&(c, j)| join::draw(axis, (c, &structure.axes[c]), j, piece))
                .chain(std::iter::once(own));
            let mut rest = piece;
            for d in draws {
                let piece = if d.relay.is_some() { d.piece } else { rest };
                rest -= if d.relay.is_some() { d.piece } else { 0.0 };
                let from = nodes[parent as usize].position;
                // ... and those still inside the wood of the joint, where a
                // turn over less than the stem's radius would fold its surface.
                let inside =
                    along != Vec3::ZERO && (d.tip - from).length() < nodes[parent as usize].radius;
                let behind = !made && ((d.tip - from).dot(along) < 0.0 || inside);
                was = Some(match (wood, was) {
                    (Some((_, _, last)), Some((_, latest))) if q != last => (latest, qw),
                    (Some(_), Some((before, _))) => (before, qw),
                    _ => (qw, qw),
                });
                wood = Some(match wood {
                    Some((_, _, last)) if q != last => (pending, last, q),
                    Some(held) => held,
                    None => (0.0, q, q),
                });
                pending += piece;
                if behind || super::foliage::plan::micrometres(pending) < SHORTEST {
                    match d.relay {
                        Some(c) => joined[c] = Some((parent, place)),
                        None => drawn[i].push(parent),
                    }
                    continue;
                }
                let mut node = Node::root();
                // Its identity is the key of the phytomer that made it, or of
                // the relay it joins: its lineage, the same node at any age and
                // any cut (fn-220).
                node.identity = NodeIdentity::lineage(d.identity);
                let inherited = nodes[parent as usize].radius;
                let first = !made;
                node.shoot = ShootState {
                    bud_fate: if lateral && first {
                        BudFate::Lateral
                    } else {
                        BudFate::Terminal
                    },
                };
                node.position = d.tip;
                node.parent = Some(parent);
                node.radius = d.radius;
                node.start_radius = if lateral && first {
                    d.radius
                } else {
                    inherited.max(d.radius)
                };
                node.branch = run;
                node.stem = stem;
                if lateral && first {
                    node.codominant = codominance(d.radius, inherited);
                }
                parent = nodes.len() as u32;
                let (h, s) = (up(p.heading), up(p.side));
                frames.push([h.x, h.y, h.z, s.x, s.y, s.z].map(|v| v as f32));
                keys.push(d.key);
                lengths.push(std::mem::take(&mut pending));
                if unfolding_on {
                    unfolded.push(unfolding.node((i, place), &rows[axis.pa], (structure.age, p.cycle)));
                }
                if leafing_on {
                    let (older, before, last) = wood.take().unwrap();
                    let (was_before, was_last) = was.take().unwrap();
                    leafing.push(Leafing {
                        older,
                        keep: u32::from(before) << 16 | u32::from(last),
                        was: u32::from(was_before) << 16 | u32::from(was_last),
                    });
                }
                // Whether the axis's last node is drawn from its own wood,
                // where its bloom may stand (fn-253).
                own_tip = d.relay.is_none();
                made = true;
                match d.relay {
                    Some(c) => joined[c] = Some((parent, place)),
                    None => drawn[i].push(parent),
                }
                nodes.push(node);
                if clustering {
                    clustered.push(cluster.0 > 0);
                }
            }
            for &(c, _) in joins[i].iter().filter(|(_, j)| j.node == place && j.t >= join::ONE) {
                joined[c] = Some((parent, place + 1));
            }
        }
        // A flowering age's inflorescence stands at its axis's tip, as its
        // cluster does (fn-253): where the shoot ends, never along it.
        if made && own_tip && share > 0.0 {
            blooms.push(Bloom {
                node: parent,
                share,
            });
        }
        if made && cluster.0 > 0 {
            clusters.push(Cluster {
                node: parent,
                leaves: cluster.0,
                spread: cluster.1,
            });
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
    for n in nodes.iter().skip(1) {
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
    let tree = Tree {
        nodes,
        crossover,
        frames,
        keys,
        grown: lengths,
        blooms,
        clusters,
        clustered,
        leafing,
        unfolding: unfolded,
        year: unfolding.year(structure, rows),
        ..Tree::default()
    };
    (tree, drawn)
}

#[cfg(test)]
mod bloom_tests;
#[cfg(test)]
mod ceiling_tests;
#[cfg(test)]
mod cluster_tests;
#[cfg(test)]
mod coarse_tests;
pub mod growth;
#[cfg(test)]
mod leafing_tests;
mod establish;
pub use establish::Establishing;
mod join;
mod unfolding;
use unfolding::Unfolding;
#[cfg(test)]
mod relay_tests;
#[cfg(test)]
mod repeat_tests;
mod stands;
#[cfg(test)]
mod tests;
