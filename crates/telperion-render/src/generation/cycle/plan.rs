//! A cycle recorded once and replayed every cycle (DESIGN.md section 5):
//! each dispatch's uniform words and workgroups computed on the device
//! at the cycle's points (`vm.rs`) from its counts (`census.rs`), so the
//! host records a cycle without reading anything back. A dispatch's
//! uniform is a 256-byte slot of one buffer; a point computes its slots'
//! dynamic words into a storage copy, then copies them into the uniform.
use super::super::io;
use super::census;
use super::vm::{program, Dst, E};
use crate::buffer::Owned;
use crate::{Gpu, Result};

/// The slots and indirect entries a plan holds.
const SLOTS: u32 = 4096;
const ARGS: u32 = 8192;
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
    UnitScan,
    Layout,
    LayScan,
    HierBuild,
    HierSums,
    HierScan,
    Light,
    Glue,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Size {
    Direct(u32),
    /// Byte offset of its (x, y, z) in the plan's arguments.
    Indirect(u64),
}

pub(super) enum Step {
    Dispatch { pass: PassId, entry: usize, bind: wgpu::BindGroup, size: Size },
    Copy { from: wgpu::Buffer, at: u64, to: wgpu::Buffer, to_at: u64, bytes: u64 },
    Clear { buffer: wgpu::Buffer, at: u64, bytes: Option<u64> },
    Point(usize),
}

/// A point: its programs (by their place in the table) and its slots.
struct Point {
    first: u32,
    n: u32,
    slots: (u32, u32),
    bind: Option<wgpu::BindGroup>,
    _cfg: Option<Owned>,
}

/// The plan's buffers and points, as its builder left them.
pub(super) struct Plan {
    pub census: Owned,
    pub cfgs: Owned,
    pub uniforms: Owned,
    pub args: Owned,
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
    entries: u32,
    steps: Vec<Step>,
}

impl<'g> Builder<'g> {
    pub(super) fn new(gpu: &'g Gpu) -> Result<Self> {
        let none = wgpu::BufferUsages::empty();
        let mut args = vec![0u32; 4 * ARGS as usize];
        args.chunks_mut(4).for_each(|a| a[2] = 1);
        let plan = Plan {
            census: storage(gpu, "census", census::WORDS as u64, none, &[])?,
            cfgs: storage(gpu, "plan cfgs", u64::from(SLOTS * SLOT_WORDS), none, &[])?,
            uniforms: io::buffer(gpu, "plan uniforms", u64::from(SLOTS * SLOT_WORDS) * 4, wgpu::BufferUsages::UNIFORM, &[])?,
            args: storage(gpu, "plan args", u64::from(4 * ARGS), wgpu::BufferUsages::INDIRECT, &args)?,
            code: storage(gpu, "plan code", 4, none, &[])?,
            points: Vec::new(),
        };
        Ok(Self { gpu, plan, image: Vec::new(), progs: Vec::new(), entries: 0, steps: Vec::new() })
    }

    /// A new point: the programs and slots that follow run and fill there.
    pub(super) fn point(&mut self) {
        let slot = (self.image.len() as u32) / SLOT_WORDS;
        if let Some(p) = self.plan.points.last_mut() {
            p.n = self.progs.len() as u32 - p.first;
            p.slots.1 = slot;
        }
        let at = self.plan.points.len();
        self.plan.points.push(Point { first: self.progs.len() as u32, n: 0, slots: (slot, slot), bind: None, _cfg: None });
        self.steps.push(Step::Point(at));
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
        self.progs.push(program(Dst::Cfg(slot * SLOT_WORDS + w as u32), &e));
    }

    /// A census word computed on the device at this point.
    pub(super) fn count(&mut self, w: usize, e: E) {
        self.progs.push(program(Dst::Census(w), &e));
    }

    /// A flag raised where `e` is not none, kept raised.
    pub(super) fn flag(&mut self, f: census::Flag, e: E) {
        let w = f.word();
        self.count(w, E::At(w).max(e));
    }

    /// An indirect dispatch's workgroups, `groups` of them; its offset.
    pub(super) fn groups(&mut self, groups: E) -> Size {
        let at = self.entries;
        assert!(at < ARGS, "an argument the plan holds");
        self.entries += 1;
        let m = E::C(MAXX);
        self.progs.push(program(Dst::Args(4 * at), &groups.clone().min(m.clone())));
        self.progs.push(program(Dst::Args(4 * at + 1), &groups.div(m)));
        Size::Indirect(u64::from(16 * at))
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

    pub(super) fn step(&mut self, s: Step) {
        self.steps.push(s);
    }

    pub(super) fn dispatch(&mut self, (pass, entry): (PassId, usize), bind: wgpu::BindGroup, size: Size) {
        self.steps.push(Step::Dispatch { pass, entry, bind, size });
    }

    /// `bytes` of `from` at `at` into the census at word `w`.
    pub(super) fn tally(&mut self, from: &wgpu::Buffer, at: u64, w: usize, bytes: u64) {
        let to = (*self.plan.census).clone();
        self.steps.push(Step::Copy { from: from.clone(), at, to, to_at: 4 * w as u64, bytes });
    }

    /// The steps recorded since the last take.
    pub(super) fn take(&mut self) -> Vec<Step> {
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
        self.plan.code = storage(gpu, "plan code", code.len() as u64, wgpu::BufferUsages::empty(), &code)?;
        gpu.queue.write_buffer(&self.plan.cfgs, 0, bytemuck::cast_slice(&self.image));
        gpu.queue.write_buffer(&self.plan.uniforms, 0, bytemuck::cast_slice(&self.image));
        for p in &mut self.plan.points {
            let words = [p.first, p.n, 0, 0];
            let cfg = io::buffer(gpu, "plan point", 16, wgpu::BufferUsages::UNIFORM, bytemuck::cast_slice(&words))?;
            let all = [&*cfg, &self.plan.code, &self.plan.census, &self.plan.cfgs, &self.plan.args];
            p.bind = Some(vm.pass.bind(gpu, &all));
            p._cfg = Some(cfg);
        }
        Ok(self.plan)
    }
}

impl Plan {
    /// Point `k` run: its programs, then its slots into the uniforms.
    pub(super) fn run_point(&self, vm: &super::vm::Kernels, e: &mut wgpu::CommandEncoder, k: usize) {
        let p = &self.points[k];
        if p.n > 0 {
            vm.pass.run(e, p.bind.as_ref().expect("finished"), 0, p.n.div_ceil(64));
        }
        if p.slots.1 > p.slots.0 {
            let at = u64::from(p.slots.0 * SLOT_WORDS) * 4;
            let bytes = u64::from((p.slots.1 - p.slots.0) * SLOT_WORDS) * 4;
            e.copy_buffer_to_buffer(&self.cfgs, at, &self.uniforms, at, bytes);
        }
    }
}
