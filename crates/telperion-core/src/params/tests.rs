//! What the wire publishes, read back through the wire.
use super::*;

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
        value["skeleton"]["bias"]["supernatural"] =
            json!({"enabled":false,"writheAmplitude":0.12,"writheWavelength":0.4,"spiralRate":3.0});
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
        ("leanRise", json!(-0.1), "lean rise"),
        ("surfaceContact", json!(2.0), "surface contact"),
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
