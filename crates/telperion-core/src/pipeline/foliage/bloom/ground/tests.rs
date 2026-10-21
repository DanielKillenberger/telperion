//! An inflorescence seated low rests on the ground as one (fn-301 R2,
//! host ruling D); one seated high keeps every bit.
use super::super::*;
use crate::pipeline::foliage::transform_point;
use crate::tree::{Node, NodeIdentity};

/// What "on the ground" admits after a float32 store (`ground_reach`).
const BELOW: f64 = 1e-4;

/// A hanging raceme on a shoot tip seated `height` up.
fn seated(height: f64) -> (Tree, BloomParams) {
    let mut tip = Node::root();
    tip.position = Vec3::new(0.3, height, 0.0);
    tip.parent = Some(0);
    tip.identity = NodeIdentity::lineage(child(7, 1));
    let tree = Tree {
        nodes: vec![Node::root(), tip],
        blooms: vec![Bloom {
            node: 1,
            share: 1.0,
        }],
        ..Tree::default()
    };
    let p = BloomParams {
        florets: 6.0,
        rachis: 0.2,
        droop: 1.0,
        spread: 70.0,
        ..BloomParams::default()
    };
    (tree, p)
}

fn within() -> Reference {
    Reference::spanning(Vec3::new(-2.0, -2.0, -2.0), Vec3::new(2.0, 4.0, 2.0))
}

#[test]
fn a_raceme_seated_low_rests_its_florets_and_pedicels_as_one() {
    let (tree, p) = seated(0.12);
    let e = element(&p).unwrap();
    let drawn = matrices(&tree, &p, None, None).unwrap();
    let rested = matrices(&tree, &p, None, Some(within().step().y)).unwrap();
    let low = |m: &[f32; 16]| {
        (e.positions.iter())
            .map(|v| transform_point(m, *v).y)
            .fold(f64::INFINITY, f64::min)
    };
    let florets = |all: &[Matrix]| {
        all.iter()
            .filter(|(_, r)| !r)
            .map(|(m, _)| *m)
            .collect::<Vec<_>>()
    };
    assert!(
        florets(&drawn).iter().any(|m| low(m) < 0.0),
        "the raceme reaches the ground"
    );
    for m in florets(&rested) {
        assert!(low(&m) >= -BELOW, "a floret stands {} below", low(&m));
    }
    // One turn about the seat: each floret keeps its distance from it and
    // its size, and every pair keeps its distance.
    let seat = tree.nodes[1].position;
    let at = |m: &[f32; 16]| Vec3::new(m[12].into(), m[13].into(), m[14].into());
    let (a, b) = (florets(&drawn), florets(&rested));
    for i in 0..a.len() {
        assert!((at(&a[i]).distance(seat) - at(&b[i]).distance(seat)).abs() < 1e-6);
        for j in 0..a.len() {
            assert!((at(&a[i]).distance(at(&a[j])) - at(&b[i]).distance(at(&b[j]))).abs() < 1e-6);
        }
    }
    // The rachis the renderer rebuilds is not turned.
    assert_eq!(drawn.last(), rested.last());
    assert!(drawn.last().unwrap().1);
}

#[test]
fn a_raceme_seated_clear_of_the_ground_keeps_every_bit() {
    let (tree, p) = seated(3.0);
    let drawn = matrices(&tree, &p, None, None).unwrap();
    let rested = matrices(&tree, &p, None, Some(within().step().y)).unwrap();
    let bits = |m: &[Matrix]| {
        m.iter()
            .map(|(m, r)| (m.map(f32::to_bits), *r))
            .collect::<Vec<_>>()
    };
    assert_eq!(bits(&drawn), bits(&rested));
}
