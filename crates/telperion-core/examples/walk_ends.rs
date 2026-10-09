//! The engine's walk at its ends (fn-231 R1, R5): for every pair of the
//! passed branching species and the ordinary tree, at 40 and 80, the raw
//! walk's (`telperion_space::blend`, before fn-230's size factor)
//! light-neutral predicted kept and grown phytomers, its expected metres of
//! wood (phytomers weighted by their size), and the time the three take,
//! at each end and just off it; then every row of the walk that moves
//! off an end by more than its share.
//!
//!   cargo run --profile ci -p telperion-core --example walk_ends -- [40|80]
use std::time::Instant;
use telperion_core::presets::Preset;
use telperion_space::{
    blend, expected_counts, expected_kept, expected_length, settings, zone_settings, Species,
};

const SPECIES: [(Preset, &str); 4] = [
    (Preset::EuropeanBeech, "beech"),
    (Preset::OregonWhiteOak, "oak"),
    (Preset::NorwaySpruce, "spruce"),
    (Preset::Ordinary, "ordinary"),
];
const TS: [f64; 8] = [0.0, 1e-6, 1e-3, 0.01, 0.99, 1.0 - 1e-3, 1.0 - 1e-6, 1.0];

fn neutral(species: &Species) -> Species {
    let mut neutral = species.clone();
    for s in &mut neutral.states {
        (s.shade_hazard, s.shade_size, s.balance_hazard, s.leaf_girth) = (0.0, 0.0, 0.0, 0.0);
    }
    neutral
}

/// Kept, grown and metres of wood, and the seconds they took.
fn measure(species: &Species, age: u32) -> [f64; 4] {
    let s = neutral(species);
    let start = Instant::now();
    let kept = expected_kept(&s, age).unwrap().unwrap();
    let counts = expected_counts(&s, age).unwrap();
    let grown = (0..s.states.len()).map(|pa| counts.total(pa)).sum();
    let length = expected_length(&s, age).unwrap();
    [kept, grown, length, start.elapsed().as_secs_f64()]
}

/// An age, its zone's role (0 for the age's own rows), the row's place
/// among them, its name and value.
type Row = (usize, usize, usize, &'static str, f64);

/// Every named row of every age, each zone's under its role, in order;
/// a role an age leaves out is no rows.
fn rows(species: &Species) -> Vec<Row> {
    let mut out = Vec::new();
    for (k, state) in species.states.iter().enumerate() {
        let mut s = state.clone();
        let own = settings(&mut s).into_iter().enumerate();
        out.extend(own.map(|(i, (n, v))| (k, 0, i, n, *v)));
        for (r, zone) in s.zones.iter_mut().enumerate() {
            let named = zone_settings(zone)
                .unwrap_or_default()
                .into_iter()
                .enumerate();
            out.extend(named.map(|(i, (n, v))| (k, r + 1, i, n, *v)));
        }
    }
    out
}

/// The rows that move from the end by more than a hundredth of what they
/// move a thousand times further in (off the end they jump), the ages the
/// walk starts living in, and the end's counts with those ages left
/// unlived: the guess checked.
fn jumps(end: &Species, near: &Species, far: &Species, age: u32) {
    let key = |r: &Row| (r.0, r.1, r.2);
    let (near_rows, far_rows) = (rows(near), rows(far));
    for e in rows(end) {
        let find = |rs: &[Row]| rs.iter().find(|r| key(r) == key(&e)).map(|r| r.4);
        let (Some(n), Some(f)) = (find(&near_rows), find(&far_rows)) else {
            continue;
        };
        let (dn, df) = ((n - e.4).abs(), (f - e.4).abs());
        if dn > 0.0 && dn > 0.01 * df {
            println!(
                "    jump age {} zone {} {}[{}]: end {} near {n} far {f}",
                e.0, e.1, e.3, e.2, e.4
            );
        }
    }
    let mut restored = near.clone();
    for (k, state) in restored.states.iter_mut().enumerate() {
        let back = |r: usize, i: usize, v: &mut f64| {
            let e = rows(end).into_iter().find(|e| (e.0, e.1, e.2) == (k, r, i));
            let n = near_rows.iter().find(|n| (n.0, n.1, n.2) == (k, r, i));
            let far = far_rows.iter().find(|f| (f.0, f.1, f.2) == (k, r, i));
            if let (Some(e), Some(n), Some(f)) = (e, n, far) {
                let (dn, df) = ((n.4 - e.4).abs(), (f.4 - e.4).abs());
                if dn > 0.0 && dn > 0.01 * df {
                    *v = e.4;
                }
            }
        };
        for (i, (_, v)) in settings(state).into_iter().enumerate() {
            back(0, i, v);
        }
        for (r, zone) in state.zones.iter_mut().enumerate() {
            for (i, (_, v)) in zone_settings(zone)
                .unwrap_or_default()
                .into_iter()
                .enumerate()
            {
                back(r + 1, i, v);
            }
        }
    }
    let [kept, grown, length, _] = measure(&restored, age);
    println!("    those rows at the end's: kept {kept:.0} grown {grown:.0} wood {length:.1} m");
    let mut unlived = near.clone();
    for (k, (e, n)) in end.states.iter().zip(&mut unlived.states).enumerate() {
        if e.lifespan == 0.0 && n.lifespan > 0.0 {
            println!("    lived age {k}: lifespan 0 to {}", n.lifespan);
            (n.lifespan, n.continuation) = (0.0, 1.0);
        }
    }
    let [kept, grown, length, _] = measure(&unlived, age);
    println!("    those ages unlived: kept {kept:.0} grown {grown:.0} wood {length:.1} m");
}

fn main() {
    let ages: Vec<u32> = match std::env::args().nth(1) {
        Some(a) => vec![a.parse().expect("an age")],
        None => vec![40, 80],
    };
    let species: Vec<(Species, &str)> = SPECIES
        .iter()
        .map(|(p, n)| (p.parameters().space.species.expect("engine"), *n))
        .collect();
    for &age in &ages {
        for (i, (a, an)) in species.iter().enumerate() {
            for (b, bn) in &species[i + 1..] {
                println!("{an} to {bn} at {age}");
                for t in TS {
                    match blend(a, b, t) {
                        Ok(walked) => {
                            let [kept, grown, length, secs] = measure(&walked, age);
                            println!("  t {t}: kept {kept:.0} grown {grown:.0} wood {length:.1} m ({secs:.2} s)");
                        }
                        Err(e) => println!("  t {t}: refused, {e}"),
                    }
                }
                for (end, near, far) in [(a, 1e-6, 1e-3), (b, 1.0 - 1e-6, 1.0 - 1e-3)] {
                    match (blend(a, b, near), blend(a, b, far)) {
                        (Ok(n), Ok(f)) => jumps(end, &n, &f, age),
                        _ => println!("    off the end at {near}: refused"),
                    }
                }
            }
        }
    }
}
