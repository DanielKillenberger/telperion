# A growing phytomer keeps what growth reads

Child of fn-351 (the speed aim; P1's old trees). Host-written, 12 Nov 2026. Builds on the phase's top branch (`fn-357-a-retired-root-keeps-only-what-is-read`).

## Problem

Measured after fn-357 (`.flow/evidence/fn-357-a-retired-root-keeps-only-what-is-read/TASK1-DONE.md`, the oak on seed 1002 at 120): the grower's footprint is 4.795 GiB, of which the phytomers' store is 2.018 GiB for 11.56 million held phytomers, about 175 bytes each; next come the axes (0.711), the pencils (0.637) and the draws (0.441). A `Phytomer` (`crates/telperion-space/src/structure.rs`) carries its node's position, its heading and side (three vectors), a radius, and five scalars in `f64`.

## What is to be found first (not checked by the host)

Which of a phytomer's fields growth itself reads between cycles, and which are written only by the settle or the rough layout and read only there or in the drawing; for each field, the reader with file and line, and the precision the reader needs (the contract already computes several of these in `f32`). The same for a `Pencil` and for the per-axis records of an axis that is closed but still held.

## The design (host), subject to that reading

- The grower's store holds, for each phytomer, only what growth reads between cycles, at the width its readers need. What the settle lays (position, heading, side, radius) lives in the settled tree, not in the grower.
- If a closed axis's records are read less than an open one's, the same is done for them.
- The tree is the same to the bit: no digest pin moves, no existing test changes. A width is narrowed only where the stored value is already an `f32` value or a contract `f32` path reads it; otherwise it stays.

## Acceptance Criteria

- **R1.** The reading, in the evidence, before any product change.
- **R2.** Every tree byte-equal: no digest moves; the paused, year, repeat and let-go tests pass unchanged; the narrow suites green; a new test would fail if a dropped field were read.
- **R3.** Measured before and after on seed 1002, the oak at 80 and 120 and the spruce at 40: the footprint by store, the process's peak, one-thread seconds in one load window. Expected, if the four geometric fields leave the store: the phytomers' store down by about half and the footprint at 120 by about a fifth. If the reading shows growth reads them all, return with the reading and no change.
- **R4.** Docs say what the grower holds of a phytomer and what the settle adds.

## Stops

- A field growth reads that cannot leave the store or narrow without changing a result: it stays; say which.
- Any digest moving; any public type's meaning changing for a caller of the package (the settled `Phytomer` a caller reads keeps its fields).

## Boundaries

No change to any rule of growth. No new value.

## Rigor

Product code, Opus; byte-equality is the proof; narrow tests while building; the gate and review at the phase's landing.
