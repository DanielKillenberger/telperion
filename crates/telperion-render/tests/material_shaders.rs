//! Validate the material stages even on hosts without a hardware adapter.
#[test]
fn production_material_stages_validate_with_uniform_derivatives() {
    use wgpu::naga::{
        front::wgsl,
        valid::{Capabilities, ValidationFlags, Validator},
    };
    let prelude = include_str!("../src/shaders/common.wgsl");
    let transmission = include_str!("../src/shaders/transmission.wgsl");
    let bark = [
        include_str!("../src/shaders/bark.wgsl"),
        include_str!("../src/shaders/plates.wgsl"),
        include_str!("../src/shaders/smooth.wgsl"),
    ]
    .join("\n");
    for (name, detail, stage) in [
        (
            "wood",
            bark.as_str(),
            include_str!("../src/shaders/wood.wgsl"),
        ),
        ("foliage", "", include_str!("../src/shaders/foliage.wgsl")),
    ] {
        let source = format!("{prelude}\n{transmission}\n{detail}\n{stage}");
        let module = wgsl::parse_str(&source)
            .unwrap_or_else(|e| panic!("{name}: {}", e.emit_to_string(&source)));
        Validator::new(ValidationFlags::all(), Capabilities::all())
            .validate(&module)
            .unwrap_or_else(|e| panic!("{name}: {}", e.emit_to_string(&source)));
    }
}
