# Trunks without weird dents

## Goal

The owner, looking at the release branch's trees in the harness (2026-10-09): "Most trunks have some weird dents in them, that look weird." Find which part of the wood surface makes them and fix it. Host (2026-10-09): release blocker for fn-237. Scope: the trunk and limb surface only.

## Decisions

- R1 cause (measured, not a values fix): the dents were already in the converted tree's node radii. Girth (`relay_parts`) entered a relay's pipe split between its bud's node and the node below by `t = relay_span(...)`, while the drawing (`converted`, `Origin::Relay`) hung the relay from a joint above the bud and patched the stem without the relay's wood: a constriction, then a collar (beech collars 15-25 %, ordinary waists to a quarter radius, oak hairline seams). `relay_at` is botany and was not moved to hide it.
- Host decisions one to three (2026-10-09) were tried and superseded: drawing from girth's node only moved the dent; changing girth broke fn-199 R4 (the owner's by-degree rule, not retired by an agent).
- Host decision four (2026-10-09, with Astra's design review): one resolved attachment in `telperion-space` (carrier span plus fraction from `relay_at`, unit extent and `b`) read by geometry, girth, sag and the drawing; girth and sag project the relay's whole section and load onto the two neighbouring samples; the drawing inserts one ordinary JOIN node (new lineage registry key, mirrored in WGSL) at the attachment; the device mirrors the rule.
- Host answers to the pre-build read (2026-10-09): clamp a relay's attachment to its parent's own units; oak and birch may change under the generator-evolution policy on viewed sheets and listed `space_hash` changes, spruce and palm stay identical; closed-form predictions stay as they are.

## Acceptance Criteria

- **R1:** trunk close-ups of every shipped species at 40 and 80 (seeds 1, 7, 42) viewed, the dents located and the stage and rows that make them named by switching candidates off, against reference photographs; construction fixes designed by the host — met.
- **R2:** trunks read smooth or as their species' real bark form; every parameter still changes the tree by degree; CPU and device agree within fn-125's screen — met.
- **R3:** same stills before and after, viewed; the owner's verdict; gate and npm test green — met.

## Result

- Beech collars, ordinary waists and terrace gone with no flange; palm and spruce byte-identical; beech, oak, birch, ordinary, Laurelin and Telperion `space_hash` changed (beech +108 to +1,781 nodes, the join nodes).
- fn-199 R3 and R4 pass unchanged; new tests `relay_tests::a_stem_never_thickens_on_its_way_up`, `girth::tests::a_relays_wood_enters_where_it_joins_and_its_stub_carries_its_own`, `a_node_growing_in_moves_no_relays_attachment`. CPU and device agree (realized, dress, route, resident tests green). Sag moves at most 0.4°. Closed-form predicted-versus-realized drift unchanged.
- Gate per package: core 454, space 222, wasm 6, field 7, jev 389, render 222 passed; npm test 154. Impl-review SHIP after two fixes.
- Owner verdict on the beech trunk sheet (2026-10-09): "much better nice".

## Open

- Review P2 declined: `grow_bounded` carries the realized girth's joins after its own fit; it has no consumer that draws a tree, so a follow-up for when one does.

## Friction

- The still runner cannot switch a row off -> candidates switched by hand; no remedy named.
- The command hook blocks loop redirects, heredocs and `git checkout --` -> local setup, reported, not specced.
- Decisions assumed a single join point the girth model did not have, and an owner requirement (`tests/relays.rs`) surfaced only at the gate -> host now asks for a 10-minute "where would this land" read of every named file and a test search before each decision (carried to fn-237's friction review).
- A device layout change has no single place to update -> one generated constants block shared by the shaders (proposed).
- A wall-clock bound fails under other sessions' load -> read the load and report "inconclusive", or run under the GPU lock (proposed).

## Evidence

- `.flow/evidence/fn-244-trunks-without-weird-dents/DENTS.md` (cause, the four host decisions and proof; cited by `relay_tests.rs` and `stand.rs`)
- `.flow/evidence/fn-244-trunks-without-weird-dents/ASTRA-JOIN.md` (Astra's join-node design review)
