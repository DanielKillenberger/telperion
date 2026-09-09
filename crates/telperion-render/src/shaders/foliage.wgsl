// The crown: the same flat clay and the same hemisphere as the wood, so a leaf
// and the branch it sits on are read under one light. The placement arrives as
// four instance columns, straight from the core's own matrices.

struct Uniforms {
    view_projection: mat4x4<f32>,
    sky: vec4<f32>,
    ground: vec4<f32>,
    clay: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: Uniforms;

struct Varying {
    @builtin(position) clip: vec4<f32>,
    @location(0) normal: vec3<f32>,
};

@vertex
fn vertex(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) column0: vec4<f32>,
    @location(3) column1: vec4<f32>,
    @location(4) column2: vec4<f32>,
    @location(5) column3: vec4<f32>,
) -> Varying {
    let placement = mat4x4<f32>(column0, column1, column2, column3);
    var out: Varying;
    out.clip = u.view_projection * placement * vec4<f32>(position, 1.0);
    out.normal = (placement * vec4<f32>(normal, 0.0)).xyz;
    return out;
}

@fragment
fn fragment(in: Varying, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // A leaf has no back. Nothing is culled, so the face the eye sees takes the
    // light; without the flip half the crown would read as holes.
    let n = normalize(select(-in.normal, in.normal, front));
    let hemisphere = mix(u.ground.rgb, u.sky.rgb, 0.5 + 0.5 * n.y);
    return vec4<f32>(u.clay.rgb * hemisphere, 1.0);
}
