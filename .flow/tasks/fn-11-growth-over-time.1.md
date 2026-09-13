---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13]
---
# fn-11-growth-over-time.1 Implement Growth over time

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Blocker — NEEDS_HUMAN, 2026-09-12 (timebox expired)

Steps 1 to 3 of the spec's order landed across seven Codex gpt-6-astra
invocations, all five gates green at every commit: identities and the retained
frontier as generational keys, pinned pure-Rust math after the parity probe found
the shipped builder was **not** byte-identical between native and wasm, the
integer monthly clock with the Chapman-Richards budget and saturation, age and the
growth traits on the JSON wire, the frontier repairs that turned a 599-node
bare-pole spruce into a whole tree, and thickening, shoot state and vigour-based
shedding. The 240-minute timebox then expired inside the cost work, which cut a
mature oak build from 198.7 s to 18.9 s and a mature slice from 211 ms to 18.7 ms.
Three things now need the owner rather than another invocation. First, a mature
build is still 301 times the envelope build (19 s on the oak), and the spec's own
tradeoff keeps the closed-form alternative available if the per-slice cost proves
too high — that is a design call, with the measurements ready for it. Second, the
monthly mature tree carries 55% more nodes than today's oak and 17% fewer than
today's spruce, with bounds within 2% and 7%: right size and shape, wrong amount
of wood, because both presets author a zero shedding threshold and step 6's
calibration has not run, so production routing and the authorized re-pin are
deliberately unspent. Third, R11's verdicts are the owner's and no still has been
rendered. Steps 4 to 7 remain; no R8 curve was sourced and no capture was taken.
The full account, with every number and its command, is in
`.flow/evidence/fn11/STATUS.md`.

## Done summary
TBD

## Evidence
- Commits: 6b799e4, 53c5629, 2fd4b49, eb34bbb, 1545138, 794f0a7, 1647566,
  33a7271, 36bda80, f952fb4, 91611ef (the flow-prefixed ones are host commits,
  three of which swept implementer work in progress)
- Tests: cargo fmt --all -- --check; cargo clippy --workspace --all-targets --
  -D warnings; cargo test --release --workspace; npm run wasm:build && npm test;
  npm run typecheck — green at the base commit and at HEAD, verified by the host
  after each of the seven invocations
- PRs:
