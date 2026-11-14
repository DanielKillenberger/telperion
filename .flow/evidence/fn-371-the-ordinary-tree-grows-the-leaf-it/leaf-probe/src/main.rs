//! fn-370 R1: one tree's leaf, as the grower holds it and as the drawing
//! draws it, through the one pipeline (`executor::grow`, its expansion and
//! mesh) and the same species and request grown a second time for the
//! grower's account (`executor::growth`, as the scoreboard's probe does).
//!
//!   leaf-probe <preset> <age|preset> <seed>
//!
//! prints one `LEAF {json}` line.
use telperion_core::foliage::transform_point;
use telperion_core::{pipeline::executor, presets::Preset, Family};

fn family(preset: &str, age: &str, seed: u32) -> Result<Family, String> {
    let mut f = Preset::from_id(preset)
        .ok_or(format!("no preset {preset}"))?
        .parameters();
    if age != "preset" {
        f.age = age.parse::<f64>().map_err(|e| format!("age {age}: {e}"))?;
    }
    f.skeleton.seed = seed;
    Ok(f)
}

/// One-sided area of the element's blade triangles at unit scale (each
/// triangle counted once, the anatomy's subset where it has one).
fn unit_area(e: &telperion_core::foliage::Element) -> f64 {
    let tris: &[u32] = match &e.anatomy {
        Some(a) => &e.indices[a.indices.clone()],
        None => &e.indices,
    };
    tris.chunks_exact(3)
        .map(|t| {
            let p = |k: usize| e.positions[t[k] as usize];
            (p(1) - p(0)).cross(p(2) - p(0)).length() / 2.0
        })
        .sum()
}

