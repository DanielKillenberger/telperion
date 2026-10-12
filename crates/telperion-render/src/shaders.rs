//! The drawing passes' WGSL, each composed once here with its defines, so
//! the device and the CPU-side check (`shader_tests.rs`) read the same
//! source. WGSL has no include of its own: a lit shader is the prelude, the
//! transmission and canopy terms, then its own stages.
use crate::device::Gpu;

const PRELUDE: &str = include_str!("shaders/common.wgsl");
const TRANSMISSION: &str = include_str!("shaders/transmission.wgsl");
const CANOPY: &str = include_str!("shaders/canopy.wgsl");
const LEAF: &str = include_str!("shaders/leaf.wgsl");
/// An organ's inflorescence as one unit and its rachis's axis (fn-253),
/// which selection and the organ's draw both read.
const UNIT: &str = include_str!("shaders/unit.wgsl");

/// The switch the wood stages are built on, as the shader states it.
const SMOOTH_ON: &str = "const SMOOTH_BARK: bool = true;";
const SMOOTH_OFF: &str = "const SMOOTH_BARK: bool = false;";

/// A module of this composed source.
pub(crate) fn module(gpu: &Gpu, label: &str, source: String) -> wgpu::ShaderModule {
    gpu.device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
}

fn lit(stages: &str) -> String {
    format!("{PRELUDE}\n{TRANSMISSION}\n{CANOPY}\n{stages}")
}

pub(crate) fn scene() -> String {
    lit(include_str!("shaders/scene.wgsl"))
}

pub(crate) fn sky() -> String {
    lit(include_str!("shaders/sky.wgsl"))
}

/// The crown's stage decodes a placement, so the decoder stands in front,
/// then the organ's unit and its stalks (fn-253).
pub(crate) fn foliage() -> String {
    let stalk = include_str!("shaders/stalk.wgsl");
    lit(&format!("{LEAF}\n{UNIT}\n{stalk}\n{}", include_str!("shaders/foliage.wgsl")))
}

/// The wood's stages, with smooth bark's terms or without them.
pub(crate) fn wood(smooth: bool) -> String {
    let stages = format!(
        "{}\n{}\n{}\n{}",
        include_str!("shaders/bark.wgsl"),
        include_str!("shaders/plates.wgsl"),
        include_str!("shaders/smooth.wgsl"),
        include_str!("shaders/wood.wgsl")
    );
    assert!(
        stages.contains(SMOOTH_ON),
        "the wood stages lost their switch"
    );
    if smooth {
        lit(&stages)
    } else {
        lit(&stages.replace(SMOOTH_ON, SMOOTH_OFF))
    }
}

pub(crate) fn shadow() -> String {
    format!("{LEAF}\n{}", include_str!("shaders/shadow.wgsl"))
}

pub(crate) fn select() -> String {
    format!("{LEAF}\n{UNIT}\n{}", include_str!("shaders/select.wgsl"))
}

pub(crate) fn curve() -> String {
    format!(
        "{}\n{}\n{}",
        include_str!("shaders/curve.wgsl"),
        include_str!("shaders/curve_walk.wgsl"),
        include_str!("shaders/curve_emit.wgsl")
    )
}

/// Every drawing pass's source by name.
#[cfg(test)]
pub(crate) fn all() -> Vec<(&'static str, String)> {
    vec![
        ("scene", scene()),
        ("sky", sky()),
        ("foliage", foliage()),
        ("wood", wood(false)),
        ("smooth wood", wood(true)),
        ("shadow", shadow()),
        ("select", select()),
        ("curve", curve()),
    ]
}
