use telperion_core::{branching::Specimen, presets::Preset};

fn seedling(preset: Preset, range: std::ops::RangeInclusive<f64>) {
    let mut family = preset.parameters();
    family.skeleton.seed = 7;
    family.age = 1.0;
    let s = Specimen::build(&family).unwrap();
    let height = s
        .tree()
        .nodes
        .iter()
        .map(|n| n.position.y)
        .fold(0.0, f64::max);
    assert!(
        range.contains(&height),
        "{preset:?} year-one height {height} m"
    );
    assert!(
        !s.placements().unwrap().is_empty(),
        "{preset:?} year-one foliage missing"
    );
}

#[test]
fn oak_starts_as_a_leafed_seedling() {
    seedling(Preset::OregonWhiteOak, 0.1..=0.5);
}

#[test]
fn spruce_starts_as_a_needled_seedling() {
    seedling(Preset::NorwaySpruce, 0.01..=0.12);
}

fn sapling(preset: Preset, age: f64) {
    let mut family = preset.parameters();
    family.skeleton.seed = 7;
    family.age = age;
    let s = Specimen::build(&family).unwrap();
    let laterals = s
        .tree()
        .nodes
        .iter()
        .filter(|n| n.shoot.bud_fate == telperion_core::tree::BudFate::Lateral)
        .count();
    assert!(
        laterals >= 2,
        "{preset:?} age {age}: only {laterals} lateral shoots"
    );
    assert!(
        s.placements().unwrap().len() >= 25,
        "{preset:?} age {age}: unleafed sapling"
    );
}

#[test]
fn young_oak_branches_and_carries_foliage() {
    sapling(Preset::OregonWhiteOak, 10.0);
}

#[test]
fn young_spruce_branches_and_carries_foliage() {
    sapling(Preset::NorwaySpruce, 5.0);
}

#[test]
fn expanding_oak_crown_keeps_leafed_shoots() {
    let mut family = Preset::OregonWhiteOak.parameters();
    family.skeleton.seed = 7;
    family.age = 26.7;
    let s = Specimen::build(&family).unwrap();
    assert!(
        s.placements().unwrap().len() >= 2_500,
        "expanding oak crown lost its juvenile foliage"
    );
}

#[test]
fn mature_bole_does_not_end_at_a_temporary_local_apex() {
    let mut family = Preset::OregonWhiteOak.parameters();
    family.skeleton.seed = 7;
    let specimen = Specimen::build(&family).unwrap();
    let tree = specimen.tree();
    let mut at = 0;
    let mut height: f64 = 0.0;
    while let Some((i, child)) = tree
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| n.parent == Some(at as u32))
        .max_by(|(_, a), (_, b)| a.start_radius.total_cmp(&b.start_radius))
    {
        height = height.max(child.position.y);
        at = i;
    }
    assert!(
        height >= family.skeleton.envelope.height * family.skeleton.envelope.crown_base,
        "thickest bole ends at {height} m below the mature crown"
    );
}

#[test]
fn height_to_trunk_diameter_falls_with_age() {
    for (preset, ages) in [
        (Preset::OregonWhiteOak, [26.7, 56.1, 112.0]),
        (Preset::NorwaySpruce, [14.1, 26.6, 36.9]),
    ] {
        let mut previous = f64::INFINITY;
        for age in ages {
            let mut family = preset.parameters();
            family.skeleton.seed = 7;
            family.age = age;
            let s = Specimen::build(&family).unwrap();
            let ratio = s.envelope().height / (2.0 * s.tree().nodes[0].radius);
            assert!(
                ratio < previous - 0.1,
                "{preset:?} age {age}: H/root diameter {ratio} did not fall from {previous}"
            );
            previous = ratio;
        }
    }
}
