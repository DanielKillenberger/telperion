//! fn-281 R2: where a relay's join leaves no wood above it (`Join::above`
//! zero), what the stopped module's head carried, in f64, against the
//! relay's share there in the f32 sum `thicken_given` takes it from. The
//! oak at 80, seed 7, under the site's light, as the preset grows it.
//! About 4 GB and 25 s: `cargo test --profile ci -p telperion-space
//! above_tests -- --ignored --nocapture`.
use super::*;
use crate::grow::{grow, Request};
use crate::light::Light;

#[test]
#[ignore = "fn-281 R2 diagnostic; about 4 GB and 25 s"]
fn what_a_join_with_no_wood_above_carried() {
    let species = crate::oak();
    let light = Light {
        extinction: 0.5,
        sky: 0.5,
    };
    let request = Request {
        age: 80,
        seed: 7,
        budget: 20_000_000,
        light,
    };
    let s = grow(&species, request).unwrap();
    let mean = leaf_mean(&s, &species);
    let (mut zero, mut dark, mut absorbed) = (0, 0, 0);
    for (c, relay) in s.axes.iter().enumerate() {
        let (Origin::Relay { parent, .. }, Some(j)) = (relay.origin, relay.joined) else {
            continue;
        };
        let p = &s.axes[parent];
        if relay.phytomers.is_empty() || j.above > 0.0 || p.phytomers[j.node].radius < 2e-4 {
            continue;
        }
        zero += 1;
        let state = &species.states[p.pa];
        let e = state.form.exponent;
        // The head's own wood in f64, phytomer `node` up, and its laterals'.
        let head = &p.phytomers[j.node..];
        let own: f64 = head
            .iter()
            .map(|q| {
                let leaf = leaf(q.light, state.leaf_girth, mean);
                own(&state.form, q, s.age, q.scale, leaf).powf(e)
            })
            .sum();
        let laterals: f64 = s
            .axes
            .iter()
            .filter(|a| matches!(a.origin, Origin::Lateral { parent: q, node, .. } if q == parent && node >= j.node))
            .filter_map(|a| a.phytomers.first())
            .map(|f| f.radius.powf(e))
            .sum();
        let at = f64::from(section(relay.phytomers[0].radius, e) * j.t);
        let darkest = head.iter().map(|q| q.light).fold(f64::INFINITY, f64::min);
        dark += usize::from(head.iter().all(|q| lit(q.light, state.leaf_girth) == 0.0));
        let lost = ((at as f32 + (own + laterals) as f32) - at as f32) == 0.0;
        absorbed += usize::from(lost && own + laterals > 0.0);
        println!(
            "relay {c} on {parent} node {} of {} t {}: head own {own:e} laterals {laterals:e} \
             relay share {at:e}, ratio {:e}; darkest light {darkest:e}; absorbed in f32 {lost}",
            j.node,
            p.phytomers.len(),
            j.t,
            (own + laterals) / at,
        );
    }
    println!("{zero} joins leave no wood above a kept phytomer; {dark} with every head leaf term zero, {absorbed} absorbed with wood above");
    assert_eq!(zero, 0);
}
