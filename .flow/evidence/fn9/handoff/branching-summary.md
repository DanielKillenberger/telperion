WIP handoff: implemented native BranchHabit::{Colonizing, Spreading, Tiered}; natural spreading scaffolds and local crookedness, persistent tiered leader with hanging woody secondaries, and preserved clipped local lateral stations. Separated BiasParams supernatural settings behind explicit enabled flag (ordinary disabled, Two Trees explicitly enabled); Ordinary Family surface twist/deep lobing disabled while keeping natural root flare.

User stopped implementation before final gates and review. Task remains in_progress. No devserver or live shell remains; the one workspace test shell completed and its exit code was collected. No flowctl done, review, shared gate receipt or tracker mutation was performed.

## Current API and integration
- SkeletonParams.habit defaults to Colonizing. Existing public append utility retains colonizing semantics; generate passes habit to internal local planner.
- SpreadingHabit: scaffold_limbs=5, subdivisions=3, crookedness=24 degrees. Crookedness governs both structural growth units and local runs. Structural scaffold reserves twigs.reach through inner envelope before local append.
- TieredHabit: tiers=16, branches_per_tier=5, secondary_spacing=.35 metres, secondary_length=.3 of primary length, upturn=.12 of primary length. Crown tier phases/primary lengths and secondary directions vary by seed. Natural gravitropism must be set appropriately for species; test/demo uses BiasParams::NONE to prove these shapes do not need supernatural effects.
- BiasParams keeps gravitropism/lean; supernatural: SupernaturalParams { enabled, writhe_amplitude, writhe_wavelength, spiral_rate }. Disabled nonzero stored effects exactly match natural output.
- Approved mechanical consumer edits: wasm params.rs and native foundation/crown reference tests. Frozen reference effects remain explicitly enabled. Wasm currently retains flat effect wire names with nested native accesses and adds explicit bias.supernaturalEnabled; no enable inference. Task7 must finish wire grouping, habit exposure and generated browser metadata. Browser metadata intentionally not regenerated.

## Checks
Baseline green: 19 growth/foliage/colonization tests plus npm typecheck. Focused regressions observed red before green (API absence, lost clipped-run laterals, absent local crookedness). Workspace suite passed before final local bend/minimum leader segment edits, including native foundation consumers and Wasm compilation; final code passed all 13 growth tests. Exact commands and log locations are in evidence. Final gate classification/full verification and final format check remain unrun because user explicitly requested immediate stop. No review verdict is claimed.

## Diagnostic visuals and calibration starting points
Scratch files in workspace .flow/tmp/: habits.rs (native exporter), habits binary, habits.txt (edge positions/radii), render.py, oak.svg/png, spruce.svg/png. These are simple orthographic edge plots, NOT neutral mesh/foliage QA or surface-junction validation. Seed42, default RadiusParams/TwigParams, BiasParams::NONE. Oak envelope: height20, crown_base.2, spread.55, fullness.55, shoulder2.2. Spruce: height16, crown_base.12, spread.3, fullness.18, shoulder1.2. Latest oak: 2003 nodes, 292 structural; spruce: 26612 nodes, 2281 structural; both complete.
Inspected oak plot has substantial crooked low scaffolds and curved subdivided local runs with broad irregular windows; still sparse compared with O-BARE and requires species calibration. Spruce plot shows continuous leader, irregular tier phases and hanging woody secondaries, but dense overlapping fine branches; secondary length/spacing and twig controls need calibration against actual attached foliage views. No species-fidelity pass.

## Remaining work / limitations
- Finish task3 final gates and inspect diff after stop; then native species templates/calibration remain tasks5/6, final Wasm/browser grouping task7 and visual protocol task9.
- New habit tests cover deterministic solved topology, crown edge samples, varying seeds, caps0/1/20, invalid controls, leader/hanging axes, substantial repeated oak forks, local crookedness, and disabled effects. Full fixed/fresh seed protocol is not done.
- Structural edge containment currently checks eight samples per edge; local clipped-run resampling retains existing bend vertices to avoid shortcutting old curves. Exact continuous containment across arbitrary pathological envelope shapes is not proved.
- Habit scaffold limits are validated; shared node cap diagnostics survive. Extremely tiny positive distances and unusual envelopes deserve further boundary review. Species architecture still needs inspected mesh junction/tip and foliage attachment views; no surface-stage fix is claimed.
- No per-species benchmark or biological pass is claimed. Existing Ordinary geometry counts intentionally change after natural defaults and retained clipped laterals; final browser fixture updates belong downstream.

stage: impl-review - skipped(policy: parallel-wave conductor owns review; REVIEW_MODE=none)
