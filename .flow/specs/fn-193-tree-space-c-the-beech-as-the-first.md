# Tree space C: the beech as the first point

## Goal

Phase C of the tree-space programme (`docs/tree-space.md`; owner, 2026-10-03): the first species on the engine of phases A and B, a mature open-grown European beech that reads as a beech, built as values only from sourced architecture data (Troll's model in *Fagus*). Today's beech was the baseline to beat; the owner judged it poor. Gate: the host confident beside `.flow/references/european-beech/` and Astra agreeing independently; wrong-path stop on two rejections naming the same trait after a change aimed at it.

## Decisions

- Values in `crates/telperion-space/src/beech.rs`, every value cited or marked estimated (SOURCES.md); drawn through the pipeline's own expansion by `crates/telperion-render/examples/space_beech.rs`.
- The engine gained per-PA `Form` (tropism, elevation, wander, plane, pipe girth); laterals leave from the parent's local direction; girth from the pipe model. Conversion rules (RESULT.md): wood under 0.2 mm radius is not drawn, internodes under 1 mm merge, a lateral of 0.4 to 0.7 of its parent's radius grows in as a codominant fork without a collar.
- Troll's module mechanism (relays from the upper side of the curvature zone, growth age carried across relays) built for trunk, limbs and boughs; later `form.roll`, `form.exponent`, `form.ripening`, `form.dominance` added and walked (dominance stays neutral 0 for the beech).
- The gate takes three independent Astra samples per sheet, a seed counting yes on a majority (host decision 16). Rounds 11 to 21: Astra failed every sheet, naming crown architecture and branch hierarchy; host and Astra did not converge, so the gate went to the owner.
- Owner on round 13 seed 2 (2026-10-04): "structure looks really good. BUt the texture of the branches looks very tube like to regular. But that should be out of scope for the growing part. So judgement seems good."
- Colours darkened in the preset, matched by render (owner, 2026-10-04).
- Four surviving Codex P1 relay-continuity findings moved to fn-199 (host override, 2026-10-04), which fn-198 depends on.

## Acceptance Criteria

- **R1:** Sourced architecture for the beech (Troll's model in *Fagus*), every value cited or marked estimated — met (SOURCES.md).
- **R2:** The beech as a point at several ages and mature, seeds 1 and 7, bare and in leaf, beside the references and today's beech — met (sheets in the worktree's ignored `raw/`).
- **R3:** The gate, host and Astra naming traits — met on the owner's verdict after host and Astra iterated without agreeing.
- **R4:** Time, fine wood and memory against the pinned baseline — met (below).

## Result

Owner verdict (2026-10-04): "i mean to me they look really good structurally." The beech passes with round 20's values plus a 9-cycle clear bole (round 22, `a8daf966`, `3bd91bc7`) for foliage lower on the tree. Five seeds at 80 years: 17.7 to 18.6 m tall, 17 to 23 m wide, deep rounded crowns, continuing trunk forking late; dbh 1.09 m (seed 1).

R4 at 80 years, against the pinned baseline: kept nodes 1.94M / 1.62M (baseline 213k / 215k); growth 3.7 s / 2.8 s (baseline skeleton 57 ms, 49 to 64 times slower); dress 1.8 s / 1.6 s (baseline full build 3.5 s); fine wood 23.8 / 21.3 km (baseline 46.7 / 47.2). Speed is phase F's. Workspace gate 1018 passed.

## Open

- Astra's branch-hierarchy critique (fewer, heavier limbs dividing progressively).
- The tube-like, regular branch surface (owner, round 13).
- Young ages (10 and 20 years) still read as a pole with rods.
- Cost: most grown wood is later shed; shedding before growth is phase F's lever.

## Friction

- Expansion refusals (`unsolved radii`, collapsed float32 triangles) did not name the offending node -> proposed: refusal names node index and radius/length.
- `/usr/bin/time` missing and the `dcg` guard blocking variable-path redirects and `git checkout -- <path>` -> local setup, reported only.
- Stale example binary across crates timed old values -> proposed: one example for stills and measures.

## Evidence

- `.flow/evidence/fn-193-tree-space-c-the-beech-as-the-first/SOURCES.md` (values and sources; loaded by `beech.rs` docs)
- `.flow/evidence/fn-193-tree-space-c-the-beech-as-the-first/RESULT.md` (engine and conversion decisions, final R4)
