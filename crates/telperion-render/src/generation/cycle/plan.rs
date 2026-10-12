//! A cycle recorded once and replayed every cycle (DESIGN.md sections 5
//! and 7a): each dispatch's uniform words computed on the device at the
//! cycle's points (`vm.rs`) from its counts (`census.rs`), so the host
//! records a cycle without reading anything back. Every dispatch is
//! direct: where its size is a count only the device knows, the host
//! gives it the most workgroups the count's bounds allow, each
//! invocation exits past the true count, and a program flags a true size
//! past what was given. A dispatch's uniform is a slot of one buffer; a
//! point computes its slots' dynamic words into a storage copy, then
//! copies them into the uniform.
use super::super::io;
use super::census;
use super::vm::{program, Dst, E};
use crate::buffer::Owned;
use crate::{Gpu, Result};

/// The slots and bounded dispatches a plan holds.
// A units round has a slot a range (its round word): the tests' small
// ranges, 64 apexes each, take some thousands.
const SLOTS: u32 = 16384;
const SIZED: u32 = 8192;
const SLOT_WORDS: u32 = 128;
/// A slot's bytes, a multiple of the uniform offset alignment.
pub(super) const SLOT_BYTES: u64 = SLOT_WORDS as u64 * 4;
/// A dispatch row's most workgroups; a slot's `row` word is this times 64
/// whatever the count (a dispatch of one row ignores it).
pub(super) const MAXX: u32 = 65535;

/// Which module a dispatch runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PassId {
    Units,
    Layout,
    HierBuild,
    HierSums,
    HierScan,
    Light,
    Glue,
    Retire,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Size {
    Direct(u32),
    /// The plan's bounded size `k` (`Plan::sized`), given each cycle.
    Bounded(u32),
}

/// A step, and the conditions under which a cycle records it.
pub(super) struct Gated {
    pub gate: Vec<super::gate::Cond>,
    pub step: Step,
}

pub(super) enum Step {
    /// A dispatch, and the phase its time is counted in.
    Dispatch { pass: PassId, entry: usize, bind: wgpu::BindGroup, size: Size, phase: &'static str },
    Copy { from: wgpu::Buffer, at: u64, to: wgpu::Buffer, to_at: u64, bytes: u64 },
    Clear { buffer: wgpu::Buffer, at: u64, bytes: Option<u64> },
    Point(usize),
}

/// A point: its programs (by their place in the table, the flagging
/// `early` first) and its slots.
struct Point {
    first: u32,
    n: u32,
    early: u32,
    slots: (u32, u32),
    bind: Option<wgpu::BindGroup>,
    _cfg: Option<Owned>,
}

/// The plan's buffers and points, as its builder left them.
pub(super) struct Plan {
    pub census: Owned,
    pub cfgs: Owned,
    pub uniforms: Owned,
    /// The workgroups each bounded size was given this cycle, and its
    /// expression over the census.
    pub given: Owned,
    pub sized: std::rc::Rc<Vec<E>>,
    code: Owned,
    points: Vec<Point>,
}

fn storage(gpu: &Gpu, label: &'static str, words: u64, usage: wgpu::BufferUsages, init: &[u32]) -> Result<Owned> {
    io::buffer(gpu, label, words.max(4) * 4, usage | wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC, bytemuck::cast_slice(init))
}

/// Records slots, programs, points and steps.
pub(super) struct Builder<'g> {
    pub gpu: &'g Gpu,
    pub plan: Plan,
    image: Vec<u32>,
    progs: Vec<Vec<u32>>,
    /// The open point's flagging programs and the rest.
    early: Vec<Vec<u32>>,
    late: Vec<Vec<u32>>,
    sized: Vec<E>,
    steps: Vec<Gated>,
    /// The conditions the steps recorded next are under.
    gates: Vec<super::gate::Cond>,
    /// The phase the dispatches recorded next are counted in.
    pub phase: &'static str,
}

impl<'g> Builder<'g> {
    pub(super) fn new(gpu: &'g Gpu) -> Result<Self> {
        let none = wgpu::BufferUsages::empty();
        let plan = Plan {
            census: storage(gpu, "census", census::WORDS as u64, none, &[])?,
            cfgs: storage(gpu, "plan cfgs", u64::from(SLOTS * SLOT_WORDS), none, &[])?,
            uniforms: io::buffer(gpu, "plan uniforms", u64::from(SLOTS * SLOT_WORDS) * 4, wgpu::BufferUsages::UNIFORM, &[])?,
            given: storage(gpu, "plan given", u64::from(SIZED), none, &[])?,
            sized: Default::default(),
            code: storage(gpu, "plan code", 4, none, &[])?,
            points: Vec::new(),
        };
        let (early, late) = (Vec::new(), Vec::new());
        Ok(Self { gpu, plan, image: Vec::new(), progs: Vec::new(), early, late, sized: Vec::new(), steps: Vec::new(), gates: Vec::new(), phase: "start" })
    }

    /// A new point: the programs and slots that follow run and fill there.
    pub(super) fn point(&mut self) {
        let slot = (self.image.len() as u32) / SLOT_WORDS;
        if let Some(p) = self.plan.points.last_mut() {
            p.early = self.early.len() as u32;
            p.n = (self.early.len() + self.late.len()) as u32;
            self.progs.append(&mut self.early);
            self.progs.append(&mut self.late);
            p.slots.1 = slot;
        }
        let at = self.plan.points.len();
        self.plan.points.push(Point { first: self.progs.len() as u32, n: 0, early: 0, slots: (slot, slot), bind: None, _cfg: None });
        self.steps.push(Gated { gate: Vec::new(), step: Step::Point(at) });
    }

