## Goal

The harness, the browser metadata and the tuning loop read parameters from fn-152's catalogue instead of their own copies; the harness shows each row's meaning and where it is dormant, and exposes every row a species can move, stems and branch orders included. Third of four stacked specs; no generator output change.

## Decisions

- owner (2026-09-24): "i'm noticing that most params don't do anything for the palm tree.."; the harness showed about 200 sliders, and side-branch order and stems were not reachable.
- Astra review of fn-152: generate metadata for the existing generic controls, keep the harness's two intentional adapters (density and torsion), migrate active tuning configs to dial ids, keep authored tuning windows separate from generator bounds.
- owner (2026-09-26): "/flow-next:flow this to its completion in one stack with pr's for each spec"
- Tuning windows and steps are authored per row in the catalogue (`tuned(..., window, steps, basis)`), not in a separate file.

## Acceptance Criteria

- **R1:** Sliders and browser metadata generated from the catalogue, dormancy shown, stems and branch orders exposed, adapters working — met (`src/browser/parameters.generated.ts`; 238 controls on the palm, 120 with a dormancy note; growth rows only under `?growth=1`).
- **R2:** `dials.json` gone; tuning reads generated metadata; active configs use dial ids; a historical snapshot replays — met (`pilot-config.json` replays on its own copy).
- **R3:** The 36-row hand list replaced by behavioural dormancy tests — met (`crates/telperion-core/tests/dormancy.rs`, 11 rows, each also checked awake).
- **R4:** Gates green; the harness renders the date palm and the oak — met.

## Result

- Dormancy finding: `/surface/lobeDepth` was not dormant at `lobes` zero (a junction's ring divides by `1 + lobeDepth`); the claim was withdrawn in the catalogue, generator unchanged.
- Harness check (`npm run dev`, 2 screenshots): palm seed 1 at 32,320 wood tris and 6,554 foliage instances; stem divergence dormant at one stem; three stems gave 96,560 / 19,662 and reverted exactly. Oak seed 1: 7,227,360 / 715,065; lateral orders 1 gave 125,160 / 7,800 and reverted. No console error. Merged in PR #127.

## Open

None.

## Friction

- Two "awake" dormancy controls picked values the row already acts like -> proposal: a catalogue row states its inert value.
- Fan-out finalize rejected a review confidence of 90 with a generic parse error -> a parse error naming the field proposed (flow-next).
