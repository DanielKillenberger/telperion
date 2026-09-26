//! One shoot's stations: where each leaf sits along the run, how far it leans
//! off the wood, and the frame the renderer receives.
use super::{CanopyParams, Instances, TwigPlacement};
use crate::math::Transcendental;
use crate::{
    envelope::Envelope, math::Vec3, pipeline::surface::AttachmentSurface, rng::Rng, tree::Tree,
    Error, Result,
};
use std::f64::consts::{PI, TAU};

/// One shoot the canopy clothes, and everything a station on it reads.
#[derive(Clone, Copy)]
pub(super) struct Run<'a> {
    pub tree: &'a Tree,
    /// Node indices, attachment first, along one unbranched shoot.
    pub nodes: &'a [usize],
    pub envelope: Envelope,
    pub params: CanopyParams,
    /// Present when the twig layer marks the run; absent on a bare shoot.
    pub twig: Option<TwigPlacement>,
    /// The wood's contact surface, built whenever surface contact is positive.
    pub contacts: Option<&'a AttachmentSurface<'a>>,
}

/// The points one run passes through and the distance along it to each, into
/// scratch the caller owns; the run's own length is the last of them. The
/// builder sweeps this walk and the count reads it, so a run is measured by
/// one rule however many times it is read.
pub(super) fn walk(
    tree: &Tree,
    nodes: &[usize],
    points: &mut Vec<Vec3>,
    along: &mut Vec<f64>,
) -> f64 {
    points.clear();
    points.extend(nodes.iter().map(|i| tree.nodes[*i].position));
    along.clear();
    along.push(0.);
    for i in 1..points.len() {
        along.push(along[i - 1] + points[i].distance(points[i - 1]));
    }
    *along.last().unwrap()
}

pub(super) fn place_run(run: &Run, rng: &mut Rng, out: &mut Instances) -> Result<()> {
    let Run {
        tree,
        nodes,
        envelope,
        params: p,
        twig,
        contacts,
    } = *run;
    let (mut points, mut along) = (Vec::new(), Vec::new());
    let length = walk(tree, nodes, &mut points, &mut along);
    let stations = stations(length, envelope, p, twig, rng)?;
    if stations.is_empty() {
        return Ok(());
    }
    let frames = station_frames(&points, &along);
    let leaflets = super::rosette::leaflets(&p);
    reserve(
        out,
        stations
            .len()
            .checked_mul(leaflets)
            .ok_or(Error::ResourceLimit("foliage count overflow"))?,
        p,
    )?;
    for (k, distance) in stations.into_iter().enumerate() {
        let mut segment = points.len() - 2;
        while segment > 0 && along[segment] > distance {
            segment -= 1;
        }
        let span = along[segment + 1] - along[segment];
        let t = if span > 1e-12 {
            (distance - along[segment]) / span
        } else {
            0.
        };
        let centre = points[segment].lerp(points[segment + 1], t);
        let distal = &tree.nodes[nodes[segment + 1]];
        let base = if twig.is_some() {
            distal.start_radius
        } else {
            tree.nodes[nodes[segment]].radius
        };
        let wood = base * (1. - t) + distal.radius * t;
        let (tangent, normal, binormal) = frames[segment];
        let turn = if let Some(a) = twig {
            (k / a.stations_per_internode as usize) as f64 * p.divergence * PI / 180.
                + (k % a.stations_per_internode as usize) as f64 * TAU
                    / a.stations_per_internode as f64
        } else {
            k as f64 * p.divergence * PI / 180.
        };
        let (sin, cos) = turn.sin_cos_fixed();
        let radial = normal * cos + binormal * sin;
        // The station sits on the shoot axis at contact 0 and on the wood's
        // own surface at 1; between them it walks out along the same radial.
        let mut point = centre + radial * wood;
        if let Some(contacts) = contacts {
            let seat = contacts
                .point(nodes[segment + 1], centre, radial, wood)
                .ok_or(Error::InvalidInput(
                    "foliage surface contact projection missed",
                ))?;
            point = point * (1. - p.surface_contact) + seat * p.surface_contact;
        }
        super::rosette::fan(
            point,
            axis(point, radial, tangent, p),
            tangent,
            normal,
            p,
            rng,
            out,
        )?;
    }
    Ok(())
}

