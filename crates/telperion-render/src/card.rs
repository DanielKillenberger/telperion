//! The floret's far level (fn-246): below `CARD_PIXELS` across, a floret is
//! drawn as one card in its petals' albedo instead of every petal and
//! stamen. The card is the renderer's own level, put in front of the core's
//! coarsest, so generation is unchanged and selection chooses it by
//! projected size exactly as it chooses the others.
use std::borrow::Cow;
use telperion_core::{
    foliage::{Element, Level},
    math::Vec3,
};

/// The size on screen, in pixels across the floret's bounding sphere, below
/// which it is drawn as its card. Owner decision (fn-246 FPS.md, 2026-10-09,
/// on the labelled comparison https://claude.ai/artifact/8WwaynDsif2JekPvHDDtDi):
/// 4 px, which puts Telperion's whole view 4.43 mean code values over its
/// unthinned distance to the 4x crown truth (`examples/crown_truth`), past
/// the 3.0 bound the leaves were fitted to, accepted on the owner's look.
pub const CARD_PIXELS: f64 = 4.0;

/// The element with its card in front of its levels, or the element as it
/// is where the card would not be the coarsest level, where its ladder has
/// no room for one more (selection holds `MAX_LEVELS`), or where there is
/// nothing to stand for.
pub fn carded(element: &Element) -> Cow<'_, Element> {
    let radius = f64::from(crate::select::frame::sphere(element)[3]);
    // Selection takes a level whose deviation times the pixels a metre is
    // under half a pixel; a deviation of r / CARD_PIXELS is that level for a
    // floret under CARD_PIXELS across.
    let deviation = radius / CARD_PIXELS;
    let Some(card) = card(element) else {
        return Cow::Borrowed(element);
    };
    // A flower's rachis levels stand in front of its florets' (fn-253);
    // the card goes in front of the florets'.
    let front = rachis_levels(element);
    if element.levels.len() >= crate::MAX_LEVELS
        || element
            .levels
            .get(front)
            .is_none_or(|l| l.deviation >= deviation)
    {
        return Cow::Borrowed(element);
    }
    let mut out = element.clone();
    let base = out.positions.len() as u32;
    out.positions.extend(card.corners);
    if !out.coords.is_empty() {
        out.coords
            .extend([0.0, -1.0, 1.0, -1.0, 1.0, 1.0, 0.0, 1.0]);
    }
    // The card is a patch of colour, not a floret: under CARD_PIXELS its
    // pedicel is far below a pixel and is not drawn (fn-253).
    let at = out.level_indices.len() as u32;
    out.level_indices
        .extend([0, 1, 2, 0, 2, 3].map(|i| base + i));
    let indices = at..out.level_indices.len() as u32;
    out.levels.insert(front, Level { indices, deviation });
    Cow::Owned(out)
}

/// How many of `element`'s levels, from the coarsest, are a flower's
/// rachis's alone (fn-253): none on a leaf.
pub(crate) fn rachis_levels(element: &Element) -> usize {
    element
        .levels
        .iter()
        .take_while(|l| rachis(element, l))
        .count()
}

/// Whether `level` draws stalk triangles alone: a flower's rachis's.
fn rachis(element: &Element, level: &Level) -> bool {
    let range = level.indices.start as usize..level.indices.end as usize;
    let indices = element.level_indices.get(range).unwrap_or_default();
    !indices.is_empty() && indices.iter().all(|&i| stalk(element, i))
}

/// Whether vertex `i` is a flower's stalk's, named by a surface coordinate
/// below zero (`bloom/stalk.rs`).
fn stalk(element: &Element, i: u32) -> bool {
    element.coords.get(2 * i as usize).is_some_and(|x| *x < 0.0)
}

struct Card {
    corners: [Vec3; 4],
}

