//! fn-183 R1 exploration, not for merge: the ladder's rungs, chosen at run
//! time by `LADDER` (0 today, 1 no outline question, 2 inherited allowance,
//! 3 radial room at the start, 4 directional probe along the first heading).
use super::*;
use crate::envelope::queries::{self, Purpose};

pub(in crate::pipeline::branching) fn rung() -> u8 {
    static RUNG: std::sync::OnceLock<u8> = std::sync::OnceLock::new();
    *RUNG.get_or_init(|| {
        std::env::var("LADDER")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
    })
}

/// The extent an axis of `length` leaving `start` along `heading` keeps on a
/// rung that bounds extent. A dropping curtain keeps its own controls.
#[allow(clippy::too_many_arguments)]
pub(super) fn cap(
    config: &GrowthConfig,
    t: TwigParams,
    curtain: Curtain,
    bound: Bound,
    start: Vec3,
    heading: Vec3,
    length: f64,
) -> f64 {
    if curtain.drops(t) && std::env::var_os("LADDER_CURTAIN").is_none() {
        return length;
    }
    match rung() {
        2 => length.min(bound.left(start)).max(0.0),
        3 => length.min(radial(config, bound, start)).max(0.0),
        4 => probe(config, bound, start, heading.normalized(), length),
        _ => length,
    }
}

/// Horizontal depth of `p` inside its system's outline, in world metres.
fn radial(config: &GrowthConfig, bound: Bound, p: Vec3) -> f64 {
    let _asks = queries::during(Purpose::TwigRoom);
    let Some(shell) = config.shell else {
        return f64::INFINITY;
    };
    let q = bound.map(p);
    (shell.radius_toward(q, config.seed) - q.x.hypot_fixed(q.z)) * bound.scale()
}

/// The authored length if its end is inside, else the last quarter of it
/// that is, marching out from the start.
fn probe(config: &GrowthConfig, bound: Bound, start: Vec3, heading: Vec3, length: f64) -> f64 {
    let _asks = queries::during(Purpose::TwigRoom);
    let inside = |d: f64| !rejected(config, bound.map(start + heading * d));
    if inside(length) {
        return length;
    }
    for k in 1..4 {
        if !inside(length * k as f64 / 4.0) {
            return length * (k - 1) as f64 / 4.0;
        }
    }
    length * 0.75
}
