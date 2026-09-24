# Generate writes three holdout seeds beside the three fixed ones

## Conversation Evidence

> fn-80 live run, 2026-09-24: the palm's seeds gate filed "3 fixed and 0 holdout seeds, fewer than 3 each" after generate ran; `generate/packet.rs` writes `"holdout_seeds": []` and only `regression` cases, so no species can pass the gate. The host waived it for the palm's run and specced the fix under the owner's standing rule for obvious friction fixes.

## Goal & Context
<!-- scope: business -->

The onboarding protocol asks every species for three fixed and three holdout seeds, the holdouts fresh and persisted before anyone inspects them. The pipeline writes only the fixed three, so the seeds gate fails for every species and is waived by hand each run. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-24 on the fn-80 branch.** `pipeline/stages/generate/packet.rs` `write_packet` writes `fixed_seeds` = the manifest seed and the next two, `holdout_seeds: []`, and one `regression` case per fixed seed; `pipeline/stages/gate.rs:331-337` counts roles `regression` and `holdout` and requires three of each (`SEEDS_PER_ROLE`). [checked]
- **Shape.** [host design] Generate draws three holdout seeds deterministically from the manifest seed and species id (distinct from the fixed ones, never chosen by looking at a render), persists them in `packet/species.json` `holdout_seeds` and in `specimens.json` as `holdout` cases, and generates their specimens like the fixed ones. A rerun keeps the same holdouts.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Generate writes three holdout seeds and three `holdout` cases, distinct from the fixed ones and the same on a rerun; the seeds gate resolves with 3 and 3. [inferred]
- **R2:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Decision Context

- An obvious friction fix under the owner's standing rule of 2026-09-23. [paraphrase]

## Open Questions

- None.
