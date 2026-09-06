//! Connected descendant redistribution after retention; no new nodes or buds.
use super::*;

pub(super) fn upper_descendants(tree: &mut Tree, envelope: Envelope) {
    let first = tree.crossover;
    let upper = tree.nodes[..first]
        .iter()
        .map(|n| n.position.y)
        .fold(0.0, f64::max)
        * 0.75;
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
    // Empty endpoint voxels bracketed by crown mass on two independent axes
    // are inter-mass candidates, not a turn toward the crown's mean position.
    // World-space sampling makes the decision independent of a showcase camera.
    use std::collections::BTreeSet;
    let cell = |p: Vec3| {
        [
            (p.x / 0.5).floor() as i32,
            (p.y / 0.5).floor() as i32,
            (p.z / 0.5).floor() as i32,
        ]
    };
    let occupied: BTreeSet<_> = tree
        .nodes
        .iter()
        .filter(|n| n.kind == NodeKind::Twig)
        .map(|n| cell(n.position))
        .collect();
    let mut empty = BTreeSet::new();
    for &c in &occupied {
        for axis in 0..3 {
            for step in 1..=6 {
                let mut q = c;
                q[axis] += step;
                if occupied.contains(&q) {
                    continue;
                }
                let bracketed = (0..3)
                    .filter(|&a| {
                        [-1, 1].iter().all(|&sign| {
                            (1..=6).any(|r| {
                                let mut v = q;
                                v[a] += sign * r;
                                occupied.contains(&v)
                            })
                        })
                    })
                    .count();
                if bracketed >= 2 {
                    empty.insert(q);
                }
            }
        }
    }
    let mut groups = vec![Vec::new(); tree.nodes.len()];
    for i in first..tree.nodes.len() {
        if owner[i] != usize::MAX {
            groups[owner[i]].push(i);
        }
    }
    let mut rotations = vec![None; tree.nodes.len()];
    for i in first..tree.nodes.len() {
        if owner[i] != i || counts[i] == 0 {
            continue;
        }
        let base = tree.nodes[tree.nodes[i].parent.unwrap() as usize].position;
        let from = (centroids[i] / counts[i] as f64 - base).normalized();
        let reach = groups[i]
            .iter()
            .map(|&j| tree.nodes[j].position.distance(base))
            .fold(0.0, f64::max);
        let mut best = 0;
        for &q in &empty {
            let target = Vec3::new(q[0] as f64 + 0.5, q[1] as f64 + 0.5, q[2] as f64 + 0.5) * 0.5;
            // Feasibility first: an unchanged connected group cannot reach
            // beyond its longest displacement from the retained insertion.
            if target.y < upper || target.distance(base) > reach {
                continue;
            }
            let toward = (target - base).normalized();
            let axis = from.cross(toward);
            if axis.length_squared() < 1e-12 {
                continue;
            }
            let axis = axis.normalized();
            let angle = from.dot(toward).clamp(-1.0, 1.0).acos();
            let hits: BTreeSet<_> = groups[i]
                .iter()
                .filter(|&&j| tree.nodes[j].kind == NodeKind::Twig)
                .map(|&j| cell(base + (tree.nodes[j].position - base).rotate(axis, angle)))
                .filter(|c| empty.contains(c))
                .collect();
            if hits.len() <= best {
                continue;
            }
            let fits = groups[i].iter().all(|&j| {
                let parent = tree.nodes[j].parent.unwrap() as usize;
                let end = base + (tree.nodes[j].position - base).rotate(axis, angle);
                let start = base + (tree.nodes[parent].position - base).rotate(axis, angle);
                end.y >= upper
                    && (0..=8).all(|k| envelope.contains(start.lerp(end, k as f64 / 8.0), 0.0))
            });
            if fits {
                best = hits.len();
                rotations[i] = Some((base, axis, angle));
            }
        }
        if let Some((base, axis, angle)) = rotations[i] {
            for &j in &groups[i] {
                empty.remove(&cell(
                    base + (tree.nodes[j].position - base).rotate(axis, angle),
                ));
            }
        }
    }
    // A rigid group must fit as a whole. Reject a proposed rotation rather
    // than clipping endpoints or moving its socket outside the growth envelope.
    for i in first..tree.nodes.len() {
        if owner[i] == usize::MAX {
            continue;
        }
        if let Some((base, axis, angle)) = rotations[owner[i]] {
            let parent = tree.nodes[i].parent.unwrap() as usize;
            let end = base + (tree.nodes[i].position - base).rotate(axis, angle);
            let start = base + (tree.nodes[parent].position - base).rotate(axis, angle);
            if (0..=8).any(|k| !envelope.contains(start.lerp(end, k as f64 / 8.0), 0.0)) {
                rotations[owner[i]] = None;
            }
        }
    }
    for i in first..tree.nodes.len() {
        if owner[i] != usize::MAX {
            if let Some((base, axis, angle)) = rotations[owner[i]] {
                tree.nodes[i].position = base + (tree.nodes[i].position - base).rotate(axis, angle);
            }
        }
    }
}

