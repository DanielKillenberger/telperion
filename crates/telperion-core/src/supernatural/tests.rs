use super::*;
use crate::presets::Preset;
use crate::Error;

/// Every override set to a value its natural row admits, the scale and both
/// reflectances set, and the group off.
pub(crate) fn legendary(enabled: bool) -> Supernatural {
    let mut g = Supernatural {
        enabled,
        scale: 3.0,
        back_reflectance: 0.6,
        back_red: 0.9,
        back_green: 0.92,
        back_blue: 0.95,
        margin_reflectance: 0.7,
        roughness: 0.2,
        ..Supernatural::NONE
    };
    let mut f = Family {
        supernatural: g,
        ..Family::default()
    };
    for (over, natural) in mirrored() {
        let b = natural.info().bounds;
        over.set(&mut f).put(b.low + 0.25 * (b.high - b.low));
    }
    g = f.supernatural;
    for k in 0..SupernaturalFlowering::AGES {
        g.flowering.set(k, 0.5);
        g.internode.set(k, 0.2);
        g.pipe.set(k, 0.01);
    }
    g
}

#[test]
fn every_override_mirrors_its_natural_row() {
    let pairs: Vec<_> = mirrored().collect();
    let declared = SupernaturalMaterial::ROWS.len()
        + SupernaturalBloom::ROWS.len()
        + SupernaturalCanopy::ROWS.len();
    assert_eq!((pairs.len(), declared), (30, 30));
    for (over, natural) in pairs {
        let (o, n) = (over.info(), natural.info());
        assert_eq!(over.path(), format!("/supernatural{}", natural.path()));
        assert_eq!((o.bounds, o.unit, o.reads), (n.bounds, n.unit, n.reads));
    }
    assert_eq!(SupernaturalFlowering::ROWS.len(), telperion_space::AGES);
    assert_eq!(SupernaturalInternode::ROWS.len(), telperion_space::AGES);
    assert_eq!(SupernaturalPipe::ROWS.len(), telperion_space::AGES);
}

#[test]
fn a_group_that_is_off_applies_nothing() {
    let mut beech = Preset::EuropeanBeech.parameters();
    beech.supernatural = legendary(false);
    assert!(matches!(beech.applied(), Cow::Borrowed(_)));
    assert_eq!(beech.supernatural.scale(), 1.0);
    assert_eq!(beech.supernatural.reflection(), Reflection::NONE);
}

#[test]
fn a_group_that_is_on_lays_each_override_over_its_natural_row() {
    let mut beech = Preset::EuropeanBeech.parameters();
    beech.supernatural = legendary(true);
    let f = beech.applied();
    assert_eq!(f.material.leaf_front_red, 0.25);
    assert_eq!(f.material.margin_red, -0.5);
    assert_eq!(f.bloom.florets, 1.0 + 0.25 * 63.0);
    assert_eq!(f.canopy.size, 250.0);
    // A row the group does not override stays the natural one.
    assert_eq!(f.material.bark_red, beech.material.bark_red);
    let states = &f.space.species.as_ref().unwrap().states;
    assert!(states.iter().all(|s| s.flowering == 0.5));
    assert!(states
        .iter()
        .all(|s| (s.internode, s.form.pipe) == (0.2, 0.01)));
    assert_eq!(beech.supernatural.scale(), 3.0);
    assert_eq!(beech.supernatural.reflection().back_tint, [0.9, 0.92, 0.95]);
    // One override alone moves only its own row.
    let mut one = Preset::EuropeanBeech.parameters();
    one.supernatural.enabled = true;
    one.supernatural.bloom.petals = Some(6.0);
    let f = one.applied();
    assert_eq!((f.bloom.petals, f.bloom.open), (6.0, one.bloom.open));
    // The leaf's size alone moves only its own row (fn-248).
    let mut large = Preset::EuropeanBeech.parameters();
    large.supernatural.enabled = true;
    large.supernatural.canopy.size = Some(2.0);
    let f = large.applied();
    assert_eq!(
        (f.canopy.size, f.canopy.size_variation),
        (2.0, large.canopy.size_variation)
    );
    // An age's internode or pipe alone moves only its own age (fn-248).
    let mut coarse = Preset::EuropeanBeech.parameters();
    coarse.supernatural.enabled = true;
    coarse.supernatural.internode.set(6, 0.1);
    coarse.supernatural.pipe.set(6, 0.02);
    let natural = &coarse.space.species.as_ref().unwrap().states;
    let applied = coarse.applied();
    let states = &applied.space.species.as_ref().unwrap().states;
    for (k, (s, n)) in states.iter().zip(natural).enumerate() {
        let want = if k == 6 {
            (0.1, 0.02)
        } else {
            (n.internode, n.form.pipe)
        };
        assert_eq!((s.internode, s.form.pipe), want, "age {k}");
    }
}

