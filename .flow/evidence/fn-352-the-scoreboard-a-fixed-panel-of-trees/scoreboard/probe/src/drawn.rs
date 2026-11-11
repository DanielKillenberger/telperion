//! Measures read off the drawn tree: the pipeline's `Tree` (y up, metres)
//! and its dressed leaves.
use telperion_core::foliage::transform_point;
use telperion_core::math::Vec3;
use telperion_core::mesh::TreeMesh;
use telperion_core::tree::Tree;

pub struct Wood {
    /// The trunk's diameter at 1.3 m above its base; none where it never
    /// stands that high.
    pub dbh: Option<f64>,
}

/// The trunk is the thickest wood from the root, child by child (fn-341's
/// read, on the drawn tree). Its diameter at 1.3 m is the edge spanning that
/// height, its radius taken linearly between the edge's proximal and distal
/// radii. The drawn mesh's root flare (`pipeline/surface.rs`) is not in it.
pub fn wood(tree: &Tree) -> Wood {
    let n = &tree.nodes;
    let mut kids: Vec<Vec<usize>> = vec![Vec::new(); n.len()];
    for (i, node) in n.iter().enumerate() {
        if let Some(p) = node.parent {
            kids[p as usize].push(i);
        }
    }
    let base = n[0].position.y;
    let mut at = 0usize;
    loop {
        let Some(&next) = kids[at]
            .iter()
            .max_by(|a, b| n[**a].start_radius.total_cmp(&n[**b].start_radius))
        else {
            return Wood { dbh: None };
        };
        let (lo, hi) = (n[at].position.y - base, n[next].position.y - base);
        if lo <= 1.3 && 1.3 <= hi && hi > lo {
            let t = (1.3 - lo) / (hi - lo);
            let r = n[next].start_radius + t * (n[next].radius - n[next].start_radius);
            return Wood { dbh: Some(2.0 * r) };
        }
        at = next;
    }
}

pub struct Leaves {
    pub count: usize,
    /// One-sided blade area in m², each triangle of the blade counted once
    /// (a needle's whole surface, as `species_metrics` reads it).
    pub area: f64,
    /// The crown's projection on the ground: the convex hull of every
    /// leaf's place, in m².
    pub crown: f64,
}

pub fn leaves(mesh: &TreeMesh) -> Leaves {
    let element = &mesh.foliage.element;
    let triangles: &[u32] = match &element.anatomy {
        Some(a) => &element.indices[a.indices.clone()],
        None => &element.indices,
    };
    let instances = &mesh.foliage.instances;
    let mut area = 0.0;
    for m in instances.matrices() {
        for t in triangles.chunks_exact(3) {
            let p = |k: usize| transform_point(&m, element.positions[t[k] as usize]);
            let (a, b, c) = (p(0), p(1), p(2));
            area += (b - a).cross(c - a).length() / 2.0;
        }
    }
    let points: Vec<(f64, f64)> = (0..instances.len())
        .map(|i| instances.position(i))
        .map(|p: Vec3| (p.x, p.z))
        .collect();
    Leaves {
        count: instances.len(),
        area,
        crown: hull_area(points),
    }
}

/// The area of the convex hull of `p` (Andrew's monotone chain).
fn hull_area(mut p: Vec<(f64, f64)>) -> f64 {
    if p.len() < 3 {
        return 0.0;
    }
    p.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    let cross = |o: (f64, f64), a: (f64, f64), b: (f64, f64)| {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    };
    let mut hull: Vec<(f64, f64)> = Vec::new();
    for pass in 0..2 {
        let start = hull.len();
        let iter: Box<dyn Iterator<Item = &(f64, f64)>> = if pass == 0 {
            Box::new(p.iter())
        } else {
            Box::new(p.iter().rev())
        };
        for &q in iter {
            while hull.len() >= start + 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], q) <= 0.0 {
                hull.pop();
            }
            hull.push(q);
        }
        hull.pop();
    }
    let k = hull.len();
    (0..k)
        .map(|i| {
            let (a, b) = (hull[i], hull[(i + 1) % k]);
            a.0 * b.1 - b.0 * a.1
        })
        .sum::<f64>()
        .abs()
        / 2.0
}
