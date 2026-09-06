//! Branch-scale retained foliage-support audit; lengths are metres, bounds are centreline bounds.
use serde_json::json;
use telperion_core::{branching, presets::Preset, tree::NodeKind};
fn main() {
    for seed in [1, 2, 3, 1982700925, 281313742, 2271779095, 4250668600] {
        let mut f = Preset::NorwaySpruce.parameters();
        f.skeleton.seed = seed;
        let report = branching::generate(&f.skeleton, f.radii).unwrap();
        let tree = report.tree;
        let mut owners = vec![None; tree.nodes.len()];
        let mut systems = Vec::new();
        let mut lengths: Vec<[f64; 2]> = Vec::new();
        let mut bounds: Vec<[[f64; 3]; 2]> = Vec::new();
        for (i, n) in tree.nodes.iter().enumerate().skip(1) {
            let p = n.parent.unwrap() as usize;
            let parent = &tree.nodes[p];
            let d = (n.position - parent.position).normalized();
            owners[i] = owners[p];
            if i < tree.crossover && d.y < -0.5 && owners[i].is_none() {
                owners[i] = Some(systems.len());
                systems.push(json!({"root":i,"socket":p,"position":[parent.position.x,parent.position.y,parent.position.z]}));
                lengths.push([0.0; 2]);
                bounds.push([[f64::INFINITY; 3], [f64::NEG_INFINITY; 3]]);
            }
            if let Some(owner) = owners[i] {
                let bearing = n.kind == NodeKind::Twig
                    || n.radius.max(n.start_radius) <= tree.nodes[0].radius * f.canopy.shoot_radius;
                lengths[owner][usize::from(bearing)] += n.position.distance(parent.position);
                if bearing {
                    for point in [parent.position, n.position] {
                        for (k, v) in [point.x, point.y, point.z].into_iter().enumerate() {
                            bounds[owner][0][k] = bounds[owner][0][k].min(v);
                            bounds[owner][1][k] = bounds[owner][1][k].max(v);
                        }
                    }
                }
            }
        }
        for i in 0..systems.len() {
            let b = bounds[i];
            let overlaps = bounds
                .iter()
                .enumerate()
                .filter(|(j, c)| {
                    *j != i && (0..3).all(|k| b[0][k] <= c[1][k] && c[0][k] <= b[1][k])
                })
                .count();
            systems[i]["bare_m"] = json!(lengths[i][0]);
            systems[i]["bearing_m"] = json!(lengths[i][1]);
            systems[i]["bearing_bounds"] = json!(b);
            systems[i]["overlapping_bearing_bounds"] = json!(overlaps);
        }
        println!(
            "{}",
            json!({"seed":seed,"nodes":tree.nodes.len(),"systems":systems,"bare_m":lengths.iter().map(|v|v[0]).sum::<f64>(),"bearing_m":lengths.iter().map(|v|v[1]).sum::<f64>(),"note":"Eligibility matches radial needle placement; length is support length, not needle length. AABB overlap is contextual, not visual acceptance."})
        );
    }
}
