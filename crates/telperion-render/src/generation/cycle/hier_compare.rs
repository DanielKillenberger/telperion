//! The hierarchy and its sums read back and held to the CPU's: each
//! axis's children in order, every integer and every f32 the CPU takes
//! at an open axis, each pencil's memory, kept share and size, the apexes
//! shed and the living after. The blocks are checked against their rules
//! (connected, at most `cap`, levels and local depths), which no CPU
//! holds.
use super::compare::Agreement;
use super::hier_build::{Hierarchy, LISTS, SHAPE, read_words};
use super::hier_columns::{Cpu, Hier};
use super::hier_device::{
    LIVE, S_BUD, S_COST, S_GONE, S_INCOMING, S_KEPT, S_LIGHT, S_LIVING, S_MEMORY, S_SIZE, S_SUB,
    SUMS, Sums, V_OUT,
};
use crate::{Gpu, Result};

/// The hierarchy as the device left it: its current lists and shape.
pub(super) struct Shape {
    pub stride: usize,
    pub lists: Vec<u32>,
    pub shape: Vec<u32>,
}

impl Hierarchy {
    pub(super) fn read(&self, gpu: &Gpu) -> Result<Shape> {
        let s = u64::from(self.stride);
        Ok(Shape {
            stride: self.stride as usize,
            lists: read_words(gpu, &self.lists[self.parity], 0, LISTS * (s + 64))?,
            shape: read_words(gpu, &self.shape, 0, SHAPE * s)?,
        })
    }
}

impl Shape {
    fn list(&self, col: usize) -> &[u32] {
        let s = self.stride + 64;
        &self.lists[col * s..(col + 1) * s]
    }

    fn col(&self, col: usize) -> &[u32] {
        &self.shape[col * self.stride..(col + 1) * self.stride]
    }

    /// Each axis's children: `n + 1` starts, then the children.
    pub fn children(&self, n: usize) -> (Vec<u32>, Vec<u32>) {
        let start = self.list(0)[..=n].to_vec();
        let kids = self.list(1)[..start[n] as usize].to_vec();
        (start, kids)
    }
}

/// The sums as the device left them.
pub(super) struct Out {
    pub sums: Vec<u32>,
    pub live: Vec<u32>,
    pub live_n: usize,
    stride: usize,
    live_stride: usize,
}

impl Sums {
    pub(super) fn read(&self, gpu: &Gpu, hier: &Hier) -> Result<Out> {
        let s = u64::from(hier.stride);
        let live_n = read_words(gpu, &self.tally, 0, 1)?[0] as usize;
        Ok(Out {
            sums: read_words(gpu, &self.sums, 0, SUMS * s)?,
            live: read_words(gpu, &self.live, 0, LIVE * u64::from(self.live_stride))?,
            live_n,
            stride: hier.stride as usize,
            live_stride: self.live_stride as usize,
        })
    }
}

impl Out {
    fn col(&self, c: u64, n: usize) -> &[u32] {
        let at = c as usize * self.stride;
        &self.sums[at..at + n]
    }

    fn live_col(&self, c: u64) -> &[u32] {
        let at = c as usize * self.live_stride;
        &self.live[at..at + self.live_n]
    }
}

/// The device's children against the CPU's.
pub(super) fn children(a: &mut Agreement, shape: &Shape, hier: &Hier, cpu: &Cpu) {
    let (start, kids) = shape.children(hier.n as usize);
    a.eq("children starts", &start, &cpu.kid_start);
    a.eq("children", &kids, &cpu.kids);
}

