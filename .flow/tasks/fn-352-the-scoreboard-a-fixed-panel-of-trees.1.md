# fn-352-the-scoreboard-a-fixed-panel-of-trees.1 Build the scoreboard and run it on the 11 Nov build

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The scoreboard is built and has its first run. `scoreboard/run.sh <label>` draws and measures the fixed panel through `telperion-heavy`, and `board.py compare` prints every measure that moved by more than 5%. The first run, `base-2026-11-11`, is at `c187c68cd`: 162 trees, none refused; 26 sheets; native, thousand-oak and Wasm costs; and 0.1.4's Wasm times and whole-view stills beside it. `SCOREBOARD.md` holds the tables and the builder's plain read per tree, and `R5-REFERENCES.md` lists the repository's reference photographs.

- **Measurement probe.** The probe is a crate outside the workspace, under this spec's evidence. No landing crate or test changed.
- **Reproduction check (measured).** The oak at 40 and the beech at 40 on seed 1001 match fn-345's logs exactly.
- **Times taken under load.** Another session's gate ran in the heavy queue during the cost and Wasm stages (load 36 to 48). I briefly overlapped two Wasm timings on one core; the 0.1.4 run was rerun alone (FRICTION.md).
- **Not done.** 0.1.4's oak was drawn only at its own preset age, not at 40 to 300.

Tier: an instrument and a measurement; no product code lands, so no gate and no codex review (model: claude-opus-5-5)

stage: impl-review - skipped(config: REVIEW_MODE=none)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: cd000a66fb0c2d651890c3df42ec98ae1b3ba76d, fc2533a60d6480d5c005d523f59b7dbe2d4ac1c1, 8a3a20b74056d4878cb5072f7b157b2395d20937, c187c68cd4e4278269e69837507c2e8d16776e0f, 3de1eda55e928eaf87a8b2d432999561129f796f, bb3e2a11f17ec076ca89137218bbdb90edb119db, bd1bbac50aa9d2aa979b3f62a1268941668803e1, 8669039db35136b080e21b156279b305778cd649, b866be2e618cc09a6fb6815d59753857f388b2be, b67eccb7a2d1666d2593baf2f27d8ab26cd70c71, bb504e385d660420f3391aaf89ff3aa01a8fd6e2, 34999005051c5c86a547959584bea4067ca783e8, b6492c7628aeda860aba0c282980f03cf5eada8d, d0357c18848ae62a719995cad345921feacdc0fe, 2e9ac9ff4ca794be402854d0700b9bd40d963f6f, fb6ce7e303687bb453d42a50f3a5f167e50920eb, 42befdaa84c31c94179b1541f699b99ba654fef3
- Tests: bash .flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/scoreboard/run.sh base-2026-11-11, python3 -I .flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/scoreboard/board.py compare (base vs itself: 0 of 178 moved)
- PRs: