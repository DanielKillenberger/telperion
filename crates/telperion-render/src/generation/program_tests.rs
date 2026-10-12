//! fn-256 task 4 (Astra): natively a module's pipelines are made side by
//! side on threads of their own, and error scopes are a thread's own, so
//! a pipeline the device refuses on a worker is caught there and reported
//! by `programs::ready` by name, never raised as an uncaptured error.
use crate::programs::{ready, Program};

#[test]
fn a_program_refused_on_a_worker_is_reported_by_name() {
    let Some(gpu) = super::test_gpu() else {
        return;
    };
    let module = gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("one entry"),
        source: wgpu::ShaderSource::Wgsl("@compute @workgroup_size(1) fn real() {}".into()),
    });
    let desc = |label, entry| wgpu::ComputePipelineDescriptor {
        label: Some(label),
        layout: None,
        module: &module,
        entry_point: Some(entry),
        compilation_options: Default::default(),
        cache: None,
    };
    let made = Program::make_many(&gpu, &[desc("made", "real"), desc("refused", "absent")]);
    assert_eq!(made.len(), 2);
    let said = pollster::block_on(ready(&gpu)).expect_err("the absent entry is refused");
    assert!(said.to_string().contains("refused"), "{said}");
    pollster::block_on(ready(&gpu)).expect("a refusal is reported once");
}

/// Astra round 2: what a module's worker makes besides its pipelines (its
/// shader module, layouts) is caught on that worker too, and reported.
#[test]
fn a_shader_refused_on_a_side_by_side_worker_is_reported() {
    let Some(gpu) = super::test_gpu() else {
        return;
    };
    let bad = || {
        let source = wgpu::ShaderSource::Wgsl("@compute @workgroup_size(1) fn broken() { let x: u32 = 1.5; }".into());
        drop(gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("broken"), source }))
    };
    super::io::pair(&gpu, || (), bad);
    let said = pollster::block_on(ready(&gpu)).expect_err("the broken shader is refused");
    assert!(said.to_string().contains("beside another"), "{said}");
}
