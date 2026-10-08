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
    }
    g
}

#[test]
fn every_override_mirrors_its_natural_row() {
    let pairs: Vec<_> = mirrored().collect();
    let declared = SupernaturalMaterial::ROWS.len() + SupernaturalBloom::ROWS.len();
    assert_eq!((pairs.len(), declared), (25, 25));
    for (over, natural) in pairs {
        let (o, n) = (over.info(), natural.info());
        assert_eq!(over.path(), format!("/supernatural{}", natural.path()));
        assert_eq!((o.bounds, o.unit, o.reads), (n.bounds, n.unit, n.reads));
    }
    assert_eq!(SupernaturalFlowering::ROWS.len(), telperion_space::AGES);
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
    // A row the group does not override stays the natural one.
    assert_eq!(f.material.bark_red, beech.material.bark_red);
    let states = &f.space.species.as_ref().unwrap().states;
    assert!(states.iter().all(|s| s.flowering == 0.5));
    assert_eq!(beech.supernatural.scale(), 3.0);
    assert_eq!(beech.supernatural.reflection().back_tint, [0.9, 0.92, 0.95]);
    // One override alone moves only its own row.
    let mut one = Preset::EuropeanBeech.parameters();
    one.supernatural.enabled = true;
    one.supernatural.bloom.petals = Some(6.0);
    let f = one.applied();
    assert_eq!((f.bloom.petals, f.bloom.open), (6.0, one.bloom.open));
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
    }
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
    let mesh = |f: &Family| {
        let text = format!("{:?}", crate::mesh::build(f).unwrap());
        text.bytes().fold(14695981039346656037_u64, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(1099511628211)
        })
    };
    let base = mesh(&beech);
    assert_eq!(mesh(&off), base);
    let mut on = off.clone();
    on.supernatural.enabled = true;
    assert_ne!(mesh(&on), base);
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
