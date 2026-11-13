//! fn-364, host: a stretch moves stations, never makes or removes one.
//! Each node's own wood is counted as allotted and keeps the kind it has
//! laid whole; a span merged only for its stretch hands its leaf-bearing
//! wood to the node it merges into, as its own short run at that node's
//! tip.
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
    let laid: f64 = (s.segments.iter())
        .map(|g| (g.endpoints[1] - g.endpoints[0]).length())
        .sum();
    laid / f64::from(s.count)
}

/// The three stretched trees the host named: every phytomer at half,
/// relaid; each axis's last two phytomers past its first at none; every
/// third phytomer at none.
fn variants(whole: &Structure) -> [(&'static str, Structure); 3] {
    [
        ("half", stretched(whole, 0.5, &|_, _| true, true)),
        (
            "tips",
            stretched(whole, 0.0, &|n, j| j > 0 && j + 2 >= n, false),
        ),
        ("thirds", stretched(whole, 0.0, &|_, j| j % 3 == 1, false)),
    ]
}

/// What the drawing reads of `structure`: its expansion, its leaf-bearing
/// wood as converted (own and handed, before any is resolved) and each
/// drawn node's leaf-bearing kind by its key.
fn read(
    family: &crate::family::Family,
    structure: &Structure,
) -> (
    executor::Expansion,
    f64,
    std::collections::HashMap<u32, Option<bool>>,
) {
    let species = family.space.species.as_ref().unwrap();
    let tree = convert(structure, (&family.space.trunk, &pa_rows(species)));
    let wood = tree.bearing.iter().sum::<f64>() + tree.handed.iter().map(|h| h.length).sum::<f64>();
    let x = executor::expand(tree, family).unwrap();
    let kinds = {
        let t = x.tree();
        let bears = crate::pipeline::foliage::plan::bearing_of(t, family.canopy);
        // A key two nodes share (a chain carried on keeps its lineage)
        // names neither.
        let mut kinds = std::collections::HashMap::new();
        for i in 1..t.nodes.len() {
            kinds
                .entry(t.keys[i])
                .and_modify(|k| *k = None)
                .or_insert(Some(bears(i)));
        }
        kinds
    };
    (x, wood, kinds)
}

/// fn-364, host: a stretch moves stations and never makes or removes wood
/// that bears them. In each stretched tree the leaf-bearing wood, own and
/// handed, is the tree laid whole's to the millimetre, and every drawn
/// node bears as its wood does laid whole. The station count moves only
/// by the station rule's rounding on the runs a stretch splits into own
/// and handed wood: each handed run's wood rounds up to whole twig
/// internodes once more, so at most one internode's stations more for
/// each handed run, none fewer.
#[test]
fn a_stretch_moves_stations_and_keeps_their_wood_and_kind() {
    let (family, whole) = grown();
    let (x, wood, kinds) = read(&family, &whole);
    let (runs, twig) = x.runs().unwrap().unwrap();
    assert!(runs
        .iter()
        .all(|r| !crate::pipeline::foliage::plan::is_handed(&r.nodes)));
    let count = x.stations().unwrap().unwrap().count;
    let per = twig.stations_per_internode;
    let mut held = Vec::new();
    for (name, s) in variants(&whole) {
        let (y, w, k) = read(&family, &s);
        let differ = k
            .iter()
            .filter(|(key, b)| {
                b.is_some() && kinds.get(key).is_some_and(|a| a.is_some() && a != *b)
            })
            .count();
        let (runs, _) = y.runs().unwrap().unwrap();
        let handed = runs
            .iter()
            .filter(|r| crate::pipeline::foliage::plan::is_handed(&r.nodes))
            .count();
        let c = y.stations().unwrap().unwrap().count;
        println!("{name}: {c} stations against {count}, {handed} handed runs, wood {w} against {wood}, {differ} kinds moved");
        held.push((name, c, handed, w, differ));
    }
    for (name, c, handed, w, differ) in held {
        assert!((w - wood).abs() <= 1e-3, "{name}: wood {w} against {wood}");
        assert_eq!(differ, 0, "{name}: nodes whose kind moved");
        assert!(
            c >= count && c - count <= per * handed as u32,
            "{name}: {c} against {count}"
        );
        assert!(
            f64::from(c - count) <= 0.02 * f64::from(count),
            "{name}: {c} against {count}"
        );
    }
}

/// At half its length everywhere a tree's stations stand half as far
/// apart on the wood as laid.
#[test]
fn a_run_laid_at_half_its_length_keeps_its_stations_at_half_the_spacing() {
    let (family, whole) = grown();
    let half = stretched(&whole, 0.5, &|_, _| true, true);
    let (a, b) = (stations(&family, &whole), stations(&family, &half));
    let ratio = spacing(&b) / spacing(&a);
    assert!(
        (ratio - 0.5).abs() < 0.01,
        "spacing at {ratio} of the whole"
    );
}

/// Wood laid at no length hands its stations to the node it merges into:
/// they stand at that node's one point, many to a segment, each turned by
/// its own ordinal (`run_station` onward) as along the wood it was
/// allotted.
#[test]
fn stations_on_wood_laid_at_no_length_stand_at_one_point() {
    let (family, whole) = grown();
    let tips = stretched(&whole, 0.0, &|n, j| j > 0 && j + 2 >= n, false);
    let s = stations(&family, &tips);
    let at_a_point = s
        .segments
        .iter()
        .filter(|g| g.endpoints[0] == g.endpoints[1] && g.span > 0.0);
    assert!(at_a_point
        .clone()
        .any(|g| g.count >= 2 && g.cluster.is_none()));
}
