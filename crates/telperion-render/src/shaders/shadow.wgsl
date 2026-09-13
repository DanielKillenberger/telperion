// The sun's own pass: depth and nothing else. One entry point per subject,
// because the wood stands where the core put it and a leaf stands where its
// placement puts it. The light also carries a fixed stride and surface scale.

struct Light {
    matrix: mat4x4<f32>,
    caster: vec4<f32>, // stride, square-root scale, padding
    centre: vec4<f32>, // surface centre, first connector vertex
};
@group(0) @binding(0) var<uniform> light: Light;

@vertex
fn wood(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
    return light.matrix * vec4<f32>(position, 1.0);
}

/// The crown's placements, as the core packed them. The sun's view is not the
/// camera's: instance i reads placement i * stride, regardless of selection.
/// The surface scales about its centre; trailing connector vertices stay put.
/// Reordering placements changes this subset, and future motion must match it.
@group(1) @binding(0) var<storage, read> placements: array<mat4x4<f32>>;

@vertex
fn foliage(
    @builtin(instance_index) instance: u32,
    @builtin(vertex_index) vertex: u32,
    @location(0) position: vec3<f32>,
) -> @builtin(position) vec4<f32> {
    let expanded = light.centre.xyz + (position - light.centre.xyz) * light.caster.y;
    let local = select(position, expanded, vertex < u32(light.centre.w));
    return light.matrix * placements[instance * u32(light.caster.x)] * vec4<f32>(local, 1.0);
}
