## Conversation Evidence

> owner (2026-09-27, on raising the Wasm budget for fn-170's fork rule): "but wait can we just raise the budget?"
> owner: "what's the point of the check if we just raise the budget.."
> owner, on the host's proposal of a product ceiling plus a per-PR growth limit, with fn-170 keeping its raise: "ok"
> owner (2026-09-27, refining): chose the "Headroom" ceilings: telperion.wasm 1.60 MB, telperion-render.wasm 2.20 MB, telperion-field.wasm 0.40 MB (kept small for the homepage), telperion.js 150 KB, field.js and voxelize.js 16 KB each

## Goal & Context
<!-- scope: business -->

The size check should stop a change the owner would refuse and let ordinary features through without paperwork. Today every shipped artifact's budget is its npm 0.1.4 size plus 5 percent, so three intended features (fn-152, fn-61, fn-170) used the whole margin in a day and fn-170 had to raise the budget file to pass, while the check's real catch (fn-150's first slim build at +48 percent) was a single change. A budget that is raised by routine checks nothing. This spec replaces it with two limits that each mean something: a ceiling set from what the product's consumers need, and a limit on how much one PR may grow an artifact. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-27 on the fn-170 branch (PR #131).** `scripts/artifact-budgets.json` holds each shipped artifact's `maxBytes`, `measuredBytes`, `measuredOn` and a `decisions` note; `scripts/artifact-budgets.mjs` (`check`, line 15) fails an artifact over `maxBytes`, and CI's package job runs it on the package it built (`.github/workflows/tests.yml:295`). `docs/principles.md` item 3 says a budget rises only in the PR that needs it, with the measurement and a Decisions line. `scripts/artifact-budgets.test.mjs` checks that 0.1.4 passes and fn-150's rejected slim build fails. [checked]

**Shape.** [inferred]
- **Ceiling.** Each shipped artifact has a ceiling the owner set from the product's needs (below). Crossing it fails and is an owner decision, never a routine raise.
- **Per-PR growth.** The package job also measures the PR's base (the merge base on master) and fails when an artifact grows by more than a stated share in one PR, unless the PR's Decisions section names the growth; the share is a named constant with its reason.
- **One file.** `artifact-budgets.json` keeps the ceilings and the growth share; the per-release `measuredBytes` becomes a recorded baseline, not a limit.

**Settled (2026-09-27).**
- **Ceilings (owner):** `dist/telperion.wasm` 1,600,000 bytes; `dist/telperion-render.wasm` 2,200,000; `dist/telperion-field.wasm` 400,000 (the homepage loads only the slim field module); `dist/telperion.js` 150,000; `dist/field.js` and `dist/voxelize.js` 16,000 each. [user]
- **Growth share (host):** 5 percent of the base size per PR, a named constant; a PR whose Decisions section names the growth passes above it. [inferred]
- **The base's sizes (host):** every package run already uploads the packed tarball as the `package` artifact (`.github/workflows/tests.yml:314`). The check reads the newest green master run whose package artifact exists and whose commit is an ancestor of the PR's head, and measures its `dist` entries; no second build. When no such artifact exists (retention expired), the job prints a visible warning that the growth limit was not applied and still enforces the ceilings: a logged skip, never a silent pass. [checked]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** An artifact over its ceiling fails the package job, naming the ceiling and its reason. [inferred]
- **R2:** A PR that grows an artifact by more than the growth share over its base fails, naming the base and the growth, unless the PR declares it; fn-150's rejected slim build (+48 percent over its base) fails and fn-170's fork rule (+1.1 percent) passes with no edit to the budget file. [inferred]
- **R3:** `docs/principles.md` item 3 and `scripts/artifact-budgets.test.mjs` describe and test the two limits; the old "last release plus 5 percent" budgets are gone. [inferred]

## Boundaries
<!-- scope: business -->

- Not a size reduction of any artifact. Not timing or memory budgets, which stay per spec on named hardware.

## Strategy Alignment

- Serves the design principles' third holding structure (the size budget) in AGENTS.md: each cost measured, each new stop one that catches what its neighbours cannot. [strategy:Our approach]
