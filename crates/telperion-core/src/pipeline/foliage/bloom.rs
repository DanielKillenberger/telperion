//! Flowers (fn-216, option B; FLOWERS-DESIGN.md, host decisions 1 to 4).
//! The engine's ages say which nodes may flower (`Tree::blooms`); here each
//! one draws whether it does and what grows there. An inflorescence is an
//! axis of florets that leaves the bud's side and turns toward hanging or
//! standing by its droop; each floret stands on a stalk off the axis and is
//! one instance of the organ element, its petals turned about it. Every
//! draw is keyed by the bearing node's lineage and the floret's index, so a
//! floret is a pure function of (bloom, index, seed) and a twig added
//! anywhere moves none. Every row acts by degree: presence grows in past
//! its draw, the last floret and the last petal by their counts' fractions,
//! and the season scales a floret in and out, so a floret leaves at size
//! zero and nothing pops.
use super::{build_element, packed::Reference, BloomParams, Element, ElementParams, Instances};
use crate::{
    math::{Transcendental, Vec3},
    tree::{Bloom, Tree},
    Error, Result,
};
use std::f64::consts::{PI, TAU};
use telperion_space::fixed::{child, draw};

// The draws a bloom keys under its node's lineage, and a floret under
// `FLORET` and its index, are the lineage-key registry's.
use telperion_space::lineage_keys::{
    BLOOM as PRESENCE, BLOOM_ROLL as ROLL, FLORET, FLORET_OPENING as OPENING, FLORET_SIZE as SIZE,
};
/// The share of flowering over which a bloom grows in past its draw. The
/// draws stand on [0, 1 - WINDOW), so every bloom is whole at a share of 1
/// and none grows in faster than any other.
const WINDOW: f64 = 0.1;
/// The share of a floret's life it takes to open, and to fade.
const GROW: f64 = 0.25;
/// The segments the axis is laid in as it turns.
const SEGMENTS: usize = 8;
/// The least size a floret is drawn at, as a share of its full size: below
/// it a floret stands under a hundredth of a millimetre and is dropped.
const LEAST: f64 = 1e-3;

/// A draw keyed by `key` under `part`, uniform on [0, 1).
fn unit(key: u64, part: u64) -> f64 {
    draw(child(key, part)) as f64 / (1u64 << 53) as f64
}

/// How grown a bloom of `share` is for its draw `u`: none until the share
/// passes the draw, whole a window past it.
fn presence(share: f64, u: f64) -> f64 {
    ((share - u * (1.0 - WINDOW)) / WINDOW).clamp(0.0, 1.0)
}

/// `v` as a unit vector, or `fallback` where it has no length.
fn or(v: Vec3, fallback: Vec3) -> Vec3 {
    if v.length_squared() > 1e-24 {
        v.normalized()
    } else {
        fallback
    }
}

/// A rise from 0 to 1 over [0, 1], flat at both ends.
fn smooth(x: f64) -> f64 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

/// The size the season gives a floret whose opening was drawn `u`, at time
/// `now` (a share of the year): none outside its own stretch of the window,
/// grown in over its first `GROW` and shrunk out over its last.
fn season(p: &BloomParams, now: f64, u: f64) -> f64 {
    let life = p.lasts * (1.0 - 0.5 * p.stagger);
    if life <= 0.0 {
        return 0.0;
    }
    let offset = u * (p.lasts - life);
    let x = (now - p.opens - offset).rem_euclid(1.0) / life;
    if x >= 1.0 {
        return 0.0;
    }
    smooth(x / GROW).min(smooth((1.0 - x) / GROW))
}

/// The time of year a request draws: its own, or the window's middle.
pub(crate) fn now(p: &BloomParams, season: Option<f64>) -> f64 {
    season.unwrap_or(p.opens + 0.5 * p.lasts).rem_euclid(1.0)
}

/// The organ element: one petal built from the bloom's rows and turned
/// `petals` times about the floret's axis (+Y), each opened `open` degrees
/// from it, facing out of the flower; the last petal is the count's
/// fraction of the others' size. Its levels are the petal's, replicated.
pub(crate) fn element(p: &BloomParams) -> Result<Element> {
    crate::catalogue::check(BloomParams::CHECKS, p, crate::catalogue::Site::Bloom)?;
    let petal = build_element(ElementParams {
        connector_length: p.petal_length * 0.05,
        length: p.petal_length,
        width: p.petal_width,
        widest_at: p.petal_widest,
        base_fullness: 0.85,
        tip_sharpness: 1.0,
        cup: p.petal_cup,
        curl: 0.0,
        lobe_count: 0,
        lobe_depth: 0.0,
        section_roundness: 0.0,
        axial_segments: 4,
        cross_segments: 2,
        card: false,
    })?;
    let count = p.petals.ceil() as usize;
    let last = p.petals - (count - 1) as f64;
    let (s, c) = p.open.to_radians().sin_cos_fixed();
    let mut e = Element {
        section_roundness: 0.0,
        ..Element::default()
    };
    let n = petal.positions.len() as u32;
    for j in 0..count {
        let turn = TAU * j as f64 / p.petals;
        let (sin, cos) = turn.sin_cos_fixed();
        let radial = Vec3::new(cos, 0.0, sin);
        let along = Vec3::Y * c + radial * s;
        let face = Vec3::Y * s - radial * c;
        let side = along.cross(face);
        let size = if j + 1 == count { last } else { 1.0 };
        e.positions.extend(petal.positions.iter().map(|v| {
            let at = (side * v.x + along * v.y + face * v.z) * size;
            Vec3::new(at.x as f32 as f64, at.y as f32 as f64, at.z as f32 as f64)
        }));
        e.coords.extend_from_slice(&petal.coords);
        let offset = j as u32 * n;
        e.indices.extend(petal.indices.iter().map(|i| i + offset));
    }
    for level in &petal.levels {
        let start = e.level_indices.len() as u32;
        let range = level.indices.start as usize..level.indices.end as usize;
        for j in 0..count as u32 {
            let indices = &petal.level_indices[range.clone()];
            e.level_indices.extend(indices.iter().map(|i| i + j * n));
        }
        e.levels.push(super::Level {
            indices: start..e.level_indices.len() as u32,
            deviation: level.deviation,
        });
    }
    e.validate()?;
    Ok(e)
}

/// One floret's transform before packing: where it stands and its frame.
type Matrix = [f32; 16];

/// Every flowering node's inflorescence, packed against a box of its own.
pub(crate) fn place(tree: &Tree, p: &BloomParams, season: Option<f64>) -> Result<Instances> {
    let matrices = matrices(tree, p, season)?;
    Ok(packed(&matrices, reference(&matrices, p)))
}

/// The same florets packed against `box`, which holds every one: the box
/// an executor drawing its own roots' florets shares (fn-229).
pub(crate) fn place_in(
    tree: &Tree,
    p: &BloomParams,
    season: Option<f64>,
    within: Reference,
) -> Result<Instances> {
    Ok(packed(&matrices(tree, p, season)?, within))
}

/// The farthest a floret stands from its node: along the axis, then its
/// stalk.
pub(crate) fn reach(p: &BloomParams) -> f64 {
    p.rachis + p.pedicel
}

fn matrices(tree: &Tree, p: &BloomParams, season: Option<f64>) -> Result<Vec<Matrix>> {
    let now = now(p, season);
    let mut matrices = Vec::new();
    for bloom in &tree.blooms {
        inflorescence(tree, bloom, p, now, &mut matrices)?;
    }
    Ok(matrices)
}

fn packed(matrices: &[Matrix], reference: Reference) -> Instances {
    let mut instances = Instances::new(reference);
    for m in matrices {
        instances.push(m);
    }
    instances
}

/// The box every floret stands in, a petal's reach round their positions.
fn reference(matrices: &[Matrix], p: &BloomParams) -> Reference {
    let at = |m: &Matrix| Vec3::new(m[12] as f64, m[13] as f64, m[14] as f64);
    let first = matrices.first().map_or(Vec3::ZERO, at);
    let (lo, hi) = matrices.iter().map(at).fold((first, first), |(lo, hi), v| {
        (
            Vec3::new(lo.x.min(v.x), lo.y.min(v.y), lo.z.min(v.z)),
            Vec3::new(hi.x.max(v.x), hi.y.max(v.y), hi.z.max(v.z)),
        )
    });
    let pad = Vec3::new(1.0, 1.0, 1.0) * p.petal_length;
    Reference::spanning(lo - pad, hi + pad)
}

/// One node's inflorescence, its florets appended to `out`.
fn inflorescence(
    tree: &Tree,
    bloom: &Bloom,
    p: &BloomParams,
    now: f64,
    out: &mut Vec<Matrix>,
) -> Result<()> {
    let node = &tree.nodes[bloom.node as usize];
    let key = node.identity.birth_order();
    let grown = presence(bloom.share, unit(key, PRESENCE));
    if grown <= 0.0 {
        return Ok(());
    }
    let from = node
        .parent
        .map_or(node.position, |q| tree.nodes[q as usize].position);
    let heading = or(node.position - from, Vec3::Y);
    let side = bloom.side - heading * heading.dot(bloom.side);
    let side = or(side, heading.perpendicular());
    let axis = Axis::laid(node.position, side, p);
    let roll = TAU * unit(key, ROLL);
    let count = p.florets.ceil() as usize;
    for i in 0..count {
        let fraction = (p.florets - i as f64).min(1.0);
        let t = ((i as f64 + 0.5) / p.florets).min(1.0);
        let floret = child(child(key, FLORET), i as u64);
        let scatter = 1.0 + p.scatter * (2.0 * unit(floret, SIZE) - 1.0);
        let size = grown
            * fraction
            * (1.0 - p.taper * t)
            * scatter
            * season(p, now, unit(floret, OPENING));
        if size < LEAST {
            continue;
        }
        let (at, tangent) = axis.at(t);
        let normal = tangent.perpendicular();
        // The golden angle about the axis, from the bloom's own roll.
        let turn = roll + i as f64 * PI * (3.0 - 5f64.sqrt());
        let radial = normal.rotate(tangent, turn);
        let (s, c) = p.spread.to_radians().sin_cos_fixed();
        let stalk = or(tangent * c + radial * s, tangent);
        let point = at + stalk * p.pedicel;
        let face = or(radial.cross(stalk), stalk.perpendicular());
        out.push(matrix(point, stalk, face, size)?);
    }
    Ok(())
}

/// The inflorescence's axis: a polyline from the node that leaves along the
/// bud's side and turns toward hanging (droop 1) or standing (droop -1).
struct Axis {
    points: [Vec3; SEGMENTS + 1],
    tangents: [Vec3; SEGMENTS],
}

impl Axis {
    fn laid(base: Vec3, side: Vec3, p: &BloomParams) -> Self {
        let toward = if p.droop >= 0.0 { -Vec3::Y } else { Vec3::Y };
        let mut points = [base; SEGMENTS + 1];
        let mut tangents = [side; SEGMENTS];
        let step = p.rachis / SEGMENTS as f64;
        for k in 0..SEGMENTS {
            let along = (k as f64 + 0.5) / SEGMENTS as f64;
            // Most of the turn is taken near the base, as a catkin hangs.
            let turned = p.droop.abs() * (1.0 - (1.0 - along).powi(3));
            tangents[k] = or(side.lerp(toward, turned), toward);
            points[k + 1] = points[k] + tangents[k] * step;
        }
        Self { points, tangents }
    }

    /// The point a share `t` along the axis, and its tangent there, which
    /// turns smoothly between the segments' midpoints.
    fn at(&self, t: f64) -> (Vec3, Vec3) {
        let x = t.clamp(0.0, 1.0) * SEGMENTS as f64;
        let k = (x as usize).min(SEGMENTS - 1);
        let point = self.points[k].lerp(self.points[k + 1], x - k as f64);
        let mid = x - 0.5;
        let j = (mid.max(0.0) as usize).min(SEGMENTS - 2);
        let share = (mid - j as f64).clamp(0.0, 1.0);
        let tangent = self.tangents[j].lerp(self.tangents[j + 1], share);
        (point, or(tangent, self.tangents[k]))
    }
}

/// A floret's transform: its axis along the stalk, its first petal's face
/// toward `face`, at `size` of the element.
fn matrix(point: Vec3, axis: Vec3, face: Vec3, size: f64) -> Result<Matrix> {
    let side = axis.cross(face);
    let m = [
        side.x * size,
        side.y * size,
        side.z * size,
        0.,
        axis.x * size,
        axis.y * size,
        axis.z * size,
        0.,
        face.x * size,
        face.y * size,
        face.z * size,
        0.,
        point.x,
        point.y,
        point.z,
        1.,
    ]
    .map(|v| v as f32);
    if !m.iter().all(|v| v.is_finite()) {
        return Err(Error::ResourceLimit("bloom transform overflow"));
    }
    Ok(m)
}

#[cfg(test)]
mod tests;
