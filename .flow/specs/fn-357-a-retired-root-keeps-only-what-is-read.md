# A retired root keeps only what is read of it

Child of fn-351 (the speed aim; P1's old trees). Host-written, 12 Nov 2026. Builds on the phase's top branch.

## Problem

Measured (`.flow/evidence/fn-351-020-the-release-candidate/PHASE5-CENSUS.md`, the oak on seed 1002): at 120 the grower's own stores are the process's peak, 7.11 GiB, and 57.9% of the 5.08 million axes it holds are roots of subtrees already retired. Such a root has no phytomers left; it stands for its subtree (`grow/cycle/retire.rs`, `compact.rs`). Each still carries every per-axis store, about 1,050 bytes on average, about 2.9 GiB of the 7.1 at 120 (an estimate from the average, not counted store by store). At 80 it is 53.6% of 2.24 million axes; on the spruce at 40, 55.8% of 706 thousand.

## What is to be found first (no claim is made here that was not run)

Not checked by the host: what is still read of a retired root, by which pass, and until when (the balance, an ancestor's evaluation at its fate, the fades at its living, the settle, the wood pass, the census). The builder reads this from the code and writes it down before changing anything.

## The design (host), subject to that reading

- A retired root keeps only the fields something still reads, in a small record of its own, and its other per-axis stores are let go at once, or when its fade has run out if that is when the last read falls.
- If nothing reads a retired root once its fade has run out, it is let go entirely then.
- The tree is the same to the bit: every digest pin and every equality test stands unchanged. This is a change of representation only.

## Acceptance Criteria

- **R1.** The reading: for each per-axis store, what reads it for a retired root and until when, with file and line, in the evidence.
- **R2.** Every tree is byte-equal: no digest pin moves; the paused, year and repeat tests pass unchanged; the narrow suites are green.
- **R3.** Measured before and after on seed 1002, the oak at 80 and 120, the spruce at 40: the grower's footprint by store, the process's peak, held axes by kind (living, closed, retired roots), one-thread seconds in one load window. Expected: the footprint at 120 down by about a third. If it falls by less than a tenth, say why and return.
- **R4.** Docs say what a retired root keeps.

## Stops

- A read of a retired root that needs a field whose removal would change a result: return with the site.
- Any digest moving.

## Boundaries

No change to what is retired, when, or to any rule of growth. No new value.

## Rigor

Product code, Opus; byte-equality is the proof; narrow tests while building; the gate and review at the phase's landing.
