//! The grown phytomers as one running total (fn-238): `expected_counts`'
//! recursion on a bud's first unit and the rest of its axis, the bud one
//! cycle younger, as the wood's is (fn-209, P1), so each substructure is a
//! number and not a table by PA and cycle. The same expectation as the
//! table's sum, added in another order.
use super::events::{first_events, Event};
use super::{steps, sub, Walk};
use crate::dormant::First;
use crate::schedule::Carried;

impl Walk<'_, f64> {
    /// The expected phytomers a bud grows: a bud of an age passed through
    /// grows in the next it lives in, counted whole.
    pub(super) fn grown(
        &mut self,
        k: usize,
        m: usize,
        carried: Carried,
        first: Option<First>,
    ) -> f64 {
        match self.species.lived(k) {
            Some((k, _)) => self.lived_grown(k, m, carried, first),
            None => 0.0,
        }
    }

    fn lived_grown(&mut self, k: usize, m: usize, carried: Carried, first: Option<First>) -> f64 {
        if m == 0 {
            return 0.0;
        }
        let state = &self.species.states[k];
        let carried = carried.canonical(state.life(), state.rises(), m);
        let key = sub(self.species, k, m, carried, first);
        if let Some(&total) = self.memo.get(&key) {
            return total;
        }
        let life = state.life();
        let (events, on) = first_events(
            self.species,
            &self.laterals[k],
            &mut self.living,
            (k, m, carried),
            first,
        );
        let mut total = 0.0;
        if on > 0.0 {
            total += on * self.lived_grown(k, m - 1, steps::rest(carried, life), None);
        }
        for event in events {
            total += match event {
                Event::Nodes { count, .. } => count,
                Event::Bud {
                    at,
                    pa,
                    carried,
                    weight,
                    first,
                    ..
                } => weight * self.grown(pa, m - at, carried, first),
            };
        }
        self.memo.insert(key, total);
        total
    }
}
