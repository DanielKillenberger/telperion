//! fn-256 task 2 (SETTLE-PLAN.md S1 to S6): the place passes recorded
//! over the node table, test-only: the scale a depth at a time from the
//! seed, each relay's attachment, the thickening and reaches a depth at
//! a time from the deepest; then the lay from the seed, and where any PA
//! sags, the levers from the deepest and the lay again under them; then
//! whether any phytomer has size. Each dispatch is its own stamped pass.
use super::build::{Build, Tables};
use super::hier_scan::{uniform, widest, words as read};
use super::settle_device::{self as d, Kernels};
use super::settle_run::Settled;
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_space::SettleTo;

/// One place's recording state: its uniform's words and the buffers it
/// binds.
struct Rec<'a> {
    gpu: &'a Gpu,
    steps: (u32, u32),
    pass: &'a super::super::io::Pass,
    form: &'a Owned,
    cfg: [u32; 20],
    held: Vec<Owned>,
}

impl Rec<'_> {
    fn run(&mut self, e: &mut wgpu::CommandEncoder, s: &mut Settled, (entry, phase): (usize, &'static str), (base, count): (u32, u32)) -> Result<()> {
        let mut cfg = self.cfg;
        (cfg[1], cfg[2]) = (base, count);
        let buf = uniform(self.gpu, &cfg)?;
        let bind = self.pass.bind(self.gpu, &[&buf, self.form, &s.work, &s.atom, &s.nt, &s.pw]);
        self.pass.run(e, &bind, entry, count.div_ceil(64).max(1));
        self.held.push(buf);
        s.stamped.push(phase);
        Ok(())
    }
}

/// The settled tree scaled and thickened (S1 and S2): the place's
/// uniform words for the lay.
pub(super) async fn thicken(gpu: &Gpu, k: &Kernels, b: &Build, t: &Tables, s: &mut Settled) -> Result<[u32; 20]> {
    let (tb, l) = (s.table, s.layout);
    let at_q = d::Y_COLS * tb.sk;
    s.pw = super::settle_run::zeroed(gpu, "settle place", u64::from(at_q) + u64::from(d::Q_COLS * tb.skp))?;
    let words = [
        widest(gpu).0, 0, 0, t.age, tb.sk, tb.skp, tb.at_np, tb.at_nu, l.sa, l.at_a, l.at_f, at_q, s.flags, 0,
        tb.kept, 0, 0, 0, 0, 0,
    ];
    let mut r = Rec { gpu, steps: (k.steps, k.walk), pass: &k.place, form: &b.layout.form, cfg: words, held: Vec::new() };
    let depths = depths(s);
    let mut e = gpu.encoder(&Default::default());
    for &at in &depths {
        r.run(&mut e, s, (d::SCALE, "settle scale"), at)?;
    }
    r.run(&mut e, s, (d::ATTACH, "settle thicken"), (0, tb.kept))?;
    for &at in depths.iter().rev() {
        r.run(&mut e, s, (d::THICK_E, "settle thicken"), at)?;
    }
    gpu.submit([e.finish()]);
    faults(gpu, s).await?;
    Ok(r.cfg)
}

/// Each kept depth's first final index and its axes.
fn depths(s: &Settled) -> Vec<(u32, u32)> {
    (s.kept_starts.windows(2))
        .map(|w| (w[0], w[1] - w[0]))
        .filter(|&(_, n)| n > 0)
        .collect()
}

/// A fault the place met, by its bits (`settle_state.wgsl`'s `FAULT_`).
async fn faults(gpu: &Gpu, s: &Settled) -> Result<()> {
    let fault = read(gpu, &s.atom, d::T_PLACE as u64, 1).await?[0];
    if fault != 0 {
        return Err(super::super::io::error(format!("the settle's place: faults {fault:#x}")));
    }
    Ok(())
}

/// The scaled, thickened tree laid; where `sags`, laid again under its
/// levers; then its first error, or a collapse where no phytomer has
/// size (`s.error`). Laid only once, its levers gathered, where `to` is
/// `Placed`.
pub(super) async fn lay(gpu: &Gpu, k: &Kernels, b: &Build, (cfg, sags, to): ([u32; 20], bool, SettleTo), s: &mut Settled) -> Result<()> {
    let mut r = Rec { gpu, steps: (k.steps, k.walk), pass: &k.place, form: &b.layout.form, cfg, held: Vec::new() };
    let depths = depths(s);
    gpu.queue.write_buffer(&s.atom, 4 * d::T_ERR as u64, bytemuck::bytes_of(&u32::MAX));
    let mut e = gpu.encoder(&Default::default());
    walk(&mut r, &mut e, s, "settle place")?;
    gpu.submit([e.finish()]);
    faults(gpu, s).await?;
    let placed = to == SettleTo::Placed;
    if first_error(gpu, s).await? || !(sags || placed) {
        return sized(gpu, &mut r, s).await;
    }
    let mut e = gpu.encoder(&Default::default());
    for &at in depths.iter().rev() {
        r.run(&mut e, s, (d::LEVER, "settle levers"), at)?;
    }
    if placed {
        gpu.submit([e.finish()]);
        return Ok(());
    }
    r.cfg[15] = 1;
    walk(&mut r, &mut e, s, "settle sag place")?;
    gpu.submit([e.finish()]);
    faults(gpu, s).await?;
    if first_error(gpu, s).await? {
        return Ok(());
    }
    sized(gpu, &mut r, s).await
}

/// The tree laid a depth at a time: each depth framed, then walked as
/// far as its longest axis reaches, each dispatch as many phytomers as
/// keep its axes' steps within `r.steps.0`, from 1 to `r.steps.1`, and
/// a depth wider than the steps walked a run of its axes at a time.
fn walk(r: &mut Rec, e: &mut wgpu::CommandEncoder, s: &mut Settled, phase: &'static str) -> Result<()> {
    let (starts, long) = (s.kept_starts.clone(), s.kept_long.clone());
    for (k, w) in starts.windows(2).enumerate() {
        let at = (w[0], w[1] - w[0]);
        if at.1 == 0 {
            continue;
        }
        r.run(e, s, (d::FRAME_E, phase), at)?;
        let walk = (r.steps.0 / at.1).clamp(1, r.steps.1);
        let run = r.steps.0 / walk;
        for off in (0..at.1).step_by(run as usize) {
            let axes = (at.0 + off, run.min(at.1 - off));
            for c in 0..long[k].div_ceil(walk) {
                (r.cfg[16], r.cfg[17]) = (c * walk, walk);
                r.run(e, s, (d::WALK_E, phase), axes)?;
            }
        }
    }
    Ok(())
}

/// The lay's first error into `s.error`, if it met one.
async fn first_error(gpu: &Gpu, s: &mut Settled) -> Result<bool> {
    let at = read(gpu, &s.atom, d::T_ERR as u64, 1).await?[0];
    if at != u32::MAX {
        s.error = [at & 3, at >> 2];
    }
    Ok(at != u32::MAX)
}

/// Whether any phytomer has size: a collapse where none has.
async fn sized(gpu: &Gpu, r: &mut Rec<'_>, s: &mut Settled) -> Result<()> {
    if s.error[0] != 0 {
        return Ok(());
    }
    let mut e = gpu.encoder(&Default::default());
    r.run(&mut e, s, (d::SIZED, "settle place"), (0, s.table.phyts))?;
    gpu.submit([e.finish()]);
    if read(gpu, &s.atom, d::T_SCALED as u64, 1).await?[0] == 0 {
        s.error = [1, u32::MAX];
    }
    Ok(())
}
