//! The person beside every shipped tree (fn-310 D7, D6): at or behind the
//! trunk's depth, out of the crown's shadow, whole in the hero frame, head
//! to feet, wherever a whole fit exists, and under no wood or foliage. The table names the presets, so it lives
//! here and not in the renderer library, which never names a family.
#[allow(dead_code)]
mod common;
use telperion_core::{
    math::Vec3,
    mesh::{self, TreeMesh},
    presets,
    surface::Bounds,
    Family,
};
use telperion_render::{figure_placement, figure_points, hero_pose, Shot, GROUND_REACH};

/// The harness's default sun and the film's, azimuth and elevation.
const SUNS: [(f64, f64); 2] = [(135.0, 30.0), (115.0, 60.0)];

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
/// catalogue age and seed and at age 1, Telperion at 80, and the owner's two
/// stills at seed 7 (oak at 40, Telperion at 80), as the generator bounds
/// them. The
/// ordinary preset has no engine species and draws no tree (fn-198).
/// Measured on 2026-10-23 (age 1 to three places) and held to the generator by
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
    (
        "oregon-white-oak",
        Some(1.0),
        None,
        [-0.042, -0.002, -0.042, 0.042, 0.385, 0.042],
    ),
    (
        "norway-spruce",
        Some(1.0),
        None,
        [-0.010, -0.000, -0.010, 0.010, 0.090, 0.010],
    ),
    (
        "european-beech",
        Some(1.0),
        None,
        [-0.025, -0.002, -0.025, 0.025, 0.515, 0.025],
    ),
    (
        "silver-birch",
        Some(1.0),
        None,
        [-0.025, -0.003, -0.025, 0.025, 0.819, 0.025],
    ),
    (
        "date-palm",
        Some(1.0),
        None,
        [-0.102, -0.056, -0.397, 0.296, 0.805, 0.181],
    ),
    (
        "telperion",
        Some(1.0),
        None,
        [-0.075, -0.005, -0.075, 0.075, 1.338, 0.081],
    ),
    (
        "laurelin",
        Some(1.0),
        None,
        [-0.085, -0.005, -0.085, 0.085, 1.338, 0.085],
    ),
];

