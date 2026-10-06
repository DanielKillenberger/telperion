//! Fine roots expanded on the GPU, wave by wave (fn-209 F1, tasks 2 and
//! 4): each root's chains grown on the device from its record, the buds a
//! wave's kept axes bear grown in the next, until none is left, and each
//! wave's chains scaled, girthed and laid on the device from their bearers
//! in the wave before (`laid.rs`). The overflow rule is the chain: an item
//! is split along its chains, each grown from its own bud's record, and a
//! chain that outgrows its region is grown again in a larger one.
use super::chains::{packed, Caps, Grew, Grower, Run, AXW};
use super::laid::{self, Chain, Resolver, Wave};
use crate::Gpu;
use telperion_space::{ChainAxis, ChainBud, ChainTables, GeometryTables, RootGeometry, Woke};

/// A chain's first region: most chains are a few axes, so a small region
/// and a large batch; one that outgrows it is grown again in a region
/// eight times larger, in batches an eighth the size.
const CAPS: Caps = Caps {
    axes: 32,
    phyts: 64,
    live: 8,
    sleepers: 16,
    woken: 8,
    units: 64,
};
/// A borne bud's vigour as made and its node's buds a whorl (`chain.wgsl`).
const VIG: usize = 36;
const WHORL: usize = 37;

/// A chain to grow: its root's index, its bud and the bud's vigour and
/// whorl, its bearer among the root's kept axes (none: the root's own
/// bud, borne above the cut) and in the wave before's laid buffer.
struct Item {
    root: usize,
    bud: ChainBud,
    made: [f32; 2],
    bearer: Option<usize>,
    laid: u32,
}

/// What expanding a tree's roots took: waves and chains, milliseconds
/// growing (upload to the device's finishing), reading back and laying,
/// and the chains grown again in larger regions.
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct Cost {
    pub waves: usize,
    pub chains: usize,
    pub grow: f64,
    pub read: f64,
    pub lay: f64,
    pub again: usize,
}

/// A species point's tables, packed and on the device.
pub(super) struct Tables<'a> {
    pub chain: &'a ChainTables,
    pub packed: super::chains::Packed,
    pub table: wgpu::Buffer,
    pub sags: bool,
}

impl<'a> Tables<'a> {
    pub(super) fn new(gpu: &Gpu, chain: &'a ChainTables, geo: &GeometryTables) -> Self {
        let (table, sags) = laid::table(geo);
        let usage = wgpu::BufferUsages::STORAGE;
        let bytes = table.len() as u64 * 4;
        let table = super::super::io::buffer(
            gpu,
            "laid table",
            bytes,
            usage,
            bytemuck::cast_slice(&table),
        )
        .unwrap();
        Self {
            chain,
            packed: packed(chain, geo),
            table,
            sags,
        }
    }
}

/// A bud a chain bears, from its axis's words, as its own chain takes it,
/// with its vigour as made and its node's buds a whorl.
fn bud(words: &[u32], i: usize, context: Option<i64>) -> (ChainBud, [f32; 2]) {
    let w = |f: usize| words[i * AXW + f];
    let wide = |f: usize| u64::from(w(f)) | u64::from(w(f + 1)) << 32;
    let kind = w(4);
    let birth = w(3);
    let bud = ChainBud {
        seed: false,
        lineage: wide(0),
        pa: w(2) as usize,
        birth,
        node: w(6) as usize,
        slot: (kind >> 8 & 0xff) as u8,
        root: wide(8),
        horizon: wide(10) as i64,
        woke: (kind & 0x80000 != 0).then(|| Woke {
            key: wide(8),
            pa: w(31) as usize,
            sleep: u64::from(w(19)),
            stage: w(20) as usize,
            spent: wide(12),
            cycle: birth + 1,
        }),
        context,
    };
    (bud, [f32::from_bits(w(VIG)), f32::from_bits(w(WHORL))])
}

/// Grows `items`, `batch` at a time, each outgrown chain again in a region
/// eight times larger: each chain as grown, the runs that grew them and
/// each chain's run and place in it.
fn grow(
    (gpu, grower): (&Gpu, &Grower),
    items: &[Item],
    batch: usize,
    tables: &Tables,
    cost: &mut Cost,
) -> (Vec<Grew>, Vec<Run>, Vec<(usize, usize)>) {
    let at = (tables.chain.span, tables.chain.age);
    let mut grown: Vec<Option<Grew>> = vec![None; items.len()];
    let mut place = vec![(0, 0); items.len()];
    let mut runs = Vec::new();
    let mut pending: Vec<usize> = (0..items.len()).collect();
    let (mut caps, mut size) = (CAPS, batch);
    while !pending.is_empty() {
        for part in pending.chunks(size) {
            let buds: Vec<ChainBud> = part.iter().map(|&c| items[c].bud).collect();
            let made: Vec<[f32; 2]> = part.iter().map(|&c| items[c].made).collect();
            let mut run = grower.run_kept(gpu, (&buds, &made), &tables.packed, at, caps);
            (cost.grow, cost.read) = (cost.grow + run.grow_ms, cost.read + run.read_ms);
            for (k, &c) in part.iter().enumerate() {
                grown[c] = run.grown[k].take();
                place[c] = (runs.len(), k);
            }
            runs.push(run);
        }
        pending.retain(|&c| grown[c].is_none());
        cost.again += pending.len();
        (caps, size) = (caps.times(8), (size / 8).max(64));
        assert!(caps.axes < 1 << 16, "a chain no region holds");
    }
    let grown = grown.into_iter().map(|g| g.expect("grown")).collect();
    (grown, runs, place)
}

