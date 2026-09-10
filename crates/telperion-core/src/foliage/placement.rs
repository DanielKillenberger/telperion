use super::{
    range,
    station::{place_run, Run},
    Instances,
};
use crate::{
    envelope::Envelope,
    rng::Rng,
    surface::AttachmentSurface,
    tree::{NodeKind, Tree},
    Error, Result,
};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CanopyParams {
    /// Wood at or below this fraction of the root radius bears foliage of its
    /// own, beside whatever the twig layer marks. Zero leaves the twigs alone
    /// with it; without a twig layer it is what selects the terminal shoots.
    pub shoot_radius: f64,
    pub spacing: f64,
    pub divergence: f64,
    pub clump: u32,
    pub clump_span: f64,
    pub outward: f64,
    pub upward: f64,
    /// Lean along the shoot, as a fraction of the radial off the wood.
    pub forward_lean: f64,
    /// Further lean along the shoot on radials that face upward.
    pub lean_rise: f64,
    /// The station sits on the shoot axis at 0 and on the wood's own contact
    /// surface at 1; the surface is built whenever it is positive.
    pub surface_contact: f64,
    pub scatter: f64,
    pub size: f64,
    pub size_variation: f64,
    /// Hard total budget. Exceeding it returns an error, never partial foliage.
    pub max_instances: usize,
}
impl Default for CanopyParams {
    fn default() -> Self {
        Self {
            shoot_radius: 0.,
            spacing: 0.006,
            divergence: 137.508,
            clump: 5,
            clump_span: 0.3,
            outward: 0.6,
            upward: 0.35,
            forward_lean: 0.,
            lean_rise: 0.,
            surface_contact: 0.,
            scatter: 18.,
            size: 1.,
            size_variation: 0.35,
            max_instances: usize::MAX,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwigPlacement {
    pub internode_length: f64,
    pub stations_per_internode: u32,
}
impl Default for TwigPlacement {
    fn default() -> Self {
        Self {
            internode_length: 0.02,
            stations_per_internode: 1,
        }
    }
}

/// Leaves sit on the runs the twig layer marks, and on any wood slender enough
/// for shoot_radius. Without a twig layer the terminal runs under that same
/// radius carry them, on height-relative spacing and a tip clump.
pub fn place(
    tree: &Tree,
    envelope: Envelope,
    seed: u32,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
) -> Result<Instances> {
    place_impl(tree, envelope, seed, p, twig, None)
}

/// Seat the foliage on the actual swept polygon, including fork sockets, as far
/// as surface contact asks; at zero contact no surface is built.
pub fn place_on_surface(
    tree: &Tree,
    envelope: Envelope,
    seed: u32,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
    surface: &crate::surface::SurfaceParams,
) -> Result<Instances> {
    if p.surface_contact <= 0. {
        return place(tree, envelope, seed, p, twig);
    }
    let contacts = AttachmentSurface::new(tree, envelope.height, surface)?;
    place_impl(tree, envelope, seed, p, twig, Some(&contacts))
}
fn place_impl(
    tree: &Tree,
    envelope: Envelope,
    seed: u32,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
    contacts: Option<&AttachmentSurface>,
) -> Result<Instances> {
    tree.validate_solved()?;
    envelope.validate()?;
    for (v, l, h, n) in [
        (p.shoot_radius, 0., 1., "shoot radius"),
        (p.spacing, 0.001, 1e6, "foliage spacing"),
        (p.divergence, -1e9, 1e9, "divergence"),
        (p.clump_span, 0., 1., "clump span"),
        (p.outward, 0., 1., "outward"),
        (p.upward, 0., 1., "upward"),
        (p.forward_lean, 0., 1., "forward lean"),
        (p.lean_rise, 0., 2., "lean rise"),
        (p.surface_contact, 0., 1., "surface contact"),
        (p.scatter, 0., 90., "scatter"),
        (p.size, 0., 1000., "foliage size"),
        (p.size_variation, 0., 0.9, "size variation"),
    ] {
        range(v, l, h, n)?;
    }
    if p.clump > 64 {
        return Err(Error::InvalidInput("foliage clump"));
    }
    if let Some(t) = twig {
        range(t.internode_length, 1e-6, 1e6, "twig internode")?;
        if !(1..=64).contains(&t.stations_per_internode) {
            return Err(Error::InvalidInput("twig stations"));
        }
    }
    if tree.nodes.len() < 2 || p.size == 0. {
        return Ok(Instances::default());
    }
    // Bound geometry before length arithmetic and float32 conversion.
    if tree.nodes.iter().any(|n| {
        [
            n.position.x,
            n.position.y,
            n.position.z,
            n.radius,
            n.start_radius,
        ]
        .iter()
        .any(|v| v.abs() > f32::MAX as f64 / 4.)
    }) {
        return Err(Error::ResourceLimit("foliage coordinate range"));
    }
    let mut out = Instances::default();
    let mut rng = Rng::new(seed ^ 0x2c9e1a7f);
    let runs = match twig {
        Some(_) => bearing_runs(tree, p),
        None => shoots(tree, tree.nodes[0].radius * p.shoot_radius),
    };
    for nodes in runs {
        place_run(
            &Run {
                tree,
                nodes: &nodes,
                envelope,
                params: p,
                twig,
                contacts,
            },
            &mut rng,
            &mut out,
        )?;
    }
    Ok(out)
}
fn shoots(tree: &Tree, max_radius: f64) -> Vec<Vec<usize>> {
    let n = tree.nodes.len();
    let mut stands = vec![0; n];
    let mut children = vec![Vec::new(); n];
    for i in 1..n {
        let at = stands[tree.nodes[i].parent.unwrap() as usize];
        if tree.nodes[i].position.distance(tree.nodes[at].position) > 1e-9 {
            stands[i] = i;
            children[at].push(i)
        } else {
            stands[i] = at
        }
    }
    let leader = |at: usize| {
        let mut best = children[at][0];
        for &c in &children[at][1..] {
            if tree.nodes[c].start_radius > tree.nodes[best].start_radius {
                best = c;
            }
        }
        best
    };
    let mut seeds = Vec::new();
    if !children[0].is_empty() {
        let first = leader(0);
        seeds.push((0, first));
        for &c in &children[0] {
            if c != first {
                seeds.push((0, c));
            }
        }
    }
    let mut found = Vec::new();
    let mut s = 0;
    while s < seeds.len() {
        let (attach, first) = seeds[s];
        s += 1;
        let mut run = vec![attach, first];
        let mut at = first;
        while !children[at].is_empty() {
            let next = leader(at);
            for &c in &children[at] {
                if c != next {
                    seeds.push((at, c));
                }
            }
            run.push(next);
            at = next;
        }
        if tree.nodes[at].radius > max_radius {
            continue;
        }
        let last = run.len() - 1;
        let mut first = last;
        while first > 0 && tree.nodes[run[first - 1]].radius <= max_radius {
            first -= 1;
        }
        if first == last {
            first -= 1;
        }
        found.push(run.split_off(first));
    }
    found
}

/// Every unbranched run of leaf-bearing wood: what the twig layer marked, plus
/// whatever else is slender enough for shoot_radius to clothe.
fn bearing_runs(tree: &Tree, p: CanopyParams) -> Vec<Vec<usize>> {
    let slender = tree.nodes[0].radius * p.shoot_radius;
    let bearing = |i: usize| {
        let n = &tree.nodes[i];
        n.parent.is_some()
            && (n.kind == NodeKind::Twig
                || (slender > 0. && n.radius.max(n.start_radius) <= slender))
    };
    let mut children = vec![Vec::new(); tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        if bearing(i) {
            if let Some(parent) = n.parent {
                children[parent as usize].push(i);
            }
        }
    }
    let continues = |parent: usize, child: usize| {
        bearing(parent)
            && tree.nodes[parent].branch == tree.nodes[child].branch
            && children[parent].len() == 1
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
        while children[at].len() == 1 && continues(at, children[at][0]) {
            at = children[at][0];
            run.push(at);
        }
        runs.push(run);
    }
    runs
}
