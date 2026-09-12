use super::*;
/// Append local branches to a solved structural crown. Cap diagnostics survive shedding.
pub fn append(
    tree: &mut Tree,
    config: &GrowthConfig,
    params: TwigParams,
    seed: u32,
    bias: Option<&GrowthBias>,
    habit: HabitParams,
) -> Result<()> {
    tree.validate_solved()?;
    config.validate()?;
    let t = params.resolved()?;
    if tree.crossover != tree.nodes.len() {
        return Err(Error::InvalidInput("branching requires a structural crown"));
    }
    // Standalone callers can supply a manually authored, unidentified crown.
    let mut frontier = Frontier::default();
    let mut identified = tree.clone();
    for (i, n) in identified.nodes.iter_mut().enumerate() {
        n.identity.birth = i as u64;
    }
    frontier.seed(&identified, config, t, habit);
    frontier.advance(
        tree,
        Planner {
            growing_envelope: false,
            planning: None,
            config,
            bias,
            twigs: t,
            crookedness: habit.crookedness,
            seed,
        },
        habit,
        usize::MAX,
    )
}
