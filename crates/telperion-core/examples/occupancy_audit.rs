//! Local descendant diagnostics. No botanical thresholds or visual acceptance.
use serde_json::json;
use telperion_core::{math, tree, Error, Result};
use telperion_core::{
    math::Vec3,
    pipeline::{self, Request},
    presets::Preset,
    tree::NodeKind,
};
// Recover path membership; mesh spans come from the radius-ordered run table.
// The audit reads which nodes a run sweeps, not where its girth eases.
#[path = "../src/pipeline/surface/paths.rs"]
#[allow(dead_code)]
mod wood_paths;
fn reserved<T>(n: usize) -> telperion_core::Result<Vec<T>> {
    let mut v = Vec::new();
    v.try_reserve(n)
        .map_err(|_| telperion_core::Error::ResourceLimit("audit"))?;
    Ok(v)
}
fn filled<T: Clone>(n: usize, x: T) -> telperion_core::Result<Vec<T>> {
    let mut v = reserved(n)?;
    v.resize(n, x);
    Ok(v)
}
fn xyz(p: Vec3) -> [f64; 3] {
    [p.x, p.y, p.z]
}
fn main() {
    let output = std::env::args().nth(1).expect("output directory");
    std::fs::create_dir_all(&output).unwrap();
    for (preset, seeds) in [
        (
            Preset::OregonWhiteOak,
            vec![1, 2, 3, 2666899686, 762807349, 1444323199],
        ),
        (
            Preset::NorwaySpruce,
            vec![1, 2, 3, 1982700925, 281313742, 2271779095, 4250668600],
        ),
    ] {
        for seed in seeds {
            let mut f = preset.parameters();
            f.skeleton.seed = seed;
            let spruce = preset == Preset::NorwaySpruce;
            if spruce {
                // One station an internode, and a shell that keeps every
                // leaf: the spruce's placement as it stood before the cull.
                f.skeleton.twigs.twig.stations_per_internode = 1;
                f.shell_depth = 1.0;
            }
            let request = if spruce {
                Request::mesh()
            } else {
                Request::default()
            };
            let built = pipeline::build(&f, request).unwrap();
            let (tree, mut outputs) = (built.skeleton.tree, built.outputs);
            let mut owner = vec![0; tree.nodes.len()];
            let mut points = vec![Vec::new(); tree.crossover];
            for i in 0..tree.nodes.len() {
                if i < tree.crossover {
                    owner[i] = i;
                } else {
                    owner[i] = owner[tree.nodes[i].parent.unwrap() as usize];
                    if tree.nodes[i].kind == NodeKind::Twig {
                        points[owner[i]].push(xyz(tree.nodes[i].position));
                    }
                }
            }
            let upper = tree.nodes[..tree.crossover]
                .iter()
                .map(|n| n.position.y)
                .fold(0.0, f64::max)
                * 0.75;
            let supports:Vec<_>=(1..tree.crossover).filter(|&i|!points[i].is_empty()&&(preset!=Preset::OregonWhiteOak||tree.nodes[i].position.y>=upper)).map(|i|json!({"node":i,"parent":tree.nodes[i].parent,"position":xyz(tree.nodes[i].position),"twig_endpoints":points[i]})).collect();
            let mut data = json!({"preset":preset.profile_id(),"seed":seed,"nodes":tree.nodes.len(),"crossover":tree.crossover,"upper_m":upper,"supports":supports});
            data["structure"] = json!(tree
                .nodes
                .iter()
                .map(|n| json!([
                    n.position.x,
                    n.position.y,
                    n.position.z,
                    n.parent,
                    n.branch,
                    format!("{:?}", n.kind)
                ]))
                .collect::<Vec<_>>());
            if spruce {
                let placed = outputs.leaves.take().unwrap().instances;
                let element = outputs.element.take().unwrap();
                // Reconstruct placement run order to retain original instance IDs and matrices.
                let bearing = |i: usize| {
                    let n = &tree.nodes[i];
                    n.parent.is_some()
                        && (n.kind == NodeKind::Twig
                            || n.radius.max(n.start_radius)
                                <= tree.nodes[0].radius * f.canopy.shoot_radius)
                };
                let mut children = vec![Vec::new(); tree.nodes.len()];
                for i in 1..tree.nodes.len() {
                    if bearing(i) {
                        children[tree.nodes[i].parent.unwrap() as usize].push(i);
                    }
                }
                let continues = |p: usize, c: usize| {
                    bearing(p)
                        && tree.nodes[p].branch == tree.nodes[c].branch
                        && children[p].len() == 1
                };
                let outward = Vec3::new(0.62, 0.0, 1.0).normalized();
                let root = (1..tree.crossover)
                    .filter(|&i| {
                        let n = &tree.nodes[i];
                        let p = &tree.nodes[n.parent.unwrap() as usize];
                        let d = (n.position - p.position).normalized();
                        d.y < -0.7 && p.position.y > 5.25 && p.position.y < 9.75
                    })
                    .max_by(|&a, &b| {
                        tree.nodes[tree.nodes[a].parent.unwrap() as usize]
                            .position
                            .dot(outward)
                            .total_cmp(
                                &tree.nodes[tree.nodes[b].parent.unwrap() as usize]
                                    .position
                                    .dot(outward),
                            )
                    })
                    .unwrap();
                let mut in_system = vec![false; tree.nodes.len()];
                for i in 1..tree.nodes.len() {
                    in_system[i] = i == root || in_system[tree.nodes[i].parent.unwrap() as usize];
                }
                // Reattachment can move a local run above the sampled edge.
                // Pin original supporting node IDs rather than silently comparing
                // a smaller new subtree with a misleading coverage percentage.
                let pinned = std::env::args().nth(2).map(|directory| {
                    let bytes =
                        std::fs::read(format!("{directory}/norway-spruce-{seed}.json")).unwrap();
                    let previous: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
                    assert_eq!(previous["curtain"]["root"].as_u64().unwrap() as usize, root);
                    in_system.fill(false);
                    for run in previous["curtain"]["runs"].as_array().unwrap() {
                        for node in run["nodes"].as_array().unwrap().iter().skip(1) {
                            in_system[node.as_u64().unwrap() as usize] = true;
                        }
                    }
                    directory
                });
                let mesh = outputs.wood.take().unwrap();
                let paths = wood_paths::paths(&tree.nodes).unwrap();
                let segments = f.surface.radial_segments.max(f.surface.lobes * 4) as usize;
                let key = |p: Vec3| [p.x as f32, p.y as f32, p.z as f32].map(f32::to_bits);
                let mut by_tip = std::collections::HashMap::new();
                for path in &paths.runs {
                    let tip = paths.nodes[path.end - 1];
                    assert!(
                        by_tip.insert(key(tree.nodes[tip].position), path).is_none(),
                        "wood paths must have distinct tips for the audit"
                    );
                }
                let mut wood_triangles = Vec::new();
                for span in &mesh.run_table {
                    let index_offset = span.first_index as usize;
                    let end = index_offset + span.index_count as usize;
                    // The final cap's centre is the path's terminal node. Match
                    // that identity, never the old path ordinal or just its length.
                    let top = mesh.indices[end - 3] as usize * 3;
                    let tip = mesh.positions[top..top + 3].try_into().unwrap();
                    let path = by_tip
                        .remove(&<[f32; 3]>::map(tip, f32::to_bits))
                        .expect("wood run cap must identify exactly one path");
                    let ns = &paths.nodes[path.start..path.end];
                    let buried = usize::from(path.trunk && f.surface.flare_depth > 0.0);
                    let samples = ns.len() + buried;
                    assert_eq!(
                        span.index_count as usize,
                        samples * segments * 6,
                        "wood run must cover this path's rings and caps"
                    );
                    let base = mesh.indices[index_offset] as usize;
                    for edge in 0..samples - 1 {
                        let child = ns[(edge + 1).saturating_sub(buried)];
                        let ring = base + (edge + 1) * segments;
                        let centre = (ring..ring + segments).fold(Vec3::ZERO, |sum, v| {
                            let p = &mesh.positions[v * 3..v * 3 + 3];
                            sum + Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)
                        }) / segments as f64;
                        assert!(
                            centre.distance(tree.nodes[child].position) < 2e-5,
                            "wood run ring belongs to another path"
                        );
                        if in_system[child] {
                            for tri in mesh.indices[index_offset + edge * segments * 6
                                ..index_offset + (edge + 1) * segments * 6]
                                .as_chunks::<3>()
                                .0
                            {
                                wood_triangles.push(
                                    tri.iter()
                                        .map(|&v| {
                                            mesh.positions[v as usize * 3..v as usize * 3 + 3]
                                                .to_vec()
                                        })
                                        .collect::<Vec<_>>(),
                                );
                            }
                        }
                    }
                }
                assert!(by_tip.is_empty(), "every path must have a wood run");
                let mut offset = 0;
                let mut runs = Vec::new();
                for (i, &selected) in in_system.iter().enumerate().skip(1) {
                    if !bearing(i) {
                        continue;
                    }
                    let parent = tree.nodes[i].parent.unwrap() as usize;
                    if continues(parent, i) {
                        continue;
                    }
                    let mut run = vec![parent, i];
                    let mut at = i;
                    while children[at].len() == 1 && continues(at, children[at][0]) {
                        at = children[at][0];
                        run.push(at);
                    }
                    let length: f64 = run
                        .windows(2)
                        .map(|w| {
                            tree.nodes[w[0]]
                                .position
                                .distance(tree.nodes[w[1]].position)
                        })
                        .sum();
                    let count = (length / f.skeleton.twigs.twig.internode_length - 1e-9)
                        .ceil()
                        .max(1.0) as usize;
                    if selected {
                        runs.push(json!({"nodes":run,"length_m":length,"first_instance":offset,"matrices":placed.matrices().skip(offset).take(count).collect::<Vec<_>>()}));
                    }
                    offset += count;
                }
                assert_eq!(offset, placed.len());
                data["curtain"] = json!({"root":root,"socket":tree.nodes[root].parent,"origin":xyz(tree.nodes[tree.nodes[root].parent.unwrap() as usize].position),"pinned_reference":pinned,"wood_triangles":wood_triangles,"runs":runs,"prototype":element.positions.iter().map(|&p|xyz(p)).collect::<Vec<_>>(),"needle_indices":element.indices[element.anatomy.unwrap().indices].to_vec(),"total_instances":placed.len()});
            }
            std::fs::write(
                format!("{output}/{}-{seed}.json", preset.profile_id().unwrap()),
                serde_json::to_vec(&data).unwrap(),
            )
            .unwrap();
            println!("{} {seed}", preset.profile_id().unwrap());
        }
    }
}
