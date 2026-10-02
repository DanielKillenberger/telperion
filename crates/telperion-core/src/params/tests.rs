//! What the wire publishes, read back through the wire.
use super::*;
use crate::presets::Preset;

#[test]
fn the_material_row_travels_the_wire_and_is_refused_by_field_name() {
    // Every shipped family carries a row a leaf can be drawn from, and the
    // row survives the wire in both directions like any other trait.
    for &(abi, _, _, _) in CATALOGUE.iter() {
        let family = preset(abi).unwrap();
        assert_eq!(family.material.validate(), Ok(()));
        let mut value = metadata(&family);
        assert_eq!(value, metadata(&parse(&value).unwrap()));
        for (trait_name, set) in [
            ("barkRed", json!(0.4)),
            ("barkRoughness", json!(0.45)),
            ("leafFrontGreen", json!(0.3)),
            ("leafBackBlue", json!(0.2)),
            ("hueRangeLow", json!(-0.2)),
            ("brightnessRangeHigh", json!(0.4)),
            ("interiorDarkening", json!(0.9)),
            ("barkReflectance", json!(0.1)),
            ("bladeGrainScale", json!(120.0)),
        ] {
            value["material"][trait_name] = set;
            assert_eq!(value, metadata(&parse(&value).unwrap()));
        }
    }
    // A value off its range and a range that runs backwards are both
    // refused by the wire, each naming what was wrong with it.
    for (trait_name, bad, message) in [
        ("barkRed", json!(1.5), "bark red"),
        ("barkRoughness", json!(-0.1), "bark roughness"),
        ("leafFrontGreen", json!(2.0), "leaf front green"),
        ("interiorDarkening", json!(1.4), "leaf interior darkening"),
        ("hueRangeLow", json!(0.4), "leaf hue range"),
        ("brightnessRangeLow", json!(0.9), "leaf brightness range"),
        ("leafReflectance", json!(1.5), "leaf reflectance"),
        ("bladeGrainScale", json!(300.0), "leaf blade grain scale"),
    ] {
        let mut value = metadata(&preset(0).unwrap());
        value["material"][trait_name] = bad;
        assert_eq!(parse(&value).err(), Some(Error::InvalidInput(message)));
    }
    // The material is a closed schema too: a swatch name is not a trait.
    assert_eq!(
        parse(&json!({"material":{"barkTexture":"furrowed"}})).err(),
        Some(Error::InvalidInput("unknown material trait"))
    );
}

