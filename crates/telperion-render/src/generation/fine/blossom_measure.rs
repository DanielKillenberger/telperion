//! fn-234 R1, where blossom hides at the whole-tree view (`STILLS=<dir>
//! cargo test --profile ci -p telperion-render blossom_measure -- --ignored
//! --nocapture --test-threads 1`, under the GPU lock). Three flowering
//! trees from fn-217 task 3 (`tests/bloom_form.rs`): the cherry-blossom
//! beech, the cherry-dense beech and Laurelin's horn racemes at scale 3.
//! Each is measured for:
//! - where its florets stand: the shader's depth in the crown's ellipsoid
//!   (`depth_in_crown`), and metres under the leaves' outer shell along the
//!   ray from the crown's centre;
//! - the share the hero camera loses to the leaves, overall and by depth
//!   band, from the organ mask with and without the leaves;
//! - the petals' drawn colour, unmixed from the whole still against the
//!   same tree without florets and the mask, beside the close-up petal;
//! - organ coverage on the CPU path (`mesh::build`) and the resident device
//!   path (`prepare` with `Delivery::Resident`).
use super::super::{Delivery, Generator};
use super::blossom_trees::{cherry, dense, laurelin, positions, quantiles, Crown};
use crate::{hero_pose, render, Camera, Level, Renderer, SceneRow, Still, View, GROUND_REACH};
use std::path::{Path, PathBuf};
use telperion_core::math::Vec3;
use telperion_core::mesh::TreeMesh;
use telperion_core::pipeline::executor;
use telperion_core::presets::Family;

const SIZE: (u32, u32) = (960, 720);

