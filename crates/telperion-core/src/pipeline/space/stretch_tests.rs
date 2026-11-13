//! fn-364, the host's answer to R1: a stretch shortens the wood and keeps
//! every node and its leaf, so a run's stations stand for its nodes. They
//! are counted from the leaf-bearing wood, the stretch left out, and placed
//! on the wood as laid; a span laid too short to draw hands its
//! leaf-bearing wood to the node that draws it.
use super::*;
use crate::pipeline::executor;
use crate::pipeline::foliage::prepared::PreparedStations;
use crate::presets::Preset;

/// The young oak, which bears its leaves under a twig layer, grown with no
/// potential height so every stretch is whole: its family and structure.
fn grown() -> (crate::family::Family, Structure) {
    let family = Preset::OregonWhiteOak.parameters();
    let mut species = family.space.species.clone().unwrap();
    for s in &mut species.states {
        s.inverse_height = 0.0;
    }
    let request = Request {
        age: 12,
        seed: 7,
        light: family.space.light,
    };
    (family, telperion_space::grow(&species, request).unwrap())
}

/// `structure` with every phytomer `pick` names given `stretch`; with
/// `relaid`, every node and base moved to that share of its place from the
/// root, as a lay at that stretch puts a tree laid straight.
fn stretched(s: &Structure, stretch: f32, pick: &dyn Fn(usize) -> bool, relaid: bool) -> Structure {
    let mut s = s.clone();
    let k = f64::from(stretch);
    for axis in &mut s.axes {
        if relaid {
            axis.base = axis.base * k;
        }
        for (j, p) in axis.phytomers.iter_mut().enumerate() {
            if pick(j) {
                p.stretch = stretch;
            }
            if relaid {
                p.tip = p.tip * k;
            }
        }
    }
    s
}

/// The stations the drawing prepares for `structure`'s tree.
fn stations(family: &crate::family::Family, structure: &Structure) -> PreparedStations {
    let species = family.space.species.as_ref().unwrap();
    let tree = convert(structure, (&family.space.trunk, &pa_rows(species)));
    let x = executor::expand(tree, family).unwrap();
    x.stations().unwrap().expect("stations under a twig layer")
}

/// A tree laid at half its length draws as many stations as at its whole
/// length, each at the share of its segment it stood at, so every station
/// stands half as far from the next: the same segments, their leaf-bearing
/// wood unchanged and their ends at half their places.
#[test]
fn a_run_laid_at_half_its_length_keeps_its_stations_at_half_the_spacing() {
    let (family, whole) = grown();
    let half = stretched(&whole, 0.5, &|_| true, true);
    let (a, b) = (stations(&family, &whole), stations(&family, &half));
    assert!(a.count > 1000, "{} stations", a.count);
    assert_eq!(a.count, b.count);
    assert_eq!(a.segments.len(), b.segments.len());
    for (s, t) in a.segments.iter().zip(&b.segments) {
        assert_eq!(
            (s.first, s.count, s.along, s.span),
            (t.first, t.count, t.along, t.span)
        );
        for (p, q) in s.endpoints.iter().zip(&t.endpoints) {
            assert!((*p * 0.5 - *q).length() <= 1e-9, "{p:?} against {q:?}");
        }
    }
}

/// Every third phytomer laid at no length merges into the next node drawn
/// on its axis, or the last one where none follows, which bears its
/// leaf-bearing wood: the tree draws as many stations as laid whole.
#[test]
fn a_span_laid_at_no_length_hands_its_leaves_to_the_node_that_draws_it() {
    let (family, whole) = grown();
    let gaps = stretched(&whole, 0.0, &|j| j % 3 == 1, false);
    let species = family.space.species.as_ref().unwrap();
    let rows = pa_rows(species);
    let [a, b] = [&whole, &gaps].map(|s| convert(s, (&family.space.trunk, &rows)));
    assert!(b.nodes.len() < a.nodes.len(), "no span merged");
    let sum = |t: &crate::tree::Tree| t.bearing.iter().sum::<f64>();
    assert!((sum(&a) - sum(&b)).abs() <= 1e-9 * sum(&a));
    assert_eq!(
        stations(&family, &whole).count,
        stations(&family, &gaps).count
    );
}
