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
        footprint, u.bark_detail.w, u.plate, u.bark_structure.x);
}

// A furrow floor is dark because its own crest stands between it and the sun.
// Walk the height field across the surface towards the sun, and compare what
// the bark rises to against what the sun ray rises to over the same ground.
// No new light and no second map: the field already in hand answers it, and
// the asymmetry a cavity term cannot give comes out of the walk's direction.
fn bark_shade(surface: vec3<f32>, sx: vec3<f32>, sy: vec3<f32>, n: vec3<f32>,
    dx: vec3<f32>, dy: vec3<f32>, radius: f32, footprint: vec2<f32>,
    here: f32, amplitude: f32) -> f32 {
    if (u.bark_structure.y <= 0.0 || amplitude <= 0.0) { return 1.0; }
    let rise = dot(n, u.sun_direction.xyz);
    let across = u.sun_direction.xyz - n * rise;
    let span = length(across);
    // A sun below the surface's own horizon lights none of it to shade.
    if (span < 1e-4 || rise <= 0.0) { return 1.0; }
    let rx = cross(dy, n);
    let ry = cross(n, dx);
    let det = dot(dx, rx);
    let inverse = sign(det) / max(abs(det), 1e-10);
    let toward = across / span;
    // Pixels to step for one metre across the surface towards the sun.
    let steps = vec2(dot(toward, rx), dot(toward, ry)) * inverse;
    let slope = rise / span;
    // How far a crest can stand from the floor it shades: a third of the
    // ridge's own shoulder, or a plate's wall where the plates are the
    // coarser structure. A walk longer than that leaves the furrow and
    // measures another one - which is what the first walk did once the
    // network became a cellular partition of the surface, whose cells are
    // smaller than the lattice they are drawn from.
    let reach = max(0.35 * u.bark_detail.x, BARK_PLATE_WALL * u.plate.x);
    // Three steps rather than two: a partition of the surface puts the crest
    // that shades this floor anywhere between here and a wall away, at any
    // bearing, and two steps over that reach can stride across it.
    var blocked = 0.0;
    for (var i = 1; i <= 3; i++) {
        let ground = reach * f32(i) / 3.0;
        let walked = steps * ground;
        let coord = surface + walked.x * sx + walked.y * sy;
        let there = bark_height(normalize(coord.yz), coord.x, radius, footprint);
        blocked = max(blocked, (there - here - ground * slope) / amplitude);
    }
    return 1.0 - u.bark_structure.y * clamp(blocked, 0.0, 1.0);
}

// Parallax: an eye looking across a furrow sees the near wall, not the floor
// behind it. The surface coordinate is walked towards the eye in proportion
// to how far below the crest this fragment stands, so the relief gains the
// depth a tilted normal alone cannot show. The mesh is untouched, so the
// silhouette stays the smooth cylinder it has always been.
fn bark_parallax(surface: vec3<f32>, sx: vec3<f32>, sy: vec3<f32>, n: vec3<f32>,
    dx: vec3<f32>, dy: vec3<f32>, world: vec3<f32>, here: f32,
    colour_range: vec2<f32>) -> vec3<f32> {
    let view = normalize(u.eye.xyz - world);
    let facing = dot(n, view);
    let across = view - n * facing;
    let span = length(across);
    if (span < 1e-4 || facing <= 0.0) { return surface; }
    let rx = cross(dy, n);
    let ry = cross(n, dx);
    let det = dot(dx, rx);
    let inverse = sign(det) / max(abs(det), 1e-10);
    let toward = across / span;
    let steps = vec2(dot(toward, rx), dot(toward, ry)) * inverse;
    // How far below the field's crest this fragment stands, and how far the
    // eye travels across the surface to look down that far. The floor of a
    // grazing furrow would walk without bound, so the slope is held.
    let below = max(colour_range.x + colour_range.y - here, 0.0);
    let walk = u.bark_structure.z * below * span / max(facing, 0.3);
    let offset = steps * walk;
    return surface + offset.x * sx + offset.y * sy;
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

// Appearance carries mottle, geometric contact, crown depth and relief
// maturity; structure carries what the plate network says about this
// fragment - which way it faces, which plate it belongs to, and how much of
// the sun its own crest leaves it.
fn bark_light(n: vec3<f32>, height: f32, world: vec3<f32>, shadow: f32,
    variance: f32, appearance: vec4<f32>, colour_range: vec2<f32>,
    structure: vec3<f32>) -> vec3<f32> {
    let sun = u.sun.rgb * max(dot(n, u.sun_direction.xyz), 0.0) * shadow * structure.z;
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
    // A weathered face is greyer and paler than the fresh wood a furrow keeps.
    // Greying the base row rather than the tinted albedo keeps the whole map
    // affine in the filtered height: the offset below is a constant vector.
    let grey = dot(u.bark.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    let weathered = vec3<f32>(grey) - u.bark.rgb + u.weathering.rgb;
    let albedo = u.bark.rgb * cavity_weight + u.fissure.rgb * u.fissure.w * fissure
        + u.crest.rgb * u.crest.w * crest
        + weathered * u.weathering.w * crest
        + u.orientation.rgb * u.orientation.w * structure.x;
    // What one plate keeps against its neighbours: a gain per channel, so the
    // value and the cast move together. A linear map on the albedo, so the
    // height affinity the filtered colour rests on survives it.
    let own = u.bark_structure.x * (2.0 * structure.y - 1.0);
    let plate = vec3<f32>(1.0) + own * vec3<f32>(0.42, 0.34, 0.22);
    let colour = clamp(albedo * plate * appearance.x, vec3<f32>(0.0), vec3<f32>(1.0));
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
    // The side turned away from the sun and the foot of the trunk are where
    // damp things settle. One weight; the row's tint says what it looks like.
    let away = clamp(0.5 - 0.5 * dot(base_normal, u.sun_direction.xyz), 0.0, 1.0);
    let orientation = away * mix(0.55, 1.0, 1.0 - smoothstep(0.0, 2.5, in.world.y));
    let appearance = vec4<f32>(mottle, contact, depth_in_crown(in.world), maturity);
    let colour_range = bark_colour_range(in.radius, u.bark_detail.x, u.bark_detail.y,
        u.bark_detail.w, u.plate);
    let shadow = sunlight(in.world, base_normal);
    // Where the eye is actually looking on the surface, once the relief has
    // depth. Every field read below starts from here; the world position,
    // the geometric contact and the crown depth remain the fragment's own.
    var surface = in.surface;
    // The one field read the walk costs is paid only where there is a walk.
    if (u.bark_structure.z > 0.0 && colour_range.y > 0.0) {
        let flat = bark_height(circle, in.surface.x, in.radius, footprint);
        surface = bark_parallax(in.surface, sx, sy, base_normal, dx, dy, in.world,
            flat, colour_range);
    }
    let seen = normalize(surface.yz);
    // One plate identity per fragment, shared by every shading cell the way
    // the mottle above is: a plate keeps one colour across its whole face.
    let own = bark_plate_identity(seen, surface.x, in.radius, u.bark_detail.x,
        footprint, u.plate, u.bark_structure.x);
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
        // A constant field has nowhere to look down into, so the walk above
        // returned the fragment's own coordinate and this is the same height.
        let height = bark_height(seen, surface.x, in.radius, footprint);
        let direct = bark_shade(surface, sx, sy, base_normal, dx, dy, in.radius,
            footprint, height, colour_range.y);
        return vec4<f32>(tone(bark_light(base_normal, height, in.world, shadow,
            variance, appearance, colour_range, vec3<f32>(orientation, own, direct))), 1.0);
    }
    // Four half-pixel shading cells share nine heights. Each height uses the
    // cell's footprint: filtering over a full pixel here and integrating the
    // cells again overfiltered the low-resolution albedo. Only wavelengths
    // below 1.33 pixels widen back to the full-pixel mean before the shortcut.
    let cell_footprint = footprint * mix(0.5, 1.0, smoothstep(0.75, 1.0, band));
    var heights: array<f32, 9>;
    for (var y = 0; y <= 2; y++) {
        for (var x = 0; x <= 2; x++) {
            let coord = surface + (0.5 * f32(x) - 0.5) * sx
                + (0.5 * f32(y) - 0.5) * sy;
            heights[y * 3 + x] = bark_height(normalize(coord.yz), coord.x, in.radius, cell_footprint);
        }
    }
    // The walk towards the sun starts from the fragment's own centre height,
    // which the nine above already carry: two more field samples, not eleven.
    let direct = bark_shade(surface, sx, sy, base_normal, dx, dy, in.radius,
        cell_footprint, heights[4], colour_range.y);
    let structure = vec3<f32>(orientation, own, direct);
    var lit = vec3(0.0);
    for (var y = 0; y < 2; y++) {
        for (var x = 0; x < 2; x++) {
            let h = vec4(heights[y * 3 + x], heights[y * 3 + x + 1],
                heights[(y + 1) * 3 + x], heights[(y + 1) * 3 + x + 1]);
            // Average the two differences, divided by a half-pixel cell.
            let n = bark_normal(base_normal, in.world, dx, dy,
                h.y + h.w - h.x - h.z, h.z + h.w - h.x - h.y);
            lit += bark_light(n, dot(h, vec4(0.25)), in.world, shadow, variance,
                appearance, colour_range, structure);
        }
    }
    return vec4<f32>(tone(lit * 0.25), 1.0);
}
