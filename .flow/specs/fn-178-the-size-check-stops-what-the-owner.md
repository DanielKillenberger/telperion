## Goal

The size check should stop a change the owner would refuse and let ordinary features through without paperwork. Every artifact's budget was its npm 0.1.4 size plus 5 percent, so three intended features (fn-152, fn-61, fn-170) used the whole margin in a day and fn-170 had to raise the budget to pass, while the check's one real catch (fn-150's first slim build, +48 percent) was a single change. A budget raised by routine checks nothing; the owner asked for two limits that each mean something.

## Decisions

- owner (2026-09-27, on raising the Wasm budget for fn-170's fork rule): "but wait can we just raise the budget?"
- owner: "what's the point of the check if we just raise the budget.."
- owner, on a product ceiling plus a per-PR growth limit, with fn-170 keeping its raise: "ok"
- owner (2026-09-27): chose the "Headroom" ceilings: telperion.wasm 1.60 MB, telperion-render.wasm 2.20 MB, telperion-field.wasm 0.40 MB (kept small for the homepage), telperion.js 150 KB, field.js and voxelize.js 16 KB each.
- Host: growth share 5 percent of the base per PR, a named constant; a PR whose `## Decisions` names the artifact passes above it. The base is the newest green master run that is an ancestor of the PR head and still holds its `package` artifact; no base prints a `::warning::` and enforces ceilings alone; any other lookup failure fails the job.

## Acceptance Criteria

- **R1:** An artifact over its ceiling fails, naming the ceiling and its reason — met.
- **R2:** Growth above the share over the base fails unless declared; fn-150's slim build (+48.4%) fails, fn-170 (+1.1%) passes with no budget edit — met.
- **R3:** `docs/principles.md` item 3 and `scripts/artifact-budgets.test.mjs` describe and test the two limits; "last release plus 5 percent" gone — met.

## Result

`scripts/artifact-budgets.json` holds the ceilings with reasons and the growth share. Tests run offline on recorded sizes behind an injected reader (ceiling, fn-150, fn-170, declared growth, exactly 5% vs one byte more, base past the first page, absent base). The GitHub reader found base ed23f773 locally. Gates: workspace 1074 passed, `npm test` 129 passed. Codex SHIP after one fix round.

## Open

The base search stops at the first ancestor whose package artifact is missing (deliberate: artifacts expire by age).

## Friction

- The dcg hook blocked a Python heredoc with a JS template literal and `$VAR`-launched flowctl calls -> local setup, reported.
- Codex reviewers could not run vitest in their sandbox -> name `npx vitest run <file>` in the review prompt.
