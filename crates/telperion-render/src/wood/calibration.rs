//! Evidence-only flat wood fixture using the complete production renderer.
use crate::{
    crop_mean, measure, measure_frame, render, write_png, Camera, Frame, Gpu, Renderer, SceneRow,
    View, STILL_FORMAT,
};
use serde_json::json;
use telperion_core::{
    foliage::{Element, Instances},
    math::Vec3,
    mesh::{Foliage, TreeMesh},
    presets::Preset,
    surface::{Bounds, SurfaceMesh, SurfaceRun},
};

const WIDTH: f64 = 0.4;
const RADIUS: f64 = 0.4;
const SIZE: u32 = 400;
const STRIPS: u32 = 128;

fn patch() -> TreeMesh {
    let bounds = Bounds {
        min: Vec3::new(-WIDTH / 2.0, 2.0 - WIDTH / 2.0, 0.0),
        max: Vec3::new(WIDTH / 2.0, 2.0 + WIDTH / 2.0, 0.0),
    };
    let mut wood = SurfaceMesh {
        bounds: Some(bounds),
        runs: 1,
        ..Default::default()
    };
    for y in [bounds.min.y, bounds.max.y] {
        for i in 0..=STRIPS {
            let x = -WIDTH / 2.0 + WIDTH * f64::from(i) / f64::from(STRIPS);
            wood.positions.extend([x as f32, y as f32, 0.0]);
            wood.normals.extend([0.0, 0.0, 1.0]);
            wood.coords.extend([y as f32, (x / RADIUS) as f32]);
        }
    }
    for i in 0..STRIPS {
        let top = i + STRIPS + 1;
        wood.indices.extend([i, i + 1, top, i + 1, top + 1, top]);
    }
    wood.run_table.push(SurfaceRun {
        first_index: 0,
        index_count: wood.indices.len() as u32,
        largest_radius: RADIUS,
    });
    TreeMesh {
        wood,
        bounds,
        foliage: Foliage {
            element: Element::default(),
            instances: Instances::default(),
        },
    }
}

#[test]
fn flat_patch_coordinates_have_the_stated_physical_scale() {
    let tree = patch();
    assert_eq!(tree.wood.indices.len(), STRIPS as usize * 6);
    for (p, uv) in tree
        .wood
        .positions
        .as_chunks::<3>()
        .0
        .iter()
        .zip(tree.wood.coords.as_chunks::<2>().0)
    {
        assert_eq!(p[2], 0.0);
        assert!((f64::from(uv[1]) * RADIUS - f64::from(p[0])).abs() < 1e-7);
        assert_eq!(p[1], uv[0]);
    }
    assert!(
        (f64::from(tree.wood.positions[STRIPS as usize * 3])
            - f64::from(tree.wood.positions[0])
            - WIDTH)
            .abs()
            < 1e-7
    );
}

