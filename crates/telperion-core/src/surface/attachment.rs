//! Exact swept polygon queries without constructing mesh indices or normals.
use super::*;

pub(crate) struct AttachmentSurface {
    rings: Vec<Vec3>,
    edges: Vec<Option<(usize, usize, usize, usize)>>,
    segments: usize,
    segment_bounds: Vec<Option<(Vec3, Vec3)>>,
}
impl AttachmentSurface {
    pub(crate) fn new(tree: &Tree, height: f64, params: &SurfaceParams) -> Result<Self> {
        tree.validate_solved()?;
        params.validate()?;
        if !height.is_finite() || height <= 0.0 {
            return Err(Error::InvalidInput("surface height"));
        }
        let height = height.max(1e-6);
        let paths = paths(&tree.nodes)?;
        let segments = params.radial_segments.max(params.lobes * 4) as usize;
        let mut out = Self {
            rings: reserved(
                paths
                    .nodes
                    .len()
                    .checked_add(1)
                    .and_then(|n| n.checked_mul(segments))
                    .ok_or(Error::ResourceLimit("attachment rings"))?,
            )?,
            edges: filled(tree.nodes.len(), None)?,
            segments,
            segment_bounds: Vec::new(),
        };
        let mut distance = filled(tree.nodes.len(), 0.0)?;
        for i in 1..tree.nodes.len() {
            let p = tree.nodes[i].parent.unwrap() as usize;
            distance[i] = distance[p] + tree.nodes[p].position.distance(tree.nodes[i].position);
        }
        let mut samples = Vec::new();
        let mut frame = Vec::new();
        let mut scratch = Vec::new();
        for path in paths.runs {
            let nodes = &paths.nodes[path.start..path.end];
            sample_path(
                tree,
                height,
                params,
                nodes,
                path.trunk,
                &distance,
                &mut samples,
            );
            frames(&samples, &mut scratch, &mut frame);
            let base = out.rings.len();
            for (i, s) in samples.iter().enumerate() {
                let (normal, binormal) = frame[i];
                let phase = std::f64::consts::TAU * params.twist_rate * (s.d / height);
                for k in 0..segments {
                    let angle = k as f64 / segments as f64 * std::f64::consts::TAU;
                    let profile = if params.lobes == 0 {
                        1.0
                    } else {
                        1.0 + params.lobe_depth * (params.lobes as f64 * (angle + phase)).cos()
                    };
                    let p = s.p + (normal * angle.cos() + binormal * angle.sin()) * (s.r * profile);
                    // Query exactly the float32 vertices submitted by build().
                    out.rings.push(Vec3::new(
                        p.x as f32 as f64,
                        p.y as f32 as f64,
                        p.z as f32 as f64,
                    ));
                }
            }
            out.segment_bounds.resize(out.rings.len() / segments, None);
            for i in 0..samples.len() - 1 {
                let lo = base + i * segments;
                let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
                let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
                for p in &out.rings[lo..lo + 2 * segments] {
                    min.x = min.x.min(p.x);
                    min.y = min.y.min(p.y);
                    min.z = min.z.min(p.z);
                    max.x = max.x.max(p.x);
                    max.y = max.y.max(p.y);
                    max.z = max.z.max(p.z);
                }
                out.segment_bounds[lo / segments] = Some((min, max));
            }
            let offset = usize::from(path.trunk && params.flare_depth > 0.0);
            for (i, &node) in nodes.iter().enumerate().skip(1) {
                out.edges[node] = Some((
                    base + (i - 1 + offset) * segments,
                    base + (i + offset) * segments,
                    base,
                    base + (samples.len() - 1) * segments,
                ));
            }
        }
        Ok(out)
    }
    pub(crate) fn point(
        &self,
        node: usize,
        origin: Vec3,
        radial: Vec3,
        radius: f64,
    ) -> Option<Vec3> {
        let (lo, hi, start, end) = self.edges[node]?;
        let mut best = self.segment_distance(lo, hi, origin, radial);
        // Near a bent station the perpendicular ray can leave through the
        // neighbouring swept segment rather than the centreline's own segment.
        if lo > start {
            best = best.min(self.segment_distance(lo - self.segments, lo, origin, radial));
        }
        if hi < end {
            best = best.min(self.segment_distance(hi, hi + self.segments, origin, radial));
        }
        if best.is_finite() {
            return Some(origin + radial * best);
        }
        // A station close to a tilted end ring can lie just outside the finite
        // sweep. Project its intended origin onto the actual neighbouring facets,
        // rather than falling back to a circular radius or dropping the needle.
        let target = origin + radial * radius;
        let mut point = None;
        let mut distance = f64::INFINITY;
        for lower in [
            lo.checked_sub(self.segments).filter(|i| *i >= start),
            Some(lo),
            (hi < end).then_some(hi),
        ]
        .into_iter()
        .flatten()
        {
            let upper = lower + self.segments;
            for k in 0..self.segments {
                let next = (k + 1) % self.segments;
                for [a, b, c] in [
                    [lower + k, lower + next, upper + k],
                    [lower + next, upper + next, upper + k],
                ] {
                    let p = closest(target, self.rings[a], self.rings[b], self.rings[c]);
                    let d = (p - target).length_squared();
                    if d < distance {
                        distance = d;
                        point = Some(p);
                    }
                }
            }
        }
        point
    }
    fn segment_distance(&self, lo: usize, hi: usize, origin: Vec3, radial: Vec3) -> f64 {
        let (min, max) = self.segment_bounds[lo / self.segments].unwrap();
        let mut near = 0.0_f64;
        let mut far = f64::INFINITY;
        for (o, d, a, b) in [
            (origin.x, radial.x, min.x, max.x),
            (origin.y, radial.y, min.y, max.y),
            (origin.z, radial.z, min.z, max.z),
        ] {
            if d.abs() < 1e-15 {
                if o < a - 1e-9 || o > b + 1e-9 {
                    return f64::INFINITY;
                }
                continue;
            }
            let t0 = (a - 1e-9 - o) / d;
            let t1 = (b + 1e-9 - o) / d;
            near = near.max(t0.min(t1));
            far = far.min(t0.max(t1));
            if near > far {
                return f64::INFINITY;
            }
        }
        let mut best = f64::INFINITY;
        for k in 0..self.segments {
            let next = (k + 1) % self.segments;
            for [a, b, c] in [[lo + k, lo + next, hi + k], [lo + next, hi + next, hi + k]] {
                let a = self.rings[a];
                let e1 = self.rings[b] - a;
                let e2 = self.rings[c] - a;
                let h = radial.cross(e2);
                let det = e1.dot(h);
                if det.abs() < 1e-18 {
                    continue;
                }
                let v = origin - a;
                let u = v.dot(h) / det;
                if !(-1e-7..=1.0 + 1e-7).contains(&u) {
                    continue;
                }
                let q = v.cross(e1);
                let w = radial.dot(q) / det;
                if w < -1e-7 || u + w > 1.0 + 1e-7 {
                    continue;
                }
                let t = e2.dot(q) / det;
                if t >= 0.0 {
                    best = best.min(t);
                }
            }
        }
        best
    }
}

fn closest(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;
    let d1 = ab.dot(ap);
    let d2 = ac.dot(ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return a;
    }
    let bp = p - b;
    let d3 = ab.dot(bp);
    let d4 = ac.dot(bp);
    if d3 >= 0.0 && d4 <= d3 {
        return b;
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        return a + ab * (d1 / (d1 - d3));
    }
    let cp = p - c;
    let d5 = ab.dot(cp);
    let d6 = ac.dot(cp);
    if d6 >= 0.0 && d5 <= d6 {
        return c;
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        return a + ac * (d2 / (d2 - d6));
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && d4 - d3 >= 0.0 && d5 - d6 >= 0.0 {
        return b + (c - b) * ((d4 - d3) / ((d4 - d3) + (d5 - d6)));
    }
    a + ab * (vb / (va + vb + vc)) + ac * (vc / (va + vb + vc))
}
