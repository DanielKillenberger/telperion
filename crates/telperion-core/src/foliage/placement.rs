use super::{range, Instances};
use crate::{
    envelope::Envelope,
    math::Vec3,
    rng::Rng,
    tree::{NodeKind, Tree},
    Error, Result,
};
use std::f64::consts::{PI, TAU};
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Attachment {
    #[default]
    Generic,
    /// One leaf at each station; azimuth advances by canopy divergence.
    Alternate,
    /// Individual needles around the twig, upper needles leaning toward its tip.
    RadialNeedles,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CanopyParams {
    /// Local modes require marked twig runs and one station per internode.
    pub attachment: Attachment,
    pub shoot_radius: f64,
    pub spacing: f64,
    pub divergence: f64,
    pub clump: u32,
    pub clump_span: f64,
    pub outward: f64,
    pub upward: f64,
    pub scatter: f64,
    pub size: f64,
    pub size_variation: f64,
    /// Hard total budget. Exceeding it returns an error, never partial foliage.
    pub max_instances: usize,
}
impl Default for CanopyParams {
    fn default() -> Self {
        Self {
            attachment: Attachment::Generic,
            shoot_radius: 0.12,
            spacing: 0.006,
            divergence: 137.508,
            clump: 5,
            clump_span: 0.3,
            outward: 0.6,
            upward: 0.35,
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

/// Blades use marked twig edges; needles also persist on slender branchlets
/// within shoot_radius (a fraction of root radius). Without anatomy,
/// terminal runs use the radius threshold and height-relative spacing/clump.
pub fn place(
    tree: &Tree,
    envelope: Envelope,
    seed: u32,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
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
    if p.attachment != Attachment::Generic && twig.is_none_or(|t| t.stations_per_internode != 1) {
        return Err(Error::InvalidInput(
            "individual foliage requires one station per internode",
        ));
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
    if let Some(t) = twig {
        if p.attachment != Attachment::Generic {
            for run in twig_runs(tree, p) {
                place_run(tree, &run, envelope, p, Some(t), &mut rng, &mut out)?;
            }
            return Ok(out);
        }
        for (i, n) in tree.nodes.iter().enumerate().skip(tree.crossover) {
            if n.kind == NodeKind::Twig {
                if let Some(parent) = n.parent {
                    place_run(
                        tree,
                        &[parent as usize, i],
                        envelope,
                        p,
                        Some(t),
                        &mut rng,
                        &mut out,
                    )?;
                }
            }
        }
    } else {
        for run in shoots(tree, tree.nodes[0].radius * p.shoot_radius) {
            place_run(tree, &run, envelope, p, None, &mut rng, &mut out)?;
        }
    }
    Ok(out)
}
fn place_run(
    tree: &Tree,
    run: &[usize],
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
    rng: &mut Rng,
    out: &mut Instances,
) -> Result<()> {
    let points: Vec<_> = run.iter().map(|i| tree.nodes[*i].position).collect();
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
    } else {
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
    }
    let total = out
        .matrices
        .len()
        .checked_add(stations.len())
        .ok_or(Error::ResourceLimit("foliage count overflow"))?;
    if total
        .checked_mul(std::mem::size_of::<[f32; 16]>())
        .is_none_or(|bytes| bytes > isize::MAX as usize)
        || total > p.max_instances
    {
        return Err(Error::ResourceLimit("foliage instance budget"));
    }
    out.matrices
        .try_reserve(stations.len())
        .map_err(|_| Error::ResourceLimit("foliage allocation"))?;
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
        let mut point = points[segment].lerp(points[segment + 1], t);
        let distal = &tree.nodes[run[segment + 1]];
        let base = if twig.is_some() {
            distal.start_radius
        } else {
            tree.nodes[run[segment]].radius
        };
        let wood = base * (1. - t) + distal.radius * t;
        let (mut tangent, mut normal, mut binormal) = frames[segment];
        if p.attachment != Attachment::Generic && span > 1e-12 {
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
        point += radial * wood;
        let mut axis = match p.attachment {
            Attachment::Generic => {
                let outward = Vec3::new(point.x, 0., point.z);
                let mut axis = radial;
                if outward.length_squared() > 1e-12 {
                    axis += outward.normalized() * p.outward;
                }
                axis.y += p.upward;
                axis
            }
            Attachment::Alternate => radial + tangent * 0.25,
            Attachment::RadialNeedles => radial + tangent * (0.05 + 1.2 * radial.y.max(0.)),
        };
        if axis.length_squared() <= 1e-12 {
            axis = radial;
        }
        axis = axis.normalized();
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
        out.matrices.push(matrix);
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

fn twig_runs(tree: &Tree, p: CanopyParams) -> Vec<Vec<usize>> {
    let bearing = |i: usize| {
        let n = &tree.nodes[i];
        n.parent.is_some()
            && (n.kind == NodeKind::Twig
                || (p.attachment == Attachment::RadialNeedles
                    && n.radius.max(n.start_radius) <= tree.nodes[0].radius * p.shoot_radius))
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