/// Metres between leaves on a run no twig layer marks: the row's share of the
/// tree's own height, never finer than a tenth of a millimetre.
fn spacing(envelope: Envelope, p: CanopyParams) -> Result<f64> {
    let spacing = p.spacing * envelope.height;
    if !spacing.is_finite() || spacing <= 0.0 {
        return Err(Error::InvalidInput("foliage spacing"));
    }
    Ok(spacing)
}

/// The most stations a run may hold before the vector that carries them would
/// outgrow what an index can address.
fn addressable() -> f64 {
    (isize::MAX as usize / std::mem::size_of::<f64>()) as f64
}

/// How many leaves a run of this length carries: one set per internode under a
/// twig layer, height-relative spacing plus a terminal clump without one. The
/// builder places exactly this many and the prediction counts them, from here.
pub(super) fn station_count(
    length: f64,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
) -> Result<usize> {
    if length == 0. {
        return Ok(0);
    }
    if !length.is_finite() {
        return Err(Error::ResourceLimit("shoot length overflow"));
    }
    if let Some(t) = twig {
        let per = t.stations_per_internode as f64;
        let internodes = (length / t.internode_length - 1e-9).ceil().max(1.);
        if !internodes.is_finite()
            || internodes * per > p.max_instances as f64
            || internodes * per >= addressable()
        {
            return Err(Error::ResourceLimit("foliage instance budget"));
        }
        return Ok(internodes as usize * t.stations_per_internode as usize);
    }
    let count = (length / spacing(envelope, p)?).ceil();
    if !count.is_finite()
        || count + p.clump as f64 > p.max_instances as f64
        || count + p.clump as f64 >= addressable()
    {
        return Err(Error::ResourceLimit("foliage instance budget"));
    }
    Ok(count as usize + p.clump as usize)
}

/// Distances along the run at which those leaves sit.
fn stations(
    length: f64,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
    rng: &mut Rng,
) -> Result<Vec<f64>> {
    let total = station_count(length, envelope, p, twig)?;
    let mut stations = Vec::new();
    if total == 0 {
        return Ok(stations);
    }
    // The count is the builder's and the prediction's alike, and the block it
    // asks for is asked for once and may be refused.
    stations
        .try_reserve_exact(total)
        .map_err(|_| Error::ResourceLimit("foliage allocation"))?;
    if let Some(t) = twig {
        let per = t.stations_per_internode as usize;
        for i in 0..total / per {
            for _ in 0..per {
                stations.push(i as f64 * t.internode_length);
            }
        }
        return Ok(stations);
    }
    let spacing = spacing(envelope, p)?;
    for i in 0..total - p.clump as usize {
        stations.push(i as f64 * spacing);
    }
    for _ in 0..p.clump {
        stations.push(length * (1. - p.clump_span * rng.next_f64()));
    }
    Ok(stations)
}

/// The row's hard total, and the block the leaves would need: a crown past
/// either is refused rather than half built.
fn budget(out: &Instances, more: usize, p: CanopyParams) -> Result<()> {
    let total = out
        .leaves
        .len()
        .checked_add(more)
        .ok_or(Error::ResourceLimit("foliage count overflow"))?;
    if total
        .checked_mul(size_of::<super::Leaf>())
        .is_none_or(|bytes| bytes > isize::MAX as usize)
        || total > p.max_instances
    {
        return Err(Error::ResourceLimit("foliage instance budget"));
    }
    Ok(())
}

/// Room for `stations` more leaves. A crown `place` has already sized finds
/// the room there and grows nothing; a caller placing into a crown of its own,
/// a shoot at a time, grows it as a vector grows.
pub(super) fn reserve(out: &mut Instances, stations: usize, p: CanopyParams) -> Result<()> {
    budget(out, stations, p)?;
    out.leaves
        .try_reserve(stations)
        .map_err(|_| Error::ResourceLimit("foliage allocation"))
}

/// The whole crown in one block, exactly the count the prediction reads: the
/// capacity a finished specimen keeps is then that count, whatever the cull
/// and the clumping later retain inside it.
pub(super) fn reserve_all(out: &mut Instances, leaves: usize, p: CanopyParams) -> Result<()> {
    budget(out, leaves, p)?;
    out.leaves
        .try_reserve_exact(leaves)
        .map_err(|_| Error::ResourceLimit("foliage allocation"))
}

