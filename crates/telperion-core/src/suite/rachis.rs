//! fn-283: a frond draws its rachis. Every frond whose leaflets string
//! along a rachis hangs one woody axis through its leaflet stations, so
//! every leaflet and spine base lies on drawn wood (R6), and no leaflet,
//! stream or draw moves.
use std::hash::{DefaultHasher, Hash, Hasher};
use telperion_core::{
    branching,
    foliage::{self, Rachis, RECORD_UNQUANTISED},
    math::Vec3,
    mesh::TreeMesh,
    pipeline::executor,
    presets::{Family, Preset},
    surface::Curve,
    tree::{Node, Tree},
    Error,
};

/// The age `a_seedlings_fronds_are_entire_and_become_pinnate` draws the
/// young palm at, whose fronds are split.
const YOUNG: f64 = 12.0;
/// What "on the wood" admits: a millimetre off the drawn curve (R6).
const ON: f64 = 1e-3;
/// The thinnest wood the engine draws (`space.rs`).
const FINEST: f64 = 2e-4;

/// The rachis width these tests draw the palm at: the shipped palm keeps
/// zero until the device draws its axes (fn-283 .3, conductor), which sets
/// this value (FAO 156 ch. I).
const WIDTH: f64 = 0.25;

/// The date palm with its fronds' axes drawn, on a table of its own.
fn palm(age: f64) -> Family {
    let mut f = Preset::DatePalm.parameters();
    (f.skeleton.seed, f.age) = (1, age);
    f.canopy.rachis_width = WIDTH;
    f
}

/// The palm grown and clothed, and its CPU build with every leaf's
/// transform as placement made it, before the box quantised it.
fn built(f: &Family) -> (Tree, TreeMesh) {
    let x = executor::grow(f).unwrap().expansion().unwrap();
    RECORD_UNQUANTISED.with(|r| r.set(true));
    let mesh = x.mesh();
    RECORD_UNQUANTISED.with(|r| r.set(false));
    let mesh = mesh.unwrap();
    let leaves = &mesh.foliage.instances;
    assert_eq!(
        leaves.unquantised.len(),
        leaves.len(),
        "every leaf recorded"
    );
    (x.tree().clone(), mesh)
}

/// Each frond's axis as the clothing reads it.
fn axes(tree: &Tree, f: &Family) -> Vec<Rachis> {
    let blade = foliage::blade(&executor::element(f.element).unwrap());
    foliage::rachises(tree, &f.canopy, blade)
}

/// Where every leaf's base stands, as placement made it.
fn bases(mesh: &TreeMesh) -> Vec<Vec3> {
    let leaves = &mesh.foliage.instances.unquantised;
    let at = |m: &[f32; 16]| Vec3::new(m[12].into(), m[13].into(), m[14].into());
    leaves.iter().map(at).collect()
}

/// How far `p` stands off the drawn wood: each ring's sphere, and between
/// the rings of one run its tapered capsule; zero inside, and infinite
/// past a millimetre, where no cluster is searched.
fn off_wood(curve: &Curve, p: Vec3) -> f64 {
    let mut least = f64::INFINITY;
    for c in &curve.clusters {
        if p.distance(c.centre) - c.reach > least.min(ON) {
            continue;
        }
        let points = &curve.points[c.first as usize..(c.first + c.count) as usize];
        for pair in points.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            let run = b.centre - a.centre;
            let t = match run.length_squared() {
                l if l > 0. => ((p - a.centre).dot(run) / l).clamp(0., 1.),
                _ => 0.,
            };
            let r = a.radius + (b.radius - a.radius) * t;
            least = least.min(p.distance(a.centre + run * t) - r);
        }
        for q in points {
            least = least.min(p.distance(q.centre) - q.radius);
        }
    }
    least.max(0.)
}

/// Every leaf's transform before quantisation and whether it is withered,
/// in order, folded by its bits.
fn digest(mesh: &TreeMesh) -> u64 {
    let leaves = &mesh.foliage.instances;
    let mut h = DefaultHasher::new();
    for (m, &leaf) in leaves.unquantised.iter().zip(&leaves.leaves) {
        m.map(f32::to_bits).hash(&mut h);
        foliage::withered(leaf).hash(&mut h);
    }
    h.finish()
}

/// R1: the axes move no leaf. The palm at 40 keeps its leaves' count,
/// order, draws and withering, each transform to the bit before the box
/// quantises it, as the base built them (recorded before the axes, at
/// 99111868b); the wider box moves the stored words by its quantisation
/// only.
#[test]
fn the_axes_move_no_leaf() {
    let _turn = crate::one_tree::turn();
    let (_, mesh) = built(&palm(40.0));
    assert_eq!(mesh.foliage.instances.len(), 6554);
    assert_eq!(digest(&mesh), 0xbcb8_7f61_4b2d_df86);
}

