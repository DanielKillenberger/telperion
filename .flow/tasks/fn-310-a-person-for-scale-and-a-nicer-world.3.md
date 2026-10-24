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
Task 3 measured what the change costs against the base `201288747` and wrote the final proof. Details are in `COST.md` and `stills.md` in the spec's evidence folder.

NEEDS_HUMAN: two costs are outside noise. They are stated in `COST.md` and have not been tuned away.
- **Start time:** `fresh_ms` rose by 1.68 s on the oak at 40 (6,083 to 7,762 ms) and by 1.66 s on Telperion at 80 (6,892 to 8,548 ms). The three-run ranges do not overlap, and the cost is paid once, at renderer start. `warm_ms` and the vegetation pass's GPU p50 did not move. The cause has not been looked into. One thing is consistent with it: the new noise and glow WGSL sits in `common.wgsl`, the prelude every module includes. That is an inference.
- **Frame time:** the wall `frame_ms` rose by 0.025 to 0.054 ms (0.3 to 1.1 percent) on the whole view of both trees and the oak's mid view. That is the room pass; the other views are within noise.

- **Probe:** `examples/frames.rs` (`full whole mid close`, 1920x1080) on the oak at 40 and Telperion at 80, seed 7. Base and branch alternated three runs each through `telperion-heavy 5 --gpu 3 --threads 2`. The rows are in `raw/frames/ab.jsonl`.
- **Render Wasm:** 3,482,370 bytes on the base and 3,498,703 on the branch (+16,333, +0.47%). Both were built by `scripts/build-render.mjs`. That is inside the 3,700,000 ceiling.
- **Docs:** `README.md` and the doc comments in `lib.rs` and `scene.rs` now name the 1.75 m person. The module doc in `scene.rs` already did.
- **Stills:** the oak at 40 and Telperion at 80, seed 7, 1280x720, under the default row and under the film's sun (115 degrees, 60 degrees up). They are in `final-stills.jpg`, with the PNGs under `raw/stills/final/`. I viewed all four. The person stands outside the crown and its shadow in each.
- **Evidence:** six committed files, about 430 KB in all. Raw output is under the ignored `raw/` folder.
- **Friction:** one entry: the shared scratchpad already held a `base/` directory.
- **Feature map:** no route changed.
- **Baseline:** green via the handoff from 13328e77 (only `.flow` paths changed since).
- **Checks:** `cargo fmt -p telperion-render --check` and `cargo build --profile ci -p telperion-render` are green.

Tier: session (jev moderate 0.35) at default effort (model: claude-opus-5-5)

stage: impl-review - skipped(policy: risk - the code diff is two doc comments; the rest is README and evidence)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: ede841a72201ad517b0e224bcc692ad92d0c2755
- Tests: cargo fmt -p telperion-render --check, cargo build --profile ci -p telperion-render, frames probe x3 alternating base/branch (raw/frames/frames.sh), node scripts/build-render.mjs (base and branch)
- PRs: