//! What the video draws around its renders, read from the pipeline itself.
//!
//! `dump blend <from> <to> <seed> [overlay.json]` steps a blend in twenty
//! steps and prints whole-tree metrics a step, so a morph is shown only once
//! it is measured smooth. The overlay, when given, is laid over the near end.
//! `BLEND_SPAN=a,b` steps that span of t instead, to tell a steep stretch
//! from a jump.
//!
//! `dump stages <preset> <seed> <out.json>` writes one build's stage
//! artifacts: the skeleton in the order it was grown, the attractor cloud the
//! grower scatters, the leaf plan's counts, the placed and kept leaves, the
//! wood's size and every stage's milliseconds.
use serde_json::{json, Value};
use telperion_core::{
    blend,
    envelope::Envelope,
    params,
    pipeline::{self, Built, Request},
    presets::Preset,
    rng::Rng,
    tree::NodeKind,
    Family,
};

type Outcome<T> = Result<T, String>;

const STEPS: u32 = 20;
/// The most leaf positions a stage file carries; a denser crown is strided.
const LEAF_CAP: usize = 80_000;

fn family(id: &str, seed: u32) -> Outcome<Family> {
    let mut f = Preset::from_id(id)
        .ok_or_else(|| format!("unknown preset {id}"))?
        .parameters();
    f.skeleton.seed = seed;
    Ok(f)
}

fn build(f: &Family) -> Outcome<Built> {
    pipeline::build(f, Request::mesh()).map_err(|e| e.to_string())
}

/// Whole-tree metrics of one build: the skeleton's extent, its size, the
/// wood's size and the kept leaves.
fn metrics(built: &Built) -> Value {
    let nodes = &built.skeleton.tree.nodes;
    let height = nodes.iter().map(|n| n.position.y).fold(0.0, f64::max);
    let spread = nodes
        .iter()
        .map(|n| n.position.x.hypot(n.position.z))
        .fold(0.0, f64::max);
    let wood = built
        .outputs
        .wood
        .as_ref()
        .map_or(0, |w| w.positions.len() / 3);
    let leaves = built.outputs.leaves.as_ref().map_or(0, |l| l.retained);
    json!({
        "height": height,
        "spread": spread,
        "nodes": nodes.len(),
        "woodVertices": wood,
        "leaves": leaves,
    })
}

fn blend_walk(from: &str, to: &str, seed: u32, overlay: Option<&str>) -> Outcome<()> {
    let mut near = family(from, seed)?;
    if let Some(path) = overlay {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        let rows: Value = serde_json::from_str(&text).map_err(|e| format!("{path}: {e}"))?;
        near = params::overlay(&near, &rows).map_err(|e| format!("{path}: {e:?}"))?;
    }
    let far = family(to, seed)?;
    let (a, b) = span()?;
    let mut previous: Option<Vec<f32>> = None;
    for step in 0..=STEPS {
        let t = a + (b - a) * f64::from(step) / f64::from(STEPS);
        let f = blend::families(&near, &far, t).map_err(|e| e.to_string())?;
        let built = build(&f).map_err(|e| format!("t {t}: {e}"))?;
        let wood = built.outputs.wood.as_ref().map(|w| w.positions.clone());
        let shift = shift(previous.as_deref(), wood.as_deref());
        previous = wood;
        println!(
            "{}",
            json!({ "t": t, "metrics": metrics(&built), "woodShift": shift })
        );
    }
    Ok(())
}

/// The farthest any wood vertex moved since the step before, in metres:
/// how a walk that keeps the wood's topology moves its surface. None where
/// there is no step before or the vertex count changed.
fn shift(before: Option<&[f32]>, after: Option<&[f32]>) -> Option<f64> {
    let (before, after) = (before?, after?);
    if before.len() != after.len() {
        return None;
    }
    let far = before
        .as_chunks::<3>()
        .0
        .iter()
        .zip(after.as_chunks::<3>().0)
        .map(|(p, q)| {
            let d = |i: usize| f64::from(q[i] - p[i]);
            (d(0) * d(0) + d(1) * d(1) + d(2) * d(2)).sqrt()
        })
        .fold(0.0, f64::max);
    Some(far)
}

/// The span of t a walk steps: `BLEND_SPAN=a,b`, or the whole walk.
fn span() -> Outcome<(f64, f64)> {
    let Ok(raw) = std::env::var("BLEND_SPAN") else {
        return Ok((0.0, 1.0));
    };
    let bad = || format!("BLEND_SPAN wants a,b within 0 to 1, not {raw}");
    let (a, b) = raw.split_once(',').ok_or_else(bad)?;
    let (a, b): (f64, f64) = (a.parse().map_err(|_| bad())?, b.parse().map_err(|_| bad())?);
    if !(0.0..=1.0).contains(&a) || !(0.0..=1.0).contains(&b) || a >= b {
        return Err(bad());
    }
    Ok((a, b))
}