/// R2, R6: on the palm at 40 and on the young palm, every leaflet and
/// spine base lies on the drawn wood, every axis runs through leaflet
/// bases only and so ends at its frond's last station, and a frond of one
/// leaflet (the young palm's entire fronds) hangs no axis: each leaf is
/// a station of an axis or a lone blade.
#[test]
fn every_leaflet_base_lies_on_its_fronds_axis() {
    let _turn = crate::one_tree::turn();
    for age in [40.0, YOUNG] {
        let f = palm(age);
        let (tree, mesh) = built(&f);
        let (axes, bases) = (axes(&tree, &f), bases(&mesh));
        let worst = bases
            .iter()
            .map(|&b| off_wood(&mesh.curve, b))
            .fold(0., f64::max);
        println!(
            "age {age}: {} axes, worst base {worst:.2e} m off",
            axes.len()
        );
        assert!(worst < ON, "a base stands {worst} m off the wood at {age}");
        let near = |p: Vec3| bases.iter().any(|b| b.distance(p) < 1e-5);
        for axis in &axes {
            assert!(
                axis.points[1..].iter().all(|&p| near(p)),
                "an axis runs past its leaflets at {age}"
            );
        }
        let stations: usize = axes.iter().map(|a| a.points.len() - 1).sum();
        assert!(!axes.is_empty() && stations <= bases.len());
        if age == YOUNG {
            assert!(stations < bases.len(), "the young palm's entire fronds");
        }
    }
}

/// R3: zero width hangs no axis, nor does a frond of one leaflet or of no
/// rachis length; an axis is never thinner than FINEST and tapers along
/// its stations from half the width scaled by its frond's size.
#[test]
fn an_axis_is_dormant_without_width_or_leaflets_and_tapers() {
    let _turn = crate::one_tree::turn();
    let f = palm(YOUNG);
    let tree = executor::grow(&f)
        .unwrap()
        .expansion()
        .unwrap()
        .tree()
        .clone();
    let drawn = axes(&tree, &f);
    for edit in [
        |f: &mut Family| f.canopy.rachis_width = 0.,
        |f: &mut Family| f.canopy.leaflet_count = 1.,
        |f: &mut Family| f.canopy.rachis_length = 0.,
    ] {
        let mut off = f.clone();
        edit(&mut off);
        assert!(axes(&tree, &off).is_empty());
    }
    let mut bare = f.clone();
    bare.canopy.rachis_width = 0.;
    let bare = executor::grow(&bare).unwrap().expansion().unwrap();
    let hung: usize = drawn.iter().map(|a| a.points.len()).sum();
    assert_eq!(bare.tree().nodes.len() + hung, tree.nodes.len());
    for axis in &drawn {
        let r = &axis.radii;
        assert!(r[0] <= WIDTH / 2.);
        assert!(r.iter().all(|&r| r >= FINEST));
        assert!(r.windows(2).all(|w| w[1] <= w[0]), "an axis thickens");
    }
}

/// R1: clothing a clothed palm leaves it unchanged, its bases and axes
/// hung once.
#[test]
fn a_clothed_palm_is_not_clothed_again() {
    let _turn = crate::one_tree::turn();
    let f = palm(40.0);
    let tree = executor::grow(&f)
        .unwrap()
        .expansion()
        .unwrap()
        .tree()
        .clone();
    assert!(!axes(&tree, &f).is_empty());
    let again = executor::expand(tree.clone(), &f).unwrap();
    assert!(again.tree().nodes == tree.nodes && again.tree().sections == tree.sections);
}

/// R1, R3: the width is refused by its own name off its rail, and the
/// axes' nodes past what a node index holds are refused by the rail's.
#[test]
fn the_width_and_the_axes_rail_are_refused_by_name() {
    for width in [11.0, -0.1, f64::NAN] {
        let mut f = palm(YOUNG);
        f.canopy.rachis_width = width;
        let refused = executor::grow(&f).and_then(|g| g.expansion()).err();
        assert!(
            format!("{refused:?}").contains("rachis width"),
            "{refused:?}"
        );
    }
    let most = u32::MAX as usize;
    assert_eq!(branching::room(most - 2, 2, "rachis nodes"), Ok(most));
    assert_eq!(
        branching::room(most - 2, 3, "rachis nodes"),
        Err(Error::ResourceLimit("rachis nodes"))
    );
}

/// R1: a handed-in stem of one node above the root hangs each axis on
/// that node, as there is no depth along it to search.
#[test]
fn a_stem_of_one_node_bears_its_axes() {
    let f = palm(40.0);
    let node = |position, parent, branch, stem| Node {
        position,
        parent,
        radius: 0.3,
        start_radius: 0.3,
        base_radius: 0.3,
        branch,
        stem,
        ..Node::root()
    };
    let mut tree = Tree::default();
    tree.nodes.push(node(Vec3::ZERO, None, 0, false));
    tree.nodes.push(node(Vec3::Y, Some(0), 1, true));
    let x = executor::expand(tree, &f).unwrap();
    let axes = axes(x.tree(), &f);
    assert!(!axes.is_empty() && axes.iter().all(|a| a.at == 1));
}

/// R1: no leaf is borne on the clothing. A palm that clothes its slender
/// wood (cauline above zero) bears no leaf on its frond axes nor on its
/// retained bases: drawing the axes moves no leaf at either slender bound.
#[test]
fn no_leaf_is_borne_on_the_clothing() {
    let _turn = crate::one_tree::turn();
    for shoot in [0.1, 1.0] {
        let count = |width: f64| {
            let mut f = palm(40.0);
            (f.canopy.cauline, f.canopy.shoot_radius) = (1.0, shoot);
            f.canopy.rachis_width = width;
            let x = executor::grow(&f).unwrap().expansion().unwrap();
            x.mesh().unwrap().foliage.instances.len()
        };
        let (bare, drawn) = (count(0.), count(WIDTH));
        println!("shoot radius {shoot}: {bare} leaves at width 0, {drawn} at 0.25");
        assert_eq!(drawn, bare, "the axes bear leaves at shoot radius {shoot}");
    }
}
