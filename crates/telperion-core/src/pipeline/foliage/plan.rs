//! The leaf plan: which wood bears leaves, how many stations each bearing
//! segment carries, which limb system owns it and how far a leaf can reach
//! from it - all read off the solved tree before any leaf is placed. The
//! station preparation and the field both build on it, so there is one
//! definition of leaf-bearing wood, and it compiles with the wood surface
//! and the placement out.
mod fronds;
use super::{canopy, clumping, cluster, rosette, CanopyParams, Element, TwigPlacement};
use crate::{
    envelope::Envelope,
    math::Vec3,
    pipeline::surface::SurfaceParams,
    tree::{NodeKind, Tree},
    Error, Result,
};

/// Capability check only; `runs` still validates every parameter. Limb
/// clumping and a canopy without a twig layer have no plan, so
/// such a family takes the placed path. A rosette is planned frond by frond
/// (`fronds`), not by runs.
pub fn supports(p: CanopyParams, twig: Option<TwigPlacement>) -> bool {
    twig.is_some() && p.limb_clumping == 0.0
}

/// A length in whole micrometres, rounded: what a run's grown wood is
/// counted in under a twig layer. The species' internodes are decimal
/// lengths, so a run whose wood is a whole number of the twig layer's
/// internodes in exact arithmetic (whole phytomers, or a PA's internode a
/// simple multiple of it, as the beech's 25 mm is 25/6 of 6 mm) sums to it
/// exactly, in float64 and in float32 alike (fn-212 D2, host decision
/// 2026-10-07).
pub fn micrometres(metres: f64) -> f64 {
    (metres * 1e6 + 0.5).floor()
}

/// The points one run passes through, the chord along it to each (what its
/// frames read) and the wood grown along it to each (`Tree::grown`), into
/// scratch the caller owns; the run's own grown length is the last of them.
/// Under a twig layer each node's wood is counted in whole micrometres
/// (`micrometres`); on a shoot no twig layer marks it is in metres. The
/// builder sweeps this walk and the count reads it, so a run is measured by
/// one rule however many times it is read.
pub(crate) fn walk(
    tree: &Tree,
    nodes: &[usize],
    twig: Option<TwigPlacement>,
    points: &mut Vec<Vec3>,
    (chord, along): (&mut Vec<f64>, &mut Vec<f64>),
) -> f64 {
    points.clear();
    points.extend(nodes.iter().map(|i| tree.nodes[*i].position));
    chord.clear();
    chord.push(0.);
    along.clear();
    along.push(0.);
    for i in 1..points.len() {
        chord.push(chord[i - 1] + points[i].distance(points[i - 1]));
        let wood = tree.grown_length(nodes[i]);
        let step = match twig {
            Some(_) => micrometres(wood),
            None => wood,
        };
        along.push(along[i - 1] + step);
    }
    *along.last().unwrap()
}

/// The share the station at `distance` along a run under a twig layer,
/// in whole micrometres of grown wood (`walk`), its `ordinal`th, draws
/// its leaf at by its wood's retention (fn-248, `Leafing::share`), none
/// where it keeps none: the segment it stands on is the last that
/// starts at or before it, as placement seats it, and the run's first
/// node keys the draw. Every station keeps its leaf whole on a tree that
/// records no retention.
pub(crate) fn kept(
    tree: &Tree,
    (nodes, along): (&[usize], &[f64]),
    distance: f64,
    ordinal: u32,
) -> Option<f64> {
    if tree.leafing.is_empty() {
        return Some(1.0);
    }
    // The first segment starts at 0, at or before every station.
    let segment = along[..along.len() - 1].partition_point(|&a| a <= distance) - 1;
    let leafing = tree.leafing(nodes[segment + 1]);
    let at = (distance - along[segment], micrometres(leafing.older));
    leafing.share(at, tree.key(nodes[1]), ordinal)
}

