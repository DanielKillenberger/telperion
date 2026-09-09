// The room: ground and figure, each carrying its own flat value, lit by the
// same hemisphere the subject stands under.

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
    @location(1) colour: vec3<f32>,
};

@vertex
fn vertex(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) colour: vec3<f32>,
) -> Varying {
    var out: Varying;
    out.clip = u.view_projection * vec4<f32>(position, 1.0);
    out.normal = normal;
    out.colour = colour;
    return out;
}

@fragment
fn fragment(in: Varying) -> @location(0) vec4<f32> {
    let n = normalize(in.normal);
    let hemisphere = mix(u.ground.rgb, u.sky.rgb, 0.5 + 0.5 * n.y);
    return vec4<f32>(in.colour * hemisphere, 1.0);
}
