use super::range;
use crate::{math::Vec3, Error, Result};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElementParams {
    pub length: f64,
    pub width: f64,
    pub widest_at: f64,
    pub base_fullness: f64,
    pub tip_sharpness: f64,
    pub cup: f64,
    pub curl: f64,
    pub axial_segments: u32,
    pub cross_segments: u32,
    pub card: bool,
}
impl Default for ElementParams {
    fn default() -> Self {
        Self {
            length: 0.12,
            width: 0.06,
            widest_at: 0.42,
            base_fullness: 0.85,
            tip_sharpness: 1.6,
            cup: 0.18,
            curl: 0.12,
            axial_segments: 5,
            cross_segments: 2,
            card: false,
        }
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Element {
    /// Metres, petiole at origin, axis +Y, face +Z. Reused by all instances.
    pub positions: Vec<Vec3>,
    /// Counterclockwise viewed from +Z.
    pub indices: Vec<u32>,
}
impl Element {
    pub fn validate(&self) -> Result<()> {
        if self
            .positions
            .iter()
            .any(|p| !p.is_finite() || [p.x, p.y, p.z].iter().any(|v| !(*v as f32).is_finite()))
            || !self.indices.len().is_multiple_of(3)
            || self
                .indices
                .iter()
                .any(|i| *i as usize >= self.positions.len())
        {
            return Err(Error::InvalidInput("leaf element"));
        }
        Ok(())
    }
}
pub fn build_element(p: ElementParams) -> Result<Element> {
    for (v, l, h, n) in [
        (p.length, 1e-4, 1e3, "leaf length"),
        (p.width, 1e-4, 1e3, "leaf width"),
        (p.widest_at, 0.05, 0.95, "leaf widest point"),
        (p.base_fullness, 0.2, 8., "leaf base fullness"),
        (p.tip_sharpness, 0.2, 8., "leaf tip sharpness"),
        (p.cup, -2., 2., "leaf cup"),
        (p.curl, -2., 2., "leaf curl"),
    ] {
        range(v, l, h, n)?;
    }
    if !(2..=64).contains(&p.axial_segments) || !(2..=64).contains(&p.cross_segments) {
        return Err(Error::InvalidInput("leaf segments"));
    }
    let half = p.width / 2.;
    if p.card {
        return Ok(Element {
            positions: vec![
                Vec3::new(-half as f32 as f64, 0., 0.),
                Vec3::new(half as f32 as f64, 0., 0.),
                Vec3::new(half as f32 as f64, p.length as f32 as f64, 0.),
                Vec3::new(-half as f32 as f64, p.length as f32 as f64, 0.),
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
        });
    }
    let columns = p.cross_segments + (p.cross_segments % 2);
    let rows = p.axial_segments - 1;
    let mut e = Element {
        positions: Vec::with_capacity((2 + rows * (columns + 1)) as usize),
        indices: Vec::with_capacity((6 * columns * rows) as usize),
    };
    e.positions.push(Vec3::ZERO);
    for row in 0..rows {
        let t = (row + 1) as f64 / p.axial_segments as f64;
        let profile = if t <= p.widest_at {
            (std::f64::consts::FRAC_PI_2 * t / p.widest_at)
                .sin()
                .powf(p.base_fullness)
        } else {
            (std::f64::consts::FRAC_PI_2 * (t - p.widest_at) / (1. - p.widest_at))
                .cos()
                .powf(p.tip_sharpness)
        };
        let width = half * profile;
        for col in 0..=columns {
            let x = 2. * col as f64 / columns as f64 - 1.;
            e.positions.push(Vec3::new(
                x * width,
                t * p.length,
                p.curl * p.length * t * t + p.cup * width * x * x,
            ));
        }
    }
    e.positions.push(Vec3::new(0., p.length, p.curl * p.length));
    let first = |r, c| 1 + r * (columns + 1) + c;
    for c in 0..columns {
        e.indices.extend([0, first(0, c + 1), first(0, c)]);
    }
    for r in 0..rows - 1 {
        for c in 0..columns {
            let a = first(r, c);
            let b = first(r, c + 1);
            let cc = first(r + 1, c + 1);
            let d = first(r + 1, c);
            e.indices.extend([a, b, cc, a, cc, d]);
        }
    }
    let tip = e.positions.len() as u32 - 1;
    for c in 0..columns {
        e.indices
            .extend([first(rows - 1, c), first(rows - 1, c + 1), tip]);
    }
    // Match the authored float32 element before culling or computing bounds.
    for v in &mut e.positions {
        *v = Vec3::new(v.x as f32 as f64, v.y as f32 as f64, v.z as f32 as f64);
    }
    Ok(e)
}