#[test]
#[ignore = "explicit evidence capture; requires hardware GPU and BARK_CAPTURE_DIR"]
fn capture_calibrated_bark() {
    let directory = std::path::PathBuf::from(
        std::env::var_os("BARK_CAPTURE_DIR")
            .expect("BARK_CAPTURE_DIR must name the evidence output directory"),
    );
    std::fs::create_dir_all(&directory).unwrap();
    let gpu = pollster::block_on(Gpu::request(None)).expect("hardware GPU required for evidence");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let tree = patch();
    renderer.submit(&tree).unwrap();
    // The production uploader infers radius from complete rings. This fixture
    // is flat, so explicitly supplies its stated material radius after upload.
    let radius_buffer = renderer.wood.radii.as_ref().unwrap();
    renderer.gpu.queue.write_buffer(
        radius_buffer.buffer(),
        0,
        bytemuck::cast_slice(&vec![RADIUS as f32; tree.wood_vertices()]),
    );
    renderer.set_view(View::Bare);
    renderer.set_figure(false);
    let scene = SceneRow {
        sun_azimuth: 45.0,
        ..SceneRow::default()
    };
    renderer.set_scene(scene);
    let fov = 38.0_f64;
    let distance = WIDTH / (2.0 * (fov.to_radians() / 2.0).tan());
    let camera = Camera {
        position: Vec3::new(0.0, 2.0, distance),
        target: Vec3::new(0.0, 2.0, 0.0),
        field_of_view: fov,
        near: 0.01,
        far: 100.0,
    };
    let mut rows = Vec::new();
    for (name, preset) in [
        ("oak", Preset::OregonWhiteOak),
        ("spruce", Preset::NorwaySpruce),
    ] {
        let mut family = preset.parameters();
        family.skeleton.seed = 7;
        renderer.set_material(family.material);
        let still = render(&mut renderer, &camera, SIZE, SIZE).unwrap();
        assert_eq!((still.width, still.height), (SIZE, SIZE));
        assert!(still.has_subject());
        let again = render(&mut renderer, &camera, SIZE, SIZE).unwrap();
        assert_eq!(
            still.rgba, again.rgba,
            "identical capture must be deterministic"
        );
        let performance = if std::env::var("BARK_CAPTURE_TIMING").as_deref() == Ok("1") {
            let frame = Frame::new(&renderer, "bark calibration timing", (SIZE, SIZE));
            let report = measure(&mut renderer, &camera, (SIZE, SIZE), frame.target()).unwrap();
            std::fs::write(
                directory.join(format!("{name}-timing.json")),
                report.to_json(),
            )
            .unwrap();
            serde_json::from_str::<serde_json::Value>(&report.to_json()).unwrap()
        } else {
            json!({"status": "not_requested", "reason": "BARK_CAPTURE_TIMING=1 requires an idle GPU preflight"})
        };
        let mut off = family.material;
        off.depth_strength = 0.0;
        renderer.set_material(off);
        let without_depth = render(&mut renderer, &camera, SIZE, SIZE).unwrap();
        let difference = still
            .rgba
            .iter()
            .zip(&without_depth.rgba)
            .map(|(a, b)| f64::from(a.abs_diff(*b)))
            .sum::<f64>()
            / still.rgba.len() as f64;
        let structure = measure_frame(&still.rgba, SIZE as usize, SIZE as usize).unwrap();
        write_png(&directory.join(format!("{name}-flat.png")), &still).unwrap();
        rows.push(json!({
            "species": name, "preset": format!("{preset:?}"), "seed": 7,
            "seed_note": "preset seed recorded; synthetic surface does not invoke generation",
            "material_debug": format!("{:?}", family.material), "crop_width_metres": WIDTH,
            "crop_height_metres": WIDTH, "material_radius_metres": RADIUS,
            "image_width": SIZE, "image_height": SIZE, "grid_strips": STRIPS,
            "mean_rgb": crop_mean(&still.rgba, SIZE as usize, SIZE as usize).unwrap(),
            "depth_off_mean_absolute_rgba_difference": difference,
            "structure": {
                "orientation_entropy": structure.orientation_entropy,
                "area_variation": structure.area_variation,
                "furrow_curvature": structure.furrow_curvature,
                "junction_arms": structure.junction_arms,
                "dark_fraction": structure.dark_fraction,
                "furrow_period_pixels": structure.furrow_period
            },
            "performance": performance,
            "reference": {"status": "qualitative_replacement", "crop_width_metres": null,
                "colour_calibrated": false, "score": null,
                "page": if name == "oak" { "https://selectree.calpoly.edu/tree-detail/1240" }
                    else { "https://commons.wikimedia.org/wiki/File:Norway_Spruce_bark_detail.jpg" },
                "limitations": if name == "oak" { "650x567 source limits fine detail; unknown scale and lighting" }
                    else { "flash illumination; unknown physical scale" },
                "role": "visual morphology comparison; render structure metrics are diagnostics only"}
        }));
    }
    let record = json!({"fixture": "flat production wood; explicit test-only material radius",
        "camera": {"position": [0.0, 2.0, distance], "target": [0.0, 2.0, 0.0],
            "field_of_view_degrees": fov, "near": 0.01, "far": 100.0},
        "scene": serde_json::from_str::<serde_json::Value>(&scene.to_json()).unwrap(),
        "samples": renderer.samples(), "captures": rows,
        "timing_protocol": "fn26: one initial render, 8 conditioning, 8 warmup, 120 measured; per-species reports retain validity verdict"});
    std::fs::write(
        directory.join("calibration.json"),
        serde_json::to_string_pretty(&record).unwrap() + "\n",
    )
    .unwrap();
}
