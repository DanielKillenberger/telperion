---
satisfies: [R1, R2, R3]
---
# fn-72-the-species-runner-finds-a-presets-own.1 Implement preset reference-profile resolution

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The species runner resolves beech and birch to fn-34 profiles, keeps oak and spruce on fn-9, and honors explicit --profiles before discovery. Coverage includes every native listed/in-work preset, missing profile sets, missing-record CLI diagnostics, and overrides when defaults are unavailable.

Tier: session (jev moderate 0.88) (explicit IMPLEMENTER preserved); bridge execution metadata: claude-opus-5.
stage: implement - ran (model: opus at high; delegated: 0)
stage: impl-review - skipped(config: REVIEW_MODE=none)

baseline: none (parent spec defines no Quick commands). Bridge additionally reported 13 existing script tests and catalogue checks green before edits; new resolver suite initially failed for missing module. Worker reproduced the missing searched-set diagnostic (1 failed, 8 passed), fixed it, then observed 22 script tests passing. Catalogue check: 5 species pass. git diff --check passes. gate classify: FULL for .mjs; no full-suite Quick commands are specified. Host owns final repository gates.

Bridge usage from modelUsage: input 102; output 33,554; thinking 18,360; cache-read 4,251,969; cache-creation 99,203 tokens. This was about eight minutes of bridge execution. No claim of low cost is made.

R1: resolver and CLI diagnostic tests; beech/birch reference records found by bridge CLI checks. R2: unchanged fn-9 mapping and records tests; CLI override checks include unavailable defaults. R3: all listed and in-work native presets covered.

Actual quick rendering was not validated: target/release/examples/headless is unbuilt in this worktree. The bridge reached the correct B-WHOLE/B-BARE/B-BASE and S-WHOLE/S-BARE/S-BARK reference records; captures fail at the missing binary. No GPU captures ran. Profile contents and record-match semantics are unchanged.

Bridge command: claude -p <pointer prompt> --model opus --effort high --permission-mode acceptEdits --output-format json --allowedTools Read,Edit,Write,Bash,Glob,Grep,Agent. Digest and modelUsage: .flow/tmp/fn72-bridge-result.json. Host-reported SSH/PR-workflow friction and bridge-reported split preset catalogue friction are recorded in .flow/evidence/fn-72-the-species-runner-finds-a-presets-own/FRICTION.md.
## Evidence
- Commits: 3d84fd648717e10a50c8b35c932b598fdd3217be, e48cdf38134f33f545ba09ba1215bf0917d52072, 73b084db90d4b7dd899cb9bafd22cf8ef3009da4
- Tests: baseline: none (no Quick commands defined), npx vitest run scripts/species-profiles.test.mjs: red before CLI diagnostic fix, 1 failed and 8 passed (.flow/tmp/fn72-gap-red.log), npx vitest run scripts/: PASS 22 tests (.flow/tmp/fn72-verify.log), npm run catalogue:check: PASS 5 species, git diff --check: PASS, flowctl gate classify --base 3345b07fd48616fe278eb58971458ff85407b2b9: FULL, node tests/species.mjs --quick european-beech / silver-birch: correct reference discovery, rendering INCONCLUSIVE (headless binary unbuilt); bridge digest .flow/tmp/fn72-bridge-result.json
- PRs: