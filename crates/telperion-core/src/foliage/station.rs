//! One shoot's stations: where each leaf sits along the run, how far it leans
//! off the wood, and the frame the renderer receives.
use super::{CanopyParams, Instances, TwigPlacement};
use crate::{
    envelope::Envelope, math::Vec3, rng::Rng, surface::AttachmentSurface, tree::Tree, Error, Result,
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
    pub contacts: Option<&'a AttachmentSurface>,
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
    let points: Vec<_> = nodes.iter().map(|i| tree.nodes[*i].position).collect();
    let mut along = vec![0.];
    for i in 1..points.len() {
        along.push(along[i - 1] + points[i].distance(points[i - 1]));
    }
    let length = *along.last().unwrap();
    if length == 0. {
        return Ok(());
    }
    if !length.is_finite() {
        return Err(Error::ResourceLimit("shoot length overflow"));
    }
    let frames = frames(&points);
    let stations = stations(length, envelope, p, twig, rng)?;
    reserve(out, stations.len(), p)?;
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
        let (mut tangent, mut normal, mut binormal) = frames[segment];
        if span > 1e-12 {
            tangent = (points[segment + 1] - points[segment]) / span;
            normal -= tangent * normal.dot(tangent);
            if normal.length_squared() <= 1e-12 {
                normal = tangent.perpendicular();
            }
            normal = normal.normalized();
            binormal = tangent.cross(normal).normalized();
        }
        let turn = if let Some(a) = twig {
            (k / a.stations_per_internode as usize) as f64 * p.divergence * PI / 180.
                + (k % a.stations_per_internode as usize) as f64 * TAU
                    / a.stations_per_internode as f64
        } else {
            k as f64 * p.divergence * PI / 180.
        };
        let (sin, cos) = turn.sin_cos();
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
        out.matrices.push(matrix(
            point,
            axis(point, radial, tangent, p),
            tangent,
            normal,
            p,
            rng,
        )?);
    }
    Ok(())
}

/// Distances along the run at which leaves sit: one set per internode under a
/// twig layer, height-relative spacing plus a terminal clump without one.
fn stations(
    length: f64,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
    rng: &mut Rng,
) -> Result<Vec<f64>> {
    let mut stations = Vec::new();
    if let Some(t) = twig {
        let internodes = (length / t.internode_length - 1e-9).ceil().max(1.);
        if internodes > 512. / t.stations_per_internode as f64 {
            return Err(Error::ResourceLimit("twig station budget"));
        }
        for i in 0..internodes as usize {
            for _ in 0..t.stations_per_internode {
                stations.push(i as f64 * t.internode_length);
            }
        }
        return Ok(stations);
    }
    let spacing = (p.spacing * envelope.height.max(1e-6)).max(1e-4);
    let count = (length / spacing).ceil();
    if count > 512. - p.clump as f64 {
        return Err(Error::ResourceLimit("shoot station budget"));
    }
    for i in 0..count as usize {
        stations.push(i as f64 * spacing);
    }
    for _ in 0..p.clump {
        stations.push(length * (1. - p.clump_span * rng.next_f64()));
    }
    Ok(stations)
}

fn reserve(out: &mut Instances, stations: usize, p: CanopyParams) -> Result<()> {
    let total = out
        .matrices
        .len()
        .checked_add(stations)
        .ok_or(Error::ResourceLimit("foliage count overflow"))?;
    if total
        .checked_mul(std::mem::size_of::<[f32; 16]>())
        .is_none_or(|bytes| bytes > isize::MAX as usize)
        || total > p.max_instances
    {
        return Err(Error::ResourceLimit("foliage instance budget"));
    }
    out.matrices
        .try_reserve(stations)
        .map_err(|_| Error::ResourceLimit("foliage allocation"))
}

/// The leaf's own axis: the radial off the wood, leaned along the shoot by
/// forward lean and by lean rise where the radial faces up, then carried
/// outward from the trunk and upward by the canopy's own terms.
fn axis(point: Vec3, radial: Vec3, tangent: Vec3, p: CanopyParams) -> Vec3 {
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

fn matrix(
    point: Vec3,
    mut axis: Vec3,
    tangent: Vec3,
    normal: Vec3,
    p: CanopyParams,
    rng: &mut Rng,
) -> Result<[f32; 16]> {
    let mut face = Vec3::Y - axis * axis.y;
    if face.length_squared() <= 1e-12 {
        face = tangent - axis * tangent.dot(axis);
    }
    if face.length_squared() <= 1e-12 {
        face = normal - axis * normal.dot(axis);
    }
    face = face.normalized();
    let mut side = axis.cross(face).normalized();
    if p.scatter > 0. {
        let z = rng.range(-1., 1.);
        let phi = rng.range(0., TAU);
        let ring = (1. - z * z).max(0.).sqrt();
        let jitter = Vec3::new(ring * phi.cos(), z, ring * phi.sin());
        let angle = p.scatter * PI / 180. * rng.next_f64();
        axis = axis.rotate(jitter, angle);
        face = face.rotate(jitter, angle);
        side = side.rotate(jitter, angle);
    }
    let scale = p.size * (1. + p.size_variation * rng.range(-1., 1.));
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
                normal = normal.rotate(cross.normalized(), cross.length().atan2(dot));
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
