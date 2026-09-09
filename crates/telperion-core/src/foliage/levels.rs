//! Nested simplifications of one foliage element, chosen by outline deviation.
//!
//! A level is a subset of the element's transverse sections. The base, the tip
//! and the widest section are always kept — dropping the widest is what turns a
//! lobed blade into a sliver — and the rest are added one at a time, always the
//! section sitting furthest from the surface spanned between its kept
//! neighbours, until nothing dropped lies outside the tolerance. Halving the
//! tolerance gives the next level down. The rule reads sections, positions and
//! the element's own triangles, so the lobed blade, the four-sided needle and
//! the generic grid all pass through it with no anatomy branch.
//!
//! A level's triangles are the element's own, with every vertex moved to the
//! nearest kept section: a coarser level's vertices are therefore a subset of a
//! finer one's, and its lateral topology is the element's by construction
//! rather than by imitation. Geometry outside the sections — the connector, a
//! cap centre — travels with the section vertex it sits closest to, so the
//! connector's millimetre of stem collapses onto the blade's base and leaves
//! the coarse levels without an exemption of its own.
//!
//! Halving the tolerance offers far more levels than are worth keeping, and
//! each one kept costs an index list, a counter and a draw call every frame it
//! is selected against. A level is therefore emitted only when it carries at
//! least twice the triangles of the last one emitted, so the ladder is a
//! doubling one from single digits to the whole element and its length grows
//! with the logarithm of the element rather than with the tolerance.
use super::Element;
use crate::math::Vec3;
use std::collections::HashSet;
use std::ops::Range;

/// One nested simplification: a range of `Element::level_indices` and the
/// tolerance, in metres, that chose it.
#[derive(Debug, Clone, PartialEq)]
pub struct Level {
    pub indices: Range<u32>,
    pub deviation: f64,
}

/// Whether an element's levels are nested detail of the element itself:
/// strictly decreasing deviations, ranges inside the shared buffer, real
/// triangles on real vertices, and a finest level that is the element's own
/// index list. An element carrying no levels passes; one built by hand has
/// nothing to contradict.
pub(super) fn nested(e: &Element) -> bool {
    let Some(finest) = e.levels.last() else {
        return true;
    };
    !(e.levels
        .windows(2)
        .any(|w| w[0].deviation <= w[1].deviation)
        || e.levels.iter().any(|l| {
            l.indices.start > l.indices.end
                || l.indices.end as usize > e.level_indices.len()
                || !l.indices.len().is_multiple_of(3)
        })
        || e.level_indices[finest.indices.start as usize..finest.indices.end as usize]
            != e.indices[..]
        || e.level_indices.as_chunks::<3>().0.iter().any(|t| {
            t.iter().any(|i| *i as usize >= e.positions.len()) || {
                let [p, q, r] = [t[0], t[1], t[2]].map(|i| e.positions[i as usize]);
                (q - p).cross(r - p).length_squared() <= 0.
            }
        }))
}

/// Returns the shared index buffer and the levels addressing it, coarsest
/// first, each at least twice the triangles of the one above it. The last
/// level is always the element's own index list, copied byte for byte, at
/// deviation zero.
pub(super) fn build(
    positions: &[Vec3],
    indices: &[u32],
    sections: &[Range<usize>],
) -> (Vec<u32>, Vec<Level>) {
    let mut buffer = Vec::new();
    let mut levels = Vec::new();
    if indices.is_empty() {
        return (buffer, levels);
    }
    // Fewer than three sections leaves nothing to drop between base and tip.
    if sections.len() >= 3 {
        let home = homes(positions, sections);
        let mut kept = anchors(positions, sections);
        let mut order = Vec::new();
        let mut errors = Vec::new();
        loop {
            let (error, section) =
                worst(positions, indices, &home, sections, &kept).unwrap_or((0.0, 0));
            // Nothing dropped, nothing outside the surface, or a distance that
            // will not compare: the refinement is finished either way.
            let refine = error > 0.0;
            errors.push(error);
            if !refine {
                break;
            }
            kept.insert(kept.partition_point(|&k| k < section), section);
            order.push(section);
        }
        let mut tolerance = errors[0];
        let mut built = usize::MAX;
        let mut carried = 0;
        while tolerance.is_finite() && tolerance > 0.0 {
            let step = errors
                .iter()
                .position(|e| *e <= tolerance)
                .expect("the fully refined set has no error left");
            if step >= order.len() {
                // Every section is kept: the element's own indices say it best.
                break;
            }
            if step != built {
                built = step;
                let mut kept = anchors(positions, sections);
                for &section in &order[..step] {
                    kept.insert(kept.partition_point(|&k| k < section), section);
                }
                let triangles = simplify(indices, &home, sections, &kept, 0..sections.len());
                // A level is kept only when it is a level apart from the last
                // one: at least twice its triangles. The steps in between cost
                // a list, a counter and a draw call each for a few per cent of
                // the element, which is not a level, it is a tail.
                if triangles.len() >= (carried * 2).max(1) {
                    carried = triangles.len();
                    let start = buffer.len() as u32;
                    buffer.extend(triangles.into_iter().flatten());
                    levels.push(Level {
                        indices: start..buffer.len() as u32,
                        deviation: tolerance,
                    });
                }
            }
            tolerance /= 2.0;
        }
    }
    let start = buffer.len() as u32;
    buffer.extend_from_slice(indices);
    levels.push(Level {
        indices: start..buffer.len() as u32,
        deviation: 0.0,
    });
    (buffer, levels)
}

