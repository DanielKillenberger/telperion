use super::paths::{Paths, Run};
use super::Sample;
use crate::math::Vec3;
use crate::tree::Tree;

/// A run's frame seed (fn-212, host decision 2026-10-07; every run since
/// fn-256): the frame its bearer's wood was laid in at the socket, its
/// heading and its side; none where the tree has no laid frames, whose
/// runs seed their own. The side, laid across the run's first tangent, is the run's first
/// normal, so a run's frame moves with its wood and never jumps between
/// axes. It turns only where a run leaves along its bearer's side, which
/// no measured run does (TASK5.md); a rotation from the bearer's heading
/// would turn where a run leaves backwards, which thousands of the
/// spruce's do.
pub(super) fn seed(tree: &Tree, paths: &Paths, run: &Run) -> Option<(Vec3, Vec3)> {
    let f = tree.frames.get(paths.nodes[run.start])?.map(f64::from);
    Some((Vec3::new(f[0], f[1], f[2]), Vec3::new(f[3], f[4], f[5])))
}
fn rotate(v: Vec3, from: Vec3, to: Vec3) -> Vec3 {
    let mut w = from.dot(to) + 1.0;
    let mut q = if w < 1e-8 {
        w = 0.0;
        if from.x.abs() > from.z.abs() {
            Vec3::new(-from.y, from.x, 0.0)
        } else {
            Vec3::new(0.0, -from.z, from.y)
        }
    } else {
        from.cross(to)
    };
    let inv = 1.0 / (q.dot(q) + w * w).sqrt();
    q = q * inv;
    w *= inv;
    let t = q.cross(v) * 2.0;
    Vec3::new(
        v.x + w * t.x + q.y * t.z - q.z * t.y,
        v.y + w * t.y + q.z * t.x - q.x * t.z,
        v.z + w * t.z + q.x * t.y - q.y * t.x,
    )
}
pub(super) fn frames(
    samples: &[Sample],
    segments: &mut Vec<Vec3>,
    out: &mut Vec<(Vec3, Vec3)>,
    seed: Option<(Vec3, Vec3)>,
) {
    segments.clear();
    out.clear();
    for pair in samples.windows(2) {
        let step = pair[1].p - pair[0].p;
        segments.push(if step.length_squared() > 0.0 {
            step.normalized()
        } else {
            segments.last().copied().unwrap_or(Vec3::Y)
        });
    }
    let tangent = |i: usize| {
        if i == 0 {
            segments[0]
        } else if i == samples.len() - 1 {
            segments[i - 1]
        } else {
            let mean = segments[i - 1] + segments[i];
            if mean.length_squared() > 1e-9 {
                mean.normalized()
            } else {
                segments[i]
            }
        }
    };
    let mut previous = tangent(0);
    let mut normal = match seed {
        Some((_, side)) => side,
        // The sweep's own seed, for a run of the structure above the cut.
        None => previous.perpendicular(),
    };
    for i in 0..samples.len() {
        let t = tangent(i);
        if i > 0 {
            normal = rotate(normal, previous, t);
        }
        normal = normal + t * (-normal.dot(t));
        if normal.length_squared() <= 1e-9 {
            normal = t.perpendicular();
        }
        normal = normal.normalized();
        out.push((normal, t.cross(normal).normalized()));
        previous = t;
    }
}
