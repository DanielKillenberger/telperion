# The design principles are checked

## Goal

STRATEGY.md's principles slipped three times in a week with nothing checking them: the slim field package kept a copied build chain (#58), the date palm shipped unbuildable through it (#115), a first fix grew its Wasm 48% and was stopped only by hand, and the species runner grew about 30 stop sites. This spec puts the principles in front of every change, tight enough to catch real breaches and stay quiet on healthy work, so nobody learns to bypass it.

Owner quotes (2026-09-25):
> "let's do it properly and make sure that we have something in place that makes sure these design philosophies really get adhered to ... It shouldn't be overbearing but agents should live and breathe this and have their decisions guided."
> "can we actually have a test suite that we run through jev to check if the diff adheres to our guiding principles." / "but why just ci? if we have it part of the local suite we don't have to open a PR first to figure out the mistake" / "we could make it a pre_commit hook?" (host: pre-push; owner: "yes")
> "so 151 really needs to be designed well. It needs to be token efficient, fast but be highly reliable in catching offending pushes ... can we run jev on specs before we start work?"
> "fn-151 should also make sure that STRATEGY.md is upheld" / "for fn-151 we should check through previous PR's identify ones that should definitely have raised flags and make sure that the hook catches it"
> On continuity: "I can imagine a multidimensional tree space where if you're on one part of the space some dimensions just don't change the output ... we need to find a principle here that holds."
> "I want an in depth review with astra on how to make sure this isn't overbearing and full of false positives. This needs to be tight." (Astra's verdict: "needs redesign before implementation"; owner on the revised outcome: "ok")

## Decisions

- After Astra's review the design blocked only on deterministic checks (production boundary, entry coverage, artifact budgets), with an advisory two-phase Jev reviewer earning blocking per principle at >= 95% measured precision; an exception registry; a pre-push hook. Owner-labelled corpus (2026-09-25): 17 positives (#6, #58, #55, #115, #3, #59, #52, #53, #61, #13, runner approval layers #38, #75, #76, #77, #83, #87, #94) and 22 clean PRs (incl. #17, #50, #82).
- The cut (owner, 2026-09-25): the Jev reviewer was built, measured and removed — guards caught every guard-owned positive (#6, #55, #58, #115); Jev caught #3 and #13, recall 0/11 in the holdout, 0/22 clean pushes flagged. Design review moves to fn-156.
- Structure over policing (owner, 2026-09-25): a principle is held by code that cannot express the breach, an ordinary test and a CI size budget — never a scanner or a hook. The boundary scanner, exception registry, policy file, `jev principles` and the pre-push hook were built then removed (git history at `061ff37d`, `132257f3`). The two sanctioned exceptions (the growth path, the GPU executor) are named in `docs/principles.md`.
- STRATEGY.md gains the owner-approved continuity sentence ("every parameter is a numeric trait defined for every tree ... a small change in any parameter makes a small change in the tree. A parameter may be dormant where the structure it shapes is absent, and it wakes smoothly ... No parameter is a switch between ways of building, and a count steps only by one unit of the structure it counts"); AGENTS.md carries the steering questions (host, 2026-09-25, after fn-154 made AGENTS.md the one instruction file).

## Acceptance Criteria

- **R1:** Production-boundary guard — moved to fn-152 (owner, 2026-09-25): "stages private to the pipeline; a second chain does not compile".
- **R2:** Entry coverage — met as an ordinary test: every catalogue preset builds every artifact through `pipeline`, beside the Wasm binding and slim entry tests; red on `39348def` for date-palm through the slim entry.
- **R3:** Artifact budget — met as CI's `scripts/artifact-budgets.mjs` against `scripts/artifact-budgets.json`.
- **R4:** Offline replay catches every labelled mechanism — moved to fn-156.
- **R5:** Live held-out Jev precision before graduation — moved to fn-156.
- **R6:** Pre-push hook within cost bounds — withdrawn with the hook.
- **R7:** Spec mode on fn-150 and a duplicate-path spec — moved to fn-156.
- **R8:** Guidance in STRATEGY.md, AGENTS.md, `docs/pr-format.md`, `docs/principles.md`; gate and `npm test` green — met (954 passed; npm 129).

## Result

Shipped structurally (commits `9734cb0c` .. `bde67062`). The reviewer's measurements are the baseline for fn-156: `.flow/evidence/fn-151-the-design-principles-are-checked/EVALUATION.md` (referenced by `docs/principles.md`).

## Open

- Design review of specs by Jev: fn-156.

## Friction

- Labelling runner positives by mechanism and span took ~25 min and ~40k tokens -> a survey column for each stop site's `path:line`.
- dcg refused redirects and `git checkout <ref> -- <path>` -> local setup, reported only.
- Jev's first confirming question barely separated breaches from clean pushes (88 calls, ~322k tokens) -> labelled phase-two cases per principle before a corpus run.

## Evidence

- `.flow/evidence/fn-151-the-design-principles-are-checked/EVALUATION.md`
