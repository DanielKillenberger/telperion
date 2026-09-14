use telperion_core::{branching::Specimen, presets::Preset};
fn main() {
    for rate in [0.08, 0.09, 0.1, 0.12] {
        let mut f = Preset::NorwaySpruce.parameters();
        f.skeleton.seed = 7;
        f.growth.rate = rate;
        f.growth.shape = 2.0;
        f.growth.juvenile_radius = 1.0;
        f.growth.seedling_height = 0.0;
        f.growth.juvenile_height = 0.0;
        f.growth.seedling_radius = 0.0;
        f.growth.crown_base_retention = 0.0;
        f.age = 66.0;
        let mut s = Specimen::build(&f).unwrap();
        let record = s.advance(1.0).unwrap();
        println!("rate={rate} nodes={} moved={} born={} resized={}", s.tree().nodes.len(), record.moved_placements.len(), record.born_placements.len(),record.resized_runs.len());
    }
}
