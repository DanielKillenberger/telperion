//! The point between two species (fn-206): on the shared reference axis a
//! species is a vector of settings, so the species a share `t` of the way
//! from `a` to `b` is every setting a share `t` of the way between theirs,
//! each on its own scale (`species/scale.rs`, host decision 9). Zones are
//! matched by role, a role one species leaves out standing in as a zone of
//! no nodes. Both species are canonical (`Species::canonical`), so an age
//! one of them does not use carries its neighbour's settings.
use crate::error::{refuse, Result};
use crate::species::scale::{scale, settings, zone_settings, OUTCOMES};
use crate::species::{NodeLaw, PaState, Species, Zone, ZONES};

/// The species a share `t` (0 to 1) of the way from `a` to `b`. Both are
/// written on one reference axis, with uniform node counts throughout, as
/// the four species are; anything else is refused.
pub fn blend(a: &Species, b: &Species, t: f64) -> Result<Species> {
    if a.states.len() != b.states.len() {
        return refuse(
            "states",
            "two species walked between share one reference axis",
        );
    }
    if !(0.0..=1.0).contains(&t) {
        return refuse("t", "a share of the way lies in 0 to 1");
    }
    a.validate()?;
    b.validate()?;
    let states = a
        .states
        .iter()
        .zip(&b.states)
        .map(|(x, y)| state(x, y, t))
        .collect::<Result<_>>()?;
    let species = Species { states };
    species.validate()?;
    Ok(species)
}

fn state(x: &PaState, y: &PaState, t: f64) -> Result<PaState> {
    let (mut out, mut other) = (x.clone(), y.clone());
    for ((name, a), (_, b)) in settings(&mut out).into_iter().zip(settings(&mut other)) {
        *a = mixed(name, *a, *b, t)?;
    }
    outcomes(&mut out, x, y, t);
    let roles = |s: &PaState| -> Vec<Zone> {
        let width = s.zones[0].lateral.len();
        (0..ZONES)
            .map(|r| s.zones.get(r).cloned().unwrap_or_else(|| empty(width)))
            .collect()
    };
    let (mut zones, mut theirs) = (roles(x), roles(y));
    for (p, q) in zones.iter_mut().zip(&mut theirs) {
        let (Some(ps), Some(qs)) = (zone_settings(p), zone_settings(q)) else {
            return refuse(
                "zones.nodes",
                "a walk between species holds uniform node counts",
            );
        };
        for ((name, a), (_, b)) in ps.into_iter().zip(qs) {
            *a = mixed(name, *a, *b, t)?;
        }
    }
    out.zones = zones;
    Ok(out)
}

/// Each stop's three outcomes, carrying on, relaying and dying, mixed as
/// shares (host decision 12): the relay's share of a stop is the mixed
/// relaying share over the mixed stopping one, so a midpoint between two
/// species whose apexes never die never dies either. The chance it stops
/// mixes by value with the settings.
fn outcomes(out: &mut PaState, x: &PaState, y: &PaState, t: f64) {
    if t <= 0.0 || t >= 1.0 {
        return;
    }
    let read = |s: &PaState, name: &str| -> f64 {
        let mut s = s.clone();
        let found = settings(&mut s)
            .into_iter()
            .find(|(n, _)| *n == name)
            .map(|(_, v)| *v);
        found.expect("an outcome names its settings")
    };
    for &(stop, relay) in OUTCOMES {
        // Abortion is the chance it stops; the others, that it carries on.
        let stops = |s: &PaState| {
            let p = read(s, stop);
            if stop == "abortion" {
                p
            } else {
                1.0 - p
            }
        };
        let (sa, sb) = (stops(x), stops(y));
        let (ra, rb) = (read(x, relay), read(y, relay));
        let relays = sa * ra + (sb * rb - sa * ra) * t;
        let stopped = sa + (sb - sa) * t;
        let share = if stopped > 0.0 {
            (relays / stopped).clamp(0.0, 1.0)
        } else {
            ra + (rb - ra) * t
        };
        for (n, v) in settings(out) {
            if n == relay {
                *v = share;
            }
        }
    }
}

/// A setting a share `t` of the way from `a` to `b` on its scale; a setting
/// the table gives no scale is refused.
fn mixed(name: &str, a: f64, b: f64, t: f64) -> Result<f64> {
    match scale(name) {
        Some(scale) => Ok(scale.mix(a, b, t)),
        None => refuse(name, "a setting walked between species declares its scale"),
    }
}

/// A role a growth unit leaves out: no nodes.
fn empty(width: usize) -> Zone {
    Zone {
        nodes: NodeLaw::Uniform { min: 0.0, max: 0.0 },
        buds: 1.0,
        lateral: vec![0.0; width],
        dormant: vec![0.0; width],
        delay: 0.0,
        rate: 0.0,
    }
}