pub(super) fn transverse_curtains(tree: &mut Tree, envelope: Envelope) {
    let first = tree.crossover;
    let original: Vec<_> = tree.nodes.iter().map(|n| n.position).collect();
    let mut roots = vec![0; tree.nodes.len()];
    let mut bases = vec![Vec3::ZERO; tree.nodes.len()];
    let mut valid = vec![true; tree.nodes.len()];
    for i in first..tree.nodes.len() {
        let parent = tree.nodes[i].parent.unwrap() as usize;
        roots[i] = roots[parent];
        if parent < first {
            if let Some(grand) = tree.nodes[parent].parent {
                let heading = (original[parent] - original[grand as usize]).normalized();
                if heading.y < -0.5 {
                    roots[i] = i;
                    bases[i] = original[parent];
                }
            }
        }
        if roots[i] != 0 {
            let base = bases[roots[i]];
            let delta = original[i] - base;
            tree.nodes[i].position = base + Vec3::new(delta.x * 0.2, delta.y, delta.z * 0.2);
        }
    }
    for i in first..tree.nodes.len() {
        if roots[i] == 0 {
            continue;
        }
        let parent = tree.nodes[i].parent.unwrap() as usize;
        if (0..=8).any(|k| {
            !envelope.contains(
                tree.nodes[parent]
                    .position
                    .lerp(tree.nodes[i].position, k as f64 / 8.0),
                0.0,
            )
        }) {
            valid[roots[i]] = false;
        }
    }
    for i in first..tree.nodes.len() {
        if !valid[roots[i]] {
            tree.nodes[i].position = original[i];
        }
    }
}

/// Reattach branched local fans along their existing uninterrupted descending
/// support. Translating the whole fan retains its lengths and needle stations;
/// this clothes the supporting axis instead of extending every fan past its tip.
pub(super) fn longitudinal_curtains(tree: &mut Tree, envelope: Envelope) {
    let first = tree.crossover;
    let mut structural_children = vec![0; first];
    for n in &tree.nodes[1..first] {
        structural_children[n.parent.unwrap() as usize] += 1;
    }
    let mut owner = vec![0; tree.nodes.len()];
    let mut groups = vec![Vec::new(); tree.nodes.len()];
    for i in first..tree.nodes.len() {
        let p = tree.nodes[i].parent.unwrap() as usize;
        owner[i] = if p < first { i } else { owner[p] };
        groups[owner[i]].push(i);
    }
    for (root, group) in groups.iter().enumerate().skip(first) {
        // A single terminal is an anatomical target, not a redistributable fan.
        if group.len() < 3 {
            continue;
        }
        let old = tree.nodes[root].parent.unwrap() as usize;
        let base = tree.nodes[old].position;
        let reach = group
            .iter()
            .map(|&i| tree.nodes[i].position.distance(base))
            .fold(0.0, f64::max);
        let mut at = old;
        while let Some(p) = tree.nodes[at].parent.map(|p| p as usize) {
            let Some(g) = tree.nodes[p].parent.map(|p| p as usize) else {
                break;
            };
            if structural_children[p] != 1
                || (tree.nodes[at].position - tree.nodes[p].position)
                    .normalized()
                    .y
                    >= -0.5
                || (tree.nodes[p].position - tree.nodes[g].position)
                    .normalized()
                    .y
                    >= -0.5
                || base.distance(tree.nodes[p].position) > reach * 0.5
            {
                break;
            }
            at = p;
        }
        if at == old {
            continue;
        }
        let shift = tree.nodes[at].position - base;
        if !group.iter().all(|&i| {
            let parent = tree.nodes[i].parent.unwrap() as usize;
            (0..=8).all(|k| {
                envelope.contains(
                    tree.nodes[parent]
                        .position
                        .lerp(tree.nodes[i].position, k as f64 / 8.0)
                        + shift,
                    0.0,
                )
            })
        }) {
            continue;
        }
        tree.nodes[root].parent = Some(at as u32);
        for &i in group {
            tree.nodes[i].position += shift;
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
    fn longitudinal_fans_keep_structural_forks_and_single_terminals() {
        let mut tree = fixture();
        tree.nodes[2].position = Vec3::new(0., 9., 0.);
        tree.nodes[3].position = Vec3::new(0., 8.9, 0.);
        tree.nodes[5].position = Vec3::new(0.05, 8.5, 0.);
        for x in [-0.1, 0.1] {
            let mut n = tree.nodes[5].clone();
            n.position = Vec3::new(x, 8.2, 0.);
            n.parent = Some(5);
            n.branch = tree.nodes.len() as u32;
            tree.nodes.push(n);
        }
        let before = tree.clone();
        longitudinal_curtains(
            &mut tree,
            Envelope {
                height: 24.,
                spread: 1.,
                crown_base: 0.,
                ..Envelope::default()
            },
        );
        tree.validate().unwrap();
        assert_eq!(&tree.nodes[..5], &before.nodes[..5]);
        assert_eq!(tree.nodes[5].parent, Some(2));
        for i in 5..tree.nodes.len() {
            let a = &tree.nodes[i];
            let b = &before.nodes[i];
            assert!(
                (a.position
                    .distance(tree.nodes[a.parent.unwrap() as usize].position)
                    - b.position
                        .distance(before.nodes[b.parent.unwrap() as usize].position))
                .abs()
                    < 1e-12
            );
        }
    }
    #[test]
    fn curtain_keeps_primary_tip_and_socket_topology() {
        let mut tree = fixture();
        let before = tree.clone();
        transverse_curtains(
            &mut tree,
            Envelope {
                height: 24.,
                spread: 1.,
                crown_base: 0.,
                ..Envelope::default()
            },
        );
        tree.validate().unwrap();
        assert_eq!(tree.nodes.len(), before.nodes.len());
        for (i, (a, b)) in tree.nodes.iter().zip(&before.nodes).enumerate() {
            assert_eq!(a.parent, b.parent);
            assert_eq!(a.branch, b.branch);
            assert_eq!(a.kind, b.kind);
            assert_eq!(a.position.y, b.position.y);
            if [0, 1, 2, 3, 4].contains(&i) {
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
        upper_descendants(
            &mut tree,
            Envelope {
                height: 24.,
                spread: 1.,
                crown_base: 0.,
                ..Envelope::default()
            },
        );
        tree.validate().unwrap();
        // No bracketed empty reach exists in this sparse fixture.
        assert_eq!(tree.nodes[4].position, before.nodes[4].position);
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
