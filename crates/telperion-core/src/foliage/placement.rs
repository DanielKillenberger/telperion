//! Leaf placement: every station on every bearing run, as a transform.
use super::{
    canopy::validate,
    clumping,
    plan::bearing_runs,
    rosette, short_shoots,
    station::{place_run, reserve_all, station_count, walk, Run},
    CanopyParams, Instances, Reference, TwigPlacement,
};
use crate::{envelope::Envelope, rng::Rng, surface::AttachmentSurface, tree::Tree, Error, Result};
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
    place_on(tree, envelope, seed, p, twig, None, reference)
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
    place_on(tree, envelope, seed, p, twig, Some(&contacts), reference)
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

/// Places on a contact surface swept once for the request, `None` where the
/// family seats no leaf on the wood.
pub(crate) fn place_on(
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
