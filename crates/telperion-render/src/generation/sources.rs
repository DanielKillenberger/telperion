//! The generation passes' WGSL, each composed once here, so the passes
//! and the CPU-side parse (`crate::shader_tests`) read the same source.
const LEAF: &str = include_str!("../shaders/leaf.wgsl");

/// The placement, its leaves' and its florets' (fn-229), which draw by
/// the contract's keys.
pub(super) fn place() -> String {
    format!(
        "{LEAF}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        super::cycle::contract(),
        bloom(),
        include_str!("order.wgsl"),
        include_str!("place.wgsl"),
        include_str!("contact.wgsl"),
        rachis(),
        include_str!("pack.wgsl")
    )
}

/// The florets as `bloom.rs` draws them (fn-229): the placement places
/// them and the resident stations count them by the same source.
pub(super) fn bloom() -> &'static str {
    include_str!("bloom.wgsl")
}

/// The frond curve, one function for every module that draws a frond:
/// the leaves' placement, the fronds' axes and the field's leaflet boxes.
pub(super) fn rachis() -> &'static str {
    include_str!("rachis.wgsl")
}

pub(super) fn mass() -> String {
    format!("{LEAF}\n{}", include_str!("mass.wgsl"))
}

/// The position programme: production's `emit`, and under test the
/// comparison route's admission entries and the wood geometry they read.
pub(super) fn positions() -> String {
    let emit = include_str!("positions.wgsl");
    #[cfg(test)]
    let emit = format!(
        "{emit}\n{}\n{}",
        include_str!("positions_admit.wgsl"),
        include_str!("wood_geometry.wgsl")
    );
    emit.to_string()
}

/// Every pass's source by name, the resident build's and the cycle's included.
#[cfg(test)]
pub(crate) fn all() -> Vec<(&'static str, String)> {
    let mut out = vec![
        ("place", place()),
        ("mass", mass()),
        ("positions", positions()),
        // The module production makes, `emit` alone.
        ("positions emit", include_str!("positions.wgsl").to_string()),
    ];
    out.extend(super::resident::sources());
    out.extend(super::cycle::sources());
    out
}
