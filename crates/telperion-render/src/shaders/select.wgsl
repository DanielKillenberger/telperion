// One thread per leaf, once per frame: which level of the element it is drawn
// at, or the bucket for the leaves this frame does not show. A level is chosen
// on its own deviation projected to pixels, so the switch is never a visible
// one; nothing is remembered between frames, so nothing has to be forgotten
// when the camera cuts or the tree is submitted again.
//
// Each thread appends its own index to the chosen level's list. The place in
// that list is reserved per workgroup, not per leaf: the workgroup tallies its
// own threads first and takes one range per level from the shared counter.

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

const WORKGROUP: u32 = 256u;
/// The tally lives in workgroup memory, which is sized when the shader is
/// compiled. The ladder doubles, so sixteen levels is an element of tens of
/// thousands of triangles; the submission refuses anything past it by name.
const MAX_LEVELS: u32 = 16u;
/// Index count, instance count, first index, base vertex, first instance.
const ARGUMENT_WORDS: u32 = 5u;
/// Below half a pixel of deviation there is nothing on screen to see.
const THRESHOLD: f32 = 0.5;

/// This workgroup's own leaves per level, and where its run of each level's
/// list begins. The unseen bucket is counted like a level and written nowhere.
var<workgroup> tally: array<atomic<u32>, MAX_LEVELS + 1u>;
var<workgroup> base: array<u32, MAX_LEVELS + 1u>;

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
) {
    if local <= u.levels {
        atomicStore(&tally[local], 0u);
    }
    workgroupBarrier();

    // The last workgroup runs past the end of the crown: those threads choose
    // nothing and write nothing, but they reach every barrier the others do.
    let instance = global.x;
    let leaf = instance < u.instances;
    var chosen = u.levels;
    var slot = 0u;
    if leaf {
        chosen = level_of(instance);
        slot = atomicAdd(&tally[chosen], 1u);
    }
    workgroupBarrier();

    if local <= u.levels {
        let counted = atomicLoad(&tally[local]);
        base[local] = 0u;
        if counted > 0u {
            base[local] = atomicAdd(&counts[local], counted);
            if local < u.levels {
                atomicAdd(&arguments[local * ARGUMENT_WORDS + 1u], counted);
            }
        }
    }
    workgroupBarrier();

    if leaf && chosen < u.levels {
        lists[chosen * u.stride + base[chosen] + slot] = instance;
    }
}
