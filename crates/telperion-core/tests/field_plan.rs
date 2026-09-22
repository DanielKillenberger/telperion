//! The field answered from the leaf plan: conservative against the leaves the
//! CPU placement retains, bounded in its over-coverage against the placed
//! field, exact in its count estimates over a grid, and honest about limbs.
mod specimens;
use std::collections::HashSet;
use telperion_core::{
    envelope::Envelope,
    field::Field,
    foliage::{self, plan, transform_point, CanopyParams, Element, Instances, TwigPlacement},
    math::Vec3,
    presets::{Family, Preset},
    surface::SurfaceParams,
    tree::{Node, NodeKind, Tree},
};

const SPECIES: [&str; 3] = ["oregon-white-oak", "silver-birch", "norway-spruce"];

struct Subject {
    tree: Tree,
    element: Element,
    plan: plan::Plan,
    planned: Field,
}
fn subject(id: &str) -> (Family, Subject) {
    let family = Preset::from_id(id).unwrap().parameters();
    let tree = specimens::tree(&family);
    let element = foliage::build_element(family.element).unwrap();
    let plan = plan::plan(
        &tree,
        family.skeleton.envelope,
        family.canopy,
        Some(TwigPlacement::of(&family).unwrap()),
        &family.surface,
        &element,
    )
    .unwrap()
    .unwrap_or_else(|| panic!("{id} has a plan"));
    let planned = Field::planned(&tree, &plan).unwrap();
    (
        family,
        Subject {
            tree,
            element,
            plan,
            planned,
        },
    )
}
/// The leaves the CPU placement retains, as the binding's foliage output does.
fn retained(family: &Family, s: &Subject) -> Instances {
    let placed = foliage::place_on_surface(
        &s.tree,
        family.skeleton.envelope,
        family.skeleton.seed,
        family.canopy,
        Some(TwigPlacement::of(family).unwrap()),
        &family.surface,
        foliage::Reference::of(family).unwrap(),
    )
    .unwrap();
    foliage::cull(placed, &s.element, family.skeleton.envelope, family.shell_depth).unwrap()
}
/// Cell centres of a grid of `cell` metres covering `bounds`.
fn grid(bounds: foliage::Bounds, cell: f64) -> impl Iterator<Item = Vec3> {
    let dims = [
        bounds.max.x - bounds.min.x,
        bounds.max.y - bounds.min.y,
        bounds.max.z - bounds.min.z,
    ]
    .map(|e| (e / cell).ceil() as usize + 1);
    let origin = bounds.min;
    (0..dims[0]).flat_map(move |i| {
        (0..dims[1]).flat_map(move |j| {
            (0..dims[2]).map(move |k| {
                origin + Vec3::new(i as f64 + 0.5, j as f64 + 0.5, k as f64 + 0.5) * cell
            })
        })
    })
}

/// R4: every vertex of every retained leaf lies in a cell the plan reports as
/// foliage, at a decimetre, a quarter metre and a metre.
#[test]
fn every_retained_leaf_vertex_lies_in_a_foliage_cell() {
    for id in SPECIES {
        let (family, s) = subject(id);
        let kept = retained(&family, &s);
        let origin = s.planned.bounds().unwrap().min;
        for cell in [0.1, 0.25, 1.0] {
            let mut cells = HashSet::new();
            for m in kept.matrices() {
                for v in &s.element.positions {
                    let p = transform_point(&m, *v) - origin;
                    cells.insert([
                        (p.x / cell).floor() as i64,
                        (p.y / cell).floor() as i64,
                        (p.z / cell).floor() as i64,
                    ]);
                }
            }
            let mut missed = 0;
            for c in &cells {
                let centre = origin + Vec3::new(c[0] as f64 + 0.5, c[1] as f64 + 0.5, c[2] as f64 + 0.5) * cell;
                if !s.planned.query(centre, cell / 2.).unwrap().foliage {
                    missed += 1;
                }
            }
            assert_eq!(
                missed, 0,
                "{id} at {cell} m: {missed} of {} vertex cells report no foliage",
                cells.len()
            );
        }
    }
}