/// One unbranched run of leaf-bearing wood under a twig layer: its nodes,
/// attachment first, the wood grown along it to each in whole micrometres
/// (`walk`), its station count before any cull, the stations that keep
/// their leaves (fn-248) and the leaves each station carries. The counts
/// and ordinals are stations, which the station preparation tiles by; a
/// placement of leaflets along a rachis multiplies the leaves, never the
/// stations. An ordinal counts every station, kept or not, so a station's
/// draws never depend on which others keep their leaves.
#[derive(Debug)]
pub struct Run {
    pub nodes: Vec<usize>,
    pub along: Vec<f64>,
    pub internodes: u32,
    pub count: u32,
    pub kept: u32,
    pub leaflets: u32,
}
impl Run {
    /// Stations standing before `distance` micrometres along the run. The
    /// original reverse search chooses the last segment starting at or
    /// below a distance; this lower bound keeps that tie convention,
    /// repeated zero-length segments included, without enumerating leaves.
    pub fn stations_before(&self, distance: f64, twig: TwigPlacement) -> u32 {
        let (mut lo, mut hi) = (0u32, self.internodes);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if f64::from(mid) * micrometres(twig.internode_length) < distance {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo * twig.stations_per_internode
    }
    /// The stations from ordinal `first` up to `last` that keep their
    /// leaves (`keeps`).
    pub fn kept_between(&self, tree: &Tree, twig: TwigPlacement, (first, last): (u32, u32)) -> u32 {
        if tree.leafing.is_empty() {
            return last - first;
        }
        let unit = micrometres(twig.internode_length);
        let per = twig.stations_per_internode;
        let wood = (&self.nodes[..], &self.along[..]);
        (first..last)
            .filter(|&k| kept(tree, wood, f64::from(k / per) * unit, k).is_some())
            .count() as u32
    }
    /// The station range one segment of the run carries.
    pub fn segment_stations(&self, segment: usize, twig: TwigPlacement) -> (u32, u32) {
        let first = if segment == 0 {
            0
        } else {
            self.stations_before(self.along[segment], twig)
        };
        let last = if segment + 2 == self.nodes.len() {
            self.count
        } else {
            self.stations_before(self.along[segment + 1], twig)
        };
        (first, last)
    }
}

/// `None` is the capability fallback, after parameter validation; a family
/// over its instance budget is an error, never a partial plan.
pub fn runs(
    tree: &Tree,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
) -> Result<Option<Vec<Run>>> {
    canopy::validate(tree, envelope, p, twig)?;
    if !supports(p, twig) {
        return Ok(None);
    }
    let twig = twig.unwrap();
    let mut runs = Vec::new();
    if tree.nodes.len() < 2 || p.size == 0.0 {
        return Ok(Some(runs));
    }
    let leaflets = rosette::leaflets(&p) as u32;
    let mut total = 0u32;
    let (mut points, mut chord) = (Vec::new(), Vec::new());
    for nodes in bearing_runs(tree, p) {
        let mut along = Vec::new();
        let length = walk(
            tree,
            &nodes,
            Some(twig),
            &mut points,
            (&mut chord, &mut along),
        );
        if length == 0.0 {
            continue;
        }
        if !length.is_finite() {
            return Err(Error::ResourceLimit("shoot length overflow"));
        }
        let internodes = (length / micrometres(twig.internode_length))
            .ceil()
            .max(1.0);
        let count = internodes * f64::from(twig.stations_per_internode);
        let leaves = count * f64::from(leaflets);
        if !leaves.is_finite()
            || leaves > p.max_instances as f64
            || leaves >= (isize::MAX as usize / size_of::<f64>()) as f64
            || leaves > u32::MAX as f64
        {
            return Err(Error::ResourceLimit("foliage instance budget"));
        }
        let mut run = Run {
            nodes,
            along,
            internodes: internodes as u32,
            count: count as u32,
            kept: 0,
            leaflets,
        };
        run.kept = run.kept_between(tree, twig, (0, run.count));
        total = total
            .checked_add(run.kept * leaflets)
            .filter(|&n| n as usize <= p.max_instances)
            .ok_or(Error::ResourceLimit("foliage instance budget"))?;
        runs.push(run);
    }
    // The short shoots' clusters count against the same budget (fn-223).
    for c in cluster::of(tree, &p) {
        total = total
            .checked_add(c.leaves * leaflets)
            .filter(|&n| n as usize <= p.max_instances)
            .ok_or(Error::ResourceLimit("foliage instance budget"))?;
    }
    Ok(Some(runs))
}

/// One leaf-bearing segment: its endpoints, the wood's radii at them, the
/// leaves it carries before any cull and the limb system that owns it. A
/// frond's leaflet is a box standing on no wood: its radii are its
/// half-thickness.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Descriptor {
    pub endpoints: [Vec3; 2],
    pub radii: [f64; 2],
    pub count: u32,
    pub system: u32,
    /// Zero for a capsule, the sphere swept along the segment. Otherwise an
    /// oriented box: the rectangle the segment sweeps from `-side` to
    /// `side`, pushed out by its radius either way along its normal.
    pub side: Vec3,
    /// A coarse tree's fine root standing in (fn-214); `None` for every
    /// leaf-bearing segment and leaflet.
    pub stand: Option<Standing>,
}

