//! The fine roots under realized mechanics against today's whole-tree
//! engine (`realized.rs`), the decision contract's draws (`draws.rs`) and
//! ticks (`ticks.rs`), and the landing rule (`landing.rs`).
use super::{digest, grow_roots, Plan};
use crate::grow::{whole, Request};
use crate::light::Light;
use crate::species::Species;

mod coarse;
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
    use super::step::{now, Step};
    use crate::structure::Structure;
    type Done<'a, T> = Step<'a, crate::Result<T>>;
    /// The CPU executor's four phases and nothing more.
    struct Plain<'s, 'r>(super::Cpu<'s, 'r>);
    impl Roots for Plain<'_, '_> {
        fn grow<'a>(&'a mut self, seeds: &'a [RootSeed]) -> Done<'a, (Vec<Lived>, u64)> {
            self.0.grow(seeds)
        }
        fn girth<'a>(&'a mut self, live: &'a [usize], s: &'a [(f64, f64)]) -> Done<'a, Vec<f64>> {
            self.0.girth(live, s)
        }
        fn lay<'a>(
            &'a mut self,
            t: &'a Structure,
            l: &'a [usize],
            b: &'a [usize],
        ) -> Done<'a, Vec<Loaded>> {
            self.0.lay(t, l, b)
        }
        fn bend<'a>(
            &'a mut self,
            t: &'a Structure,
            l: &'a [usize],
            b: &'a [usize],
        ) -> Done<'a, ()> {
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
    let refused = now(settle(&species, request, Plan::default(), &mut Plain(cpu)));
    assert!(
        matches!(refused, Err(crate::Error::Refused { .. })),
        "{:?}",
        refused.err()
    );
}
