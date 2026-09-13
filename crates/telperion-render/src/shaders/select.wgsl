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
    instances: u32,
    levels: u32,
    // The u32 slots each level's list is given in the shared buffer.
    stride: u32,
    // The level every leaf in the frame is held at, or -1 to choose one.
    forced: i32,
};

@group(0) @binding(0) var<uniform> u: Selection;
@group(0) @binding(1) var<storage, read> placements: array<mat4x4<f32>>;
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
    let placement = placements[instance];
    let centre = (placement * vec4<f32>(u.sphere.xyz, 1.0)).xyz;
    // The placement scales the element, so the sphere it stands in and the
    // deviation it is judged on both travel through the longest column.
    let scale = max(
        length(placement[0].xyz),
        max(length(placement[1].xyz), length(placement[2].xyz)),
    );
    let radius = u.sphere.w * scale;
    for (var plane = 0u; plane < 6u; plane = plane + 1u) {
        if dot(u.planes[plane].xyz, centre) + u.planes[plane].w < -radius {
            return u.levels;
        }
    }
    if u.forced >= 0 {
        return min(u32(u.forced), u.levels - 1u);
    }
    // A perspective projection shrinks by depth alone, so one division gives
    // the pixels a metre spans where this leaf stands.
    let depth = max(dot(centre - u.eye.xyz, u.forward.xyz), u.forward.w);
    let pixels = u.eye.w / depth;
    for (var level = 0u; level < u.levels; level = level + 1u) {
        if deviations[level] * scale * pixels < THRESHOLD {
            return level;
        }
    }
    // The finest level is the element itself at deviation zero, so the loop
    // above always answers; this is what the compiler needs to hear.
    return u.levels - 1u;
}

@compute @workgroup_size(256)
fn select(
    @builtin(global_invocation_id) global: vec3<u32>,
    @builtin(local_invocation_index) local: u32,
    @builtin(workgroup_id) group: vec3<u32>,
) {
    for (var word = local; word < (u.levels + 1u) * WORDS; word += WORKGROUP) {
        atomicStore(&members[word], 0u);
    }
    workgroupBarrier();

    // Tail threads participate in every barrier but claim no membership.
    let instance = global.x;
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
        scratch[group_slot(local, group.x)] = count;
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
fn scatter(@builtin(global_invocation_id) global: vec3<u32>) {
    let instance = global.x;
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
