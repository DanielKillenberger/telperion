//! fn-332: the trunk's path from the ground to the crown, read off the one
//! cycle's grown structure.
//!
//!   cargo run --profile ci -p telperion-core --example trunk -- oregon-white-oak 200 1
//!
//! The trunk is the seed axis and, at each step, the axis that carries it on
//! (a continuation or a relay) with the thickest base. Prints the path every
//! metre of its length (length, height, and distance out from the base), how
//! each step was carried on, the highest live tip, and how far the crown's
//! mean sits from the base. The seed stands at the origin, growing up (+z).
//! `TRUNK_SAG`, `TRUNK_WANDER` and `TRUNK_TROPISM` scale every PA's form
//! setting of that name, to isolate a cause (1 when unset).
use telperion_core::presets::Preset;
use telperion_space::{grow_census, Origin, Request, Structure, Vec3};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let preset = Preset::from_id(&args[0]).expect("a shipped preset");
    let family = preset.parameters();
    let mut species = family.space.species.expect("an engine species");
    let factor = |name: &str| std::env::var(name).map_or(1.0, |v| v.parse().expect("a factor"));
    for state in &mut species.states {
        state.form.sag *= factor("TRUNK_SAG");
        state.form.wander *= factor("TRUNK_WANDER");
        state.form.tropism *= factor("TRUNK_TROPISM");
    }
    let request = Request {
        age: args[1].parse().expect("an age"),
        seed: args[2].parse().expect("a seed"),
        light: family.space.light,
    };
    let grown = grow_census(&species, request).expect("grown");
    report(&grown.structure, &args);
}

/// The axis that carries `i` on, the thickest at its base; none at the top.
fn next(s: &Structure, carriers: &[Vec<usize>], i: usize) -> Option<usize> {
    let base = |j: usize| s.axes[j].phytomers.first().map_or(0.0, |p| p.radius);
    carriers[i]
        .iter()
        .copied()
        .filter(|&j| !s.axes[j].phytomers.is_empty())
        .max_by(|&a, &b| base(a).total_cmp(&base(b)))
}

fn report(s: &Structure, args: &[String]) {
    let mut carriers = vec![Vec::new(); s.axes.len()];
    for (j, axis) in s.axes.iter().enumerate() {
        if let Origin::Continuation { parent } | Origin::Relay { parent, .. } = axis.origin {
            carriers[parent].push(j);
        }
    }
    let (mut path, mut steps) = (Vec::new(), Vec::new());
    let mut at = Some(0);
    while let Some(i) = at {
        path.extend(s.axes[i].phytomers.iter().map(|p| p.tip));
        at = next(s, &carriers, i);
        if let Some(j) = at {
            let rise = s.axes[j].heading.z.asin().to_degrees();
            steps.push(format!(
                "{}{}@{rise:.0}",
                if matches!(s.axes[j].origin, Origin::Relay { .. }) { 'R' } else { 'C' },
                s.axes[j].pa
            ));
        }
    }
    println!("TRUNK {} age {} seed {}", args[0], args[1], args[2]);
    let (mut length, mut last, mut mark) = (0.0, Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 0.0);
    for p in &path {
        length += distance(last, *p);
        last = *p;
        if length >= mark {
            println!("  at {length:6.1} m: height {:6.2} out {:6.2}", p.z, (p.x * p.x + p.y * p.y).sqrt());
            mark = length.floor() + 1.0;
        }
    }
    let relays = steps.iter().filter(|s| s.starts_with('R')).count();
    println!(
        "  carried on (kind, PA, rise in degrees): {} ({} continuations, {relays} relays)",
        steps.join(" "),
        steps.len() - relays
    );
    let tips: Vec<Vec3> = s.axes.iter().flat_map(|a| a.phytomers.iter().map(|p| p.tip)).collect();
    let top = tips.iter().map(|p| p.z).fold(0.0, f64::max);
    let n = tips.len().max(1) as f64;
    let (cx, cy) = tips.iter().fold((0.0, 0.0), |(x, y), p| (x + p.x / n, y + p.y / n));
    println!(
        "  top {top:.2} m; crown mean {:.2} m out from the base; trunk path ends {:.2} high, {:.2} out",
        (cx * cx + cy * cy).sqrt(),
        last.z,
        (last.x * last.x + last.y * last.y).sqrt()
    );
}

fn distance(a: Vec3, b: Vec3) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
}
