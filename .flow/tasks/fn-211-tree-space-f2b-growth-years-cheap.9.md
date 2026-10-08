---
satisfies: [R4]
---
# fn-211-tree-space-f2b-growth-years-cheap.9 Contract A on a small lit tree across a re-lay

## Description
Host decision on CONTRACT-A.md 2.14 (2026-10-07): option (a), the final lay in fixed point too: one layout implementation in fixed point for rough and final, converted to f64 only at the output; measure the settle's cost against today's f64 settle and report if it exceeds 1.3x. Then Astra's next integration proof (ASTRA-REVIEW-CONTRACT-A.md section 4): the CPU reference moved onto contract A (layout, deposits, sampling, balance, allocation, re-lay) for a small lit specimen spanning a scheduled re-lay; complete phase state compared CPU against the GPU mirror of the slice extended to fine roots, across cuts, orders, chunkings and a retry. Show what contract A changes in today's trees per the review's section 3 (the 28-tree comparison by lineage, topology changes apart from displacement, stills of the four species at 80 viewed) on the CPU reference before the GPU mirror widens.

## Acceptance
- Option (a) built; settle cost measured.
- The small lit specimen identical CPU against GPU in every phase state across cuts, orders, chunkings and a retry.
- What A changes in today's trees shown (comparison and stills viewed).
- Gate green (two parts, 8-thread limits).

## Done summary
9a (host split, 2026-10-07: the specimen proof CPU against device moved to task 10). The CPU reference on contract A, feedback-first: light lattice and sampling, D16, allocation, one fixed-point layout for the rough and the final lay (option (a)), re-lay, girth and sag. Cost decisions (host): vector-built rotations, polynomial sin/cos, atan2, log2 and exp2_neg (the decision contract re-specified, FINE-CONTRACT.md), carry's Newton step, the tropism's elevation from its lean. Final whole builds against the base at 80: beech +6 %, spruce +22 %, oak +4 %, lit oak +1 % (layout 1.7 to 2.3x); the host accepts the residual for the device. Against the base, topology unchanged in 27 of 28 trees (the lit oak at 80 seed 7: 39 of 342,705 axes), tips at most 55 um elsewhere; the four species' stills at 80, bare and in leaf, the oak lit too, viewed side by side: none can be told apart. Pins re-taken once (the closed form's rows). Probes (contract and feedback) green word for word. Gate green: render 166, rest 1,012. TASK9-CPU.md.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: dd4f4bef, 4f6c2888, 02979d5a, 7bd941e2, 0c70e64b, 053ba4ec, a64b3045, cb8466bd, f4dd8b0e, de934601, 02cd51bc, afa6eec2, eb3f1416, 5e85bf6c, 0c6ec96e, 61891ef8
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (under gpu.lock), cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render
- PRs: