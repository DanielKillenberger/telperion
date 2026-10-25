//! A recorded cycle's rough layout and light (DESIGN.md sections 2.3 and
//! 2, step 8): the axes that grew listed by index and drawn in ranges
//! of grown axes, the new axes framed and drawn a generation at a time
//! up to the cycle's limit and the deeper ones by a closing pass (fn-298
//! R2 (d)), the limit only a cost hint; then the cycle's leaves and the living
//! apexes' tips gathered, the box, the deposit in ranges of leaves,
//! the sweep's layer ranges up to their bound, the reads into the
//! pencils. Each bound passed raises its flag.
use super::build::{Build, Kit};
use super::build_light::LAYERS;
use super::census::{self as c, Flag};
use super::gate::{rec, Cond};
use super::glue;
use super::hier_scan::sums_words;
use super::hier_scan::zeroed;
use super::lay_device::{self as ld, DEEP, T_DEEP, T_FAULT};
use super::plan::{Builder, PassId, Size, Step, MAXX};
use super::plan_scan::{scanned, COLUMN};
use super::vm::E;
use crate::buffer::Owned;
use crate::Result;

/// How far the layout's generations and the sweep's layer ranges are
/// recorded, and the deposit's ranges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LayBounds {
    pub generations: u32,
    pub sweeps: u32,
    pub deposits: u32,
    /// The draw's dispatches over the grown axes, at most.
    pub draws: u32,
}

impl Build {
    fn lay_slot(&self, b: &mut Builder, (n, listed, generation, base): (E, u32, u32, u32)) -> u32 {
        let s = self.state.caps;
        let slot = b.slot(&[
            0,
            MAXX * 64,
            0,
            0,
            s.axes,
            s.phyts,
            s.units,
            0,
            listed,
            generation,
            0,
            0,
            0,
            base,
        ]);
        let first = E::At(c::LAID0);
        let new = E::At(c::AXES).sub(first.clone());
        for (w, e) in [
            (0, n),
            (2, E::At(c::CYCLE)),
            (3, first),
            (7, new),
            (11, E::At(c::GREW)),
        ] {
            b.set(slot, w, e);
        }
        slot
    }

    fn lay_bind(&self, b: &Builder, kit: &Kit, slot: u32) -> wgpu::BindGroup {
        let (s, l) = (&self.state, &self.layout);
        b.bind(
            &kit.layout.layout,
            slot,
            &[
                &l.form, &s.units, &s.grown, &s.laid, &s.axes, &s.pencils, &l.lists, &l.counts,
            ],
        )
    }

    /// The cycle's layout, recorded.
    pub(super) fn plan_layout(
        &self,
        b: &mut Builder,
        kit: &Kit,
        (bound, range): (LayBounds, u32),
    ) -> Result<Vec<Owned>> {
        let s = self.state.caps.axes;
        b.step(Step::Clear {
            buffer: (*self.flags).clone(),
            at: 0,
            bytes: None,
        });
        b.step(Step::Clear {
            buffer: (*self.layout.lists).clone(),
            at: 4 * T_DEEP,
            bytes: Some(4 * u64::from(DEEP)),
        });
        b.point();
        let (live0, laid0) = (E::At(c::LIVE0), E::At(c::LAID0));
        let w = glue::Words {
            live_stride: self.sums.live_stride,
            ..glue::Words::default()
        };
        let mark = [(0, live0.clone()), (14, live0), (3, laid0.clone())];
        self.glue_step(
            b,
            kit,
            (glue::GREW_MARK, &self.flags, &self.spare),
            w,
            &mark,
        );
        scanned(
            b,
            &kit.hier.scan,
            (&self.flags, u64::from(s), s, COLUMN),
            &self.scan_sums,
            (&self.scan_tally, 0),
            laid0.clone(),
        );
        let gather = [(0, laid0.clone()), (3, laid0)];
        self.glue_step(
            b,
            kit,
            (glue::GREW_GATHER, &self.flags, &self.layout.lists),
            w,
            &gather,
        );
        b.tally(&self.scan_tally, 0, c::GREW, 4);
        b.point();
        let grew = E::At(c::GREW);
        let new = E::At(c::AXES).sub(E::At(c::LAID0));
        let lay = |k| (PassId::Layout, k);
        for r in 0..bound.draws {
            let slot = self.lay_slot(b, (grew.clone(), 0, 0, r * range));
            let left = grew.clone().sub(E::C(r * range)).min(E::C(range));
            let size = b.groups(left.div(E::C(64)));
            let bind = self.lay_bind(b, kit, slot);
            b.enter(Cond::Draw(r));
            b.dispatch(lay(ld::DRAW), bind, size);
            b.leave();
        }
        let sums = zeroed(b.gpu, "layout scan sums", sums_words(s, 1))?;
        let over_new = b.groups(new.clone().div(E::C(64)));
        let slot = self.lay_slot(b, (new.clone(), 0, 0, 0));
        let bind = self.lay_bind(b, kit, slot);
        b.dispatch(lay(ld::RELAY), bind.clone(), over_new);
        b.dispatch(lay(ld::GENERATION), bind, over_new);
        for g in 0..bound.generations {
            let slot = self.lay_slot(b, (new.clone(), 1, g, 0));
            let bind = self.lay_bind(b, kit, slot);
            b.enter(Cond::LayGen(g));
            b.dispatch(lay(ld::FLAG), bind.clone(), over_new);
            let lists = &self.layout.lists;
            scanned(
                b,
                &kit.hier.scan,
                (&self.layout.counts, 0, s, COLUMN),
                &sums,
                (lists, 0),
                new.clone(),
            );
            for entry in [ld::GATHER, ld::FRAME, ld::FRAME_ON, ld::DRAW] {
                b.dispatch(lay(entry), bind.clone(), over_new);
            }
            b.leave();
        }
        // The closing pass: each new axis the generations recorded left,
        // laid chain by chain under its head at the cycle's limit.
        let slot = self.lay_slot(b, (new.clone(), 0, 0, 0));
        b.set(slot, 9, E::At(rec::LAYGENS));
        let bind = self.lay_bind(b, kit, slot);
        b.dispatch(lay(ld::TAIL), bind, over_new);
        b.tally(
            &self.layout.lists,
            4 * T_DEEP,
            c::LAY_DEEP,
            4 * u64::from(DEEP),
        );
        b.tally(&self.layout.lists, 4 * T_FAULT, c::LAY_FAULT, 4);
        b.point();
        // A generation past the plan's cap, reached (the last mark holds
        // every deeper one).
        let deeper = (bound.generations.min(DEEP)..DEEP)
            .fold(E::C(0), |a, g| a.max(E::At(c::LAY_DEEP + g as usize)));
        b.flag(Flag::LayGenerations, deeper);
        b.flag(Flag::LayFault, E::At(c::LAY_FAULT));
        b.flag(
            Flag::DrawRanges,
            E::At(c::GREW).gt(E::At(rec::DRAWS).mul(E::C(range))),
        );
        Ok(vec![sums])
    }

