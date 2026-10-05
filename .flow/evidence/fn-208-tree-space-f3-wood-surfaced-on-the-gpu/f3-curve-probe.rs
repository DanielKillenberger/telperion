//! Scratch (fn-208 R3/R4, copied into examples/ to run, not committed): an
//! engine species' wood through the renderer's one wood path, the curve, at
//! the hero view, the limb close-up, 5 cm off a twig and 5 cm behind a twig
//! looking along it: stills (bare and whole), the vegetation and surfacing
//! passes' GPU time, the frame's wall time, triangles, the demand a pixel at
//! the finest scale and the wood's bytes on the device. Today's mesh path is
//! gone from the renderer (host decision 20); its numbers are STEP4.md's.
//!   f3_curve <beech|spruce|oak|palm> curve <out dir> [seed]
#[path = "space/tree.rs"]
mod tree;
use std::time::Instant;
use telperion_core::{
    math::Vec3, mesh::TreeMesh, params, pipeline::executor, presets::Preset, tree::NodeKind,
};
use telperion_render::{
    hero_pose, measure, render, write_png, Camera, Frame, Gpu, Level, Renderer, SceneRow,
    View, GROUND_REACH, STILL_FORMAT,
};
use telperion_space::{Light, Request};

const SIZE: (u32, u32) = (960, 720);

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (name, path, out) = (a[0].as_str(), a[1].as_str(), a[2].as_str());
    let seed: u64 = a.get(3).map_or(1, |s| s.parse().unwrap());
    let lit = Light { extinction: 0.5, sky: 0.5 };
    let (species, trunk, preset, rows, light): (_, &[usize], _, _, _) = match name {
        "beech" => (telperion_space::beech(), &tree::BEECH_TRUNK, "european-beech",
            r#"{"canopy": {"shortShootSpacing": 0, "shootRadius": 0.018, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.006}}}}"#, Light::NEUTRAL),
        "spruce" => (telperion_space::spruce(), &tree::SPRUCE_TRUNK, "norway-spruce",
            r#"{"canopy": {"shortShootSpacing": 0, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.0032}}}}"#, Light::NEUTRAL),
        "oak" => (telperion_space::oak(), &tree::OAK_TRUNK, "oregon-white-oak",
            r#"{"canopy": {"shortShootSpacing": 0, "shootRadius": 0.06, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.008}}}}"#, lit),
        "palm" => (telperion_space::palm(), &tree::PALM_TRUNK, "date-palm", "{}", Light::NEUTRAL),
        _ => panic!("species"),
    };
    let base = Preset::from_id(preset).unwrap().parameters();
    let mut family = params::overlay(&base, &serde_json::from_str(rows).unwrap()).unwrap();
    family.skeleton.seed = (seed ^ (seed >> 32)) as u32;
    let s = telperion_space::grow(&species, Request { age: 80, seed, budget: 20_000_000, light }).unwrap();
    let x = executor::expand(tree::convert(&s, trunk), &family).unwrap();
    let t = x.tree();
    let top = t.nodes.iter().map(|n| n.position.y).fold(0.0, f64::max);
    let outermost = |n: &&telperion_core::tree::Node| {
        n.kind == NodeKind::Twig && n.parent.is_some() && (n.position.y - 0.5 * top).abs() < 1.0
    };
    let chosen = t
        .nodes
        .iter()
        .filter(outermost)
        .max_by(|p, q| p.position.x.hypot(p.position.z).total_cmp(&q.position.x.hypot(q.position.z)))
        .or_else(|| t.nodes.last())
        .unwrap();
    let (twig, twig_radius) = (chosen.position, chosen.radius);
    // The chosen twig's base and its direction, for the shot along it.
    let base = chosen.parent.map_or(twig, |p| t.nodes[p as usize].position);
    let along = (twig - base) * (1.0 / (twig - base).length().max(1e-9));
    assert_eq!(path, "curve", "the renderer has one wood path");
    let mesh: TreeMesh = x.mesh().unwrap();
    drop(x);
    let gpu = pollster::block_on(Gpu::request(None)).unwrap();
    let mut r = Renderer::new(gpu, STILL_FORMAT);
    r.submit_at(&mesh, Level::Chosen).unwrap();
    r.set_material(family.material);
    r.set_scene(SceneRow { sun_azimuth: 115.0, sun_elevation: 60.0, ..SceneRow::default() });
    let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
    let hero = hero_pose(mesh.bounds, aspect, GROUND_REACH);
    let toward = Vec3::new(hero.position.x, 0.0, hero.position.z);
    let toward = toward * (1.0 / toward.length());
    let b = mesh.bounds;
    let side = Vec3::new(toward.z, 0.0, -toward.x);
    let limb_at = Vec3::new(0.0, 0.5 * (b.max.y - b.min.y), 0.0) + side * ((b.max.x - b.min.x) / 3.0);
    let shot = |target: Vec3, from: Vec3, near: f64| Camera { position: from, target, field_of_view: 38.0, near, far: 500.0 };
    let outward = Vec3::new(twig.x, 0.0, twig.z);
    let outward = outward * (1.0 / outward.length().max(1e-9));
    let views = [
        ("hero", hero),
        ("limb", shot(limb_at, limb_at + toward * 9.0 + Vec3::new(0.0, 0.5, 0.0), 0.05)),
        // 5 cm off the wood's surface.
        ("twig", shot(twig, twig + outward * (twig_radius + 0.05) + Vec3::new(0.0, 0.01, 0.0), 0.005)),
        // 5 cm behind the twig's base on its own axis, looking along it.
        ("along", shot(twig + along * 0.1, base - along * 0.05 + Vec3::new(0.0, 0.004, 0.0), 0.002)),
    ];
    let frame = Frame::new(&r, "f3", SIZE);

    let demand_only = std::env::var_os("F3_DEMAND").is_some();
    for (label, camera) in views {
        let mut line = format!("{name} {path} {label}:");
        if demand_only {
            r.set_view(View::Bare);
            render(&mut r, &camera, SIZE.0, SIZE.1).unwrap();
            let c = r.curve_report().unwrap();
            let per = c.demand.map(|d| format!("{:.2}", f64::from(d) / f64::from(SIZE.0 * SIZE.1)));
            let sun = r.curve_sun_report().unwrap();
            let texels = f64::from(1024u32 * 1024);
            let sun_per = sun.demand.map(|d| format!("{:.2}", f64::from(d) / texels));
            println!(
                "{line} demand a pixel {per:?}, budget {:?}, scale {}; sun demand a texel \
                 {sun_per:?}, scale {}",
                c.budget, c.scale, sun.scale
            );
            continue;
        }
        let modes: Vec<(View, &str)> = if std::env::var_os("F3_CLAY").is_some() {
            vec![(View::Clay, "clay")]
        } else {
            vec![(View::Bare, "bare"), (View::Whole, "whole")]
        };
        for (view, view_name) in modes {
            r.set_view(view);
            let still = render(&mut r, &camera, SIZE.0, SIZE.1).unwrap();
            write_png(std::path::Path::new(&format!("{out}/{name}-{path}-{label}-{view_name}.png")), &still).unwrap();
            let report = measure(&mut r, &camera, SIZE, frame.target()).unwrap();
            let j: serde_json::Value = serde_json::from_str(&report.to_json()).unwrap();
            // The frame's wall time: the curve's compute passes, the shadow,
            // the room and the vegetation, waited for.
            let mut walls = Vec::new();
            for _ in 0..40 {
                let t = Instant::now();
                r.draw(&camera, SIZE, frame.target());
                let _ = r.gpu().device.poll(wgpu::PollType::wait_indefinitely());
                walls.push(t.elapsed().as_secs_f64() * 1e3);
            }
            walls.sort_by(f64::total_cmp);
            line += &format!(
                " {view_name}: vegetation {} ms, surfacing {} ms ({}), frame {:.2} ms;",
                j["p50_ms"], j["surfacing_p50_ms"], j["verdict"].as_str().unwrap_or("?"),
                walls[walls.len() / 2]
            );
        }
        let c = r.curve_report().unwrap();
        let pixels = f64::from(SIZE.0 * SIZE.1);
        let per = c.demand.map(|d| format!("{:.2}", f64::from(d) / pixels));
        let memory = r.curve_bytes().unwrap();
        println!(
            "{line} {} tube + {} ribbon triangles at scale {} (overrun {}); demand a pixel \
             (rings, vertices, tube, ribbon indices) {per:?}; wood on the device {:.1} MB",
            c.tube_triangles, c.ribbon_triangles, c.scale, c.overrun, memory as f64 / 1e6
        );
    }
}
