// The bark: the material row's colour and roughness over the plaited surface,
// under the sun it stands in and the sky it stands beneath. In the clay room it
// is the one flat value it has always been, so form can still be judged with no
// material over it.

struct Varying {
    @builtin(position) clip: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) world: vec3<f32>,
    // A circle survives the shared wrap triangle; a scalar angle does not.
    @location(2) surface: vec3<f32>,
};

@vertex
fn vertex(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) coord: vec2<f32>,
) -> Varying {
    var out: Varying;
    out.clip = u.view_projection * vec4<f32>(position, 1.0);
    out.normal = normal;
    out.world = position;
    out.surface = vec3<f32>(coord.x, cos(coord.y), sin(coord.y));
    return out;
}

// Each angular harmonic is integral, so the height agrees at both sides of
// atan2's wrap. Pixel footprints fade unresolved bands before differentiation.
fn bark_height(coord: vec3<f32>) -> f32 {
    let angle = atan2(coord.z, coord.y);
    let ridge_scale = max(u.bark_detail.x, 0.0001);
    let plate_scale = max(u.bark_detail.y, 0.0001);
    let ridge = 24.0 * angle + 0.55 * sin(coord.x / ridge_scale);
    let plate = coord.x * 6.2831853 / plate_scale + 1.3 * sin(8.0 * angle);
    let circle = coord.yz;
    let angular_width = (length(dpdx(circle)) + length(dpdy(circle)))
        / max(length(circle), 0.0001);
    let ridge_filter = 1.0 - smoothstep(0.5, 3.0,
        24.0 * angular_width + 0.55 * fwidth(coord.x) / ridge_scale);
    let plate_filter = 1.0 - smoothstep(0.5, 3.0,
        10.4 * angular_width + 6.2831853 * fwidth(coord.x) / plate_scale);
    return u.bark_detail.x * 0.035 * cos(ridge) * ridge_filter
        + u.bark_detail.y * 0.012 * cos(plate) * plate_filter;
}

// Surface-gradient bump mapping needs no tangent attribute and displaces no
// vertex. The determinant handles either orientation of the screen axes.
fn bark_normal(n: vec3<f32>, world: vec3<f32>, height: f32) -> vec3<f32> {
    let dx = dpdx(world);
    let dy = dpdy(world);
    let rx = cross(dy, n);
    let ry = cross(n, dx);
    let det = dot(dx, rx);
    let gradient = dpdx(height) * rx + dpdy(height) * ry;
    return normalize(n - gradient * sign(det) / max(abs(det), 1e-10));
}

@fragment
fn fragment(in: Varying) -> @location(0) vec4<f32> {
    let base_normal = normalize(in.normal);
    let height = bark_height(in.surface);
    let perturbed = bark_normal(base_normal, in.world, height);
    let n = select(base_normal, perturbed, any(u.bark_detail.xy > vec2<f32>(0.0)));
    if (is_clay()) {
        return vec4<f32>(u.clay.rgb * clay_light(base_normal), 1.0);
    }
    let sun = u.sun.rgb * max(dot(n, u.sun_direction.xyz), 0.0)
        * sunlight(in.world, base_normal);
    // Roughness is what a surface does with the sun it does not scatter: chalk
    // spreads it over the whole face, a smooth young bark keeps a narrow sheen
    // along the light. One lobe, no second light - the sun is the only thing
    // bright enough to glance off a trunk.
    let detail = height / max(0.035 * u.bark_detail.x + 0.012 * u.bark_detail.y, 0.0001);
    let gloss = 1.0 - clamp(u.bark.w + u.bark_detail.z * detail, 0.0, 1.0);
    let half_way = normalize(normalize(u.eye.xyz - in.world) + u.sun_direction.xyz);
    let sheen = gloss * pow(max(dot(n, half_way), 0.0), exp2(1.0 + 10.0 * gloss));
    let lit = u.bark.rgb * (ambient(n) + sun) + sun * sheen;
    return vec4<f32>(tone(lit), 1.0);
}
