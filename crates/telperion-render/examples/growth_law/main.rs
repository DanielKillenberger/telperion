//! Scratch (fn-190 R1a, never merged): one organogenesis rule from trunk to
//! twig by continuous physiological age, species as points in its space.
//!
//! growth_law grow  PRESET SEED SETTINGS.json OUT.json [STILLS_PREFIX]
//! growth_law today PRESET SEED OUT.json [STILLS_PREFIX]
//! growth_law walk  PRESET_A A.json PRESET_B B.json SEED STEPS OUT.jsonl
mod bands;
mod params;
mod score;
mod organ;
mod tree;

use params::Params;
use std::time::Instant;
use telperion_core::envelope::Envelope;
use telperion_core::math::Vec3;
use telperion_core::pipeline::{self, executor};
use telperion_core::presets::Preset;
use telperion_core::surface::Bounds;
use telperion_core::tree::Tree;
use telperion_core::Family;
use telperion_render::{hero_pose, render, write_png, Camera, Gpu, Level, Renderer, View, GROUND_REACH, STILL_FORMAT};

const POSE: &str = ".flow/evidence/fn-190-one-growth-law-species-are-points-in-a/raw/pose-beech.json";

fn family(id: &str, seed: u32) -> Family {
    let mut f = Preset::from_id(id).unwrap_or_else(|| panic!("unknown preset {id}")).parameters();
    f.skeleton.seed = seed;
    f
}

fn settings(path: &str) -> Params {
    let mut p = Params::default();
    let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    p.apply(&v).unwrap();
    p
}

fn timed<T>(f: impl FnOnce() -> T) -> (f64, T) {
    let t = Instant::now();
    let out = f();
    (t.elapsed().as_secs_f64() * 1e3, out)
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    v[v.len() / 2]
}

/// What the law needs from the preset: its envelope, root radius and pipe
/// exponent (today's tree supplies the root radius).
#[derive(Clone, Copy)]
struct Site {
    env: Envelope,
    root: f64,
    exponent: f64,
    seed: u32,
}

fn site(f: &Family) -> Site {
    let t = pipeline::build(f, pipeline::Request::default()).unwrap().skeleton.tree;
    Site { env: f.skeleton.envelope, root: t.nodes[0].radius, exponent: f.radii.fork_exponent, seed: f.skeleton.seed }
}

fn grow(p: &Params, s: Site, diag: bool) -> Result<organ::Grown, String> {
    organ::grow(p, organ::World { height: s.env.height, root_radius: s.root, exponent: s.exponent, seed: u64::from(s.seed) }, diag)
}

/// Crown shape against the authored envelope: height, and the p95
/// horizontal reach of fine wood over the envelope's widest radius.
fn shape(t: &Tree, env: &Envelope) -> serde_json::Value {
    let mut r: Vec<f64> = t.nodes.iter().filter(|n| n.kind != telperion_core::tree::NodeKind::Structural).map(|n| n.position.x.hypot(n.position.z)).collect();
    r.sort_by(|a, b| a.total_cmp(b));
    let p95 = r.get(r.len() * 95 / 100).copied().unwrap_or(f64::NAN);
    let h = t.nodes.iter().map(|n| n.position.y).fold(0.0, f64::max);
    serde_json::json!({"height_m": h, "height_over_authored": h / env.height, "reach_p95_over_spread": p95 / env.max_radius(),
        "width_over_height": 2.0 * p95 / h.max(1e-9), "authored_width_over_height": 2.0 * env.max_radius() / env.height})
}

/// Troll against Rauh: first-order axes off the stem (axis >= 0.05 H), their
/// elevation at birth and their base segment's elevation now, degrees.
fn trace(g: &organ::Grown, t: &Tree) -> serde_json::Value {
    let n = g.pos.len();
    let h = g.pos.iter().map(|q| q.y).fold(0.0, f64::max);
    let mut stem = vec![false; n];
    stem[0] = true;
    let mut cont = vec![usize::MAX; n];
    for i in 1..n {
        let p = g.parent[i].unwrap();
        stem[i] = !g.lateral[i] && stem[p];
        if !g.lateral[i] && cont[p] == usize::MAX {
            cont[p] = i;
        }
    }
    let (mut born, mut now) = (vec![], vec![]);
    for i in 1..n {
        let p = g.parent[i].unwrap();
        if !(g.lateral[i] && stem[p]) || g.elev0[i].is_nan() {
            continue;
        }
        let (mut e, mut l) = (i, (g.pos[i] - g.pos[p]).length());
        while cont[e] != usize::MAX {
            let c = cont[e];
            l += (g.pos[c] - g.pos[e]).length();
            e = c;
        }
        if l < 0.05 * h {
            continue;
        }
        let d = g.pos[i] - g.pos[p];
        born.push(g.elev0[i]);
        now.push(d.y.atan2(d.x.hypot(d.z)).to_degrees());
    }
    let rise: Vec<f64> = now.iter().zip(&born).map(|(a, b)| a - b).collect();
    let top = (0..n).filter(|&i| stem[i]).map(|i| g.pos[i].y).fold(0.0, f64::max);
    let _ = t;
    let md = |v: &[f64]| if v.is_empty() { f64::NAN } else { median(v.to_vec()) };
    serde_json::json!({"axes": born.len(), "birth_elev_median": md(&born), "now_elev_median": md(&now), "rise_median": md(&rise),
        "stem_top_share": top / h.max(1e-9)})
}

