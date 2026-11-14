# Generate writes three holdout seeds beside the three fixed ones

## Goal

The onboarding protocol asks every species for three fixed and three holdout seeds, the holdouts fresh and persisted before anyone inspects them, but `generate/packet.rs` wrote `"holdout_seeds": []`, so the seeds gate failed for every species and was waived by hand each run (fn-80, 2026-09-24). An obvious friction fix under the owner's standing rule of 2026-09-23.

## Decisions

- Host design: generate draws three holdout seeds deterministically from the species id (distinct from the fixed seeds, never chosen by looking at a render), persists them in `species.json` `holdout_seeds` and as `holdout` cases in `specimens.json`, and generates their specimens; a rerun keeps them.

## Acceptance Criteria

- **R1:** Three holdout seeds and cases, distinct and stable; the seeds gate resolves 3 and 3 — met (red then green).
- **R2:** Gate green — met (1054 passed).

## Result

Landed in `1bd7dd12` (`write_packet` now takes `&Paths`); carried by the fn-149 rewrite (#121).

## Open

None.