/// The device's integers and its reals against the CPU's.
pub(super) fn compare(hier: &Hier, cpu: &Cpu, out: &Out) -> (Agreement, Agreement) {
    let n = hier.n as usize;
    let at_open = |c: u64| -> Vec<u32> {
        cpu.open
            .iter()
            .map(|&i| out.col(c, n)[i as usize])
            .collect()
    };
    let cpu_open = |v: &[u32]| -> Vec<u32> { cpu.open.iter().map(|&i| v[i as usize]).collect() };
    let mut ints = Agreement::default();
    ints.eq(
        "living at the open axes",
        &at_open(S_LIVING),
        &cpu_open(&cpu.living),
    );
    ints.eq("apexes shed", out.col(S_GONE, n), &cpu.gone);
    ints.eq("living after", out.live_col(V_OUT), &cpu.live_axis);
    let spent: Vec<u64> = (0..out.live_n)
        .map(|k| {
            u64::from(out.live_col(V_OUT + 1)[k]) | u64::from(out.live_col(V_OUT + 2)[k]) << 32
        })
        .collect();
    ints.eq("their times", &spent, &cpu.live_spent);
    let mut reals = Agreement::default();
    for (name, col, cpu_col) in [
        ("light at the open axes", S_LIGHT, &cpu.light),
        ("upkeep at the open axes", S_COST, &cpu.cost),
        ("lit presence at the open axes", S_SUB, &cpu.sub),
        ("incoming at the open axes", S_INCOMING, &cpu.incoming),
        ("bud vigour at the open axes", S_BUD, &cpu.bud),
    ] {
        reals.eq(name, &at_open(col), &cpu_open(cpu_col));
    }
    reals.eq("memory", out.col(S_MEMORY, n), &cpu.memory);
    reals.eq("kept", out.col(S_KEPT, n), &cpu.kept);
    reals.eq("size", out.col(S_SIZE, n), &cpu.size);
    (ints, reals)
}

/// Every word the sums leave, for two runs' comparison: the columns, and
/// the living after.
pub(super) fn words(out: &Out) -> Vec<u32> {
    let mut all = out.sums.clone();
    all.extend((V_OUT..V_OUT + 3).flat_map(|c| out.live_col(c).to_vec()));
    all
}

/// The blocks' rules broken, each named: every axis in a block of at most
/// `cap`, joined to its bearer's at one local depth below it or opening
/// one a level below its bearer's; each block's members by local depth
/// then index; each level's blocks in index order; each depth its
/// bearer's plus one.
pub(super) fn blocks(h: &Hierarchy, shape: &Shape, tree: &[u32], n: usize) -> Vec<String> {
    let parent = &tree[..n];
    let (depth, block, local, level) = (shape.col(0), shape.col(1), shape.col(2), shape.col(4));
    let mut broken = Vec::new();
    let mut size = vec![0u32; h.blocks as usize];
    for i in 0..n {
        let b = block[i] as usize;
        if b >= size.len() {
            broken.push(format!("axis {i}: block {b} of {}", size.len()));
            continue;
        }
        size[b] += 1;
        let p = parent[i] as usize;
        let ok = match (parent[i] == u32::MAX, local[i]) {
            (true, l) => l == 0 && depth[i] == 0 && level[b] == 0,
            (false, 0) => {
                block[p] as usize != b
                    && level[b] == level[block[p] as usize] + 1
                    && depth[i] == depth[p] + 1
            }
            (false, l) => block[p] as usize == b && l == local[p] + 1 && depth[i] == depth[p] + 1,
        };
        if !ok {
            broken.push(format!(
                "axis {i}: block {b}, local {}, depth {}",
                local[i], depth[i]
            ));
        }
    }
    if let Some(b) = size.iter().position(|&s| s == 0 || s > h.cap) {
        broken.push(format!("block {b} holds {}", size[b]));
    }
    let (ms, members) = (shape.list(2), shape.list(3));
    for b in 0..h.blocks as usize {
        let run = &members[ms[b] as usize..ms[b + 1] as usize];
        let ordered = run
            .windows(2)
            .all(|w| (local[w[0] as usize], w[0]) < (local[w[1] as usize], w[1]));
        if run.len() != size[b] as usize
            || !ordered
            || run.iter().any(|&m| block[m as usize] as usize != b)
        {
            broken.push(format!("block {b}'s members"));
        }
    }
    let (ls, lb) = (shape.list(4), shape.list(5));
    let mut listed = vec![0u32; h.blocks as usize];
    for l in 0..h.levels as usize {
        let run = &lb[ls[l] as usize..ls[l + 1] as usize];
        if !run.windows(2).all(|w| w[0] < w[1])
            || run.iter().any(|&b| level[b as usize] as usize != l)
        {
            broken.push(format!("level {l}'s blocks"));
        }
        run.iter().for_each(|&b| listed[b as usize] += 1);
    }
    if listed.iter().any(|&c| c != 1) {
        broken.push("a block not listed once".into());
    }
    broken
}