#[test]
fn catalogue_roundtrips_all_controls_and_identities() {
    for &(abi, id, _, _) in CATALOGUE.iter().rev() {
        let mut value = metadata(&preset(abi).unwrap());
        assert_eq!(value, metadata(&parse(&json!(id)).unwrap()));
        assert_eq!(value, metadata(&parse(&value).unwrap()));
        value["skeleton"]["bias"]["supernatural"] = json!({"enabled":false,"writheAmplitude":0.12,"writheWavelength":0.4,"spiralRate":3.0,"maxWritheMagnitude":0.9});
        value["element"]["connectorLength"] = json!(0.002);
        // Every habit trait is a flat numeric row, set one at a time.
        for (trait_name, set) in [
            ("apicalDominance", json!(0.75)),
            ("whorlStrength", json!(0.4)),
            ("lateralsPerStation", json!(4)),
            ("riseSecondary", json!(-0.5)),
            ("crookedness", json!(13.0)),
            ("lateralOrders", json!(2)),
            ("attractorWeight", json!(0.0)),
        ] {
            value["skeleton"]["habit"][trait_name] = set;
            assert_eq!(value, metadata(&parse(&value).unwrap()));
        }
    }
    assert!(preset(999).is_err());
    assert!(parse(&json!("missing")).is_err());
    // The element's outline traits are flat numeric rows too.
    for &(abi, _, _, _) in CATALOGUE.iter() {
        let mut value = metadata(&preset(abi).unwrap());
        for (trait_name, set) in [
            ("lobeCount", json!(3)),
            ("lobeDepth", json!(0.55)),
            ("sectionRoundness", json!(0.4)),
            ("axialSegments", json!(20)),
        ] {
            value["element"][trait_name] = set;
            assert_eq!(value, metadata(&parse(&value).unwrap()));
        }
        assert!(crate::foliage::build_element(parse(&value).unwrap().element).is_ok());
    }
    // An anatomy tag is a retired shape, not a parameter: the closed schema
    // refuses it by name, and every trait keeps its range.
    for bad in [
        json!({"anatomy":"lobedBlade"}),
        json!({"anatomy":"fourSidedNeedle","width":0.02}),
    ] {
        assert_eq!(
            parse(&json!({"element":bad})).err(),
            Some(Error::InvalidInput("unknown element trait"))
        );
    }
    for (trait_name, bad, message) in [
        ("lobeCount", json!(9), "leaf lobe count"),
        ("lobeDepth", json!(1.5), "leaf lobe depth"),
        ("sectionRoundness", json!(-0.5), "leaf section roundness"),
        (
            "lobeDepth",
            json!(0.5),
            "leaf axial segments too few for the lobe count",
        ),
    ] {
        let mut value = metadata(&preset(0).unwrap());
        value["element"]["lobeCount"] = json!(5);
        value["element"][trait_name] = bad;
        assert_eq!(
            parse(&value).and_then(|f| crate::foliage::build_element(f.element)),
            Err(Error::InvalidInput(message))
        );
    }
    // The canopy's lean and contact are flat numeric rows as well.
    for &(abi, _, _, _) in CATALOGUE.iter() {
        let mut value = metadata(&preset(abi).unwrap());
        for (trait_name, set) in [
            ("forwardLean", json!(0.3)),
            ("leanRise", json!(0.8)),
            ("surfaceContact", json!(0.5)),
            // Signed, so a leaf can lean back down its shoot and toward the
            // ground; the wire carries the sign as readily as the magnitude.
            ("forwardLean", json!(-0.45)),
            ("leanRise", json!(-1.5)),
            ("outward", json!(-0.2)),
            ("upward", json!(-0.6)),
            // Short shoots are rows too, neutral at a spacing of zero.
            ("shortShootSpacing", json!(0.12)),
            ("shortShootRadius", json!(0.3)),
            ("shortShootLength", json!(0.02)),
            ("shortShootLeaves", json!(5)),
            ("shortShootSpread", json!(60.0)),
            // So is the gap between limb systems, neutral at none.
            ("limbClumping", json!(0.4)),
        ] {
            value["canopy"][trait_name] = set;
            assert_eq!(value, metadata(&parse(&value).unwrap()));
        }
    }
    // An attachment tag is a retired shape, not a parameter: the closed
    // schema refuses it by name, and every trait keeps its range.
    for bad in [
        json!({"attachment":"alternate"}),
        json!({"attachment":"radialNeedles","shootRadius":0.02}),
    ] {
        assert_eq!(
            parse(&json!({"canopy":bad})).err(),
            Some(Error::InvalidInput("unknown canopy trait"))
        );
    }
    for (trait_name, bad, message) in [
        ("forwardLean", json!(1.5), "forward lean"),
        ("forwardLean", json!(-1.5), "forward lean"),
        ("leanRise", json!(-2.5), "lean rise"),
        ("outward", json!(-1.5), "outward"),
        ("upward", json!(-1.5), "upward"),
        ("surfaceContact", json!(2.0), "surface contact"),
        ("shortShootSpacing", json!(0.001), "short shoot spacing"),
        ("shortShootRadius", json!(1.5), "short shoot radius"),
        ("shortShootLength", json!(0.6), "short shoot length"),
        ("shortShootLeaves", json!(9), "short shoot leaves"),
        ("shortShootSpread", json!(95.0), "short shoot spread"),
        ("limbClumping", json!(1.5), "limb clumping"),
        ("limbClumping", json!(-0.1), "limb clumping"),
    ] {
        let mut value = metadata(&preset(0).unwrap());
        value["canopy"][trait_name] = bad;
        let f = parse(&value).unwrap();
        assert_eq!(
            crate::foliage::place(
                &crate::branching::generate(&f.skeleton, f.radii)
                    .unwrap()
                    .tree,
                f.skeleton.envelope,
                f.skeleton.seed,
                f.canopy,
                None,
                crate::foliage::Reference::of(&f).unwrap(),
            )
            .err(),
            Some(Error::InvalidInput(message))
        );
    }
    // A kind tag is a retired shape, not a parameter: the closed schema
    // refuses it by name, and every trait keeps its range.
    for bad in [
        json!({"kind":"spreading"}),
        json!({"kind":"tiered","tiers":4}),
        json!({"scaffoldLimbs":6}),
    ] {
        assert_eq!(
            parse(&json!({"skeleton":{"habit":bad}})).err(),
            Some(Error::InvalidInput("unknown habit trait"))
        );
    }
    for (trait_name, bad, message) in [
        ("apicalDominance", json!(1.5), "apical dominance"),
        ("whorlStrength", json!(-0.1), "whorl strength"),
        ("leaderInternode", json!(0.0), "leader internode"),
        ("lateralsPerStation", json!(0), "laterals per station"),
        ("crookedness", json!(90.0), "crookedness"),
        ("riseSecondary", json!(-2.0), "secondary rise per order"),
        ("attractorWeight", json!(2.0), "attractor weight"),
    ] {
        let mut habit = json!({});
        habit[trait_name] = bad;
        assert_eq!(
            parse(&json!({"skeleton":{"habit":habit}})).and_then(|f| f.skeleton.habit.validate()),
            Err(Error::InvalidInput(message))
        );
    }
    assert!(parse(&json!({"skeleton":{"bias":{"writheAmplitude":0.1}}})).is_err());
    assert!(parse(&json!({"skeleton":{"bias":{"supernatural":{"enabled":1}}}})).is_err());
}

