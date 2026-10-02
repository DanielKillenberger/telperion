//! fn-183: the twig layer grows without the crown as a wall. The direct
//! build's twigs run to the length their own rules give them, so the twig
//! layer asks the crown's outline nothing, on every shipped preset.
use telperion_core::{
    branching,
    envelope::queries::{self, Purpose},
    params, pipeline,
};

/// The purpose only the twig layer's curtain search asks the outline for.
const TWIG_LAYER: [Purpose; 1] = [Purpose::CurtainBand];

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

/// Where the canopy stands a rosette on every stem apex, no twig layer is
/// grown there to be cleared: nothing past the crossover hangs from an apex.
/// And the apex is no tip: it keeps the girth it had when twigs grew on it.
#[test]
fn an_apex_that_bears_a_rosette_grows_no_twig_and_keeps_its_girth() {
    let family = params::by_identity("date-palm").unwrap();
    assert!(family.canopy.rosette_fronds > 0, "the palm stands no rosette");
    let (skeleton, radii) = (&family.skeleton, family.radii);
    let crowned = branching::crowned(skeleton, radii, true).unwrap().tree;
    let twigged = branching::crowned(skeleton, radii, false).unwrap().tree;
    let apices = crowned.stem_apices();
    assert!(!apices.is_empty());
    for n in &crowned.nodes[crowned.crossover..] {
        let parent = n.parent.unwrap() as usize;
        assert!(!apices.contains(&parent), "a twig grew on a crowned apex");
    }
    assert!(
        twigged.nodes.len() > twigged.crossover,
        "the palm grows a twig layer at its apex when it is not crowned"
    );
    for &apex in &apices {
        assert_eq!(
            crowned.nodes[apex].radius, twigged.nodes[apex].radius,
            "a crowned apex lost its girth"
        );
    }
    let _ = queries::take();
    pipeline::build(&family, pipeline::Request::default()).unwrap();
    let counts = queries::take().unwrap();
    assert_eq!(counts.planned_axes, 0, "the palm planned a twig axis");
}
