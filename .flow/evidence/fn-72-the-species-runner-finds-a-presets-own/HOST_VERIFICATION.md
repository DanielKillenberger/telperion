# Host verification

2026-09-19. The host checked the implementation and regression tests against R1-R3. The default resolves beech and birch to fn-34; explicit profiles bypass discovery; oak and spruce remain on fn-9. Missing-record diagnostics name the preset and searched sets. No profile data, matching predicate, generator or renderer changed.

- `npm run wasm:build` passed in 18.85 seconds after the fresh worktree's missing artifact caused the first full Vitest run to fail.
- `npx vitest run` passed all 107 tests across eight files in 36.38 seconds. The first run passed 96 and failed 11 solely on the absent Wasm file.
- `node --check scripts/species-profiles.mjs`, `node --check tests/species.mjs` and `git diff --check` passed.
- `flowctl validate --spec fn-72 --json` passed. `flowctl show fn-72.1 --json` verified done.
- Actual headless captures remain untested. The CLI reached the expected reference IDs but the headless binary was absent. No GPU or full-forest capture ran.
- No Rust source changed; the Rust workspace suite was not run. This report claims the full JavaScript suite only.

stage: work - ran (implementation bridge model: claude-opus-5)
stage: impl-review - skipped(config: review=none; host inspected the diff)
stage: completion-review - skipped(config: review=none)
stage: plan-sync - skipped(empty: one task, no downstream tasks in fn-72)
stage: qa - skipped(config: pipeline.qa=auto: no UI-observable criteria (jev 0.03))
stage: make-pr - failed(NEEDS_HUMAN: owner friction rule paused disproportionately large mandatory PR procedure)

Route: all_done_make_pr (code).
Gates: full JavaScript suite passed; no prior green receipt reused.
Tracker sync: n/a (bridge inactive; end-of-run check returned no output).
Shipped: 0. No push, PR or merge. Spec remains open.

The host read every FRICTION.md entry. Proposed remedies for owner selection are a lightweight small-spec PR path and a note or exported native-preset inventory covering both listed and in-work presets. Local manager installation and SSH transport issues need local setup correction, not repository specs. Gate setup should build the ignored Wasm artifact before direct Vitest. No remedy spec was created. The Opus bridge's token telemetry is retained in the task receipt; it is not evidence of a cheap run.