/// A square of the floret's one-sided petal area, about its area-weighted
/// centre, across the direction its faces mostly look. A flat face seen
/// from every direction shows half its area on average, as the floret's
/// petals do, so the card covers what the floret covers.
fn card(element: &Element) -> Option<Card> {
    let triangles: Vec<[Vec3; 3]> = element
        .indices
        .as_chunks::<3>()
        .0
        .iter()
        .map(|t| t.map(|i| element.positions[i as usize]))
        .collect();
    let faces: Vec<(Vec3, f64, Vec3)> = triangles
        .iter()
        .map(|[a, b, c]| {
            let cross = (*b - *a).cross(*c - *a);
            (cross, cross.length() * 0.5, (*a + *b + *c) / 3.0)
        })
        .filter(|(_, area, _)| *area > 0.0)
        .collect();
    let area: f64 = faces.iter().map(|f| f.1).sum();
    if area <= 0.0 {
        return None;
    }
    let centre = faces.iter().fold(Vec3::ZERO, |s, f| s + f.2 * f.1) / area;
    // The direction the faces mostly look, sign-free: the leading axis of
    // their area-weighted normal scatter, by power iteration from each
    // axis, the one that scatters most kept. A single start can sit in the
    // scatter's null space (flat petals across it) and never leave it.
    let scatter = |axis: Vec3| {
        faces.iter().fold(Vec3::ZERO, |s, (cross, _, _)| {
            s + *cross * (cross.dot(axis) / cross.length().max(1e-12))
        })
    };
    let lead = |start: Vec3| {
        let mut axis = start;
        for _ in 0..32 {
            let next = scatter(axis);
            if next.length_squared() <= 0.0 {
                return None;
            }
            axis = next.normalized();
        }
        Some((scatter(axis).dot(axis), axis))
    };
    let axis = [Vec3::X, Vec3::Y, Vec3::Z]
        .into_iter()
        .filter_map(lead)
        .max_by(|a, b| a.0.total_cmp(&b.0))?
        .1;
    let side = if axis.cross(Vec3::Y).length_squared() > 1e-6 {
        Vec3::Y
    } else {
        Vec3::X
    };
    let u = side.cross(axis).normalized();
    let v = axis.cross(u);
    let half = area.sqrt() * 0.5;
    Some(Card {
        corners: [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
            .map(|(a, b)| centre + u * (a * half) + v * (b * half)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A hand floret: three petals about the origin, a coarse level of one
    /// of them and the finest of all three.
    fn floret() -> Element {
        let p = |x, y, z| Vec3::new(x, y, z);
        Element {
            positions: vec![
                p(0.0, 0.0, 0.0),
                p(0.01, 0.0, 0.0),
                p(0.0, 0.01, 0.0),
                p(-0.01, 0.0, 0.002),
                p(0.0, -0.01, 0.002),
            ],
            indices: vec![0, 1, 2, 0, 2, 3, 0, 3, 4],
            coords: vec![0.0; 10],
            level_indices: vec![0, 1, 2, 0, 1, 2, 0, 2, 3, 0, 3, 4],
            levels: vec![
                Level {
                    indices: 0..3,
                    deviation: 0.001,
                },
                Level {
                    indices: 3..12,
                    deviation: 0.0,
                },
            ],
            ..Element::default()
        }
    }

    #[test]
    fn a_floret_gets_a_card_of_its_petal_area_in_front_of_its_levels() {
        let element = floret();
        let carded = carded(&element);
        let levels = &carded.levels;
        assert_eq!(levels.len(), element.levels.len() + 1, "no card level");
        assert!(levels.windows(2).all(|w| w[0].deviation > w[1].deviation));
        assert_eq!(carded.coords.len(), 2 * carded.positions.len());
        let range = levels[0].indices.clone();
        let quad = &carded.level_indices[range.start as usize..range.start as usize + 6];
        let [a, b, c, d] =
            [quad[0], quad[1], quad[2], quad[5]].map(|i| carded.positions[i as usize]);
        let card = (b - a).cross(c - a).length() * 0.5 + (c - a).cross(d - a).length() * 0.5;
        let petals: f64 = element
            .indices
            .as_chunks::<3>()
            .0
            .iter()
            .map(|t| {
                let [p, q, r] = t.map(|i| element.positions[i as usize]);
                (q - p).cross(r - p).length() * 0.5
            })
            .sum();
        assert!((card - petals).abs() <= 1e-9, "{card} against {petals}");
        let kept = element.level_indices.len();
        assert_eq!(&carded.level_indices[..kept], &element.level_indices[..]);
        assert_eq!(&levels[1..], &element.levels[..]);
        // The card faces where the petals mostly do: about +Z, two of the
        // three petals tilted eleven degrees off it.
        let n = (b - a).cross(c - a).normalized();
        assert!(n.z.abs() > 0.95, "{n:?}");
    }

    #[test]
    fn a_card_that_would_not_be_coarsest_or_has_no_room_is_left_out() {
        let mut element = floret();
        element.levels[0].deviation = 1.0;
        assert!(matches!(carded(&element), Cow::Borrowed(_)));
        let mut full = floret();
        full.levels = (0..crate::MAX_LEVELS)
            .map(|i| Level {
                indices: 3..12,
                deviation: 1e-4 / (i + 1) as f64,
            })
            .collect();
        assert!(
            matches!(carded(&full), Cow::Borrowed(_)),
            "a seventeenth level"
        );
    }

    #[test]
    fn flat_petals_across_any_axis_face_their_card_along_their_normal() {
        for flat in [0usize, 1, 2] {
            let mut element = floret();
            // Lay the petals flat, then turn them so they face along `flat`.
            for p in &mut element.positions {
                let q = Vec3::new(p.x, p.y, 0.0);
                *p = match flat {
                    0 => Vec3::new(0.0, q.x, q.y),
                    1 => Vec3::new(q.y, 0.0, q.x),
                    _ => q,
                };
            }
            let carded = carded(&element);
            let range = carded.levels[0].indices.clone();
            let quad = &carded.level_indices[range.start as usize..range.start as usize + 6];
            let [a, b, c] = [quad[0], quad[1], quad[2]].map(|i| carded.positions[i as usize]);
            let n = (b - a).cross(c - a).normalized();
            let along = [n.x, n.y, n.z][flat].abs();
            assert!(along > 0.999, "petals facing axis {flat}: card faces {n:?}");
        }
    }

    /// fn-253: the card goes behind a flower's rachis levels, which stay
    /// in front, and is its quad alone: under CARD_PIXELS the pedicel is
    /// not drawn.
    #[test]
    fn the_card_follows_the_rachis_levels_and_drops_the_pedicel() {
        let mut e = floret();
        let p = |x, y, z| Vec3::new(x, y, z) * 1e-6;
        e.positions
            .extend([p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, 1.0)]);
        e.coords.extend([-1.0, 0.0, -1.0, 1.0, -1.0, 2.0]);
        e.indices.extend([5, 6, 7]);
        e.level_indices.extend([5, 6, 7]);
        e.levels[1].indices.end += 3;
        e.positions
            .extend([p(2.0, 0.0, 0.0), p(0.0, 2.0, 0.0), p(0.0, 0.0, 2.0)]);
        e.coords.extend([-2.0, 0.0, -2.0, 1.0, -2.0, 2.0]);
        let at = e.level_indices.len() as u32;
        e.level_indices.extend([8, 9, 10, 8, 10, 9]);
        e.levels.insert(
            0,
            Level {
                indices: at + 3..at + 6,
                deviation: 1e29,
            },
        );
        e.levels.insert(
            0,
            Level {
                indices: at..at + 3,
                deviation: 1e30,
            },
        );
        let carded = carded(&e);
        assert_eq!(
            &carded.levels[..2],
            &e.levels[..2],
            "the rachis levels first"
        );
        let card = carded.levels[2].indices.clone();
        assert_eq!(card.len(), 6, "the quad alone");
        assert_eq!(&carded.levels[3..], &e.levels[2..]);
    }
}
