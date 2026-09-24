use std::io::Write;
use telperion_core::{branching, presets::Preset};
fn main() {
    let mut out = std::io::BufWriter::new(std::io::stdout());
    for id in ["ordinary","oregon-white-oak","norway-spruce","silver-birch","telperion","laurelin","european-beech","date-palm"] {
        for seed in [1u32, 2, 3, 5, 7, 8, 13, 21, 34, 55, 89, 144, 233] {
            let mut f = Preset::from_id(id).unwrap().parameters();
            f.skeleton.seed = seed;
            let tree = branching::generate(&f.skeleton, f.radii).unwrap().tree;
            write!(out, "{id} {seed} {}", tree.nodes.len()).unwrap();
            for n in &tree.nodes { write!(out, " {:?},{:?},{:?},{:?},{:?}", n.position.x, n.position.y, n.position.z, n.radius, n.parent).unwrap(); }
            writeln!(out).unwrap();
        }
    }
}