/// The leaf's own axis: the radial off the wood, leaned along the shoot by
/// forward lean and by lean rise where the radial faces up, then carried
/// outward from the trunk and upward by the canopy's own terms.
pub(super) fn axis(point: Vec3, radial: Vec3, tangent: Vec3, p: CanopyParams) -> Vec3 {
    let outward = Vec3::new(point.x, 0., point.z);
    let mut axis = radial + tangent * (p.forward_lean + p.lean_rise * radial.y.max(0.));
    if outward.length_squared() > 1e-12 {
        axis += outward.normalized() * p.outward;
    }
    axis.y += p.upward;
    if axis.length_squared() <= 1e-12 {
        axis = radial;
    }
    axis.normalized()
}

pub(super) fn matrix(
    point: Vec3,
    axis: Vec3,
    tangent: Vec3,
    normal: Vec3,
    p: CanopyParams,
    rng: &mut Rng,
) -> Result<[f32; 16]> {
    let ([side, axis, face], scale) = super::leaflet::drawn(axis, tangent, normal, &p, rng);
    let matrix = [
        side.x * scale,
        side.y * scale,
        side.z * scale,
        0.,
        axis.x * scale,
        axis.y * scale,
        axis.z * scale,
        0.,
        face.x * scale,
        face.y * scale,
        face.z * scale,
        0.,
        point.x,
        point.y,
        point.z,
        1.,
    ]
    .map(|v| v as f32);
    if !matrix.iter().all(|v| v.is_finite()) {
        return Err(Error::ResourceLimit("foliage transform overflow"));
    }
    Ok(matrix)
}

pub(super) fn station_frames(points: &[Vec3], along: &[f64]) -> Vec<(Vec3, Vec3, Vec3)> {
    station_frame_iter(points, along).collect()
}

pub(super) fn station_frame_iter<'a>(
    points: &'a [Vec3],
    along: &'a [f64],
) -> impl Iterator<Item = (Vec3, Vec3, Vec3)> + 'a {
    let mut previous_segment = None;
    let mut previous_tangent = None;
    let mut carried_normal = Vec3::ZERO;
    points.windows(2).enumerate().map(move |(i, pair)| {
        let delta = pair[1] - pair[0];
        let segment = if delta.length_squared() > 0. {
            delta.normalized()
        } else {
            previous_segment.unwrap_or(Vec3::Y)
        };
        let tangent = previous_segment.map_or(segment, |previous: Vec3| {
            let sum = previous + segment;
            if sum.length_squared() > 1e-9 {
                sum.normalized()
            } else {
                segment
            }
        });
        carried_normal = previous_tangent.map_or_else(
            || tangent.perpendicular(),
            |previous| transport_normal(carried_normal, previous, tangent),
        );
        carried_normal -= tangent * carried_normal.dot(tangent);
        if carried_normal.length_squared() <= 1e-9 {
            carried_normal = tangent.perpendicular();
        }
        carried_normal = carried_normal.normalized();
        previous_segment = Some(segment);
        previous_tangent = Some(tangent);
        let mut output = (
            tangent,
            carried_normal,
            tangent.cross(carried_normal).normalized(),
        );
        let span = along[i + 1] - along[i];
        if span > 1e-12 {
            output.0 = delta / span;
            output.1 -= output.0 * output.1.dot(output.0);
            if output.1.length_squared() <= 1e-12 {
                output.1 = output.0.perpendicular();
            }
            output.1 = output.1.normalized();
            output.2 = output.0.cross(output.1).normalized();
        }
        output
    })
}

fn transport_normal(normal: Vec3, previous: Vec3, tangent: Vec3) -> Vec3 {
    let cross = previous.cross(tangent);
    let dot = previous.dot(tangent).clamp(-1., 1.);
    if dot < -1. + f64::EPSILON {
        let axis = if previous.x.abs() > previous.z.abs() {
            Vec3::new(-previous.y, previous.x, 0.)
        } else {
            Vec3::new(0., -previous.z, previous.y)
        };
        normal.rotate(axis.normalized(), PI)
    } else if cross.length_squared() > 0. {
        let sin = cross.length();
        let axis = cross / sin;
        let norm = (sin * sin + dot * dot).sqrt();
        if dot < -0.999999 || !norm.is_finite() || norm == 0. {
            normal.rotate(axis, sin.atan2_fixed(dot))
        } else {
            normal.rotate_sin_cos(axis, (sin / norm, dot / norm))
        }
    } else {
        normal
    }
}

