//! The crown's radius against the `pow` form it replaced (fn-143).
use telperion_core::{
    envelope::Envelope,
    presets::{Preset, CATALOGUE, IN_WORK},
};

/// The largest departure the new evaluation may make from the `pow` form, as
/// a share of the envelope's widest radius. Away from the ends the two agree
/// to about 1e-13. Within a few ulps of the crown base and the top the `pow`
/// form loses `1 - p^shoulder` to cancellation and the new one does not, so
/// that is where they part most: 3.7e-7 of the widest radius at shoulder 3.2.
const BOUND: f64 = 1e-6;

fn pow_form(e: &Envelope, y: f64) -> f64 {
    let base = e.height * e.crown_base;
    let t = (y - base) / (e.height - base);
    if !(t > 0.0 && t < 1.0) {
        return 0.0;
    }
    let f = e.fullness;
    let p = if t < f {
        1.0 - t / f
    } else {
        (t - f) / (1.0 - f)
    };
    let s = e.shoulder;
    e.max_radius() * (1.0 - p.powf(s)).max(0.0).powf(1.0 / s)
}

/// Heights over the whole tree, then the last few representable heights on
/// either side of the crown base, the widest height and the top.
fn heights(e: &Envelope) -> impl Iterator<Item = f64> + '_ {
    const STEPS: u32 = 200_000;
    let base = e.height * e.crown_base;
    let widest = base + (e.height - base) * e.fullness;
    let even = (0..=STEPS).map(move |i| e.height * f64::from(i) / f64::from(STEPS));
    let edges = [base, widest, e.height]
        .into_iter()
        .flat_map(|y| (-64_i64..=64).map(move |k| f64::from_bits((y.to_bits() as i64 + k) as u64)));
    even.chain(edges)
}

#[test]
fn radius_stays_within_its_stated_bound_of_the_pow_form() {
    let mut worst = 0.0_f64;
    for &(_, id, _, _) in CATALOGUE.iter().chain(IN_WORK) {
        let e = Preset::from_id(id).unwrap().parameters().skeleton.envelope;
        for y in heights(&e) {
            let (new, old) = (e.radius_at(y), pow_form(&e, y));
            if e.shoulder == 1.0 {
                assert_eq!(
                    new.to_bits(),
                    old.to_bits(),
                    "{id} at {y}: a straight shoulder moved"
                );
            }
            let error = (new - old).abs() / e.max_radius();
            assert!(
                error <= BOUND,
                "{id} at {y}: {error:e} of the widest radius"
            );
            worst = worst.max(error);
        }
    }
    println!("largest departure: {worst:e} of the widest radius");
}
