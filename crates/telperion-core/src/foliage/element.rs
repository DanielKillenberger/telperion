use super::range;
use crate::{math::Vec3, Error, Result};
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ElementAnatomy {
    #[default]
    GenericBlade,
    LobedBlade,
    FourSidedNeedle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoliageUnit {
    Leaf,
    Needle,
}

/// Geometry of one biological unit, excluding its petiole or woody peg.
#[derive(Debug, Clone, PartialEq)]
pub struct AnatomyGeometry {
    pub unit: FoliageUnit,
    pub vertices: std::ops::Range<usize>,
    /// Offsets into Element::indices, not triangle numbers.
    pub indices: std::ops::Range<usize>,
    /// Vertex ranges at successive transverse sections, base to tip.
    pub sections: Vec<std::ops::Range<usize>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElementParams {
    pub anatomy: ElementAnatomy,
    /// Metres; used by lobed blades and needles, excluded from unit dimensions.
    pub connector_length: f64,
    /// Blade/needle longitudinal extent, excluding connector, in metres.
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
            anatomy: ElementAnatomy::GenericBlade,
            connector_length: 0.01,
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
    /// Metres, attachment at origin, axis +Y, blade face +Z. One unit per instance.
    pub positions: Vec<Vec3>,
    /// Blade triangles face +Z; needle and connector triangles face outward.
    pub indices: Vec<u32>,
    pub anatomy: Option<AnatomyGeometry>,
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
        if let Some(a) = &self.anatomy {
            if a.vertices.is_empty()
                || a.vertices.end > self.positions.len()
                || a.indices.is_empty()
                || a.indices.end > self.indices.len()
                || !a.indices.start.is_multiple_of(3)
                || !a.indices.end.is_multiple_of(3)
                || a.sections.len() < 2
                || a.sections
                    .iter()
                    .any(|r| r.is_empty() || r.start < a.vertices.start || r.end > a.vertices.end)
                || a.sections.windows(2).any(|w| w[0].end > w[1].start)
                || self.indices[a.indices.clone()]
                    .iter()
                    .any(|i| !a.vertices.contains(&(*i as usize)))
                || self.indices.as_chunks::<3>().0.iter().any(|t| {
                    let [p, q, r] = [t[0], t[1], t[2]].map(|i| self.positions[i as usize]);
                    (q - p).cross(r - p).length_squared() <= 0.
                })
            {
                return Err(Error::InvalidInput("foliage anatomy geometry"));
            }
        }
        Ok(())
    }
}
pub fn build_element(p: ElementParams) -> Result<Element> {
    for (v, l, h, n) in [
        (p.connector_length, 0., 1e3, "foliage connector length"),
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
    if p.anatomy != ElementAnatomy::GenericBlade {
        range(
            p.connector_length,
            1e-6,
            p.length,
            "foliage connector length",
        )?;
        if p.card {
            return Err(Error::InvalidInput("species anatomy cannot be a card"));
        }
        return build_anatomy(p);
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
            anatomy: None,
        });
    }
    let columns = p.cross_segments + (p.cross_segments % 2);
    let rows = p.axial_segments - 1;
    let mut e = Element {
        positions: Vec::with_capacity((2 + rows * (columns + 1)) as usize),
        indices: Vec::with_capacity((6 * columns * rows) as usize),
        anatomy: None,
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

fn build_anatomy(p: ElementParams) -> Result<Element> {
    let mut e = Element::default();
    let mut sections = Vec::new();
    let needle = p.anatomy == ElementAnatomy::FourSidedNeedle;
    if needle {
        let rows = p.axial_segments.max(4);
        for row in 0..rows {
            let t = row as f64 / rows as f64;
            let radius = p.width
                * 0.5
                * if row == 0 {
                    0.35
                } else {
                    // Retain the four-sided shaft until the short distal point.
                    1.0 - 0.12 * t
                };
            let start = e.positions.len();
            for (x, z) in [(1., 0.), (0., 1.), (-1., 0.), (0., -1.)] {
                e.positions.push(Vec3::new(
                    x * radius,
                    p.connector_length + t * p.length,
                    z * radius + p.curl * p.length * t * t,
                ));
            }
            sections.push(start..e.positions.len());
        }
        let tip = e.positions.len() as u32;
        e.positions.push(Vec3::new(
            0.,
            p.connector_length + p.length,
            p.curl * p.length,
        ));
        sections.push(tip as usize..tip as usize + 1);
        for row in 0..rows - 1 {
            for side in 0..4 {
                let a = row * 4 + side;
                let b = row * 4 + (side + 1) % 4;
                e.indices.extend([a, a + 4, b + 4, a, b + 4, b]);
            }
        }
        for side in 0..4 {
            e.indices
                .extend([(rows - 1) * 4 + side, tip, (rows - 1) * 4 + (side + 1) % 4]);
        }
        let base = e.positions.len() as u32;
        e.positions.push(Vec3::new(0., p.connector_length, 0.));
        for side in 0..4 {
            e.indices.extend([base, side, (side + 1) % 4]);
        }
    } else {
        let rows = p.axial_segments.max(64);
        let columns = p.cross_segments + p.cross_segments % 2;
        e.positions.push(Vec3::new(0., p.connector_length, 0.));
        sections.push(0..1);
        for row in 1..rows {
            let t = (1.0 - (std::f64::consts::PI * row as f64 / rows as f64).cos()) * 0.5;
            let left = oak_half_width(t, false) * p.width * 0.5;
            let right = oak_half_width(t, true) * p.width * 0.5;
            let start = e.positions.len();
            for col in 0..=columns {
                let x = 2. * col as f64 / columns as f64 - 1.;
                e.positions.push(Vec3::new(
                    x * if x < 0.0 { left } else { right },
                    p.connector_length + t * p.length,
                    p.curl * p.length * t * t + p.cup * if x < 0.0 { left } else { right } * x * x,
                ));
            }
            sections.push(start..e.positions.len());
        }
        let tip = e.positions.len() as u32;
        e.positions.push(Vec3::new(
            0.,
            p.connector_length + p.length,
            p.curl * p.length,
        ));
        sections.push(tip as usize..tip as usize + 1);
        for col in 0..columns {
            e.indices.extend([0, 2 + col, 1 + col]);
        }
        for row in 0..rows - 2 {
            for col in 0..columns {
                let a = 1 + row * (columns + 1) + col;
                let d = a + columns + 1;
                e.indices.extend([a, a + 1, d + 1, a, d + 1, d]);
            }
        }
        for col in 0..columns {
            let a = 1 + (rows - 2) * (columns + 1) + col;
            e.indices.extend([a, a + 1, tip]);
        }
    }
    e.anatomy = Some(AnatomyGeometry {
        unit: if needle {
            FoliageUnit::Needle
        } else {
            FoliageUnit::Leaf
        },
        vertices: 0..e.positions.len(),
        indices: 0..e.indices.len(),
        sections,
    });
    connector(
        &mut e,
        p.connector_length,
        p.width * if needle { 0.175 } else { 0.018 },
    );
    for v in &mut e.positions {
        *v = Vec3::new(v.x as f32 as f64, v.y as f32 as f64, v.z as f32 as f64);
    }
    e.validate()?;
    Ok(e)
}

/// Unequal rounded lateral lobes joined by a narrow midrib region. Elliptic
/// ends give each lobe a rounded nose; staggered sides avoid a periodic wave.
fn oak_half_width(t: f64, right: bool) -> f64 {
    let mut width = 0.17 * (std::f64::consts::PI * t).sin().sqrt();
    for (centre, reach, half_height) in [
        (0.15, 0.62, 0.115),
        (0.36, 0.94, 0.14),
        (0.59, 1.0, 0.15),
        (0.78, 0.73, 0.12),
        (0.88, 0.45, 0.12),
    ] {
        let centre = centre + if right && centre < 0.85 { 0.018 } else { 0.0 };
        let along = (t - centre) / half_height;
        if along.abs() < 1.0 {
            width = width.max(reach * (1.0 - along * along).sqrt());
        }
    }
    width
}

fn connector(e: &mut Element, length: f64, radius: f64) {
    let start = e.positions.len() as u32;
    for y in [0., length] {
        for (x, z) in [(1., 0.), (0., 1.), (-1., 0.), (0., -1.)] {
            e.positions.push(Vec3::new(x * radius, y, z * radius));
        }
    }
    let base = e.positions.len() as u32;
    e.positions.push(Vec3::ZERO);
    e.positions.push(Vec3::new(0., length, 0.));
    for side in 0..4 {
        let a = start + side;
        let b = start + (side + 1) % 4;
        e.indices.extend([
            a,
            a + 4,
            b + 4,
            a,
            b + 4,
            b,
            base,
            a,
            b,
            base + 1,
            b + 4,
            a + 4,
        ]);
    }
}