/// R5: at a quarter metre the plan reports at most twice the foliage cells the
/// placed field does, for each of the three species.
#[test]
fn over_coverage_is_at_most_twice_the_placed_field_at_a_quarter_metre() {
    for id in SPECIES {
        let (family, s) = subject(id);
        let kept = retained(&family, &s);
        let placed = Field::new(&s.tree, Some((&kept, &s.element))).unwrap();
        let (mut base, mut planned) = (0usize, 0usize);
        for c in grid(s.planned.bounds().unwrap(), 0.25) {
            base += usize::from(placed.query(c, 0.125).unwrap().foliage);
            planned += usize::from(s.planned.query(c, 0.125).unwrap().foliage);
        }
        let ratio = planned as f64 / base as f64;
        eprintln!("{id}: base {base} planned {planned} ratio {ratio:.2}");
        assert!(ratio <= 2.0, "{id}: {planned} planned cells against {base} placed");
        assert!(
            s.planned.storage_bytes() < placed.storage_bytes(),
            "{id}: the plan stores more than the placed leaves"
        );
    }
}

/// R6: over a non-overlapping grid covering the tree the count estimates sum
/// to the plan's total within one percent.
#[test]
fn count_estimates_sum_to_the_plan_total_over_a_grid() {
    for id in SPECIES {
        let (_, s) = subject(id);
        let sum: f64 = grid(s.planned.bounds().unwrap(), 1.0)
            .map(|c| s.planned.query(c, 0.5).unwrap().leaves)
            .sum();
        let total = f64::from(s.plan.total);
        assert!(
            (sum - total).abs() <= total * 0.01,
            "{id}: estimates sum to {sum} against {total} stations"
        );
    }
}

/// A trunk with two limbs, each carrying one twig run of two segments.
fn two_limbs() -> (Tree, plan::Plan) {
    let node = |parent: u32, x: f64, y: f64, r: f64, kind: NodeKind| Node {
        parent: Some(parent),
        position: Vec3::new(x, y, 0.0),
        radius: r,
        start_radius: r,
        kind,
        ..Node::root()
    };
    let mut root = Node::root();
    root.radius = 0.2;
    root.start_radius = 0.2;
    let nodes = vec![
        root,
        node(0, 0.0, 1.0, 0.2, NodeKind::Structural),
        node(1, -1.0, 2.0, 0.1, NodeKind::Structural),
        node(1, 1.0, 2.0, 0.1, NodeKind::Structural),
        node(2, -1.5, 2.5, 0.01, NodeKind::Twig),
        node(4, -2.0, 3.0, 0.005, NodeKind::Twig),
        node(3, 1.5, 2.5, 0.01, NodeKind::Twig),
        node(6, 2.0, 3.0, 0.005, NodeKind::Twig),
    ];
    let tree = Tree {
        crossover: 4,
        nodes,
        ..Tree::default()
    };
    let element = foliage::build_element(Default::default()).unwrap();
    let plan = plan::plan(
        &tree,
        Envelope::default(),
        CanopyParams {
            size: 0.1,
            size_variation: 0.0,
            ..Default::default()
        },
        Some(TwigPlacement {
            internode_length: 0.5,
            stations_per_internode: 2,
        }),
        &SurfaceParams::default(),
        &element,
    )
    .unwrap()
    .unwrap();
    (tree, plan)
}

