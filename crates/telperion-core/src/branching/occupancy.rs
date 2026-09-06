//! Connected descendant redistribution after retention; no new nodes or buds.
use super::*;

pub(super) fn upper_descendants(tree: &mut Tree) {
    let first = tree.crossover;
    let upper = tree.nodes[..first]
        .iter()
        .map(|n| n.position.y)
        .fold(0.0, f64::max)
        * 0.75;
    let mut centre = Vec3::ZERO;
    let mut count = 0;
    for n in &tree.nodes[..first] {
        if n.position.y >= upper {
            centre += n.position;
            count += 1;
        }
    }
    if count == 0 {
        return;
    }
    centre = centre / count as f64;
    let mut owner = vec![usize::MAX; tree.nodes.len()];
    let mut centroids = vec![Vec3::ZERO; tree.nodes.len()];
    let mut counts = vec![0; tree.nodes.len()];
    for i in first..tree.nodes.len() {
        let parent = tree.nodes[i].parent.unwrap() as usize;
        owner[i] = if parent < first && tree.nodes[parent].position.y >= upper {
            i
        } else {
            owner[parent]
        };
        if owner[i] != usize::MAX && tree.nodes[i].kind == NodeKind::Twig {
            centroids[owner[i]] += tree.nodes[i].position;
            counts[owner[i]] += 1;
        }
    }
    let mut rotations = vec![None; tree.nodes.len()];
    for i in first..tree.nodes.len() {
        if owner[i] != i || counts[i] == 0 {
            continue;
        }
        let base = tree.nodes[tree.nodes[i].parent.unwrap() as usize].position;
        let from = (centroids[i] / counts[i] as f64 - base).normalized();
        let toward = (centre - base).normalized();
        if from.dot(toward) > 0.25 {
            continue;
        }
        let hinge = from.cross(toward);
        if hinge.length_squared() < 1e-12 {
            continue;
        }
        rotations[i] = Some((base, hinge.normalized(), 35.0_f64.to_radians()));
    }
    for i in first..tree.nodes.len() {
        if owner[i] != usize::MAX {
            if let Some((base, axis, angle)) = rotations[owner[i]] {
                tree.nodes[i].position = base + (tree.nodes[i].position - base).rotate(axis, angle);
            }
        }
    }
}

pub(super) fn transverse_curtains(tree: &mut Tree) {
    let mut sockets = vec![None; tree.nodes.len()];
    let original: Vec<_> = tree.nodes.iter().map(|n| n.position).collect();
    for i in 1..tree.nodes.len() {
        let parent = tree.nodes[i].parent.unwrap() as usize;
        sockets[i] = sockets[parent];
        let direction = (original[i] - original[parent]).normalized();
        if i < tree.crossover && sockets[i].is_none() && direction.y < -0.5 {
            sockets[i] = Some(original[parent]);
        }
        if let Some(base) = sockets[i] {
            let delta = original[i] - base;
            tree.nodes[i].position = base + Vec3::new(delta.x * 0.2, delta.y, delta.z * 0.2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Tree {
        let mut tree = Tree::default();
        for (i, (position, parent, kind)) in [
            (Vec3::ZERO, None, NodeKind::Structural),
            (Vec3::new(0., 10., 0.), Some(0), NodeKind::Structural),
            (Vec3::new(2., 10., 0.), Some(1), NodeKind::Structural),
            (Vec3::new(2.2, 9., 0.1), Some(2), NodeKind::Structural),
            (Vec3::new(3., 10.5, 0.), Some(2), NodeKind::Twig),
            (Vec3::new(2.5, 8., 0.3), Some(3), NodeKind::Twig),
        ]
        .into_iter()
        .enumerate()
        {
            tree.nodes.push(Node {
                position,
                parent,
                branch: i as u32,
                kind,
                ..Node::root()
            });
        }
        tree.crossover = 4;
        tree
    }
    #[test]
    fn curtain_keeps_primary_tip_and_socket_topology() {
        let mut tree = fixture();
        let before = tree.clone();
        transverse_curtains(&mut tree);
        tree.validate().unwrap();
        assert_eq!(tree.nodes.len(), before.nodes.len());
        for (i, (a, b)) in tree.nodes.iter().zip(&before.nodes).enumerate() {
            assert_eq!(a.parent, b.parent);
            assert_eq!(a.branch, b.branch);
            assert_eq!(a.kind, b.kind);
            assert_eq!(a.position.y, b.position.y);
            if [0, 1, 2, 4].contains(&i) {
                assert_eq!(a.position, b.position);
            }
        }
        assert!(
            tree.nodes[5].position.distance(tree.nodes[2].position)
                < before.nodes[5].position.distance(before.nodes[2].position)
        );
    }
    #[test]
    fn upper_rotations_preserve_connected_lengths_and_lower_axes() {
        let mut tree = fixture();
        tree.nodes[3].position.y = 5.;
        tree.nodes[5].position.y = 4.;
        let before = tree.clone();
        upper_descendants(&mut tree);
        tree.validate().unwrap();
        assert_ne!(tree.nodes[4].position, before.nodes[4].position);
        assert_eq!(tree.nodes[5].position, before.nodes[5].position);
        for (i, n) in tree.nodes.iter().enumerate().skip(1) {
            let p = n.parent.unwrap() as usize;
            assert_eq!(n.parent, before.nodes[i].parent);
            assert!(
                (n.position.distance(tree.nodes[p].position)
                    - before.nodes[i].position.distance(before.nodes[p].position))
                .abs()
                    < 1e-12
            );
        }
    }
}
