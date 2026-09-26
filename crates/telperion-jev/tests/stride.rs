//! The gap-magnitude question and the labelled set that sets its cut.
//! No call is made.
use telperion_jev::tuning::stride::{self, Class, QUESTION};

#[test]
fn the_labelled_set_covers_every_class_and_the_palm_s_words() {
    let m = stride::labelled();
    assert!(m.min_confidence > 0. && m.min_confidence < 1.);
    let criteria = stride::questions()[QUESTION]["criteria"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    assert!(criteria.contains(&"no_match".to_string()));
    for split in ["tuning", "heldout"] {
        for answer in &criteria {
            assert!(
                m.cases
                    .iter()
                    .any(|c| c["split"] == split && &c["expected"][QUESTION] == answer),
                "the {split} split has no {answer} case"
            );
        }
    }
    // The runtime shows Jev exactly the shape the cases were labelled on.
    for c in &m.cases {
        let (p, f) = (&c["state"]["priority"], &c["state"]["finding"]);
        assert_eq!(
            stride::shown(p.as_str().unwrap(), f.as_str().unwrap()),
            c["state"]
        );
    }
    let labelled = |finding: &str| {
        m.cases
            .iter()
            .find(|c| c["state"]["finding"].as_str().unwrap().contains(finding))
            .map(|c| c["expected"][QUESTION].clone())
    };
    for words in ["far too short", "two to three times longer"] {
        assert_eq!(labelled(words), Some("far_off".into()), "{words}");
    }
    assert!(m.cases.iter().any(|c| c["state"]["priority"]
        .as_str()
        .unwrap()
        .contains("the fronds are far too thin and short today")));
}

#[test]
fn a_class_is_code_s_multiplier_and_no_match_is_none() {
    for (name, class, multiplier, lower) in [
        ("near", Class::Near, 1., Class::Near),
        ("clearly_off", Class::ClearlyOff, 2., Class::Near),
        ("far_off", Class::FarOff, 4., Class::ClearlyOff),
    ] {
        assert_eq!(Class::parse(name), Some(class));
        assert_eq!(class.name(), name);
        assert_eq!(class.multiplier(), multiplier);
        assert_eq!(class.lower(), lower);
    }
    assert_eq!(Class::parse("no_match"), None);
}
