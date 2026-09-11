// The sun's own pass: depth and nothing else. One entry point per subject,
// because the wood stands where the core put it and a leaf stands where its
// placement puts it; neither carries a normal, a coordinate or a colour here.

@group(0) @binding(0) var<uniform> light: mat4x4<f32>;

@vertex
fn wood(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
    return light * vec4<f32>(position, 1.0);
}

/// The crown's placements, as the core packed them. The sun's view is not the
/// camera's, so this draw takes every placement in order and the level list the
/// layout carries beside them is not consulted.
@group(1) @binding(0) var<storage, read> placements: array<mat4x4<f32>>;

@vertex
fn foliage(
    @builtin(instance_index) instance: u32,
    @location(0) position: vec3<f32>,
) -> @builtin(position) vec4<f32> {
    return light * placements[instance] * vec4<f32>(position, 1.0);
}
