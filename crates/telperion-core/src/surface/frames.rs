use super::Sample;
use crate::math::Vec3;
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
pub(super) fn frames(samples: &[Sample], segments: &mut Vec<Vec3>, out: &mut Vec<(Vec3, Vec3)>) {
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
    let mut normal = previous.perpendicular();
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
