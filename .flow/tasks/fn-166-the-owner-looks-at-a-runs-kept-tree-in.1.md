---
satisfies: [R1, R2, R3, R4]
---
# fn-166-the-owner-looks-at-a-runs-kept-tree-in.1 Implement The owner looks at a run's kept tree in the harness

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
`species <id> --look` writes Tune's kept tree to the ignored `harness/looks/<id>.json` with core's preset and overlaid families (`params::overlay`, the call `headless --family` makes) and prints `http://localhost:5173/?look=<id>&seed=<n>`. The harness opens `?look=<name>`, lays the overlay over the preset on its own dials, refuses to draw a family that differs from core's (the R1 code comparison, run on every load and in `harness/look.test.ts` against the real runner binary), names the run, revision, round and rounds kept, and switches between the kept tree and the preset in place, saying when the dials have moved off both. A run with no kept tree, or an overlay row the family lacks, is refused naming the file or the path, with core's replacement for a retired row (R3: `crates/telperion-jev/tests/runner_look.rs`, `harness/look.test.ts`). A plain `headless --family` overlay opens over `?species=<preset>`. The runbook and the add-species skill describe the look (R4).

Deviation from the spec's example URL: a runner look is opened by `?look=` alone, because the beech is an in-work preset the harness catalogue does not list; the look carries its preset family itself. Finding: fn-157's live beech overlay uses `stemDivergence`/`stemForkHeight`, retired on master by fn-170, so that run's look is refused on master (FRICTION.md); the fixture drops those two rows.

stage: impl-review - ran (codex fan-out NEEDS_WORK: stale shown-tree indicator and seed; fixed; re-review SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: fbc982d93559ce96ef839c85d5590a75aff408bf, 1b7095921eb69f9ce12ce225a7d02432b1f5da1e, 03af1681121d49cf4d04de95e1090380aab0237c
- Tests: npm test, cargo test --profile ci --workspace --no-fail-fast, baseline: none (spec defines no Quick commands)
- PRs: