//! A captured hierarchy cycle (`grow_capture_hierarchy_at`) as the
//! device's columns (`hier_cols.wgsl`), and the CPU's outputs it is held
//! to. Every column is one word a row, `stride` rows apart.
#[cfg(test)]
use super::columns::Cap;

/// The tree's columns (`hier_cols.wgsl`'s `T_` words).
pub(super) const TREE: usize = 15;
#[cfg(test)]
/// A PA's row: six f32s, padded to eight.
pub(super) const ROW: usize = 8;

/// Rows a column holds for `n`: a multiple of 64, so each column binds at
/// a 256-byte offset.
pub(super) fn stride(n: u32) -> u32 {
    n.max(1).next_multiple_of(64)
}

#[cfg(test)]
/// The cycle's inputs.
pub(super) struct Hier {
    pub cycle: u32,
    pub n: u32,
    /// The axes laid at each cycle's start, from the first.
    pub laid: Vec<u32>,
    pub stride: u32,
    pub tree: Vec<u32>,
    pub rows: Vec<f32>,
    pub pas: u32,
    pub live_axis: Vec<u32>,
    pub live_spent: Vec<u64>,
}

#[cfg(test)]
/// What the CPU's passes left.
pub(super) struct Cpu {
    pub kid_start: Vec<u32>,
    pub kids: Vec<u32>,
    pub open: Vec<u32>,
    pub light: Vec<u32>,
    pub cost: Vec<u32>,
    pub living: Vec<u32>,
    pub sub: Vec<u32>,
    pub incoming: Vec<u32>,
    pub bud: Vec<u32>,
    pub memory: Vec<u32>,
    pub kept: Vec<u32>,
    pub size: Vec<u32>,
    /// Each axis whose apex the balance shed.
    pub gone: Vec<u32>,
    pub live_axis: Vec<u32>,
    pub live_spent: Vec<u64>,
}

#[cfg(test)]
fn split(v: &[u64]) -> (Vec<u32>, Vec<u32>) {
    (
        v.iter().map(|&x| x as u32).collect(),
        v.iter().map(|&x| (x >> 32) as u32).collect(),
    )
}

#[cfg(test)]
impl Hier {
    #[cfg(test)]
    pub(super) fn new(c: &Cap) -> Self {
        let [cycle, _age, n, _axes] = c.u32s("hier_counts")[..] else {
            panic!("cycle, age, laid axes, axes")
        };
        let stride = stride(n);
        let mut cols: Vec<Vec<u32>> = vec![
            c.u32s("hier_parent"),
            c.u32s("hier_node"),
            c.u32s("hier_pa"),
        ];
        for name in ["hier_lineage", "hier_in_wood", "hier_in_light"] {
            let (lo, hi) = split(&c.u64s(name));
            cols.extend([lo, hi]);
        }
        for name in ["base_scale", "running", "kept", "memory", "size"] {
            cols.push(c.u32s(&format!("hier_in_{name}")));
        }
        // No root's upkeep is frozen: the CPU's tree is whole.
        cols.push(vec![u32::MAX; n as usize]);
        assert_eq!(cols.len(), TREE, "the tree's columns");
        let mut tree = vec![0; TREE * stride as usize];
        for (k, col) in cols.iter().enumerate() {
            assert_eq!(col.len(), n as usize, "a word an axis");
            let at = k * stride as usize;
            tree[at..at + col.len()].copy_from_slice(col);
        }
        let six = c.f32s("hier_rows");
        let pas = (six.len() / 6) as u32;
        let rows = six
            .chunks(6)
            .flat_map(|r| r.iter().copied().chain([0.0; ROW - 6]))
            .collect();
        Self {
            cycle,
            n,
            laid: c.u32s("hier_laid"),
            stride,
            tree,
            rows,
            pas,
            live_axis: c.u32s("hier_in_live_axis"),
            live_spent: c.u64s("hier_in_live_spent"),
        }
    }

    pub fn live(&self) -> u32 {
        self.live_axis.len() as u32
    }
}

#[cfg(test)]
impl Cpu {
    #[cfg(test)]
    pub(super) fn new(c: &Cap) -> Self {
        let (before, after) = (c.u32s("hier_in_apex_end"), c.u32s("hier_out_apex_end"));
        let gone = before
            .iter()
            .zip(&after)
            .map(|(a, b)| u32::from(a != b))
            .collect();
        Self {
            kid_start: c.u32s("hier_kid_start"),
            kids: c.u32s("hier_kids"),
            open: c.u32s("hier_open"),
            light: c.u32s("hier_light"),
            cost: c.u32s("hier_cost"),
            living: c.u32s("hier_living"),
            sub: c.u32s("hier_sub"),
            incoming: c.u32s("hier_incoming"),
            bud: c.u32s("hier_bud"),
            memory: c.u32s("hier_out_memory"),
            kept: c.u32s("hier_out_kept"),
            size: c.u32s("hier_out_size"),
            gone,
            live_axis: c.u32s("hier_out_live_axis"),
            live_spent: c.u64s("hier_out_live_spent"),
        }
    }
}
