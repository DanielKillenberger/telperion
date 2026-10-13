//! A short shoot's cluster (fn-223): the leaves an engine axis of an age
//! that states one bears at its own tip, fanned level about the tip's
//! bearing, in place of the leaves it would bear along its length. Leaf
//! `j` of a cluster draws at a station's ordinals on its tip's cluster
//! stream (`tree::cluster_key`), so both executors seat the same leaves
//! whatever order they visit the tips in.
use super::{rosette, CanopyParams};
#[cfg(feature = "geometry")]
use super::{station, Instances};
use crate::{
    math::Vec3,
    tree::{Cluster, Tree},
    Error, Result,
};
#[cfg(feature = "geometry")]
use crate::{
    rng::Rng,
    tree::{cluster_key, leaf_stream},
};

/// The clusters this family seats on this tree: every short shoot's tip,
/// none on a leafless canopy or a frond crown, which bears nothing along
/// its wood.
pub(super) fn of<'a>(tree: &'a Tree, p: &CanopyParams) -> &'a [Cluster] {
    if p.size == 0.0 || rosette::bearing(p) {
        return &[];
    }
    &tree.clusters
}

/// Leaves the clusters place, each placement its leaflets.
pub(super) fn count(tree: &Tree, p: &CanopyParams) -> Result<usize> {
    let leaflets = rosette::leaflets(p);
    of(tree, p).iter().try_fold(0usize, |total, c| {
        (c.leaves as usize)
            .checked_mul(leaflets)
            .and_then(|leaves| total.checked_add(leaves))
            .ok_or(Error::ResourceLimit("foliage count overflow"))
    })
}

/// The tip's laid heading, where the engine laid it, else its chord.
pub(super) fn heading(tree: &Tree, node: usize) -> Vec3 {
    let laid = tree
        .frames
        .get(node)
        .map(|f| Vec3::new(f[0].into(), f[1].into(), f[2].into()));
    let chord = || {
        let n = &tree.nodes[node];
        n.position - tree.nodes[n.parent.unwrap_or(0) as usize].position
    };
    let h = laid.unwrap_or_else(chord);
    if h.length_squared() > 1e-24 {
        h.normalized()
    } else {
        Vec3::Y
    }
}

/// Leaf `j`'s radial in a cluster of `leaves` about `heading`: the level
/// bearing of the heading (+X where it stands vertical), turned about +Y
/// by the spread either way, evenly; a lone leaf keeps the bearing.
pub(super) fn radial(heading: Vec3, leaves: u32, spread: f64, j: u32) -> Vec3 {
    let flat = Vec3::new(heading.x, 0.0, heading.z);
    let bearing = if flat.length_squared() > 1e-12 {
        flat.normalized()
    } else {
        Vec3::X
    };
    let across = if leaves > 1 {
        spread * (2.0 * f64::from(j) / f64::from(leaves - 1) - 1.0)
    } else {
        0.0
    };
    bearing.rotate(Vec3::Y, across)
}

#[cfg(feature = "geometry")]
/// Seats every cluster after the placements already made, tip by tip in
/// node order; `owners` keeps each leaf's tip where limb systems clump.
pub(super) fn clothe(
    tree: &Tree,
    seed: u32,
    p: CanopyParams,
    out: &mut Instances,
    mut owners: Option<&mut Vec<u32>>,
) -> Result<()> {
    let leaflets = rosette::leaflets(&p);
    for c in of(tree, &p) {
        let tip = c.node as usize;
        station::reserve(out, c.leaves as usize * leaflets, p)?;
        let at = tree.nodes[tip].position;
        let heading = heading(tree, tip);
        let mut rng = Rng::new(leaf_stream(seed, cluster_key(tree.key(tip))));
        // Leaves still expanding, or falling, are drawn at their share
        // (fn-270 R5, R5c), each passing its lifespan by its draw on the
        // cluster's stream.
        let (unfolded, key) = (tree.unfolding(tip), cluster_key(tree.key(tip)));
        for j in 0..c.leaves {
            let radial = radial(heading, c.leaves, c.spread, j);
            let axis = station::axis(at, radial, heading, p);
            let size = p.size * unfolded.leaf(true, key, j).0;
            rosette::fan(at, axis, heading, radial, CanopyParams { size, ..p }, &mut rng, out)?;
        }
        if let Some(owners) = owners.as_mut() {
            owners.resize(out.leaves.len(), c.node);
        }
    }
    Ok(())
}
