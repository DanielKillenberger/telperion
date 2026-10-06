//! The device's expansion read back, for the tests only (fn-209 F1, task
//! 5): each root's kept axes in the order the CPU reference grows them,
//! each one's wave and index in its wave's laid buffer, and every chain's
//! geometry status. The executor itself never reads any of this.
use super::chains::{topology, AXW, PW};
use super::laid::{SA, SP};
use super::roots::Expanded;
use super::steps::{CW, RW};
use crate::Gpu;
use std::collections::HashMap;
use telperion_space::{ChainAxis, ChainBud};

pub(super) struct Read {
    pub roots: Vec<Vec<ChainAxis>>,
    pub laid: Vec<Vec<(usize, u32)>>,
    pub statuses: u32,
}

fn words(gpu: &Gpu, buffer: &wgpu::Buffer) -> Vec<u32> {
    let bytes = super::super::io::read(gpu, buffer, buffer.size()).unwrap();
    bytemuck::cast_slice(&bytes).to_vec()
}

/// What `ex` grew for `roots` roots.
pub(super) fn read(gpu: &Gpu, ex: &Expanded, roots: usize) -> Read {
    let mut out = Read {
        roots: vec![Vec::new(); roots],
        laid: vec![Vec::new(); roots],
        statuses: 0,
    };
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
        horizon: 0,
        woke: None,
        context: None,
    };
    for (w0, g) in ex.grown.iter().enumerate() {
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
            out.statuses |= scratch[(sb + na * SA + np * SP) as usize];
            let axes = &axes[a as usize * AXW..(a + na) as usize * AXW];
            let phyts = &phyts[p as usize * PW..(p + np) as usize * PW];
            let chain = topology(&stub, axes, phyts);
            let wi = wi as usize;
            let (root, bearer) = (meta[2 * wi] as usize, meta[2 * wi + 1]);
            let bearer = (w > 1).then(|| local[&(w - 1, bearer)]);
            let out_axes = places[wi * CW as usize];
            let kept = &mut out.roots[root];
            let mut at = vec![None; chain.len()];
            let mut own = 0;
            for (i, ax) in chain.iter().enumerate() {
                if !(i == 0 || ax.kind != 1) || !ax.kept {
                    continue;
                }
                let parent = ax.parent.map_or(bearer, |q| at[q]);
                at[i] = Some(kept.len());
                local.insert((w, out_axes + own), kept.len());
                out.laid[root].push((w, out_axes + own));
                own += 1;
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
