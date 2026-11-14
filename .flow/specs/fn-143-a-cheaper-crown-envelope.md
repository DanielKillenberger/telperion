# A cheaper crown envelope

## Goal

fn-124 missed its half-time growth gate; its profile put two `pow` calls in `Envelope::radius_at` (`max_radius * (1 - p^shoulder)^(1/shoulder)`) at about 22 of the oak's 62-65 ms browser skeleton, and the same function runs per vertex in the CPU cull, the field and station code. This spec is the remaining path to half time: a cheaper evaluation of the same profile curve, allowed to change output bytes under the 2026-09-20 policy with measured gain and no perceptible visual regression. Owner, choosing "Ship + envelope spec" when fn-124 stopped: accepted.

## Decisions

- One implementation for growth, the cull and every caller; no preset branch. The GPU cull's WGSL copy (`place.wgsl` `radius_at`) changes in the same change, as its own commit.
- Owner (2026-09-24): at merge, and again if fn-125 lands after, the host confirms no `.wgsl` file still evaluates the old `pow` envelope — a one-time check recorded in the PR, not a test.
- Supersedes reviving fn-91's rejected prepared envelope bounds.
- Shipped form: `exp(ln(-expm1(s ln p))/s)` from pinned libm, shoulder 1 kept as the exact line; the pendant lower-surface search and `place.wgsl` use the same form.

## Acceptance Criteria

- **R1:** Max radius error stated and tested over every preset envelope — met: 3.68e-7 of the widest radius (bound 1e-6, `tests/envelope_profile.rs`).
- **R2:** Browser skeleton and native stage medians recorded; oak reaches half of fn-124's base or the next limiting stage is named — oak 59.5 -> 51.9 and 65.8 -> 58.1 ms, short of half (34.9 / 38.3); next limiting stage: the local-advance loop body.
- **R3:** The owner compares whole-tree stills base vs candidate and records no perceptible regression — met: owner verdict 2026-09-24, "all 16 still pairs identical, no regression"; the re-pin commit `d40d0e4a` is kept.
- **R4:** Cull time recorded — birch -7 to -9%, spruce -6 to -8%, oak -18 to -20%, beech -20%.

## Result

All 16 hero stills and every wood/leaf hash byte-identical; skeleton positions move by at most 5.5e-10 m with identical topology, so only pinned skeleton digests were re-baselined. Commits `df3dc904`, `a5f31dc9`, `4148f650`, `0f4496d2`, `d40d0e4a`; gate 935 passed; codex review found no defects.

## Open

- Half-time growth still unreached; the local-advance loop body is next.

## Friction

- The native stage comparison over eight fixtures hit the 590 s foreground guard and lost five fixtures' numbers (~10 min) -> a stage selector on `generation_stages`, or the compare script writing each fixture as it finishes.
