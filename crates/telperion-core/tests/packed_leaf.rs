//! The twelve-byte leaf, judged on the shipped presets rather than on cases
//! made up for it: the box every species is quantised against, that the box
//! does not move with age, and that no station the generator places falls
//! outside it.
use telperion_core::{
    foliage::{Leaf, Reference},
    mesh::{self, Detail},
    params, presets,
    specimen::SpecimenView,
};

/// Every family the core ships, read off the catalogue itself so a species
/// added later is judged here without anyone remembering to add it. The
/// species pipeline's own catalogue carries two more - the European beech,
/// still in work, and the European ash, authored as a manifest - and
/// `species:qa` judges those over their manifests.
fn shipped() -> impl Iterator<Item = &'static str> {
    params::CATALOGUE.iter().map(|entry| entry.1)
}

fn family(id: &str) -> presets::Family {
    params::by_identity(id).unwrap_or_else(|e| panic!("{id} is not a shipped preset: {e}"))
}

/// R1, on the CPU: a leaf is twelve bytes, and a crown of n leaves is 12n.
#[test]
fn a_crown_is_twelve_bytes_a_leaf() {
    assert_eq!(size_of::<Leaf>(), 12);
    let m = mesh::build(&family("silver-birch"), Detail::Full).unwrap();
    let leaves = m.foliage.instances.leaves.as_slice();
    assert!(!leaves.is_empty(), "the birch placed no leaves");
    assert_eq!(size_of_val(leaves), leaves.len() * 12);
    assert_eq!(m.foliage_instances(), leaves.len());
}

/// R3: the box is the parameters', so it is the same box at every age. One
/// species grown to two of them, and the crown quantised against the same box
/// at both - which is what lets a leaf cached at one age decode at the next.
#[test]
fn the_reference_box_does_not_move_with_age() {
    let f = family("silver-birch");
    let mut view = SpecimenView::build(&f).unwrap();
    let young = view.mesh().unwrap().foliage.instances.reference;
    view.seek(view.frontier() / 3.0).unwrap();
    let younger = view.mesh().unwrap().foliage.instances.reference;
    assert_eq!(young, younger, "the birch's box moved between two ages");
    assert_eq!(
        young,
        Reference::of(&f).unwrap(),
        "the box is not the family's"
    );
    for id in shipped() {
        let mut young = family(id);
        let mature = family(id);
        young.age = (mature.age / 4.0).max(1.0);
        assert_eq!(
            Reference::of(&young).unwrap(),
            Reference::of(&mature).unwrap(),
            "{id}: the box moved between ages"
        );
        // And it is a box: a finite corner and three sides that are not
        // negative, which is what every decode divides the position by.
        let reference = Reference::of(&mature).unwrap();
        assert!(reference.is_finite(), "{id}: the box is not a box");
        assert!(reference.extent.y > 0.0, "{id}: the box is flat");
    }
}

/// R3: a station outside the box cannot occur. The debug assertion inside
/// `pack` says so while the tests run; this says so for the release build the
/// gate uses, over every leaf of every shipped species.
#[test]
fn no_station_of_any_shipped_species_falls_outside_its_box() {
    for id in shipped() {
        let f = family(id);
        let reference = Reference::of(&f).unwrap();
        let m = mesh::build(&f, Detail::Full).unwrap();
        let instances = &m.foliage.instances;
        assert_eq!(instances.reference, reference, "{id}: box differs");
        assert!(!instances.is_empty(), "{id}: no leaves to judge");
        // The decoded position is the clamped one, so a station that had
        // fallen outside would decode onto the wall. The crown's own bounds
        // are what tell on it: every leaf's translation has to stand strictly
        // inside the box, never on its face.
        let max = reference.max();
        for index in 0..instances.len() {
            let p = instances.position(index);
            assert!(reference.contains(p), "{id}: leaf {index} at {p:?}");
            assert!(
                p.x > reference.min.x
                    && p.y > reference.min.y
                    && p.z > reference.min.z
                    && p.x < max.x
                    && p.y < max.y
                    && p.z < max.z,
                "{id}: leaf {index} stands on the wall at {p:?}, so the box is too small"
            );
        }
    }
}

/// R2's position half: the round trip reproduces where a leaf stands to
/// within half a code step, and that step is under the quarter millimetre the
/// spec holds the encoding to, for every botanical species.
///
/// The two legendary families are held to nothing here and measured anyway.
/// Telperion is 148 m tall, so its box is four and a half times the tallest
/// real tree's and its step is four and a half times as coarse - a millimetre
/// on a silver tree taller than any that grows, which is the encoding
/// behaving exactly as stated rather than a defect.
#[test]
fn the_position_step_is_under_a_quarter_millimetre_for_every_species() {
    for id in shipped() {
        let reference = Reference::of(&family(id)).unwrap();
        let step = reference.step();
        let worst = step.x.max(step.y).max(step.z) / 2.0;
        println!(
            "{id}: half a step {worst:.3e} m, box {:?}",
            reference.extent
        );
        if matches!(id, "telperion" | "laurelin") {
            continue;
        }
        assert!(
            worst <= 2.5e-4,
            "{id}: half a step is {worst} m over a box of {:?}",
            reference.extent
        );
    }
}
