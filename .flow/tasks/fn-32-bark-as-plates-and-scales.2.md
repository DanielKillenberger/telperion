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

### Round three (after the owner's "it's much much better. but still has much room for improvement")

`plateFurrowWidth` joins the family, inert by default: the flat floor of a
furrow, up to a fifth of a plate's own width, with the far path's mean falling
exponentially with it. The climb gains two constraints rather than two terms -
the crop's mean must stay inside a stated band around the reference's own and
may never walk away from it, and the dark fraction may not fall - and a level
pass that walks every row towards the reference's level while the score stays
within two per cent of its plateau. The three plate means were re-measured
over sixteen slices of the field instead of one.

The oak reaches 1.3293 and the spruce 0.1870, against 1.2284 and 0.1658. The
loss is the corrected pins, not the rows: the same rows measure 1.5231 under
them, and the round bought 0.194 of that 0.295 back. Two bounds were measured
rather than argued: with every row this spec owns at its darkest the oak's
crop falls only to 140,144,133 against a reference of 118,119,114, so the
level belongs to fn-29's colour rows; and with every term of this spec at zero
the oak still measures a dark fraction of 0.127 of its 0.147, because on a lit
cylinder the threshold cuts the trunk's shading rather than its furrows. The
furrow row ships at zero - every width costs score on both species and the two
images spent agree, a wide floor over a four-millimetre relief reading as
flakes rather than valleys - and the widest guard-holding setting is kept
beside the eight as `stills/oak-trunk-wide-furrow.png`. Native total 5.2142 ms
p50, unchanged.

NEEDS_HUMAN: R6 owner judgment on .flow/evidence/fn32/stills (round three); R7 bound is the owner's call, measured 5.2142 ms

## Evidence
- Commits: bd2f193, 438aac2, cb86074, 2a23a59, 526da27, 5f75ec1
- Tests: cargo fmt --all -- --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --release --workspace; npm run wasm:build; npm test; npm run typecheck - all exit 0, 53 test binaries ok, zero adapter skips
- PRs:
