The scoreboard is built and has its first run. `scoreboard/run.sh <label>` draws and measures the fixed panel through `telperion-heavy`, and `board.py compare` prints every measure that moved by more than 5%. The first run, `base-2026-11-11`, is at `c187c68cd`: 162 trees, none refused; 26 sheets; native, thousand-oak and Wasm costs; and 0.1.4's Wasm times and whole-view stills beside it. `SCOREBOARD.md` holds the tables and the builder's plain read per tree, and `R5-REFERENCES.md` lists the repository's reference photographs.

- **Measurement probe.** The probe is a crate outside the workspace, under this spec's evidence. No landing crate or test changed.
- **Reproduction check (measured).** The oak at 40 and the beech at 40 on seed 1001 match fn-345's logs exactly.
- **Times taken under load.** Another session's gate ran in the heavy queue during the cost and Wasm stages (load 36 to 48). I briefly overlapped two Wasm timings on one core; the 0.1.4 run was rerun alone (FRICTION.md).
- **Not done.** 0.1.4's oak was drawn only at its own preset age, not at 40 to 300.

Tier: an instrument and a measurement; no product code lands, so no gate and no codex review (model: claude-opus-5-5)

stage: impl-review - skipped(config: REVIEW_MODE=none)
