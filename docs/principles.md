# The design principles, checked

STRATEGY.md's "Our approach" is checked on every push and, on request, on every spec. Deterministic guards block what is certain. The Jev reviewer only advises, and a principle earns a louder voice one step at a time, by measured precision on real pushes. This page is the guide; the code is `crates/telperion-jev/src/principles/` and the data is `crates/telperion-jev/data/principles/`.

## The policy

`policy.json` holds five principles. Each has a stable question id, the exact clause it rests on, the legitimate readings the reviewer is told about, its mode and its cuts. Nothing is derived from STRATEGY.md by code: a person edits a clause, and bumps `version` with it.

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

1. **Production boundary.** Production code outside `telperion_core::pipeline` and `telperion_core::mesh` never calls a build stage (`policy.json`, `boundary.stages`). Imports, renames and `pub use` re-exports are resolved; test modules, `#[test]` functions and the `tests`, `examples` and `benches` trees are never read; a glob import or a type alias of a stage module outside it fails with its location. On a push, callers the change adds are compared with those it removes, so a move is not an addition. `jev principles boundary` checks the checkout; `--base B --head H` checks a change.
2. **Entry coverage.** Every shipped preset builds through every callable generation entry: native (`every_family_builds_the_same_bytes_under_either_schedule`), the main Wasm (`every_shipped_preset_builds_through_the_main_entry`) and the slim field (`every_shipped_preset_answers_as_the_main_pipeline_does`, fn-150's regression). `policy.json` gives every package export a role, and a test fails when an export has none. A change to preset values, catalogue membership, entry wiring or the build script triggers the three tests in the hook, each crate alone, so the slim entry runs in its shipped configuration.
3. **Artifact budgets.** `budgets.json` holds each shipped Wasm module's and script's budget from the fixed recipe (`npm run build`), with the measurement and the Decisions reference behind it. CI's package job measures the built package (`jev principles budget`). A budget that moves without a new measurement and Decisions reference fails, in `cargo test` and on a push.

## The reviewer, which advises

`jev principles push` hands Jev only what code found, of three kinds: a branch the change added whose arms run different builders or drop work on a setting; a function or script the change added that shares most of its body with one that survives, or an added exit that hands back a stop; and a new output field or shader input nothing reads. Cargo features, cfg gates, names, commands and exports are never a candidate on their own.

Phase one asks, for all candidates in one call, which breach mechanism each shows and which excerpt shows it, or `none`, or `insufficient_evidence`. Phase two asks, for those above their principle's `select_min`, whether an excerpt shows the breach (`shown`) and whether a legitimate reading or a registered exception describes the change (`covered`). A warning needs `shown` at or above `confirm_min`, `covered` at or below `cover_max`, and code's check that the cited excerpt holds a line the change added. Otherwise the reviewer abstains. The two answers are never multiplied.

Every principle starts in **shadow**: its warnings are logged in `.flow/ledger/principles/runs.jsonl`, not shown. It moves to **warn**, and later to **block**, only when the owner's verdicts on its warnings from real pushes reach 95% precision over at least 20 reviewed warnings (`review::may_advance`); the move is an edit to `policy.json`.

**Spec mode**, `jev principles spec <spec.md>`, judges each current proposal with its decision context. What exists, measurements, unknowns, quotations, rejected designs and the designs a `Superseded` entry names are left out; a proposal with no decision context abstains. It advises before ready and adds no approval step. **Audit mode**, `jev principles audit`, reports debt that already exists; push and spec modes report only what a change introduces.

## The hook

`npm run setup` sets `core.hooksPath` to `.githooks` once, and every worktree of the clone then runs `.githooks/pre-push`. It checks each pushed ref against its remote tip, or against its merge base with `origin/master` for a new branch, and builds the checker once per change to `crates/telperion-jev`. A guard failure blocks the push. A missing key, a failed or slow call, a candidate overflow, or entry coverage triggered on a checkout that is not the pushed head reports **incomplete**, never clean. `git push --no-verify` skips the hook. CI runs the same guards on each pull request.

Bounds: at most 12 candidates, two batched calls and 8,000 input tokens per run, and a 10 s deadline; answers and extractions are cached under their full inputs and every version they depend on. Measured on this desk (fn-151, 14 recent master commits): 18 ms median and 19 ms p95 cached, 1.28 s median and 2.58 s p95 uncached.

## The corpus

`corpus.json` holds the 39 pull requests the owner labelled on 2026-09-25: each positive by mechanism and span on its immutable revision, calibration and holdout groups split by lineage, and matched clean cases for dormancy, count steps, backend choice, extraction and delegation, necessary validation and accepted byte changes. `frozen/` holds each case's extraction and Jev's recorded answers, and the replay test runs them offline. A positive no candidate reached is reported as unsupported coverage, never as a pass. `jev principles report` prints the replay and each group's recall, precision and clean-push false flags with 95% intervals; `jev principles eval` re-records the answers (live, paid, capped at 200 calls). The corpus grows from confirmed findings and dismissals, and from a small chronological sample of clean changes.