/// What a stand-in adds to its descriptor: how far past a leaf's own reach
/// it spreads (its expected wood's reach) and the key its occupancy of a
/// cell draws under (`tree::stand_key`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Standing {
    pub spread: f64,
    pub key: u32,
}

/// Every descriptor of one tree under one family, with the family's reach.
#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    pub descriptors: Vec<Descriptor>,
    /// Leaves over every descriptor, before any cull.
    pub total: u32,
    /// The leaf element's farthest vertex from its attachment, at the largest
    /// scale placement can draw.
    pub blade: f64,
    /// The most a seated station stands off its wood, as a multiple of the
    /// wood's radius.
    pub seat: f64,
    /// How far a rachis carries its leaflets out from the station, arched out
    /// of that line again; zero where a placement is one blade.
    pub rachis: f64,
}
// A rosette's plan is its leaflets' boxes alone, each carrying its whole
// reach in its side and thickness: its `blade` and `rachis` are zero and its
// `seat` one.
impl Plan {
    /// Every point a leaf on this segment can occupy lies within this
    /// distance of the segment: the wood's radius carried out to the seat,
    /// plus the rachis, plus the blade.
    pub fn reach(&self, d: &Descriptor) -> f64 {
        d.radii[0].max(d.radii[1]) * self.seat
            + self.rachis
            + self.blade
            + d.stand.map_or(0.0, |s| s.spread)
    }
}

/// The rachis's own reach, as the reference box grows by it: a length out
/// from the station and the arch's share of that length again.
fn rachis(p: CanopyParams) -> f64 {
    if rosette::leaflets(&p) > 1 {
        p.rachis_length * (1.0 + p.rachis_arch.abs())
    } else {
        0.0
    }
}

/// How far a seated station stands off its wood, as a multiple of the wood's
/// radius: on the shoot axis it is the radius itself; seated on the swept
/// surface it walks out along the same radial to wherever the sweep's own
/// lobes, fork swell and basal flare put that wood's skin.
pub(crate) fn seating(surface: &SurfaceParams, canopy: CanopyParams) -> f64 {
    if canopy.surface_contact > 0.0 {
        (surface.fork_swell * (1.0 + surface.lobe_depth) * surface.flare_radius).max(1.0)
    } else {
        1.0
    }
}

/// The plan of one tree, or `None` for a family the plan cannot describe.
/// `limb_order` is the deepest lateral order that opens a limb system of its
/// own, `None` for the family's `clump_system_order`; a higher order parts
/// the crown into more and smaller systems. `seed` is the family's: a frond
/// crown is bounded leaflet by leaflet on the streams placement draws from.
// Each argument is a table the plan reads; bundling them would only rename
// the family.
#[allow(clippy::too_many_arguments)]
pub fn plan(
    tree: &Tree,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
    surface: &SurfaceParams,
    element: &Element,
    limb_order: Option<u32>,
    seed: u32,
) -> Result<Option<Plan>> {
    surface.validate()?;
    element.validate()?;
    let Some(runs) = runs(tree, envelope, p, twig)? else {
        return Ok(None);
    };
    let twig = twig.unwrap();
    let system = clumping::systems(tree, limb_order.unwrap_or(p.clump_system_order));
    let mut descriptors = Vec::new();
    let mut total = 0u32;
    for run in &runs {
        for segment in 0..run.nodes.len() - 1 {
            let (first, last) = run.segment_stations(segment, twig);
            let kept = run.kept_between(tree, twig, (first, last));
            if kept == 0 {
                continue;
            }
            let distal = &tree.nodes[run.nodes[segment + 1]];
            descriptors
                .try_reserve(1)
                .map_err(|_| Error::ResourceLimit("foliage plan allocation"))?;
            descriptors.push(Descriptor {
                endpoints: [tree.nodes[run.nodes[segment]].position, distal.position],
                radii: [distal.start_radius, distal.radius],
                count: kept * run.leaflets,
                system: system[run.nodes[segment + 1]],
                side: Vec3::ZERO,
                stand: None,
            });
        }
        total += run.kept * run.leaflets;
    }
    // A cluster stands at its tip: a segment of no length there.
    for c in cluster::of(tree, &p) {
        let tip = &tree.nodes[c.node as usize];
        descriptors
            .try_reserve(1)
            .map_err(|_| Error::ResourceLimit("foliage plan allocation"))?;
        descriptors.push(Descriptor {
            endpoints: [tip.position; 2],
            radii: [tip.radius; 2],
            count: c.leaves * rosette::leaflets(&p) as u32,
            system: system[c.node as usize],
            side: Vec3::ZERO,
            stand: None,
        });
        total += c.leaves * rosette::leaflets(&p) as u32;
    }
    let extent = element
        .positions
        .iter()
        .map(|v| v.length())
        .fold(0.0, f64::max);
    if rosette::bearing(&p) {
        total = fronds::describe(tree, p, element, seed, &system, &mut descriptors)?;
        total = stands(tree, &runs, &system, total, &mut descriptors)?;
        return Ok(Some(Plan {
            descriptors,
            total,
            blade: 0.0,
            seat: 1.0,
            rachis: 0.0,
        }));
    }
    total = stands(tree, &runs, &system, total, &mut descriptors)?;
    Ok(Some(Plan {
        descriptors,
        total,
        blade: extent * p.size * (1.0 + p.size_variation),
        seat: seating(surface, p),
        rachis: rachis(p),
    }))
}

