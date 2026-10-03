//! The share of the walk tree's length lost to partly grown branches
//! (fn-192 RESULT.md), over the seeds 0 to 49 that grow a standing tree.
#[path = "../tests/walk/mod.rs"]
mod walk;
fn main() {
    let species = walk::species();
    let (mut full, mut drawn, mut born, mut axes) = (0.0, 0.0, 0, 0);
    for seed in 0..50 {
        let Ok(tree) = walk::tree(&species, seed) else {
            continue;
        };
        for axis in &tree.axes {
            axes += 1;
            born += usize::from(axis.vigour < 0.999);
            let internode = species.states[axis.pa].internode;
            for p in &axis.phytomers {
                full += internode;
                drawn += internode * p.scale;
            }
        }
    }
    println!(
        "GROW_IN {}: length lost {:.1}%, axes born partly grown {:.1}%",
        telperion_space::GROW_IN,
        100.0 * (1.0 - drawn / full),
        100.0 * born as f64 / axes as f64
    );
}