    /// The cycle's light, recorded.
    pub(super) fn plan_light(
        &self,
        b: &mut Builder,
        kit: &Kit,
        (bound, ranges): (LayBounds, super::build_plan::Ranges),
    ) {
        let l = &self.light;
        let leaves = E::At(c::PHYTS).sub(E::At(c::PHYTS0));
        let tips = E::At(c::NEXT);
        let w = glue::Words {
            live_stride: self.sums.live_stride,
            ..glue::Words::default()
        };
        let points = [
            (0, leaves.clone().add(tips.clone())),
            (11, E::At(c::PHYTS0)),
            (12, leaves.clone()),
        ];
        self.glue_step(b, kit, (glue::POINTS, &l.points, &self.spare), w, &points);
        let slot = |b: &mut Builder, chunk: (u32, u32)| {
            let slot = b.slot(&l.words(chunk));
            b.set(slot, 0, leaves.clone());
            b.set(slot, 1, tips.clone());
            b.set(slot, 9, E::At(c::CYCLE));
            b.bind(&kit.light.pass, slot, &l.buffers())
        };
        let main = slot(b, (0, 0));
        let light = |k| (PassId::Light, k);
        let span = b.groups(leaves.clone().add(tips.clone()).div(E::C(64)));
        b.step(Step::Copy {
            from: (*l.start).clone(),
            at: 0,
            to: (*l.state).clone(),
            to_at: 0,
            bytes: 32,
        });
        b.dispatch(light(0), main.clone(), span);
        b.dispatch(light(1), main.clone(), Size::Direct(1));
        // The deposit, and its refusals of a leaf's area and a node's
        // quanta, guard only the lattice's input: a dormant lattice
        // (nothing shades) reads none, so records neither (fn-256 task
        // 12), nor any of the lattice's own dispatches.
        let range = ranges.deposit;
        if !l.dormant {
            b.step(Step::Clear {
                buffer: (*l.quanta).clone(),
                at: 0,
                bytes: None,
            });
            for r in 0..bound.deposits {
                let bind = slot(b, (0, r * range));
                let left = leaves.clone().sub(E::C(r * range)).min(E::C(range));
                let size = b.groups(left.div(E::C(64)));
                b.enter(Cond::Deposit(r));
                b.dispatch(light(2), bind, size);
                b.leave();
            }
        }
        b.tally(&l.grid, 0, c::GRID, 32);
        b.tally(&l.grid, 32, c::SWEPT, 4);
        b.point();
        let nodes = E::At(c::GRID + 6);
        if !l.dormant {
            self.plan_lattice(b, (&slot, &main), (bound, nodes));
        }
        let read = b.groups(tips.clone().div(E::C(64)));
        b.dispatch(light(5), main, read);
        self.glue_step(b, kit, (glue::LIT, &l.reads, &self.spare), w, &[(0, tips)]);
        b.tally(&l.state, 24, c::LIGHT_ERRORS, 4);
        b.point();
        b.flag(Flag::LightErrors, E::At(c::LIGHT_ERRORS));
        let points = leaves.clone().add(E::At(c::NEXT));
        b.flag(
            Flag::Points,
            points
                .gt(E::C(l.points_cap))
                .max(E::At(c::NEXT).gt(E::C(l.tips_cap))),
        );
        // The deposit and the sweep, and so their ranges, only where the
        // lattice is swept: the sweep from the highest node a leaf with
        // area reaches down.
        if !l.dormant {
            b.flag(
                Flag::DepositRanges,
                leaves.gt(E::At(rec::DEPOSITS).mul(E::C(range))),
            );
            b.flag(
                Flag::Sweeps,
                E::At(c::SWEPT).div(E::C(LAYERS)).gt(E::At(rec::SWEEPS)),
            );
        }
        b.count(c::LAID, E::At(c::AXES));
    }

    /// The lattice's dispatches: the densities, then the sweep, a range
    /// of layers a dispatch, each over its tiles of every direction.
    fn plan_lattice<S>(
        &self,
        b: &mut Builder,
        (slot, main): (&S, &wgpu::BindGroup),
        (bound, nodes): (LayBounds, E),
    ) where
        S: Fn(&mut Builder, (u32, u32)) -> wgpu::BindGroup,
    {
        let light = |k| (PassId::Light, k);
        let all = b.groups(nodes.div(E::C(64)));
        b.dispatch(light(3), main.clone(), all);
        let tiles = b.groups(
            self.light
                .sweep_tiles((E::At(c::GRID + 3), E::At(c::GRID + 4))),
        );
        for k in 0..bound.sweeps {
            let bind = slot(b, (k, 0));
            b.enter(Cond::Sweep(k));
            b.dispatch(light(4), bind, tiles);
            b.leave();
        }
    }
}
