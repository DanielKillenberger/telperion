//! fn-256 task 3 (DESIGN.md 12): a family's tree on the device, whole.
//! The CPU computes the species' tables once (`grow_capture_tables`);
//! the device grows the tree through the one cycle, settles and lays it,
//! plans its chains and draws it as the pipeline's nodes, and folds its
//! trunk's measures. Only counts and summaries come back.
use super::super::resident::fold::Dressed;
use super::build::{Kit, Tables};
use super::build_grow::grow_hinted;
use super::drawn::{dress, trunk, Rows, Trunk};
use super::settle_device::Kernels as Settle;
use super::settle_run::settle;
use super::{chains, drawn, present};
use crate::{Gpu, Result};
use telperion_core::pipeline::executor::{ConvertRow, Growth};
use telperion_space::{ChainTables, SettleTo};

/// Every kernel the tree is grown and drawn by.
pub(in crate::generation) struct Kernels {
    pub(super) build: Kit,
    pub(super) settle: Settle,
    pub(super) chains: chains::Kernels,
    pub(super) drawn: drawn::Kernels,
    pub(super) present: present::Kernels,
}

impl Kernels {
    /// Every module made side by side (`io::pair`).
    pub(in crate::generation) fn new(gpu: &Gpu) -> Self {
        use super::super::io::pair;
        let drawing = || pair(gpu, || chains::Kernels::new(gpu), || pair(gpu, || drawn::Kernels::new(gpu), || present::Kernels::new(gpu)));
        let (build, (settle, (chains, (drawn, present)))) = pair(gpu, || Kit::new(gpu), || pair(gpu, || Settle::new(gpu), drawing));
        Self { build, settle, chains, drawn, present }
    }
}

/// Each PA's row as `dress.wgsl` reads it.
pub(super) fn rows(rows: &[ConvertRow]) -> Vec<f32> {
    let bits = f32::from_bits;
    let row = |r: &ConvertRow| {
        [r.internode as f32, r.spread as f32, r.flowering as f32, r.leaves as f32, r.fan as f32, bits(r.sheds), bits(u32::from(r.share)), bits(u32::from(r.trunk))]
    };
    rows.iter().flat_map(row).collect()
}

/// The refusal of a tree its settle could not lay, by the settle's words.
pub(super) fn unlaid(error: [u32; 2]) -> crate::RenderError {
    super::super::io::error(format!("the settled tree could not be laid (fault {}, axis {})", error[0], error[1]))
}

/// The family's tree (`growth`) grown, settled and drawn on the device:
/// its drawn records and its trunk's measures.
pub(in crate::generation) async fn drawn(gpu: &Gpu, k: &Kernels, growth: &Growth) -> Result<(Dressed, Trunk)> {
    let (species, request) = (&growth.species, growth.request);
    let refused = |what: &str| super::super::io::error(format!("the species' {what}"));
    let trace = std::env::var("CYCLE_TRACE").is_ok();
    let clock = super::super::Clock::now();
    let tables = telperion_space::grow_capture_tables(species, request).map_err(|e| refused(&e.to_string()))?;
    let chain = ChainTables::cycle(species, request.age).map_err(|e| refused(&e.to_string()))?;
    let t = Tables::new(gpu, &tables, &chain)?;
    let at_tables = clock.elapsed_ms();
    let hint = super::build_arena::hint(species, request.age)?;
    let (b, ..) = grow_hinted(gpu, &k.build, &t, &tables, (hint, request.age)).await?;
    let b = b.settling(gpu, (&t, &tables)).await?;
    let at_grown = clock.elapsed_ms();
    let s = settle(gpu, (&k.build, &k.settle), (&b, &t), (&t.settle_pa, SettleTo::Laid)).await?;
    let at_settled = clock.elapsed_ms();
    drop(b);
    if s.error[0] != 0 {
        return Err(unlaid(s.error));
    }
    let planned = chains::plan(gpu, (&k.chains, &k.build.hier.scan), s.view()).await?;
    let rows = rows(&growth.rows);
    let at = Rows { rows: &rows, age: request.age, scale: growth.scale as f32, fraction: 1.0, leaf: None };
    let dressed = dress(gpu, &k.drawn, (s.view(), &planned), &at)?;
    let measured = trunk(gpu, &k.drawn, &dressed).await?;
    if trace {
        eprintln!("TRACE tree: tables {at_tables:.0} ms, grown {at_grown:.0}, settled {at_settled:.0}, drawn {:.0}", clock.elapsed_ms());
    }
    Ok((dressed, measured))
}
