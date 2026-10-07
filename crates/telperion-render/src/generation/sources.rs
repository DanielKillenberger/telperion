//! The generation passes' WGSL, each composed once here, so the passes
//! and the CPU-side parse (`shader_tests.rs`) read the same source.
const LEAF: &str = include_str!("../shaders/leaf.wgsl");

pub(super) fn place() -> String {
    format!(
        "{LEAF}\n{}\n{}\n{}\n{}",
        include_str!("order.wgsl"),
        include_str!("place.wgsl"),
        include_str!("contact.wgsl"),
        include_str!("pack.wgsl")
    )
}

pub(super) fn scatter() -> String {
    include_str!("scatter.wgsl").into()
}

pub(super) fn mass() -> String {
    format!("{LEAF}\n{}", include_str!("mass.wgsl"))
}

pub(super) fn positions() -> String {
    format!(
        "{}\n{}",
        include_str!("positions.wgsl"),
        include_str!("wood_geometry.wgsl")
    )
}

#[cfg(test)]
fn position_probe() -> String {
    include_str!("position_probe.wgsl").into()
}

/// Every pass's source by name, the fine-root executor's included.
#[cfg(test)]
pub(super) fn all() -> Vec<(&'static str, String)> {
    let mut out = vec![
        ("place", place()),
        ("scatter", scatter()),
        ("mass", mass()),
        ("positions", positions()),
        ("position probe", position_probe()),
    ];
    out.extend(super::fine::sources());
    out
}
