//! The fine roots under realized mechanics against today's whole-tree
//! engine (`realized.rs`), the decision contract's draws (`draws.rs`) and
//! ticks (`ticks.rs`), and the landing rule (`landing.rs`).
use super::{digest, grow_roots, Plan};
use crate::grow::{whole, Request};
use crate::light::Light;
use crate::species::Species;

mod draws;
mod landing;
mod realized;
mod ticks;

pub(super) fn request(age: u32, seed: u64) -> Request {
    Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light::NEUTRAL,
    }
}

/// The tree at each of `cuts` is today's whole-tree engine's.
pub(super) fn same_at_cuts(species: &Species, request: Request, cuts: &[f64]) {
    let whole = digest(&whole(species, request).expect("grows whole"));
    for &cut in cuts {
        let plan = Plan {
            cut,
            ..Plan::default()
        };
        let fine = grow_roots(species, request, plan).expect("grows fine");
        assert_eq!(digest(&fine.structure), whole, "at cut {cut}");
    }
}

/// Without the as-grown exchange (host decision on fn-211 task 12, the
/// GPU executor's named gap) an executor refuses by name a species that
/// keeps its tree as grown, never settles it another way.
#[test]
fn an_executor_without_the_exchange_refuses_a_tree_kept_as_grown() {
    use super::expansion::{settle, Lived, Loaded, RootSeed, Roots};
    use crate::structure::Structure;
    /// The CPU executor's four phases and nothing more.
    struct Plain<'s, 'r>(super::Cpu<'s, 'r>);
    impl Roots for Plain<'_, '_> {
        fn grow(&mut self, seeds: &[RootSeed]) -> crate::Result<(Vec<Lived>, u64)> {
            self.0.grow(seeds)
        }
        fn girth(&mut self, live: &[usize], scales: &[(f64, f64)]) -> Vec<f64> {
            self.0.girth(live, scales)
        }
        fn lay(&mut self, t: &Structure, l: &[usize], b: &[usize]) -> crate::Result<Vec<Loaded>> {
            self.0.lay(t, l, b)
        }
        fn bend(&mut self, t: &Structure, l: &[usize], b: &[usize]) -> crate::Result<()> {
            self.0.bend(t, l, b)
        }
    }
    let mut species = crate::beech();
    species.states.iter_mut().for_each(|s| s.retained = 0.5);
    let request = request(20, 1);
    let rules = crate::grow::alone::rules(&species, 20, request.budget).expect("rules");
    let cpu = super::Cpu {
        species: &species,
        rules: Some(&rules),
        plan: Plan::default(),
        threads: 1,
        roots: Vec::new(),
    };
    let refused = settle(&species, request, Plan::default(), &mut Plain(cpu));
    assert!(
        matches!(refused, Err(crate::Error::Refused { .. })),
        "{:?}",
        refused.err()
    );
}