/// The envelope the grower scatters its attractors into: the crown inset by
/// the twig reach, as `pipeline::branching::inner_envelope` states it. That
/// function is private to the pipeline; this is its formula, restated.
fn inner(e: Envelope, reach: f64) -> Envelope {
    let share = 1.0 - reach;
    let base = e.height * e.crown_base;
    let height = base + (e.height - base) * share;
    if height <= 0.0 {
        return e;
    }
    Envelope {
        height,
        crown_base: base / height,
        spread: e.spread * share * (e.height / height),
        ..e
    }
}

fn rounded(values: impl Iterator<Item = f64>) -> Vec<Value> {
    values
        .map(|v| json!((v * 1000.0).round() / 1000.0))
        .collect()
}

fn stages(id: &str, seed: u32, out: &str) -> Outcome<()> {
    let f = family(id, seed)?;
    // The field as well, so the leaf plan it reads is built and timed.
    let request = Request {
        field: Some(None),
        ..Request::mesh()
    };
    let built = pipeline::build(&f, request).map_err(|e| e.to_string())?;
    let tree = &built.skeleton.tree;
    let s = &f.skeleton;
    let points = if s.habit.attractor_weight > 0.0 {
        inner(s.envelope, s.twigs.reach)
            .sample_with_attempts(
                s.attractors,
                &mut Rng::new(s.seed),
                s.seed,
                s.sampling_attempts_per_attractor,
            )
            .map_err(|e| e.to_string())?
    } else {
        Vec::new()
    };
    let growth = s.resolved_growth(points.len()).map_err(|e| e.to_string())?;
    let kind = |k: NodeKind| match k {
        NodeKind::Structural => 0,
        NodeKind::Branch => 1,
        NodeKind::Twig => 2,
    };
    let o = &built.outputs;
    let leaves = o.leaves.as_ref().ok_or("the build kept no leaves")?;
    let kept = &leaves.instances;
    let stride = kept.leaves.len().div_ceil(LEAF_CAP).max(1);
    let wood = o.wood.as_ref().ok_or("the build drew no wood")?;
    let plan = o.plan.as_ref();
    let document = json!({
        "preset": id,
        "seed": seed,
        "crossover": tree.crossover,
        "nodes": rounded(tree.nodes.iter().flat_map(|n| {
            [n.position.x, n.position.y, n.position.z, n.radius]
        })),
        "parents": tree.nodes.iter().map(|n| n.parent.map_or(-1, i64::from)).collect::<Vec<_>>(),
        "kinds": tree.nodes.iter().map(|n| kind(n.kind)).collect::<Vec<_>>(),
        "attractors": rounded(points.iter().flat_map(|p| [p.x, p.y, p.z])),
        "killDistance": growth.kill_distance,
        "influenceRadius": growth.influence_radius,
        "plan": {
            "descriptors": plan.map_or(0, |p| p.descriptors.len()),
            "total": plan.map_or(0, |p| p.total),
        },
        "placed": leaves.placed,
        "retained": leaves.retained,
        "leafStride": stride,
        "leaves": rounded((0..kept.leaves.len()).step_by(stride).flat_map(|i| {
            let p = kept.position(i);
            [p.x, p.y, p.z]
        })),
        "wood": {
            "vertices": wood.positions.len() / 3,
            "triangles": wood.indices.len() / 3,
        },
        "stagesMs": {
            "grow": o.stages.skeleton_ms,
            "plan": o.stages.plan_ms,
            "rings": o.stages.rings_ms,
            "wood": o.stages.wood_ms,
            "placement": o.stages.placement_ms,
            "cull": o.stages.cull_ms,
            "field": o.stages.field_ms,
            "total": o.stages.total_ms,
        },
        "metrics": metrics(&built),
    });
    std::fs::write(out, document.to_string()).map_err(|e| format!("{out}: {e}"))
}

fn run() -> Outcome<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let seed = |raw: Option<&String>| -> Outcome<u32> {
        raw.ok_or("a seed is required")?
            .parse()
            .map_err(|_| "the seed is a whole number".to_owned())
    };
    match args.first().map(String::as_str) {
        Some("blend") if args.len() >= 4 => blend_walk(
            &args[1],
            &args[2],
            seed(args.get(3))?,
            args.get(4).map(String::as_str),
        ),
        Some("stages") if args.len() == 4 => stages(&args[1], seed(args.get(2))?, &args[3]),
        _ => Err("usage: dump blend <from> <to> <seed> [overlay.json] | \
                  dump stages <preset> <seed> <out.json>"
            .into()),
    }
}

fn main() {
    if let Err(reason) = run() {
        eprintln!("{reason}");
        std::process::exit(1);
    }
}