#[test]
fn a_walk_from_the_group_off_ramps_up_from_the_natural_rows() {
    let off = Preset::EuropeanBeech.parameters();
    let mut on = off.clone();
    on.supernatural = legendary(true);
    // The other side's rows are its, too, while it is off: off is natural.
    let mut dormant = off.clone();
    dormant.supernatural = legendary(false);
    for from in [&off, &dormant] {
        let half = crate::blend::families(from, &on, 0.5).unwrap();
        let g = half.supernatural;
        assert!(g.enabled);
        assert_eq!(g.scale, 2.0);
        assert_eq!(g.back_reflectance, 0.3);
        let natural = off.material.leaf_front_red;
        assert_eq!(g.material.leaf_front_red, Some((natural + 0.25) / 2.0));
        assert_eq!(g.flowering.get(3), Some(0.25));
        // The leaf's size, an age's internode and its pipe walk up from the
        // natural rows too.
        assert_eq!(g.canopy.size, Some(linear(off.canopy.size, 250.0, 0.5)));
        let age = &off.space.species.as_ref().unwrap().states[6];
        assert_eq!(g.internode.get(6), Some(linear(age.internode, 0.2, 0.5)));
        assert_eq!(g.pipe.get(6), Some(linear(age.form.pipe, 0.01, 0.5)));
        let a = half.applied();
        assert_eq!(a.material.leaf_front_red, (natural + 0.25) / 2.0);
    }
    // Two families with the group off walk to none.
    let none = crate::blend::families(&off, &dormant, 0.5).unwrap();
    assert_eq!(none.supernatural, Supernatural::NONE);
}

#[test]
fn a_row_off_its_bounds_is_refused_by_name_whether_or_not_the_group_is_on() {
    for enabled in [false, true] {
        let mut f = Preset::EuropeanBeech.parameters();
        f.supernatural.enabled = enabled;
        f.supernatural.scale = 100.5;
        let refused = Error::InvalidValue {
            field: "scale",
            value: "100.5".into(),
        };
        assert_eq!(f.validate(), Err(refused));
        f.supernatural.scale = 0.1;
        assert_eq!(f.validate(), Ok(()));
        f.supernatural.material.margin_blue = Some(1.5);
        let name = "supernatural material marginBlue";
        assert_eq!(f.validate(), Err(Error::InvalidInput(name)));
        f.supernatural.material.margin_blue = None;
        // The leaf's size, an age's internode and pipe off their bounds
        // (fn-248).
        f.supernatural.canopy.size = Some(1000.5);
        let name = "supernatural canopy size";
        assert_eq!(f.validate(), Err(Error::InvalidInput(name)));
        f.supernatural.canopy.size = None;
        f.supernatural.internode.set(4, 0.0);
        let name = "supernatural internode";
        assert_eq!(f.validate(), Err(Error::InvalidInput(name)));
        f.supernatural.internode.set(4, 0.1);
        f.supernatural.pipe.set(4, 1.5);
        assert_eq!(f.validate(), Err(Error::InvalidInput("supernatural pipe")));
    }
}

/// A natural row an override stands in for is judged as written (fn-248):
/// switching the group off never uncovers a refused row.
#[test]
fn a_natural_row_under_an_override_is_still_judged() {
    let mut f = Preset::EuropeanBeech.parameters();
    f.supernatural.enabled = true;
    f.supernatural.internode.set(6, 0.1);
    f.space.species.as_mut().unwrap().states[6].internode = 0.0;
    assert!(matches!(
        f.validate(),
        Err(Error::InvalidValue {
            field: "engine",
            ..
        })
    ));
    let mut f = Preset::EuropeanBeech.parameters();
    f.supernatural.enabled = true;
    f.supernatural.canopy.size = Some(2.0);
    f.canopy.size = 1000.5;
    assert!(f.validate().is_err());
}

#[cfg(feature = "json")]
#[test]
fn the_group_reads_and_writes_on_the_wire_and_in_a_value_file() {
    let mut f = Preset::EuropeanBeech.parameters();
    f.supernatural = legendary(true);
    let wire = crate::params::metadata(&f);
    assert_eq!(wire["supernatural"]["bloom"]["petals"], 3.75);
    assert!(wire["supernatural"]["bloom"].is_object());
    let read = crate::params::parse(&wire).unwrap();
    assert_eq!(read.supernatural, f.supernatural);
    let written = crate::presets::values::write(&f, "", "").unwrap();
    assert!(
        written.contains("/supernatural/flowering/age14 = 0.5\n"),
        "{written}"
    );
    let back = crate::presets::values::read(&written).unwrap();
    assert_eq!(back.supernatural, f.supernatural);
}

/// Off is the base species, to the byte: every row of the group set and the
/// group off draws the tree the family with no group draws.
#[cfg(feature = "geometry")]
#[test]
fn a_family_with_the_group_off_draws_its_base_species_to_the_byte() {
    let mut beech = Preset::EuropeanBeech.parameters();
    beech.age = 12.0;
    let mut off = beech.clone();
    off.supernatural = legendary(false);
    let base = mesh(&beech);
    assert_eq!(mesh(&off), base);
    let mut on = off.clone();
    on.supernatural.enabled = true;
    assert_ne!(mesh(&on), base);
}

/// Each of the Two Trees with the group off is the beech (fn-218 R2,
/// fn-219 R2): every natural row of theirs is the beech preset's but the
/// age, so at one age each draws the beech's tree to the byte, and the
/// group changes it.
#[cfg(feature = "geometry")]
#[test]
fn the_two_trees_with_the_group_off_are_the_beech() {
    let mut beech = Preset::EuropeanBeech.parameters();
    beech.age = 12.0;
    let base = mesh(&beech);
    for preset in [Preset::Laurelin, Preset::Telperion] {
        let mut tree = preset.parameters();
        tree.age = 12.0;
        tree.supernatural.enabled = false;
        assert_eq!(mesh(&tree), base, "{preset:?}");
        tree.supernatural.enabled = true;
        assert_ne!(mesh(&tree), base, "{preset:?}");
    }
}

/// A hash of everything `f`'s mesh holds.
#[cfg(feature = "geometry")]
fn mesh(f: &Family) -> u64 {
    let text = format!("{:?}", crate::mesh::build(f).unwrap());
    text.bytes().fold(14695981039346656037_u64, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(1099511628211)
    })
}

/// The bark's metre rows are lengths: the drawn material carries them at
/// the group's scale, once, and every other row as the applied family's
/// (DESIGN.md 8.3). At scale one, and with the group off, it is the applied
/// material to the bit.
#[test]
fn the_bark_s_lengths_are_drawn_at_the_group_s_scale() {
    let mut beech = Preset::EuropeanBeech.parameters();
    beech.material.shoot_radius = 0.02;
    beech.material.mature_radius = 0.3;
    beech.material.mature_span = 0.4;
    let bits = |m: &crate::material::MaterialParams| format!("{m:?}");
    for (enabled, scale) in [(false, 3.0), (true, 1.0)] {
        beech.supernatural = Supernatural {
            enabled,
            scale,
            ..legendary(enabled)
        };
        assert_eq!(
            bits(&beech.drawn_material()),
            bits(&beech.applied().material)
        );
    }
    beech.supernatural = legendary(true);
    let (applied, drawn) = (beech.applied().material, beech.drawn_material());
    assert_eq!(
        (drawn.shoot_radius, drawn.mature_radius, drawn.mature_span),
        (0.02 * 3.0, 0.3 * 3.0, 0.4 * 3.0)
    );
    let restored = crate::material::MaterialParams {
        shoot_radius: applied.shoot_radius,
        mature_radius: applied.mature_radius,
        mature_span: applied.mature_span,
        ..drawn
    };
    assert_eq!(bits(&restored), bits(&applied));
}
