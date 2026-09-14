use serde_json::{json, Value};
use std::io::{self, Write};
use telperion_core::{
    branching::{self, Specimen},
    presets::Preset,
    tree::{BudFate, NodeKind, Tree},
};

const USAGE: &str = "growth_curve --preset <id> --seed <n> --ages a,b,c[,...] [--envelope]";

struct Arguments {
    preset: String,
    seed: u32,
    ages: Vec<f64>,
    envelope: bool,
}

fn arguments() -> Result<Arguments, String> {
    let (mut preset, mut seed, mut ages, mut envelope) = (None, None, None, false);
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        if flag == "--envelope" {
            envelope = true;
            continue;
        }
        let value = args
            .next()
            .ok_or_else(|| format!("{flag} needs a value; {USAGE}"))?;
        match flag.as_str() {
            "--preset" => preset = Some(value),
            "--seed" => seed = Some(value.parse().map_err(|_| "invalid seed")?),
            "--ages" => {
                ages = Some(
                    value
                        .split(',')
                        .map(|raw| {
                            raw.parse::<f64>()
                                .ok()
                                .filter(|v| v.is_finite() && *v >= 0.0)
                                .ok_or_else(|| format!("invalid age: {raw}"))
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                );
            }
            _ => return Err(format!("unknown argument: {flag}; {USAGE}")),
        }
    }
    Ok(Arguments {
        preset: preset.ok_or(USAGE)?,
        seed: seed.ok_or(USAGE)?,
        ages: ages.ok_or(USAGE)?,
        envelope,
    })
}

fn trunk_dbh(tree: &Tree) -> Option<f64> {
    let ground = tree.nodes.first()?.position.y;
    let mut children: Vec<Option<usize>> = vec![None; tree.nodes.len()];
    for (i, child) in tree.nodes.iter().enumerate().skip(1) {
        if child.kind != NodeKind::Structural {
            continue;
        }
        let selected = &mut children[child.parent? as usize];
        if selected.is_none_or(|old| child.radius > tree.nodes[old].radius) {
            *selected = Some(i);
        }
    }
    let mut parent = 0;
    while let Some(i) = children[parent] {
        let child = &tree.nodes[i];
        let a = tree.nodes[parent].position.y - ground;
        let b = child.position.y - ground;
        if (a <= 1.3 && b > 1.3) || (b <= 1.3 && a > 1.3) {
            return Some(
                2.0 * (child.start_radius
                    + (child.radius - child.start_radius) * (1.3 - a) / (b - a)),
            );
        }
        parent = i;
    }
    None
}

fn measure(tree: &Tree, args: &Arguments, kind: &str) -> Result<Value, String> {
    let root = tree.nodes.first().ok_or("tree has no root")?;
    let (mut min, mut max) = (root.position, root.position);
    for n in &tree.nodes {
        if !n.position.is_finite() || !n.radius.is_finite() || !n.start_radius.is_finite() {
            return Err("non-finite geometry".into());
        }
        min.x = min.x.min(n.position.x);
        min.y = min.y.min(n.position.y);
        min.z = min.z.min(n.position.z);
        max.x = max.x.max(n.position.x);
        max.y = max.y.max(n.position.y);
        max.z = max.z.max(n.position.z);
    }
    Ok(json!({"kind":kind,"preset":args.preset,"seed":args.seed,
        "height_m":max.y-root.position.y,"trunk_dbh_m":trunk_dbh(tree),
        "nodes":tree.nodes.len(),"crossover":tree.crossover,
        "laterals":tree.nodes.iter().filter(|n|n.shoot.bud_fate == BudFate::Lateral).count(),
        "structural_laterals":tree.nodes.iter().filter(|n|n.kind == NodeKind::Structural && n.shoot.bud_fate == BudFate::Lateral).count(),
        "bounds":{"min":[min.x,min.y,min.z],"max":[max.x,max.y,max.z]}}))
}

fn line(value: &Value) -> Result<(), String> {
    writeln!(io::stdout().lock(), "{value}").map_err(|e| e.to_string())
}

fn run() -> Result<(), String> {
    let mut args = arguments()?;
    let mut family = Preset::from_id(&args.preset)
        .ok_or("unknown preset")?
        .parameters();
    family.skeleton.seed = args.seed;
    let mature_age = family.growth.mature_age();
    args.ages.push(mature_age);
    args.ages.sort_by(f64::total_cmp);
    args.ages.dedup();
    family.age = *args.ages.last().unwrap();
    let specimen = Specimen::build(&family).map_err(|e| e.to_string())?;
    for &age in &args.ages {
        let read = specimen.read_at_age(age).map_err(|e| e.to_string())?;
        let mut value = measure(&read.tree, &args, "growth")?;
        value["age"] = json!(age);
        value["mature_age"] = json!(mature_age);
        value["placements"] = json!(read.placements.len());
        line(&value)?;
    }
    if args.envelope {
        let report =
            branching::generate(&family.skeleton, family.radii).map_err(|e| e.to_string())?;
        line(&measure(&report.tree, &args, "envelope")?)?;
    }
    Ok(())
}

fn main() {
    if let Err(message) = run() {
        let _ = line(&json!({"kind":"error","message":message}));
        std::process::exit(1);
    }
}
