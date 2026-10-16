## Goal

A species run stops for one person: the owner's look at the tree Tune kept, which decides whether `species <id> --accept` writes it into the preset. The runbook said the look happens in the harness, but the harness showed only shipped presets, so on fn-157's beech the owner could only see stills rendered by hand. The owner should orbit the kept tree in the harness, switch views and seeds, and compare it with the shipped preset before deciding.

## Decisions

- owner (2026-09-26), on the host's proposal that the harness shows a run's tuned tree before acceptance: "ok then go ahead with your 1-4"
- The look is read-only in the harness; accepting stays `species <id> --accept`.
- A runner look opens by `?look=<id>` alone (a deviation from the spec's `?species=<id>&look=<id>`), because an in-work preset like the beech is not in the harness catalogue; the look carries its preset family itself.

## Acceptance Criteria

- **R1:** `species <id> --look` prints a URL opening the kept tree, equal to `headless --family` over the same overlay — met (the harness refuses a family that differs from core's, checked on every load and in `harness/look.test.ts` against the real runner binary).
- **R2:** The harness names run and revision and switches between the look and the shipped preset without reloading — met.
- **R3:** A look with no kept tree, or an overlay path the family lacks, is refused naming the path — met (`crates/telperion-jev/tests/runner_look.rs`, `harness/look.test.ts`).
- **R4:** Runbook and add-species skill describe the look; gates green — met.

## Result

`species <id> --look` writes the kept tree to the ignored `harness/looks/<id>.json` (core's preset and overlaid families via `params::overlay`) and prints `http://localhost:5173/?look=<id>&seed=<n>`. The harness lays the overlay on its own dials, shows run, revision, round and rounds kept, and says when the dials have moved off both trees. Finding: fn-157's live beech overlay uses `stemDivergence`/`stemForkHeight`, retired by fn-170, so that run's look is refused on master; the test fixture drops those two rows.

## Open

Mapping retired rows onto their replacements (the host's decision, not built); the beech's next Tune revision on master (fn-62).

## Friction

- The only real run's overlay used retired rows and could not be looked at -> the fixture drops them; mapping left to the host.
- Vite's SPA fallback served a missing look as 200 text/html -> the fetch treats non-JSON as missing and names the file.
- The R1 test builds `species` in release, about 80 s cold -> accepted.
