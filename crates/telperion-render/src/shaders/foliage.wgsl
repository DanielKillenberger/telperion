// The crown: the material row's two leaf faces, each leaf turned a little off
// the row's own colour by its seeded offset, darkened by how deep into the mass
// it stands, under the same sun and the same sky as the wood. The placement is
// not fed in per instance: this draw is one level of the element, and its
// instances are the leaves selection put in that level's list, so each one
// looks its own placement - and its own identity - up through the list.

/// The crown's placements, as the core packed them - three words a leaf,
/// decoded by `leaf.wgsl` against the box in the frame block - and the list of
/// the ones this draw's level was given, bound at the level's own offset.
@group(1) @binding(0) var<storage, read> placements: array<u32>;
@group(1) @binding(1) var<storage, read> list: array<u32>;
/// How deep each cell of the crown stands in its own leaf mass, after a
/// header of the grid's corner, its cell's edge and its three counts
/// (mass.rs).
@group(1) @binding(2) var<storage, read> masses: array<f32>;

struct Varying {
    @builtin(position) clip: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) world: vec3<f32>,
    // One seed and crown depth per placement. Carry the seed once for both
    // colour variation and mottle, instead of adding another vertex varying.
    @location(2) @interpolate(flat) leaf: vec3<f32>,
    @location(3) coord: vec2<f32>,
    // How deep the placement stands in its own mass; none without the row.
    @location(4) @interpolate(flat) mass: f32,
    // One where the placement is a withered leaf, drawn in the dead colour.
    @location(5) @interpolate(flat) withered: f32,
};

/// The depth in its own mass of the cell a placement stands in. A grid of no
/// cells, as the leaf view binds, reads none.
fn mass_depth(at: vec3<f32>) -> f32 {
    let n = vec3<i32>(i32(masses[4]), i32(masses[5]), i32(masses[6]));
    if (n.x <= 0) {
        return 0.0;
    }
    let corner = vec3<f32>(masses[0], masses[1], masses[2]);
    let cell = vec3<i32>(floor((at - corner) / masses[3]));
    let c = clamp(cell, vec3<i32>(0), n - vec3<i32>(1));
    return masses[8 + c.x + n.x * (c.y + n.y * c.z)];
}

@vertex
fn vertex(
    @builtin(instance_index) instance: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) coord: vec2<f32>,
) -> Varying {
    let id = list[instance];
    let base = id * LEAF_WORDS;
    let words = vec3<u32>(placements[base], placements[base + 1u], placements[base + 2u]);
    let placement = leaf_transform(words, u.leaf_box_min.xyz, u.leaf_box_extent.xyz);
    // A leaf selection kept from a thinned share is drawn enlarged by the
    // inverse root of that share, so the share covers what all of it would
    // (fn-246): its size on screen from the frame's pixels a metre (eye.w)
    // and the element's bounding radius (leaf_box_min.w).
    let depth = max((u.view_projection * placement[3]).w, 1e-4);
    let across = 2.0 * u.leaf_box_min.w * leaf_scale(words) * u.eye.w / depth;
    let world = placement * vec4<f32>(position / sqrt(max(leaf_kept(across), 1e-4)), 1.0);
    let offsets = vary(id);
    var out: Varying;
    out.clip = u.view_projection * world;
    out.normal = (placement * vec4<f32>(normal, 0.0)).xyz;
    out.world = world.xyz;
    // Restore the side before interpolation, then fold in the fragment.
    // Shared coarse triangles can cross the midrib without erasing it.
    out.coord = vec2<f32>(coord.x, coord.y * sign(position.x));
    out.leaf = vec3<f32>(offsets, depth_in_crown(placement[3].xyz));
    out.mass = 0.0;
    out.withered = select(0.0, 1.0, leaf_withered(words));
    if (u.crown_shade.y > 0.0) {
        out.mass = mass_depth(placement[3].xyz);
    }
    return out;
}

