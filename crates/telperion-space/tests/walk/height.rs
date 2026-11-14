//! The tallest living tip of a grown tree above its root collar (fn-349,
//! DEVICE-ANSWERS.md 1; fn-363): the unit a walk over the inverse height
//! is measured in, as `sag_bend` measures sag's.
use telperion_space::Structure;

/// The height, in metres, of the tallest living tip above the root
/// collar's base: each living axis's last node, or its base where it has
/// none, as the balance charges it.
pub fn tallest_tip(tree: &Structure) -> f64 {
    let collar = tree.axes[0].base.z;
    let living = tree.axes.iter().filter(|a| a.apex_end.is_none());
    let tips = living.map(|a| a.phytomers.last().map_or(a.base, |p| p.tip).z);
    tips.fold(0.0, |most, z| most.max(z - collar))
}
