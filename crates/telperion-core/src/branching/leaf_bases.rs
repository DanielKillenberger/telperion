//! The retained leaf bases: the boots of shed fronds a stem keeps below its
//! crown, clothing the trunk in the lattice every date palm shows.
//!
//! A base is wood, not a station and not an instance. One base is two nodes
//! hung on the stem's own polyline - the point it leaves the axis and the
//! point it stands out of the bark - appended as a run of its own, so the
//! surface builder's socket and swell draw the junction as one piece of wood
//! and the bark material, the bounds, the shadow caster and the wood level of
//! detail all inherit it with no new code.
//!
//! Two things fix where a base may stand. It is appended after the radius
//! solve, so no base enters the pipe model and no trunk thickens under what it
//! carries. And the run leaves the axis at an exact height along the stem's
//! polyline rather than at the nearest node, so the lattice's vertical pitch
//! is the row's own arithmetic and not the skeleton's step distance, which
//! every preset shares.
//!
//! The spiral is the rosette's own. Base `k`, counting from the newest
//! downward, stands at `(rosette_fronds + k) * rosette_divergence` in the
//! frame the crown's fronds are turned in, so the crown's spiral and the
//! trunk's lattice are one sequence with one divergence and nothing authors a
//! second spiral.
//!
//! At a width the bases pack: each is drawn as the cell that spiral gives it
//! on the bark, carried outward as a wedge, so neighbours meet edge to edge
//! and the trunk reads as the diamond lattice of flat-faced boots.
use super::lattice;
use crate::{
    foliage::{self, CanopyParams},
    math::{Transcendental, Vec3},
    tree::{Node, NodeKind, Section, SectionRing, Tree},
    Error, Result,
};

/// The most retained bases one stem may keep: a rail wide enough for any
/// trunk a table has asked to clothe.
pub const MAX_LEAF_BASES: u32 = 256;

/// The least of its own length and girth weathering leaves a base at. A base
/// worn away to nothing is no base, and a node of no radius is no wood.
const WORN: f64 = 0.05;

/// The nearest a packed base leans to the stem's axis, in degrees either way.
/// A base packed into the lattice stands out of the bark at its cell; one
/// lying along the axis would never leave it.
const UPRIGHT: f64 = 15.;

/// One retained base: the stem node it is borne on, the point it leaves the
/// axis, the radial its place on the spiral puts it on, the heading it stands
/// on, how far down the spiral it is, the stem's own girth and axis where it
/// leaves, and the lattice its stem packs its bases into: the girth the
/// lattice is laid on, the stem's mean over its bases, and the full cell that
/// girth gives each base.
///
/// The radial is the spiral itself, square to the stem's own axis where the
/// base leaves it; the heading is that radial leaned back toward the axis by
/// the pitch the row states, held at least [`UPRIGHT`] off the axis when the
/// bases pack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LeafBase {
    pub at: usize,
    pub from: Vec3,
    pub radial: Vec3,
    pub heading: Vec3,
    pub age: f64,
    pub girth: f64,
    pub axis: Vec3,
    pub bark: f64,
    pub cell: [[f64; 2]; 2],
}

/// Whether this table clothes a trunk at all. At no count or no length the
/// trunk is bare and the bark is what it always was.
fn bearing(p: &CanopyParams) -> bool {
    p.leaf_bases > 0 && p.leaf_base_length > 0.
}

/// Whether this table packs its bases into the lattice rather than hanging
/// round pegs. At no width every base is the peg it always was.
fn packed(p: &CanopyParams) -> bool {
    p.leaf_base_width > 0.
}