/// R6: the two limbs' foliage reports two limb ids, a cell reached by both
/// reports the one with the larger estimate, an exact tie the lower id, and a
/// cell no foliage reaches reports none.
#[test]
fn two_limbs_report_their_own_systems() {
    let (tree, plan) = two_limbs();
    let field = Field::planned(&tree, &plan).unwrap();
    let left = field.query(Vec3::new(-1.75, 2.75, 0.0), 0.3).unwrap();
    let right = field.query(Vec3::new(1.75, 2.75, 0.0), 0.3).unwrap();
    assert!(left.foliage && right.foliage);
    assert_eq!((left.limb, right.limb), (Some(1), Some(3)));
    assert!(left.leaves > 0.0 && right.leaves > 0.0);
    // Both runs whole, the tie broken toward the lower id.
    let whole = field.query(Vec3::new(0.0, 2.5, 0.0), 3.0).unwrap();
    assert_eq!(whole.limb, Some(1));
    assert_eq!(whole.leaves, f64::from(plan.total));
    // More of the right limb's run than the left's.
    let lean = field.query(Vec3::new(0.5, 2.5, 0.0), 2.0).unwrap();
    assert_eq!(lean.limb, Some(3));
    let bare = field.query(Vec3::new(0.0, 0.5, 0.0), 0.1).unwrap();
    assert!(bare.wood && !bare.foliage && bare.leaves == 0.0 && bare.limb.is_none());
    assert!(field.is_planned());
    let snapshot = field.snapshot().unwrap();
    assert_eq!(snapshot.plan.len(), plan.descriptors.len() * 7);
    assert_eq!(snapshot.plan_stations.len(), plan.descriptors.len() * 2);
    assert_eq!(snapshot.leaves.node_count, 0);
    assert!(snapshot.plan_index.node_count > 0);
}

/// R4: the spruce's needles reach about two centimetres; a cell ten times
/// that, and one a hundred times, still report the foliage a segment passes
/// through.
#[test]
fn spruce_needles_are_found_by_cells_far_larger_than_their_reach() {
    let (_, s) = subject("norway-spruce");
    let d = s.plan.descriptors[s.plan.descriptors.len() / 2];
    assert!(s.plan.reach(&d) < 0.05, "reach {}", s.plan.reach(&d));
    let mid = (d.endpoints[0] + d.endpoints[1]) * 0.5;
    for half in [0.12, 1.2] {
        let q = s.planned.query(mid, half).unwrap();
        assert!(q.foliage && q.leaves > 0.0 && q.limb.is_some(), "{q:?}");
    }
}

/// R4: a cube that only touches a sweep's boundary is covered; one clearly
/// beyond it is not.
#[test]
fn a_cube_touching_only_the_sweep_boundary_is_covered() {
    let (tree, plan) = two_limbs();
    let field = Field::planned(&tree, &plan).unwrap();
    let d = plan.descriptors[0];
    let reach = plan.reach(&d);
    let mid = (d.endpoints[0] + d.endpoints[1]) * 0.5;
    let half = 0.1;
    // A face touching the sweep's side, and a point on it.
    assert!(field.query(mid + Vec3::new(0., 0., reach + half), half).unwrap().foliage);
    assert!(field.query(mid + Vec3::new(0., 0., reach), 0.).unwrap().foliage);
    assert!(!field.query(mid + Vec3::new(0., 0., reach * 1.01), 0.).unwrap().foliage);
    // A corner touching it: the segment runs along (-1, 1, 0), so a cube
    // pushed out along (1, 1, 0) meets the sweep with an edge at root two
    // half extents, inside the circumsphere's root three.
    let across = Vec3::new(1., 1., 0.).normalized();
    assert!(field
        .query(mid + across * (reach + half * 2_f64.sqrt()), half)
        .unwrap()
        .foliage);
    assert!(!field
        .query(mid + across * (reach + half * 3_f64.sqrt() * 1.01), half)
        .unwrap()
        .foliage);
}

/// R1: every shipped preset but the beech, whose short shoots and limb
/// clumping have no plan, is described by the plan.
#[test]
fn every_shipped_preset_but_the_beech_has_a_plan() {
    for (id, planned) in [
        ("ordinary", true),
        ("oregon-white-oak", true),
        ("norway-spruce", true),
        ("european-beech", false),
        ("silver-birch", true),
        ("telperion", true),
        ("laurelin", true),
    ] {
        let family = Preset::from_id(id).unwrap().parameters();
        let twig = Some(TwigPlacement::of(&family).unwrap());
        assert_eq!(plan::supports(family.canopy, twig), planned, "{id}");
    }
}
