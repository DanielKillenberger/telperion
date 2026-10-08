//! The generation passes' WGSL, each composed once here, so the passes
//! and the CPU-side parse (`shader_tests.rs`) read the same source.
const LEAF: &str = include_str!("../shaders/leaf.wgsl");

/// The placement, its leaves' and its florets' (fn-229), which draw by
/// the contract's keys.
pub(super) fn place() -> String {
    format!(
        "{LEAF}\n{}\n{}\n{}\n{}\n{}\n{}",
        super::fine::contract(),
        bloom(),
        include_str!("order.wgsl"),
        include_str!("place.wgsl"),
        include_str!("contact.wgsl"),
        include_str!("pack.wgsl")
    )
}

/// The florets as `bloom.rs` draws them (fn-229): the placement places
/// them and the fine stations count them by the same source.
pub(super) fn bloom() -> &'static str {
    include_str!("bloom.wgsl")
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

/// Every pass's source by name, the fine-root executor's included.
#[cfg(test)]
pub(super) fn all() -> Vec<(&'static str, String)> {
    let mut out = vec![
        ("place", place()),
        ("mass", mass()),
        ("positions", positions()),
    ];
    out.extend(super::fine::sources());
    out
}
