// The ground and the figure, under the same sky and the same sun the subject
// stands in, taking the subject's shadow where the map throws it across the
// floor. The floor takes the scene row's ground colour; the figure keeps the
// value it was built with, because a scale reference is not part of the
// weather. In the clay room both keep their own neutral values.

struct Varying {
    @builtin(position) clip: vec4<f32>,
    @location(0) normal: vec3<f32>,
    /// The vertex's own value, and in `w` how much of it is the floor: the
    /// ground disc is painted by the scene row, the figure by nothing.
    @location(1) colour: vec4<f32>,
    @location(2) world: vec3<f32>,
};

@vertex
fn vertex(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) colour: vec4<f32>,
) -> Varying {
    var out: Varying;
    out.clip = u.view_projection * vec4<f32>(position, 1.0);
    out.normal = normal;
    out.colour = colour;
    out.world = position;
    return out;
}

@fragment
fn fragment(in: Varying) -> @location(0) vec4<f32> {
    let n = normalize(in.normal);
    if (is_clay()) {
        return vec4<f32>(in.colour.rgb * clay_light(n), 1.0);
    }
    let albedo = mix(in.colour.rgb, u.ground_colour.rgb, in.colour.a);
    return vec4<f32>(tone(albedo * (ambient(n) + key(n, in.world))), 1.0);
}
