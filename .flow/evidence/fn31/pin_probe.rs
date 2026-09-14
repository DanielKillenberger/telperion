use telperion_core::{
    branching::Specimen,
    mesh::{self, Detail},
    presets::Preset,
};

/// FNV-1a over the bytes, the pattern the branching audit already pins with.
fn fnv(bytes: impl IntoIterator<Item = u8>) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for byte in bytes {
        hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
    }
    hash
}

fn main() {
    for id in ["oregon-white-oak", "norway-spruce"] {
        let mut family = Preset::from_id(id).unwrap().parameters();
        family.skeleton.seed = 7;
        let tree = Specimen::build(&family).unwrap().read().unwrap().tree;
        let skeleton = fnv(tree.nodes.iter().skip(1).flat_map(|n| {
            [n.position.x, n.position.y, n.position.z].into_iter().flat_map(f64::to_le_bytes)
                .chain(n.parent.unwrap().to_le_bytes())
        }));
        let m = mesh::build(&family, Detail::Full).unwrap();
        let placement = fnv(m.foliage.instances.matrices.iter().flatten().flat_map(|v| v.to_le_bytes()));
        let e = &m.foliage.element;
        let element = fnv(e.positions.iter().flat_map(|p| [p.x,p.y,p.z]).flat_map(f64::to_le_bytes)
            .chain(e.indices.iter().flat_map(|i| i.to_le_bytes())));
        println!("{}", serde_json::json!({"id":id,"wood_vertices":m.wood_vertices(),
            "wood_triangles":m.wood_triangles(),"instances":m.foliage_instances(),
            "min":[m.bounds.min.x,m.bounds.min.y,m.bounds.min.z],
            "max":[m.bounds.max.x,m.bounds.max.y,m.bounds.max.z],
            "skeleton":skeleton,"placement":placement,"element":element}));
    }
}
