//! fn-256 task 5 (COUNCIL.md round 4, step 1): a halted cycle leaves every
//! later dispatch empty. A flag raised as a cycle begins halts it: no
//! kernel of the cycle writes the tree's state, whatever count it reads,
//! and the flag stands for the host to read a cycle late. Four driver
//! resets in tasks 2 and 4 came from kernels that ran on what a halted
//! cycle left (FRICTION.md).
use super::build::{Build, Late, Tables};
use super::build_grow::{grow_now, Sizes};
use super::build_size::limits;
use super::census::Flag;
use super::device::{T_ROUNDS, T_USED};
use super::given::Span;
use super::grown_tests::{device, grown};
use crate::Gpu;
use telperion_core::presets::Preset;

/// The buffers a cycle carries to the next: the tree's state, the
/// hierarchy's shape and lists, and retirement's columns.
const CARRIED: [&str; 10] = [
    "axes",
    "pencils",
    "units",
    "grown",
    "laid",
    "lists",
    "shape",
    "lists 0",
    "lists 1",
    "retirement",
];

/// Every word of what a cycle carries, as the device holds it now, but
/// the units' rounds flag and rounds used: a copy clears them as each
/// cycle's units begin, a clear and no dispatch (`plan_units.rs`).
fn state(gpu: &Gpu, b: &Build) -> Vec<Vec<u8>> {
    let (s, h) = (&b.state, &b.hier);
    let read = |buffer: &wgpu::Buffer| {
        super::super::io::read(gpu, buffer, buffer.size()).expect("a readback")
    };
    let carried = [
        &s.axes,
        &s.pencils,
        &s.units,
        &s.grown,
        &s.laid,
        &s.lists,
        &h.shape,
        &h.lists[0],
        &h.lists[1],
        &b.retire.ret,
    ];
    let mut words: Vec<Vec<u8>> = carried.map(|b| read(b)).into();
    words[5][4 * T_ROUNDS..4 * (T_USED + 2)].fill(0);
    words
}

#[test]
fn a_halted_cycle_leaves_later_dispatches_empty() {
    let Some((gpu, kit)) = device() else {
        return;
    };
    let g = grown(Preset::EuropeanBeech, 8, 42);
    let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
    let (b, sizes, _) =
        grow_now(&gpu, &kit, &t, &g.tables, (Sizes::least(), 5)).expect("grown to 5");
    let (plan, phases) = b.plan(&gpu, &kit, &t, sizes.bounds).expect("a plan");
    let mut census = b.census.clone();
    census[Flag::Axes.word()] = 1;
    gpu.queue
        .write_buffer(&plan.census, 0, bytemuck::cast_slice(&census));
    let before = state(&gpu, &b);
    let cycle = 6;
    let limits = limits(&sizes, Some(&b.census), None);
    let span = Span::new(
        &b.census,
        Some(&b.census),
        (&sizes.ceilings(), None),
        (cycle, &limits),
    );
    let mut late = Late::new(&gpu).expect("the census's readback");
    let (stamped, ..) = b.submit(
        &gpu,
        (&kit, &t),
        (&plan, &phases, &limits, &span),
        (&mut late, cycle),
    );
    let read = pollster::block_on(late.take(&gpu, cycle)).expect("the census");
    assert!(
        stamped.len() > 100,
        "the cycle recorded {} dispatches",
        stamped.len()
    );
    assert_ne!(read[Flag::Axes.word()], 0, "the flag stands for the host");
    let after = state(&gpu, &b);
    let written: Vec<_> = CARRIED
        .iter()
        .zip(before.iter().zip(&after))
        .filter(|(_, (a, b))| a != b)
        .map(|(n, _)| n)
        .collect();
    assert!(
        written.is_empty(),
        "a halted cycle wrote the tree's {written:?}"
    );
}
