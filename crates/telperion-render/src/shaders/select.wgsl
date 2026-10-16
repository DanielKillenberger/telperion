// One thread per leaf, once per frame: which level of the element it is drawn
// at, or the bucket for the leaves this frame does not show. A level is chosen
// on its own deviation projected to pixels, so the switch is never a visible
// one; nothing is remembered between frames, so nothing has to be forgotten
// when the camera cuts or the tree is submitted again.
//
// Three dispatches compact each level in placement-index order: classify and
// rank within each workgroup, prefix the group counts, then scatter. Equal
// depth samples therefore resolve in the same order on every draw.

struct Selection {
    planes: array<vec4<f32>, 6>,
    // The eye, and the pixels one metre spans at one metre of depth.
    eye: vec4<f32>,
    // The direction the eye looks, and the near plane behind which depth is
    // not a number to divide by.
    forward: vec4<f32>,
    // The element's own bounding sphere at the origin: centre, then radius.
    sphere: vec4<f32>,
    // The box the core quantised every leaf position against.
    box_min: vec4<f32>,
    box_extent: vec4<f32>,
    instances: u32,
    levels: u32,
    // The u32 slots each level's list is given in the shared buffer.
    stride: u32,
    // The level every leaf in the frame is held at, or -1 to choose one.
    forced: i32,
    // The rachis's and the pedicel's lengths on the organ (fn-253), how
    // many levels, coarsest first, are the rachis's, and its droop; zero on
    // the leaves.
    stalk: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: Selection;
@group(0) @binding(1) var<storage, read> placements: array<u32>;
/// One deviation in metres per level, coarsest first, the finest exactly zero.
@group(0) @binding(2) var<storage, read> deviations: array<f32>;
/// Every level's list end to end, `u.stride` slots each.
@group(0) @binding(3) var<storage, read_write> lists: array<u32>;
/// One counter per level, and the unseen bucket last.
@group(0) @binding(4) var<storage, read_write> counts: array<atomic<u32>>;
/// One indexed indirect draw per level, five words each.
@group(0) @binding(5) var<storage, read_write> arguments: array<atomic<u32>>;
/// Packed local ranks, then one count/offset per level (including unseen) and group.
@group(0) @binding(6) var<storage, read_write> scratch: array<u32>;

const WORKGROUP: u32 = 256u;
/// The tally lives in workgroup memory, which is sized when the shader is
/// compiled. The ladder doubles, so sixteen levels is an element of tens of
/// thousands of triangles; the submission refuses anything past it by name.
const MAX_LEVELS: u32 = 16u;
/// Index count, instance count, first index, base vertex, first instance.
const ARGUMENT_WORDS: u32 = 5u;
/// Below half a pixel of deviation there is nothing on screen to see.
const THRESHOLD: f32 = 0.5;

/// Eight membership words per level; atomic OR is independent of arrival order.
const WORDS: u32 = WORKGROUP / 32u;
var<workgroup> members: array<atomic<u32>, (MAX_LEVELS + 1u) * WORDS>;
var<workgroup> sums: array<u32, WORKGROUP>;

fn group_count() -> u32 {
    return (u.instances + WORKGROUP - 1u) / WORKGROUP;
}

fn group_slot(level: u32, group: u32) -> u32 {
    return u.instances + level * group_count() + group;
}

/// The coarsest level whose deviation stays under half a pixel at this leaf's
/// depth, or the unseen bucket for a leaf whose sphere is outside the frame.
fn level_of(instance: u32) -> u32 {
    let base = instance * LEAF_WORDS;
    let words = vec3<u32>(placements[base], placements[base + 1u], placements[base + 2u]);
    // An organ's inflorescence (fn-253) is thinned as one, by its size from
    // its seat, so no floret is kept without the rachis it hangs from.
    // It is culled whole too, by the sphere it is drawn in once enlarged.
    var kept = 1.0;
    let unit = u.stalk.x > 0.0;
    let stems = u32(u.stalk.z);
    if (unit) {
        let r = unit_rachis(instance);
        let rachis = unit_words(r);
        let seat = leaf_position(rachis, u.box_min.xyz, u.box_extent.xyz);
        let reach = u.stalk.x + u.stalk.y + u.sphere.w;
        let at = max(dot(seat - u.eye.xyz, u.forward.xyz), u.forward.w);
        kept = leaf_kept(2.0 * reach * u.eye.w / at);
        if leaf_draw_rank(rachis) >= kept {
            return u.levels;
        }
        // Culled by all it can reach: a floret's sphere stands off its
        // base, and a floret is at most twice its element (`bloom.rs`, a
        // scatter of at most one), offset and radius alike.
        let bound = u.stalk.x + u.stalk.y + 2.0 * (length(u.sphere.xyz) + u.sphere.w);
        for (var plane = 0u; plane < 6u; plane = plane + 1u) {
            if dot(u.planes[plane].xyz, seat) + u.planes[plane].w < -bound / sqrt(kept) {
                return u.levels;
            }
        }
        if r == instance {
            // Pixels a metre of the enlarged unit spans at its seat.
            let pixels = u.eye.w / at / sqrt(kept);
            let heading = leaf_rotation(rachis.x)[1];
            return rachis_level(stems, seat, heading, 2.0 * leaf_scale(rachis) * pixels, pixels);
        }
    }
    // The placement scales the element, and the scale is a word of its own
    // now: the sphere the leaf stands in and the deviation it is judged on
    // both read it directly rather than measuring a column to find it.
    let scale = leaf_scale(words);
    let centre = leaf_position(words, u.box_min.xyz, u.box_extent.xyz)
        + leaf_rotation(words.x) * (u.sphere.xyz * scale);
    let radius = u.sphere.w * scale;
    for (var plane = 0u; plane < 6u; plane = plane + 1u) {
        if !unit && dot(u.planes[plane].xyz, centre) + u.planes[plane].w < -radius {
            return u.levels;
        }
    }
    // A perspective projection shrinks by depth alone, so one division gives
    // the pixels a metre spans where this leaf stands.
    let depth = max(dot(centre - u.eye.xyz, u.forward.xyz), u.forward.w);
    let pixels = u.eye.w / depth;
    // A leaf under THIN_PIXELS across is drawn by a share of the crown that
    // follows its size, the rest given to the unseen bucket (fn-246); a
    // floret by its inflorescence's, above.
    if (u.stalk.x <= 0.0) {
        kept = leaf_kept(2.0 * radius * pixels);
        if leaf_draw_rank(words) >= kept {
            return u.levels;
        }
    }
    // A forced level is held within the floret's own ladder, behind the
    // rachis's levels.
    if u.forced >= 0 {
        return clamp(u32(u.forced), stems, u.levels - 1u);
    }
    // A kept leaf is drawn enlarged, and judged at the size it is drawn.
    let drawn = scale / sqrt(kept);
    for (var level = 0u; level < u.levels; level = level + 1u) {
        if deviations[level] * drawn * pixels < THRESHOLD {
            return level;
        }
    }
    // The finest level is the element itself at deviation zero, so the loop
    // above always answers; this is what the compiler needs to hear.
    return u.levels - 1u;
}

/// A rachis's level among the first `stems` (`stalk.rs`): a forced one held
/// within them; its tube where its seat is three pixels thick or more;
/// below, the coarsest ribbon whose chords stand under half a pixel off the
/// axis the pedicels are seated on, `pixels` a metre.
fn rachis_level(stems: u32, seat: vec3<f32>, heading: vec3<f32>, width: f32, pixels: f32) -> u32 {
    let finest = max(stems, 1u) - 1u;
    if u.forced >= 0 {
        return min(u32(u.forced), finest);
    }
    if width >= 3.0 || finest == 0u {
        return finest;
    }
    // The ribbons join every fourth ring, every second, then every one.
    var stride = 4u;
    for (var level = 0u; level + 1u < finest; level++) {
        if rachis_error(seat, heading, u.stalk.x, u.stalk.w, stride) * pixels < THRESHOLD {
            return level;
        }
        stride /= 2u;
    }
    return finest - 1u;
}

/// The leaf a thread of a per-leaf pass stands for. A crown past one
/// dimension of groups is dispatched in rows, `grid.x` groups wide.
fn leaf_index(global: vec3<u32>, grid: vec3<u32>) -> u32 {
    return global.x + global.y * grid.x * WORKGROUP;
}

@compute @workgroup_size(256)
fn select(
    @builtin(global_invocation_id) global: vec3<u32>,
    @builtin(local_invocation_index) local: u32,
    @builtin(workgroup_id) id: vec3<u32>,
    @builtin(num_workgroups) grid: vec3<u32>,
) {
    // The last row's overhang has no leaves and no slot of its own; the whole
    // group returns together, so every barrier below is still uniform.
    let group = id.x + id.y * grid.x;
    if group >= group_count() {
        return;
    }
    for (var word = local; word < (u.levels + 1u) * WORDS; word += WORKGROUP) {
        atomicStore(&members[word], 0u);
    }
    workgroupBarrier();

    // Tail threads participate in every barrier but claim no membership.
    let instance = leaf_index(global, grid);
    var chosen = u.levels;
    if instance < u.instances {
        chosen = level_of(instance);
        atomicOr(&members[chosen * WORDS + local / 32u], 1u << (local % 32u));
    }
    workgroupBarrier();

    if instance < u.instances {
        let word = local / 32u;
        var rank = countOneBits(atomicLoad(&members[chosen * WORDS + word])
            & ((1u << (local % 32u)) - 1u));
        for (var before = 0u; before < word; before += 1u) {
            rank += countOneBits(atomicLoad(&members[chosen * WORDS + before]));
        }
        scratch[instance] = (chosen << 8u) | rank;
    }
    if local <= u.levels {
        var count = 0u;
        for (var word = 0u; word < WORDS; word += 1u) {
            count += countOneBits(atomicLoad(&members[local * WORDS + word]));
        }
        scratch[group_slot(local, group)] = count;
    }
}

/// One workgroup per level scans contiguous chunks of its group counts. The
/// scan has a bounded shared-memory cost even for crowns of millions of leaves.
@compute @workgroup_size(256)
fn prefix(
    @builtin(local_invocation_index) local: u32,
    @builtin(workgroup_id) group: vec3<u32>,
) {
    let level = group.x;
    let groups = group_count();
    let chunk = (groups + WORKGROUP - 1u) / WORKGROUP;
    let start = min(local * chunk, groups);
    let end = min(start + chunk, groups);
    var total = 0u;
    for (var i = start; i < end; i += 1u) {
        total += scratch[group_slot(level, i)];
    }
    sums[local] = total;
    workgroupBarrier();
    for (var step = 1u; step < WORKGROUP; step *= 2u) {
        var previous = 0u;
        if local >= step {
            previous = sums[local - step];
        }
        workgroupBarrier();
        sums[local] += previous;
        workgroupBarrier();
    }
    var offset = sums[local] - total;
    for (var i = start; i < end; i += 1u) {
        let slot = group_slot(level, i);
        let count = scratch[slot];
        scratch[slot] = offset;
        offset += count;
    }
    if local == WORKGROUP - 1u {
        atomicStore(&counts[level], sums[local]);
        if level < u.levels {
            atomicStore(&arguments[level * ARGUMENT_WORDS + 1u], sums[local]);
        }
    }
}

@compute @workgroup_size(256)
fn scatter(
    @builtin(global_invocation_id) global: vec3<u32>,
    @builtin(num_workgroups) grid: vec3<u32>,
) {
    let instance = leaf_index(global, grid);
    if instance >= u.instances {
        return;
    }
    let packed = scratch[instance];
    let level = packed >> 8u;
    if level < u.levels {
        let offset = scratch[group_slot(level, instance / WORKGROUP)];
        lists[level * u.stride + offset + (packed & 255u)] = instance;
    }
}