/// Where this table's retained bases stand on this tree, newest first, in the
/// rosette's own spiral order. Reads the tree and changes nothing, so a tree
/// already clothed reads back the same table.
pub fn leaf_bases(tree: &Tree, p: &CanopyParams) -> Vec<LeafBase> {
    if !bearing(p) {
        return Vec::new();
    }
    let mut out = Vec::new();
    for rosette in foliage::rosettes(tree) {
        let stem = polyline(tree, rosette.apex);
        let mut depth = Vec::with_capacity(stem.len());
        let mut run = 0.;
        depth.push(0.);
        for pair in stem.windows(2) {
            run += tree.nodes[pair[0]]
                .position
                .distance(tree.nodes[pair[1]].position);
            depth.push(run);
        }
        // The bases begin where the oldest living frond leaves the axis and
        // reach the foot of the stem, so the two sequences meet without a gap.
        let clothed = run - p.rosette_depth;
        if clothed <= 0. {
            continue;
        }
        // One reference a stem, taken on the apex's own axis: the crown's
        // own, so the two sequences are one spiral.
        let (reference, _) = foliage::frame(rosette.axis);
        let oldest = f64::from(p.leaf_bases.saturating_sub(1));
        let spacing = clothed / oldest.max(1.);
        let pitch = if packed(p) {
            p.leaf_base_pitch.clamp(UPRIGHT, 180. - UPRIGHT)
        } else {
            p.leaf_base_pitch
        };
        let first = out.len();
        for k in 0..p.leaf_bases {
            let age = if oldest > 0. {
                f64::from(k) / oldest
            } else {
                0.
            };
            let want = p.rosette_depth + clothed * age;
            let (at, mut from, mut axis, girth) = along(tree, &stem, &depth, want);
            if packed(p) {
                (from, axis) = centreline(tree, &stem, &depth, p.rosette_depth, want, girth);
            }
            let (sin, cos) = (f64::from(p.rosette_fronds + k) * p.rosette_divergence.to_radians())
                .sin_cos_fixed();
            let (normal, binormal) = carried(rosette.axis, axis, reference);
            let radial = (normal * cos + binormal * sin).normalized();
            let (lean, upright) = pitch.to_radians().sin_cos_fixed();
            out.push(LeafBase {
                at,
                from,
                radial,
                heading: (axis * upright + radial * lean).normalized(),
                age,
                girth,
                axis,
                bark: girth,
                cell: [[0.; 2]; 2],
            });
        }
        // One lattice a stem, laid on its mean girth: a cell is an angle and
        // a height, so the bases of a tapering stem still share one tiling.
        let bases = &mut out[first..];
        let bark = bases.iter().map(|b| b.girth).sum::<f64>() / bases.len() as f64;
        // Packed bases stand on the smoothed centreline, which is shorter than
        // the polyline by every turn smoothed away; the lattice's rows are set
        // the distance its bases actually stand apart along it.
        let pitch = if packed(p) && bases.len() > 1 {
            let length: f64 = bases
                .windows(2)
                .map(|w| w[0].from.distance(w[1].from))
                .sum();
            length / (bases.len() - 1) as f64
        } else {
            spacing
        };
        let cell = lattice::cell(bark, pitch, p.rosette_divergence, 1.);
        for base in bases {
            (base.bark, base.cell) = (bark, cell);
        }
    }
    out
}

/// Hang this table's retained bases on every stem as wood, appended after the
/// radius solve so no base enters the pipe model. A table that clothes no
/// trunk leaves the tree exactly as it found it.
pub fn clothe_leaf_bases(tree: &mut Tree, p: &CanopyParams) -> Result<()> {
    if !bearing(p) {
        return Ok(());
    }
    foliage::validate_canopy(p)?;
    tree.validate_solved()?;
    let table = leaf_bases(tree, p);
    let room = table
        .len()
        .checked_mul(2)
        .and_then(|nodes| tree.nodes.len().checked_add(nodes))
        .filter(|total| *total <= u32::MAX as usize)
        .ok_or(Error::ResourceLimit("leaf base nodes"))?;
    tree.nodes.reserve(room - tree.nodes.len());
    for base in &table {
        // One row wears the foot of the trunk away: the lowest and oldest base
        // is worn back in its reach out of the bark and in its girth alike.
        let wear = (1. - p.leaf_base_weathering * base.age).max(WORN);
        let thick = (base.girth * p.leaf_base_radius).max(1e-6);
        let leaves = tree.nodes.len() as u32;
        // The point it leaves the axis: a run of its own, so no stem run is
        // ever continued into a base, and wood rather than twig, so no base is
        // clothed with foliage.
        tree.nodes.push(Node {
            position: base.from,
            parent: Some(base.at as u32),
            radius: thick,
            start_radius: thick,
            base_radius: thick,
            branch: leaves,
            kind: NodeKind::Branch,
            stem: false,
            ..Node::root()
        });
        // A packed base stands its length out of the bark, where its cell is;
        // a peg stands it out of a girth's reach along its heading.
        let emerges = if packed(p) {
            base.girth / base.radial.dot(base.heading)
        } else {
            base.girth
        };
        let reach = emerges + p.leaf_base_length * wear;
        let mut tip = base.from + base.heading * reach;
        if packed(p) {
            let drawn = section(base, p, leaves + 1, emerges, reach, wear);
            tip = drawn.centre(&drawn.rings[2]);
            tree.sections.push(drawn);
        }
        tree.nodes.push(Node {
            position: tip,
            parent: Some(leaves),
            radius: thick * wear,
            start_radius: thick,
            base_radius: thick,
            branch: leaves,
            kind: NodeKind::Branch,
            stem: false,
            ..Node::root()
        });
    }
    tree.validate_solved()
}

