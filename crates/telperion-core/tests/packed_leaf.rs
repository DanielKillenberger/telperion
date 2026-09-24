//! The twelve-byte leaf, judged on the shipped presets rather than on cases
//! made up for it: the box every species is quantised against, that the box
//! does not move with age, and that no station the generator places falls
//! outside it.
use std::collections::BTreeMap;
use telperion_core::{
    branching::{Specimen, SpecimenRead},
    foliage::{Instances, Leaf, PlacementIdentity, Reference},
    math::Vec3,
    mesh, params, presets,
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
    let m = mesh::build(&family("silver-birch")).unwrap();
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
        let m = mesh::build(&f).unwrap();
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

/// R6's growth half: a leaf the growth path cached at one age decodes to the
/// same station at a later one.
///
/// This is the criterion the parameter-derived box exists for. `timeline`
/// writes a shoot's twelve bytes once and hands the same three words back at
/// every later age the shoot's wood has not changed under, while the tree
/// around them grows. The test takes only those leaves - the ones whose words
/// at the later age are byte-identical to the ones written at the earlier age,
/// so they were served from the cache and not re-derived - and asks three
/// things of them: that they decode to the same point, that the point still
/// stands off the wood its identity names at the same distance it did when it
/// was written, and that the point is still inside the box.
///
/// The last third says what a tree-derived box would have cost. The crown's
/// own bounds are not the same at the two ages, and the same cached words read
/// against the older crown's bounds land orders of magnitude further from the
/// station than the quarter millimetre R2 allows: that displacement is the
/// requantisation pass over every cached leaf that the family's box removes.
#[test]
fn a_leaf_cached_at_one_age_decodes_at_a_later_one() {
    let mut f = family("silver-birch");
    f.age = 16.0;
    let reference = Reference::of(&f).unwrap();

    let mut specimen = Specimen::build(&f).unwrap();
    let young = specimen.read().unwrap();
    assert!(
        !young.placements.is_empty(),
        "the young birch cached no leaf"
    );
    assert_eq!(
        young.reference, reference,
        "the read's box is not the family's"
    );

    specimen.advance(2.0).unwrap();
    let older = specimen.read().unwrap();
    assert_eq!(
        older.reference, reference,
        "the box moved as the tree grew, so cached words decode against another one"
    );
    assert!(
        older.placements.len() > young.placements.len(),
        "the birch grew no leaves between the two ages"
    );

    // Only the leaves the cache carried forward unchanged. A shoot whose wood
    // thickened is re-derived and says nothing about decoding an old word.
    let written: BTreeMap<_, _> = young
        .placements
        .iter()
        .map(|p| (p.identity, p.leaf))
        .collect();
    let carried: Vec<PlacementIdentity> = older
        .placements
        .iter()
        .filter(|p| written.get(&p.identity) == Some(&p.leaf))
        .map(|p| p.identity)
        .collect();
    assert!(
        carried.len() * 2 > written.len(),
        "only {} of {} leaves written at the earlier age were carried forward \
         unchanged, so the decode was barely exercised",
        carried.len(),
        written.len()
    );

    let standoff = |read: &SpecimenRead, id: PlacementIdentity, at: Vec3| {
        let i = read
            .tree
            .nodes
            .iter()
            .position(|n| n.identity == id.shoot)
            .unwrap_or_else(|| panic!("{id:?}: no shoot of that identity"));
        let node = &read.tree.nodes[i];
        let from = read.tree.nodes[node.parent.unwrap() as usize].position;
        let along = node.position - from;
        let t = ((at - from).dot(along) / along.length_squared().max(1e-12)).clamp(0.0, 1.0);
        (at - (from + along * t)).length()
    };
    let decode = |leaf: Leaf| Instances {
        leaves: vec![leaf],
        reference,
        thinned: 0,
    };
    let mut worst_standoff = 0.0_f64;
    for &id in &carried {
        let leaf = written[&id];
        // The same three words and the same box at both ages, so one decode.
        // What differs between the ages is the tree it is measured against.
        let at = decode(leaf).position(0);
        assert!(
            reference.contains(at),
            "{id:?}: the cached leaf decodes outside the box at the later age"
        );
        let moved = (standoff(&older, id, at) - standoff(&young, id, at)).abs();
        worst_standoff = worst_standoff.max(moved);
        assert!(
            moved <= 2.5e-4,
            "{id:?}: the cached leaf stands {moved} m further off its own wood \
             at the later age"
        );
    }

    // What a tree-derived box would have cost.
    let crown = |read: &SpecimenRead| {
        let instances = Instances {
            leaves: read.placements.iter().map(|p| p.leaf).collect(),
            reference,
            thinned: 0,
        };
        let mut lo = instances.position(0);
        let mut hi = lo;
        for i in 0..instances.len() {
            let p = instances.position(i);
            lo = Vec3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
            hi = Vec3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
        }
        Reference::spanning(lo, hi)
    };
    let older_crown = crown(&older);
    assert_ne!(
        crown(&young),
        older_crown,
        "the crown's own bounds did not move, so this comparison proves nothing"
    );
    let drift = carried
        .iter()
        .map(|&id| {
            let leaf = written[&id];
            (older_crown.position(leaf) - decode(leaf).position(0)).length()
        })
        .fold(0.0_f64, f64::max);
    assert!(
        drift > 2.5e-4,
        "reading the cached words against the older crown's own box drifts only \
         {drift} m, so a tree-derived box would have been harmless here"
    );
    println!(
        "{} of {} leaves carried forward unchanged, standing at most {worst_standoff:.2e} m \
         further off their wood; against the older crown's own box they would \
         drift up to {drift:.3} m",
        carried.len(),
        written.len()
    );
}
