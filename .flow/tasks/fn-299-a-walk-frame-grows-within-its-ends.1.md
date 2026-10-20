---
satisfies: [R1]
---
# fn-299-a-walk-frame-grows-within-its-ends.1 The repro and the trace: oak to palm leaves its ends

## Description
R1. A CPU-only test reads `most_units` along oak to palm at age 40 at the spec's eight t and three seeds, and fails on the base where a frame leaves the range its ends span. A step trace of `most_units`'s chain at t = 0.962 (and at beech to spruce t = 0.00037) confirms or refutes the suspected cause. The task ends with a proposed blend rule and its measured effect on `most_units` at t = 0.5, 0.9 and 0.97 (read from a scratch probe, never committed), written to `.flow/evidence/fn-299-a-walk-frame-grows-within-its-ends/R1.md`, and returns `NEEDS_HUMAN` to the host before any fix (spec, Decision Context), unless the trace shows one obvious rule whose only effect is to keep the frame within its ends.

The repro test lands committed red, marked `#[ignore = "fn-299 R2: red until the blend rule lands"]` so the gate stays green; task .2 removes the ignore.

**Size:** M
**Files:** `crates/telperion-core/tests/units_bound.rs`, `.flow/evidence/fn-299-a-walk-frame-grows-within-its-ends/R1.md`
**Touches:** [crates/telperion-core/tests/units_bound.rs, .flow/evidence/fn-299-a-walk-frame-grows-within-its-ends/**]

### Approach
- Follow `the_floor_counts_the_ages_a_cycle_chains` and the `at` / `floor` helpers at `crates/telperion-core/tests/units_bound.rs:30-52`.
- The range is [min, max] of `floor` at t = 0 and t = 1 for the same seed; print every frame's count before asserting so the red run states the whole table.
- The trace may be a scratch test (fn-298's `fn298_palm_units` in `/home/daniel/Projects/telperion/.worktrees/f4-298/.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/raw/t4-scratch.patch` prints each age's rows), ported to the CPU and kept out of the commit; record its output under `raw/`.
- Every cargo run goes through `telperion-heavy 8 --threads 2 -- <cmd>`.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-space/src/schedule.rs:259-301` - `most_units`, the chain it counts
- `crates/telperion-space/src/blend.rs:38-120` - `state`, `outcomes`, `mixed`
- `crates/telperion-space/src/species/scale.rs:57-95` - `Scale::mix`, `Log { floor }`
- `crates/telperion-space/src/species.rs:415-473` - `canonical`, `lived`, `successor`

**Optional** (reference as needed):
- `crates/telperion-space/src/species/scale/table.rs:60-95` - the rows' scales
- `/home/daniel/Projects/telperion/.worktrees/f4-298/.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/R2.md` - last section, fn-298's readings (read only)

### Key context
`most_units` reads only the species, so seeds should agree; a seed that disagrees is a finding.

## Acceptance
- [ ] The repro test exists in `units_bound.rs`, ignored with the fn-299 R2 reason, and its red run on the base is saved under `.flow/evidence/fn-299-a-walk-frame-grows-within-its-ends/raw/` with the per-frame table.
- [ ] `R1.md` states the trace: the chain `most_units` counts at t = 0.962, each age's blended `lifespan`, `continuation` and `relay_ended`, and whether beech to spruce at t = 0.00037 is the same cause.
- [ ] `R1.md` proposes the rule with `most_units` at oak to palm t = 0.5, 0.9 and 0.97 under it, measured by a scratch probe, and the task returns to the host before task .2.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
