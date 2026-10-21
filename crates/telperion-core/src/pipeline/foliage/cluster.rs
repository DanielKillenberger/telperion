//! A short shoot's cluster (fn-223): the leaves an engine axis of an age
//! that states one bears at its own tip, fanned level about the tip's
//! bearing, in place of the leaves it would bear along its length. Leaf
//! `j` of a cluster draws at a station's ordinals on its tip's cluster
//! stream (`tree::cluster_key`), so both executors seat the same leaves
//! whatever order they visit the tips in.
use super::{organs, CanopyParams};
#[cfg(feature = "geometry")]
use super::{rosette, station, Instances};
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
use telperion_space::organ;

/// The clusters this family seats on this tree: every short shoot's tip
/// that keeps its leaves at the cauline share (fn-277), none on a
/// leafless canopy.
pub(super) fn of<'a>(tree: &'a Tree, p: &CanopyParams) -> impl Iterator<Item = &'a Cluster> {
    let (cauline, leafless) = (p.cauline, p.size == 0.0);
    let kept = move |c: &&Cluster| {
        telperion_space::organs(cauline, cluster_tip(tree, c), organ::CAULINE) == 1
    };
    tree.clusters.iter().filter(move |_| !leafless).filter(kept)
}

/// What keys a cluster's draws: its tip's cluster key.
fn cluster_tip(tree: &Tree, c: &Cluster) -> u32 {
    crate::tree::cluster_key(tree.key(c.node as usize))
}

/// The table leaf `j` of the cluster keyed `key` is drawn by.
pub(super) fn placement(p: &CanopyParams, key: u32, j: u32) -> CanopyParams {
    organs::placed(p, key, j)
}

/// Leaves one cluster places, each of its leaves its leaflets.
pub(super) fn leaves(tree: &Tree, p: &CanopyParams, c: &Cluster) -> u32 {
    let key = cluster_tip(tree, c);
    (0..c.leaves)
        .map(|j| organs::leaflets(&placement(p, key, j)) as u32)
        .sum()
}

/// Leaves the clusters place.
pub(super) fn count(tree: &Tree, p: &CanopyParams) -> Result<usize> {
    let overflow = || Error::ResourceLimit("foliage count overflow");
    of(tree, p).try_fold(0usize, |total, c| {
        total
            .checked_add(leaves(tree, p, c) as usize)
            .ok_or_else(overflow)
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
    for c in of(tree, &p) {
        let tip = c.node as usize;
        let at = tree.nodes[tip].position;
        let heading = heading(tree, tip);
        let mut rng = Rng::new(leaf_stream(seed, cluster_key(tree.key(tip))));
        // Leaves still expanding are drawn at their share, falling ones
        // whole until each one's abscission day (fn-270 R5, R5c), each
        // passing its lifespan by its draw on the cluster's stream.
        let (unfolded, key) = (tree.unfolding(tip), cluster_key(tree.key(tip)));
        for j in 0..c.leaves {
            let radial = radial(heading, c.leaves, c.spread, j);
            let axis = station::axis(at, radial, heading, p);
            let (share, colour) = unfolded.leaf(true, key, j);
            let from = out.leaves.len();
            let table = placement(
                &CanopyParams {
                    size: p.size * share,
                    ..p
                },
                key,
                j,
            );
            station::reserve(out, organs::leaflets(&table), p)?;
            rosette::fan(
                at,
                axis,
                heading,
                radial,
                (table, organs::leaflets(&table), None, 0.0),
                (&mut rng, None),
                out,
            )?;
            // A colouring leaf is drawn as one withering (fn-270 R5c).
            if colour > 0.0 {
                out.wither(from);
            }
        }
        if let Some(owners) = owners.as_mut() {
            owners.resize(out.leaves.len(), c.node);
        }
    }
    Ok(())
}
