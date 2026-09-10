use super::{levels, outline, range, Level};
use crate::{math::Vec3, Error, Result};

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
    /// Metres of petiole or woody peg below the blade, excluded from unit
    /// dimensions. Every element carries one.
    pub connector_length: f64,
    /// Blade/needle longitudinal extent, excluding connector, in metres.
    pub length: f64,
    pub width: f64,
    pub widest_at: f64,
    pub base_fullness: f64,
    pub tip_sharpness: f64,
    pub cup: f64,
    pub curl: f64,
    /// Lobes along each margin; 0 is an entire margin.
    pub lobe_count: u32,
    /// How far each sinus cuts toward the midrib, 0 to 1.
    pub lobe_depth: f64,
    /// Flat blade at 0, four-sided shaft at 1.
    pub section_roundness: f64,
    pub axial_segments: u32,
    pub cross_segments: u32,
    pub card: bool,
}
impl Default for ElementParams {
    fn default() -> Self {
        Self {
            connector_length: 0.01,
            length: 0.12,
            width: 0.06,
            widest_at: 0.42,
            base_fullness: 0.85,
            tip_sharpness: 1.6,
            cup: 0.18,
            curl: 0.12,
            lobe_count: 0,
            lobe_depth: 0.,
            section_roundness: 0.,
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
    /// Blade triangles face +Z; a rounded section and the connector face outward.
    pub indices: Vec<u32>,
    pub anatomy: Option<AnatomyGeometry>,
    /// Every level's triangles in one buffer, coarsest first; both are empty
    /// for an element built by hand.
    pub level_indices: Vec<u32>,
    /// Nested simplifications sharing `positions`, coarsest first. Deviations
    /// strictly decrease to zero at the finest, which is `indices` byte for byte.
    pub levels: Vec<Level>,
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
        if !levels::nested(self) {
            return Err(Error::InvalidInput("foliage element levels"));
        }
        Ok(())
    }
}

/// One outline routine for every element. The blade, the lobed blade and the
/// needle are rows of the same traits: the margin comes from the outline
/// profile and its lobes, the transverse section from the roundness, and the
/// axial ladder from the sections either way.
pub fn build_element(p: ElementParams) -> Result<Element> {
    for (v, l, h, n) in [
        (p.length, 1e-4, 1e3, "leaf length"),
        (p.width, 1e-4, 1e3, "leaf width"),
        (p.widest_at, 0.05, 0.95, "leaf widest point"),
        (p.base_fullness, 0.2, 8., "leaf base fullness"),
        (p.tip_sharpness, 0.2, 8., "leaf tip sharpness"),
        (p.cup, -2., 2., "leaf cup"),
        (p.curl, -2., 2., "leaf curl"),
        (p.lobe_depth, 0., 1., "leaf lobe depth"),
        (p.section_roundness, 0., 1., "leaf section roundness"),
    ] {
        range(v, l, h, n)?;
    }
    range(
        p.connector_length,
        1e-6,
        p.length,
        "foliage connector length",
    )?;
    if p.lobe_count > 8 {
        return Err(Error::InvalidInput("leaf lobe count"));
    }
    if !(2..=64).contains(&p.axial_segments) || !(2..=64).contains(&p.cross_segments) {
        return Err(Error::InvalidInput("leaf segments"));
    }
    // A lobed margin needs a section at every crest and every sinus, plus the
    // base and the tip. Both sides are linear in a blend, and the blend rounds
    // counts the way that keeps them so.
    if p.lobe_depth > 0. && p.axial_segments as usize + 1 < 2 * p.lobe_count as usize + 2 {
        return Err(Error::InvalidInput(
            "leaf axial segments too few for the lobe count",
        ));
    }
    if p.card {
        if p.lobe_count > 0 || p.section_roundness > 0. {
            return Err(Error::InvalidInput(
                "leaf card carries no lobes and no section roundness",
            ));
        }
        return card(p);
    }
    let columns = p.cross_segments + p.cross_segments % 2;
    let rows = p.axial_segments - 1;
    let mut e = Element {
        positions: Vec::with_capacity((2 + rows * (columns + 1)) as usize),
        indices: Vec::with_capacity((6 * columns * rows) as usize),
        ..Element::default()
    };
    // The blade stands on its connector, and narrows to a point at either end:
    // one vertex at the base, one at the tip, a section apiece.
    e.positions.push(Vec3::new(0., p.connector_length, 0.));
    let mut sections = Vec::with_capacity(p.axial_segments as usize + 1);
    sections.push(0..1);
    for row in 1..=rows {
        let t = row as f64 / p.axial_segments as f64;
        let half = outline::half_width(&p, t);
        let start = e.positions.len();
        for col in 0..=columns {
            let (x, z) = outline::section(&p, half, 2. * col as f64 / columns as f64 - 1.);
            e.positions.push(Vec3::new(
                x,
                p.connector_length + t * p.length,
                z + p.curl * p.length * t * t,
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
    let first = |r, c| 1 + r * (columns + 1) + c;
    for c in 0..columns {
        e.indices.extend([0, first(0, c + 1), first(0, c)]);
    }
    for r in 0..rows - 1 {
        for c in 0..columns {
            let (a, b) = (first(r, c), first(r, c + 1));
            let (cc, d) = (first(r + 1, c + 1), first(r + 1, c));
            e.indices.extend([a, b, cc, a, cc, d]);
        }
    }
    for c in 0..columns {
        e.indices
            .extend([first(rows - 1, c), first(rows - 1, c + 1), tip]);
    }
    e.anatomy = Some(AnatomyGeometry {
        // The unit a section this round reads as. It names the geometry for a
        // measurement, and nothing in the build branches on it.
        unit: if p.section_roundness >= 0.5 {
            FoliageUnit::Needle
        } else {
            FoliageUnit::Leaf
        },
        vertices: 0..e.positions.len(),
        indices: 0..e.indices.len(),
        sections,
    });
    // A flat blade hangs on a petiole and a shaft sits on a woody peg: one
    // radius, read from the same roundness the section is built from.
    connector(
        &mut e,
        p.connector_length,
        p.width * (0.018 + p.section_roundness * (0.175 - 0.018)),
    );
    // Match the authored float32 element before culling or computing bounds.
    for v in &mut e.positions {
        *v = Vec3::new(v.x as f32 as f64, v.y as f32 as f64, v.z as f32 as f64);
    }
    let (buffer, list) = {
        let a = e.anatomy.as_ref().expect("anatomy just recorded");
        levels::build(&e.positions, &e.indices, &a.sections)
    };
    (e.level_indices, e.levels) = (buffer, list);
    e.validate()?;
    Ok(e)
}

/// Two triangles standing in for the element at distance. A card carries no
/// outline: it is the authored extent and nothing else.
fn card(p: ElementParams) -> Result<Element> {
    let half = (p.width / 2.) as f32 as f64;
    let length = p.length as f32 as f64;
    let mut e = Element {
        positions: vec![
            Vec3::new(-half, 0., 0.),
            Vec3::new(half, 0., 0.),
            Vec3::new(half, length, 0.),
            Vec3::new(-half, length, 0.),
        ],
        indices: vec![0, 1, 2, 0, 2, 3],
        ..Element::default()
    };
    // Two triangles are already the coarsest a card gets.
    (e.level_indices, e.levels) = levels::build(&e.positions, &e.indices, &[]);
    Ok(e)
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
