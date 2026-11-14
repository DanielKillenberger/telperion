//! fn-198 task 4 scratch probe: phytomers above the cut and in fine roots.
use telperion_space::{grow_fine, Light, Plan, Request};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let species = match args[1].as_str() {
        "beech" => telperion_space::beech(),
        "spruce" => telperion_space::spruce(),
        _ => panic!(),
    };
    for age in args[2..].iter().map(|a| a.parse::<u32>().unwrap()) {
        let f = grow_fine(
            &species,
            Request {
                age,
                seed: 1,
                budget: 40_000_000,
                light: Light::default(),
            },
            Plan::default(),
        )
        .unwrap();
        let above: usize = f.structure.axes[..f.above]
            .iter()
            .map(|a| a.phytomers.len())
            .sum();
        let all: usize = f.structure.axes.iter().map(|a| a.phytomers.len()).sum();
        println!(
            "{} {age}: axes {} (above {}), phytomers {all} (above {above}, fine {})",
            args[1],
            f.structure.axes.len(),
            f.above,
            all - above
        );
    }
}