/// The section each vertex travels with, and its place within that section.
/// A vertex outside every section joins the section vertex nearest to it.
fn homes(positions: &[Vec3], sections: &[Range<usize>]) -> Vec<(usize, usize)> {
    let mut home = vec![(usize::MAX, 0); positions.len()];
    for (s, section) in sections.iter().enumerate() {
        for (offset, vertex) in section.clone().enumerate() {
            home[vertex] = (s, offset);
        }
    }
    for vertex in 0..positions.len() {
        if home[vertex].0 == usize::MAX {
            home[vertex] = sections
                .iter()
                .enumerate()
                .flat_map(|(s, r)| r.clone().enumerate().map(move |(offset, v)| (s, offset, v)))
                .min_by(|a, b| {
                    positions[a.2]
                        .distance_squared(positions[vertex])
                        .total_cmp(&positions[b.2].distance_squared(positions[vertex]))
                })
                .map(|(s, offset, _)| (s, offset))
                .expect("at least one section vertex");
        }
    }
    home
}

/// The sections no level may drop: base, tip, and the greatest lateral extent.
fn anchors(positions: &[Vec3], sections: &[Range<usize>]) -> Vec<usize> {
    let extent = |section: &Range<usize>| {
        let row = &positions[section.clone()];
        row.iter()
            .flat_map(|a| row.iter().map(move |b| a.distance_squared(*b)))
            .fold(0.0_f64, f64::max)
    };
    let widest = (0..sections.len())
        .max_by(|a, b| extent(&sections[*a]).total_cmp(&extent(&sections[*b])))
        .expect("at least one section");
    let mut kept = vec![0, widest, sections.len() - 1];
    kept.sort_unstable();
    kept.dedup();
    kept
}

/// The dropped vertex furthest from the surface spanned between its kept
/// neighbours, and the section it belongs to. Measuring one gap at a time
/// rather than against the whole coarse element only overstates the distance,
/// which is the safe direction for a tolerance.
fn worst(
    positions: &[Vec3],
    indices: &[u32],
    home: &[(usize, usize)],
    sections: &[Range<usize>],
    kept: &[usize],
) -> Option<(f64, usize)> {
    let mut worst: Option<(f64, usize)> = None;
    for pair in kept.windows(2) {
        let (low, high) = (pair[0], pair[1]);
        if high - low < 2 {
            continue;
        }
        let span = simplify(indices, home, sections, &[low, high], low..high + 1);
        for (section, row) in sections.iter().enumerate().take(high).skip(low + 1) {
            for vertex in row.clone() {
                let distance = span
                    .iter()
                    .map(|t| {
                        distance_to_triangle(
                            positions[vertex],
                            positions[t[0] as usize],
                            positions[t[1] as usize],
                            positions[t[2] as usize],
                        )
                    })
                    .fold(f64::INFINITY, f64::min);
                if worst.is_none_or(|(furthest, _)| distance > furthest) {
                    worst = Some((distance, section));
                }
            }
        }
    }
    worst
}

/// The element's own triangles with every vertex moved to the nearest kept
/// section, keeping only those inside `span` that survive as triangles.
fn simplify(
    indices: &[u32],
    home: &[(usize, usize)],
    sections: &[Range<usize>],
    kept: &[usize],
    span: Range<usize>,
) -> Vec<[u32; 3]> {
    let target: Vec<usize> = (0..sections.len())
        .map(|s| {
            *kept
                .iter()
                .min_by_key(|k| (k.abs_diff(s), **k))
                .expect("at least one kept section")
        })
        .collect();
    let mut seen = HashSet::new();
    let mut triangles = Vec::new();
    for triangle in indices.as_chunks::<3>().0 {
        if triangle
            .iter()
            .any(|i| !span.contains(&home[*i as usize].0))
        {
            continue;
        }
        let moved = triangle.map(|i| {
            let (section, offset) = home[i as usize];
            let row = &sections[target[section]];
            (row.start + offset.min(row.len() - 1)) as u32
        });
        if moved[0] == moved[1] || moved[1] == moved[2] || moved[2] == moved[0] {
            continue;
        }
        let mut key = moved;
        key.sort_unstable();
        if seen.insert(key) {
            triangles.push(moved);
        }
    }
    triangles
}

/// Distance from a point to the nearest point of one triangle.
fn distance_to_triangle(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> f64 {
    let (ab, ac, ap) = (b - a, c - a, p - a);
    let (d1, d2) = (ab.dot(ap), ac.dot(ap));
    if d1 <= 0.0 && d2 <= 0.0 {
        return p.distance(a);
    }
    let bp = p - b;
    let (d3, d4) = (ab.dot(bp), ac.dot(bp));
    if d3 >= 0.0 && d4 <= d3 {
        return p.distance(b);
    }
    let cp = p - c;
    let (d5, d6) = (ab.dot(cp), ac.dot(cp));
    if d6 >= 0.0 && d5 <= d6 {
        return p.distance(c);
    }
    let face = d1 * d4 - d3 * d2;
    if face <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        return p.distance(a + ab * (d1 / (d1 - d3)));
    }
    let side = d5 * d2 - d1 * d6;
    if side <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        return p.distance(a + ac * (d2 / (d2 - d6)));
    }
    let far = d3 * d6 - d5 * d4;
    if far <= 0.0 && d4 - d3 >= 0.0 && d5 - d6 >= 0.0 {
        return p.distance(b + (c - b) * ((d4 - d3) / ((d4 - d3) + (d5 - d6))));
    }
    let total = face + side + far;
    if total <= 0.0 {
        // A triangle with no area is only ever as near as its corners.
        return p.distance(a).min(p.distance(b)).min(p.distance(c));
    }
    p.distance(a + ab * (side / total) + ac * (face / total))
}