/// The trees beside which no whole fit exists on the camera's side: every
/// seedling, 0.09 to 1.34 m against the 1.75 m person, keeps the crown's
/// distance and shows cut by the frame (host ruling, 2026-10-23). A preset,
/// an age and a seed, as in `TREES`.
const EXEMPT: &[(&str, Option<f64>, Option<u32>)] = &[
    ("oregon-white-oak", Some(1.0), None),
    ("norway-spruce", Some(1.0), None),
    ("european-beech", Some(1.0), None),
    ("silver-birch", Some(1.0), None),
    ("date-palm", Some(1.0), None),
    ("telperion", Some(1.0), None),
    ("laurelin", Some(1.0), None),
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

/// Being seen in shot is why the person stands where it does (fn-310 D7,
/// D6): at the hero pose of every tree in the table the whole person, head
/// to feet, projects inside the frame and the depth range, except beside
/// the trees `EXEMPT` names, where no distance out along its side holds
/// it whole; a new exemption fails here by name.
#[test]
fn the_whole_person_stands_inside_the_hero_frame_beside_every_shipped_tree() {
    let aspect = 16.0 / 9.0;
    for &(id, age, seed, row) in TREES {
        let bounds = tree_bounds(row);
        let view_projection = hero_pose(bounds, aspect, GROUND_REACH).view_projection(aspect);
        let (_, whole, body) = figure_points(bounds, SUNS[0].0);
        let exempt = EXEMPT.contains(&(id, age, seed));
        assert_eq!(
            whole, !exempt,
            "{id} at {age:?}, seed {seed:?}: whole in frame is {whole}"
        );
        let inside = body.iter().all(|&point| {
            let (x, y, depth) = project(&view_projection, point);
            x.abs() <= 1.0 && y.abs() <= 1.0 && (0.0..=1.0).contains(&depth)
        });
        assert_eq!(
            inside, whole,
            "{id} at {age:?}, seed {seed:?}: the person is whole in frame: {inside}"
        );
    }
}

/// Beside every tree in the table, under the harness's sun and the film's
/// (fn-310 D7): the foot at the trunk's depth along the hero view, square
/// to it, the crown's reach and the margin out, and no point of the person
/// in the shadow the crown the bounds inscribe throws.
#[test]
fn the_person_stands_beside_the_trunk_and_out_of_the_crowns_shadow() {
    let (x, z) = Shot::default().azimuth.to_radians().sin_cos();
    for &(id, age, _, row) in TREES {
        let bounds = tree_bounds(row);
        for (sun, elevation) in SUNS {
            let (placement, _, body) = figure_points(bounds, sun);
            let foot = placement.foot;
            let depth = -(foot.x * x + foot.z * z);
            assert!(depth > -1e-9, "{id} at {age:?}: {depth} m in front");
            let reach = reach(bounds, foot.x / foot.length(), foot.z / foot.length());
            assert!(
                (foot.length() - reach - 0.5).abs() < 1e-9,
                "{id} at {age:?}"
            );
            assert_eq!(placement, figure_placement(bounds, sun).0);
            let (a, e) = (sun.to_radians(), elevation.to_radians());
            let to_sun = Vec3::new(e.cos() * a.sin(), e.sin(), e.cos() * a.cos());
            let lit = body.iter().all(|&p| !shaded(bounds, p, to_sun));
            assert!(lit, "{id} at {age:?}: in the crown's shadow under {sun}");
        }
        let [a, b] = SUNS.map(|(sun, _)| figure_placement(bounds, sun).0);
        assert_eq!(a, b, "{id} at {age:?}: the two suns stand it apart");
    }
}

/// How much coarser the person may make the sun's map: the ground a texel
/// covers with the person in the fit over without it, on either axis (host
/// ruling on the shadow cost, fn-310 task 9). Measured 1.000 in every case
/// (`SHADOW-FIT.md`).
const COARSER: f64 = 1.1;

/// The person in the sun's map costs the tree's shadow little: beside every
/// shipped preset at its catalogue age and Telperion at 80, under the
/// harness's sun and the film's, the map fitted to the tree and the person
/// covers at most `COARSER` times the ground per texel that the tree's alone
/// does. The film's frames are `film_placing`'s.
#[test]
fn the_person_coarsens_the_suns_map_by_at_most_a_tenth() {
    let rows = TREES.iter().filter(|t| matches!(t.1, None | Some(80.0)));
    for &(id, age, seed, row) in rows {
        let tree = tree_bounds(row);
        for sun in SUNS {
            let (_, _, body) = figure_points(tree, sun.0);
            let ratio = common::coarsened(sun, &[tree], common::box_of(&body));
            assert!(
                ratio <= COARSER,
                "{id} at {age:?}, seed {seed:?}, sun {sun:?}: {ratio} times coarser"
            );
        }
    }
}

/// The reach of the crown the bounds inscribe along the unit bearing
/// (`x`, `z`).
fn reach(bounds: Bounds, x: f64, z: f64) -> f64 {
    let (centre, half) = (
        (bounds.min + bounds.max) * 0.5,
        (bounds.max - bounds.min) * 0.5,
    );
    centre.x * x + centre.z * z + (half.x * x).hypot(half.z * z)
}

/// Whether the ray from `from` towards the sun passes through the crown the
/// bounds inscribe.
fn shaded(bounds: Bounds, from: Vec3, to_sun: Vec3) -> bool {
    let (centre, half) = (
        (bounds.min + bounds.max) * 0.5,
        (bounds.max - bounds.min) * 0.5,
    );
    let scale = |v: Vec3| Vec3::new(v.x / half.x, v.y / half.y, v.z / half.z);
    let (o, d) = (scale(from - centre), scale(to_sun));
    let (a, b, c) = (d.dot(d), 2.0 * o.dot(d), o.dot(o) - 1.0);
    let root = b * b - 4.0 * a * c;
    root >= 0.0 && (-b + root.sqrt()) / (2.0 * a) > 0.0
}

/// The first wood or foliage vertex of `tree` standing over the disc of
/// `radius` round `foot`, at any height. A leaf whose element's reach
/// under its own transform comes near is placed vertex by vertex, and a
/// flower's stalks count out to their reach.
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
            let (origin, scale) = (column(12), scale.fold(0.0, f64::max));
            if !near(origin, reach * scale) {
                continue;
            }
            if near(origin, element.stalk.max(0.0) * scale) {
                return Some(origin);
            }
            let placed = element
                .positions
                .iter()
                .map(|p| origin + column(0) * p.x + column(4) * p.y + column(8) * p.z);
            if let Some(p) = placed.into_iter().find(|&p| near(p, 0.0)) {
                return Some(p);
            }
        }
    }
    None
}

/// Builds every tree in the table, one at a time: the table is what the
/// generator bounds, and no wood or foliage stands over the person's
/// footprint beside it (fn-310 D2, D7, host ruling). Both of `SUNS` stand
/// the person in one place (`the_person_stands_beside_the_trunk_...`). Over a minute, most of
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
        let (placement, _, body) = figure_points(tree.bounds, SUNS[0].0);
        let foot = placement.foot;
        let radius = body
            .iter()
            .fold(0.0f64, |r, p| r.max((p.x - foot.x).hypot(p.z - foot.z)));
        if let Some(p) = over(&tree, foot, radius) {
            panic!("{id} at {age:?}, seed {seed:?}: {p:?} stands over the person at {foot:?}");
        }
    }
}
