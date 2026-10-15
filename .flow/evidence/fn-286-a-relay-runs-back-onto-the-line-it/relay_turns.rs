//! fn-286 probe, not product code: every relay's turn off the axis it
//! replaces, as the engine drew it. Copied into
//! `crates/telperion-core/examples/relay_turns.rs` by `turns.sh` for a run.
//!
//!   relay_turns <value file> <seeds> [<path>=<value>...]
//!
//! Per relay with wood: `joint`, the angle between its parent's heading at
//! the relay's node and its own first phytomer's heading; `chord`, between
//! that parent heading and the chord from its base to its wood 0.5 m on
//! (none where it is shorter). One JSON line per tree: the relays' count,
//! the joint and chord turns' median and 90th percentile, and how many
//! turn 40 degrees or more, by PA.
use telperion_core::presets::values;
use telperion_space::{grow, Origin, Request, Vec3};

fn family(file: &str, sets: &[String]) -> telperion_core::Family {
    let mut text = std::fs::read_to_string(file).expect("the value file");
    for set in sets {
        let (path, value) = set.split_once('=').expect("<path>=<value>");
        let line = format!("{path} = {value}");
        let mut lines: Vec<String> = text.lines().map(String::from).collect();
        match lines.iter().position(|l| l.split('=').next().map(str::trim) == Some(path)) {
            Some(i) => lines[i] = line,
            None => lines.push(line),
        }
        text = lines.join("\n") + "\n";
    }
    values::read(&text).expect("the value file reads")
}

fn angle(a: Vec3, b: Vec3) -> f64 {
    let (la, lb) = (a.length(), b.length());
    if la < 1e-9 || lb < 1e-9 {
        return 0.0;
    }
    (a.dot(b) / (la * lb)).clamp(-1.0, 1.0).acos().to_degrees()
}

fn quantile(v: &mut [f64], q: f64) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.sort_by(f64::total_cmp);
    v[((v.len() - 1) as f64 * q).round() as usize]
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let f = family(&a[0], &a[2..]);
    let sp = f.space.species.clone().expect("a space species");
    for seed in a[1].split(',').map(|v| v.parse::<u64>().unwrap()) {
        let r = Request { age: f.age as u32, seed, budget: 20_000_000, light: f.space.light };
        let s = match grow(&sp, r) {
            Ok(s) => s,
            Err(e) => {
                println!("{{\"seed\":{seed},\"error\":\"{e}\"}}");
                continue;
            }
        };
        let (mut joints, mut chords) = (vec![], vec![]);
        let mut by_pa = std::collections::BTreeMap::<usize, (usize, usize)>::new();
        for x in &s.axes {
            let Origin::Relay { parent, node } = x.origin else { continue };
            let Some(first) = x.phytomers.first() else { continue };
            let p = &s.axes[parent];
            let along = p.phytomers.get(node).map_or(p.heading, |q| q.heading);
            let joint = angle(along, first.heading);
            let mut run = 0.0;
            let mut at = x.base;
            for q in &x.phytomers {
                run += (q.tip - at).length();
                at = q.tip;
                if run >= 0.5 {
                    chords.push(angle(along, q.tip - x.base));
                    break;
                }
            }
            joints.push(joint);
            let e = by_pa.entry(x.pa).or_default();
            e.0 += 1;
            e.1 += usize::from(joint >= 40.0);
        }
        let big = |v: &[f64]| v.iter().filter(|&&t| t >= 40.0).count();
        let (jb, cb) = (big(&joints), big(&chords));
        let n = joints.len();
        let pas: Vec<String> = by_pa.iter().map(|(k, (n, b))| format!("\"{k}\":[{n},{b}]")).collect();
        println!(
            "{{\"seed\":{seed},\"relays\":{n},\"joint50\":{:.1},\"joint90\":{:.1},\"joint40\":{jb},\"chord50\":{:.1},\"chord90\":{:.1},\"chord40\":{cb},\"pa\":{{{}}}}}",
            quantile(&mut joints, 0.5),
            quantile(&mut joints, 0.9),
            quantile(&mut chords, 0.5),
            quantile(&mut chords, 0.9),
            pas.join(",")
        );
    }
}
