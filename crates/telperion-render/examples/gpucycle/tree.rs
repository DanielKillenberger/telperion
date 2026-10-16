//! The hierarchy's host-side shape: children by index and by (node,
//! index), depths, and schedule B's bounded blocks. In a device grower
//! these would be maintained at birth; here they are built once, timed
//! apart and reported as preparation.
pub const NONE: u32 = u32::MAX;

pub struct Shape {
    pub n: usize,
    pub start: Vec<u32>,
    /// Children in index order, and in (node, index) order.
    pub by_index: Vec<u32>,
    pub by_node: Vec<u32>,
    pub max_depth: u32,
}

pub fn shape(parent: &[u32], node: &[u32]) -> Shape {
    let n = parent.len();
    let mut count = vec![0u32; n + 1];
    for &p in parent.iter().filter(|&&p| p != NONE) {
        count[p as usize] += 1;
    }
    let mut start = vec![0u32; n + 1];
    for i in 0..n {
        start[i + 1] = start[i] + count[i];
    }
    let mut fill = start.clone();
    let mut by_index = vec![0u32; start[n] as usize];
    let mut depth = vec![0u32; n];
    for (i, &p) in parent.iter().enumerate() {
        if p != NONE {
            assert!((p as usize) < i, "parents precede children");
            by_index[fill[p as usize] as usize] = i as u32;
            fill[p as usize] += 1;
            depth[i] = depth[p as usize] + 1;
        }
    }
    let mut by_node = by_index.clone();
    for i in 0..n {
        let (s, e) = (start[i] as usize, start[i + 1] as usize);
        by_node[s..e].sort_by_key(|&c| (node[c as usize], c));
    }
    let max_depth = depth.iter().copied().max().unwrap_or(0);
    Shape {
        n,
        start,
        by_index,
        by_node,
        max_depth,
    }
}

/// Schedule B: the tree cut into connected blocks of at most `cap` axes,
/// each block's axes by local height (bottom-up) and by local depth
/// (top-down), and the blocks by level.
pub struct Blocks {
    pub count: usize,
    pub levels: u32,
    /// Block ids, bottom-up level by level, and each level's first.
    pub by_level: Vec<u32>,
    pub level_start: Vec<u32>,
    /// Per block: its first offset in `up_offs` and its heights; its first
    /// in `down_offs` and its depths.
    pub up_meta: Vec<u32>,
    pub up_offs: Vec<u32>,
    pub up_nodes: Vec<u32>,
    pub down_meta: Vec<u32>,
    pub down_offs: Vec<u32>,
    pub down_nodes: Vec<u32>,
    pub largest: usize,
    pub tallest: u32,
}

pub fn blocks(parent: &[u32], cap: u32) -> Blocks {
    let n = parent.len();
    let mut size = vec![1u32; n];
    let mut cut = vec![false; n];
    cut[0] = true;
    for i in (1..n).rev() {
        let p = parent[i] as usize;
        if size[p] + size[i] <= cap {
            size[p] += size[i];
        } else {
            cut[i] = true;
        }
    }
    let mut block = vec![0u32; n];
    let mut roots = Vec::new();
    let mut depth = vec![0u32; n];
    for i in 0..n {
        if cut[i] {
            block[i] = roots.len() as u32;
            roots.push(i);
        } else {
            let p = parent[i] as usize;
            block[i] = block[p];
            depth[i] = depth[p] + 1;
        }
    }
    let count = roots.len();
    let mut height = vec![0u32; n];
    for i in (1..n).rev() {
        if !cut[i] {
            let p = parent[i] as usize;
            height[p] = height[p].max(height[i] + 1);
        }
    }
    let mut level = vec![0u32; count];
    for &r in roots.iter().rev().filter(|&&r| r != 0) {
        let (b, pb) = (block[r] as usize, block[parent[r] as usize] as usize);
        level[pb] = level[pb].max(level[b] + 1);
    }
    let levels = level.iter().copied().max().unwrap_or(0) + 1;
    let mut by_level: Vec<u32> = (0..count as u32).collect();
    by_level.sort_by_key(|&b| (level[b as usize], b));
    let mut level_start = vec![0u32; levels as usize + 1];
    for &b in &by_level {
        level_start[level[b as usize] as usize + 1] += 1;
    }
    for l in 0..levels as usize {
        level_start[l + 1] += level_start[l];
    }
    let mut members: Vec<Vec<u32>> = vec![Vec::new(); count];
    for i in 0..n {
        members[block[i] as usize].push(i as u32);
    }
    let lay = |key: &dyn Fn(u32) -> u32| {
        let (mut meta, mut offs, mut nodes) = (Vec::new(), Vec::new(), Vec::new());
        for m in &members {
            let mut m = m.clone();
            m.sort_by_key(|&i| (key(i), i));
            let top = m.iter().map(|&i| key(i)).max().unwrap_or(0);
            meta.extend([offs.len() as u32, top]);
            let mut at = 0;
            for h in 0..=top + 1 {
                while at < m.len() && key(m[at]) < h {
                    at += 1;
                }
                offs.push((nodes.len() + at) as u32);
            }
            nodes.extend(&m);
        }
        (meta, offs, nodes)
    };
    let (up_meta, up_offs, up_nodes) = lay(&|i| height[i as usize]);
    let (down_meta, down_offs, down_nodes) = lay(&|i| depth[i as usize]);
    let largest = members.iter().map(Vec::len).max().unwrap_or(0);
    let tallest = height
        .iter()
        .copied()
        .max()
        .unwrap_or(0)
        .max(depth.iter().copied().max().unwrap_or(0));
    Blocks {
        count,
        levels,
        by_level,
        level_start,
        up_meta,
        up_offs,
        up_nodes,
        down_meta,
        down_offs,
        down_nodes,
        largest,
        tallest,
    }
}

/// `x` in units of 2^-32, rounded half away from zero: schedule A's
/// integer sums, as the device takes them.
pub fn fixed(x: f32) -> u64 {
    assert!(x >= 0.0 && x < 4.0e9, "a sum inside the fixed range");
    (f64::from(x) * 4_294_967_296.0).round() as u64
}

/// Each subtree's sum of `own` in fixed point: the exact reference for A.
pub fn fixed_sums(parent: &[u32], own: &[f32]) -> Vec<u64> {
    let mut sum: Vec<u64> = own.iter().map(|&x| fixed(x)).collect();
    for i in (1..parent.len()).rev() {
        let p = parent[i] as usize;
        sum[p] += sum[i];
    }
    sum
}
