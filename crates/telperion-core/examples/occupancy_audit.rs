//! Local descendant diagnostics. No botanical thresholds or visual acceptance.
use serde_json::json;
use telperion_core::{
    branching,
    foliage::{self, TwigPlacement},
    math::Vec3,
    presets::Preset,
    tree::NodeKind,
};
use telperion_core::{tree, Error, Result};
// Reuse the exact surface path ordering to select original mesh triangles.
#[path = "../src/surface/paths.rs"]
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
            let tree = branching::generate(&f.skeleton, f.radii).unwrap().tree;
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
            if preset == Preset::NorwaySpruce {
                let t = f.skeleton.twigs.resolved().unwrap();
                let placed = foliage::place_on_surface(
                    &tree,
                    f.skeleton.envelope,
                    seed,
                    f.canopy,
                    Some(TwigPlacement {
                        internode_length: t.twig.internode_length,
                        stations_per_internode: 1,
                    }),
                    &f.surface,
                )
                .unwrap();
                let element = foliage::build_element(f.element).unwrap();
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
                let mesh =
                    telperion_core::surface::build(&tree, f.skeleton.envelope.height, &f.surface)
                        .unwrap();
                let paths = wood_paths::paths(&tree.nodes).unwrap();
                let segments = f.surface.radial_segments.max(f.surface.lobes * 4) as usize;
                let mut index_offset = 0;
                let mut wood_triangles = Vec::new();
                for path in &paths.runs {
                    let ns = &paths.nodes[path.start..path.end];
                    let buried = usize::from(path.trunk && f.surface.flare_depth > 0.0);
                    let samples = ns.len() + buried;
                    for edge in 0..samples - 1 {
                        let child = ns[(edge + 1).saturating_sub(buried)];
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
                    index_offset += samples * segments * 6;
                }
                assert_eq!(index_offset, mesh.indices.len());
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
                    let count = (length / t.twig.internode_length - 1e-9).ceil().max(1.0) as usize;
                    if selected {
                        runs.push(json!({"nodes":run,"length_m":length,"first_instance":offset,"matrices":placed.matrices[offset..offset+count]}));
                    }
                    offset += count;
                }
                assert_eq!(offset, placed.matrices.len());
                data["curtain"] = json!({"root":root,"socket":tree.nodes[root].parent,"origin":xyz(tree.nodes[tree.nodes[root].parent.unwrap() as usize].position),"wood_triangles":wood_triangles,"runs":runs,"prototype":element.positions.iter().map(|&p|xyz(p)).collect::<Vec<_>>(),"needle_indices":element.indices[element.anatomy.unwrap().indices].to_vec(),"total_instances":placed.matrices.len()});
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