/// The cell a packed base is drawn as: its stem's cell at the row's width,
/// standing inside the trunk, on the bark and at the outer end, and worn at
/// the end as the base is worn. A base where the stem is thinner than the
/// lattice's girth is raised along the axis by the difference its heading
/// makes, so it meets the bark at the height its neighbours meet it.
fn section(
    base: &LeafBase,
    p: &CanopyParams,
    node: u32,
    emerges: f64,
    reach: f64,
    wear: f64,
) -> Section {
    let lean = base.radial.dot(base.heading);
    let upright = base.axis.dot(base.heading);
    let lift = (base.bark - base.girth) * upright / lean;
    let ring = |along: f64, scale: f64| SectionRing {
        reach: along * lean,
        rise: along * upright + lift,
        scale,
    };
    let width = p.leaf_base_width;
    let [a, b] = base.cell;
    Section {
        node,
        origin: base.from,
        axis: base.axis,
        radial: base.radial,
        across: base.axis.cross(base.radial),
        corners: [a.map(|v| v * width), b.map(|v| v * width)],
        flatness: p.leaf_base_flatness,
        rings: [ring(emerges / 2., 1.), ring(emerges, 1.), ring(reach, wear)],
    }
}

/// The crown's own frame carried down to the axis a base leaves on: the
/// reference turned by the one rotation that takes the apex's axis to this
/// one. A trunk wanders, and squaring the crown's reference onto a tilted axis
/// would turn the spiral's phase with the wander - a rotation carries the
/// phase instead, so one sequence runs the whole trunk. Where the two axes
/// stand against each other the local frame stands in for it.
fn carried(apex: Vec3, axis: Vec3, reference: Vec3) -> (Vec3, Vec3) {
    let turn = apex.cross(axis);
    let normal = if turn.length_squared() > 1e-18 {
        reference.rotate(
            turn.normalized(),
            apex.dot(axis).clamp(-1., 1.).acos_fixed(),
        )
    } else if apex.dot(axis) > 0. {
        reference
    } else {
        return foliage::frame(axis);
    };
    (normal, axis.cross(normal))
}

/// One stem's own polyline, apex first: the order-zero axis down to the last
/// stem node above the ground. The root is every stem's base and none, so a
/// base is never hung on it and no run of a base is ever swept as a trunk.
fn polyline(tree: &Tree, apex: usize) -> Vec<usize> {
    let mut stem = vec![apex];
    while let Some(parent) = tree.nodes[*stem.last().unwrap()].parent {
        let parent = parent as usize;
        if !tree.nodes[parent].stem {
            break;
        }
        stem.push(parent);
    }
    stem
}

/// The stem's centreline about a depth: the mean of its polyline over a
/// trunk's width either way, and the way that mean runs there. A stem's
/// polyline turns at every node by its crookedness, and a lattice laid on each
/// segment's own axis would tilt every cell against its neighbours; the trunk
/// the lattice wraps is the polyline with its turns shorter than its own
/// width smoothed away. The mean never reaches above `top`, where the crown's
/// own fronds leave the axis.
fn centreline(
    tree: &Tree,
    stem: &[usize],
    depth: &[f64],
    top: f64,
    want: f64,
    girth: f64,
) -> (Vec3, Vec3) {
    let run = depth[depth.len() - 1];
    // Symmetric about a depth, narrowed toward either end of the clothed run
    // so a base near the crown or the foot stays centred on its own place.
    let mean = |at: f64| {
        const SAMPLES: u32 = 9;
        let at = at.clamp(top, run);
        let half = (2. * girth).min(at - top).min(run - at);
        let mut sum = Vec3::ZERO;
        for k in 0..SAMPLES {
            let d = at - half + 2. * half * f64::from(k) / f64::from(SAMPLES - 1);
            sum += along(tree, stem, depth, d).1;
        }
        sum * (1. / f64::from(SAMPLES))
    };
    let chord = mean(want - girth) - mean(want + girth);
    let axis = if chord.length_squared() > 1e-18 {
        chord.normalized()
    } else {
        along(tree, stem, depth, want).2
    };
    (mean(want), axis)
}

/// Where a depth below the apex falls on the stem: the node the base is borne
/// on, the point on the polyline, the stem's own direction there and its girth
/// there. The polyline is split for the base; the stem itself is untouched.
fn along(tree: &Tree, stem: &[usize], depth: &[f64], want: f64) -> (usize, Vec3, Vec3, f64) {
    let deepest = stem.len() - 1;
    let lower = depth
        .iter()
        .position(|&d| d >= want)
        .unwrap_or(deepest)
        .clamp(1, deepest);
    let (upper, at) = (stem[lower - 1], stem[lower]);
    let span = depth[lower] - depth[lower - 1];
    let t = if span > 1e-12 {
        ((want - depth[lower - 1]) / span).clamp(0., 1.)
    } else {
        0.
    };
    let (above, below) = (tree.nodes[upper].position, tree.nodes[at].position);
    let along = above - below;
    let axis = if along.length_squared() > 1e-18 {
        along.normalized()
    } else {
        Vec3::Y
    };
    // A node's radius is its distal edge and its start radius the proximal
    // one, so the stem's girth eases from the lower node's end to the upper's.
    let girth =
        tree.nodes[upper].radius + (tree.nodes[upper].start_radius - tree.nodes[upper].radius) * t;
    (at, above + (below - above) * t, axis, girth)
}
