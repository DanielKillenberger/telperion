use telperion_core::{branching::Specimen, presets::Preset};

fn seedling(preset: Preset, range: std::ops::RangeInclusive<f64>) {
    let mut family = preset.parameters();
    family.skeleton.seed = 7;
    family.age = 1.0;
    let s = Specimen::build(&family).unwrap();
    let height = s.tree().nodes.iter().map(|n| n.position.y).fold(0.0, f64::max);
    assert!(range.contains(&height), "{preset:?} year-one height {height} m");
    assert!(!s.placements().unwrap().is_empty(), "{preset:?} year-one foliage missing");
}

#[test]
fn oak_starts_as_a_leafed_seedling() { seedling(Preset::OregonWhiteOak, 0.1..=0.5); }

#[test]
fn spruce_starts_as_a_needled_seedling() { seedling(Preset::NorwaySpruce, 0.01..=0.12); }

fn sapling(preset: Preset, age: f64) {
    let mut family = preset.parameters();
    family.skeleton.seed = 7;
    family.age = age;
    let s = Specimen::build(&family).unwrap();
    let laterals = s.tree().nodes.iter().filter(|n|
        n.shoot.bud_fate == telperion_core::tree::BudFate::Lateral).count();
    assert!(laterals >= 2, "{preset:?} age {age}: only {laterals} lateral shoots");
    assert!(s.placements().unwrap().len() >= 25, "{preset:?} age {age}: unleafed sapling");
}

#[test]
fn young_oak_branches_and_carries_foliage() { sapling(Preset::OregonWhiteOak, 10.0); }

#[test]
fn young_spruce_branches_and_carries_foliage() { sapling(Preset::NorwaySpruce, 5.0); }

