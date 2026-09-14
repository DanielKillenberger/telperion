//! Temporary fn-29 evidence stills. The host removes this example before commit.
use std::path::Path;

use serde_json::{json, Value};
use telperion_core::{
    branching,
    math::Vec3,
    mesh::{self, Detail},
    presets::{Family, Preset},
    tree::Tree,
};
use telperion_render::{
    hero_pose, render, write_png, Camera, Gpu, Level, Renderer, SceneRow, View, GROUND_REACH,
    STILL_FORMAT,
};

#[derive(Clone, Copy)]
struct Fork {
    index: usize,
    radius: f64,
    camera: Camera,
}

fn camera(position: Vec3, target: Vec3) -> Camera {
    Camera {
        position,
        target,
        field_of_view: 38.0,
        near: 0.01,
        far: 1000.0,
    }
}

fn camera_json(camera: &Camera) -> Value {
    json!({
        "position": camera.position, "target": camera.target,
        "field_of_view": camera.field_of_view, "near": camera.near, "far": camera.far,
    })
}

fn fork_json(fork: Fork) -> Value {
    json!({"index": fork.index, "radius": fork.radius, "height": fork.camera.target.y})
}

fn fork_pose(tree: &Tree) -> Result<Fork, String> {
    let nodes = &tree.nodes;
    let mut children = vec![Vec::new(); nodes.len()];
    for (i, node) in nodes.iter().enumerate() {
        if let Some(parent) = node.parent {
            children[parent as usize].push(i);
        }
    }
    let largest = |forks_only: bool| {
        nodes
            .iter()
            .enumerate()
            .filter(|&(i, n)| n.position.y > 1.5 && (!forks_only || children[i].len() >= 2))
            .max_by(|a, b| a.1.radius.total_cmp(&b.1.radius).then(b.0.cmp(&a.0)))
            .map(|(i, _)| i)
    };
    let index = largest(true)
        .or_else(|| largest(false))
        .ok_or("no wood above 1.5 m")?;
    let node = &nodes[index];
    let mean = children[index].iter().fold(Vec3::ZERO, |sum, &i| {
        let direction = nodes[i].position - node.position;
        sum + Vec3::new(direction.x, 0.0, direction.z).normalized()
    });
    let side = if mean.length_squared() > 1e-12 {
        Vec3::new(-mean.z, 0.0, mean.x).normalized()
    } else {
        Vec3::X
    };
    // A wood run follows the thickest continuation at each junction. Walk
    // back to its attachment, then forward to its tip; other runs obstruct.
    let leaders: Vec<Option<usize>> = children
        .iter()
        .map(|list| {
            list.iter().copied().max_by(|&a, &b| {
                nodes[a]
                    .start_radius
                    .total_cmp(&nodes[b].start_radius)
                    .then(b.cmp(&a))
            })
        })
        .collect();
    let mut start = index;
    while let Some(parent) = nodes[start].parent.map(|p| p as usize) {
        if leaders[parent] != Some(start) {
            break;
        }
        start = parent;
    }
    let mut own_run = vec![false; nodes.len()];
    let mut at = Some(start);
    while let Some(i) = at {
        own_run[i] = true;
        at = leaders[i];
    }
    let stand_off = (6.0 * node.radius).max(1.2);
    let target = node.position;
    let eye = |sign: f64| target + side * (sign * stand_off) + Vec3::Y * (0.15 * stand_off);
    let clearance = |eye: Vec3| {
        let segment = eye - target;
        nodes
            .iter()
            .enumerate()
            .filter(|&(i, _)| !own_run[i])
            .map(|(_, n)| {
                let t =
                    ((n.position - target).dot(segment) / segment.length_squared()).clamp(0.0, 1.0);
                n.position.distance(target + segment * t) - n.radius
            })
            .fold(f64::INFINITY, f64::min)
    };
    let position = if clearance(eye(-1.0)) > clearance(eye(1.0)) {
        eye(-1.0)
    } else {
        eye(1.0)
    };
    Ok(Fork {
        index,
        radius: node.radius,
        camera: camera(position, target),
    })
}