fn run(preset: &str, age: &str, seed: u32) -> Result<String, String> {
    let f = family(preset, age, seed)?;
    let age = f.age as u32;
    let applied = f.applied().into_owned();
    // The grower's account.
    let g = executor::growth(&f).map_err(|e| e.to_string())?;
    let s = telperion_space::grow(&g.species, g.request).map_err(|e| e.to_string())?;
    let scale = f.supernatural.scale();
    let grower = telperion_space::floor::bearing_area(&g.species, &s.axes, age) * scale * scale;
    let (mut leafed, mut current, mut phytomers) = (0usize, 0usize, 0usize);
    let mut areas: Vec<f64> = Vec::new();
    for a in &s.axes {
        let st = &g.species.states[a.pa];
        for p in &a.phytomers {
            phytomers += 1;
            if p.cycle > age {
                continue;
            }
            current += usize::from(p.cycle == age);
            let kept = (st.leaf_years - f64::from(age - p.cycle)).clamp(0.0, 1.0);
            if kept > 0.0 && st.leaf_area > 0.0 && p.scale > 0.0 {
                leafed += 1;
                if !areas.contains(&st.leaf_area) {
                    areas.push(st.leaf_area);
                }
            }
        }
    }
    // The short shoots' (an age stating a cluster) leaf, and the allotted
    // wood of every other leafed phytomer (`join::Wood::of`'s bearing:
    // internode x scale x spread factor), in metres.
    let short: Vec<telperion_space::Axis> = (s.axes.iter())
        .filter(|a| g.species.states[a.pa].cluster_leaves > 0.0)
        .cloned()
        .collect();
    let short_m2 = telperion_space::floor::bearing_area(&g.species, &short, age) * scale * scale;
    drop(short);
    let mut leafed_wood = 0.0;
    for a in s.axes.iter() {
        let st = &g.species.states[a.pa];
        if st.cluster_leaves > 0.0 || st.leaf_area <= 0.0 {
            continue;
        }
        for p in a.phytomers.iter().filter(|p| p.cycle <= age) {
            if st.leaf_years - f64::from(age - p.cycle) > 0.0 {
                let f = telperion_space::internode_factor(p.key(), st.form.internode_spread);
                leafed_wood += st.internode * p.scale * f * scale;
            }
        }
    }
    // The grower's leaf on wood the conversion never draws: phytomers
    // thinner than its finest wood (`pipeline/space.rs` FINEST, 0.2 mm
    // drawn), each axis cut where its wood first gets that thin.
    let finest = 2e-4 / scale;
    let thin: Vec<telperion_space::Axis> = s
        .axes
        .iter()
        .map(|a| {
            let cut = a.phytomers.iter().position(|p| p.radius < finest);
            let mut t = a.clone();
            t.phytomers = cut.map_or(Vec::new(), |k| a.phytomers[k..].to_vec());
            t
        })
        .collect();
    let undrawn = telperion_space::floor::bearing_area(&g.species, &thin, age) * scale * scale;
    drop(thin);
    // The grower's structure is let go before the drawing grows its own.
    drop(s);
    // The drawing.
    let grown = executor::grow(&f).map_err(|e| e.to_string())?;
    let x = grown.expansion().map_err(|e| e.to_string())?;
    let (mut run_stations, mut cluster_leaves, mut clusters) = (0u64, 0u64, 0u64);
    if let Some(st) = x.stations().map_err(|e| e.to_string())? {
        for seg in &st.segments {
            match seg.cluster {
                Some(_) => {
                    clusters += 1;
                    cluster_leaves += u64::from(seg.count);
                }
                None => run_stations += u64::from(seg.count),
            }
        }
    }
    let unit = unit_area(x.element());
    let mesh = x.mesh().map_err(|e| e.to_string())?;
    let el = &mesh.foliage.element;
    let tris: &[u32] = match &el.anatomy {
        Some(a) => &el.indices[a.indices.clone()],
        None => &el.indices,
    };
    let inst = &mesh.foliage.instances;
    let mut drawn = 0.0;
    for m in inst.matrices() {
        for t in tris.chunks_exact(3) {
            let p = |k: usize| transform_point(&m, el.positions[t[k] as usize]);
            drawn += (p(1) - p(0)).cross(p(2) - p(0)).length() / 2.0;
        }
    }
    let n = inst.len();
    let c = &applied.canopy;
    let blade = unit * c.size * c.size;
    // fn-371 R1: the mean drawn blade from the rows, as `leaflet.rs` draws
    // it: scale = size x (1 + v x U(-1, 1)), so E[scale^2] = size^2 (1 + v^2/3).
    let v = c.size_variation;
    let mean_blade = blade * (1.0 + v * v / 3.0);
    if std::env::var_os("ROWS").is_some() {
        eprintln!("ELEMENT {:?}", applied.element);
    }
    Ok(format!(
        "{{\"preset\":\"{preset}\",\"age\":{age},\"seed\":{seed},\"scale\":{scale},\
         \"grower_m2\":{grower:.3},\"undrawn_m2\":{undrawn:.3},\"short_m2\":{short_m2:.3},\
         \"leafed_wood_m\":{leafed_wood:.1},\"phytomers\":{phytomers},\"current\":{current},\
         \"leafed\":{leafed},\"leaf_area_cm2\":{:?},\
         \"drawn_m2\":{drawn:.3},\"instances\":{n},\"run_stations\":{run_stations},\
         \"clusters\":{clusters},\"cluster_leaves\":{cluster_leaves},\
         \"unit_cm2\":{:.4},\"size\":{},\"size_variation\":{},\"blade_cm2\":{:.4},\
         \"mean_blade_from_rows_cm2\":{:.4},\"mean_drawn_blade_cm2\":{:.4}}}",
        areas.iter().map(|a| (a * 1e4 * 1e4).round() / 1e4).collect::<Vec<_>>(),
        unit * 1e4,
        c.size,
        c.size_variation,
        blade * 1e4,
        mean_blade * 1e4,
        if n > 0 { drawn / n as f64 * 1e4 } else { 0.0 },
    ))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [preset, age, seed] = &args[..] else {
        eprintln!("leaf-probe <preset> <age|preset> <seed>");
        std::process::exit(2);
    };
    let seed: u32 = seed.parse().expect("seed");
    match run(preset, age, seed) {
        Ok(j) => println!("LEAF {j}"),
        Err(e) => println!("LEAF {{\"preset\":\"{preset}\",\"seed\":{seed},\"refused\":{e:?}}}"),
    }
}
