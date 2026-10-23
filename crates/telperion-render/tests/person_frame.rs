//! The person beside every shipped tree (fn-310 D2): inside the hero frame,
//! and under no wood or foliage. The table names the presets, so it lives
//! here and not in the renderer library, which never names a family.
use telperion_core::{
    math::Vec3,
    mesh::{self, TreeMesh},
    presets,
    surface::Bounds,
    Family,
};
use telperion_render::{figure_points, hero_pose, GROUND_REACH};

/// Clip-space x, y and depth of a world point through this view-projection.
fn project(view_projection: &[f32; 16], p: Vec3) -> (f64, f64, f64) {
    let v = [p.x as f32, p.y as f32, p.z as f32, 1.0];
    let mut clip = [0.0f32; 4];
    for (row, slot) in clip.iter_mut().enumerate() {
        *slot = (0..4).map(|k| view_projection[k * 4 + row] * v[k]).sum();
    }
    let w = clip[3] as f64;
    (clip[0] as f64 / w, clip[1] as f64 / w, clip[2] as f64 / w)
}

/// The trees the person is framed beside: every shipped preset at its
/// catalogue age and seed, Telperion at 80, and the owner's two stills at
/// seed 7 (oak at 40, Telperion at 80), as the generator bounds them. The
/// ordinary preset has no engine species and draws no tree (fn-198).
/// Measured on 2026-10-23 and held to the generator by
/// `the_bounds_table_is_the_generators_and_no_canopy_covers_the_person`.
/// A preset, an age and a seed (each the catalogue's when none), and
/// the bounds' low and high corners.
type Tree = (&'static str, Option<f64>, Option<u32>, [f64; 6]);
const TREES: &[Tree] = &[
    (
        "oregon-white-oak",
        None,
        None,
        [-4.59, -0.04, -3.13, 4.01, 10.07, 3.76],
    ),
    (
        "norway-spruce",
        None,
        None,
        [-3.05, -0.05, -3.91, 3.02, 11.64, 3.95],
    ),
    (
        "european-beech",
        None,
        None,
        [-3.86, -0.04, -3.50, 4.40, 9.98, 3.44],
    ),
    (
        "silver-birch",
        None,
        None,
        [-3.36, -0.07, -5.83, 4.85, 16.50, 3.43],
    ),
    (
        "date-palm",
        None,
        None,
        [-9.13, -0.06, -9.36, 9.51, 21.68, 9.58],
    ),
    (
        "telperion",
        None,
        None,
        [-25.79, -0.18, -25.23, 26.55, 45.77, 29.27],
    ),
    (
        "laurelin",
        None,
        None,
        [-24.70, -0.18, -25.23, 25.94, 44.63, 28.12],
    ),
    (
        "telperion",
        Some(80.0),
        None,
        [-24.52, -0.18, -24.82, 25.52, 44.62, 28.12],
    ),
    (
        "oregon-white-oak",
        Some(40.0),
        Some(7),
        [-3.94, -0.04, -3.51, 3.34, 11.15, 4.01],
    ),
    (
        "telperion",
        Some(80.0),
        Some(7),
        [-25.64, -0.18, -23.66, 30.64, 46.28, 27.53],
    ),
];

fn family(id: &str, age: Option<f64>, seed: Option<u32>) -> Family {
    let mut family = presets::Preset::from_id(id).unwrap().parameters();
    family.age = age.unwrap_or(family.age);
    family.skeleton.seed = seed.unwrap_or(family.skeleton.seed);
    family
}

fn tree_bounds(row: [f64; 6]) -> Bounds {
    Bounds {
        min: Vec3::new(row[0], row[1], row[2]),
        max: Vec3::new(row[3], row[4], row[5]),
    }
}

/// Being seen in shot is why the person stands where it does (fn-310
/// D2): at the hero pose of every tree in the table, the whole person
/// projects inside the frame and the depth range.
#[test]
fn the_person_stands_inside_the_hero_frame_beside_every_shipped_tree() {
    let aspect = 16.0 / 9.0;
    for &(id, age, seed, row) in TREES {
        let bounds = tree_bounds(row);
        let view_projection = hero_pose(bounds, aspect, GROUND_REACH).view_projection(aspect);
        for point in figure_points(bounds).1 {
            let (x, y, depth) = project(&view_projection, point);
            assert!(
                x.abs() <= 1.0 && y.abs() <= 1.0 && (0.0..=1.0).contains(&depth),
                "{id} at {age:?}, seed {seed:?}: the person left the frame: {x}, {y}, {depth}"
            );
        }
    }
}

/// The first wood or foliage vertex of `tree` standing over the disc of
/// `radius` round `foot`, at any height. A leaf counts out to its
/// element's reach under its own transform.
fn over(tree: &TreeMesh, foot: Vec3, radius: f64) -> Option<Vec3> {
    let near = |p: Vec3, reach: f64| (p.x - foot.x).hypot(p.z - foot.z) < radius + reach;
    let wood = tree
        .wood
        .chunks
        .iter()
        .flat_map(|c| c.positions.as_chunks::<3>().0);
    let wood = wood.map(|p| Vec3::new(p[0].into(), p[1].into(), p[2].into()));
    if let Some(p) = wood.clone().find(|&p| near(p, 0.0)) {
        return Some(p);
    }
    assert!(wood.count() > 0, "a tree with no wood proves nothing");
    for foliage in std::iter::once(&tree.foliage).chain(&tree.organ) {
        let element = &foliage.element;
        let size = element
            .positions
            .iter()
            .fold(0.0f64, |r, p| r.max(p.length()));
        let reach = size + element.stalk.max(0.0);
        for m in foliage.instances.matrices() {
            let column = |k: usize| Vec3::new(m[k].into(), m[k + 1].into(), m[k + 2].into());
            let scale = [0, 4, 8].map(|k| column(k).length()).into_iter();
            let origin = column(12);
            if near(origin, reach * scale.fold(0.0, f64::max)) {
                return Some(origin);
            }
        }
    }
    None
}

/// Builds every tree in the table, one at a time: the table is what the
/// generator bounds, and no wood or foliage stands over the person's
/// footprint beside it (fn-310 D2, host ruling). Over a minute, most of
/// it Telperion, so it runs by name (`--ignored`) when a crown changes;
/// the fast tests above read the table.
#[test]
#[ignore]
fn the_bounds_table_is_the_generators_and_no_canopy_covers_the_person() {
    for &(id, age, seed, row) in TREES {
        let tree = mesh::build(&family(id, age, seed)).unwrap();
        let (low, high) = (tree.bounds.min, tree.bounds.max);
        let built = [low.x, low.y, low.z, high.x, high.y, high.z];
        assert!(
            built.iter().zip(row).all(|(b, r)| (b - r).abs() <= 0.006),
            "{id} at {age:?}, seed {seed:?}: the generator bounds it at {built:?}"
        );
        let (foot, body) = figure_points(tree.bounds);
        let radius = body
            .iter()
            .fold(0.0f64, |r, p| r.max((p.x - foot.x).hypot(p.z - foot.z)));
        if let Some(p) = over(&tree, foot, radius) {
            panic!("{id} at {age:?}, seed {seed:?}: {p:?} stands over the person at {foot:?}");
        }
    }
}
