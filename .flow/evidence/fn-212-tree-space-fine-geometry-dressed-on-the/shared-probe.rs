//! fn-212 task 1's sharing probe (copied to
//! crates/telperion-render/examples/f212_shared.rs to run, never committed
//! there; CPU only, no GPU). Grows a preset through the pipeline's own
//! stage 2, then times each CPU preparation step the resident path runs
//! on its expansion, and checks whether the compact rings are the curve's
//! points: every ring's twelve float words against the curve point at the
//! same place in the same run order, rounded to float32.
//!
//!   f212_shared <preset> <age> <seed> <rounds>
use std::time::Instant;
use telperion_core::{pipeline::executor, presets::Preset};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let mut family = Preset::from_id(&a[0]).ok_or("no preset")?.parameters();
    family.age = a[1].parse()?;
    family.skeleton.seed = a[2].parse()?;
    let rounds: usize = a[3].parse()?;
    let x = executor::grow(&family)?.expansion()?;
    let nodes = x.tree().nodes.len();
    let seats = x.seats();
    for round in 0..rounds {
        let t = Instant::now();
        let (rings, contact_bytes) = if seats {
            let shared = x.compact_with_contacts()?;
            let bytes = shared.contact_bytes();
            (shared.into_surface(), bytes)
        } else {
            (x.compact()?, 0)
        };
        let rings_ms = ms(t);
        let t = Instant::now();
        let curve = x.curve()?;
        let curve_ms = ms(t);
        let t = Instant::now();
        let stations = x.stations()?.map_or(0, |s| s.count);
        let stations_ms = ms(t);
        // The rings as the curve would give them: each point's twelve
        // words in float32, the twist's phase from its distance along.
        let t = Instant::now();
        let twist = curve.twist_rate;
        let height = curve.height;
        let derived: Vec<[u32; 12]> = curve
            .points
            .iter()
            .map(|p| {
                [
                    p.centre.x, p.centre.y, p.centre.z, p.radius, p.normal.x, p.normal.y,
                    p.normal.z, p.along, p.binormal.x, p.binormal.y, p.binormal.z,
                    std::f64::consts::TAU * twist * (p.along / height),
                ]
                .map(|v| (v as f32).to_bits())
            })
            .collect();
        let derive_ms = ms(t);
        let same_count = derived.len() == rings.rings.len();
        let (mut exact, mut worst) = (0usize, 0f32);
        for (r, d) in rings.rings.iter().zip(&derived) {
            if r[..12] == d[..] {
                exact += 1;
            } else {
                for k in 0..12 {
                    let e = (f32::from_bits(r[k]) - f32::from_bits(d[k])).abs();
                    worst = worst.max(e);
                }
            }
        }
        let runs_same = rings.runs.len() == curve.runs.len()
            && rings
                .runs
                .iter()
                .zip(&curve.runs)
                .all(|(a, b)| a.rings == b.count && a.ring_start == b.first);
        println!(
            "{{\"preset\":\"{}\",\"age\":{},\"seed\":{},\"round\":{round},\"load\":{},\"nodes\":{nodes},\"seats\":{seats},\"rings\":{},\"curve_points\":{},\"runs\":{},\"curve_runs\":{},\"clusters\":{},\"stations\":{stations},\"rings_ms\":{rings_ms:.1},\"curve_ms\":{curve_ms:.1},\"stations_ms\":{stations_ms:.1},\"derive_ms\":{derive_ms:.1},\"contact_bytes\":{contact_bytes},\"vertices\":{},\"indices\":{},\"segments\":{},\"same_count\":{same_count},\"runs_same\":{runs_same},\"rings_exact\":{exact},\"worst_abs\":{worst:e}}}",
            a[0], a[1], a[2], load(), rings.rings.len(), curve.points.len(), rings.runs.len(),
            curve.runs.len(), curve.clusters.len(), rings.vertices, rings.indices, rings.segments,
        );
    }
    Ok(())
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1e3
}
fn load() -> String {
    std::fs::read_to_string("/proc/loadavg")
        .map(|s| s.split_whitespace().next().unwrap_or("?").to_owned())
        .unwrap_or_default()
}
