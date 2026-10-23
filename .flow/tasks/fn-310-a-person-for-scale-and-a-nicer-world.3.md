---
satisfies: [R4, R5]
---
# fn-310-a-person-for-scale-and-a-nicer-world.3 Cost and final proof: frame time against the base, Wasm bytes, docs, final stills

## Description
R4's measurements and R5's closing proof once both halves exist: frame time against the base on the RTX 3080, the render Wasm's byte change against the budget, the docs that name the 1.8 m figure, and the final stills at the concept's poses in the evidence folder.

**Size:** S
**Files:** `README.md`, `crates/telperion-render/src/lib.rs` (doc comment), `crates/telperion-render/src/scene.rs` (module doc), `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/`
**Touches:** [README.md, crates/telperion-render/src/lib.rs, crates/telperion-render/src/scene.rs, .flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/**]

### Approach
- Frame time: `examples/frames.rs` (documented in `docs/gpu-tests.md:44-54`) for the oak at 40 and Telperion at 80, seed 7, on this branch and on the base. Build the base from `git archive 201288747` into a scratch directory with its own `CARGO_TARGET_DIR` (two checkouts never share a target, AGENTS.md). Run both through `telperion-heavy 5 --gpu 3 --threads 2`, alternating, three runs each; state medians and spread.
- Wasm size: build the render module as the budget script does and state the byte change against `scripts/artifact-budgets.json`'s `dist/telperion-render.wasm` entry.
- Docs: `README.md:89` ("1.8 m scale figure") and module and doc comments naming the capsule or 1.8 m.
- Final stills: oak at 40 and Telperion at 80, seed 7, 1280x720, kept small in the evidence folder (raw under `raw/`), with a short `stills.md` naming the commands.

### Investigation targets
**Required:**
- `docs/gpu-tests.md:44-54` - the frames probe
- `scripts/artifact-budgets.json` - the render module entry
- `docs/evidence-retention.md` - what to keep
**Optional:**
- `scripts/gpu-row.mjs` - the scorecard row wrapper

## Acceptance
- [ ] Frame time for both trees, base vs branch, recorded with medians and spread; a difference outside noise is stated with its number
- [ ] The render Wasm's byte change is stated and inside the ceiling
- [ ] README and doc comments name the 1.75 m person, not the 1.8 m figure
- [ ] Final stills at the concept's poses are in the evidence folder with their commands
- [ ] Evidence count and size checked per docs/evidence-retention.md

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
