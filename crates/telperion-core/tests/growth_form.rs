use telperion_core::{branching::Specimen, presets::Preset};

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