/// The roots expanded: each root's kept axes, by its index, and each
/// one's wave and index in that wave's laid buffer; the laid buffers, the
/// first the roots' attachments; the statuses of every chain resolved.
pub(super) struct Expanded {
    pub roots: Vec<Vec<ChainAxis>>,
    pub laid: Vec<Vec<(usize, u32)>>,
    pub waves: Vec<Wave>,
    pub statuses: Vec<(wgpu::Buffer, Vec<u32>)>,
}

/// Expands every root of `buds`, taken in `order`, `batch` chains at a
/// time, its geometry laid on the device as each wave grows.
pub(super) fn expand(
    gpu: (&Gpu, &Grower, &Resolver),
    (buds, attached, order, batch): (&[ChainBud], &[RootGeometry], &[usize], usize),
    tables: &Tables,
) -> (Expanded, Cost) {
    let (device, grower, resolver) = gpu;
    let mut roots: Vec<Vec<ChainAxis>> = vec![Vec::new(); buds.len()];
    let mut laid: Vec<Vec<(usize, u32)>> = vec![Vec::new(); buds.len()];
    let mut cost = Cost::default();
    let (stubs, origins) = laid::stubs(attached);
    let usage = wgpu::BufferUsages::STORAGE;
    let bytes = origins.len().max(4) as u64 * 4;
    let origins = super::super::io::buffer(
        device,
        "laid origins",
        bytes,
        usage,
        bytemuck::cast_slice(&origins),
    )
    .unwrap();
    let n = buds.len() as u32;
    let mut waves = vec![Wave::new(device, (n, n), &stubs)];
    let mut statuses = Vec::new();
    let mut wave: Vec<Item> = order
        .iter()
        .map(|&k| Item {
            root: k,
            bud: buds[k],
            made: [attached[k].vigour as f32, attached[k].whorl as f32],
            bearer: None,
            laid: k as u32,
        })
        .collect();
    while !wave.is_empty() {
        cost.waves += 1;
        cost.chains += wave.len();
        let (grown, runs, place) = grow((device, grower), &wave, batch, tables, &mut cost);
        let mut next = Vec::new();
        let mut chains = Vec::new();
        let (mut out_axes, mut out_phyts) = (0u32, 0u32);
        let w = waves.len();
        for ((item, chain), &(run, c)) in wave.iter().zip(&grown).zip(&place) {
            let axes = &chain.axes;
            if !axes[0].kept {
                continue;
            }
            chains.push(Chain {
                run,
                chain: c,
                bearer: item.laid,
                out_axes,
                out_phyts,
                stub: w == 1,
                root: item.root as u32,
            });
            // The chain's kept own axes join its root's, each on its
            // parent's place there; the stub stands for the item's bearer.
            let mut at = vec![None; axes.len()];
            let mut own = vec![0u32; axes.len()];
            let kept = &mut roots[item.root];
            for (i, a) in axes.iter().enumerate() {
                if !(i == 0 || a.kind != 1) || !a.kept {
                    continue;
                }
                let parent = a.parent.map_or(item.bearer, |p| at[p]);
                at[i] = Some(kept.len());
                own[i] = out_axes;
                laid[item.root].push((w, out_axes));
                out_axes += 1;
                out_phyts += a.phytomers.len() as u32;
                kept.push(ChainAxis {
                    parent,
                    context: None,
                    ..a.clone()
                });
            }
            for (i, a) in axes.iter().enumerate().skip(1) {
                let bearer = a.parent.and_then(|p| at[p]);
                if a.kind == 1 && bearer.is_some() {
                    let (bud, made) = bud(&chain.words, i + 1, a.context);
                    next.push(Item {
                        root: item.root,
                        bud,
                        made,
                        bearer,
                        laid: own[a.parent.expect("borne")],
                    });
                }
            }
        }
        let out = Wave::new(device, (out_axes, out_phyts), &[]);
        let (ms, got) = resolver.resolve(
            device,
            (&runs, &chains),
            (&tables.table, &origins),
            (&waves[w - 1], &out),
            (tables.chain.age, tables.sags),
        );
        cost.lay += ms;
        statuses.extend(got);
        waves.push(out);
        wave = next;
    }
    let expanded = Expanded {
        roots,
        laid,
        waves,
        statuses,
    };
    (expanded, cost)
}
