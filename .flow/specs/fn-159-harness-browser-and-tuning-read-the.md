## Conversation Evidence

> owner (2026-09-24): "i'm noticing that most params don't do anything for the palm tree.."; the harness showed about 200 sliders, and side-branch order and stems were not reachable.
> Astra review of fn-152: generate metadata for the existing generic controls, and keep the harness's two intentional adapters (density and torsion, `harness/family.ts:73`). Migrate active tuning configs to dial ids. Keep authored tuning windows separate from generator bounds.
> owner (2026-09-26): "/flow-next:flow this to its completion in one stack with pr's for each spec"

## Goal & Context
<!-- scope: business -->

The harness, the browser metadata and the tuning loop read parameters from fn-152's catalogue instead of their own copies. The harness shows each row's meaning and where it is dormant, and exposes every row a species can move, stems and branch orders included. Third of four stacked specs. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists.** `harness/dials.tsx` keeps its own list, and `presets.generated.ts` is emitted from the wire table. `crates/telperion-jev/data/dials.json` and `dials.excluded.json` are hand-kept. Active tuning configs embed full dial copies. The dial test keeps a hand list of 36 "switch at zero" rows. [checked]
- **Harness and browser.** Sliders and browser metadata come from the catalogue. A dormant row is shown with its applicability; every catalogue row a family can move is reachable. The density and torsion adapters stay as named transforms. [paraphrase]
- **Tuning.** `dials.json` is replaced by generated dial metadata plus a small authored file of tuning windows and steps. Active configs move to dial ids and overrides; historical snapshots stay replayable. [paraphrase]
- **Dial test.** The hand list is retired. Representative dormancy claims are tested by behaviour: moving the row changes no artifact while it is dormant. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The harness's sliders and the browser metadata are generated from the catalogue, show dormancy with its reason, and expose stems and branch orders. The density and torsion adapters still work. [paraphrase]
- **R2:** `dials.json` is gone. Tuning reads generated metadata plus authored windows, the active configs use dial ids, and one historical snapshot still replays. [paraphrase]
- **R3:** The dial test's hand list is replaced by behavioural dormancy tests over representative rows, fields included. [paraphrase]
- **R4:** The workspace gate and `npm test` are green, and the harness renders the date palm and the oak. [paraphrase]

**Host decision (2026-09-26).** Tuning windows and steps stay on each catalogue row, beside but separate from the row's bounds, rather than in a separate authored file: one declaration per parameter is stage 1's design. [paraphrase]

## Boundaries
<!-- scope: business -->

- Not the continuity fixes (fn-148). No generator output change.

## Strategy Alignment

- Serves "Our approach": one continuous tree space. [strategy:Our approach]