#[cfg(test)]
/// A rotation-minimising frame at every point of the run.
fn frames(points: &[Vec3]) -> Vec<(Vec3, Vec3, Vec3)> {
    let mut segments = Vec::new();
    for w in points.windows(2) {
        let d = w[1] - w[0];
        segments.push(if d.length_squared() > 0. {
            d.normalized()
        } else {
            *segments.last().unwrap_or(&Vec3::Y)
        });
    }
    let mut tangents = vec![segments[0]];
    for w in segments.windows(2) {
        let d = w[0] + w[1];
        tangents.push(if d.length_squared() > 1e-9 {
            d.normalized()
        } else {
            w[1]
        });
    }
    tangents.push(*segments.last().unwrap());
    let mut normal = tangents[0].perpendicular();
    let mut result = Vec::new();
    for (i, &t) in tangents.iter().enumerate() {
        if i > 0 {
            let prev = tangents[i - 1];
            let cross = prev.cross(t);
            let dot = prev.dot(t).clamp(-1., 1.);
            if dot < -1. + f64::EPSILON {
                let a = if prev.x.abs() > prev.z.abs() {
                    Vec3::new(-prev.y, prev.x, 0.)
                } else {
                    Vec3::new(0., -prev.z, prev.y)
                };
                normal = normal.rotate(a.normalized(), PI);
            } else if cross.length_squared() > 0. {
                normal = normal.rotate(cross.normalized(), cross.length().atan2_fixed(dot));
            }
        }
        normal -= t * normal.dot(t);
        if normal.length_squared() <= 1e-9 {
            normal = t.perpendicular();
        }
        normal = normal.normalized();
        result.push((t, normal, t.cross(normal).normalized()));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streamed_frames_match_canonical_transport_and_projection() {
        for points in [
            vec![Vec3::ZERO, Vec3::Y, Vec3::Y, Vec3::ZERO, Vec3::X],
            vec![Vec3::ZERO, Vec3::new(1e-14, 0., 0.), Vec3::new(1., 2., -3.)],
            (0..128)
                .map(|i| {
                    let t = i as f64 * 0.17;
                    Vec3::new(t.cos_fixed(), t * 0.2, t.sin_fixed())
                })
                .collect(),
            vec![
                Vec3::ZERO,
                Vec3::new(1e30, 0., 0.),
                Vec3::new(1e30, 1e30, 0.),
            ],
            vec![Vec3::ZERO, Vec3::Y, Vec3::new(1e-8, 0., 0.), Vec3::X],
        ] {
            let mut along = vec![0.];
            for p in points.windows(2) {
                along.push(along.last().unwrap() + p[1].distance(p[0]));
            }
            let actual = station_frames(&points, &along);
            assert_eq!(actual.len(), points.len() - 1);
            let original = frames(&points);
            for segment in 0..points.len() - 1 {
                let (mut tangent, mut normal, mut binormal) = original[segment];
                let span = along[segment + 1] - along[segment];
                if span > 1e-12 {
                    tangent = (points[segment + 1] - points[segment]) / span;
                    normal -= tangent * normal.dot(tangent);
                    if normal.length_squared() <= 1e-12 {
                        normal = tangent.perpendicular();
                    }
                    normal = normal.normalized();
                    binormal = tangent.cross(normal).normalized();
                }
                assert_eq!(actual[segment].0, tangent);
                assert!((actual[segment].1 - normal).length() < 1e-12);
                assert!((actual[segment].2 - binormal).length() < 1e-12);
                let (t, n, b) = actual[segment];
                for axis in [t, n, b] {
                    assert!((axis.length() - 1.).abs() < 1e-12);
                }
                assert!(t.dot(n).abs() < 1e-12);
                assert!((t.cross(n) - b).length() < 1e-12);
            }
        }
    }

    #[test]
    fn algebraic_transport_handles_parallel_antiparallel_and_near_turns() {
        for t in [
            Vec3::Y,
            -Vec3::Y,
            Vec3::new(1e-9, -1., 0.).normalized(),
            Vec3::new(0.3, 0.8, -0.2).normalized(),
        ] {
            let n = transport_normal(Vec3::X, Vec3::Y, t);
            assert!(n.is_finite());
            assert!((n.length() - 1.).abs() < 1e-12);
            assert!(n.dot(t).abs() < 1e-8);
        }
    }
}
