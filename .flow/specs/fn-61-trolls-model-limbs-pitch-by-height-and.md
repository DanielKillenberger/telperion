## Goal

The European beech grows on Troll's model: its limbs spread wider low in the crown than high and its crown edge is ragged where each limb system ends at its own reach. Telperion's scaffold built every tree with one pitch and primaries running to the shell, a symmetric fan under a smooth dome; the 2026-09-16 path review found this the first cause of the beech's bare and leaf-on mismatch. This spec adds the two scaffold rows the bare photograph needs: a lateral pitch that changes with height and a primary reach that stops short of the shell by a per-axis share. A true sympodial relay is named and left for later.

## Decisions

- user (2026-09-16): "The tree has clear regular outline/border that doesn't look natural."
- user (2026-09-16): "We still haven't achieved the reference for the bare one either. It still looks off."
- user (2026-09-16, on the review's path): "ok let's do that."
- Owner, 2026-09-27: a shortened limb system keeps its descendants within its own reach (deeper axes, twigs, leaves, curtains inside the shell scaled about the primary's station by the share it kept).
- Host decisions: the top pitch is a signed offset from `lateralPitch` (an absolute row could only be neutral as an optional row, which blends discontinuously); the grade applies at every order; specimen snapshot schema 5 (4 never reached master).
- No relay, no leaf-load bending (fn-59), no envelope change; rows, not species.

## Acceptance Criteria

- **R1:** both rows railed, neutral byte-identical on every table at two seeds and specimen views, refused by name, wire, blend, metadata, harness — met (`pitchByHeight`, `raggedReach`; generation digest identical on 8 tables x seeds 1 and 7, growth view included).
- **R2:** synthetic family: graded pitch base to top; per-axis, per-seed reach shortfall within the row; descendants of a shortened primary within its share — met (`scaffold/troll_tests.rs`, `specimen/limb_tests.rs`, red-first checked).
- **R3:** bare beech rendered once with the rows; implementer answers whether lower limbs spread wider and the edge is ragged — met: at `pitchByHeight -20, raggedReach 0.35` the lower limbs spread wider and the edge breaks in places but reads mostly as one ovoid; at `-45, 0.6` the edge breaks clearly into limb systems with gaps, the top near-columnar. The beech's values are the beech spec's.

## Result

Both rows declared once in the parameter catalogue; limb bounds in `pipeline/branching/limbs.rs`. Catalogue identity digests, dial counts, sweep held rows, limit inventory re-pinned; the beech replay tape extended live with `--extend`. Codex fan-out review NEEDS_WORK on one finding (snapshot schema), fixed in 857085fa, re-review SHIP. Workspace gate green (1062 passed), `npm test` 112. PR #130.

## Open

`scaffold.rs` is over the 400-line guideline. A relay spec if the beech's value round shows the wave cannot read as Troll's zigzag.

## Friction

- dcg hook refused a redirect with a shell-variable path (~1 min) -> an `--out <file>` flag on `generation_digest`.
- Neutrality digest is a ~3 min GPU run, twice (~7 min) -> a CPU-only digest mode for scaffold-only changes.
- Fixes committed before the fan-out finalize forced a second review (~2 min) -> worker's ordering error; none needed.
- Two new rows broke seven pins across three crates, found only by the full gate (~10 min) -> a checklist in `docs/parameters.md` of pins a new row moves; replay tape not keyed on the full dial table.
- Resumed task gated before review, then again after (~5 min) -> review first, gate last; a focused test script that runs the wasm pretest.
