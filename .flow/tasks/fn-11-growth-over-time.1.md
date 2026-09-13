---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R9, R10, R12, R13]
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
Growth over time is built as the machine: a deterministic specimen with an age, the chronicle (append-only lifetime record, every reader a filter over stamps), yearly slices chosen by the number, thickening by keyframes under a resize tolerance, shedding by vigour as death stamps, foliage by living shoot and cohort (ratified amendment, R12), the wasm and native specimen handles, the snapshot, and a harness page where a tree grows live. Costs measured and recorded in .flow/evidence/fn11/REPORT.md: sparse oak advance 13.4 ms, record plus read 6.6 ms, mature oak build 1,273.5 ms, snapshot round trip 320.7 ms at 121.9 MB. Calibration, routing, the re-pin, the age strips and the mature stills moved to fn-30 (R8, R11 and R3's validation half marked moved). 36 commits on fn-11-growth-over-time; all five gates green at HEAD; native-to-wasm parity on every preset.
## Evidence
- Commits: 972bace, 2fd2668, d1aa548, e3e3e0d, 28b4295, bff3de7, 20fa5ec, a6e7ba2, 9b35f72
- Tests: cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, cargo test --release --workspace, npm run wasm:build && npm test, npm run typecheck, harness/parity.test.ts
- PRs: