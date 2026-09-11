// The bark: the material row's colour and roughness over the plaited surface,
// under the sun it stands in and the sky it stands beneath. In the clay room it
// is the one flat value it has always been, so form can still be judged with no
// material over it.

struct Varying {
    @builtin(position) clip: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) world: vec3<f32>,
};

@vertex
// The surface coordinate rides along declared and unread: bark is drawn along
// it in the spec that follows this one.
fn vertex(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) coord: vec2<f32>,
) -> Varying {
    var out: Varying;
    out.clip = u.view_projection * vec4<f32>(position, 1.0);
    out.normal = normal;
    out.world = position;
    return out;
}

@fragment
fn fragment(in: Varying) -> @location(0) vec4<f32> {
    let n = normalize(in.normal);
    if (is_clay()) {
        return vec4<f32>(u.clay.rgb * clay_light(n), 1.0);
    }
    let sun = key(n, in.world);
    // Roughness is what a surface does with the sun it does not scatter: chalk
    // spreads it over the whole face, a smooth young bark keeps a narrow sheen
    // along the light. One lobe, no second light - the sun is the only thing
    // bright enough to glance off a trunk.
    let gloss = 1.0 - u.bark.w;
    let half_way = normalize(normalize(u.eye.xyz - in.world) + u.sun_direction.xyz);
    let sheen = gloss * pow(max(dot(n, half_way), 0.0), exp2(1.0 + 10.0 * gloss));
    let lit = u.bark.rgb * (ambient(n) + sun) + sun * sheen;
    return vec4<f32>(tone(lit), 1.0);
}