    /// A slot holding `words`, its other words none.
    pub(super) fn slot(&mut self, words: &[u32]) -> u32 {
        let at = self.image.len() as u32 / SLOT_WORDS;
        assert!(at < SLOTS && words.len() <= SLOT_WORDS as usize, "a slot the plan holds");
        self.image.extend(words);
        self.image.resize(self.image.len() + SLOT_WORDS as usize - words.len(), 0);
        at
    }

    /// Word `w` of `slot` computed on the device at this point.
    pub(super) fn set(&mut self, slot: u32, w: usize, e: E) {
        self.late.push(program(Dst::Cfg(slot * SLOT_WORDS + w as u32), &e));
    }

    /// A census word computed on the device at this point.
    pub(super) fn count(&mut self, w: usize, e: E) {
        self.late.push(program(Dst::Census(w), &e));
    }

    /// A flag raised where `e` is not none, kept raised; the point's
    /// uniform words none once it is.
    pub(super) fn flag(&mut self, f: census::Flag, e: E) {
        let w = f.word();
        self.early.push(program(Dst::Census(w), &E::At(w).max(e)));
    }

    /// A dispatch of `groups` workgroups, a count only the device knows:
    /// the host gives it the most its bounds allow, and the device flags
    /// a cycle where that was too few.
    pub(super) fn groups(&mut self, groups: E) -> Size {
        let k = self.sized.len() as u32;
        assert!(k < SIZED, "a size the plan holds");
        self.early.push(program(Dst::Raise(census::Flag::Given.word()), &groups.clone().gt(E::Given(k))));
        self.sized.push(groups);
        Size::Bounded(k)
    }

    /// A bind of `pass` with `slot` as its uniform and `rest` after it.
    pub(super) fn bind(&self, pass: &io::Pass, slot: u32, rest: &[&wgpu::Buffer]) -> wgpu::BindGroup {
        let mut entries = vec![wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &self.plan.uniforms,
                offset: u64::from(slot * SLOT_WORDS) * 4,
                size: std::num::NonZeroU64::new(u64::from(SLOT_WORDS) * 4),
            }),
        }];
        entries.extend(rest.iter().enumerate().map(|(i, b)| wgpu::BindGroupEntry { binding: i as u32 + 1, resource: b.as_entire_binding() }));
        self.gpu.device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("plan"), layout: &pass.layout, entries: &entries })
    }

    pub(super) fn step(&mut self, step: Step) {
        self.steps.push(Gated { gate: self.gates.clone(), step });
    }

    /// The steps recorded until `leave` under `cond` as well.
    pub(super) fn enter(&mut self, cond: super::gate::Cond) {
        self.gates.push(cond);
    }

    pub(super) fn leave(&mut self) {
        self.gates.pop();
    }

    pub(super) fn dispatch(&mut self, (pass, entry): (PassId, usize), bind: wgpu::BindGroup, size: Size) {
        self.step(Step::Dispatch { pass, entry, bind, size, phase: self.phase });
    }

    /// `bytes` of `from` at `at` into the census at word `w`.
    pub(super) fn tally(&mut self, from: &wgpu::Buffer, at: u64, w: usize, bytes: u64) {
        let to = (*self.plan.census).clone();
        self.step(Step::Copy { from: from.clone(), at, to, to_at: 4 * w as u64, bytes });
    }

    /// The steps recorded since the last take.
    pub(super) fn take(&mut self) -> Vec<Gated> {
        std::mem::take(&mut self.steps)
    }

    /// The plan, its programs uploaded and its points' uniforms made.
    pub(super) fn finish(mut self, vm: &super::vm::Kernels) -> Result<Plan> {
        self.point();
        self.plan.points.pop();
        self.steps.pop();
        let table_at = 0usize;
        let mut code: Vec<u32> = vec![0; self.progs.len()];
        for (k, p) in self.progs.iter().enumerate() {
            code[table_at + k] = code.len() as u32;
            code.extend(p);
        }
        let gpu = self.gpu;
        self.plan.sized = std::rc::Rc::new(std::mem::take(&mut self.sized));
        self.plan.code = storage(gpu, "plan code", code.len() as u64, wgpu::BufferUsages::empty(), &code)?;
        gpu.queue.write_buffer(&self.plan.cfgs, 0, bytemuck::cast_slice(&self.image));
        gpu.queue.write_buffer(&self.plan.uniforms, 0, bytemuck::cast_slice(&self.image));
        for p in &mut self.plan.points {
            let words = [p.first, p.n, census::FLAGS as u32, census::FLAG_NAMES.len() as u32, p.early, 0, 0, 0];
            let cfg = io::buffer(gpu, "plan point", 32, wgpu::BufferUsages::UNIFORM, bytemuck::cast_slice(&words))?;
            let all = [&*cfg, &self.plan.code, &self.plan.census, &self.plan.cfgs, &self.plan.given];
            p.bind = Some(vm.pass.bind(gpu, &all));
            p._cfg = Some(cfg);
        }
        Ok(self.plan)
    }
}

impl Plan {
    /// Point `k` run: its programs, then its slots into the uniforms;
    /// whether it dispatched.
    pub(super) fn run_point(&self, vm: &super::vm::Kernels, e: &mut wgpu::CommandEncoder, k: usize) -> bool {
        let p = &self.points[k];
        if p.n > 0 {
            vm.pass.run(e, p.bind.as_ref().expect("finished"), 0, 1);
        }
        if p.slots.1 > p.slots.0 {
            let at = u64::from(p.slots.0 * SLOT_WORDS) * 4;
            let bytes = u64::from((p.slots.1 - p.slots.0) * SLOT_WORDS) * 4;
            e.copy_buffer_to_buffer(&self.cfgs, at, &self.uniforms, at, bytes);
        }
        p.n > 0
    }
}
