## Conversation Evidence

> owner (2026-09-26), on the host's proposal that the harness shows a run's tuned tree before acceptance: "ok then go ahead with your 1-4"

## Goal & Context
<!-- scope: business -->

A species run stops for one person: the owner's look at the tree Tune kept, which decides whether `species <id> --accept` writes it into the preset. The runbook says the look happens in the harness, but the harness shows only shipped presets, so on the beech's first run (fn-157, 2026-09-26) the owner could only see stills rendered by hand. The owner looks at a run's kept tree in the harness, orbiting it, switching views and seeds, and comparing it with the shipped preset, before deciding. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-26 on master (`849fa4b1`) and the fn-157 branch (`a2f90467`); rechecked 2026-09-27 on master (`harness/GrowerDev.tsx:100-101`).** [checked]
- The harness reads `?species=<id>` and `?seed=<n>` and loads the preset through `presetToParams(presetById(id))` (`harness/GrowerDev.tsx:97-107`); nothing else in the URL changes the family.
- Tune's kept tree is an overlay of family overrides at `<run-dir>/runner/tuning/result.json`, `outcome.current.overrides`; `headless --family <file>` renders a preset with such an overlay applied, which is how the beech's stills were made.
- `docs/species-runner.md` says the owner looks "in the harness".

**Shape.** [inferred]
- `species <id> --look` writes the kept overlay where the dev server serves it (an ignored path) and prints the harness URL that opens it, for example `?species=<id>&look=<id>`.
- The harness applies a named look's overrides on top of the preset through the same path its dials use, shows that a look is loaded (the run, the revision, the rounds kept), and offers the shipped preset beside it or on a toggle.
- The look is read-only in the harness: accepting stays `species <id> --accept`.

**Unknown.** [unknown]
- Whether the overlay's paths map one to one onto the harness's family (the runner's dial paths against `family.ts`), which the implementer checks first.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** After a run reaches the owner's look, `species <id> --look` prints a URL that opens the harness on the kept tree, and the tree drawn equals `headless --family` over the same overlay at the same seed and view. [inferred]
- **R2:** The harness shows which run and revision the look comes from, and switches between the look and the shipped preset without reloading. [inferred]
- **R3:** A look with no kept tree, or an overlay path the family does not have, is refused with the path named. [inferred]
- **R4:** `docs/species-runner.md` and the add-species skill describe the look; the workspace gate and `npm test` are green. [inferred]

## Boundaries
<!-- scope: business -->

- Not acceptance (still `--accept`), not editing the look in the harness, not fn-152's preset value files. Builds on fn-157's runner changes.

## Strategy Alignment

- Serves "The catalogue": the owner judges the tree a run made, in the tool every verdict is taken in. [strategy:The catalogue]
