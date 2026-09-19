use std::collections::BTreeSet;
mod specimens;
use std::ops::Range;
use telperion_core::{
    envelope::Envelope,
    foliage::*,
    math::Vec3,
    presets::{Family, Preset},
    tree::{Node, NodeKind, Tree},
    Error,
};
fn twig(length: f64) -> Tree {
    let mut root = Node::root();
    root.position = Vec3::new(0., 10., 0.);
    root.radius = 1.;
    root.start_radius = 1.;
    Tree {
        nodes: vec![
            root,
            Node {
                position: Vec3::new(0., 10. + length, 0.),
                parent: Some(0),
                radius: 0.0025,
                start_radius: 0.0025,
                base_radius: 0.0025,
                branch: 1,
                kind: NodeKind::Twig,
                ..Node::root()
            },
        ],
        crossover: 1,
        ..Tree::default()
    }
}
/// A box around the hand-built twig trees below: one internode of at most a
/// quarter metre from (0, 10, 0), and the 2.5 mm of wood the stations stand
/// off it. Tight across the shoot, so a station's radial offset survives the
/// round trip to a tenth of a micron.
fn twig_box(length: f64) -> Reference {
    Reference::spanning(
        Vec3::new(-0.004, 9.996, -0.004),
        Vec3::new(0.004, 10.004 + length, 0.004),
    )
}
fn bare() -> CanopyParams {
    CanopyParams {
        outward: 0.,
        upward: 0.,
        scatter: 0.,
        size_variation: 0.,
        divergence: 90.,
        ..CanopyParams::default()
    }
}
/// The two ends of the element's trait space, as rows and nothing else: a
/// deeply lobed blade and a four-sided shaft, built by the one routine.
fn lobed_blade() -> ElementParams {
    ElementParams {
        length: 0.1,
        width: 0.07,
        connector_length: 0.012,
        cup: 0.08,
        curl: 0.02,
        widest_at: 0.55,
        base_fullness: 0.6,
        tip_sharpness: 0.6,
        lobe_count: 5,
        lobe_depth: 0.7,
        axial_segments: 20,
        ..ElementParams::default()
    }
}
fn four_sided_needle() -> ElementParams {
    ElementParams {
        length: 0.02,
        width: 0.0015,
        connector_length: 0.0007,
        cup: 0.08,
        curl: 0.02,
        widest_at: 0.2,
        base_fullness: 0.2,
        tip_sharpness: 0.2,
        section_roundness: 1.0,
        cross_segments: 4,
        ..ElementParams::default()
    }
}

/// Distance from a point to the nearest point of one triangle: the projection
/// when it lands inside, the nearest point of the three edges otherwise.
/// Written out here rather than borrowed from the builder, so a builder that
/// measures its own deviation wrongly cannot also certify it.
fn distance_to_triangle(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> f64 {
    let edges = [(a, b), (b, c), (c, a)];
    let normal = (b - a).cross(c - a);
    if normal.length_squared() > 0.0 {
        let q = p - normal * ((p - a).dot(normal) / normal.length_squared());
        if edges
            .iter()
            .all(|(u, v)| (*v - *u).cross(q - *u).dot(normal) >= 0.0)
        {
            return p.distance(q);
        }
    }
    edges
        .iter()
        .map(|(u, v)| {
            let edge = *v - *u;
            let along = if edge.length_squared() > 0.0 {
                ((p - *u).dot(edge) / edge.length_squared()).clamp(0.0, 1.0)
            } else {
                0.0
            };
            p.distance(*u + edge * along)
        })
        .fold(f64::INFINITY, f64::min)
}

/// The transverse sections a level is chosen from: every element built by the
/// one routine publishes its own, whatever its traits say.
fn sections(e: &Element) -> Vec<Range<usize>> {
    e.anatomy
        .as_ref()
        .expect("a built element publishes its sections")
        .sections
        .clone()
}

fn triangles(e: &Element, level: &Level) -> Vec<[u32; 3]> {
    e.level_indices[level.indices.start as usize..level.indices.end as usize]
        .as_chunks::<3>()
        .0
        .to_vec()
}

/// FNV-1a over the element's positions and its whole index list: what the
/// renderer receives, and nothing about how it was asked for.
fn element_hash(p: ElementParams) -> u64 {
    let e = build_element(p).unwrap_or_else(|err| panic!("{p:?}: {err}"));
    let mut hash = 14695981039346656037_u64;
    for byte in e
        .positions
        .iter()
        .flat_map(|v| [v.x, v.y, v.z])
        .flat_map(f64::to_le_bytes)
        .chain(e.indices.iter().flat_map(|i| i.to_le_bytes()))
    {
        hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
    }
    hash
}

/// FNV-1a over the crown as the renderer receives it: the reference box and
/// then every leaf's three words - where each leaf sits and how it leans, and
/// nothing about how it was asked for.
/// Along the blade and across it, base to tip and midrib to margin. The
/// connector has no blade of its own and takes the base's pair.
fn placement_hash(f: &Family, tree: &Tree) -> u64 {
    let twig = f.skeleton.twigs.resolved().unwrap().twig;
    let placed = place_on_surface(
        tree,
        f.skeleton.envelope,
        f.skeleton.seed,
        f.canopy,
        Some(TwigPlacement {
            internode_length: twig.internode_length,
            stations_per_internode: twig.stations_per_internode,
        }),
        &f.surface,
        Reference::of(f).unwrap(),
    )
    .unwrap_or_else(|err| panic!("{:?}: {err}", f.canopy));
    let mut hash = 14695981039346656037_u64;
    let r = placed.reference;
    for byte in [
        r.min.x, r.min.y, r.min.z, r.extent.x, r.extent.y, r.extent.z,
    ]
    .iter()
    .flat_map(|v| v.to_le_bytes())
    .chain(
        placed
            .leaves
            .iter()
            .flat_map(|leaf| leaf.iter().flat_map(|w| w.to_le_bytes())),
    ) {
        hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
    }
    hash
}

/// One step inside a trait's own range, whichever way there is room for it.
fn step(v: f64, hi: f64) -> f64 {
    if v + 0.1 <= hi {
        v + 0.1
    } else {
        v - 0.1
    }
}

#[path = "foliage/attachments.rs"]
mod attachments;
#[path = "foliage/elements.rs"]
mod elements;
#[path = "foliage/levels.rs"]
mod levels;
#[path = "foliage/stations.rs"]
mod stations;
