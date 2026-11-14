# The grower's speed loop: the same tree to the bit, each pass cheaper

Child of fn-351 (P3; Phase 5's speed loop, plan section 4 item 1). Host-written, 14 Nov 2026. Builds on `fn-364-a-tip-extends-by-what-its-balance-leaves`.

## Problem

P3's aims (host's numbers): the oak at 40 in 100 ms natively, to 300 in 15 s. Measured on fn-364's grower on 14 Nov (`early three <preset> <age> <seed> cycle`, four threads, the machine loaded; `scratchpad/p5split/`): the oak at 40 grows in 0.98 s (Balance 0.23, Sketch 0.18, Units 0.14, Wood 0.07, ShadeField 0.07, Allot 0.05, the rest under 0.05 each); the spruce at 40 in 4.67 s (Sketch 1.04, Balance 1.03, Wood 0.42, Units 0.41, ShadeField 0.41, Allot 0.25); the beech at 80 in 14.6 s (Balance 3.25, Units 2.65, Sketch 2.60, Wood 1.51, ShadeField 1.22, Allot 1.13); the oak at 120 in about 30 s once a 123 s watcher pause inside its Balance pass is taken out (Balance about 9.6, Sketch 5.4, Units 4.3, Wood 2.8, Allot 2.5, ShadeField 2.2). No pass dominates. The oak at 40 spends about 3 microseconds per living apex per year (310,820 apex-years) across all passes, which is thousands of instructions for a handful of draws and sums: the cost per apex is the lever this spec pulls. How many apexes are grown is counted foliage's question and not this spec's.

## The design (host)

The plan's loop, on the grower alone, with the tree held equal to the bit: profile, name the largest cost, remove it, measure, repeat while a round gains a tenth or more on the oak at 40 or the beech at 80.

- **Equal to the bit, every round.** Every shipped preset's digest (the existing pin) and the oak at 120 on seed 1001 by `tree_digest` are unchanged. A change that reorders a floating-point sum, a draw, or the order-free deposit's quanta is not a round of this loop: return with it and what it would gain.
- **Measure in CPU seconds, not wall seconds.** The yield watcher freezes queue jobs for minutes at a time, and a frozen process accrues no CPU time. Each round's before and after is the process's user plus system CPU seconds (`python3 scratchpad/peak.py <cmd>` prints CPU_S, WALL_S, PEAK_MIB) with `--threads 1` in the queue so the number is one core's work, three runs each, median. Per-pass seconds from the `early` report are quoted only from runs with no PAUSE inside (`scratchpad/yield.log`).
- **Profile with what the machine has** (`perf` if it is installed and permitted; otherwise scoped timers behind a test-only or example-only switch that never ships). Nothing is installed.
- **Where to look first**, from the split: the balance (largest on three of four trees; per living lateral subtree it keeps sums, a remembered balance, a keyed draw and a window), the rough layout (Sketch), the unit growth (keyed draws), the wood pass (visits every held open phytomer), the allotment's normalisation. For each: what is recomputed every year that the year before already had, what is allocated per apex per year, what is hashed more than once, what touches cold memory. The reading names candidates with their expected gain before any is built.
- A round that needs a new record, a changed record layout, or a new carried state is built only if R3's size tests and the year-independence tests hold as they are.

## Acceptance Criteria

- **R1.** The reading first, in the evidence: for each of the six largest passes, what it does per apex, per axis and per phytomer each year, its allocations, its hashes and draws, and the candidates with the gain expected on paper.
- **R2.** `ROUNDS.md`: one row per round with the change in one sentence, CPU seconds before and after on the oak at 40 (seed 1001), the beech at 80 (seed 1), the spruce at 40 (seed 1) and the oak at 120 (seed 1001), peak memory, and the digests equal. One commit per round. A round that gains under 3% on every tree is reverted, not kept.
- **R3.** After every kept round: the five tests of `grow/cycle/paused/tests.rs`, `every_preset_grows_the_same_tree_again_and_on_any_cores`, `many_cores_grow_the_tree_one_does`, the growing-phytomer and retired-root size tests, and the preset digest pin, by name.
- **R4.** The loop ends when a round gains under a tenth on both the oak at 40 and the beech at 80, or at a Stop. The final table states where the oak at 40 stands against 100 ms on one thread and on four.
- **R5.** The two sub-second guards before finishing.

## Stops

- A gain that needs the tree to change (a reordered sum, a different draw, a rule read differently).
- A gain that needs a law or a carried state the host has not written.
- Three rounds in a row reverted.

## Boundaries

`crates/telperion-space` and the example or test tooling that measures it. No preset, no pipeline stage after growth, no law.

## Rigor

Opus builds. Heavy runs through `telperion-heavy` with `TELPERION_HEAVY_RESERVE=8` in the background, checked by the clock. Friction in this spec's FRICTION.md as it happens.