fn today(id: &str, seed: u32) -> serde_json::Value {
    let p = format!(".flow/evidence/fn-190-one-growth-law-species-are-points-in-a/raw/today/{id}-{seed}.json");
    std::fs::read_to_string(p).map_or(serde_json::Value::Null, |s| serde_json::from_str(&s).unwrap())
}

fn camera(id: &str, t: &Tree) -> Camera {
    if id == "european-beech" {
        if let Ok(txt) = std::fs::read_to_string(POSE) {
            let v: Vec<f64> = serde_json::from_str(&txt).unwrap();
            return Camera { position: Vec3::new(v[0], v[1], v[2]), target: Vec3::new(v[3], v[4], v[5]), field_of_view: v[6], near: v[7], far: v[8] };
        }
    }
    let (mut lo, mut hi) = (Vec3::new(1e9, 1e9, 1e9), Vec3::new(-1e9, -1e9, -1e9));
    for n in &t.nodes {
        let q = n.position;
        lo = Vec3::new(lo.x.min(q.x), lo.y.min(q.y), lo.z.min(q.z));
        hi = Vec3::new(hi.x.max(q.x), hi.y.max(q.y), hi.z.max(q.z));
    }
    hero_pose(Bounds { min: lo, max: hi }, 960.0 / 720.0, GROUND_REACH)
}

fn measure(id: &str, t: &Tree) -> serde_json::Value {
    let cam = camera(id, t);
    let sc = score::score(t, &cam);
    let fine = tree::fine(t);
    serde_json::json!({"score": sc, "fine": fine})
}

fn stills(id: &str, f: &Family, t: Tree, prefix: &str) {
    let cam = camera(id, &t);
    let m = executor::expand(t, f).and_then(|x| x.mesh()).unwrap();
    let gpu = pollster::block_on(Gpu::request(None)).unwrap();
    let mut rd = Renderer::new(gpu, STILL_FORMAT);
    for (view, k) in [(View::Bare, "bare"), (View::Whole, "whole")] {
        rd.submit_at(&m, Level::Chosen).unwrap();
        rd.set_view(view);
        let s = render(&mut rd, &cam, 960, 720).unwrap();
        write_png(std::path::Path::new(&format!("{prefix}-{k}.png")), &s).unwrap();
    }
}

