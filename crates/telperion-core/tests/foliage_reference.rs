//! Optional frozen-FN6 comparison. Export buffers as documented in tests/migration/README.md.
use std::{fs, path::Path};
use telperion_core::{
    envelope::Envelope,
    foliage::*,
    math::Vec3,
    tree::{Node, NodeKind, Tree},
};
fn bytes(dir: &Path, case: &str, name: &str) -> Vec<u8> {
    fs::read(dir.join(format!("{case}-{name}.bin"))).unwrap()
}
fn f64s(b: &[u8]) -> Vec<f64> {
    assert_eq!(b.len() % 8, 0);
    b.as_chunks::<8>()
        .0
        .iter()
        .map(|v| f64::from_le_bytes(*v))
        .collect()
}
fn number(json: &str, key: &str) -> f64 {
    json.split(&format!("\"{key}\":"))
        .nth(1)
        .unwrap()
        .trim_start()
        .split(|c: char| !(c.is_ascii_digit() || ".-+eE".contains(c)))
        .next()
        .unwrap()
        .parse()
        .unwrap()
}
#[test]
#[ignore = "requires REFERENCE_DIRECTORY exported with REFERENCE_BUFFERS=1"]
fn frozen_solved_tree_comparison() {
    let dir = std::env::var("REFERENCE_DIRECTORY").expect("REFERENCE_DIRECTORY required");
    let dir = Path::new(&dir);
    let cases = std::env::var("REFERENCE_CASES").unwrap_or("ordinary,telperion".into());
    for case in cases.split(',') {
        let json = fs::read_to_string(dir.join(format!("{case}.json"))).unwrap();
        assert!(json.contains("fdafb099b1495519de75a6b9a66d37f7d07e47bd"));
        let positions = f64s(&bytes(dir, case, "positions"));
        let radii = f64s(&bytes(dir, case, "radius"));
        let start = f64s(&bytes(dir, case, "start-radius"));
        let parents = bytes(dir, case, "parents");
        let twigs = bytes(dir, case, "twig");
        let crossover = radii.len() - twigs.len();
        let nodes = radii
            .iter()
            .enumerate()
            .map(|(i, &radius)| {
                let parent = i32::from_le_bytes(parents[i * 4..i * 4 + 4].try_into().unwrap());
                Node {
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
                    radius,
                    start_radius: start[i],
                    base_radius: radius,
                    branch: i as u32,
                    kind: if i >= crossover && twigs[i - crossover] == 1 {
                        NodeKind::Twig
                    } else {
                        NodeKind::Structural
                    },
                    ..Node::root()
                }
            })
            .collect();
        let tree = Tree {
            nodes,
            crossover,
            ..Tree::default()
        };
        let env = Envelope {
            height: number(&json, "height"),
            crown_base: number(&json, "crownBase"),
            spread: number(&json, "spread"),
            fullness: number(&json, "fullness"),
            shoulder: number(&json, "shoulder"),
            irregularity: 0.0,
            lobe_scale: 0.5,
        };
        let c = json.split("\"canopy\":").nth(1).unwrap();
        let params = CanopyParams {
            shoot_radius: number(c, "shootRadius"),
            spacing: number(c, "spacing"),
            divergence: number(c, "divergence"),
            clump: number(c, "clump") as u32,
            clump_span: number(c, "clumpSpan"),
            outward: number(c, "outward"),
            upward: number(c, "upward"),
            scatter: number(c, "scatter"),
            size: number(c, "size"),
            size_variation: number(c, "sizeVariation"),
            ..CanopyParams::default()
        };
        let anatomy = TwigPlacement {
            internode_length: number(&json, "internodeLength"),
            stations_per_internode: number(&json, "stationsPerInternode") as u32,
        };
        let element = build_element(ElementParams::default()).unwrap();
        // The frozen run predates the reference box, so one is spanned over
        // the tree it froze, grown by a metre for the stand-off a station
        // takes from the wood it sits on.
        let (lo, hi) = tree.nodes.iter().fold(
            (
                Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
                Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            ),
            |(lo, hi), n| {
                let p = n.position;
                (
                    Vec3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z)),
                    Vec3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z)),
                )
            },
        );
        let box_of = Reference::spanning(lo - Vec3::new(1., 1., 1.), hi + Vec3::new(1., 1., 1.));
        let placed = place(
            &tree,
            env,
            number(&json, "seed") as u32,
            params,
            Some(anatomy),
            box_of,
        )
        .unwrap();
        let placed_count = placed.len();
        let kept = cull(placed, &element, env, 0.45).unwrap();
        let bounds = kept.bounds(&element).unwrap();
        let old = bytes(dir, case, "foliage");
        assert_eq!(old.len() % 64, 0);
        assert_eq!(
            kept.len(),
            old.len() / 64,
            "{case}: retained membership/count drift"
        );
        let mut max_error = 0f64;
        for (a, b) in kept.matrices().flatten().zip(old.as_chunks::<4>().0.iter()) {
            let b = f32::from_le_bytes(*b);
            max_error = max_error.max((a as f64 - b as f64).abs());
        }
        // Quantisation is the floor now, not float drift: half a position
        // code on the widest axis, and a rotation read back through
        // ten-bit smallest-three components, whose worst column entry moves
        // four code steps at the leaf's own scale.
        let step = box_of.step();
        let scale = kept
            .leaves
            .iter()
            .map(|&leaf| box_of.scale(leaf))
            .fold(0., f64::max);
        let code = 2. * std::f64::consts::FRAC_1_SQRT_2 / 1023.;
        let tolerance = (step.x.max(step.y).max(step.z) / 2.).max(4. * code * scale);
        assert!(max_error <= tolerance, "{case}: matrix error {max_error}");
        let mut frozen = Instances::new(box_of);
        for m in old.as_chunks::<64>().0 {
            frozen.push(&std::array::from_fn(|i| {
                f32::from_le_bytes(m[i * 4..i * 4 + 4].try_into().unwrap())
            }));
        }
        assert_eq!(
            bounds,
            frozen.bounds(&element).unwrap(),
            "{case}: bounds drift"
        );
        println!(
            "{case}: placed={}, retained={}, max_matrix_error={max_error}, bounds={bounds:?}",
            placed_count,
            kept.len()
        );
    }
}
