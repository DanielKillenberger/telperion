use super::{
    clumping, range, rosette, short_shoots,
    station::{place_run, reserve_all, station_count, walk, Run},
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
    /// Metres between leaves along a shoot, as a share of the tree's
    /// height. Raising it spreads the leaves further apart, so the crown
    /// carries fewer of them.
    pub spacing: f64,
    /// The degrees each successive leaf is turned around its shoot.
    /// Raising it turns the next leaf further round, so the leaves spiral
    /// differently.
    pub divergence: f64,
    /// How many extra leaves are gathered at the end of a shoot that has
    /// no twig layer. Raising it packs a denser tuft at the tip.
    pub clump: u32,
    /// How far back from the tip that tuft is scattered, as a share of the
    /// shoot's length. Raising it spreads the tuft further down the shoot.
    pub clump_span: f64,
    /// How far a leaf turns away from the trunk. Raising it points the
    /// leaves outward, away from the tree's axis.
    pub outward: f64,
    /// How far a leaf turns toward the sky. Raising it tips the leaves up.
    pub upward: f64,
    /// Lean along the shoot, as a fraction of the radial off the wood.
    pub forward_lean: f64,
    /// Further lean along the shoot on radials that face upward.
    pub lean_rise: f64,
    /// The station sits on the shoot axis at 0 and on the wood's own contact
    /// surface at 1; the surface is built whenever it is positive.
    pub surface_contact: f64,
    /// The degrees a leaf may be turned at random from where it was
    /// placed. Raising it leaves the crown less combed.
    pub scatter: f64,
    /// The size every leaf is drawn at, as a multiple of the element's own
    /// dimensions. Raising it enlarges every leaf.
    pub size: f64,
    /// How far leaf size varies leaf to leaf, as a share of that size.
    /// Raising it mixes larger and smaller leaves more widely.
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
    /// How deep a lateral may be and still start a limb system of its own.
    /// Raising it parts the crown into more and smaller leaf masses; it
    /// does nothing until `limb_clumping` is above zero.
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
    /// Fronds the rosette bears at the apex of each stem. At zero no rosette
    /// stands and the canopy clothes wood as it always did; any rise makes the
    /// rosette the tree's only foliage.
    #[cfg_attr(feature = "json", serde(default))]
    pub rosette_fronds: u32,
    /// The degrees each successive frond is turned about the apex.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_rosette_divergence")
    )]
    pub rosette_divergence: f64,
    /// Degrees from the axis the youngest frond stands: 0 upright, 90 level,
    /// 180 hanging.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_rosette_pitch")
    )]
    pub rosette_pitch: f64,
    /// How many degrees further than the youngest the oldest frond leans, so
    /// the crown opens from a spike to a skirt.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_rosette_pitch_spread")
    )]
    pub rosette_pitch_spread: f64,
    /// Metres below the apex the frond insertions are spread down the axis. At
    /// zero every frond leaves one point.
    #[cfg_attr(feature = "json", serde(default))]
    pub rosette_depth: f64,
    /// Leaflets one placement carries along its rachis. One is the single
    /// blade every family drew.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_leaflet_count")
    )]
    pub leaflet_count: u32,
    /// Metres of rachis the leaflets are strung along. At zero the placement
    /// is one blade whatever the count says.
    #[cfg_attr(feature = "json", serde(default))]
    pub rachis_length: f64,
    /// The degrees a leaflet leaves its rachis.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_leaflet_pitch")
    )]
    pub leaflet_pitch: f64,
    /// How far the rachis bends out of the straight line from its station, as
    /// a share of its length. Positive arches up, negative droops.
    #[cfg_attr(feature = "json", serde(default))]
    pub rachis_arch: f64,
    /// Whether a single leaflet closes the rachis's end, blended 0 to 1: the
    /// last leaflet turns from standing off the rachis to lying along it.
    #[cfg_attr(feature = "json", serde(default))]
    pub terminal_leaflet: f64,
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
            // Neutral: no rosette stands and no placement groups until a
            // table states a frond count and a rachis to string leaflets on.
            rosette_fronds: 0,
            rosette_divergence: crate::ranges::default_rosette_divergence(),
            rosette_pitch: crate::ranges::default_rosette_pitch(),
            rosette_pitch_spread: crate::ranges::default_rosette_pitch_spread(),
            rosette_depth: 0.,
            leaflet_count: crate::ranges::default_leaflet_count(),
            rachis_length: 0.,
            leaflet_pitch: crate::ranges::default_leaflet_pitch(),
            rachis_arch: 0.,
            terminal_leaflet: 0.,
            max_instances: usize::MAX,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct TwigPlacement {
    /// Metres between the leaf stations along a twig, copied from the twig
    /// layer's own internode length. Raising it spaces the leaves further
    /// apart along the shoot.
    pub internode_length: f64,
    /// How many leaf stations sit at each of those joints, spread around
    /// the shoot. Raising it crowds more leaves onto each joint.
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
impl TwigPlacement {
    /// The rows a family's own twig table states, resolved: what a caller
    /// clothing that family hands `place`, and what the prediction counts by.
    pub fn of(family: &crate::presets::Family) -> Result<Self> {
        let twig = family.skeleton.twigs.resolved()?.twig;
        Ok(Self {
            internode_length: twig.internode_length,
            stations_per_internode: twig.stations_per_internode,
        })
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
/// Everything both the builder and the count check before a leaf is placed,
/// and whether this tree and this family bear any at all.
fn bearing(
    tree: &Tree,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
) -> Result<bool> {
    validate(tree, envelope, p, twig)?;
    if tree.nodes.len() < 2 || p.size == 0. {
        return Ok(false);
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
    Ok(true)
}

/// Every unbranched run of leaf-bearing wood this family clothes: what the
/// twig layer marked, or the terminal shoots slender enough for `shoot_radius`
/// where there is no twig layer.
fn runs(tree: &Tree, p: CanopyParams, twig: Option<TwigPlacement>) -> Vec<Vec<usize>> {
    // A stem that bears a frond crown bears nothing along its length.
    if rosette::bearing(&p) {
        return Vec::new();
    }
    match twig {
        Some(_) => bearing_runs(tree, p),
        None => shoots(
            tree,
            tree.stem_radius(|i| tree.nodes[i].radius) * p.shoot_radius,
        ),
    }
}

/// Leaves these runs and this family's short shoots place, before any cull.
fn leaves_on(
    tree: &Tree,
    envelope: Envelope,
    seed: u32,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
    runs: &[Vec<usize>],
) -> Result<usize> {
    let overflow = || Error::ResourceLimit("foliage count overflow");
    let (mut points, mut along) = (Vec::new(), Vec::new());
    let leaflets = rosette::leaflets(&p);
    let mut total = if rosette::bearing(&p) {
        rosette::count(tree, &p)?
    } else {
        short_shoots::count(tree, envelope, seed, &p)?
    };
    for nodes in runs {
        let length = walk(tree, nodes, &mut points, &mut along);
        total = station_count(length, envelope, p, twig)?
            .checked_mul(leaflets)
            .and_then(|leaves| total.checked_add(leaves))
            .ok_or_else(overflow)?;
    }
    Ok(total)
}

/// How many leaves this family places on this tree, before any cull. `place`
/// reserves exactly this many, so a finished crown's capacity is this count,
/// and a prediction reads it without placing a leaf or building a matrix.
pub(crate) fn leaf_count(
    tree: &Tree,
    envelope: Envelope,
    seed: u32,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
) -> Result<usize> {
    if !bearing(tree, envelope, p, twig)? {
        return Ok(0);
    }
    leaves_on(tree, envelope, seed, p, twig, &runs(tree, p, twig))
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
    if !bearing(tree, envelope, p, twig)? {
        return Ok(Instances::new(reference));
    }
    let mut out = Instances::new(reference);
    let runs = runs(tree, p, twig);
    // One reservation for the whole crown, to the count the prediction reads:
    // every later reserve finds the room already there, so the vector's
    // capacity is that count and nothing the cull or the clumping retains
    // holds a block larger than the specimen was said to cost.
    reserve_all(
        &mut out,
        leaves_on(tree, envelope, seed, p, twig, &runs)?,
        p,
    )?;
    let mut rng = Rng::new(seed ^ 0x2c9e1a7f);
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
    // The second source: a rosette at every stem apex where the rows state
    // one, else short shoots over the limbs and branches. Either draws from
    // its own wood's stream, so the leaves above keep every byte.
    if rosette::bearing(&p) {
        rosette::clothe(tree, seed, &p, &mut out, owners.as_mut())?;
    } else {
        short_shoots::clothe(tree, envelope, seed, &p, &mut out, owners.as_mut())?;
    }
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
    rosette::validate(&p)?;
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
