//! The grown crown's shoulder (fn-198 host decision 11a): the least
//! shoulder whose outline holds every node of the crown, on the pinned
//! transcendentals of the outline the cull reads (`quadrant`).
use crate::envelope::quadrant;
use crate::math::Vec3;

/// The shoulders searched, from nearly a cone's point to nearly a
/// cylinder: a crown held by none takes the widest. The narrowest is what
/// a crown with nothing to hold takes, the palm's, whose crown has no
/// span. The range holds every shipped tree's measured need (host, fn-198
/// task 6: 2.8 to 26.5 at 40 and 80).
const NARROWEST: f64 = 1.0 / 16.0;
const WIDEST: f64 = 64.0;
/// Halvings of the search's logarithmic range: about 1e-11 of a shoulder.
const HALVINGS: usize = 40;
/// Bins of the way from the widest height to an end, for the front.
pub const BINS: usize = 1024;

/// The least shoulder in the range whose outline (`radius_at` of an
/// envelope with this `base`, `height`, `radius` and `fullness`) holds
/// every node strictly between the crown base and the top, of the nodes
/// at `points`.
pub(super) fn contained(points: impl Iterator<Item = Vec3> + Clone, shape: [f64; 4]) -> f64 {
    let front = front(points, shape);
    let holds = |s: f64| front.iter().all(|&(p, q)| quadrant(p, s) >= q);
    if holds(NARROWEST) {
        return NARROWEST;
    }
    if !holds(WIDEST) {
        return WIDEST;
    }
    let (mut lo, mut hi) = (NARROWEST, WIDEST);
    for _ in 0..HALVINGS {
        let mid = (lo * hi).sqrt();
        if holds(mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    hi
}

/// A crown node at `position` as (p, q): its way from the widest height to
/// the end it lies toward, and its reach as a share of the widest; none
/// outside the crown's span, or on the stem's vertical.
pub fn crown_point(
    position: Vec3,
    [base, height, radius, fullness]: [f64; 4],
) -> Option<(f64, f64)> {
    let (y, reach) = (position.y, super::reach(position));
    if y <= base || y >= height || reach <= 0.0 {
        return None;
    }
    let t = (y - base) / (height - base);
    let p = if t < fullness {
        1.0 - t / fullness
    } else {
        (t - fullness) / (1.0 - fullness)
    };
    (p > 0.0).then_some((p.min(1.0), reach / radius))
}

/// The bin of the way `p`.
fn bin(p: f64) -> usize {
    ((p * BINS as f64) as usize).min(BINS - 1)
}

/// The most reach (q) each of the `BINS` bins of the way holds, of the
/// nodes at `points`; the bins of parts of a tree combine by their most.
pub fn bins(points: impl Iterator<Item = Vec3>, shape: [f64; 4]) -> Vec<f64> {
    let mut most = vec![0.0f64; BINS];
    let [base, height, radius, _] = shape;
    if !(radius > 0.0 && height > base) {
        return most;
    }
    for (p, q) in points.filter_map(|v| crown_point(v, shape)) {
        most[bin(p)] = most[bin(p)].max(q);
    }
    most
}

/// The most any bin further out than each holds: a node at or under it
/// is passed by another, further out and as wide.
pub fn beyond(most: &[f64]) -> Vec<f64> {
    let mut beyond = vec![0.0f64; most.len()];
    for b in (0..most.len().saturating_sub(1)).rev() {
        beyond[b] = beyond[b + 1].max(most[b + 1]);
    }
    beyond
}

/// Every crown node as (p, q). Only the nodes no other node passes in both
/// are kept: an outline holds a node where `p^s + q^s <= 1`, so one that
/// holds the front holds them all. The bins only sift: the front is the
/// nodes no other passes, whatever the order or the bins.
fn front(points: impl Iterator<Item = Vec3> + Clone, shape: [f64; 4]) -> Vec<(f64, f64)> {
    let [base, height, radius, _] = shape;
    if !(radius > 0.0 && height > base) {
        return Vec::new();
    }
    let beyond = beyond(&bins(points.clone(), shape));
    let mut kept: Vec<(f64, f64)> = points
        .filter_map(|v| crown_point(v, shape))
        .filter(|&(p, q)| q > beyond[bin(p)])
        .collect();
    kept.sort_by(|a, b| b.0.total_cmp(&a.0).then(b.1.total_cmp(&a.1)));
    let mut front = Vec::new();
    let mut reach = 0.0f64;
    for (p, q) in kept {
        if q > reach {
            front.push((p, q));
            reach = q;
        }
    }
    front
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(x: f64, y: f64) -> Vec3 {
        Vec3::new(x, y, 0.0)
    }

    /// The fit holds every node, and no narrower shoulder does; a crown
    /// with no span takes the narrowest.
    #[test]
    fn the_shoulder_is_the_least_that_holds_the_crown() {
        let nodes: Vec<Vec3> = (1..40)
            .flat_map(|i| {
                let y = f64::from(i) * 0.25;
                let w = (y - 0.0) * (10.0 - y) / 25.0;
                [node(w, y), node(-0.5 * w, y), node(0.0, y)]
            })
            .collect();
        let shape = [0.0, 10.0, 1.0, 0.5];
        let s = contained(nodes.iter().copied(), shape);
        let front = front(nodes.iter().copied(), shape);
        assert!(front.iter().all(|&(p, q)| quadrant(p, s) >= q));
        assert!(front
            .iter()
            .any(|&(p, q)| quadrant(p, s * (1.0 - 1e-6)) < q));
        assert!((NARROWEST..WIDEST).contains(&s), "{s}");
        assert_eq!(
            contained(nodes.iter().copied(), [10.0, 10.0, 1.0, 0.0]),
            NARROWEST
        );
    }
}
