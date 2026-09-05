use std::{fs, path::Path};
use telperion_core::{
    math::Vec3,
    surface::{build, SurfaceParams},
    tree::{Node, Tree},
};
fn f64s(dir: &Path, id: &str, suffix: &str) -> Vec<f64> {
    fs::read(dir.join(format!("{id}-{suffix}.bin")))
        .unwrap()
        .as_chunks::<8>()
        .0
        .iter()
        .map(|b| f64::from_le_bytes(*b))
        .collect()
}
fn f32s(dir: &Path, id: &str, suffix: &str) -> Vec<f32> {
    fs::read(dir.join(format!("{id}-{suffix}.bin")))
        .unwrap()
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| f32::from_le_bytes(*b))
        .collect()
}
#[test]
#[ignore = "run node crates/telperion-core/tests/surface_reference.mjs to export pinned FN6 inputs"]
fn final_fn6_geometry_comparison() {
    let directory = std::env::var("SURFACE_REFERENCE").expect("reference directory");
    let dir = Path::new(&directory);
    for id in std::env::var("SURFACE_CASES").unwrap().split(',') {
        let config = fs::read_to_string(dir.join(format!("{id}-surface-config.txt"))).unwrap();
        let c: Vec<f64> = config
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let params = SurfaceParams {
            radial_segments: c[1] as u32,
            lobes: c[2] as u32,
            lobe_depth: c[3],
            twist_rate: c[4],
            flare_radius: c[5],
            flare_falloff: c[6],
            flare_depth: c[7],
            fork_socket: c[8],
            fork_swell: c[9],
        };
        let positions = f64s(dir, id, "positions");
        let radius = f64s(dir, id, "radius");
        let start = f64s(dir, id, "start-radius");
        let parents: Vec<i32> = fs::read(dir.join(format!("{id}-parents.bin")))
            .unwrap()
            .as_chunks::<4>()
            .0
            .iter()
            .map(|b| i32::from_le_bytes(*b))
            .collect();
        let tree = Tree {
            nodes: parents
                .iter()
                .enumerate()
                .map(|(i, &parent)| Node {
                    position: Vec3::new(
                        positions[i * 3],
                        positions[i * 3 + 1],
                        positions[i * 3 + 2],
                    ),
                    parent: if parent < 0 {
                        None
                    } else {
                        Some(parent as u32)
                    },
                    radius: radius[i],
                    start_radius: start[i],
                    ..Node::root()
                })
                .collect(),
            ..Tree::default()
        };
        let mesh = build(&tree, c[0], &params).unwrap_or_else(|e| panic!("{id}: {e}"));
        let reference = f32s(dir, id, "surface-positions");
        let normals = f32s(dir, id, "surface-normals");
        let indices: Vec<u32> = fs::read(dir.join(format!("{id}-surface-indices.bin")))
            .unwrap()
            .as_chunks::<4>()
            .0
            .iter()
            .map(|b| u32::from_le_bytes(*b))
            .collect();
        assert_eq!(mesh.positions.len(), reference.len(), "{id} vertex count");
        assert_eq!(mesh.indices, indices, "{id} connectivity");
        let error = mesh
            .positions
            .iter()
            .zip(&reference)
            .map(|(&a, &b)| (a as f64 - b as f64).abs())
            .fold(0.0, f64::max);
        let normal_error = mesh
            .normals
            .iter()
            .zip(&normals)
            .map(|(&a, &b)| (a as f64 - b as f64).abs())
            .fold(0.0, f64::max);
        assert!(
            error <= 8.0 * f32::EPSILON as f64 * c[0].max(1.0),
            "{id} position error {error}"
        );
        assert!(normal_error <= 1e-5, "{id} normal error {normal_error}");
        if let Some(bounds) = mesh.bounds {
            for (k, (lo, hi)) in [
                (bounds.min.x, bounds.max.x),
                (bounds.min.y, bounds.max.y),
                (bounds.min.z, bounds.max.z),
            ]
            .into_iter()
            .enumerate()
            {
                let values = reference.as_chunks::<3>().0.iter().map(|p| p[k] as f64);
                assert!((lo - values.clone().fold(f64::INFINITY, f64::min)).abs() <= error);
                assert!((hi - values.fold(f64::NEG_INFINITY, f64::max)).abs() <= error);
            }
        } else {
            assert!(reference.is_empty());
        }
        println!("{id}: {} vertices, {} triangles; max position error {error}, normal error {normal_error}",mesh.positions.len()/3,mesh.indices.len()/3);
    }
}
