# Growth in half the time

## Goal

After fn-91 the browser drew a mature tree through `setTreeGpu` in 158-181 ms (oak) and 164-179 ms (spruce), and the largest remaining part was CPU growth (browser skeleton medians 73.3/81.7 ms oak, 57.9/53.4 ms spruce). Growth also bounds the slim homepage package and the CPU build. fn-91's profile left about 28 ms of the oak's local advance unattributed. This spec profiles growth first, then fixes what it shows, targeting half the base skeleton time.

Owner quotes:
> "In the end this needs to be a high performing pipeline at every stage of it to be able to achieve the performance goals we have."
> "Faster growth matters for everything. We need to get these generation speeds down that is our mission."
> "ok go"

## Decisions

- Profile first, then ranked candidates; byte-identical preferred, byte changes allowed under the 2026-09-20 policy with evidence. Parallel planning only native, same output serially in Wasm.
- Readiness (host, 2026-09-24): `Planner::run` is pure, but `Frontier::advance` is sequential (queue order, birth-order ids, shared budget), so runs could only be planned speculatively; worth at most ~20 ms native.
- R2 restated by the owner (2026-09-24) after the half-time gate was missed: the byte-identical candidates ship at their measured gain, and the remaining path — a cheaper crown-envelope formula that changes output — is its own spec (fn-143).

## Acceptance Criteria

- **R1:** A profile attributes at least 90% of oak and spruce skeleton time, native and browser — met (97-99% native, 98-99% browser).
- **R2:** Browser skeleton medians recorded base vs candidate, target half — missed; restated by the owner to ship the measured gain (oak 69.7 -> 61.8 and 76.5 -> 65.3 ms, spruce 55.2 -> 41.1 and 51.6 -> 40.3 ms; completed frames 4.5-8% faster).
- **R3:** Each candidate states byte identity — met: all byte-identical (node-record hashes, stage hashes, Wasm array SHA-256s).
- **R4:** Native build time and peak memory recorded, no stage >3% slower — met (growth -5 to -10%; one +3.06% rings reading on spruce 7 is run-to-run noise, +1.55% on a serial rerun; RSS unchanged).
- **R5:** Wasm SIMD measured once and kept or dropped — dropped (within ±0.5 ms skeleton).

## Result

Three byte-identical fixes (`059f844d`, review fix `4e9d30bf`): a hardware-sqrt port of libm's `hypot` (libm's `arch` feature was off, so every hypot used software sqrt; tested bit for bit on 250,000 inputs); no bearing computed for an outline without lobes; planner buffers sized once. The cull got 4-15% faster from hypot.

Where the oak's time goes (base, browser ms): local-advance loop body 22.4, envelope admission in the planner 15.8, admission outside the planner 10.6, planner rest 8.7, heading 6.4, scaffold 5.1. `libm::pow` in `Envelope::radius_at` is 22 of the candidate oak's 65 ms. Spruce: no item above 4 ms remains besides scaffold (7.6), seeding (7.4) and the loop body (8.2). Ranked remainder: prepared envelope bounds (fn-91's rejected candidate, projected oak ~45-53 ms); spruce gains need ~13 ms in 1-3 ms pieces; byte-changing options (cheaper envelope formula, coarser clip bisection, reserved node capacity) need the owner's word -> fn-143.

Codex review: pass 1 NEEDS_WORK (an up-front reservation could panic on a valid 100M-internode run; weak NaN check in the hypot test), fixed; pass 2 SHIP.

## Open

- Half-time growth continues in fn-143.

## Friction

- No native sampling profiler (`perf_event_paranoid` 2, no `perf`) -> scratch SIGPROF sampler; local setup.
- R4's paired spruce stage comparison took ~25 min -> a stage selector on `generation_stages` or a documented noise floor would remove it.

## Evidence

- `.flow/evidence/fn-124-growth-in-half-the-time/growth_sampler.rs` and `analyze-native.py`: the scratch SIGPROF native profiler and its analyser (reused by fn-173). Build in a scratch checkout's `crates/telperion-core/examples/` with `RUSTFLAGS="-C force-frame-pointers=yes" CARGO_PROFILE_RELEASE_DEBUG=line-tables-only`.