struct Subject {
    id: &'static str,
    name: &'static str,
    leaf: &'static str,
    family: Family,
    fork: Fork,
}

fn subject(id: &'static str, name: &'static str, leaf: &'static str) -> Result<Subject, String> {
    let mut family = Preset::from_id(id).ok_or("unknown preset")?.parameters();
    family.skeleton.seed = 7;
    let skeleton = branching::generate(&family.skeleton, family.radii)
        .map_err(|e| e.to_string())?
        .tree;
    let fork = fork_pose(&skeleton)?;
    eprintln!(
        "{id} fork: {} camera: {}",
        fork_json(fork),
        camera_json(&fork.camera)
    );
    Ok(Subject {
        id,
        name,
        leaf,
        family,
        fork,
    })
}

fn still(
    renderer: &mut Renderer,
    directory: &Path,
    subject: &Subject,
    name: &str,
    camera: Camera,
    scene: SceneRow,
    fork: Option<Fork>,
) -> Result<(), String> {
    renderer.set_scene(scene);
    let frame = render(renderer, &camera, 1600, 1000).map_err(|e| e.to_string())?;
    let path = directory.join(format!("{name}.png"));
    write_png(&path, &frame).map_err(|e| e.to_string())?;
    println!(
        "{}",
        json!({
            "name": name, "preset": subject.id,
            "view": if name.contains("trunk") || name.contains("branch") { "Bare" } else { "Leaf" },
            "scene": serde_json::from_str::<Value>(&scene.to_json()).map_err(|e| e.to_string())?,
            "camera": camera_json(&camera), "png": path, "fork": fork.map(fork_json),
        })
    );
    Ok(())
}

fn run() -> Result<(), String> {
    let directory = std::env::args_os()
        .nth(1)
        .ok_or("usage: fn29_stills <directory>")?;
    let directory = Path::new(&directory);
    let subjects = [
        subject("oregon-white-oak", "oak", "leaf")?,
        subject("norway-spruce", "spruce", "needle")?,
    ];
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    std::fs::create_dir_all(directory).map_err(|e| e.to_string())?;
    let offset = Vec3::new(1.7307636095778745, 0.2967023330704928, -1.7307636095778745);
    for subject in subjects {
        let tree = mesh::build(&subject.family, Detail::Full).map_err(|e| e.to_string())?;
        renderer
            .submit_at(&tree, Level::Chosen)
            .map_err(|e| e.to_string())?;
        renderer.set_material(subject.family.material);
        renderer.set_view(View::Bare);
        let trunk = if subject.name == "oak" {
            camera(
                Vec3::new(1.7307636095778745, 2.2967023330704928, -1.7307636095778745),
                Vec3::new(0.0, 2.0, 0.0),
            )
        } else {
            let target = Vec3::new(0.0, 0.65, 0.0);
            camera(target + offset.normalized() * 1.8, target)
        };
        still(
            &mut renderer,
            directory,
            &subject,
            &format!("{}-trunk", subject.name),
            trunk,
            SceneRow::default(),
            None,
        )?;
        still(
            &mut renderer,
            directory,
            &subject,
            &format!("{}-branch", subject.name),
            subject.fork.camera,
            SceneRow::default(),
            Some(subject.fork),
        )?;
        renderer.set_view(View::Leaf);
        let leaf = hero_pose(
            renderer.bounds().ok_or("no leaf bounds")?,
            1.6,
            GROUND_REACH,
        );
        for (light, row) in [
            ("frontlit", r#"{"sunAzimuth":0,"sunElevation":10}"#),
            ("backlit", r#"{"sunAzimuth":180,"sunElevation":10}"#),
        ] {
            still(
                &mut renderer,
                directory,
                &subject,
                &format!("{}-{}-{light}", subject.name, subject.leaf),
                leaf,
                SceneRow::parse(row).map_err(|e| e.to_string())?,
                None,
            )?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
