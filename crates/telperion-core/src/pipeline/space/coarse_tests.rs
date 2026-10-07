//! A coarse tree (fn-214): the request's cut reaches the engine, its fine
//! roots stand in the field as their expected occupancy, and a species the
//! cut cannot draw is refused by name.
use crate::pipeline::{build, Request};
use crate::presets::Preset;

fn field(cut: u32) -> Request {
    Request {
        field: Some(None),
        cut,
        ..Request::default()
    }
}

/// The spruce's fine roots stand in its field: its plan holds a capsule for
/// each, carrying leaves, and the field answers foliage where they stand.
#[test]
fn a_coarse_spruce_answers_its_fine_roots_as_occupancy() {
    let mut family = Preset::NorwaySpruce.parameters();
    family.age = 20.0;
    let built = build(&family, field(1024)).expect("builds coarse");
    let stands = &built.skeleton.tree.stands;
    assert!(!stands.is_empty());
    let plan = built.outputs.plan.expect("a plan");
    let spread: Vec<_> = plan.descriptors.iter().filter(|d| d.spread > 0.0).collect();
    assert!(!spread.is_empty() && spread.len() <= stands.len());
    assert!(spread.iter().map(|d| d.count).sum::<u32>() > 0);
    let field = built.outputs.field.expect("a field");
    let s = &stands[stands.len() / 2];
    let middle = (s.base + s.tip) * 0.5;
    assert!(field.query(middle, 0.05).expect("answers").foliage);
}

/// The palm bears no lateral: at any cut its field is the full field's.
#[test]
fn a_cut_is_dormant_where_no_root_is_fine() {
    let family = Preset::DatePalm.parameters();
    let full = build(&family, field(0)).expect("builds");
    let coarse = build(&family, field(1024)).expect("builds coarse");
    assert!(coarse.skeleton.tree.stands.is_empty());
    let (a, b) = (full.outputs.field.unwrap(), coarse.outputs.field.unwrap());
    let (a, b) = (a.snapshot().unwrap(), b.snapshot().unwrap());
    assert_eq!((a.wood, a.plan), (b.wood, b.plan));
}

/// The oak's structure above any cut reads its fine roots as grown, and a
/// coarse tree has no wood or leaves to draw: both are refused by name.
#[test]
fn a_cut_the_tree_cannot_take_is_refused() {
    let mut family = Preset::OregonWhiteOak.parameters();
    family.age = 5.0;
    let oak = build(&family, field(1024)).err().expect("refused");
    assert!(oak.to_string().contains("cut"), "{oak}");
    let wood = Request {
        wood: true,
        ..field(1024)
    };
    let drawn = build(&Preset::DatePalm.parameters(), wood)
        .err()
        .expect("refused");
    assert!(drawn.to_string().contains("coarse"), "{drawn}");
}
