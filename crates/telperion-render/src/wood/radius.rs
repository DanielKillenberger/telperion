//! Recover ring radii without changing the core's mesh or vertex layout.
use telperion_core::surface::SurfaceMesh;

pub fn radii(mesh: &SurfaceMesh) -> Vec<f32> {
    let count = mesh.positions.len() / 3;
    let mut radii = vec![0.0; count];
    if mesh.coords.len() != count * 2 {
        return radii;
    }
    let mut start = 0;
    while start < count {
        let mut end = start + 1;
        while end < count
            && mesh.coords[end * 2] == mesh.coords[start * 2]
            && mesh.coords[end * 2 + 1] > mesh.coords[(end - 1) * 2 + 1]
        {
            end += 1;
        }
        // Complete rings start at zero and sweep past pi. Caps and absent
        // coordinates have no defined circumferential metric: leave them smooth.
        if end - start >= 3 && mesh.coords[(end - 1) * 2 + 1] > std::f32::consts::PI {
            let radius =
                telperion_core::surface::prepared::ring_radius(&mesh.positions[start * 3..end * 3]);
            radii[start..end].fill(radius);
        }
        start = end;
    }
    radii
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translated_tapered_rings_keep_their_metric_radius_and_caps_are_smooth() {
        let mut mesh = SurfaceMesh::default();
        for (along, radius) in [(2.0, 0.4), (3.0, 0.2)] {
            for i in 0..16 {
                let angle = i as f32 * std::f32::consts::TAU / 16.0;
                mesh.positions.extend([
                    7.0 + radius * angle.cos(),
                    along,
                    -4.0 + radius * angle.sin(),
                ]);
                mesh.coords.extend([along, angle]);
            }
        }
        mesh.positions.extend([7.0, 2.0, -4.0, 7.0, 3.0, -4.0]);
        mesh.coords.extend([2.0, 0.0, 3.0, 0.0]);
        let values = radii(&mesh);
        for (i, radius) in values.iter().enumerate() {
            let expected = if i < 16 {
                0.4
            } else if i < 32 {
                0.2
            } else {
                0.0
            };
            assert!(
                (radius - expected).abs() < 1e-5,
                "vertex {i}: radius {radius}, expected {expected}"
            );
        }
    }
}
