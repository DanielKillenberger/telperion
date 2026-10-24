//! The replay's buffers, uploaded from a capture, and each kernel set's
//! bind groups: chained (each phase reads the one before on the device)
//! for timing, isolated (each phase reads the CPU's inputs) for agreement.
use crate::cap::Cap;
use crate::gpu::{Gpu, Kit};
use crate::tree::{Blocks, Shape};
use wgpu::Buffer;

pub struct Sizes {
    pub n: usize,
    pub edges: usize,
    pub groups: usize,
    pub leaves: usize,
    pub tips: usize,
    pub nodes: usize,
    pub layer: usize,
    pub nz: usize,
    pub dirs: usize,
    pub apexes: usize,
    pub extinction: f32,
    pub cycle: u32,
    pub before: u32,
}

/// Every buffer, by name.
pub struct Bufs {
    pub map: std::collections::HashMap<&'static str, Buffer>,
    pub scratch: std::collections::HashMap<&'static str, u64>,
}

impl Bufs {
    pub fn get(&self, name: &str) -> &Buffer {
        self.map
            .get(name)
            .unwrap_or_else(|| panic!("buffer {name}"))
    }
}

pub fn upload(gpu: &Gpu, cap: &Cap, shape: &Shape, blocks: &Blocks) -> (Bufs, Sizes) {
    let mut map = std::collections::HashMap::new();
    let mut scratch = std::collections::HashMap::new();
    let n = shape.n;
    let live = cap.u32s("live");
    let mut flag = vec![0u32; n];
    for &a in &live {
        flag[a as usize] = 1;
    }
    let tab_names = [
        "pa_upkeep",
        "pa_hazard",
        "pa_tolerance",
        "pa_lambda",
        "pa_psi",
        "pa_window",
        "pa_viability",
        "pa_shade_hazard",
    ];
    let cols: Vec<Vec<f32>> = tab_names.iter().map(|t| cap.f32s(t)).collect();
    let groups = cols[0].len();
    let tab: Vec<f32> = (0..groups)
        .flat_map(|g| cols.iter().map(move |c| c[g]))
        .collect();
    let grid = cap.u32s("grid_size");
    let nodes = grid.iter().map(|&s| s as usize).product::<usize>();
    let dirs = cap.f32s("directions").len() / 4;
    let leaves = cap.f32s("leaves");
    let tips = cap.f32s("tips");
    let sizes = Sizes {
        n,
        edges: shape.by_index.len(),
        groups,
        leaves: leaves.len() / 4,
        tips: tips.len() / 3,
        nodes,
        layer: (grid[0] * grid[1]) as usize,
        nz: grid[2] as usize,
        dirs,
        apexes: cap.u32s("fate_axis").len(),
        extinction: cap.f32s("extinction")[0],
        cycle: cap.u32s("cycle")[0],
        before: cap.u32s("axes_before")[0],
    };
    let mut put = |name: &'static str, b: Buffer| {
        map.insert(name, b);
    };
    // Inputs.
    put("parent", gpu.storage(&cap.u32s("parent")));
    put("node", gpu.storage(&cap.u32s("node")));
    put("pa", gpu.storage(&cap.u32s("pa")));
    put("ch_start", gpu.storage(&shape.start));
    put("by_index", gpu.storage(&shape.by_index));
    put("by_node", gpu.storage(&shape.by_node));
    put("lineage", gpu.storage(&cap.u64s("lineage")));
    put("tab", gpu.storage(&tab));
    for name in ["base_scale", "running", "light", "wood", "memory"] {
        put(name, gpu.storage(&cap.f32s(name)));
    }
    put("live", gpu.storage(&flag));
    // The CPU's phase outputs, as the isolated phases' inputs.
    put("cpu_tot_a", gpu.storage(&cap.f32s("cpu_total_light")));
    put("cpu_tot_b", gpu.storage(&cap.f32s("cpu_total_cost")));
    put("cpu_living", gpu.storage(&cap.u32s("cpu_living")));
    put("cpu_lit", gpu.storage(&cap.f32s("cpu_lit")));
    put("cpu_sub", gpu.storage(&cap.f32s("cpu_sub")));
    put("cpu_bud", gpu.storage(&cap.f32s("cpu_bud")));
    put("cpu_presence", gpu.storage(&cap.f32s("cpu_presence")));
    put("cpu_lattice", gpu.storage(&cap.f32s("lattice")));
    // Outputs shared by both schedules.
    let f = |k: usize| gpu.zeroed(4 * k as u64);
    for name in [
        "own_a", "own_b", "tot_a", "tot_b", "living", "shed", "kept", "mem_out", "lit", "pres",
        "sub", "incoming", "bud", "sizes",
    ] {
        put(name, f(n));
    }
    put("sdraws", f(2 * n));
    put("gacc", f(4 * groups));
    put("sums", f(2 * groups));
    // Schedule A's scratch: the integer sums, the local decisions, the
    // incoming beside the shares.
    let a_scratch = [
        ("acc_bal", 4 * n),
        ("liv_acc", n),
        ("acc_alloc", 4 * n),
        ("loc", 4 * n),
        ("rest", n.max(sizes.edges)),
    ];
    for (name, k) in a_scratch {
        put(name, f(k));
        scratch.insert(name, 4 * k as u64);
    }
    // Schedule B's blocks.
    let b_arrays: [(&'static str, &[u32]); 7] = [
        ("blist", &blocks.by_level),
        ("up_meta", &blocks.up_meta),
        ("up_offs", &blocks.up_offs),
        ("up_nodes", &blocks.up_nodes),
        ("down_meta", &blocks.down_meta),
        ("down_offs", &blocks.down_offs),
        ("down_nodes", &blocks.down_nodes),
    ];
    for (name, v) in b_arrays {
        put(name, gpu.storage(v));
        scratch.insert(name, 4 * v.len() as u64);
    }
    // Light.
    put("leaves", gpu.storage(&leaves));
    put("tips", gpu.storage(&tips));
    put("box", f(6));
    put("quanta", f(nodes));
    put("density", f(nodes));
    put("dirs", gpu.storage(&cap.f32s("directions")));
    put("dlight", f(dirs * nodes));
    put("depth_wg", f(dirs * 2 * sizes.layer));
    put("depth_layer", f(dirs * nodes));
    put("lattice", f(nodes));
    put("reads", f(sizes.tips));
    put("reads_cpu", f(sizes.tips));
    put("flags", f(1));
    scratch.insert("dlight", 4 * (dirs * nodes) as u64);
    scratch.insert("depth_wg", 4 * (dirs * 2 * sizes.layer) as u64);
    scratch.insert("depth_layer", 4 * (dirs * nodes) as u64);
    // Fate and births.
    put("keys", gpu.storage(&cap.u64s("fate_key")));
    put("shares", gpu.storage(&cap.u64s("fate_share")));
    put("fate_light", gpu.storage(&cap.f32s("fate_light")));
    put("fate_pa", gpu.storage(&cap.u32s("fate_pa")));
    put("births", gpu.storage(&cap.u32s("cpu_births")));
    put("thr", f(2 * sizes.apexes));
    put("fdraws", f(2 * sizes.apexes));
    put("survives", f(sizes.apexes));
    put("first", f(sizes.apexes));
    put("block_sums", f(sizes.apexes.div_ceil(256) + 1));
    (Bufs { map, scratch }, sizes)
}

