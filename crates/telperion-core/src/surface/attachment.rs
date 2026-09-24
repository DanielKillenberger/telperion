//! Exact contact queries on the swept polygons, read without mesh indices
//! or normals: from rings swept here, or in place from the ring step's.
use super::*;
use std::borrow::Cow;

pub(crate) struct AttachmentSurface<'w> {
    /// Ring points as float32 triples: the vertices `build` submits, swept
    /// and rounded here or read in place from the ring step.
    rings: Cow<'w, [f32]>,
    /// Each node's lower and upper ring, then its run's first and last ring,
    /// as point offsets into `rings`.
    pub(crate) edges: Cow<'w, [Option<[usize; 4]>]>,
    pub(crate) segments: usize,
    segment_bounds: Vec<Option<(Vec3, Vec3)>>,
}

/// Ring point `i`, widened to the f64 the queries run in.
#[inline]
fn at(rings: &[f32], i: usize) -> Vec3 {
    let [x, y, z] = rings.as_chunks::<3>().0[i];
    Vec3::new(x as f64, y as f64, z as f64)
}

impl<'w> AttachmentSurface<'w> {
    /// The contact surface of rings swept with their contacts, read in place
    /// from the vertices the wood draws.
    pub(crate) fn on_wood(rings: &'w Rings) -> Result<Self> {
        Self::bounded(
            Cow::Borrowed(&rings.positions),
            Cow::Borrowed(&rings.edges),
            rings.segments,
        )
    }

    fn bounded(
        rings: Cow<'w, [f32]>,
        edges: Cow<'w, [Option<[usize; 4]>]>,
        segments: usize,
    ) -> Result<Self> {
        let mut out = Self {
            segment_bounds: filled(rings.len() / 3 / segments + 1, None)?,
            rings,
            edges,
            segments,
        };
        out.bound();
        Ok(out)
    }

    /// Every ring point, widened, for a caller that keeps them past the query.
    pub(crate) fn points(&self) -> Vec<Vec3> {
        let points = self.rings.as_chunks::<3>().0.iter();
        points
            .map(|&[x, y, z]| Vec3::new(x as f64, y as f64, z as f64))
            .collect()
    }
}

impl AttachmentSurface<'static> {
    pub(crate) fn new(tree: &Tree, height: f64, params: &SurfaceParams) -> Result<Self> {
        Self::selected(tree, height, params, None)
    }

    /// Derive only sweep paths used by the selected shoots. Topology and frames
    /// are identical to the whole surface, including neighbouring segments.
    pub(crate) fn selected(
        tree: &Tree,
        height: f64,
        params: &SurfaceParams,
        selected: Option<&std::collections::BTreeSet<crate::tree::NodeIdentity>>,
    ) -> Result<Self> {
        tree.validate_solved()?;
        params.validate()?;
        crate::surface::height(height)?;
        let height = height.max(1e-6);
        let paths = paths(&tree.nodes)?;
        let segments = segments(params);
        let angular = angular::samples(segments, params)?;
        let mut rings = reserved(
            paths
                .nodes
                .len()
                .checked_add(1)
                .and_then(|n| n.checked_mul(segments * 3))
                .ok_or(Error::ResourceLimit("attachment rings"))?,
        )?;
        let mut edges = filled(tree.nodes.len(), None)?;
        let mut distance = filled(tree.nodes.len(), 0.0)?;
        for i in 1..tree.nodes.len() {
            let p = tree.nodes[i].parent.unwrap() as usize;
            distance[i] = distance[p] + tree.nodes[p].position.distance(tree.nodes[i].position);
        }
        let mut samples = Vec::new();
        let mut frame = Vec::new();
        let mut scratch = Vec::new();
        for path in &paths.runs {
            let nodes = &paths.nodes[path.start..path.end];
            if selected.is_some_and(|ids| {
                !nodes
                    .iter()
                    .skip(1)
                    .any(|&i| ids.contains(&tree.nodes[i].identity))
            }) {
                continue;
            }
            sample_path(tree, height, params, &paths, path, &distance, &mut samples);
            frames(&samples, &mut scratch, &mut frame);
            let base = rings.len() / 3;
            for (i, s) in samples.iter().enumerate() {
                let (normal, binormal) = frame[i];
                let phase = std::f64::consts::TAU * params.twist_rate * (s.d / height);
                for sample in &angular {
                    let profile = sample.profile(params, phase);
                    let p = s.p + (normal * sample.cos + binormal * sample.sin) * (s.r * profile);
                    // Query exactly the float32 vertices submitted by build().
                    rings.extend([p.x as f32, p.y as f32, p.z as f32]);
                }
            }
            let offset = usize::from(path.trunk && params.flare_depth > 0.0);
            record_edges(&mut edges, nodes, base, samples.len(), segments, offset);
        }
        Self::bounded(Cow::Owned(rings), Cow::Owned(edges), segments)
    }
}

impl AttachmentSurface<'_> {
    /// Each ring pair a query can read, bounded once: a node's own pair and
    /// the pairs on either side of it within its run. A pair's lower ring
    /// starts at least a ring past the one before, so `lo / segments` keys
    /// it apart from every other.
    fn bound(&mut self) {
        let s = self.segments;
        for &[lo, hi, start, end] in self.edges.iter().flatten() {
            let below = lo.checked_sub(s).filter(|&i| i >= start);
            for lo in [below, Some(lo), (hi < end).then_some(hi)]
                .into_iter()
                .flatten()
            {
                let slot = &mut self.segment_bounds[lo / s];
                if slot.is_some() {
                    continue;
                }
                let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
                let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
                for i in lo..lo + 2 * s {
                    let p = at(&self.rings, i);
                    min.x = min.x.min(p.x);
                    min.y = min.y.min(p.y);
                    min.z = min.z.min(p.z);
                    max.x = max.x.max(p.x);
                    max.y = max.y.max(p.y);
                    max.z = max.z.max(p.z);
                }
                *slot = Some((min, max));
            }
        }
    }

    /// Exact neighboring polygons on which a station contact can depend.
    pub(crate) fn signature(&self, node: usize) -> Vec<Vec3> {
        let Some([lo, hi, start, end]) = self.edges[node] else {
            return Vec::new();
        };
        let first = if lo > start { lo - self.segments } else { lo };
        let last = if hi < end { hi + self.segments } else { hi };
        (first..last + self.segments)
            .map(|i| at(&self.rings, i))
            .collect()
    }
    pub(crate) fn point(
        &self,
        node: usize,
        origin: Vec3,
        radial: Vec3,
        radius: f64,
    ) -> Option<Vec3> {
        let [lo, hi, start, end] = self.edges[node]?;
        let rings: &[f32] = &self.rings;
        let s = self.segments;
        let mut best = self.segment_distance(rings, lo, hi, origin, radial);
        // Near a bent station the perpendicular ray can leave through the
        // neighbouring swept segment rather than the centreline's own segment.
        if lo > start {
            best = best.min(self.segment_distance(rings, lo - s, lo, origin, radial));
        }
        if hi < end {
            best = best.min(self.segment_distance(rings, hi, hi + s, origin, radial));
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
                    let p = closest(target, at(rings, a), at(rings, b), at(rings, c));
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
    fn segment_distance(
        &self,
        rings: &[f32],
        lo: usize,
        hi: usize,
        origin: Vec3,
        radial: Vec3,
    ) -> f64 {
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
                let a = at(rings, a);
                let e1 = at(rings, b) - a;
                let e2 = at(rings, c) - a;
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

#[cfg(test)]
mod tests;
