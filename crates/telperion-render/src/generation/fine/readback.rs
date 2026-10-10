//! The device's expansion read back, for the tests only (fn-209 F1, task
//! 5): each root's kept axes in the order the CPU reference grows them,
//! each one's wave and index in its wave's laid buffer, and every chain's
//! geometry status. The executor itself never reads any of this.
use super::chains::{topology, AXW, PW};
use super::laid::{SA, SP};
use super::roots::Grown;
use super::steps::{CW, RW};
use crate::Gpu;
use std::collections::HashMap;
use telperion_space::{ChainAxis, ChainBud, Presences};

pub(super) struct Read {
    pub roots: Vec<Vec<ChainAxis>>,
    /// Each kept axis's Q32 presences (`assign.wgsl`), as `roots`.
    pub presences: Vec<Vec<Presences>>,
    pub laid: Vec<Vec<(usize, u32)>>,
    pub statuses: u32,
    /// Each root's chains' statuses.
    pub root_statuses: Vec<u32>,
}

impl Read {
    /// Nothing read yet, for `roots` roots.
    pub(super) fn empty(roots: usize) -> Self {
        Self {
            roots: vec![Vec::new(); roots],
            presences: vec![Vec::new(); roots],
            laid: vec![Vec::new(); roots],
            statuses: 0,
            root_statuses: vec![0; roots],
        }
    }

    /// A group's read (task 9), its roots at `roots` by their places in
    /// it, its laid buffers numbered from `offset`.
    pub(super) fn take(&mut self, group: Read, roots: &[usize], offset: usize) {
        self.statuses |= group.statuses;
        let laid = group.laid.into_iter();
        let parts = group.roots.into_iter().zip(group.presences).zip(laid);
        for ((&r, ((axes, presences), laid)), status) in
            roots.iter().zip(parts).zip(group.root_statuses)
        {
            self.roots[r] = axes;
            self.presences[r] = presences;
            self.laid[r] = laid.into_iter().map(|(w, i)| (w + offset, i)).collect();
            self.root_statuses[r] = status;
        }
    }
}

fn words(gpu: &Gpu, buffer: &wgpu::Buffer) -> Vec<u32> {
    let bytes = super::super::io::read(gpu, buffer, buffer.size()).unwrap();
    bytemuck::cast_slice(&bytes).to_vec()
}

/// What `grown` holds for `roots` roots, each wave's laid places counting
/// every own axis, kept or not, as realized growth places them.
pub(super) fn read(gpu: &Gpu, grown: &[Grown], roots: usize) -> Read {
    let mut out = Read::empty(roots);
    // Each laid axis's place among its root's kept axes.
    let mut local: HashMap<(usize, u32), usize> = HashMap::new();
    let stub = ChainBud {
        seed: false,
        lineage: 0,
        pa: 0,
        birth: 0,
        node: 0,
        slot: 0,
        root: 0,
        horizon: 0.0,
        woke: None,
        context: None,
    };
    for (w0, g) in grown.iter().enumerate() {
        let w = w0 + 1;
        let meta = words(gpu, &g.meta);
        let places = words(gpu, &g.places);
        let runs: Vec<[Vec<u32>; 4]> = g
            .runs
            .iter()
            .map(|r| [&r.runoff, &r.axes, &r.phyts, &r.scratch].map(|b| words(gpu, b)))
            .collect();
        // The wave's chains in its order.
        let mut chains: Vec<(u32, usize, usize)> = Vec::new();
        for (k, run) in g.runs.iter().enumerate() {
            for c in 0..run.n as usize {
                let rec = &runs[k][0][c * RW as usize..][..RW as usize];
                if rec[6] != 0 {
                    chains.push((rec[7], k, c));
                }
            }
        }
        chains.sort_unstable();
        for (wi, k, c) in chains {
            let [runoff, axes, phyts, scratch] = &runs[k];
            let rec = &runoff[c * RW as usize..][..RW as usize];
            let (a, p, sb, na, np) = (rec[0], rec[1], rec[2], rec[8], rec[9]);
            let status = scratch[(sb + na * SA + np * SP) as usize];
            out.statuses |= status;
            let axes = &axes[a as usize * AXW..(a + na) as usize * AXW];
            let phyts = &phyts[p as usize * PW..(p + np) as usize * PW];
            let chain = topology(&stub, axes, phyts);
            let wi = wi as usize;
            let (root, bearer) = (meta[2 * wi] as usize, meta[2 * wi + 1]);
            out.root_statuses[root] |= status;
            // A chain whose bearer was shed is shed with it.
            let bearer = match (w > 1).then(|| local.get(&(w - 1, bearer))) {
                None => None,
                Some(Some(&b)) => Some(b),
                Some(None) => continue,
            };
            let out_axes = places[wi * CW as usize];
            let kept = &mut out.roots[root];
            let mut at = vec![None; chain.len()];
            let mut own = 0;
            for (i, ax) in chain.iter().enumerate() {
                let own_axis = i == 0 || ax.kind != 1;
                if own_axis && !ax.kept {
                    own += 1;
                }
                if !own_axis || !ax.kept {
                    continue;
                }
                let parent = ax.parent.map_or(bearer, |q| at[q]);
                at[i] = Some(kept.len());
                local.insert((w, out_axes + own), kept.len());
                out.laid[root].push((w, out_axes + own));
                own += 1;
                out.presences[root].push(presences(axes, phyts, i + 1));
                kept.push(ChainAxis {
                    parent,
                    context: None,
                    ..ax.clone()
                });
            }
        }
    }
    out
}

/// Region axis `i`'s presences, its units' summed.
fn presences(axes: &[u32], phyts: &[u32], i: usize) -> Presences {
    let field = |f: usize| f32::from_bits(axes[i * AXW + f]);
    let scales = phyts
        .as_chunks::<PW>()
        .0
        .iter()
        .filter(|p| p[3] as usize == i)
        .map(|p| f32::from_bits(p[9]))
        .collect();
    Presences {
        vigour: field(48),
        units: vec![field(54)],
        scales,
        alive: field(52),
        sleep: axes[i * AXW + 19] as f32 / (1 << 20) as f32,
    }
}
