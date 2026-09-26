# fn-159-harness-browser-and-tuning-read-the.1 Harness, browser and tuning read the parameter catalogue

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The harness and the browser metadata now read the parameter catalogue. `catalogue::browser()` renders `src/browser/parameters.generated.ts`, which holds the `Family` type with each row's meaning and a row table. The panel draws one control per row the build reads, shows each row's meaning and dormancy, and exposes stems and side-branch orders. Density and torsion stay as named adapters, bounded by the rows they move and carrying those rows' dormancy.

- **R2:** verified against stage 1. `dials.json` is gone, no active config embeds a dial copy, and `pilot-config.json` still replays. Windows and steps are authored per row in the catalogue rather than kept as a separate file, which is left open for the host.
- **R3:** the 36-row switch list is replaced by `tests/dormancy.rs`. It covers 11 representative rows, each with an awake control, and every artifact is compared, fields included. The test found `lobeDepth`'s "dormant at lobes zero" claim false, and the claim was withdrawn.
- **R4:** the harness rendered the date palm and the oak under `npm run dev` (2 screenshots). Evidence is in `.flow/evidence/fn-159-.../EVIDENCE.md`.

stage: impl-review - ran (codex: fan-out NEEDS_WORK, 4 findings fixed; re-review SHIP)
## Evidence
- Commits: 28312b51cfcefe4a6ee31dc04bf62bd9fada9250, 43040a7296c2a4c450cc49121e31007fa957c19c, 9201de9006f063df935a94a6c0f48e7dfb420c77, 37997abeb8c135df0f8831eba26517c989f12f7c, 4c04a743972213e2cf8e823a3803c51f28ee1de4
- Tests: cargo test --profile ci --workspace --no-fail-fast (1047 passed, 0 failed, 22 ignored), npm test (13 files, 110 tests), node scripts/artifact-budgets.mjs (all within budget)
- PRs: