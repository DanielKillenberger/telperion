// A leaf lit as part of its crown rather than as a lone card, and the one
// highlight every material glances the sun off in. Every term here is a pure
// function of directions and one row value, so a synthetic surface can be put
// to it with nothing else bound, and at a value of zero each returns exactly
// what the card was lit by before the term existed.

/// The crown's outward direction at a point: the gradient of the ellipsoid the
/// placements fill, unit. Zero at the centre, where there is no outward.
fn crown_outward(position: vec3<f32>, centre: vec3<f32>, radii: vec3<f32>) -> vec3<f32> {
    let r = max(radii, vec3<f32>(1e-6));
    let gradient = (position - centre) / (r * r);
    let square = dot(gradient, gradient);
    return select(vec3<f32>(0.0), gradient * inverseSqrt(square), square > 1e-12);
}

/// The normal a leaf is lit by: its face's own, bent toward the crown's
/// outward direction by `bend`, so the sunward shell of the mass is lit
/// whichever way its leaves turn. No outward, or no bend, is the face alone.
fn canopy_normal(face: vec3<f32>, outward: vec3<f32>, bend: f32) -> vec3<f32> {
    if (bend <= 0.0 || dot(outward, outward) < 0.25) {
        return face;
    }
    let bent = mix(face, outward, bend);
    let square = dot(bent, bent);
    return select(outward, bent * inverseSqrt(square), square > 1e-8);
}

/// The sun's cosine wrapped past the terminator by `wrap` and renormalised:
/// a face square to the sun takes all of it, one turned edge-on takes
/// wrap / (1 + wrap), and one turned `wrap` past edge-on takes none.
fn wrapped(cosine: f32, wrap: f32) -> f32 {
    if (wrap <= 0.0) {
        return max(cosine, 0.0);
    }
    return max(cosine + wrap, 0.0) / (1.0 + wrap);
}

/// A thin leaf's diffuse transmission: what reaches its far face, the sun
/// where the map lets it through and the sky behind, leaving the near face
/// evenly in every direction. `transmittance` is tint, strength and the
/// thickness's attenuation together.
fn diffuse_through(normal: vec3<f32>, to_sun: vec3<f32>, sun: vec3<f32>,
    visibility: f32, sky_behind: vec3<f32>, transmittance: vec3<f32>) -> vec3<f32> {
    return transmittance * (sun * (visibility * max(dot(-normal, to_sun), 0.0)) + sky_behind);
}

/// The share of the sky a cuticle returns toward the eye: Schlick's Fresnel on
/// the reflectance `f0` at normal incidence, rising to all of it at grazing.
/// A reflectance of zero returns nothing at any angle.
fn sheen(normal: vec3<f32>, to_eye: vec3<f32>, f0: f32) -> f32 {
    if (f0 <= 0.0) {
        return 0.0;
    }
    let grazing = 1.0 - clamp(dot(normal, to_eye), 0.0, 1.0);
    let square = grazing * grazing;
    return f0 + (1.0 - f0) * square * square * grazing;
}

/// The sun a surface returns toward the eye as its one highlight: Schlick's
/// Fresnel on the reflectance `f0` at the half vector, over a Blinn-Phong term
/// normalised to the hemisphere whose width follows `roughness`, with the
/// geometry term that cancels the microfacet denominator. `x` is the share of
/// the sun the surface mirrors, which the diffuse gives up; `y` is that term
/// toward the eye, to multiply by the sun's irradiance the caller has. Over
/// every direction the eye can take the surface returns at most `x` of the
/// sun and never more than the sun. A reflectance of zero mirrors none, and
/// nothing reaches an eye below the surface's own horizon.
fn highlight(n: vec3<f32>, to_sun: vec3<f32>, to_eye: vec3<f32>, f0: f32,
    roughness: f32) -> vec2<f32> {
    if (f0 <= 0.0 || dot(n, to_eye) <= 0.0) {
        return vec2<f32>(0.0);
    }
    let half_way = normalize(to_sun + to_eye);
    // The width is the roughness squared, and a floor keeps a mirror's
    // exponent finite.
    let alpha = max(roughness * roughness, 0.01);
    let exponent = 2.0 / (alpha * alpha) - 2.0;
    let spread = (exponent + 2.0) / 8.0 * pow(max(dot(n, half_way), 1e-6), exponent);
    return vec2<f32>(sheen(half_way, to_eye, f0), spread);
}

/// How much crown lies ahead of a point along a direction: the chord, in
/// crown radii, that the ray from the point cuts through the ellipsoid the
/// placements fill. Nothing once the ray has left it; two across a diameter.
fn crown_chord(position: vec3<f32>, direction: vec3<f32>, centre: vec3<f32>,
    radii: vec3<f32>) -> f32 {
    let r = max(radii, vec3<f32>(1e-6));
    let q = (position - centre) / r;
    let d = direction / r;
    let a = max(dot(d, d), 1e-12);
    let b = dot(q, d);
    let discriminant = b * b - a * (dot(q, q) - 1.0);
    if (discriminant <= 0.0) {
        return 0.0;
    }
    let root = sqrt(discriminant);
    let far = (-b + root) / a;
    let near = max((-b - root) / a, 0.0);
    return max(far - near, 0.0) * sqrt(a);
}

/// The share of the sky that reaches a leaf along a direction through the
/// mass: each crown radius of chord takes `shade` of it, none at zero.
fn through_crown(chord: f32, shade: f32) -> f32 {
    return max(1.0 - shade * chord, 0.0);
}
