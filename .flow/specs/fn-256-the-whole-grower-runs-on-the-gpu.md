# The whole grower runs on the GPU

## Goal & Context

fn-245's third stage. Today the CPU grows structure above a fine-root cut and the GPU grows each chain alone in one invocation (`chain.wgsl:1`), which cannot carry feedback between branches. Astra's architecture review (fn-245 `ASTRA-ARCH.md` §1, §4) proposes putting every axis on the device grower, scheduled in bounded blocks with dispatch boundaries for dependencies, so the CPU/GPU cut and the expected-phytomer predictor that places it can go. Release item.

## Requirements

- **R1 (one executor per context):** the executor is chosen when the build context is created, never per species; every axis belongs to the device grower; no per-cycle CPU botanical work. A GPU capacity failure returns a named error and is never rebuilt on the CPU.
- **R2 (scheduling never changes the tree):** buds and axes in compact arrays with stable parent/child ranges; queues, totals and indirect dispatch arguments stay resident; block size and scheduling changes yield the canonical tree (fn-245 stage 1 R4 check 1).
- **R2a (the device's node table repeats; from fn-261, host 2026-10-11):** the device-grown node table is byte-identical run to run for every preset; today the beech's overflow queue (`record.wgsl`, `atomicAdd` order) and an unnamed source on Laurelin renumber node slots between runs while leaves, segments and stills stay identical. Check: fn-261's probe (`.flow/evidence/fn-261-the-same-binary-draws-the-same-leaves/grown-probe-and-queue-experiment.patch`, made a test), red before the fix.
- **R3 (deletions):** the fine-root execution cut (`fine.rs:6`) and, if no other consumer remains, `closed_form/predictor.rs`; capacity comes from checked counts and bounded allocation, never an expectation used as a hard bound. The unbent-before-bent frame dependency (`fine/lay.rs:58`) is kept.
- **R3a (CPU peak, from fn-258 RESULT.md):** the oak at 100 peaks at 9.0 GB of CPU memory during preparation against master's 0.35 GB (7.6 GB at seed 42; the ordinary at 100, 6.5 GB). The peak during preparation is measured per preset and brought to or under master's, or the remainder named for the owner.
- **R4 (proved):** CPU-GPU agreement (stage 1 R4 check 4) on every preset; build time per preset including preparation, transfers and device completion, beside master and the current branch; cold compile time; stills viewed; Astra's agreement; gate and npm test green.

## Boundaries

Execution placement only; the growth law is stage 2's. Release item for fn-237.
