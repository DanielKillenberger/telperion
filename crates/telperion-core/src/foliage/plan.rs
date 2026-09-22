//! The leaf plan: which wood bears leaves, how many stations each bearing
//! segment carries, which limb system owns it and how far a leaf can reach
//! from it - all read off the solved tree before any leaf is placed. The
//! station preparation and the field both build on it, so there is one
//! definition of leaf-bearing wood, and it compiles with the wood surface
//! and the placement out.
use super::{canopy, clumping, CanopyParams, Element, TwigPlacement};
use crate::{
    envelope::Envelope,
    math::Vec3,
    surface::SurfaceParams,
    tree::{NodeKind, Tree},
    Error, Result,
};

/// Capability check only; `runs` still validates every parameter. Short
/// shoots, limb clumping and a canopy without a twig layer have no plan.
pub fn supports(p: CanopyParams, twig: Option<TwigPlacement>) -> bool {
    twig.is_some() && p.short_shoot_spacing == 0.0 && p.limb_clumping == 0.0
}

/// One unbranched run of leaf-bearing wood under a twig layer: its nodes,
/// attachment first, the distance along it to each, and its station count
/// before any cull.
#[derive(Debug)]
pub struct Run {
    pub nodes: Vec<usize>,
    pub along: Vec<f64>,
    pub internodes: u32,
    pub count: u32,
}
impl Run {
    /// Stations standing before `distance` along the run. The original
    /// reverse search chooses the last segment starting at or below a
    /// distance; this lower bound keeps that tie convention, repeated
    /// zero-length segments included, without enumerating leaves.
    pub fn stations_before(&self, distance: f64, twig: TwigPlacement) -> u32 {
        let (mut lo, mut hi) = (0u32, self.internodes);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if f64::from(mid) * twig.internode_length < distance {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo * twig.stations_per_internode
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
    let mut total = 0u32;
    for nodes in bearing_runs(tree, p) {
        let mut along = vec![0.0];
        for pair in nodes.windows(2) {
            let step = tree.nodes[pair[0]]
                .position
                .distance(tree.nodes[pair[1]].position);
            along.push(along.last().unwrap() + step);
        }
        let length = *along.last().unwrap();
        if length == 0.0 {
            continue;
        }
        if !length.is_finite() {
            return Err(Error::ResourceLimit("shoot length overflow"));
        }
        let internodes = (length / twig.internode_length - 1e-9).ceil().max(1.0);
        let count = internodes * f64::from(twig.stations_per_internode);
        if !count.is_finite()
            || count > p.max_instances as f64
            || count >= (isize::MAX as usize / size_of::<f64>()) as f64
            || count > u32::MAX as f64
        {
            return Err(Error::ResourceLimit("foliage instance budget"));
        }
        let count = count as u32;
        total = total
            .checked_add(count)
            .filter(|&n| n as usize <= p.max_instances)
            .ok_or(Error::ResourceLimit("foliage instance budget"))?;
        runs.push(Run {
            nodes,
            along,
            internodes: internodes as u32,
            count,
        });
    }
    Ok(Some(runs))
}

/// One leaf-bearing wood segment: its endpoints, the wood's radii at them,
/// the stations it carries before any cull and the limb system that owns it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Descriptor {
    pub endpoints: [Vec3; 2],
    pub radii: [f64; 2],
    pub count: u32,
    pub system: u32,
}

/// Every descriptor of one tree under one family, with the family's reach.
#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    pub descriptors: Vec<Descriptor>,
    /// Stations over every descriptor, before any cull.
    pub total: u32,
    /// The leaf element's farthest vertex from its attachment, at the largest
    /// scale placement can draw.
    pub blade: f64,
    /// The most a seated station stands off its wood, as a multiple of the
    /// wood's radius.
    pub seat: f64,
}
impl Plan {
    /// Every point a leaf on this segment can occupy lies within this
    /// distance of the segment: the wood's radius carried out to the seat,
    /// plus the blade.
    pub fn reach(&self, d: &Descriptor) -> f64 {
        d.radii[0].max(d.radii[1]) * self.seat + self.blade
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
pub fn plan(
    tree: &Tree,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
    surface: &SurfaceParams,
    element: &Element,
) -> Result<Option<Plan>> {
    surface.validate()?;
    element.validate()?;
    let Some(runs) = runs(tree, envelope, p, twig)? else {
        return Ok(None);
    };
    let twig = twig.unwrap();
    let system = clumping::systems(tree, p.clump_system_order);
    let mut descriptors = Vec::new();
    let mut total = 0u32;
    for run in &runs {
        for segment in 0..run.nodes.len() - 1 {
            let (first, last) = run.segment_stations(segment, twig);
            if first == last {
                continue;
            }
            let distal = &tree.nodes[run.nodes[segment + 1]];
            descriptors
                .try_reserve(1)
                .map_err(|_| Error::ResourceLimit("foliage plan allocation"))?;
            descriptors.push(Descriptor {
                endpoints: [tree.nodes[run.nodes[segment]].position, distal.position],
                radii: [distal.start_radius, distal.radius],
                count: last - first,
                system: system[run.nodes[segment + 1]],
            });
        }
        total += run.count;
    }
    let extent = element
        .positions
        .iter()
        .map(|v| v.length())
        .fold(0.0, f64::max);
    Ok(Some(Plan {
        descriptors,
        total,
        blade: extent * p.size * (1.0 + p.size_variation),
        seat: seating(surface, p),
    }))
}

/// Every unbranched run of leaf-bearing wood: what the twig layer marked, plus
/// whatever else is slender enough for shoot_radius to clothe.
pub(super) fn bearing_runs(tree: &Tree, p: CanopyParams) -> Vec<Vec<usize>> {
    let slender = tree.stem_radius(|i| tree.nodes[i].radius) * p.shoot_radius;
    let bearing = |i: usize| {
        let n = &tree.nodes[i];
        n.parent.is_some()
            && (n.kind == NodeKind::Twig
                || (slender > 0. && n.radius.max(n.start_radius) <= slender))
    };
    let mut children = vec![(0usize, 0usize); tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        if bearing(i) {
            if let Some(parent) = n.parent {
                children[parent as usize].0 += 1;
                children[parent as usize].1 = i;
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Node;

    /// A trunk with two limbs, each carrying one twig run of two segments.
    fn two_limbs() -> Tree {
        let node = |parent: u32, x: f64, y: f64, r: f64, kind: NodeKind| Node {
            parent: Some(parent),
            position: Vec3::new(x, y, 0.0),
            radius: r,
            start_radius: r,
            kind,
            ..Node::root()
        };
        let mut root = Node::root();
        root.radius = 0.2;
        root.start_radius = 0.2;
        let nodes = vec![
            root,
            node(0, 0.0, 1.0, 0.2, NodeKind::Structural),
            node(1, -1.0, 2.0, 0.1, NodeKind::Structural),
            node(1, 1.0, 2.0, 0.1, NodeKind::Structural),
            node(2, -1.5, 2.5, 0.01, NodeKind::Twig),
            node(4, -2.0, 3.0, 0.005, NodeKind::Twig),
            node(3, 1.5, 2.5, 0.01, NodeKind::Twig),
            node(6, 2.0, 3.0, 0.005, NodeKind::Twig),
        ];
        Tree {
            crossover: 4,
            nodes,
            ..Tree::default()
        }
    }

    #[test]
    fn descriptors_carry_counts_systems_and_reach() {
        let tree = two_limbs();
        let twig = Some(TwigPlacement {
            internode_length: 0.5,
            stations_per_internode: 2,
        });
        let element = super::super::build_element(Default::default()).unwrap();
        let p = CanopyParams {
            size: 2.0,
            size_variation: 0.5,
            ..Default::default()
        };
        let plan = plan(
            &tree,
            Envelope::default(),
            p,
            twig,
            &SurfaceParams::default(),
            &element,
        )
        .unwrap()
        .unwrap();
        // Two runs of two segments, each segment about 0.71 m: three
        // internodes of two stations, the first two internodes on the first
        // segment and the third on the second. The first limb carries the
        // stem's system on; the second opens its own.
        assert_eq!(plan.descriptors.len(), 4);
        assert_eq!(plan.total, 12);
        let counts: Vec<u32> = plan.descriptors.iter().map(|d| d.count).collect();
        assert_eq!(counts, [4, 2, 4, 2]);
        let systems: Vec<u32> = plan.descriptors.iter().map(|d| d.system).collect();
        assert_eq!(systems, [1, 1, 3, 3]);
        let extent = element
            .positions
            .iter()
            .map(|v| v.length())
            .fold(0.0, f64::max);
        assert_eq!(plan.blade, extent * 3.0);
        assert_eq!(plan.seat, 1.0);
        assert_eq!(plan.reach(&plan.descriptors[0]), 0.01 + plan.blade);
        let seated = seating(
            &SurfaceParams::default(),
            CanopyParams {
                surface_contact: 0.5,
                ..p
            },
        );
        assert!(seated > 1.0);
    }

    #[test]
    fn unsupported_families_have_no_plan_and_bad_rows_are_refused() {
        let tree = two_limbs();
        let element = super::super::build_element(Default::default()).unwrap();
        let twig = Some(TwigPlacement::default());
        let none = |p: CanopyParams, twig| {
            plan(
                &tree,
                Envelope::default(),
                p,
                twig,
                &SurfaceParams::default(),
                &element,
            )
        };
        assert!(none(CanopyParams::default(), None).unwrap().is_none());
        let spurs = CanopyParams {
            short_shoot_spacing: 0.03,
            ..Default::default()
        };
        assert!(none(spurs, twig).unwrap().is_none());
        let clumped = CanopyParams {
            limb_clumping: 0.25,
            ..Default::default()
        };
        assert!(none(clumped, twig).unwrap().is_none());
        let bad = CanopyParams {
            scatter: 91.0,
            ..Default::default()
        };
        assert!(none(bad, twig).is_err());
        let budget = CanopyParams {
            max_instances: 1,
            ..Default::default()
        };
        assert!(none(budget, twig).is_err());
        let bare = CanopyParams {
            size: 0.0,
            ..Default::default()
        };
        assert!(none(bare, twig).unwrap().unwrap().descriptors.is_empty());
    }
}
