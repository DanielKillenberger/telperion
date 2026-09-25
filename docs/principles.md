# The design principles, checked

STRATEGY.md's "Our approach" is checked on every push by three deterministic guards, which block what is certain. Jev review of specs and designs against what already exists is planned in fn-156. This page is the guide; the code is `crates/telperion-jev/src/principles/` and the data is `crates/telperion-jev/data/principles/`.

## The policy

`policy.json` holds five principles. Each has a stable question id, the exact clause it rests on and the legitimate readings it allows. Exceptions and PR Decisions lines cite these ids. Nothing is derived from STRATEGY.md by code: a person edits a clause, and bumps `version` with it.

| Id | Clause (STRATEGY.md, Our approach) |
|---|---|
| `P-ONE-PIPELINE` | Generation is one pipeline that every tree passes through; each family feature is a term inside a stage. |
| `P-NO-FALLBACK` | An input the pipeline cannot represent is an explicit error, never a fallback path. |
| `P-NO-SWITCH` | Every parameter changes the tree by degree; it may lie dormant; it is never a switch between ways of building. |
| `P-CONSUMER-READS` | Generate only the detail the consuming engine needs. |
| `P-BLOCKING-STEP` | A step that stops work must catch a defect the steps around it cannot. |

Measured cost and look are held by the artifact budgets below and by the specs' own measurements; timing and memory are measured on named hardware, never per push.

## Exceptions

`exceptions.json` is the registry. An entry names a principle, the exact caller symbols and stages it covers, why, and the owner decision or strategy allowance it rests on. No entry covers a pull request, a directory or future behaviour, and a test rejects one that tries. A code comment, a spec or a PR's Decisions line cites an entry by id; a claim of approval with no entry is none. The day-one entries:

- `EX-17`, the growth path (`SpecimenView::mesh`): a hidden feature, kept buildable (AGENTS.md, Mature trees are the product; PR #17).
- `EX-50`, the GPU executor (`Generator::prepare_async`): one algorithm with two executors (STRATEGY.md; PR #50).

## The guards, which block

They run in `cargo test` (`crates/telperion-jev/tests/principles_guards.rs`, and the entry tests in each crate) and in the pre-push hook.

1. **Production boundary.** Production code outside `telperion_core::pipeline` and `telperion_core::mesh` never calls a build stage (`policy.json`, `boundary.stages`). Imports, renames and `pub use` re-exports are resolved; test modules, `#[test]` functions and the `tests`, `examples` and `benches` trees are never read; a glob import or a type alias of a stage module outside it fails with its location. On a push, callers the change adds are compared with those it removes, so a move is not an addition. `jev principles boundary` checks the checkout; `jev principles push --base B --head H` checks a change.
2. **Entry coverage.** Every shipped preset builds through every callable generation entry: native (`every_family_builds_the_same_bytes_under_either_schedule`), the main Wasm (`every_shipped_preset_builds_through_the_main_entry`) and the slim field (`every_shipped_preset_answers_as_the_main_pipeline_does`, fn-150's regression). `policy.json` gives every package export a role, and a test fails when an export has none. A change to preset values, catalogue membership, entry wiring or the build script triggers the three tests in the hook, each crate alone, so the slim entry runs in its shipped configuration.
3. **Artifact budgets.** `budgets.json` holds each shipped Wasm module's and script's budget from the fixed recipe (`npm run build`), with the measurement and the Decisions reference behind it. CI's package job measures the built package (`jev principles budget`). A budget that moves without a new measurement and Decisions reference fails, in `cargo test` and on a push.

## The hook

`npm run setup` sets `core.hooksPath` to `.githooks` once, and every worktree of the clone then runs `.githooks/pre-push`. It runs `jev principles push` on each pushed ref, against its remote tip or, for a new branch, its merge base with `origin/master`, and builds the checker once per change to `crates/telperion-jev`. A guard failure blocks the push. Entry coverage triggered on a checkout that is not the pushed head reports **incomplete**, never clean. `git push --no-verify` skips the hook. CI runs the boundary and budget-evidence guards on each pull request, and its crate jobs run the entry tests.

Measured on this desk under a load average of 15: 0.40 to 0.49 s for the boundary and budget guards on this branch's push; the first run in a checkout builds the checker (about 45 s), and a triggered entry-coverage run adds 57 to 91 s of cargo tests.

## Jev review, planned

fn-151 built and measured an advisory Jev reviewer on 39 owner-labelled pull requests. It caught 2 of 3 Jev-owned positives in calibration and 0 of 11 in the holdout, so the owner cut it on 2026-09-25 and moved design review to fn-156. The measurements and recordings are fn-156's baseline: `.flow/evidence/fn-151-the-design-principles-are-checked/EVALUATION.md`.