/// The kernels' bind groups, one set per mode.
pub struct Binds {
    pub sum_bal_a: wgpu::BindGroup,
    pub sum_bal_b: wgpu::BindGroup,
    pub sum_alloc_a: wgpu::BindGroup,
    pub sum_alloc_b: wgpu::BindGroup,
    pub shed: wgpu::BindGroup,
    pub flat: wgpu::BindGroup,
    pub split: wgpu::BindGroup,
    pub light_read: wgpu::BindGroup,
    pub fate: wgpu::BindGroup,
}

pub struct Kits {
    pub sum: Kit,
    pub shed: Kit,
    pub flat: Kit,
    pub split: Kit,
    pub light: Kit,
    pub fate: Kit,
}

/// The bind groups; `chained` reads each phase's input from the phase
/// before, else from the CPU's capture. Each unused slot holds its own
/// small buffer, so no writable buffer is bound twice.
pub fn binds(gpu: &Gpu, k: &Kits, b: &Bufs, uniform: &Buffer, chained: bool) -> Binds {
    let dummies: Vec<Buffer> = (0..16).map(|_| gpu.zeroed(16)).collect();
    let d = |i: usize| &dummies[i];
    let g = |name: &str| b.get(name);
    let pick = |dev: &'static str, cpu: &'static str| if chained { g(dev) } else { g(cpu) };
    let up = [g("blist"), g("up_meta"), g("up_offs"), g("up_nodes")];
    let down = [g("blist"), g("down_meta"), g("down_offs"), g("down_nodes")];
    let sum_bal = |acc: &'static str| {
        let mut v = vec![
            g("parent"),
            g("ch_start"),
            g("by_index"),
            g("own_a"),
            g("own_b"),
            g("live"),
        ];
        v.extend([g("tot_a"), g("tot_b"), g("living"), g(acc), g("liv_acc")]);
        v.extend(up);
        gpu.bind(&k.sum, uniform, &v)
    };
    let sum_alloc = |acc: &'static str| {
        let mut v = vec![
            g("parent"),
            g("ch_start"),
            g("by_index"),
            pick("lit", "cpu_lit"),
            d(0),
            d(1),
        ];
        v.extend([g("sub"), d(2), d(3), g(acc), d(4)]);
        v.extend(up);
        gpu.bind(&k.sum, uniform, &v)
    };
    let mut shed = vec![g("parent"), g("node"), g("pa"), g("lineage"), g("tab")];
    shed.extend([
        pick("tot_a", "cpu_tot_a"),
        pick("tot_b", "cpu_tot_b"),
        pick("living", "cpu_living"),
    ]);
    shed.extend([g("memory"), g("loc"), g("shed"), g("kept"), g("mem_out")]);
    shed.extend(down);
    shed.push(g("sdraws"));
    let mut flat = vec![
        g("pa"),
        g("tab"),
        g("base_scale"),
        g("running"),
        g("light"),
        g("wood"),
        g("live"),
    ];
    flat.extend([g("own_a"), g("own_b"), g("shed"), g("kept"), g("lit")]);
    flat.extend([
        pick("pres", "cpu_presence"),
        pick("bud", "cpu_bud"),
        g("gacc"),
        g("sizes"),
        g("sums"),
    ]);
    let mut split = vec![
        g("parent"),
        g("ch_start"),
        g("by_node"),
        g("node"),
        g("pa"),
        g("tab"),
    ];
    split.extend([
        pick("lit", "cpu_lit"),
        pick("sub", "cpu_sub"),
        g("incoming"),
        g("bud"),
        g("rest"),
    ]);
    split.extend(down);
    let mut light_read = vec![g("leaves"), g("tips"), g("box"), d(5), d(6), d(7), d(8)];
    light_read.extend([d(9), d(10), g("reads"), d(11), g("lattice")]);
    let fate = [
        "keys",
        "shares",
        "fate_light",
        "fate_pa",
        "tab",
        "thr",
        "fdraws",
        "survives",
        "births",
        "first",
        "block_sums",
    ]
    .map(g);
    Binds {
        sum_bal_a: sum_bal("acc_bal"),
        sum_bal_b: sum_bal("acc_bal"),
        sum_alloc_a: sum_alloc("acc_alloc"),
        sum_alloc_b: sum_alloc("acc_alloc"),
        shed: gpu.bind(&k.shed, uniform, &shed),
        flat: gpu.bind(&k.flat, uniform, &flat),
        split: gpu.bind(&k.split, uniform, &split),
        light_read: gpu.bind(&k.light, uniform, &light_read),
        fate: gpu.bind(&k.fate, uniform, &fate),
    }
}

/// The light kit's bind group with `depth` as its depths (the two sweeps
/// keep theirs apart).
pub fn light_with(gpu: &Gpu, k: &Kits, b: &Bufs, uniform: &Buffer, depth: &str) -> wgpu::BindGroup {
    let g = |name: &str| b.get(name);
    let mut light = vec![
        g("leaves"),
        g("tips"),
        g("box"),
        g("quanta"),
        g("density"),
        g("dirs"),
        g("dlight"),
    ];
    light.extend([
        g(depth),
        g("lattice"),
        g("reads_cpu"),
        g("flags"),
        g("cpu_lattice"),
    ]);
    gpu.bind(&k.light, uniform, &light)
}
