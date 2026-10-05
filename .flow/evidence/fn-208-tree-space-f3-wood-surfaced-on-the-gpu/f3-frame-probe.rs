//! Scratch (fn-208 measurement, not committed): today's wood path's frame
//! time on this GPU, for a tree at the standard views and a 5 cm twig
//! close-up, through the timing protocol (`telperion_render::measure`).
//!   f3_frame <engine|today> <spruce|oak> <seed> [views]
#[path = "space/tree.rs"]
mod tree;
use telperion_core::{math::Vec3, mesh::TreeMesh, params, pipeline::executor, presets::Preset, tree::NodeKind};
use telperion_render::{
    hero_pose, measure, Camera, Frame, Gpu, Level, Renderer, SceneRow, View, GROUND_REACH,
    STILL_FORMAT,
};
use telperion_space::{Light, Request};

const SIZE: (u32, u32) = (960, 720);

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (source, name, seed) = (a[0].as_str(), a[1].as_str(), a[2].parse::<u64>().unwrap());
    let views: Vec<String> = a.get(3).map_or(vec!["hero".into(), "limb".into(), "twig".into()], |v| v.split(',').map(String::from).collect());
    let (preset, rows) = match name {
        "spruce" => ("norway-spruce", r#"{"canopy": {"shortShootSpacing": 0, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.0032}}}}"#),
        "oak" => ("oregon-white-oak", r#"{"canopy": {"shortShootSpacing": 0, "shootRadius": 0.06, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.008}}}}"#),
        _ => panic!("species"),
    };
    let base = Preset::from_id(preset).unwrap().parameters();
    let (family, x) = if source == "engine" {
        let mut family = params::overlay(&base, &serde_json::from_str(rows).unwrap()).unwrap();
        family.skeleton.seed = (seed ^ (seed >> 32)) as u32;
        let (species, trunk, light): (_, &[usize], _) = if name == "spruce" {
            (telperion_space::spruce(), &tree::SPRUCE_TRUNK, Light::NEUTRAL)
        } else {
            (telperion_space::oak(), &tree::OAK_TRUNK, Light { extinction: 0.5, sky: 0.5 })
        };
        let s = telperion_space::grow(&species, Request { age: 80, seed, budget: 20_000_000, light }).unwrap();
        let x = executor::expand(tree::convert(&s, trunk), &family).unwrap();
        (family, x)
    } else {
        let mut family = base.clone();
        family.skeleton.seed = seed as u32;
        let x = executor::grow(&family).unwrap().expansion().unwrap();
        (family, x)
    };
    // The twig: a childless fine node at mid-height farthest out from the
    // trunk, seen from 5 cm further out.
    let tree = x.tree();
    let mid = tree.nodes.iter().map(|n| n.position.y).fold(0.0, f64::max) * 0.5;
    let twig = tree
        .nodes
        .iter()
        .filter(|n| n.kind == NodeKind::Twig && (n.position.y - mid).abs() < 1.0)
        .max_by(|p, q| p.position.x.hypot(p.position.z).total_cmp(&q.position.x.hypot(q.position.z)))
        .map(|n| (n.position, n.radius))
        .unwrap();
    let mut mesh: TreeMesh = x.mesh().unwrap();
    // Step 2 times wood alone: the leaves go, so the clay view draws wood only.
    mesh.foliage.instances.leaves.clear();
    let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
    let hero = hero_pose(mesh.bounds, aspect, GROUND_REACH);
    let toward = Vec3::new(hero.position.x, 0.0, hero.position.z);
    let toward = toward * (1.0 / toward.length());
    let b = mesh.bounds;
    let side = Vec3::new(toward.z, 0.0, -toward.x);
    let limb_at = Vec3::new(0.0, 0.5 * (b.max.y - b.min.y), 0.0) + side * ((b.max.x - b.min.x) / 3.0);
    let shot = |target: Vec3, from: Vec3, near: f64| Camera { position: from, target, field_of_view: 38.0, near, far: 500.0 };
    let out = Vec3::new(twig.0.x, 0.0, twig.0.z);
    let out = out * (1.0 / out.length().max(1e-9));
    eprintln!(
        "{source} {name} {seed}: {} wood triangles, {} wood vertices, {} leaves; twig radius {:.4} m at {:?}",
        mesh.wood_triangles(), mesh.wood_vertices(), mesh.foliage_instances(), twig.1, twig.0
    );
    let estimate_only = std::env::var_os("F3_ESTIMATE").is_some();
    let mut gpu_state = None;
    for v in &views {
        let camera = match v.as_str() {
            "hero" => hero,
            "limb" => shot(limb_at, limb_at + toward * 9.0 + Vec3::new(0.0, 0.5, 0.0), 0.05),
            "twig" => shot(twig.0, twig.0 + out * 0.05 + Vec3::new(0.0, 0.01, 0.0), 0.005),
            _ => panic!("view"),
        };
        if estimate_only {
            estimate(x.tree(), &camera, &format!("{source} {name} {seed} {v}"));
            continue;
        }
        if gpu_state.is_none() {
            let gpu = pollster::block_on(Gpu::request(None)).unwrap();
            let mut renderer = Renderer::new(gpu, STILL_FORMAT);
            renderer.submit_at(&mesh, Level::Chosen).unwrap();
            renderer.set_material(family.material);
            renderer.set_scene(SceneRow { sun_azimuth: 115.0, sun_elevation: 60.0, ..SceneRow::default() });
            gpu_state = Some(renderer);
        }
        let renderer = gpu_state.as_mut().unwrap();
        // Shading scales with pixels and triangles do not: the same view at
        // four sizes of one aspect, the bark (bare) and flat (clay).
        for size in [(64u32, 48u32), (480, 360), (960, 720), (1920, 1440)] {
            let frame = Frame::new(renderer, "f3", size);
            for (view, label) in [(View::Bare, "bark"), (View::Clay, "flat")] {
                renderer.set_view(view);
                let report = measure(renderer, &camera, size, frame.target()).unwrap();
                let j: serde_json::Value = serde_json::from_str(&report.to_json()).unwrap();
                println!(
                    "{source} {name} {seed} {v} {}x{} {label}: p50 {} ms p95 {} ms verdict {}",
                    size.0, size.1, j["p50_ms"], j["p95_ms"], j["verdict"]
                );
            }
        }
    }
}

/// The triangles a half-pixel tessellator would emit for this tree's wood at
/// this camera: per segment (parent to node), its projected radius rho in
/// pixels sets sides n = max(3, ceil(pi / acos(1 - 0.5 / rho))); below 1 px a
/// two-triangle ribbon, below 0.25 px nothing; rings per segment from its
/// turn at the parent, m = ceil(sqrt(L theta P / 4)) at least 1, where P is
/// pixels per metre at its depth. Segments outside the frustum (with their
/// radius as margin) or behind the eye are dropped.
fn estimate(tree: &telperion_core::tree::Tree, camera: &Camera, label: &str) {
    let ppm1 = 0.5 * f64::from(SIZE.1) / (camera.field_of_view.to_radians() * 0.5).tan();
    let forward = (camera.target - camera.position) * (1.0 / (camera.target - camera.position).length());
    let right = {
        let r = Vec3::new(-forward.z, 0.0, forward.x);
        r * (1.0 / r.length())
    };
    let upv = Vec3::new(
        right.y * forward.z - right.z * forward.y,
        right.z * forward.x - right.x * forward.z,
        right.x * forward.y - right.y * forward.x,
    );
    let (tv, th) = ((camera.field_of_view.to_radians() * 0.5).tan(), (camera.field_of_view.to_radians() * 0.5).tan() * f64::from(SIZE.0) / f64::from(SIZE.1));
    let n = &tree.nodes;
    let (mut tube, mut ribbon, mut dropped, mut culled, mut tris, mut rings, mut maxsides) = (0u64, 0u64, 0u64, 0u64, 0f64, 0f64, 0u32);
    let (mut merged, mut cover) = (0f64, 0f64);
    for (i, node) in n.iter().enumerate().skip(1) {
        let p = node.parent.unwrap() as usize;
        let (a, b) = (n[p].position, node.position);
        let mid = (a + b) * 0.5;
        let rel = mid - camera.position;
        let depth = rel.dot(forward);
        let r = node.start_radius.max(node.radius);
        let len = (b - a).length();
        let reach = r + 0.5 * len;
        if depth + reach < camera.near
            || rel.dot(right).abs() - reach > depth.max(camera.near) * th
            || rel.dot(upv).abs() - reach > depth.max(camera.near) * tv
        {
            culled += 1;
            continue;
        }
        let pp = ppm1 / depth.max(camera.near);
        let rho = r * pp;
        let turn = n[p].parent.map_or(0.0, |g| {
            let (u, w) = (a - n[g as usize].position, b - a);
            (u.dot(w) / (u.length() * w.length()).max(1e-12)).clamp(-1.0, 1.0).acos()
        });
        let m = (len * turn * pp / 4.0).sqrt().ceil().max(1.0);
        let bend = (len * turn * pp / 4.0).sqrt();
        if rho < 0.25 {
            // A coverage ribbon one pixel wide, alpha by its true width, its
            // rings merged along straight wood up to 8 px apart.
            dropped += 1;
            cover += 2.0 * (len * pp / 8.0).max(bend);
            continue;
        }
        if rho < 1.0 {
            merged += 2.0 * (len * pp / 8.0).max(bend);
            ribbon += 1;
            tris += 2.0 * m;
        } else {
            tube += 1;
            let sides = (std::f64::consts::PI / (1.0 - 0.5 / rho).acos()).ceil().max(3.0);
            maxsides = maxsides.max(sides as u32);
            tris += 2.0 * sides * m;
            merged += 2.0 * sides * (len * pp / 32.0).max(bend);
        }
        rings += m;
        let _ = i;
    }
    println!("{label}: estimate {:.2}M triangles, {:.2}M rings; merged {:.2}M, plus coverage ribbons {:.2}M; segments: {tube} tube, {ribbon} ribbon, {dropped} sub-pixel, {culled} outside; most sides {maxsides}", tris / 1e6, rings / 1e6, merged / 1e6, cover / 1e6);
}
