//! A build moved into larger tables on the queue (DESIGN.md section 5):
//! only the tables whose rows grew are new, the state's and the
//! hierarchy's columns copied column by column to their new strides;
//! every other buffer is kept, and a scratch buffer the cycle writes
//! before it reads is made anew. No synchronisation: the copies follow
//! the cycles already submitted.
use super::build::{Build, Tables};
use super::build_grow::Sizes;
use super::build_light::Light;
use super::columns::Cap;
use super::hier_build::{Hierarchy, COUNTS, LISTS, SHAPE};
use super::hier_device::Sums;
use super::hier_scan::{sums_words, zeroed};
use super::lay_device::Layout;
use super::state::{State, AXES, GROWN, LAID, PENCILS, SLEEPERS, TALLY, UNITS, WOKEN};
use super::{device, hier_columns};
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_space::Capture;

/// `columns` columns of `from`, `a` rows apart from word `at_a`, into
/// `into`, `b` rows apart from word `at_b`, each `a` rows long.
fn columns(e: &mut wgpu::CommandEncoder, (from, into): (&Owned, &Owned), columns: u64, (a, b): (u64, u64), (at_a, at_b): (u64, u64)) {
    for k in 0..columns {
        e.copy_buffer_to_buffer(from, 4 * (at_a + k * a), into, 4 * (at_b + k * b), 4 * a);
    }
}

/// `old` kept where its rows did not grow, else a new buffer of `words`
/// with `copy` recorded from it.
fn kept(gpu: &Gpu, e: &mut wgpu::CommandEncoder, old: Owned, (grew, label, words): (bool, &'static str, u64), copy: &dyn Fn(&mut wgpu::CommandEncoder, &Owned, &Owned)) -> Result<Owned> {
    if !grew {
        return Ok(old);
    }
    let new = zeroed(gpu, label, words)?;
    copy(e, &old, &new);
    Ok(new)
}

impl State {
    /// The state in tables of `n` rows, copies recorded into `e`.
    fn moved(self, gpu: &Gpu, e: &mut wgpu::CommandEncoder, n: super::state::Caps) -> Result<State> {
        let o = self.caps;
        let w = u64::from;
        let table = |cols, (a, b): (u32, u32)| move |e: &mut wgpu::CommandEncoder, f: &Owned, t: &Owned| columns(e, (f, t), cols, (w(a), w(b)), (0, 0));
        let axes = (o.axes, n.axes);
        let lists = o.sleepers != n.sleepers || o.next != n.next || o.woken != n.woken;
        let copy_lists = |e: &mut wgpu::CommandEncoder, f: &Owned, t: &Owned| {
            e.copy_buffer_to_buffer(f, 0, t, 0, 4 * TALLY);
            let parts = [
                (SLEEPERS, o.sleepers, n.sleepers, o.sleep_at(), n.sleep_at()),
                (2, o.next, n.next, o.next_at(), n.next_at()),
                (WOKEN, o.woken, n.woken, o.woken_at(), n.woken_at()),
            ];
            for (cols, a, b, at_a, at_b) in parts {
                columns(e, (f, t), cols, (w(a), w(b)), (w(at_a), w(at_b)));
            }
        };
        let fresh = State::words(n);
        Ok(State {
            caps: n,
            axes: kept(gpu, e, self.axes, (axes.0 != axes.1, "cycle state axes", fresh[0]), &table(AXES, axes))?,
            pencils: kept(gpu, e, self.pencils, (axes.0 != axes.1, "cycle state pencils", fresh[1]), &table(PENCILS, axes))?,
            units: kept(gpu, e, self.units, (o.units != n.units, "cycle state units", fresh[2]), &table(UNITS, (o.units, n.units)))?,
            grown: kept(gpu, e, self.grown, (o.phyts != n.phyts, "cycle state phytomers grown", fresh[3]), &table(GROWN, (o.phyts, n.phyts)))?,
            laid: kept(gpu, e, self.laid, (o.phyts != n.phyts, "cycle state phytomers laid", fresh[4]), &table(LAID, (o.phyts, n.phyts)))?,
            lists: kept(gpu, e, self.lists, (lists, "cycle state lists", fresh[5]), &copy_lists)?,
        })
    }
}

impl Hierarchy {
    /// The hierarchy at `stride` rows a column, copies recorded into `e`.
    fn moved(self, gpu: &Gpu, e: &mut wgpu::CommandEncoder, stride: u32) -> Result<Hierarchy> {
        if stride == self.stride {
            return Ok(self);
        }
        let new = Hierarchy::new(gpu, stride, self.cap)?;
        let (a, b) = (u64::from(self.stride), u64::from(stride));
        columns(e, (&self.shape, &new.shape), SHAPE, (a, b), (0, 0));
        columns(e, (&self.counts, &new.counts), COUNTS, (a, b), (0, 0));
        for k in 0..2 {
            columns(e, (&self.lists[k], &new.lists[k]), LISTS, (a + 64, b + 64), (0, 0));
        }
        e.copy_buffer_to_buffer(&self.tally, 0, &new.tally, 0, self.tally.size());
        Ok(new)
    }
}

impl Build {
    /// This build sized `next` (it was `now`): its state and hierarchy
    /// moved where their rows grew, its scratch made anew where its size
    /// did, every other buffer kept.
    pub(super) fn moved(self, gpu: &Gpu, (t, tables): (&Tables, &Capture), (now, next): (Sizes, Sizes)) -> Result<Build> {
        let mut e = gpu.encoder(&Default::default());
        let (o, n) = (now.caps, next.caps);
        let (s, axes) = (n.axes, o.axes != n.axes);
        let state = self.state.moved(gpu, &mut e, n)?;
        let hier = self.hier.moved(gpu, &mut e, s)?;
        let retire = self.retire.moved(gpu, &mut e, s)?;
        gpu.submit([e.finish()]);
        let live = axes || o.next != n.next;
        let lit = (now.points, now.tips, now.nodes) != (next.points, next.tips, next.nodes);
        let light = match lit {
            true => {
                let mut l = Light::new(gpu, &Cap(tables))?;
                let (want, nodes) = next.lit();
                l.ensure(gpu, want, nodes)?;
                l
            }
            false => self.light,
        };
        let new = |keep: Owned, grew: bool, label, words: u64| if grew { zeroed(gpu, label, words) } else { Ok(keep) };
        let counts = 2 * u64::from(device::b_at(n.next.max(n.sleepers)));
        Ok(Build {
            sums: if live { Sums::alloc(gpu, s, n.next, t.pas)? } else { self.sums },
            tree: new(self.tree, axes, "cycle tree", u64::from(hier_columns::TREE as u32 * s))?,
            layout: if axes { Layout::new(gpu, &t.form, s)? } else { self.layout },
            inputs: new(self.inputs, live, "cycle inputs", 2 * u64::from(n.next) + 6 * u64::from(s))?,
            counts: new(self.counts, o.next != n.next || o.sleepers != n.sleepers, "cycle counts", counts)?,
            flags: new(self.flags, axes, "cycle flags", 2 * u64::from(s))?,
            scan_sums: new(self.scan_sums, axes, "cycle scan sums", sums_words(s))?,
            spare: self.spare,
            scan_tally: self.scan_tally,
            state,
            hier,
            retire,
            light,
            held: self.held,
            laid: self.laid,
            grown: self.grown,
            census: self.census,
        })
    }
}
