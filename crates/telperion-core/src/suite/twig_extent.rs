//! fn-183: the twig layer grows without the crown as a wall. The direct
//! build's twigs run to the length their own rules give them, so the twig
//! layer asks the crown's outline nothing, on every shipped preset.
use telperion_core::{
    envelope::queries::{self, Purpose},
    params, pipeline,
};

/// The purposes only the twig layer asks the outline for.
const TWIG_LAYER: [Purpose; 4] = [
    Purpose::TwigStride,
    Purpose::TwigBisection,
    Purpose::TerminalAdmission,
    Purpose::CurtainBand,
];

#[test]
fn the_twig_layer_asks_the_crown_nothing_and_builds_one_tree() {
    for entry in params::CATALOGUE {
        let id = entry.1;
        let family = params::by_identity(id).unwrap();
        let _ = queries::take();
        let first = pipeline::build(&family, pipeline::Request::default())
            .unwrap_or_else(|e| panic!("{id}: {e}"));
        let counts = queries::take().expect("the crate's tests count queries");
        for purpose in TWIG_LAYER {
            assert_eq!(
                counts.queries[purpose as usize],
                0,
                "{id}: the twig layer asked the crown for {}",
                purpose.name()
            );
        }
        let again = pipeline::build(&family, pipeline::Request::default()).unwrap();
        assert!(
            first.skeleton.tree == again.skeleton.tree,
            "{id}: two builds at one revision grew two trees"
        );
        println!("{id}: {} axes planned", counts.planned_axes);
    }
}
