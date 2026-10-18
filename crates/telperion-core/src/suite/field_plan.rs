//! The field read from the leaf plan: conservative against the leaves the
//! CPU placement retains, exact in its count estimates over a grid, and
//! honest about limbs. A cube is asked as the middle cell of a grid of three.
use super::specimens;
use telperion_core::{
    envelope::Envelope,
    field::{self, Frame, Grid, Occupancy},
    foliage::{self, plan, transform_point, CanopyParams, Element, Instances, TwigPlacement},
    math::Vec3,
    presets::{Family, Preset},
    surface::SurfaceParams,
    tree::{Node, NodeKind, Tree},
};

/// The scaffold's species the plan is judged on: the oak and spruce it was
/// judged on are engine presets now, whose trees the scaffold no longer
/// grows (fn-198); the birch and the ordinary tree are.
const SPECIES: [&str; 2] = ["silver-birch", "ordinary"];

struct Subject {
    tree: Tree,
    element: Element,
    plan: plan::Plan,
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
        None,
        1,
    )
    .unwrap()
    .unwrap_or_else(|| panic!("{id} has a plan"));
    (
        family,
        Subject {
            tree,
            element,
            plan,
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
        foliage::Reference::grown(&s.tree, foliage::Reach::of(family).unwrap()),
    )
    .unwrap();
    foliage::cull(
        placed,
        &s.element,
        family.skeleton.envelope,
        family.shell_depth,
    )
    .unwrap()
}
/// The field of `plan` on `tree` read to `n` cells a side over its box.
pub(super) fn grid(tree: &Tree, plan: &plan::Plan, n: u32) -> Grid {
    let b = field::bounds(tree, plan).unwrap().unwrap();
    field::read(tree, plan, Frame::over(b, n).unwrap()).unwrap()
}
/// The cell of `g` holding `p`.
pub(super) fn holding(g: &Grid, p: Vec3) -> &Occupancy {
    let f = g.frame;
    let i = |v: f64, o: f64| ((v - o) / f.cell).floor() as usize;
    let n = f.n as usize;
    &g.cells[i(p.x, f.origin.x) + n * (i(p.y, f.origin.y) + n * i(p.z, f.origin.z))]
}
/// The closed cube of `half` about `centre`: the middle cell of a grid of
/// three.
fn cube(tree: &Tree, plan: &plan::Plan, centre: Vec3, half: f64) -> Occupancy {
    let frame = Frame {
        n: 3,
        cell: 2. * half,
        origin: centre - Vec3::new(3. * half, 3. * half, 3. * half),
    };
    field::read(tree, plan, frame).unwrap().cells[13]
}

/// R4: every vertex of every retained leaf lies in a cell the plan reports as
/// foliage, on grids of 24, 48 and 96 cells a side.
#[test]
fn every_retained_leaf_vertex_lies_in_a_foliage_cell() {
    for id in SPECIES {
        let (family, s) = subject(id);
        let kept = retained(&family, &s);
        for n in [24, 48, 96] {
            let g = grid(&s.tree, &s.plan, n);
            let (mut missed, mut all) = (0, 0);
            for m in kept.matrices() {
                for v in &s.element.positions {
                    all += 1;
                    missed += usize::from(!holding(&g, transform_point(&m, *v)).foliage);
                }
            }
            assert_eq!(missed, 0, "{id} at {n}: {missed} of {all} vertices");
        }
    }
}

/// R6: over the grid the count estimates sum to the plan's total within one
/// percent.
#[ignore = "fn-198: the scaffold's grower on an engine preset's rows, which no build grows now; R0 removes both"]
#[test]
fn count_estimates_sum_to_the_plan_total_over_a_grid() {
    for id in SPECIES {
        let (_, s) = subject(id);
        let sum: f64 = grid(&s.tree, &s.plan, 24)
            .cells
            .iter()
            .map(|c| c.leaves)
            .sum();
        let total = f64::from(s.plan.total);
        assert!(
            (sum - total).abs() <= total * 0.01,
            "{id}: estimates sum to {sum} against {total} stations"
        );
    }
}

/// A trunk with two limbs, each carrying one twig run of two segments, its
/// plan at the given limb order (`None` for the family's).
fn two_limbs(order: Option<u32>) -> (Tree, plan::Plan) {
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
        order,
        1,
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
    let (tree, plan) = two_limbs(None);
    let q = |c: Vec3, half: f64| cube(&tree, &plan, c, half);
    let left = q(Vec3::new(-1.75, 2.75, 0.0), 0.3);
    let right = q(Vec3::new(1.75, 2.75, 0.0), 0.3);
    assert!(left.foliage && right.foliage);
    assert_eq!((left.limb, right.limb), (Some(1), Some(3)));
    assert!(left.leaves > 0.0 && right.leaves > 0.0);
    // Both runs whole, the tie broken toward the lower id.
    let whole = q(Vec3::new(0.0, 2.5, 0.0), 3.0);
    assert_eq!(whole.limb, Some(1));
    assert_eq!(whole.leaves, f64::from(plan.total));
    // More of the right limb's run than the left's.
    let lean = q(Vec3::new(0.5, 2.5, 0.0), 2.0);
    assert_eq!(lean.limb, Some(3));
    let bare = q(Vec3::new(0.0, 0.5, 0.0), 0.1);
    assert!(bare.wood && !bare.foliage && bare.leaves == 0.0 && bare.limb.is_none());
}

