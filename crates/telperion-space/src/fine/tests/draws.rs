//! Threshold-adjacent draws (FINE-CONTRACT.md D1 to D4): a draw exactly at
//! a probability's bound, and one bit either side, decide as the contract
//! says, in the lineage's decisions and in a fine root's first growth
//! unit.
use super::{at, request, same_at_cuts};
use crate::beech;
use crate::bounded::Expected;
use crate::grow::alone;
use crate::lineage::{bud, nodes, threshold, Draw, Key, ONE, VIABILITY};
use crate::species::NodeLaw;

fn real(bits: u64) -> f64 {
    bits as f64 / ONE as f64
}

#[test]
fn a_draw_is_under_a_probability_exactly_below_its_threshold() {
    for d in [0, 1, 12_345, ONE / 3, ONE / 2, ONE - 2, ONE - 1] {
        let draw = Draw(d);
        // At its own real value it is not under; one bit above, it is.
        assert!(!draw.under(real(d)));
        assert!(draw.under(real(d + 1)));
        // The real comparison and the integer one agree either side.
        for p in [real(d), real(d + 1), f64::from_bits(real(d).to_bits() + 1)] {
            assert_eq!(draw.under(p), draw.unit() < p, "{d} under {p}");
        }
    }
    assert_eq!(threshold(0.0), 0);
    assert_eq!(threshold(-1.0), 0);
    assert_eq!(threshold(f64::NAN), 0);
    assert_eq!(threshold(1.0), ONE);
    assert!(!Draw(ONE - 1).under(real(ONE - 1)));
}

#[test]
fn a_bud_at_its_band_s_bound_takes_the_next_band() {
    let lateral = [0.25, 0.25];
    let quarter = ONE / 4;
    assert_eq!(bud(&lateral, Draw(quarter - 1)).map(|b| b.0), Some(0));
    assert_eq!(bud(&lateral, Draw(quarter)).map(|b| b.0), Some(1));
    assert_eq!(bud(&lateral, Draw(ONE / 2 - 1)).map(|b| b.0), Some(1));
    assert_eq!(bud(&lateral, Draw(ONE / 2)).map(|b| b.0), None);
}

#[test]
fn a_node_count_at_its_law_s_bound_counts_the_node() {
    let mut leads = Vec::new();
    let law = NodeLaw::Uniform { min: 0.0, max: 1.0 };
    nodes(law, Draw(ONE / 2 - 1), 8, &mut leads);
    assert_eq!(leads.len(), 0);
    nodes(law, Draw(ONE / 2), 8, &mut leads);
    assert_eq!(leads.len(), 1);
    // The Poisson law's first bound, P(N = 0) = e^-mean.
    let law = NodeLaw::Poisson { mean: 1.7 };
    let first = threshold((-1.7f64).exp());
    nodes(law, Draw(first - 1), 8, &mut leads);
    assert_eq!(leads.len(), 0);
    nodes(law, Draw(first), 8, &mut leads);
    assert_eq!(leads.len(), 1);
}

/// A fine root's first growth unit, its PA's viability set exactly at its
/// survival draw, fails; one bit above, it grows. Either way the tree is
/// the same at every cut.
#[test]
fn a_fine_root_s_first_unit_at_its_viability_draw() {
    let species = beech();
    let request = request(20, 1);
    let expected = Expected::new(&species, 20, 0);
    let fine = at(&species, request, &expected, 1024.0);
    // A root borne by a younger PA, whose own PA no ancestor holds, so
    // setting it moves nothing above it.
    let (root, ..) = fine
        .roots
        .iter()
        .find(|(r, ..)| !r.woken() && r.record.pa > r.attachment.pa + 1)
        .expect("a root of its own PA");
    let (pa, birth) = (root.record.pa, root.record.birth);
    let unit = Key(root.record.lineage).child(u64::from(birth + 1));
    let d = unit.child(VIABILITY).draw().0;
    for (bits, grows) in [(d, false), (d + 1, true)] {
        let mut at_draw = species.clone();
        at_draw.states[pa].viability = real(bits);
        let rules = alone::rules(&at_draw, request.age, request.budget).expect("rules");
        let chain = alone::grow(&rules, request.seed, Some(&root.bud)).expect("grows");
        let first = &chain.axes[chain.root];
        let grew = first.phytomers.iter().any(|p| p.cycle == birth + 1);
        assert_eq!(grew, grows, "viability at {bits} against the draw {d}");
        same_at_cuts(&at_draw, request, &[0.0, 256.0, f64::INFINITY]);
    }
}
