use telperion_core::{branching::Specimen, presets::Preset};
fn main() {
    for mode in 0..5 {
        let mut f = Preset::OregonWhiteOak.parameters();
        f.skeleton.seed = 7;
        match mode {
            1 => f.growth.juvenile_radius = 1.,
            2 => f.growth.crown_base_retention = 0.,
            3 => f.growth.juvenile_height = 0.,
            4 => {
                f.growth.juvenile_radius = 1.;
                f.growth.crown_base_retention = 0.;
                f.growth.juvenile_height = 0.;
            }
            _ => {}
        }
        let s = Specimen::build(&f).unwrap();
        println!(
            "{mode} nodes={} cross={}",
            s.tree().nodes.len(),
            s.tree().crossover
        );
    }
}
