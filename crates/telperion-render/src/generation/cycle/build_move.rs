//! A build moved into larger tables on the queue (DESIGN.md section 5):
//! only the tables whose rows grew are new, the state's and the
//! hierarchy's columns copied column by column to their new strides;
//! every other buffer is kept, and a scratch buffer the cycle writes
//! before it reads is made anew once the old is freed.
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
/// with `copy` submitted from it.
fn kept(gpu: &Gpu, old: Owned, (grew, label, words): (bool, &'static str, u64), copy: &dyn Fn(&mut wgpu::CommandEncoder, &Owned, &Owned)) -> Result<Owned> {
    if !grew {
        return Ok(old);
    }
    let new = zeroed(gpu, label, words)?;
    let mut e = gpu.encoder(&Default::default());
    copy(&mut e, &old, &new);
    gpu.submit([e.finish()]);
    Ok(new)
}

impl State {
    /// The state in tables of `n` rows, a table at a time.
    async fn moved(self, gpu: &Gpu, n: super::state::Caps) -> Result<State> {
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
        let phyts = (o.phyts, n.phyts);
        // A table group at a time, the device waited on after each so its
        // old tables are freed before the next group's are made: a move
        // holds one group twice at most (fn-270 R4).
        let grown = kept(gpu, self.grown, (o.phyts != n.phyts, "cycle state phytomers grown", fresh[3]), &table(GROWN, phyts))?;
        let laid = kept(gpu, self.laid, (o.phyts != n.phyts, "cycle state phytomers laid", fresh[4]), &table(LAID, phyts))?;
        super::super::io::complete(gpu).await?;
        let axes_t = kept(gpu, self.axes, (axes.0 != axes.1, "cycle state axes", fresh[0]), &table(AXES, axes))?;
        let pencils = kept(gpu, self.pencils, (axes.0 != axes.1, "cycle state pencils", fresh[1]), &table(PENCILS, axes))?;
        super::super::io::complete(gpu).await?;
        let units = kept(gpu, self.units, (o.units != n.units, "cycle state units", fresh[2]), &table(UNITS, (o.units, n.units)))?;
        let lists = kept(gpu, self.lists, (lists, "cycle state lists", fresh[5]), &copy_lists)?;
        super::super::io::complete(gpu).await?;
        Ok(State { caps: n, grown, laid, axes: axes_t, pencils, units, lists })
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
    /// This build sized `next` (it was `now`): its scratch whose size
    /// grows let go first and the device waited on, its state, hierarchy
    /// and retirement moved where their rows grew and the device waited on
    /// again, then that scratch made anew; every other buffer kept. A move
    /// holds no more than the old tables beside the new (fn-270 R4: the
    /// tables grow by observed counts, so moves are the rule).
    pub(super) async fn moved(mut self, gpu: &Gpu, (t, tables): (&Tables, &Capture), (now, next): (Sizes, Sizes)) -> Result<Build> {
        let (o, n) = (now.caps, next.caps);
        let (s, axes) = (n.axes, o.axes != n.axes);
        let live = axes || o.next != n.next;
        let lit = (now.points, now.tips, now.nodes) != (next.points, next.tips, next.nodes);
        let counted = o.next != n.next || o.sleepers != n.sleepers;
        let least = 64;
        if axes {
            self.tree = zeroed(gpu, "cycle tree", 4)?;
            self.layout = Layout::new(gpu, &t.form, least)?;
            self.flags = zeroed(gpu, "cycle flags", 4)?;
            self.scan_sums = zeroed(gpu, "cycle scan sums", 4)?;
        }
        if live {
            self.sums = Sums::alloc(gpu, least, least, t.pas)?;
            self.inputs = zeroed(gpu, "cycle inputs", 4)?;
        }
        if counted {
            self.counts = zeroed(gpu, "cycle counts", 4)?;
        }
        if lit {
            self.light = Light::new(gpu, &Cap(tables))?;
        }
        if o != n {
            self.compactor = super::compact::Compactor::new(gpu, Sizes::least().caps, least)?;
        }
        super::super::io::complete(gpu).await?;
        self.state = self.state.moved(gpu, n).await?;
        let mut e = gpu.encoder(&Default::default());
        self.hier = self.hier.moved(gpu, &mut e, s)?;
        self.retire = self.retire.moved(gpu, &mut e, s)?;
        gpu.submit([e.finish()]);
        super::super::io::complete(gpu).await?;
        if lit {
            let (want, nodes) = next.lit();
            self.light.ensure(gpu, want, nodes)?;
        }
        let new = |keep: Owned, grew: bool, label, words: u64| if grew { zeroed(gpu, label, words) } else { Ok(keep) };
        let counts = device::counts(n, t.zones).words;
        Ok(Build {
            sums: if live { Sums::alloc(gpu, s, n.next, t.pas)? } else { self.sums },
            tree: new(self.tree, axes, "cycle tree", u64::from(hier_columns::TREE as u32 * s))?,
            layout: if axes { Layout::new(gpu, &t.form, s)? } else { self.layout },
            inputs: new(self.inputs, live, "cycle inputs", 2 * u64::from(n.next) + 7 * u64::from(s))?,
            counts: new(self.counts, counted, "cycle counts", counts)?,
            flags: new(self.flags, axes, "cycle flags", 2 * u64::from(s))?,
            scan_sums: new(self.scan_sums, axes, "cycle scan sums", sums_words(s, 1))?,
            compactor: match o == n {
                true => self.compactor,
                false => super::compact::Compactor::new(gpu, n, s)?,
            },
            ..self
        })
    }
}

impl Build {
    /// The build as the settle reads it (`settle_run.rs`: its state,
    /// retirement, counts and the layout's form): every buffer only a
    /// cycle reads let go and the device waited on, so the settle's own
    /// are made in the room they held (fn-256 task 5: the Two Trees' settle
    /// passed the 4 GiB budget beside them).
    pub(super) async fn settling(mut self, gpu: &Gpu, (t, tables): (&Tables, &Capture)) -> Result<Build> {
        let least = super::build_grow::Sizes::least();
        let s = 64;
        self.hier = Hierarchy::new(gpu, s, self.hier.cap)?;
        self.sums = Sums::alloc(gpu, s, s, t.pas)?;
        self.light = Light::new(gpu, &Cap(tables))?;
        self.compactor = super::compact::Compactor::new(gpu, least.caps, s)?;
        for (b, label) in [(&mut self.tree, "cycle tree"), (&mut self.inputs, "cycle inputs"), (&mut self.counts, "cycle counts"), (&mut self.flags, "cycle flags"), (&mut self.scan_sums, "cycle scan sums")] {
            *b = zeroed(gpu, label, 4)?;
        }
        self.layout.lists = zeroed(gpu, "layout lists", 4)?;
        self.layout.counts = zeroed(gpu, "layout flags", 4)?;
        super::super::io::complete(gpu).await?;
        Ok(self)
    }
}
