# Growth runs on ordinary floats

## Goal

fn-245's first stage. The owner (2026-10-09): "doesn't sound very elegant to have hundreds of emulated inlined copies of 32 bit operations", then "yes" to the breakthrough and "convenve with astra about 3". Contract A (exact Q32/Q62 fixed point with emulated 64-bit products, so CPU and GPU agree to the bit) was the main cause of the `grow` shader's 13-17 s cold compile and a large share of the Wasm cost (fn-245 `PROFILE.md`). fn-245's experiment nudged every decision boundary at 1e-7 and 1e-6: 0-2 of 3.5-19M decisions flipped, height moved at most 0.1 mm. Astra's architecture review (fn-245 `ASTRA-ARCH.md` §3) agreed and named what stays exact. Release item for fn-237 (R0: performance no worse than 0.1.4). Scope: arithmetic only; growers, light and the CPU/GPU cut unchanged.

## Decisions

- Biology in f32 on both executors (f64 only for CPU preparation; the light sweep's optical depth stays f64, rounded to f32 where growth reads it, named as an exception for Astra). Identity stays exact: lineage keys, 53-bit draws compared against `ceil(p 2^53)`, ticks, indices, fixed reduction orders.
- Host (2026-10-10), decision 9 at f32 resolution: a continuity walk checks the model, not the last bit; refinement stops at 16 f32 ulps, and the shrink requirement applies only to a change above four times a measured noise floor. No f64 islands.
- Astra's first review DISAGREE (stepped reductions depended on plan order and chunks); after `2666c211` (partials and leaf deposits in stable item order) "ASTRA: AGREE".
- Codex review round cap reset under the owner's standing rule (every finding fixed, Astra agreed).

## Acceptance Criteria

- **R1:** inventory of every biological quantity on contract A and every place identity depends on it, before code — met.
- **R2:** biological state and arithmetic in f32; Q32/Q62 and wide products removed — met.
- **R3:** identity, cycle and event indices, sibling order, prefix allocation and reduction trees stay exact; no float atomics; draws compare an exact uniform; presence windows unchanged — met.
- **R4:** four checks replace byte parity (repeatability, discrete contracts, invariants, CPU-GPU agreement by lineage with measured tolerances) — met.
- **R4a:** checks scoped by executor; oak, ordinary (read light) and date palm (no regular twig stations) grow on the CPU, their GPU checks deferred to fn-256 — met.
- **R5:** cold `grow` compile and Wasm stage time before and after, stills of every preset viewed, baselines re-pinned, Astra's agreement, gate and npm test green — met.

## Result

- Repeatability: `repeat_tests::every_preset_grows_the_same_tree_again_and_on_any_cores` (8 presets, 4/4/1 cores, bit-equal); `the_build_is_the_same_in_any_order_and_chunks`; device settle in any order and batch gives one digest.
- CPU-GPU agreement (`preset_agree_tests::every_gpu_preset_grows_on_the_device_as_on_the_cpu`, seed 42): kept axes match exactly; bounds about twice the measured: tip most 8e-4 m, RMS 2.5e-5 m, radius 1e-6 m, crown 1.5e-5 m. The silver birch's 0.219 m tip gap predates the floats (same under contract A) and is held to its own bound.
- Cold `grow` compile 14.6 → 11.7 s (median); `girths` 1.4 → 0.06 s; browser-route build 19.7 → 15.1 s. Wasm core builds 13-79 % faster (spruce 9,548 → 2,009 ms; Telperion 9,016 → 3,776 ms); site field oak grow 950 → 409 ms.
- Stills against the base: RMSE 0.2-1.5 % on the natural species, ~4-6 % on Laurelin; viewed, unchanged.

## Open

- The birch's device gap (held to its own bound) predates the floats.
- The cold-compile timing test flakes under parallel load.
- Four catalogue pins are stale and untested; artifact ceilings were already crossed at the base.
- For fn-255: re-measure the light-to-subtree-total gain (a few-ulp sky change moved `q` ~300×) and the memory change that tips the shed window.

## Friction

- R5's "beside master's" compared two generators (~15 min, extra base build) -> word it "beside the base's" on F branches; a scratch location outside the tmpfs quota (proposed).
- Merging F4 left `BLOSSOM_BOUND` (owner-derived, fn-246) on its edge (~20 min) -> a bound that records its drift, or re-take it at measured plus 5 points with evidence (proposed).