/// A coarse tree's fine roots (fn-214), each a capsule from its wood's base
/// to its tip, spread by its reach, on its bearer's limb system, carrying
/// the leaves its expected kept phytomers bear at the plan's own leaves per
/// leaf-bearing node of the tree it stands on; the plan's total with theirs.
fn stands(
    tree: &Tree,
    runs: &[Run],
    system: &[u32],
    total: u32,
    descriptors: &mut Vec<Descriptor>,
) -> Result<u32> {
    if tree.stands.is_empty() {
        return Ok(total);
    }
    let nodes: usize = runs.iter().map(|r| r.nodes.len() - 1).sum();
    let per = if nodes > 0 {
        f64::from(total) / nodes as f64
    } else {
        0.0
    };
    descriptors
        .try_reserve(tree.stands.len())
        .map_err(|_| Error::ResourceLimit("foliage plan allocation"))?;
    let mut total = total;
    for s in &tree.stands {
        let count = (s.phytomers * per).round() as u32;
        total = total
            .checked_add(count)
            .ok_or(Error::ResourceLimit("foliage instance budget"))?;
        descriptors.push(Descriptor {
            endpoints: [s.base, s.tip],
            radii: [0.0; 2],
            count,
            system: system[s.node as usize],
            side: Vec3::ZERO,
            stand: Some(Standing {
                spread: s.reach,
                key: s.key,
            }),
        });
    }
    Ok(total)
}

/// Every unbranched run of leaf-bearing wood: what the twig layer marked, plus
/// whatever else is slender enough for shoot_radius to clothe.
pub(super) fn bearing_runs(tree: &Tree, p: CanopyParams) -> Vec<Vec<usize>> {
    // A stem that bears a frond crown bears nothing along its length.
    if rosette::bearing(&p) {
        return Vec::new();
    }
    let bearing = bearing_of(tree, p);
    let children = bearing_children(tree, &bearing);
    let continues = |parent: usize, child: usize| {
        bearing(parent)
            && tree.nodes[parent].branch == tree.nodes[child].branch
            && children[parent].0 == 1
    };
    let mut runs = Vec::new();
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        if !bearing(i) {
            continue;
        }
        let Some(parent) = n.parent.map(|p| p as usize) else {
            continue;
        };
        if continues(parent, i) {
            continue;
        }
        let mut run = vec![parent, i];
        let mut at = i;
        while children[at].0 == 1 && continues(at, children[at].1) {
            at = children[at].1;
            run.push(at);
        }
        runs.push(run);
    }
    runs
}

/// Whether a node bears leaves: a twig, or wood within the slender bound,
/// never a short shoot's, which bears its leaves at its tip (fn-223).
/// The girth and the bound are compared in whole micrometres, as both
/// executors compute them alike (fn-212 task 10, host decision 4, as
/// `SHORTEST` in task 8): in float32 and float64 a girth within a float
/// step of the bound read on either side of it.
fn bearing_of(tree: &Tree, p: CanopyParams) -> impl Fn(usize) -> bool + '_ {
    let slender = micrometres(tree.stem_radius(|i| tree.pipe(i)[1]) * p.shoot_radius);
    move |i: usize| {
        let n = &tree.nodes[i];
        let girth = || micrometres(tree.pipe(i)[0].max(tree.pipe(i)[1]));
        n.parent.is_some()
            && !tree.is_clustered(i)
            && (n.kind == NodeKind::Twig || (slender > 0. && girth() <= slender))
    }
}

/// Each node's bearing children, counted, and the last of them.
fn bearing_children(tree: &Tree, bearing: &dyn Fn(usize) -> bool) -> Vec<(usize, usize)> {
    let mut children = vec![(0usize, 0usize); tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        if let (true, Some(parent)) = (bearing(i), n.parent) {
            children[parent as usize].0 += 1;
            children[parent as usize].1 = i;
        }
    }
    children
}

#[cfg(test)]
mod tests;
