use super::{
    clumping, range, short_shoots,
    station::{place_run, Run},
    Instances, Reference,
};
use crate::{
    envelope::Envelope,
    rng::Rng,
    surface::AttachmentSurface,
    tree::{NodeKind, Tree},
    Error, Result,
};
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
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
    /// Metres between short shoots along limb and branch wood: spurs a few
    /// centimetres long, each ending in a cluster of leaves. Zero grows none.
    pub short_shoot_spacing: f64,
    /// Wood thicker than this fraction of the stem's radius carries no short
    /// shoot, and neither does twig wood or anything below the crown base.
    pub short_shoot_radius: f64,
    /// Metres from the bark to the cluster a short shoot carries.
    pub short_shoot_length: f64,
    /// Leaves in one short shoot's cluster, 1 to 8.
    pub short_shoot_leaves: u32,
    /// Degrees either side of its short shoot's bearing a cluster's leaves
    /// fan across, held level: 90 is a half circle, 0 stacks them.
    pub short_shoot_spread: f64,
    /// How far into each limb system the gap between it and its neighbours
    /// reaches, as a share of the way from their shared boundary to the
    /// system's centre: each limb system then keeps a rounded leaf mass of its
    /// own. Zero, the neutral, thins nothing.
    pub limb_clumping: f64,
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_clump_system_order")
    )]
    pub clump_system_order: u32,
    /// Nearest neighbours and cell crossings in the clumping approximation.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_clump_neighbours")
    )]
    pub clump_neighbours: u32,
    /// Hard total budget. Exceeding it returns an error, never partial foliage.
    #[cfg_attr(feature = "json", serde(with = "crate::specimen::portable::index"))]
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
            // Neutral: no short shoot grows until a table states a spacing.
            // The other four are a beech's spur, so a spacing alone reads.
            short_shoot_spacing: 0.,
            short_shoot_radius: 0.15,
            short_shoot_length: 0.04,
            short_shoot_leaves: 3,
            short_shoot_spread: 45.,
            // Neutral: every leaf the stations and the short shoots place.
            limb_clumping: 0.,
            clump_system_order: crate::ranges::default_clump_system_order(),
            clump_neighbours: crate::ranges::default_clump_neighbours(),
            max_instances: usize::MAX,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
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
    reference: Reference,
) -> Result<Instances> {
    place_impl(tree, envelope, seed, p, twig, None, reference)
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
    reference: Reference,
) -> Result<Instances> {
    if p.surface_contact <= 0. {
        return place(tree, envelope, seed, p, twig, reference);
    }
    let contacts = AttachmentSurface::new(tree, envelope.height, surface)?;
    place_impl(tree, envelope, seed, p, twig, Some(&contacts), reference)
}
fn place_impl(
    tree: &Tree,
    envelope: Envelope,
    seed: u32,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
    contacts: Option<&AttachmentSurface>,
    reference: Reference,
) -> Result<Instances> {
    validate(tree, envelope, p, twig)?;
    if tree.nodes.len() < 2 || p.size == 0. {
        return Ok(Instances::new(reference));
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
    let mut out = Instances::new(reference);
    let mut rng = Rng::new(seed ^ 0x2c9e1a7f);
    let runs = match twig {
        Some(_) => bearing_runs(tree, p),
        None => shoots(
            tree,
            tree.stem_radius(|i| tree.nodes[i].radius) * p.shoot_radius,
        ),
    };
    // Which wood bears each leaf, kept only where limb systems clump.
    let mut owners = (p.limb_clumping > 0.).then(Vec::new);
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
        if let Some(owners) = owners.as_mut() {
            owners.resize(out.leaves.len(), nodes[1] as u32);
        }
    }
    // A second source over the limbs and branches: short shoots draw from
    // their own wood's stream, so the leaves above keep every byte.
    short_shoots::clothe(tree, envelope, seed, &p, &mut out, owners.as_mut())?;
    if let Some(owners) = owners {
        clumping::thin(tree, &owners, seed, p, &mut out);
    }
    Ok(out)
}
pub(super) fn validate(
    tree: &Tree,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
) -> Result<()> {
    tree.validate_solved()?;
    envelope.validate()?;
    for (v, l, h, n) in [
        (p.shoot_radius, 0., 1., "shoot radius"),
        (p.spacing, 0.001, 1e6, "foliage spacing"),
        (p.divergence, -1e9, 1e9, "divergence"),
        (p.clump_span, 0., 1., "clump span"),
        // Signed: a leaf may lean back down its shoot and turn toward the
        // ground as readily as toward the tip and the sky. Zero is still zero,
        // so every row authored before the rails widened is the row it was.
        (p.outward, -1., 1., "outward"),
        (p.upward, -1., 1., "upward"),
        (p.forward_lean, -1., 1., "forward lean"),
        (p.lean_rise, -2., 2., "lean rise"),
        (p.surface_contact, 0., 1., "surface contact"),
        (p.scatter, 0., 90., "scatter"),
        (p.size, 0., 1000., "foliage size"),
        (p.size_variation, 0., 0.9, "size variation"),
        (p.limb_clumping, 0., 1., "limb clumping"),
    ] {
        range(v, l, h, n)?;
    }
    if p.clump > 64 {
        return Err(Error::InvalidInput("foliage clump"));
    }
    short_shoots::validate(&p)?;
    if let Some(t) = twig {
        range(t.internode_length, 1e-6, 1e6, "twig internode")?;
        if !(1..=64).contains(&t.stations_per_internode) {
            return Err(Error::InvalidInput("twig stations"));
        }
    }
    Ok(())
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
pub(super) fn bearing_runs(tree: &Tree, p: CanopyParams) -> Vec<Vec<usize>> {
    let slender = tree.stem_radius(|i| tree.nodes[i].radius) * p.shoot_radius;
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
