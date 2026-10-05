//! Scratch (fn-208 step (i) measurement, not committed): the bark shader's
//! cost per covered pixel, and stills, for a tree's wood at the hero view,
//! the limb close-up and a 5 cm twig.
//!   f3_shade <engine|today> <spruce|oak> <seed> <out dir> [samples]
#[path = "space/tree.rs"]
mod tree;
use telperion_core::{math::Vec3, mesh::TreeMesh, params, pipeline::executor, presets::Preset, tree::NodeKind};
use telperion_render::{
    hero_pose, measure, render, write_png, Camera, Frame, Gpu, Level, Renderer, SceneRow, View,
    GROUND_REACH, STILL_FORMAT,
};
use telperion_space::{Light, Request};

const SIZE: (u32, u32) = (960, 720);

fn renderer(samples: u32) -> Renderer {
    let gpu = pollster::block_on(Gpu::request(None)).unwrap();
    make(gpu, samples)
}

fn make(gpu: Gpu, samples: u32) -> Renderer {
    Renderer::with_samples(gpu, STILL_FORMAT, samples)
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (source, name, seed, out) = (a[0].as_str(), a[1].as_str(), a[2].parse::<u64>().unwrap(), a[3].as_str());
    let samples: u32 = a.get(4).map_or(4, |s| s.parse().unwrap());
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
        (family.clone(), executor::expand(tree::convert(&s, trunk), &family).unwrap())
    } else {
        let mut family = base.clone();
        family.skeleton.seed = seed as u32;
        (family.clone(), executor::grow(&family).unwrap().expansion().unwrap())
    };
    let tree = x.tree();
    let mid = tree.nodes.iter().map(|n| n.position.y).fold(0.0, f64::max) * 0.5;
    let twig = tree
        .nodes
        .iter()
        .filter(|n| n.kind == NodeKind::Twig && (n.position.y - mid).abs() < 1.0)
        .max_by(|p, q| p.position.x.hypot(p.position.z).total_cmp(&q.position.x.hypot(q.position.z)))
        .map(|n| n.position)
        .unwrap();
    let mut mesh: TreeMesh = x.mesh().unwrap();
    mesh.foliage.instances.leaves.clear();
    // The same tree sunk far below the ground: what the frame is without wood.
    let mut hidden = TreeMesh {
        wood: Default::default(),
        foliage: telperion_core::mesh::Foliage {
            element: mesh.foliage.element.clone(),
            instances: mesh.foliage.instances.clone(),
        },
        bounds: mesh.bounds,
    };
    hidden.wood.positions = mesh.wood.positions.iter().enumerate().map(|(i, &v)| if i % 3 == 1 { v - 1.0e4 } else { v }).collect();
    hidden.wood.indices = mesh.wood.indices.clone();
    hidden.wood.normals = mesh.wood.normals.clone();
    hidden.wood.coords = mesh.wood.coords.clone();
    hidden.wood.run_table = mesh.wood.run_table.clone();
    hidden.wood.runs = mesh.wood.runs;
    hidden.wood.bounds = mesh.wood.bounds;
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
        ("twig", shot(twig, twig + outward * 0.05 + Vec3::new(0.0, 0.01, 0.0), 0.005)),
        // The bark close up: the trunk at 1.2 m from 0.75 m off its axis.
        ("trunk", shot(Vec3::new(0.0, 1.2, 0.0), Vec3::new(0.0, 1.2, 0.0) + toward * 0.75, 0.05)),
    ];
    let only: Option<String> = std::env::var("F3_VIEW").ok();
    let views: Vec<_> = views.into_iter().filter(|v| only.as_deref().is_none_or(|o| o == v.0)).collect();
    let mut r = renderer(samples);
    r.set_material(family.material);
    r.set_scene(SceneRow { sun_azimuth: 115.0, sun_elevation: 60.0, ..SceneRow::default() });
    let frame = Frame::new(&r, "f3", SIZE);
    for (label, camera) in views {
        // Covered pixels: the clay room with and without the wood.
        r.set_view(View::Clay);
        r.submit_at(&hidden, Level::Chosen).unwrap();
        let empty = render(&mut r, &camera, SIZE.0, SIZE.1).unwrap();
        r.submit_at(&mesh, Level::Chosen).unwrap();
        r.set_material(family.material);
        let clay = render(&mut r, &camera, SIZE.0, SIZE.1).unwrap();
        let covered = clay.rgba.chunks(4).zip(empty.rgba.chunks(4)).filter(|(p, q)| p != q).count();
        r.set_view(View::Bare);
        let still = render(&mut r, &camera, SIZE.0, SIZE.1).unwrap();
        write_png(std::path::Path::new(&format!("{out}/{source}-{name}-{seed}-{label}-s{samples}.png")), &still).unwrap();
        let mut ms = Vec::new();
        for view in [View::Bare, View::Clay] {
            r.set_view(view);
            let report = measure(&mut r, &camera, SIZE, frame.target()).unwrap();
            let j: serde_json::Value = serde_json::from_str(&report.to_json()).unwrap();
            ms.push((j["p50_ms"].as_f64().unwrap_or(f64::NAN), j["verdict"].as_str().unwrap_or("?").to_string()));
        }
        let shading = ms[0].0 - ms[1].0;
        println!(
            "{source} {name} {seed} {label} samples {samples}: bark {:.3} ms ({}), flat {:.3} ms ({}), covered {covered} px, bark shading {:.3} ms, {:.1} ns a covered pixel",
            ms[0].0, ms[0].1, ms[1].0, ms[1].1, shading, shading * 1e6 / covered as f64
        );
    }
}
