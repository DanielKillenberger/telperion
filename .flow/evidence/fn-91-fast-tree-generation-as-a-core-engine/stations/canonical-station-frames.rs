pub(super) fn station_frames(points: &[Vec3], along: &[f64]) -> Vec<(Vec3, Vec3, Vec3)> {
    let mut frames = frames(points);
    for (segment, (tangent, normal, binormal)) in frames[..points.len() - 1].iter_mut().enumerate()
    {
        let span = along[segment + 1] - along[segment];
        if span > 1e-12 {
            *tangent = (points[segment + 1] - points[segment]) / span;
            *normal -= *tangent * normal.dot(*tangent);
            if normal.length_squared() <= 1e-12 {
                *normal = tangent.perpendicular();
            }
            *normal = normal.normalized();
            *binormal = tangent.cross(*normal).normalized();
        }
    }
    frames
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

