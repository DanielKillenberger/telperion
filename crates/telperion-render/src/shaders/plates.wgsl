// Concatenated after bark.wgsl, whose field reads this network and whose edge
// integral the network uses.
//
// The plate network, the field's second primitive. Ridges are parallel: they
// cannot branch, cannot merge, and cannot give one plate an identity of its
// own. A cellular partition does all three.
//
// Round one partitioned the circumference into columns and then cut each
// column along its run. That is a wall: the columns run the whole height of
// the trunk, every cut meets one square, and every cell is one size. The
// owner saw it and called it armour plating. This is the partition the
// surface actually wants - sites scattered in the space the bark passes
// through, so a boundary is where two of them are equally near. Three
// boundaries meet at a point because three sites do; a cell is the size its
// own site says it is; and going round the trunk returns to the same sites,
// so the angular wrap has no seam to hide.
//
// The lattice is the circle embedding in plate widths and the run along the
// trunk, so an elongated plate is one whose sites stand further apart
// axially - and the axial offset is scaled back by the same elongation
// before any distance is taken, which keeps a wall the same width in metres
// whichever way it runs. A peeling strip is stretched across the wood the
// same way, so `stretch` carries both: across, across and along.
fn bark_network(p: vec3<f32>, stretch: vec3<f32>) -> vec4<f32> {
    let base = floor(p);
    var first = 64.0;
    var second = 64.0;
    var nearest = vec3(0.0);
    var next = vec3(0.0);
    var seed = 0.0;
    var beside = 0.0;
    for (var z = -1; z <= 1; z++) {
        for (var y = -1; y <= 1; y++) {
            for (var x = -1; x <= 1; x++) {
                let id = base + vec3(f32(x), f32(y), f32(z));
                // Two hashes carry the site: where it stands in its own cell,
                // how big a cell it keeps, and what the plate around it is.
                let a = bark_hash(id.xy + vec2(37.0, 17.0) * id.z);
                let b = bark_hash(id.xy + vec2(11.0, 29.0) * id.z + vec2(53.1, 91.7));
                let jitter = vec3(a, b, fract(a * 43.7 + b * 71.3));
                // Cells of one size are a wall. A weight per site is what
                // makes one plate broad and its neighbour narrow off one row.
                let weight = mix(0.72, 1.28, fract(a * 17.3 + b * 31.1));
                let offset = (id + 0.15 + 0.7 * jitter - p) * stretch;
                let distance = length(offset) / weight;
                if (distance < first) {
                    second = first;
                    next = nearest;
                    beside = seed;
                    first = distance;
                    nearest = offset;
                    seed = fract(a * 91.7 + b * 13.9);
                } else if (distance < second) {
                    second = distance;
                    next = offset;
                    beside = fract(a * 91.7 + b * 13.9);
                }
            }
        }
    }
    // Dividing the difference by how fast it grows gives a boundary of
    // physical width, rather than a hairline whose width depends on how far
    // apart the two sites happened to fall. The plate across that boundary
    // keeps its own seed too, for a colour that has to change across it.
    return vec4(0.5 * (second - first) / max(length(next - nearest), 0.001), seed,
        nearest.z / max(stretch.z, 0.001), beside);
}

// Where the network's profile stands on average over a whole plate, measured
// over the field and pinned by the plate test. A trunk too far to resolve a
// plate converges here on both axes and the constant-height shortcut returns
// it, so the near and the far path agree at the boundary. A longer plate
// carries slightly more face than a round one, so one constant cannot be
// exact for every row: these are the midpoint of the shipped two, and the
// test holds both inside two hundredths of it.
const BARK_PLATE_FACE = 0.5208;
const BARK_PLATE_DOME = 0.2839;
const BARK_PLATE_RIM = 0.3891;
// How proud a plate stands of its furrow, as a fraction of its own width,
// and how much of that width the wall between the two takes.
// How far the furrow row opens the floor, as a fraction of a plate's width at
// the top of its range, and how fast each of the three levels falls per unit
// of the row. Measured over the same sweep, pinned by the same test.
const BARK_PLATE_FURROW = 0.2;
const BARK_PLATE_FURROW_FACE = 1.744;
const BARK_PLATE_FURROW_DOME = 2.376;
const BARK_PLATE_FURROW_RIM = 1.619;
const BARK_PLATE_DEPTH = 0.045;
const BARK_PLATE_WALL = 0.14;
// Peel: how many plates wide a fully curled strip is stretched across the
// wood, how far its lower edge lifts against a whole rim, and the share of a
// rim that lies on a strip's lower side - measured with the plate means.
const PEEL_WIDE = 3.0;
const PEEL_LIFT = 1.0;
const PEEL_LOWER = 0.5;
// How wide the edge of a peeled strip's colour is, in plates: a torn edge
// frays, and a white-to-black step narrower than a few pixels is one the
// tone curve aliases however exactly the footprint integrates it.
const PEEL_FRAY = 0.15;