// Vein widths are in leaf coordinates; a pixel footprint filters the lines
// on every level, without a level-dependent phase or per-triangle randomness.
fn vein_tone(coord: vec2<f32>) -> f32 {
    let across = abs(coord.y);
    let width = max(fwidth(coord.y), 0.001);
    let midrib = 1.0 - smoothstep(0.015, 0.015 + width, across);
    let phase = coord.x * u.leaf_detail.x - 0.65 * across;
    let pair_distance = abs(fract(phase + 0.5) - 0.5);
    let pair_width = max(fwidth(phase), 0.001);
    let secondary = (1.0 - smoothstep(0.018, 0.018 + pair_width, pair_distance))
        * (1.0 - smoothstep(0.65, 0.95, across))
        * (1.0 - smoothstep(0.25, 0.75, pair_width));
    let margin = smoothstep(0.86 - width, 1.0, across);
    let vein = max(midrib, secondary * 0.6);
    return 1.0 + u.leaf_detail.y * (1.0 - u.leaf_detail.z)
        * (0.8 * vein - 0.25 * margin);
}

// The cells between the veins: filtered noise in leaf coordinates at the
// row's count per leaf length, each leaf's own by its seed. It fades an
// octave before the mottle above it, from two pixels a cell to one, because
// its tilt of the normal is not linear in it.
fn leaf_cells(coord: vec2<f32>, seed: vec2<f32>, pixel: vec2<f32>) -> f32 {
    let across = vec2(1.0, 0.5) * u.grain.z;
    return bark_noise2_filtered(coord * across + seed * 53.0, 2.0 * pixel * across);
}

// The grain here, and its differences over one pixel each way across the
// screen, for the tilt of the normal.
fn leaf_grain(coord: vec2<f32>, seed: vec2<f32>, pixel: vec2<f32>,
    cx: vec2<f32>, cy: vec2<f32>) -> vec3<f32> {
    let here = leaf_cells(coord, seed, pixel);
    return vec3(here, leaf_cells(coord + cx, seed, pixel) - here,
        leaf_cells(coord + cy, seed, pixel) - here);
}

/// The share of a pixel's footprint on the leaf, `at` in a leaf coordinate
/// over [0, 1], that falls in the band [`low`, `high`]: the band's edges
/// filtered over `footprint`, and where the footprint spans the whole leaf,
/// the band's share of the coordinate, which the margin weighs to its area
/// (fn-217 DESIGN.md 8.2). The band is set in leaf space, never in pixels.
/// A coordinate that stands still, as a stamen's does, is wholly in or out.
fn band(at: f32, low: f32, high: f32, footprint: f32) -> f32 {
    let half = 0.5 * max(footprint, 1e-6);
    let start = max(at - half, 0.0);
    let end = min(at + half, 1.0);
    let inside = max(min(end, high) - max(start, low), 0.0);
    return inside / max(end - start, 1e-12);
}

/// The share of the sky that reaches a leaf at `world` through the crown
/// standing over it: the chord straight up, where an overcast sky is
/// brightest, taken at the row's shade per crown radius. All of it at zero.
fn crown_sky(world: vec3<f32>) -> f32 {
    if (u.crown_shade.x <= 0.0 || u.crown_centre.w < 0.5) {
        return 1.0;
    }
    let chord = crown_chord(world, vec3<f32>(0.0, 1.0, 0.0), u.crown_centre.xyz, u.crown_radii.xyz);
    return through_crown(chord, u.crown_shade.x);
}

@fragment
fn fragment(in: Varying, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // A leaf has no back. Nothing is culled, so the face the eye sees takes the
    // light; without the flip half the crown would read as holes.
    var n = normalize(select(-in.normal, in.normal, front));
    if (is_clay()) {
        return vec4<f32>(u.clay.rgb * clay_light(n), 1.0);
    }
    // A leaf is paler underneath, and the eye is shown whichever face it is
    // looking at; the seeded offset is the leaf's own and applies to both. A
    // withered leaf has aged to one colour on either face.
    var face = select(u.leaf_back.rgb, u.leaf_front.rgb, front);
    if (in.withered > 0.5) {
        face = u.leaf_dead.rgb;
    }
    let scale = u.leaf_colour_detail.x;
    let pixel = fwidth(in.coord);
    // A floret's eye (fn-217 F1): its petals turn toward the eye's colour
    // from their base up the row's share of their length, and the stamens,
    // which stand at the base, take it whole. None on a leaf.
    if (u.bloom_eye.w > 0.0) {
        face = mix(face, u.bloom_eye.rgb, band(in.coord.x, 0.0, u.bloom_eye.w, pixel.x));
    }
    // Geometry derivatives before any row branch: WGSL wants them in
    // uniform control flow, including in Chromium.
    let dx = dpdx(in.world);
    let dy = dpdy(in.world);
    let cx = dpdx(in.coord);
    let cy = dpdy(in.coord);
    let veins = vein_tone(in.coord);
    var modulation = 1.0;
    if (scale > 0.0 && u.leaf_colour_detail.y > 0.0) {
        let mottle = bark_noise2_filtered(in.coord * scale + in.leaf.xy * 37.0, pixel * scale);
        modulation += u.leaf_colour_detail.y * (2.0 * mottle - 1.0);
    }
    // The grain below the mottle: a factor on the colour and a tilt of the
    // normal, a cell's relief a third of its width at full strength, read
    // off how far a pixel walks the leaf against the world.
    if (u.grain.z > 0.0 && u.grain.w > 0.0) {
        let grain = leaf_grain(in.coord, in.leaf.xy, pixel, cx, cy);
        modulation += u.grain.w * (2.0 * grain.x - 1.0);
        let metres = length(dx) / max(length(cx), 1e-6);
        let height = u.grain.w * metres / (3.0 * u.grain.z);
        n = relief_normal(n, in.world, dx, dy, grain.y * height, grain.z * height);
    }
    let edge = max(abs(in.coord.y), in.coord.x);
    let width = max(pixel.x, pixel.y);
    // The band's coordinate does not run with the leaf's area, so as the
    // footprint grows past the leaf the share it tends to is weighed to the
    // band's measured area share.
    let area = u.leaf_colour_detail.w / max(u.margin.w, 1e-6);
    let margin = band(edge, 1.0 - u.margin.w, 1.0, width)
        * mix(1.0, area, smoothstep(0.5, 2.0, width))
        * select(0.0, 1.0, u.margin.w > 0.0);
    let hue = mix(u.leaf_variation.x, u.leaf_variation.y, in.leaf.x);
    let brightness = 1.0 + mix(u.leaf_variation.z, u.leaf_variation.w, in.leaf.y);
    let colour = clamp(hue_shift(face, hue) * brightness
        * veins * modulation + margin * u.margin.rgb,
        vec3<f32>(0.0), vec3<f32>(1.0));
    // A leaf in a mass is lit as part of it: the normal the light arriving
    // at it is read by - sky, sun and what passes through - bends from the
    // face toward the crown's outward direction by the row's canopy normal,
    // and is the face's own at zero or with no crown. What the face itself
    // reflects, the sun's glint and the sky's sheen, stays the face's.
    var lit = n;
    if (u.canopy.x > 0.0 && u.crown_centre.w > 0.5) {
        let outward = crown_outward(in.world, u.crown_centre.xyz, u.crown_radii.xyz);
        lit = canopy_normal(n, outward, u.canopy.x);
    }
    let to_eye = normalize(u.eye.xyz - in.world);
    // Deep in the crown there is no sky to see: thousands of leaves stand
    // between this one and it, and what is left reads as a shaded mass rather
    // than as speckle. The sun is not attenuated - what reaches through is
    // the dapple the shared normal-offset comparison kernel reads.
    let interior = 1.0 - u.leaf_front.w * in.leaf.z;
    // The sky a leaf reads, over it, behind it and in its sheen, is what the
    // crown standing over it lets through, so a crown's underside falls into
    // its own shade.
    let overhead = crown_sky(in.world);
    var shaded = occluded_ambient(lit, in.leaf.z) * interior;
    if (u.crown_shade.x > 0.0) {
        shaded *= overhead;
    }
    // One comparison result gates reflection and transmission alike. The
    // original face normal still offsets the receiver, as before this term.
    let visibility = sunlight(in.world, n);
    let direct = u.sun.rgb * wrapped(dot(lit, u.sun_direction.xyz), u.canopy.y) * visibility;
    var through = u.sun.rgb * transmitted(lit, to_eye,
        u.sun_direction.xyz, u.transmission.rgb, u.transmission.w,
        u.leaf_detail.w, visibility);
    if (u.canopy.z > 0.0) {
        // A thin leaf scatters what passes through it: the row's share leaves
        // the near face evenly, and carries the sky behind the leaf with it.
        let transmittance = u.transmission.rgb * (u.transmission.w * exp(-u.leaf_detail.w));
        let behind = occluded_ambient(-lit, in.leaf.z) * interior * overhead;
        through = mix(through, diffuse_through(lit, u.sun_direction.xyz, u.sun.rgb,
            visibility, behind, transmittance), u.canopy.z);
    }
    let gloss = u.leaf_colour_detail.z;
    var mirrored = vec2<f32>(0.0);
    // A fully shadowed or backlit face has no reflected sun to glint, and
    // the back has no cuticle. Its width follows the gloss row and its
    // foot is the row's reflectance, taken from the diffuse.
    if (front && u.reflectance.y > 0.0 && any(direct > vec3<f32>(0.0))) {
        mirrored = highlight(n, u.sun_direction.xyz, to_eye, u.reflectance.y, 1.0 - gloss);
    }
    // The supernatural group's mirrors (fn-217 E2): a metal's on the
    // underside, tinted by the group's back tint, and on the margin band of
    // either face, tinted by the colour the band shows. Each takes its share
    // of the sun from the face's diffuse, as the cuticle's glint does: the
    // front glint and the underside's are never on one face, and within the
    // band the rim's share adds to the face's, the two together at most all
    // of it. None of it at a reflectance of zero, on every natural leaf.
    let back = select(u.leaf_reflection.w, 0.0, front);
    let rough = u.reflectance.w;
    var silver = vec2<f32>(0.0);
    var rim = vec2<f32>(0.0);
    if (any(direct > vec3<f32>(0.0))) {
        if (back > 0.0) {
            silver = highlight(n, u.sun_direction.xyz, to_eye, back, rough);
        }
        if (u.reflectance.z > 0.0 && margin > 0.0) {
            rim = highlight(n, u.sun_direction.xyz, to_eye, u.reflectance.z, rough);
        }
    }
    // The two mirrors share what the cuticle's glint leaves: past it, both
    // are scaled down together, in what they reflect as in what they take.
    let added = silver.x + margin * rim.x;
    let room = max(1.0 - mirrored.x, 0.0);
    let share = select(1.0, room / added, added > room);
    let given = mirrored.x + added * share;
    let metal = (u.leaf_reflection.rgb * (silver.x * silver.y)
        + colour * (margin * rim.x * rim.y)) * share;
    // The leaves of its own mass standing over it take their share of the
    // sky and of what passes through, so a clump's face is lit and what
    // hangs under it falls into its shade.
    var own = 1.0;
    if (u.crown_shade.y > 0.0) {
        own = 1.0 - u.crown_shade.y * in.mass;
        shaded *= own;
        through *= own;
    }
    // The underside's metal mirrors the sky as well, its share taken from
    // the sky's diffuse on that face.
    let sky_given = sheen(n, to_eye, back);
    var radiance = colour * (shaded * (1.0 - sky_given) + direct * (1.0 - given)) + through
        + direct * mirrored.x * mirrored.y + direct * metal;
    let mirror = reflect(-to_eye, n);
    if (u.canopy.w > 0.0) {
        // The cuticle returns the sky its own face mirrors, most of it at
        // grazing. Reflection is the surface's, so it reads the face, as the
        // sun's glint does; the bent normal stands for light arriving through
        // the mass, and on its silhouette would mirror the sky off every rim.
        radiance += sheen(n, to_eye, u.canopy.w) * occluded_ambient(mirror, in.leaf.z)
            * interior * overhead * own;
    }
    if (back > 0.0) {
        radiance += u.leaf_reflection.rgb * sky_given * occluded_ambient(mirror, in.leaf.z)
            * interior * overhead * own;
    }
    return vec4<f32>(tone(radiance), 1.0);
}
