use super::*;
#[test]
fn reach_probe_budget_changes_room_in_a_wide_crown() {
    let e = Envelope {
        spread: 4.,
        ..Default::default()
    };
    let config = default_growth(e, 0, DEFAULT_STEP);
    let bias = GrowthBias::new(e, 1, BiasParams::NONE).unwrap();
    let mut tree = Tree::default();
    let mut builder = Builder {
        tree: &mut tree,
        envelope: e,
        planning: e,
        config: &config,
        bias: &bias,
        habit: HabitParams::default(),
        points: &[],
        consumed: &mut [],
        year: 0,
        influence_sq: 0.,
        kill_sq: 0.,
        point_scale: 1.,
        growing_envelope: false,
        paused: false,
        limbs: &mut Limbs::default(),
    };
    let origin = Vec3::new(
        0.,
        e.height * (e.crown_base + (1. - e.crown_base) * e.fullness),
        0.,
    );
    let first = builder.reach(origin, Vec3::X);
    builder.habit.reach_probe_steps = 512;
    let extended = builder.reach(origin, Vec3::X);
    assert!(extended > first * 2.);
    assert!(extended <= e.max_radius());
}
