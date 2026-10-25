//! fn-198 task 1: how many scalars an engine species holds, how many its
//! lived ages set off the neutral passed-through age, and non-finite ones.
use telperion_space::{NodeLaw, PaState, Species};
fn flat(s: &PaState) -> Vec<(String, f64)> {
    let f = &s.form;
    let mut v: Vec<(String, f64)> = vec![
        ("lifespan", s.lifespan), ("continuation", s.continuation), ("viability", s.viability),
        ("shedding", s.shedding), ("internode", s.internode), ("insertion", s.insertion),
        ("divergence", s.divergence), ("abortion", s.abortion), ("abortion_rise", s.abortion_rise),
        ("relay", s.relay), ("relay_ended", s.relay_ended), ("relay_failed", s.relay_failed),
        ("relay_at", s.relay_at), ("epitony", s.epitony), ("erection", s.erection),
        ("readiness", s.readiness), ("rhythm", s.rhythm), ("leaf_area", s.leaf_area),
        ("shade_hazard", s.shade_hazard), ("shade_size", s.shade_size), ("apical_control", s.apical_control),
        ("upkeep", s.upkeep), ("balance_hazard", s.balance_hazard), ("tolerance", s.tolerance),
        ("retained", s.retained), ("leaf_girth", s.leaf_girth), ("straightening", s.straightening),
        ("form.tropism", f.tropism), ("form.elevation", f.elevation), ("form.wander", f.wander),
        ("form.plane", f.plane), ("form.pipe", f.pipe), ("form.exponent", f.exponent),
        ("form.ripening", f.ripening), ("form.dominance", f.dominance), ("form.roll", f.roll),
        ("form.sag", f.sag), ("form.secondary", f.secondary), ("form.bend_length", f.bend_length),
    ].into_iter().map(|(n, x)| (n.to_string(), x)).collect();
    for (z, zone) in s.zones.iter().enumerate() {
        let (lo, hi, kind) = match zone.nodes { NodeLaw::Uniform { min, max } => (min, max, 0.0), NodeLaw::Poisson { mean } => (mean, mean, 1.0) };
        v.push((format!("zones.{z}.nodes.law"), kind));
        v.push((format!("zones.{z}.nodes.min"), lo));
        v.push((format!("zones.{z}.nodes.max"), hi));
        v.push((format!("zones.{z}.buds"), zone.buds));
        v.push((format!("zones.{z}.delay"), zone.delay));
        v.push((format!("zones.{z}.rate"), zone.rate));
        for (j, p) in zone.lateral.iter().enumerate() { v.push((format!("zones.{z}.lateral.{j}"), *p)); }
        for (j, p) in zone.dormant.iter().enumerate() { v.push((format!("zones.{z}.dormant.{j}"), *p)); }
    }
    v
}
fn main() {
    let palm = telperion_space::palm();
    for (name, s) in [("palm", palm.clone()), ("beech", telperion_space::beech()), ("oak", telperion_space::oak()), ("spruce", telperion_space::spruce())] {
        let s: Species = s;
        let lived: Vec<usize> = (0..s.states.len()).filter(|&k| s.states[k].lifespan > 0.0).collect();
        let total: usize = s.states.iter().map(|p| flat(p).len()).sum();
        let lived_total: usize = lived.iter().map(|&k| flat(&s.states[k]).len()).sum();
        let lived_nonzero: usize = lived.iter().map(|&k| flat(&s.states[k]).iter().filter(|(_, x)| *x != 0.0).count()).sum();
        let nonfinite: Vec<String> = s.states.iter().enumerate().flat_map(|(k, p)| flat(p).into_iter().filter(|(_, x)| !x.is_finite()).map(move |(n, _)| format!("{k}.{n}"))).collect();
        let zones: Vec<usize> = s.states.iter().map(|p| p.zones.len()).collect();
        let poisson = s.states.iter().flat_map(|p| p.zones.iter()).filter(|z| matches!(z.nodes, NodeLaw::Poisson { .. })).count();
        println!("{name}: ages {} lived {:?}; scalars all ages {total}; lived ages {lived_total}, of them non-zero {lived_nonzero}; zones per age {zones:?}; poisson zones {poisson}; non-finite {} ({:?}...)", s.states.len(), lived, nonfinite.len(), nonfinite.iter().take(3).collect::<Vec<_>>());
    }
}
