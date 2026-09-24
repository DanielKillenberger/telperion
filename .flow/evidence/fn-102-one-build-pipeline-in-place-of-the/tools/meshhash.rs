//! Hashes every byte `mesh::build` hands a renderer, per preset and seed.
//! With FAMILIES set it prints each build's family wire instead.
use serde_json::json;
use telperion_core::{mesh, params, presets::Preset};

struct Fnv(u64);
impl Fnv {
    fn new() -> Self {
        Self(14695981039346656037)
    }
    fn eat(&mut self, bytes: impl IntoIterator<Item = u8>) {
        for b in bytes {
            self.0 = (self.0 ^ b as u64).wrapping_mul(1099511628211);
        }
    }
}
fn main() {
    let ids: Vec<&str> = params::CATALOGUE.iter().chain(params::IN_WORK).map(|e| e.1).collect();
    let only: Option<String> = std::env::args().nth(1);
    for id in ids {
        if only.as_deref().is_some_and(|o| o != id) {
            continue;
        }
        for seed in [1u32, 7] {
            let mut f = params::by_identity(id)
                .unwrap_or_else(|_| Preset::from_id(id).unwrap().parameters());
            f.skeleton.seed = seed;
            if std::env::var_os("FAMILIES").is_some() {
                println!("{}", json!({"id": id, "seed": seed, "family": params::metadata(&f)}));
                continue;
            }
            let m = mesh::build(&f).unwrap();
            let mut h = Fnv::new();
            let w = &m.wood;
            h.eat(w.positions.iter().flat_map(|v| v.to_le_bytes()));
            h.eat(w.normals.iter().flat_map(|v| v.to_le_bytes()));
            h.eat(w.indices.iter().flat_map(|v| v.to_le_bytes()));
            h.eat(w.coords.iter().flat_map(|v| v.to_le_bytes()));
            h.eat(format!("{:?}{:?}{}{}", w.bounds, w.run_table, w.runs, w.dropped).into_bytes());
            let e = &m.foliage.element;
            let mut g = Fnv::new();
            g.eat(e.positions.iter().flat_map(|p| [p.x, p.y, p.z]).flat_map(|v| v.to_le_bytes()));
            g.eat(e.indices.iter().flat_map(|v| v.to_le_bytes()));
            g.eat(e.coords.iter().flat_map(|v| v.to_le_bytes()));
            g.eat(format!("{:?}", e.anatomy).into_bytes());
            let i = &m.foliage.instances;
            g.eat(i.leaves.iter().flat_map(|l| l.iter().flat_map(|w| w.to_le_bytes())));
            g.eat(format!("{:?}{}", i.reference, i.thinned).into_bytes());
            println!(
                "{}",
                json!({"id": id, "seed": seed, "wood": format!("{:016x}", h.0),
                "foliage": format!("{:016x}", g.0), "bounds": format!("{:?}", m.bounds),
                "vertices": m.wood_vertices(), "instances": m.foliage_instances()})
            );
        }
    }
}
