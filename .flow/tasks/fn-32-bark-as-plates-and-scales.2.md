---
satisfies: [R1, R4, R5, R6]
---
# fn-32-bark-as-plates-and-scales.2 Organicness harness and hill climb to the owner's acceptance

## Description
Round one's stills were rejected by the owner: "i see some issues in how
organic it looks. Looks like armor plating in some cases. Spruce is much
better. But also doesn't look too organic." Acceptance is to be reached inside
this spec by a measured hill climb - numbers judge every iteration, no model
runs inside the loop, and the host views at most four images at the end.

## Acceptance
A structure score against the reference photographs separates bark from brickwork; the preset rows are hill-climbed against it with no model in the loop; the primitive changes only when the row plateau is far from the reference; the owner judges four images at most per round and records the verdict in the spec.

## Done summary
Not done: R6 is the owner's judgment and R7's bound is the owner's call.

A six-component structure score ships as `crates/telperion-render/src/structure.rs`
with `examples/bark_score.rs` and its own guard test, which separates a lattice
of identical cells from a jittered cellular network on every component. On round
one's stills it reproduced the owner's ranking without being told it.

Coordinate descent over the fifteen bark rows against that score, in one process
that renders, measures and guards each candidate itself: 1,510 trials, zero
tokens per iteration, no image inspected until the end. Round one's rows-only
plateau left the oak at 64% of its starting distance, past the task's test of
one half, so the primitive changed once: the column-and-cut network became a
cellular partition of the surface - sites scattered in the space the bark passes
through, three boundaries meeting at a point, a size per site, two filtered
warps, and the same seamless wrap. The oak trunk falls 1.9346 to 1.2284 and the
spruce trunk 0.7764 to 0.1658, the spruce matching its reference on every
structure component.

Native total 5.2129 ms p50 and 5.6993 p95, against round one's 4.5192 / 4.9339
and fn-29's accepted 3.9823 / 4.5814. Every bound holds unwidened and the oak
grazing pose has 1% of its aliasing bound left. The remaining gap the numbers
name is the oak's furrow darkness, whose rows belong to fn-29 and which this
spec's boundary forbids redoing - a further spec owns it.

Evidence: `.flow/evidence/fn32/REPORT.md` (second half), `hillclimb.json`,
`stills.json`, `stills/`.

NEEDS_HUMAN: R6 owner judgment on .flow/evidence/fn32/stills (round two); R7 bound is the owner's call, measured 5.2129 ms

## Evidence
- Commits: bd2f193, 438aac2, cb86074, 2a23a59
- Tests: cargo fmt --all -- --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --release --workspace; npm run wasm:build; npm test; npm run typecheck - all exit 0, 53 test binaries ok, zero adapter skips
- PRs:
