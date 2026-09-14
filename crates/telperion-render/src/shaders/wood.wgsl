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
fn bark_normal(n: vec3<f32>, world: vec3<f32>, dx: vec3<f32>, dy: vec3<f32>,
    height_x: f32, height_y: f32) -> vec3<f32> {
    let rx = cross(dy, n);
    let ry = cross(n, dx);
    let det = dot(dx, rx);
    // Relief cannot keep its full shading slope at a grazing silhouette.
    // Blend the resulting normal so large slopes cannot defeat visibility.
    let facing = abs(dot(n, normalize(u.eye.xyz - world)));
    let gradient = height_x * rx + height_y * ry;
    let perturbed = normalize(n - gradient * sign(det) / max(abs(det), 1e-10));
    return normalize(mix(n, perturbed, smoothstep(0.0, 0.6, facing)));
}

// Minimum principal curvature of the existing smooth surface. A convex tube
// has no contact cavity; the concave direction at a joined neck does. This
// cannot shade two disconnected surfaces merely because they are nearby.
fn socket_contact(n: vec3<f32>, dx: vec3<f32>, dy: vec3<f32>,
    nx: vec3<f32>, ny: vec3<f32>, radius: f32) -> f32 {
    let rx = cross(dy, n);
    let ry = cross(n, dx);
    let det = dot(dx, rx);
    let inverse = sign(det) / max(abs(det), 1e-10);
    let trace = (dot(nx, rx) + dot(ny, ry)) * inverse;
    let gaussian = dot(cross(nx, ny), n) * inverse;
    let minimum = 0.5 * (trace - sqrt(max(trace * trace - 4.0 * gaussian, 0.0)));
    return clamp(-radius * minimum, 0.0, 1.0);
}

// Appearance carries mottle, geometric contact, crown depth and relief maturity.
fn bark_light(n: vec3<f32>, height: f32, world: vec3<f32>, shadow: f32,
    variance: f32, appearance: vec4<f32>, colour_range: vec2<f32>) -> vec3<f32> {
    let sun = u.sun.rgb * max(dot(n, u.sun_direction.xyz), 0.0) * shadow;
    // Roughness is what a surface does with the sun it does not scatter: chalk
    // spreads it over the whole face, a smooth young bark keeps a narrow sheen
    // along the light. One lobe, no second light - the sun is the only thing
    // bright enough to glance off a trunk.
    let detail = height / max(0.055 * u.bark_detail.x, 0.0001);
    let gloss = 1.0 - clamp(u.bark.w + u.bark_detail.z * (detail + variance), 0.0, 1.0);
    var sheen = 0.0;
    // The result is exactly zero without gloss or reflected sun. Rough bark
    // and shadowed sockets need no eye vector or specular exponentiation.
    if (gloss > 0.0 && any(sun > vec3<f32>(0.0))) {
        let half_way = normalize(normalize(u.eye.xyz - world) + u.sun_direction.xyz);
        sheen = gloss * pow(max(dot(n, half_way), 0.0), exp2(1.0 + 10.0 * gloss));
    }
    // The field's mean is the untinted face, including its constant far path.
    // Each signed side is affine until saturation; splitting at zero adds a
    // kink, but never the tint-times-cavity quadratic of the original map.
    let t = clamp((height - colour_range.x) / max(colour_range.y, 1e-10), -1.0, 1.0);
    let crest = max(t, 0.0) * appearance.w;
    let fissure = max(-t, 0.0) * appearance.w;
    // Apply cavity to the base here: multiplying tinted colour by it would
    // introduce height squared and change the mean as the footprint widens.
    let cavity_weight = 1.0 - u.bark_colour_detail.z * fissure;
    let albedo = u.bark.rgb * cavity_weight + u.fissure.rgb * u.fissure.w * fissure
        + u.crest.rgb * u.crest.w * crest;
    let colour = clamp(albedo * appearance.x, vec3<f32>(0.0), vec3<f32>(1.0));
    let contact = 1.0 - u.bark_colour_detail.z * appearance.y;
    return (colour * (occluded_ambient(n, appearance.z) + sun)
        + sun * sheen * cavity_weight) * contact;
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
    // Evaluate geometry derivatives before any per-fragment shortcut. WGSL
    // derivatives require uniform control flow, including in Chromium.
    let dx = dpdx(in.world);
    let dy = dpdy(in.world);
    let nx = dpdx(base_normal);
    let ny = dpdy(base_normal);
    let sx = dpdx(in.surface);
    let sy = dpdy(in.surface);
    // One low-frequency noise sample per fragment, shared by all shading cells.
    let mottle_scale = max(u.bark_colour_detail.x, 0.0001);
    var mottle = 1.0;
    if (u.bark_colour_detail.x > 0.0 && u.bark_colour_detail.y > 0.0) {
        let noise = bark_noise2_filtered(vec2<f32>(dot(arc, vec2<f32>(0.8, 0.6)), in.surface.x)
            / mottle_scale, footprint / mottle_scale);
        mottle += u.bark_colour_detail.y * (2.0 * noise - 1.0);
    }
    // Ground contact and concave joined necks use geometry, independently
    // of relief height. Derivatives above stay outside material branches.
    let base = 1.0 - smoothstep(0.0, max(in.radius, 0.0001), max(in.world.y, 0.0));
    let maturity = smoothstep(2.0, 5.0, 2.0 * in.radius / max(u.bark_detail.x, 0.000001))
        * select(0.0, 1.0, u.bark_detail.x > 0.0);
    var contact = base;
    if (u.bark_colour_detail.z > 0.0) {
        contact = max(contact, socket_contact(base_normal, dx, dy, nx, ny, in.radius));
    }
    let appearance = vec4<f32>(mottle, contact, depth_in_crown(in.world), maturity);
    let colour_range = bark_colour_range(in.radius, u.bark_detail.x, u.bark_detail.y, u.bark_detail.w);
    let shadow = sunlight(in.world, base_normal);
    let spacing = clamp(u.bark_detail.y, u.bark_detail.x * 1.5, u.bark_detail.x * 2.0);
    let pixel = footprint / max(vec2(u.bark_detail.x, spacing), vec2(0.000001));
    let band = max(pixel.x, pixel.y);
    // Lost high-frequency slope variance remains a roughness contribution.
    // The same numeric row controls it; fully resolved and young wood add none.
    let fine = max(pixel.x / 0.19, pixel.y / 0.19 + pixel.x * 5.64);
    let retained = bark_box(fine) * bark_pass(fine);
    let fine_slope = 0.012 / (0.19 * 0.3);
    let broad_slope = 0.095 * u.bark_detail.w / mix(0.04, 0.28, u.bark_detail.w);
    let coarse = bark_pass(band);
    let variance = (fine_slope * fine_slope * (1.0 - retained * retained)
        + (broad_slope * broad_slope + 0.05 * 0.05 / (0.3 * 0.3)) * (1.0 - coarse * coarse))
        * smoothstep(2.0, 5.0, 2.0 * in.radius / max(u.bark_detail.x, 0.000001))
        * select(0.0, 1.0, u.bark_detail.x > 0.0);
    // Constant height has zero gradient. Shade it once, avoiding twenty
    // redundant field evaluations and four identical lighting evaluations.
    if (u.bark_detail.x <= 0.0 || in.radius <= u.bark_detail.x || band >= 1.0) {
        let height = bark_height(circle, in.surface.x, in.radius, footprint);
        return vec4<f32>(tone(bark_light(base_normal, height, in.world, shadow, variance, appearance, colour_range)), 1.0);
    }
    // Four half-pixel shading cells share nine heights. Each height uses the
    // cell's footprint: filtering over a full pixel here and integrating the
    // cells again overfiltered the low-resolution albedo. Only wavelengths
    // below 1.33 pixels widen back to the full-pixel mean before the shortcut.
    let cell_footprint = footprint * mix(0.5, 1.0, smoothstep(0.75, 1.0, band));
    var heights: array<f32, 9>;
    for (var y = 0; y <= 2; y++) {
        for (var x = 0; x <= 2; x++) {
            let coord = in.surface + (0.5 * f32(x) - 0.5) * sx
                + (0.5 * f32(y) - 0.5) * sy;
            heights[y * 3 + x] = bark_height(normalize(coord.yz), coord.x, in.radius, cell_footprint);
        }
    }
    var lit = vec3(0.0);
    for (var y = 0; y < 2; y++) {
        for (var x = 0; x < 2; x++) {
            let h = vec4(heights[y * 3 + x], heights[y * 3 + x + 1],
                heights[(y + 1) * 3 + x], heights[(y + 1) * 3 + x + 1]);
            // Average the two differences, divided by a half-pixel cell.
            let n = bark_normal(base_normal, in.world, dx, dy,
                h.y + h.w - h.x - h.z, h.z + h.w - h.x - h.y);
            lit += bark_light(n, dot(h, vec4(0.25)), in.world, shadow, variance, appearance, colour_range);
        }
    }
    return vec4<f32>(tone(lit * 0.25), 1.0);
}