#[test]
fn age_round_trips_and_refuses_invalid_values() {
    let mut family = preset(0).unwrap();
    family.age = 12.25;
    let wire = metadata(&family);
    assert_eq!(wire["age"], 12.25);
    assert_eq!(parse(&wire).unwrap().age, family.age);
    for value in [-1.0, 1_000_001.0] {
        let mut bad = wire.clone();
        bad["age"] = serde_json::json!(value);
        let message = parse(&bad).unwrap_err().to_string();
        assert!(message.contains("age"), "{message}");
        assert!(message.contains(&value.to_string()), "{message}");
        let mut f = family.clone();
        f.age = value;
        assert!(f.validate().unwrap_err().to_string().contains("age"));
    }
}

/// The growth path's rows went with it: a wire that still names them is
/// refused by name, never read silently.
#[test]
fn the_retired_growth_rows_are_refused_by_name() {
    let why = "/growth is retired";
    for wire in [json!({"growth": {"rate": 0.08}}), json!({"growth": {}})] {
        let error = parse(&wire).unwrap_err().to_string();
        assert!(error.contains(why), "{error}");
    }
    let oak = preset(0).unwrap();
    let error = overlay(&oak, &json!({"growth": {"leafLifetime": 6.0}})).unwrap_err();
    assert!(error.to_string().contains(why), "{error}");
}



#[test]
fn a_table_in_work_is_reserved_unlisted_and_not_built_by_name() {
    for &(abi, id, _, _) in IN_WORK {
        assert!(
            CATALOGUE
                .iter()
                .all(|entry| entry.0 != abi && entry.1 != id),
            "{id}"
        );
        assert!(preset(abi).is_err(), "{id} served by id");
        assert!(by_identity(id).is_err(), "{id} built by name");
        assert!(parse(&json!(id)).is_err(), "{id} parsed by name");
        assert!(
            Preset::from_id(id).is_some(),
            "{id} unreachable by the core"
        );
    }
}

#[test]
fn an_overlay_moves_the_rows_it_names_and_nothing_else() {
    let base = preset(3).unwrap();
    let moved = overlay(
        &base,
        &json!({"skeleton": {"habit": {"lateralPitch": 51.0}}, "radii": {"forkExponent": 2.5}}),
    )
    .unwrap();
    let mut expected = metadata(&base);
    expected["skeleton"]["habit"]["lateralPitch"] = json!(51.0);
    expected["radii"]["forkExponent"] = json!(2.5);
    assert_eq!(metadata(&moved), expected);
    assert_eq!(
        metadata(&overlay(&base, &json!({})).unwrap()),
        metadata(&base)
    );
    assert_eq!(
        overlay(&base, &json!({"skeleton": {"habit": {"pitch": 1.0}}})).err(),
        Some(Error::InvalidInput("unknown family parameter"))
    );
    assert_eq!(
        overlay(&base, &json!({"material": {"barkRed": 1.5}})).err(),
        Some(Error::InvalidInput("bark red"))
    );
}

/// A wire with several values of the wrong type is refused by the one that
/// came first on the wire before the catalogue, whatever order the catalogue
/// declares its groups in.
#[test]
fn the_first_malformed_row_on_the_wire_names_the_refusal() {
    for (wire, first) in [
        (
            json!({"skeleton": {"habit": {"apicalDominance": "bad"}}, "shellDepth": "bad"}),
            "/skeleton/habit/apicalDominance",
        ),
        (
            json!({"skeleton": {"habit": {"reachProbeSteps": "bad", "apicalDominance": "bad"}}}),
            "/skeleton/habit/apicalDominance",
        ),
        (
            json!({"material": {"barkRed": "bad"}, "shellDepth": "bad"}),
            "/material/barkRed",
        ),
        (
            json!({"element": {"connectorLength": "bad"}, "skeleton": {"seed": "bad"}}),
            "/element/connectorLength",
        ),
    ] {
        assert_eq!(
            parse(&wire).unwrap_err(),
            Error::InvalidInput(first),
            "{wire}"
        );
    }
}
