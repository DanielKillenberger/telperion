// The bark: the material row's colour and roughness over the plaited surface,
// under the sun it stands in and the sky it stands beneath. In the clay room it
// is the one flat value it has always been, so form can still be judged with no
// material over it.

@group(1) @binding(0) var<storage, read> radii: array<f32>;

struct Varying {
    @builtin(position) clip: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) world: vec3<f32>,
    // A circle survives the shared wrap triangle; a scalar angle does not.
    @location(2) surface: vec3<f32>,
    @location(3) radius: f32,
};

@vertex
fn vertex(
    @builtin(vertex_index) index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) coord: vec2<f32>,
) -> Varying {
    var out: Varying;
    out.clip = u.view_projection * vec4<f32>(position, 1.0);
    out.normal = normal;
    out.world = position;
    out.radius = radii[index];
    out.surface = vec3<f32>(coord.x, cos(coord.y), sin(coord.y));
    return out;
}

// Filter the physical footprint, not atan2's discontinuous derivative. Both
// grain directions resolve at the same surface scale on a trunk and a limb.
fn bark_height(circle: vec2<f32>, along: f32, radius: f32, footprint: vec2<f32>) -> f32 {
    return bark_field_filtered(circle, along, radius, u.bark_detail.x, u.bark_detail.y,
        footprint, u.bark_detail.w);
}

// Surface-gradient bump mapping needs no tangent attribute and displaces no
// vertex. The determinant handles either orientation of the screen axes.
fn bark_normal(n: vec3<f32>, world: vec3<f32>, circle: vec2<f32>, along: f32,
    radius: f32, footprint: vec2<f32>) -> vec3<f32> {
    // The hardware's 2x2 difference puts both pixels' slope halfway between
    // them, which crawls even for a band-limited height. Differentiate the
    // SAME filtered field symmetrically at the fragment instead. Hold the
    // footprint fixed: changing the camera's filter is not surface relief.
    let step = max(u.bark_detail.x * 0.002, 0.000001);
    let tangent = vec2(-circle.y, circle.x);
    let offset = tangent * (step / max(radius, 0.0001));
    let across = (bark_height(normalize(circle + offset), along, radius, footprint)
        - bark_height(normalize(circle - offset), along, radius, footprint)) / (2.0 * step);
    let axial = (bark_height(circle, along + step, radius, footprint)
        - bark_height(circle, along - step, radius, footprint)) / (2.0 * step);
    let hx = across * radius * dot(dpdx(circle), tangent) + axial * dpdx(along);
    let hy = across * radius * dot(dpdy(circle), tangent) + axial * dpdy(along);
    let dx = dpdx(world);
    let dy = dpdy(world);
    let rx = cross(dy, n);
    let ry = cross(n, dx);
    let det = dot(dx, rx);
    let gradient = hx * rx + hy * ry;
    return normalize(n - gradient * sign(det) / max(abs(det), 1e-10));
}

fn bark_light(n: vec3<f32>, height: f32, world: vec3<f32>, shadow: f32) -> vec3<f32> {
    let sun = u.sun.rgb * max(dot(n, u.sun_direction.xyz), 0.0) * shadow;
    // Roughness is what a surface does with the sun it does not scatter: chalk
    // spreads it over the whole face, a smooth young bark keeps a narrow sheen
    // along the light. One lobe, no second light - the sun is the only thing
    // bright enough to glance off a trunk.
    let detail = height / max(0.055 * u.bark_detail.x, 0.0001);
    let gloss = 1.0 - clamp(u.bark.w + u.bark_detail.z * detail, 0.0, 1.0);
    let half_way = normalize(normalize(u.eye.xyz - world) + u.sun_direction.xyz);
    let sheen = gloss * pow(max(dot(n, half_way), 0.0), exp2(1.0 + 10.0 * gloss));
    return u.bark.rgb * (ambient(n) + sun) + sun * sheen;
}

@fragment
fn fragment(in: Varying) -> @location(0) vec4<f32> {
    let base_normal = normalize(in.normal);
    if (is_clay()) {
        return vec4<f32>(u.clay.rgb * clay_light(base_normal), 1.0);
    }
    let circle = normalize(in.surface.yz);
    let arc = circle * in.radius;
    let footprint = vec2(length(dpdx(arc)) + length(dpdy(arc)), fwidth(in.surface.x));
    let sx = dpdx(in.surface);
    let sy = dpdy(in.surface);
    let shadow = sunlight(in.world, base_normal);
    var lit = vec3(0.0);
    // Integrate shading, whose clamped cosine and sheen are nonlinear in the
    // normal. Each subpixel still differentiates a footprint-filtered height.
    // Geometry coverage and the existing single shadow lookup stay unchanged.
    for (var y = 0; y < 2; y++) {
        for (var x = 0; x < 2; x++) {
            let coord = in.surface + (f32(x) * 0.5 - 0.25) * sx + (f32(y) * 0.5 - 0.25) * sy;
            let unit = normalize(coord.yz);
            let height = bark_height(unit, coord.x, in.radius, footprint);
            let perturbed = bark_normal(base_normal, in.world, unit, coord.x, in.radius, footprint);
            let n = select(base_normal, perturbed, any(u.bark_detail.xy > vec2<f32>(0.0)));
            lit += bark_light(n, height, in.world, shadow);
        }
    }
    return vec4<f32>(tone(lit * 0.25), 1.0);
}