// A curled strip's lower lip averages as that much more rim lift.
fn bark_peel_lift(edge_lift: f32, curl: f32) -> f32 {
    return edge_lift + PEEL_LIFT * PEEL_LOWER * curl;
}

// A wider furrow floor leaves less face, less dome and less rim, and each of
// the three falls off exponentially in the row: the share of a cell further
// than a given distance from its own boundary falls that way. The three rates
// are measured over the same sweep as the three levels and pinned beside them;
// the plate test holds the shipped rows and a wide furrow against the curve.
fn bark_plate_mean(dome: f32, edge_lift: f32, furrow_width: f32) -> f32 {
    let w = max(furrow_width, 0.0);
    return BARK_PLATE_FACE * exp(-BARK_PLATE_FURROW_FACE * w)
        + dome * BARK_PLATE_DOME * exp(-BARK_PLATE_FURROW_DOME * w)
        + edge_lift * BARK_PLATE_RIM * exp(-BARK_PLATE_FURROW_RIM * w);
}

// One plate's width across the run, in metres. Girth carries it, so an old
// trunk wears big plates and a young limb small ones off the same row.
fn bark_plate_size(cell_scale: f32, girth: f32) -> f32 {
    return cell_scale * girth;
}

// What the network says about one point. Relief is in units of one plate's
// width; identity is the plate's own as colour reads it. The rest is for a
// colour that changes whole from one plate to the next, the peel: this
// plate's identity and its neighbour's across the nearer boundary, the share
// of a pixel on this side of that boundary, and how much the footprint keeps.
struct BarkPlate {
    relief: f32,
    identity: f32,
    own: f32,
    beside: f32,
    side: f32,
    kept: f32,
};

// Faded to the mean on both axes: a plate too small to resolve costs no
// hash, carries no aliasing, and leaves the mean exactly. `structure` is the
// identity row, the furrow width and the peel's curl.
fn bark_plate_field(arc: vec2<f32>, along: f32, ridge_scale: f32, girth: f32,
    footprint: vec2<f32>, wander: vec2<f32>, plate: vec4<f32>,
    structure: vec3<f32>) -> BarkPlate {
    let identity = structure.x;
    let dome = plate.z;
    let edge_lift = plate.w;
    let curl = structure.z;
    let mean = bark_plate_mean(dome, bark_peel_lift(edge_lift, curl), structure.y);
    let size = bark_plate_size(plate.x, girth);
    let far = BarkPlate(mean, 0.5, 0.5, 0.5, 1.0, 0.0);
    if (size <= 0.0) { return far; }
    let run = size * (1.0 + max(plate.y, 0.0));
    // A curling strip runs across the wood: its cell is wider than a plate.
    let wide = 1.0 + PEEL_WIDE * curl;
    let span = size * wide;
    // The circumferential lattice is the arc in plate widths; the axial one is
    // the run. Both footprints are measured in their own cell units.
    // Two footprints, because there are two bands. The plate itself leaves on
    // the cell it fills, which is a width across and a run along. Its walls
    // are measured in widths in both directions once the elongation below has
    // brought the cross-cut back, so their footprint is a width in both.
    let band = max(footprint.x / span, footprint.y / run);
    let pixel = max(footprint.x, footprint.y) / size;
    // A plate leaves the picture on its own band, converging to a mean the
    // colour range below knows, so nothing steps as a trunk recedes. The wall
    // inside it is integrated by its own edge rather than faded: a band that
    // fades on a footprint two renders disagree about is a band that aliases
    // between them, however exactly its mean is preserved.
    let retained = bark_pass(band);
    if (retained <= 0.0) { return far; }
    let elongated = 1.0 + max(plate.y, 0.0);
    // Where the surface stands in the network's own space: the circle
    // embedding in plate widths, and the trunk's run in cells of one plate.
    let lattice = vec3(arc * ridge_scale / span, along / run);
    // Two warps, both filtered. The long one carries a whole furrow off the
    // vertical; the one at the plate's own scale keeps a boundary between two
    // sites from running straight, which is what a boundary between two sites
    // would otherwise do.
    let ragged = vec2(
        bark_noise2_filtered(vec2(dot(lattice.xy, vec2(0.9, -0.7)), lattice.z * 1.3),
            vec2(footprint.x * 1.14 / span, footprint.y * 1.3 / run)),
        bark_noise2_filtered(vec2(dot(lattice.xy, vec2(-0.6, 1.1)), lattice.z * 1.1),
            vec2(footprint.x * 1.30 / span, footprint.y * 1.1 / run)));
    let warped = lattice
        + vec3(0.9 * (wander - vec2(0.5)), 0.45 * (wander.x + wander.y - 1.0))
        + 0.55 * vec3(ragged.x - 0.5, ragged.y - 0.5, 0.5 * (ragged.x - ragged.y));
    // A furrow runs wherever two sites are equally near; a face is the
    // interior one site keeps to itself. That is what makes the network
    // branch and merge, and why three of its boundaries meet at a point.
    let network = bark_network(warped, vec3(wide, wide, elongated));
    let edge = network.x;
    // The floor of the furrow: the hairline the network cuts between two
    // faces, widened by the row. It is a fraction of a plate's own width, so
    // a bigger plate carries a wider furrow off one row and nothing about the
    // structure changes with girth. The walls that climb out of it are as wide
    // as the ridges' own shoulders - a furrow cut in a third of that distance
    // is a feature no footprint can integrate - and both edges are integrated
    // against the footprint, so the floor fades with everything else.
    let floor = 0.012 + BARK_PLATE_FURROW * max(structure.y, 0.0);
    let face = bark_edge(floor, floor + BARK_PLATE_WALL, edge, pixel);
    let ramp = clamp((edge - floor) / (floor + 2.0 * BARK_PLATE_WALL), 0.0, 1.0);
    let rim = face * (1.0 - bark_edge(floor + BARK_PLATE_WALL,
        floor + 2.2 * BARK_PLATE_WALL, edge, pixel));
    // What this plate keeps of its own: how proud it stands, and how far it
    // leans across its own run. A scale lifted at one edge is a plate leaning.
    let own = bark_plate_own(network.y);
    let jitter = 2.0 * own - 1.0;
    let lean = (2.0 * fract(own * 71.7) - 1.0) * clamp(network.z, -1.0, 1.0);
    let proud = 1.0 + identity * (0.40 * jitter + 0.75 * lean);
    var relief = face * proud + dome * ramp * face + edge_lift * rim * proud;
    // A curling strip lifts at its lower edge, where its own site stands above.
    // Added apart, so a row with no curl keeps the arithmetic it always had.
    if (curl > 0.0) { relief += curl * PEEL_LIFT * rim * smoothstep(0.0, 0.25, network.z); }
    // The identity is one value over a whole plate and nothing between two:
    // point-sampling it at a boundary is the one step in this field a box
    // filter cannot recover. Within a footprint of an edge it is the mean,
    // which is what a pixel straddling two plates actually averages to.
    let inside = clamp(edge / max(pixel, 1e-5), 0.0, 1.0);
    return BarkPlate(mix(mean, relief, retained), mix(0.5, own, inside * retained), own,
        bark_plate_own(network.w), clamp(0.5 + edge / max(pixel, PEEL_FRAY), 0.5, 1.0), retained);
}

