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

/// `structure` with every phytomer `pick` names (by its axis's phytomer
/// count and its place) given `stretch`; with
/// `relaid`, every node and base moved to that share of its place from the
/// root, as a lay at that stretch puts a tree laid straight.
fn stretched(
    s: &Structure,
    stretch: f32,
    pick: &dyn Fn(usize, usize) -> bool,
    relaid: bool,
) -> Structure {
    let mut s = s.clone();
    let k = f64::from(stretch);
    for axis in &mut s.axes {
        if relaid {
            axis.base = axis.base * k;
        }
        let n = axis.phytomers.len();
        for (j, p) in axis.phytomers.iter_mut().enumerate() {
            if pick(n, j) {
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

/// The stations' summed laid span over their count: how far apart they
/// stand on the wood as laid, on average.
fn spacing(s: &PreparedStations) -> f64 {
    let laid: f64 = s
        .segments
        .iter()
        .map(|g| (g.endpoints[1] - g.endpoints[0]).length())
        .sum();
    laid / f64::from(s.count)
}

/// A tree laid at half its length draws its stations at half the spacing
/// and as many of them, but for the spans a halved lay brings inside the
/// wood of the node they leave, which merge into the next node and may
/// carry their wood onto wood of another kind (measured 1,794 against
/// 1,784 on the oak at 12, seed 7).
#[test]
fn a_run_laid_at_half_its_length_keeps_its_stations_at_half_the_spacing() {
    let (family, whole) = grown();
    let half = stretched(&whole, 0.5, &|_, _| true, true);
    let (a, b) = (stations(&family, &whole), stations(&family, &half));
    assert!(a.count > 1000, "{} stations", a.count);
    let moved = f64::from(a.count.abs_diff(b.count)) / f64::from(a.count);
    assert!(moved < 0.01, "{} against {}", a.count, b.count);
    let ratio = spacing(&b) / spacing(&a);
    assert!(
        (ratio - 0.5).abs() < 0.01,
        "spacing at {ratio} of the whole"
    );
}

/// A tip past its limit lays its last units at no length: every axis's
/// last two phytomers past its first laid at none merge into the axis's
/// last drawn node, which bears their leaf-bearing wood, so the tree keeps
/// every metre of it. (How many stations the drawing makes of it depends
/// on which nodes bear, a node's kind, and is the host's question: 1,794
/// laid whole against 4,389 here, R1-READING.md.)
#[test]
fn a_span_laid_at_no_length_hands_its_leaves_to_the_node_that_draws_it() {
    let (family, whole) = grown();
    let tips = stretched(&whole, 0.0, &|n, j| j > 0 && j + 2 >= n, false);
    let species = family.space.species.as_ref().unwrap();
    let rows = pa_rows(species);
    let [a, b] = [&whole, &tips].map(|s| convert(s, (&family.space.trunk, &rows)));
    assert!(b.nodes.len() < a.nodes.len(), "no span merged");
    let sum = |t: &crate::tree::Tree| t.bearing.iter().sum::<f64>();
    assert!((sum(&a) - sum(&b)).abs() <= 1e-9 * sum(&a));
}