fn write(path: &str, v: &serde_json::Value) {
    std::fs::write(path, serde_json::to_string_pretty(v).unwrap()).unwrap();
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a[1].as_str() {
        "grow" => {
            let (id, seed) = (&a[2], a[3].parse().unwrap());
            let p = settings(&a[4]);
            let f = family(id, seed);
            let s = site(&f);
            let fail = |e: String| -> ! {
                eprintln!("error: {id} seed {seed}: {e}");
                std::process::exit(2)
            };
            let (cold, g) = timed(|| grow(&p, s, false));
            let g = g.unwrap_or_else(|e| fail(e));
            let warm: Vec<f64> = (0..3).map(|_| timed(|| grow(&p, s, false)).0).collect();
            if g.pos.len() < 1000 {
                fail(format!("grew {} nodes, under 1,000; not a scored tree", g.pos.len()));
            }
            let diag = grow(&p, s, true).unwrap_or_else(|e| fail(e)).stats;
            let t = tree::to_tree(&g);
            t.validate().unwrap();
            let st = &g.stats;
            let stages: serde_json::Map<_, _> = organ::STAGES.iter().zip(st.ms).map(|(k, v)| ((*k).to_string(), serde_json::json!((v * 100.0).round() / 100.0))).collect();
            let mut out = measure(id, &t);
            out["preset"] = serde_json::json!(id);
            out["seed"] = serde_json::json!(seed);
            out["settings"] = p.to_json();
            out["trace"] = trace(&g, &t);
            out["shape"] = shape(&t, &s.env);
            let time = median(warm);
            out["time_ms"] = serde_json::json!({"cold": cold, "warm_median": time, "passes_cold": stages});
            let mut kids = vec![false; g.pos.len()];
            g.parent.iter().flatten().for_each(|&q| kids[q] = true);
            out["law"] = serde_json::json!({"cycles": st.cycles, "buds_max": st.buds_max, "tips": kids.iter().filter(|k| !**k).count(), "exponent": s.exponent,
                "factorisation": {"laterals": diag.laterals, "distinct_phi_birth": diag.distinct, "distinct_phi": diag.distinct_phi,
                    "laterals_per_key": diag.laterals as f64 / diag.distinct.max(1) as f64}});
            out["votes"] = bands::judge(id, &out, &today(id, seed), time);
            write(&a[5], &out);
            if let Some(prefix) = a.get(6) {
                stills(id, &f, t, prefix);
            }
        }
        "today" => {
            let (id, seed) = (&a[2], a[3].parse().unwrap());
            let f = family(id, seed);
            let build = || pipeline::build(&f, pipeline::Request::default()).unwrap();
            let b = build();
            let warm: Vec<f64> = (0..3).map(|_| build().outputs.stages.skeleton_ms).collect();
            let mut t = b.skeleton.tree;
            tree::reclass(&mut t);
            let mut out = measure(id, &t);
            out["preset"] = serde_json::json!(id);
            out["seed"] = serde_json::json!(seed);
            out["time_ms"] = serde_json::json!({"cold": b.outputs.stages.skeleton_ms, "warm_median": median(warm)});
            write(&a[4], &out);
            if let Some(prefix) = a.get(5) {
                let m = telperion_core::mesh::build(&f).unwrap();
                let cam = camera(id, &t);
                let gpu = pollster::block_on(Gpu::request(None)).unwrap();
                let mut rd = Renderer::new(gpu, STILL_FORMAT);
                for (view, k) in [(View::Bare, "bare"), (View::Whole, "whole")] {
                    rd.submit_at(&m, Level::Chosen).unwrap();
                    rd.set_view(view);
                    let s = render(&mut rd, &cam, 960, 720).unwrap();
                    write_png(std::path::Path::new(&format!("{prefix}-{k}.png")), &s).unwrap();
                }
            }
        }
        "walk" => walk(&a[2], &a[3], &a[4], &a[5], a[6].parse().unwrap(), a[7].parse().unwrap(), &a[8]),
        other => panic!("unknown command {other}"),
    }
}

/// Grows `steps + 1` trees on the straight line between two species' points
/// (their settings, envelopes, root radii and exponents) and writes one row
/// per step.
fn walk(ida: &str, pa: &str, idb: &str, pb: &str, seed: u32, steps: usize, out: &str) {
    let (fa, fb) = (family(ida, seed), family(idb, seed));
    let (sa, sb) = (site(&fa), site(&fb));
    let (xa, xb) = (settings(pa), settings(pb));
    let mut rows = String::new();
    for k in 0..=steps {
        let w = k as f64 / steps as f64;
        let lerp = |a: f64, b: f64| a + (b - a) * w;
        let mut p = xa;
        for (key, _) in xa.to_json().as_object().unwrap() {
            p.set(key, lerp(xa.get(key).unwrap(), xb.get(key).unwrap())).unwrap();
        }
        let mut env = sa.env;
        let (ea, eb) = (sa.env, sb.env);
        env.height = lerp(ea.height, eb.height);
        env.crown_base = lerp(ea.crown_base, eb.crown_base);
        env.spread = lerp(ea.spread, eb.spread);
        env.fullness = lerp(ea.fullness, eb.fullness);
        env.shoulder = lerp(ea.shoulder, eb.shoulder);
        env.irregularity = lerp(ea.irregularity, eb.irregularity);
        env.lobe_scale = lerp(ea.lobe_scale, eb.lobe_scale);
        let s = Site { env, root: lerp(sa.root, sb.root), exponent: lerp(sa.exponent, sb.exponent), seed };
        let (ms, g) = timed(|| grow(&p, s, false));
        let Ok(g) = g else { eprintln!("error at w {w}"); std::process::exit(2) };
        let t = tree::to_tree(&g);
        // Walk rows are judged on numbers, with one camera rule for all.
        let mut row = measure("walk", &t);
        row["trace"] = trace(&g, &t);
        row["w"] = serde_json::json!(w);
        row["ms"] = serde_json::json!(ms);
        rows.push_str(&serde_json::to_string(&row).unwrap());
        rows.push('\n');
    }
    std::fs::write(out, rows).unwrap();
}
