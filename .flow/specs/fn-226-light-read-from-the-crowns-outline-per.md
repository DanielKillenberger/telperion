# Light read from the crown's outline, per bud

## Goal

The lit oak built 5.6-6.2 times slower than unlit at 80 (fn-211 OAK-LIGHT.md: seed 7, 14.9 s against 2.4 s). Light earns its place: owner and host compared light on and off against the reference photographs (2026-10-08): light on is wider (25-45 % at 80), open, with a clear trunk; light off is a dense column. Owner (2026-10-08): "Couldn't we approximate effects of light in a way that's easily computed for each bud in isolation => parallelization on gpu?", "ok go on the light simplification", and "light algo should sit cleanly within the relevant gpu expansion path not adding additional passes, it should just do it in one go." Boundaries: the engine's light and the growth sharing that reads it; the oak's own crown shape is a species round.

## Decisions

- R1's measurement moved the target: the lattice itself is 11-13 % of the lit build; the cost is the per-cycle machinery light switches on (re-lay 16-27 %, allotment 16-18 %, settle 16-21 %, rough layout 9-11 %, balance 5-6 %). Host decisions (2026-10-08, owner "ok let's try it"): delete the re-lay first, measured; then make a bud's light and its share of growth local against expectations.
- R2: the re-lay's visible limb-level change at 80 accepted by the host (2026-10-08).
- R4 inventory stops S1-S8 decided by the host (2026-10-08): fn-211.4's `Aggregate` as the one fine-root expectation model; `Recorded` split; CPU only in task 3 (S8 c).
- **R5 owner verdict (2026-10-08), on the host's recommendation to close with today's model: "ok".** R3+R4 at 80 built a tree half as tall (8.2-11.9 m against 22.3-26.9 m) with a clear trunk about 2 m against 4.7 m, 44-70 % slower at 40 and not faster per phytomer at 80, though with 0 synchronisation points a cycle against 4. R5's sheet narrowed to today's model against O-WHOLE (O-BARE not cached; amended 2026-10-09: the owner, shown the sheet and told what it is, did not object).
- The carried `apical_control` walk (31.1 and 32.1 against 30 after R2): no cheap correction (a sagless re-lay restores it but keeps 70-85 % of the re-lay's cost). Host decision (2026-10-08), option 3: keep today's model, project bound stays 30, a named exception `ROUGH_LAYOUT` in `tests/walk/degree.rs` holds the two light walks at 33.

## Acceptance Criteria

- **R1:** Cost by pass and which effect comes from which pass — met (task 1).
- **R2:** The periodic whole-tree re-lay deleted, light reads the rough layout; measured and viewed on the lit oak at 40 and 80, seeds 1, 7, 42 — met (`R2.md`).
- **R3:** Each bud's light from the crown's outline (Beer-Lambert over nine sky directions through 16 cone sections), inline, both executors — built on the CPU and measured, then deleted by the R5 verdict.
- **R4:** Local sharing against outline expectations, inventory and equations written before code — built on the CPU, then deleted by the verdict.
- **R5:** Head-to-head of today's model against R3+R4 on the oak with the reference sheet and measures; the owner's verdict — met: today's model.
- **R6:** One model by verdict — met: R3+R4 deleted (`crates/` equals task 2's 3304848b); occluder follow-up recorded.

## Result

- Re-lay deleted on both executors: lit oak 13-19 % faster at 40 and 27-35 % faster at 80; clear trunk unchanged; phytomers grown 0.1-2.7 % fewer; crown within 3 % at 40 and -7.7 to +4.8 % at 80.
- After R2 the lit oak at 80 still builds about 3.7-4.5 times slower than unlit (4.65 s and 9.36 s against 1.03 s and 2.54 s).
- R5 sheet (`sheet-today-vs-refs.jpg`, host viewed): today's oak at 80 is a smaller, open, irregular crown on a short trunk against O-WHOLE's broad dense dome; a gap that predates this spec, left to the oak's own spec.
- Final gate: render 188 passed, workspace 1,027 passed, npm test 143 passed.

## Open

- The lit oak's remaining cost (allotment, settle, rough layout, balance): the owner's open item.
- A terrain or obstacle occluder per bud, for the owner's cliff scenes: named follow-up.
- The oak's crown form against O-WHOLE: the oak's own spec.

## Friction

- OAK-LIGHT.md's measuring probe sat in a gitignored `raw/` and was gone, so the definitions were re-implemented from prose -> proposed a committed measures example in `telperion-space/examples`.
- Timing under other sessions' load (10-31) made timings unusable -> proposed a shared timing lock; local setup, reported only.

## Evidence

- `.flow/evidence/fn-226-light-read-from-the-crowns-outline-per/R2.md` (the re-lay deletion, measures, the walk's cause; cited by `tests/walk/degree.rs`)
- `.flow/evidence/fn-226-light-read-from-the-crowns-outline-per/R5-R6.md` (verdict table, R6, the walk exception; cited by `tests/walk/degree.rs`)
- `.flow/evidence/fn-226-light-read-from-the-crowns-outline-per/r2-measure.rs`, `r2-run.sh` (the measuring probe)
- `.flow/evidence/fn-226-light-read-from-the-crowns-outline-per/sheet-today-vs-refs.jpg` (R5 sheet)