pub(super) fn linear(v: u8) -> f64 {
    let c = f64::from(v) / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

pub(super) fn srgb(c: f64) -> f64 {
    let c = c.clamp(0.0, 1.0);
    255.0
        * if c <= 0.0031308 {
            12.92 * c
        } else {
            1.055 * c.powf(1.0 / 2.4) - 0.055
        }
}

/// The share of the frame the mask covers.
pub(super) fn coverage(mask: &Still) -> f64 {
    let n = mask.rgba.len() / 4;
    mask.rgba.chunks(4).map(|p| linear(p[1])).sum::<f64>() / n as f64
}

/// The petals' drawn colour (display-linear, then sRGB) where the mask
/// covers at least `least` of a pixel, unmixed from `with` against
/// `without`: P = (C - (1 - c) B) / c, weighted by c. And how many pixels.
fn petal_colour(with: &Still, without: &Still, mask: &Still, least: f64) -> ([f64; 3], usize) {
    let (mut sum, mut weight, mut n) = ([0.0; 3], 0.0, 0);
    for k in 0..mask.rgba.len() / 4 {
        let c = linear(mask.rgba[k * 4 + 1]);
        if c < least {
            continue;
        }
        for ch in 0..3 {
            let (a, b) = (
                linear(with.rgba[k * 4 + ch]),
                linear(without.rgba[k * 4 + ch]),
            );
            sum[ch] += a - (1.0 - c) * b;
        }
        (weight, n) = (weight + c, n + 1);
    }
    (sum.map(|s| srgb(s / weight.max(1e-12))), n)
}

struct Shooter {
    renderer: Renderer,
    camera: Camera,
    dir: PathBuf,
    name: &'static str,
    level: Level,
}

impl Shooter {
    fn draw(&mut self, view: View, mask: bool, size: (u32, u32), file: Option<&str>) -> Still {
        self.renderer.set_view(view);
        self.renderer.set_organ_mask(mask);
        let still = render(&mut self.renderer, &self.camera, size.0, size.1).unwrap();
        self.renderer.set_organ_mask(false);
        if let Some(file) = file {
            let path = self.dir.join(format!("{}-{file}.png", self.name));
            crate::write_png(Path::new(&path), &still).unwrap();
        }
        still
    }

    fn submit(&mut self, mesh: &TreeMesh, family: &Family) {
        self.renderer.submit_at(mesh, self.level).unwrap();
        self.renderer.set_family(family);
    }

    fn mask(&mut self, mesh: &TreeMesh, family: &Family, file: Option<&str>) -> Still {
        self.submit(mesh, family);
        self.draw(View::Clay, true, SIZE, file)
    }

    fn whole(&mut self, mesh: &TreeMesh, family: &Family, file: Option<&str>) -> Still {
        self.submit(mesh, family);
        self.draw(View::Whole, false, SIZE, file)
    }
}

/// A camera `distance` out from the outermost whole floret on the hero
/// camera's side half-way up the crown (fn-217 `at_a_bloom`).
fn at_a_bloom(mesh: &TreeMesh, hero: &Camera, distance: f64) -> Camera {
    let toward = Vec3::new(hero.position.x, 0.0, hero.position.z).normalized();
    let height = mesh.bounds.max.y - mesh.bounds.min.y;
    let organ = mesh.organ.as_ref().unwrap();
    let r = organ.instances.reference;
    let floret = (organ.instances.leaves.iter())
        .filter(|l| r.scale(**l) > 0.6)
        .map(|l| r.position(*l))
        .filter(|p| (0.45..0.8).contains(&(p.y / height)))
        .max_by(|a, b| a.dot(toward).total_cmp(&b.dot(toward)))
        .unwrap();
    Camera {
        position: floret + toward * distance + Vec3::Y * (0.2 * distance),
        target: floret,
        field_of_view: 38.0,
        near: 0.005,
        far: 500.0,
    }
}

fn measure(gpu: &crate::Gpu, name: &'static str, family: &Family, dir: &Path) {
    let mut mesh = telperion_core::mesh::build(family).unwrap();
    let leaves = positions(&mesh.foliage.instances);
    let all = mesh.organ.as_ref().unwrap().instances.clone();
    let florets = positions(&all);
    let crown = Crown::of(&leaves);
    println!(
        "\n== {name}: {} florets, {} leaves",
        florets.len(),
        leaves.len()
    );
    println!("crown box centre {:?}, half {:?}", crown.centre, crown.half);
    let qs = [0.1, 0.25, 0.5, 0.75, 0.9];
    for (what, set) in [("florets", &florets), ("leaves", &leaves)] {
        let mut e: Vec<f64> = set.iter().map(|p| crown.ellipsoid(*p)).collect();
        let mut u: Vec<f64> = (set.iter().map(|p| crown.under(*p)))
            .filter(|d| d.is_finite())
            .collect();
        let near = |m: f64| u.iter().filter(|d| **d <= m).count() as f64 / u.len() as f64;
        let (n25, n50, n100) = (near(0.25), near(0.5), near(1.0));
        println!(
            "{what}: ellipsoid depth q10/25/50/75/90 {:.3?}; metres under shell {:.2?}; \
             within 0.25/0.5/1 m of the shell {n25:.3}/{n50:.3}/{n100:.3}",
            quantiles(&mut e, &qs),
            quantiles(&mut u, &qs),
        );
    }
    let hero = hero_pose(
        mesh.bounds,
        f64::from(SIZE.0) / f64::from(SIZE.1),
        GROUND_REACH,
    );
    let scene = SceneRow {
        sun_azimuth: 115.0,
        sun_elevation: 40.0,
        ..SceneRow::default()
    };
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    renderer.set_scene(scene);
    let mut s = Shooter {
        renderer,
        camera: hero,
        dir: dir.to_path_buf(),
        name,
        level: Level::Chosen,
    };
    // Bands by metres under the shell (quartiles), and the camera's side.
    let mut depths: Vec<f64> = florets.iter().map(|p| crown.under(*p)).collect();
    let edges = quantiles(&mut depths.clone(), &[0.25, 0.5, 0.75]);
    depths = florets.iter().map(|p| crown.under(*p)).collect();
    let toward = (hero.position - crown.centre).normalized();
    // The sun's direction as `shadow/fit.rs` takes it, at 115 and 40 degrees.
    let (az, el) = (115f64.to_radians(), 40f64.to_radians());
    let sun = Vec3::new(el.cos() * az.sin(), el.sin(), el.cos() * az.cos());
    let bands: Vec<(String, Vec<bool>)> = {
        let band = |lo: f64, hi: f64| depths.iter().map(|d| *d >= lo && *d < hi).collect();
        vec![
            ("all".into(), vec![true; florets.len()]),
            (
                format!("under <{:.2} m", edges[0]),
                band(f64::MIN, edges[0]),
            ),
            (
                format!("{:.2}-{:.2} m", edges[0], edges[1]),
                band(edges[0], edges[1]),
            ),
            (
                format!("{:.2}-{:.2} m", edges[1], edges[2]),
                band(edges[1], edges[2]),
            ),
            (format!(">{:.2} m", edges[2]), band(edges[2], f64::MAX)),
            (
                "camera side".into(),
                florets
                    .iter()
                    .map(|p| (*p - crown.centre).dot(toward) > 0.0)
                    .collect(),
            ),
            (
                "far side".into(),
                florets
                    .iter()
                    .map(|p| (*p - crown.centre).dot(toward) <= 0.0)
                    .collect(),
            ),
            (
                "camera side, outer quartile".into(),
                (florets.iter().zip(&depths))
                    .map(|(p, d)| (*p - crown.centre).dot(toward) > 0.0 && *d < edges[0])
                    .collect(),
            ),
            (
                "sun side".into(),
                florets
                    .iter()
                    .map(|p| (*p - crown.centre).dot(sun) > 0.0)
                    .collect(),
            ),
            (
                "sun and camera side, outer quartile".into(),
                (florets.iter().zip(&depths))
                    .map(|(p, d)| {
                        let o = *p - crown.centre;
                        o.dot(sun) > 0.0 && o.dot(toward) > 0.0 && *d < edges[0]
                    })
                    .collect(),
            ),
        ]
    };
    let keep = |m: &mut TreeMesh, on: &[bool]| {
        let organ = m.organ.as_mut().unwrap();
        organ.instances.leaves = (all.leaves.iter().zip(on))
            .filter(|(_, k)| **k)
            .map(|(l, _)| *l)
            .collect();
    };
    // The whole stills and the full mask, with and without the florets.
    let full_mask = s.mask(&mesh, family, Some("cpu-mask"));
    let with = s.whole(&mesh, family, Some("cpu-whole"));
    let organ = mesh.organ.take();
    let without = s.whole(&mesh, family, Some("cpu-bare-of-florets"));
    mesh.organ = organ;
    let leaf_store = mesh.foliage.instances.leaves.clone();
    for (label, on) in &bands {
        keep(&mut mesh, on);
        let shown = coverage(&s.mask(&mesh, family, None));
        let lit = s.whole(&mesh, family, None);
        let band_mask = s.mask(&mesh, family, None);
        let (petal, px) = petal_colour(&lit, &without, &band_mask, 0.5);
        mesh.foliage.instances.leaves.clear();
        let open = coverage(&s.mask(&mesh, family, None));
        mesh.foliage.instances.leaves = leaf_store.clone();
        println!(
            "band {label}: {} florets; coverage with leaves {shown:.5}, without leaves \
             {open:.5}; occluded {:.3}; petal sRGB {petal:.0?} over {px} pixels of c>=0.5",
            on.iter().filter(|k| **k).count(),
            1.0 - shown / open.max(1e-12),
        );
    }
    keep(&mut mesh, &vec![true; florets.len()]);
    for least in [0.25, 0.5, 0.9] {
        let (p, n) = petal_colour(&with, &without, &full_mask, least);
        println!("petals at c>={least}: sRGB {p:.0?} over {n} pixels");
    }
    println!("cpu coverage {:.5}", coverage(&full_mask));
    // The crown's silhouette: the leaves drawn in the organ's place.
    let leaves_as_organ = telperion_core::mesh::Foliage {
        element: mesh.foliage.element.clone(),
        instances: mesh.foliage.instances.clone(),
    };
    let organ = mesh.organ.replace(leaves_as_organ);
    mesh.foliage.instances.leaves.clear();
    let silhouette = s.mask(&mesh, family, Some("cpu-leaf-silhouette"));
    let crown_share = coverage(&silhouette);
    let leaf_px: Vec<usize> = (0..silhouette.rgba.len() / 4)
        .filter(|k| linear(silhouette.rgba[k * 4 + 1]) >= 0.9)
        .collect();
    let mean = |ch: usize| {
        let sum: f64 = leaf_px
            .iter()
            .map(|k| linear(without.rgba[k * 4 + ch]))
            .sum();
        srgb(sum / leaf_px.len().max(1) as f64)
    };
    println!(
        "leaves drawn: sRGB [{:.0}, {:.0}, {:.0}] over {} pixels of c>=0.9",
        mean(0),
        mean(1),
        mean(2),
        leaf_px.len()
    );
    mesh.organ = organ;
    mesh.foliage.instances.leaves = leaf_store;
    println!(
        "leaf silhouette {crown_share:.5}; florets shown / leaf silhouette {:.3}",
        coverage(&full_mask) / crown_share
    );
    lighting(&mut s, &mut mesh, family);
    // The petal close up, from 0.1 m, in the whole view.
    let mut close = Shooter {
        camera: at_a_bloom(&mesh, &hero, 0.1),
        ..s
    };
    for (k, file) in [(1.0, Some("close")), (4.0, None)] {
        let mut f = family.clone();
        bright(&mut f.material, k);
        let lit = close.whole(&mesh, &f, file);
        let m = close.mask(&mesh, &f, None);
        let organ = mesh.organ.take();
        let bare = close.whole(&mesh, &f, None);
        mesh.organ = organ;
        let (p, n) = petal_colour(&lit, &bare, &m, 0.99);
        println!("close petal (0.1 m), bloom x{k}: sRGB {p:.0?} over {n} pixels");
    }
    let s = Shooter {
        camera: hero,
        ..close
    };
    device(gpu, s, family, &mesh);
}

/// The petals' drawn colour at the hero camera with each of the leaf
/// shader's crown terms the organ block keeps turned off, and at brighter
/// petal values: which term makes them grey.
fn lighting(s: &mut Shooter, mesh: &mut TreeMesh, family: &Family) {
    type Set = fn(&mut telperion_core::material::MaterialParams);
    let variants: [(&str, Set); 8] = [
        ("as drawn", |_| {}),
        ("interiorDarkening 0", |m| m.interior_darkening = 0.0),
        ("crownShade 0", |m| m.crown_shade = 0.0),
        ("lobeShade 0", |m| m.lobe_shade = 0.0),
        ("transmission 0", |m| {
            (m.transmission_strength, m.diffuse_transmission) = (0.0, 0.0)
        }),
        ("all four 0", |m| {
            (m.interior_darkening, m.crown_shade, m.lobe_shade) = (0.0, 0.0, 0.0);
            (m.transmission_strength, m.diffuse_transmission) = (0.0, 0.0);
        }),
        ("bloom x2", |m| bright(m, 2.0)),
        ("bloom x4", |m| bright(m, 4.0)),
    ];
    for (label, set) in variants {
        let mut f = family.clone();
        set(&mut f.material);
        let mask = s.mask(mesh, &f, None);
        let with = s.whole(mesh, &f, None);
        let organ = mesh.organ.take();
        let without = s.whole(mesh, &f, None);
        mesh.organ = organ;
        let (p, n) = petal_colour(&with, &without, &mask, 0.5);
        println!("lighting {label}: petal sRGB {p:.0?} over {n} pixels");
    }
    // Every element at its finest level (levels run coarsest first).
    s.level = Level::Forced(1_000);
    let mask = s.mask(mesh, family, None);
    let with = s.whole(mesh, family, None);
    let organ = mesh.organ.take();
    let without = s.whole(mesh, family, None);
    mesh.organ = organ;
    let (p, n) = petal_colour(&with, &without, &mask, 0.5);
    println!(
        "lighting finest levels: petal sRGB {p:.0?} over {n} pixels; coverage {:.5}",
        coverage(&mask)
    );
    s.level = Level::Chosen;
}

fn bright(m: &mut telperion_core::material::MaterialParams, k: f64) {
    m.bloom_front_red = (m.bloom_front_red * k).min(1.0);
    m.bloom_front_green = (m.bloom_front_green * k).min(1.0);
    m.bloom_front_blue = (m.bloom_front_blue * k).min(1.0);
    m.bloom_back_red = (m.bloom_back_red * k).min(1.0);
    m.bloom_back_green = (m.bloom_back_green * k).min(1.0);
    m.bloom_back_blue = (m.bloom_back_blue * k).min(1.0);
}

/// The same tree on the resident device path, from the same camera.
fn device(gpu: &crate::Gpu, s: Shooter, family: &Family, cpu: &TreeMesh) {
    drop(s.renderer);
    let mut s = Shooter {
        renderer: Renderer::new(gpu.clone(), crate::STILL_FORMAT),
        ..s
    };
    s.renderer.set_scene(SceneRow {
        sun_azimuth: 115.0,
        sun_elevation: 40.0,
        ..SceneRow::default()
    });
    println!("executor::apart: {:?}", executor::apart(family).err());
    let gen = Generator::new(&s.renderer).unwrap();
    let prepared = gen.prepare(family, Delivery::Resident).unwrap();
    let held = prepared
        .mesh
        .organ
        .as_ref()
        .map_or(0, |o| o.instances.len());
    let placed = prepared.organ.as_ref().map_or(0, |o| o.count as usize);
    let cpu_florets = cpu.organ.as_ref().map_or(0, |o| o.instances.len());
    println!(
        "device: {held} florets held + {placed} placed on the device = {} (cpu {cpu_florets}); \
         backend {:?}; bounds {:?} (cpu {:?})",
        held + placed,
        prepared.backend,
        prepared.bounds(),
        cpu.bounds,
    );
    s.renderer.submit_prepared(prepared).unwrap();
    s.renderer.set_family(family);
    let mask = s.draw(View::Clay, true, SIZE, Some("device-mask"));
    s.draw(View::Whole, false, SIZE, Some("device-whole"));
    println!("device coverage {:.5}", coverage(&mask));
}

#[test]
#[ignore]
fn blossom_measure() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let dir = PathBuf::from(std::env::var("STILLS").expect("STILLS: where the stills go"));
    std::fs::create_dir_all(&dir).unwrap();
    let only = std::env::var("PROBE").unwrap_or_default();
    for (name, family) in [
        ("cherry", cherry as fn() -> Family),
        ("dense", dense),
        ("laurelin", laurelin),
    ] {
        if only.is_empty() || only == name {
            measure(&gpu, name, &family(), &dir);
        }
    }
}
