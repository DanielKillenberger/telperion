//! Scratch (fn-208 R3/R4, not committed): an engine species' wood through
//! today's mesh path or the curve path, at the hero view, the limb close-up
//! and a 5 cm twig: stills (bare and whole), the vegetation pass's GPU time,
//! the frame's wall time (the curve's compute passes included), triangles
//! and the wood's bytes on the device.
//!   f3_curve <beech|spruce|oak|palm> <mesh|curve> <out dir> [seed]
#[path = "space/tree.rs"]
mod tree;
use std::time::Instant;
use telperion_core::{
    math::Vec3, mesh::TreeMesh, params, pipeline::executor, presets::Preset, tree::NodeKind,
};
use telperion_render::{
    hero_pose, measure, render, write_png, Camera, Frame, Gpu, Level, Region, Renderer, SceneRow,
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
    let twig = t
        .nodes
        .iter()
        .filter(|n| n.kind == NodeKind::Twig && (n.position.y - 0.5 * top).abs() < 1.0)
        .max_by(|p, q| p.position.x.hypot(p.position.z).total_cmp(&q.position.x.hypot(q.position.z)))
        .or_else(|| t.nodes.last())
        .map(|n| (n.position, n.radius))
        .unwrap();
    let (twig, twig_radius) = twig;
    let mut mesh: TreeMesh = x.mesh().unwrap();
    let curve = x.curve().unwrap();
    drop(x);
    let wood_bytes = {
        let w = &mesh.wood;
        let vertices = (w.positions.len() / 3) as u64;
        [w.positions.len() as u64 * 4, w.normals.len() as u64 * 4, w.coords.len() as u64 * 4,
            w.indices.len() as u64 * 4, vertices * 4]
            .iter()
            .map(|&b| Region::capacity_for(b))
            .sum::<u64>()
    };
    let mesh_triangles = mesh.wood_triangles();
    if path == "curve" {
        // The wood comes from the curve: the mesh brings only leaves and bounds.
        mesh.wood = telperion_core::surface::SurfaceMesh {
            bounds: mesh.wood.bounds,
            ..Default::default()
        };
    }
    let gpu = pollster::block_on(Gpu::request(None)).unwrap();
    let mut r = Renderer::new(gpu, STILL_FORMAT);
    r.submit_at(&mesh, Level::Chosen).unwrap();
    if path == "curve" {
        r.submit_curve(&curve);
    }
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
    ];
    let frame = Frame::new(&r, "f3", SIZE);
    let memory = if path == "curve" { r.curve_bytes().unwrap() } else { wood_bytes };
    for (label, camera) in views {
        let mut line = format!("{name} {path} {label}:");
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
                " {view_name}: vegetation {} ms ({}), frame {:.2} ms;",
                j["p50_ms"], j["verdict"].as_str().unwrap_or("?"), walls[walls.len() / 2]
            );
        }
        let triangles = match r.curve_report() {
            Some(c) => format!("{} tube + {} ribbon triangles at scale {} (overrun {})",
                c.tube_triangles, c.ribbon_triangles, c.scale, c.overrun),
            None => format!("{mesh_triangles} triangles"),
        };
        println!("{line} {triangles}; wood on the device {:.1} MB", memory as f64 / 1e6);
    }
}