/// R10: the plan at limb order zero keeps both limbs on the stem's system,
/// the family's order is what no order asks for, and a deeper order changes
/// nothing on a fixture with no deeper laterals.
#[test]
fn the_limb_order_selects_how_finely_the_crown_parts() {
    let (tree, coarse) = two_limbs(Some(0));
    let left = cube(&tree, &coarse, Vec3::new(-1.75, 2.75, 0.0), 0.3);
    let right = cube(&tree, &coarse, Vec3::new(1.75, 2.75, 0.0), 0.3);
    assert_eq!((left.limb, right.limb), (Some(1), Some(1)));
    let (_, family) = two_limbs(None);
    assert_eq!(
        family,
        two_limbs(Some(CanopyParams::default().clump_system_order)).1
    );
    assert_eq!(family, two_limbs(Some(9)).1);
    assert_ne!(family, coarse);
}

/// R9: a cell reports the largest radius of any wood sweep reaching it, in
/// metres, and zero where no wood does; the wood flag is untouched.
#[test]
fn wood_radius_is_the_thickest_sweep_reaching_the_cell() {
    let (tree, plan) = two_limbs(None);
    let q = |c: Vec3, half: f64| cube(&tree, &plan, c, half);
    let trunk = q(Vec3::new(0.0, 0.5, 0.0), 0.1);
    assert!(trunk.wood);
    assert_eq!(trunk.wood_radius, 0.2);
    let twig = q(Vec3::new(-2.0, 3.0, 0.0), 0.05);
    assert!(twig.wood);
    assert_eq!(twig.wood_radius, 0.005);
    let whole = q(Vec3::new(0.0, 2.0, 0.0), 4.0);
    assert_eq!(whole.wood_radius, 0.2);
    let bare = q(Vec3::new(0.0, -5.0, 0.0), 0.1);
    assert!(!bare.wood);
    assert_eq!(bare.wood_radius, 0.0);
    for id in SPECIES {
        let (_, s) = subject(id);
        for c in grid(&s.tree, &s.plan, 24).cells {
            assert_eq!(c.wood, c.wood_radius > 0.0, "{id}: {c:?}");
        }
    }
}

/// R4: the spruce's needles reach about two centimetres; a cell ten times
/// that, and one a hundred times, still report the foliage a segment passes
/// through.
#[ignore = "fn-198: the scaffold's grower on an engine preset's rows, which no build grows now; R0 removes both"]
#[test]
fn spruce_needles_are_found_by_cells_far_larger_than_their_reach() {
    let (_, s) = subject("norway-spruce");
    let d = s.plan.descriptors[s.plan.descriptors.len() / 2];
    assert!(s.plan.reach(&d) < 0.05, "reach {}", s.plan.reach(&d));
    let mid = (d.endpoints[0] + d.endpoints[1]) * 0.5;
    for half in [0.12, 1.2] {
        let q = cube(&s.tree, &s.plan, mid, half);
        assert!(q.foliage && q.leaves > 0.0 && q.limb.is_some(), "{q:?}");
    }
}

/// R4: a cube that only touches a sweep's boundary is covered; one clearly
/// beyond it is not.
#[test]
fn a_cube_touching_only_the_sweep_boundary_is_covered() {
    let (tree, plan) = two_limbs(None);
    let q = |c: Vec3, half: f64| cube(&tree, &plan, c, half).foliage;
    let d = plan.descriptors[0];
    let reach = plan.reach(&d);
    let mid = (d.endpoints[0] + d.endpoints[1]) * 0.5;
    let half = 0.1;
    // A face touching the sweep's side, and a cube a hair across on it.
    let point = 1e-9;
    assert!(q(mid + Vec3::new(0., 0., reach + half), half));
    assert!(q(mid + Vec3::new(0., 0., reach), point));
    assert!(!q(mid + Vec3::new(0., 0., reach * 1.01), point));
    // A corner touching it: the segment runs along (-1, 1, 0), so a cube
    // pushed out along (1, 1, 0) meets the sweep with an edge at root two
    // half extents, inside the circumsphere's root three.
    let across = Vec3::new(1., 1., 0.).normalized();
    assert!(q(mid + across * (reach + half * 2_f64.sqrt()), half));
    assert!(!q(
        mid + across * (reach + half * 3_f64.sqrt() * 1.01),
        half
    ));
}

/// R1: every shipped preset is described by the plan; the date palm's fronds
/// frond by frond (fn-150). The beech, whose short shoots and limb clumping
/// had none, is dressed for the engine's tree with neither (fn-198).
#[test]
fn every_shipped_preset_has_a_plan() {
    for id in [
        "ordinary",
        "oregon-white-oak",
        "norway-spruce",
        "european-beech",
        "silver-birch",
        "telperion",
        "laurelin",
        "date-palm",
    ] {
        let family = Preset::from_id(id).unwrap().parameters();
        let twig = Some(TwigPlacement::of(&family).unwrap());
        assert!(plan::supports(family.canopy, twig), "{id}");
    }
}
