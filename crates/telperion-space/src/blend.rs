//! The point between two species (fn-206): on the shared reference axis a
//! species is a vector of settings, so the species a share `t` of the way
//! from `a` to `b` is every setting a share `t` of the way between theirs,
//! each on its own scale (`species/scale.rs`, host decision 9). Zones are
//! matched by role, a role one species leaves out standing in as a zone of
//! no nodes. Both species are canonical (`Species::canonical`), so an age
//! one of them does not use carries its neighbour's settings. Between the
//! ends, for a tree of a given age, each end is read as it lives (fn-308
//! R2): an age one of them never reaches takes the other's every row
//! (`reaching`), each lifespan past the tree's age is clamped to it, and an
//! age one end leaves within the tree's age and the other does not takes
//! the leaving end's exit rows (`ending`).
use crate::error::{refuse, Result};
use crate::species::scale::{scale, settings, zone_settings, OUTCOMES};
use crate::species::{NodeLaw, PaState, Species, Zone, ZONES};

/// The species a share `t` (0 to 1) of the way from `a` to `b`, for a tree
/// `age` cycles old: the whole age the grower grows it to. Both are
/// written on one reference axis, with uniform node counts throughout, as
/// the four species are; anything else is refused. At either end the
/// species is that end, whatever the age.
pub fn blend(a: &Species, b: &Species, t: f64, age: u32) -> Result<Species> {
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
    let (a, b) = if t > 0.0 && t < 1.0 {
        ending(reaching(a, b), f64::from(age))
    } else {
        (a.clone(), b.clone())
    };
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

/// Both ends with each age one of them never reaches given the other's
/// every row (fn-299, fn-308 rule B, host 2026-10-22). A species' rows at
/// an age no bud of it is ever in are canonical placeholders (`lifespan`
/// 0, `continuation` 1, no laterals), absence and not settings: mixed,
/// they damped every branching order the other end grows there. Kept, the
/// age lives as the end that reaches it lives it, and the walk fades how
/// often a bud enters it, through the continuations and laterals that lead
/// there. Reached here is entered at all, at any age of the tree.
fn reaching(a: &Species, b: &Species) -> (Species, Species) {
    let (ra, rb) = (reached(a), reached(b));
    let (mut x, mut y) = (a.clone(), b.clone());
    for k in 0..a.states.len() {
        match (ra[k], rb[k]) {
            (false, true) => x.states[k] = b.states[k].clone(),
            (true, false) => y.states[k] = a.states[k].clone(),
            _ => {}
        }
    }
    (x, y)
}

/// Both ends as a tree `age` cycles old lives them (fn-308 R2). Each
/// lifespan past the age is clamped to it, since a lifespan at or past the
/// tree's age is dormant. Then, at an age one end leaves within the tree's
/// age (`leaves`) and the other does not, the other takes the leaving
/// end's exit rows: whether a bud moves on, and whether a relay replaces
/// it when it does not. The staying end never draws its own within the
/// age, so mixed in they would switch the frame's exit by seed. Where both
/// ends leave, or neither, the rows mix.
fn ending((mut a, mut b): (Species, Species), age: f64) -> (Species, Species) {
    // `age` is the number of cycles the build grows: a family's age is a
    // whole count (`Blend::Count`) and the build grows exactly that age
    // (host, 2026-10-22). The clamp is the smallest valid lifespan at or
    // past it: a lifespan is a whole cycle at least, the floor
    // `Log { floor: 1 }` mixes it on, so under one cycle no lifespan is
    // clamped to none and no end leaves an age; the exits mix as before.
    let dormant = age.max(1.0);
    for s in a.states.iter_mut().chain(&mut b.states) {
        s.lifespan = s.lifespan.min(dormant);
    }
    for (x, y) in a.states.iter_mut().zip(&mut b.states) {
        match (leaves(x, age), leaves(y, age)) {
            (false, true) => (x.continuation, x.relay_ended) = (y.continuation, y.relay_ended),
            (true, false) => (y.continuation, y.relay_ended) = (x.continuation, x.relay_ended),
            _ => {}
        }
    }
    (a, b)
}

/// Whether a bud living `state` comes to its age's end within a tree `age`
/// cycles old: its lifespan is under the age.
fn leaves(state: &PaState, age: f64) -> bool {
    state.lifespan < age
}

/// The ages a bud of `s` is ever in, entered or passed through: from the
/// seed's bud of the youngest PA, each apex moving on, and each lateral
/// bud the axis draws (`PaState::laterals`) or sleeping bud of the PA it
/// carries. A relay bud keeps its own PA.
fn reached(s: &Species) -> Vec<bool> {
    let mut seen = vec![false; s.states.len()];
    let mut lived = Vec::new();
    enter(s, 0, &mut seen, &mut lived);
    while let Some(a) = lived.pop() {
        let state = &s.states[a];
        if state.continuation > 0.0 {
            enter(s, a + 1, &mut seen, &mut lived);
        }
        // The lateral probabilities as the axis draws them, readiness and
        // rhythm applied; a zone of no nodes holds no bud.
        for (zone, drawn) in state.zones.iter().zip(state.laterals()) {
            if zone.nodes.mean() <= 0.0 {
                continue;
            }
            for (pa, (&p, &d)) in drawn.iter().zip(&zone.dormant).enumerate() {
                if p > 0.0 || d > 0.0 {
                    enter(s, pa, &mut seen, &mut lived);
                }
            }
        }
    }
    seen
}

/// A bud entering age `pa`, passing through each age of no lifespan by its
/// continuation (`Species::lived`), marking each age it is in; the age it
/// lives in, once, joins `lived`.
fn enter(s: &Species, mut pa: usize, seen: &mut [bool], lived: &mut Vec<usize>) {
    while let Some(state) = s.states.get(pa) {
        if seen[pa] {
            return;
        }
        seen[pa] = true;
        if state.lifespan > 0.0 {
            lived.push(pa);
            return;
        }
        if state.continuation <= 0.0 {
            return;
        }
        pa += 1;
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
