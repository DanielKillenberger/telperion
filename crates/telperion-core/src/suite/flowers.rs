//! Flowers on an engine tree (fn-216): the beech's blossom test, built
//! through the one pipeline and through the executor's step, and walked
//! across its flowering share and its petal count with no pop.
use telperion_core::{
    foliage::{BloomParams, Instances},
    mesh,
    pipeline::{self, executor, Request},
    presets::{Family, Preset},
};

/// The beech young and small, flowering on its two finest ages.
fn blossoming(share: f64, petals: f64) -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    f.age = 15.0;
    let species = f
        .space
        .species
        .as_mut()
        .expect("the beech is an engine species");
    for age in [12, 14] {
        species.states[age].flowering = share;
    }
    f.bloom = BloomParams {
        florets: 4.0,
        pedicel: 0.025,
        droop: 0.6,
        petals,
        ..BloomParams::default()
    };
    f
}

fn florets(f: &Family) -> Instances {
    let built = pipeline::build(f, Request::mesh()).unwrap();
    built.outputs.organ.map(|o| o.instances).unwrap_or_default()
}

#[test]
fn a_tree_that_flowers_nowhere_has_no_flowers() {
    let built = mesh::build(&blossoming(0.0, 5.0)).unwrap();
    assert!(built.organ.is_none());
}

/// Both executors take the flowers through the one step, so they agree.
#[test]
fn both_executors_draw_the_same_flowers() {
    let f = blossoming(0.4, 5.0);
    let cpu = mesh::build(&f).unwrap().organ.unwrap();
    let step = executor::grow(&f)
        .unwrap()
        .expansion()
        .unwrap()
        .organ()
        .unwrap();
    let step = step.unwrap();
    assert!(cpu.instances.len() > 100, "{} florets", cpu.instances.len());
    assert_eq!(cpu.instances, step.instances);
    assert_eq!(cpu.element.positions, step.element.positions);
}

/// A walk across the share: the first florets to appear are barely
/// grown, and the crown's flowering only rises as the share does, so no
/// flower appears whole.
#[test]
fn a_walk_across_the_flowering_share_has_no_pop() {
    let size = |i: &Instances, k: usize| {
        let m = i.matrix(k);
        f64::from(m[4] * m[4] + m[5] * m[5] + m[6] * m[6]).sqrt()
    };
    let first = florets(&blossoming(0.005, 5.0));
    assert!(
        !first.is_empty(),
        "a share past the first draws flowers none"
    );
    let largest = (0..first.len())
        .map(|k| size(&first, k))
        .fold(0.0, f64::max);
    assert!(largest < 0.06, "a flower appeared at {largest} of its size");
    let mut last = 0.0;
    for step in 0..=20 {
        let now = florets(&blossoming(f64::from(step) / 20.0, 5.0));
        let total: f64 = (0..now.len()).map(|k| size(&now, k)).sum();
        assert!(total >= last - 1e-6, "flowers shrank as the share rose");
        last = total;
    }
}

/// A walk across the petal count changes no floret's place and grows the
/// next petal in from nothing (host decision 4).
#[test]
fn a_walk_across_the_petal_count_moves_no_floret() {
    let five = mesh::build(&blossoming(0.4, 5.0)).unwrap().organ.unwrap();
    let more = mesh::build(&blossoming(0.4, 5.25)).unwrap().organ.unwrap();
    assert_eq!(five.instances, more.instances);
    // One petal's vertices, from five petals against one: the element's
    // stalks (fn-253) are in both.
    let one = mesh::build(&blossoming(0.4, 1.0)).unwrap().organ.unwrap();
    let (five, one) = (five.element.positions.len(), one.element.positions.len());
    let petal = (five - one) / 4;
    assert_eq!(more.element.positions.len(), one + 5 * petal);
}

fn field(f: &Family, n: u32) -> telperion_core::field::Grid {
    let ask = Request {
        field: Some(pipeline::FieldRequest {
            n,
            limb_order: None,
        }),
        ..Request::default()
    };
    pipeline::build(f, ask).unwrap().outputs.field.unwrap()
}

/// The field counts the florets the drawn tree hangs (fn-316): each in
/// the cell its seat lies in, every one inside the grid, their sum the
/// tree's florets, and a tree that flowers nowhere answers none.
#[test]
fn the_field_counts_the_florets_the_tree_hangs() {
    use telperion_core::field;
    let f = blossoming(0.4, 5.0);
    let organ = florets(&f);
    let hung = field::seats(&organ).count();
    assert!(hung > 0, "the beech flowers");
    for n in [24, 36] {
        let grid = field(&f, n);
        let counted: f64 = grid.cells.iter().map(|c| c.flowers).sum();
        assert_eq!(counted, hung as f64, "{n} cells a side");
        let mut alone = grid.clone();
        alone.cells.iter_mut().for_each(|c| c.flowers = 0.0);
        field::florets_into(&mut alone, &organ).unwrap();
        assert_eq!(alone, grid, "the tree's own florets, cell by cell");
    }
    // Asked beside the leaves, the field's florets are the organ's own.
    let both = Request {
        field: Some(pipeline::FieldRequest {
            n: 24,
            limb_order: None,
        }),
        ..Request::mesh()
    };
    let built = pipeline::build(&f, both).unwrap().outputs;
    assert_eq!(built.organ.unwrap().instances.leaves, organ.leaves);
    assert_eq!(built.field.unwrap(), field(&f, 24));
    let bare = field(&blossoming(0.0, 5.0), 24);
    assert!(bare.cells.iter().all(|c| c.flowers == 0.0));
}

/// A floret whose seat lies outside the grid is refused by name, never
/// dropped.
#[test]
fn a_floret_outside_the_field_is_refused() {
    use telperion_core::field::{self, Frame, Grid, Occupancy};
    use telperion_core::math::Vec3;
    let organ = florets(&blossoming(0.4, 5.0));
    let seat = field::seats(&organ).next().unwrap();
    let frame = Frame {
        n: 3,
        cell: 1e-3,
        origin: seat + Vec3::new(1.0, 1.0, 1.0),
    };
    let mut grid = Grid {
        frame,
        cells: vec![Occupancy::default(); frame.cells()],
    };
    let refused = field::florets_into(&mut grid, &organ).unwrap_err();
    assert_eq!(
        refused,
        telperion_core::Error::InvalidInput("a floret outside the field")
    );
}
