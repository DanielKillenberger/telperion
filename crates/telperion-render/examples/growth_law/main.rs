//! Scratch (fn-190 R1, never merged): one growth law from trunk to twig,
//! species as points in a continuous architecture space.
//!
//! growth_law grow  PRESET SEED SETTINGS.json OUT.json [STILLS_PREFIX]
//! growth_law today PRESET SEED OUT.json [STILLS_PREFIX]
//! growth_law walk  PRESET_A A.json PRESET_B B.json SEED STEPS OUT.jsonl
mod bands;
mod law;
mod params;
mod score;
mod shadow;
mod space;
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

fn grow(p: &Params, s: Site) -> law::Grown {
    let e = s.env;
    let unit = p.unit * e.height;
    let r = e.max_radius() * 1.3;
    let (lo, hi) = (Vec3::new(-r, 0.0, -r), Vec3::new(r, e.height * 1.05, r));
    let size = hi - lo;
    let count = (p.density * size.x * size.y * size.z / unit.powi(3)) as usize;
    let seedling = p.seedling;
    // Each marker is a point of the mature crown scaled to the crown of the
    // age it joins at: scale seedling + (1 - seedling) u^(1/young), so
    // young = 3 spreads markers evenly over the grown volume and a smaller
    // value gives the young crowns more.
    let place = |q: Vec3, m: u64| {
        if !e.contains(q, 0.0, s.seed) {
            return None;
        }
        let u = law::unit_hash(u64::from(s.seed) ^ 0xA5A5, m);
        let sc = seedling + (1.0 - seedling) * u.powf(1.0 / p.young);
        Some((q * sc, sc))
    };
    let threads = p.threads as usize;
    let space = space::Space::fill(lo, hi, count, p.perception * unit, p.occupancy * unit, u64::from(s.seed), threads, &place);
    let shadow = (p.light > 0.0).then(|| {
        let m = e.max_radius() + 0.3 * e.height;
        shadow::Shadow::new(Vec3::new(-m, 0.0, -m), Vec3::new(m, 1.3 * e.height, m), 2.0 * unit, p.shade, p.falloff, 6)
    });
    let ramp = (p.ontogeny * p.cycles).max(1.0);
    let scale = move |t: usize| seedling + (1.0 - seedling) * (t as f64 / ramp).min(1.0);
    law::grow(p, law::World { space, shadow, scale: &scale, height: e.height, root_radius: s.root, exponent: s.exponent, seed: u64::from(s.seed) })
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
    let b = bands::judge(id, &sc, &fine);
    serde_json::json!({"score": sc, "fine": fine, "bands": b})
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
            let (cold, g) = timed(|| grow(&p, s));
            let warm: Vec<f64> = (0..3).map(|_| timed(|| grow(&p, s)).0).collect();
            let t = tree::to_tree(&g);
            t.validate().unwrap();
            let st = &g.stats;
            let stages: serde_json::Map<_, _> = law::STAGES.iter().zip(st.ms).map(|(k, v)| ((*k).to_string(), serde_json::json!((v * 10.0).round() / 10.0))).collect();
            let mut out = measure(id, &t);
            out["preset"] = serde_json::json!(id);
            out["seed"] = serde_json::json!(seed);
            out["settings"] = p.to_json();
            out["time_ms"] = serde_json::json!({"cold": cold, "warm_median": median(warm), "stages_cold": stages});
            out["law"] = serde_json::json!({"cycles": st.cycles, "shed": st.shed, "buds_max": st.buds_max, "capped": st.capped, "markers": st.markers});
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
        let (ms, g) = timed(|| grow(&p, s));
        let t = tree::to_tree(&g);
        // Walk rows are judged on numbers, with one camera rule for all.
        let mut row = measure("walk", &t);
        row["w"] = serde_json::json!(w);
        row["ms"] = serde_json::json!(ms);
        rows.push_str(&serde_json::to_string(&row).unwrap());
        rows.push('\n');
    }
    std::fs::write(out, rows).unwrap();
}
