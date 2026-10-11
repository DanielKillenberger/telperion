//! fn-256 task 2 (SETTLE-PLAN.md S1 to S6): the place passes recorded
//! over the node table, test-only: the scale a depth at a time from the
//! seed, each relay's attachment, the thickening and reaches a depth at
//! a time from the deepest. Each dispatch is its own stamped pass.
use super::build::{Build, Tables};
use super::hier_scan::{read_words, uniform, widest};
use super::settle_device::{self as d, Kernels};
use super::settle_run::Settled;
use crate::buffer::Owned;
use crate::{Gpu, Result};

/// One place's recording state: its uniform's words and the buffers it
/// binds.
struct Rec<'a> {
    gpu: &'a Gpu,
    pass: &'a super::super::io::Pass,
    form: &'a Owned,
    cfg: [u32; 16],
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

/// The settled tree scaled and thickened (S1 and S2).
pub(super) fn thicken(gpu: &Gpu, k: &Kernels, b: &Build, t: &Tables, s: &mut Settled) -> Result<()> {
    let (tb, l) = (s.table, s.layout);
    let at_q = d::Y_COLS * tb.sk;
    s.pw = super::settle_run::zeroed(gpu, "settle place", u64::from(at_q) + u64::from(tb.skp))?;
    let words = [
        widest(gpu).0, 0, 0, t.age, tb.sk, tb.skp, tb.at_np, tb.at_nu, l.sa, l.at_a, l.at_f, at_q, s.flags, 0,
        tb.kept, 0,
    ];
    let mut r = Rec { gpu, pass: &k.place, form: &b.layout.form, cfg: words, held: Vec::new() };
    let depths: Vec<(u32, u32)> = (s.kept_starts.windows(2))
        .map(|w| (w[0], w[1] - w[0]))
        .filter(|&(_, n)| n > 0)
        .collect();
    let mut e = gpu.encoder(&Default::default());
    for &at in &depths {
        r.run(&mut e, s, (d::SCALE, "settle scale"), at)?;
    }
    r.run(&mut e, s, (d::ATTACH, "settle thicken"), (0, tb.kept))?;
    for &at in depths.iter().rev() {
        r.run(&mut e, s, (d::THICK_E, "settle thicken"), at)?;
    }
    gpu.submit([e.finish()]);
    let fault = read_words(gpu, &s.atom, d::T_PLACE as u64, 1)?[0];
    if fault != 0 {
        return Err(super::super::io::error(format!("the settle's place: faults {fault:#x}")));
    }
    Ok(())
}
