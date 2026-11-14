# The build refuses a family its own checks refuse

## Goal & Context

Design principle 2 (owner, 2026-09-25) says an input the pipeline cannot draw fails with an error and never takes a fallback path. fn-306's conductor (2026-10-22) reported that `pipeline::build` never calls `Family::validate` (`crates/telperion-core/src/family.rs:91`). Its example: an age above the declared bound is refused by `validate`, but if it does not come through `artifact_digest`, which now calls `validate` itself, it reaches growth. Today `validate`'s callers are the digest example and `tree.rs:361`.

## Unknown, check before the plan

- Whether any entry builds a family without `validate`: the Wasm `generate` entry, `Generator::prepare`, the film walks' blended families, and `mesh::build`. The fn-306 report is unchecked.
- What an age over the bound does today: an error later on, a clamp, or a tree drawn from out-of-range values. Run it as a standalone test before an acceptance criterion is written.
- What `validate` costs, since it builds an `Instances` reference, and whether the one pipeline already performs part of it.

## Requirements

- **R1 (repro):** a standalone test that builds a family with an out-of-bound age through each public entry and records what each one does.
- **R2:** every public entry refuses such a family with `validate`'s error before growth, once, in one place inside the one pipeline. Blended walk families pass the same check.
- **R3:** shipped presets and every film-walk frame build byte-identically. The cost is measured, and the gate and `npm test` are green, with a codex review per task.

## Boundaries

- Post-release, unless R1 finds a shipped entry drawing a tree from out-of-range values.