// One plate's own value in 0..1, from its site's seed.
fn bark_plate_own(seed: f32) -> f32 {
    return bark_hash(vec2(seed * 53.0, seed * 131.0 + 11.0));
}

// The identity of the plate under this fragment, and the share of it peeled
// away to the inner bark. Colour takes one value for the whole plate rather
// than one per shading cell, the way the low-frequency mottle does; at
// distance both are their means and cost no hash.
//
// A share of the strips has peeled away whole. The old wood carries them, as
// it carries the relief, so the share rides on the same maturity: fewer go as
// the wood thins, and each that goes goes whole. A whole plate changing
// colour is a step no footprint integrates unless the plate across the
// boundary is known too, so the pixel takes each side's share by how much of
// it lies on that side.
fn bark_plate_identity(circle: vec2<f32>, along: f32, radius: f32,
    ridge_scale: f32, footprint: vec2<f32>, plate: vec4<f32>,
    structure: vec3<f32>) -> vec2<f32> {
    if (plate.x <= 0.0 || ridge_scale <= 0.0 || radius <= ridge_scale) {
        return vec2(0.5, 0.0);
    }
    let girth = 0.3 + 0.7 * smoothstep(2.5, 20.0, radius / ridge_scale);
    let arc = circle * radius / ridge_scale;
    let field = bark_plate_field(arc, along, ridge_scale, girth, footprint, vec2(0.5),
        plate, structure);
    let share = structure.z * smoothstep(2.0, 5.0, 2.0 * radius / ridge_scale);
    let here = mix(smooth_presence(share, field.beside), smooth_presence(share, field.own),
        field.side);
    return vec2(field.identity, mix(smooth_share(share), here, field.kept));
}

// How deep the network cuts, in the units the rest of the field is written in:
// a plate stands a fixed fraction of its own width proud of its furrow, and a
// width is metres, so the ridge scale converts it.
fn bark_plate_depth(plate: vec4<f32>, ridge_scale: f32, girth: f32) -> f32 {
    return BARK_PLATE_DEPTH * bark_plate_size(plate.x, girth) / max(ridge_scale, 1e-6);
}
