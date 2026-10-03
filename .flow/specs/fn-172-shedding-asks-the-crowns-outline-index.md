# Shedding asks the crown's outline index a yes-or-no question

## Conversation Evidence

> user (2026-09-27): "we have found many optimizations but skeleton generation seems to still take a lot of time. Can we investigate how we can speed that up?"
> user (2026-09-27): "i have said many times as we build the engine we don't need things byte identical. If we get massive improvements for different generators that aren't identical that's fine. If it being identical helps verifying at no downside then it's good req to have."
> user (2026-09-27, on splitting the growth work into focused specs): "yes and make fn-125 depend on B if that's the most efficient way"

## Goal & Context
<!-- Goal & Context: 30% [user], 70% [checked] from the 2026-09-27 growth profile -->

Growth (stage 2) is the largest stage left on the CPU: 53 of 126 ms of the oak's preparation on fn-125's master baseline. fn-125, fn-126 and fn-171 all leave growth out of scope, and fn-124 closed with its gate missed. This is one of four focused growth specs taken from the 2026-09-27 profile. [user]

Shedding is the largest cost on the ordinary preset and the second largest on Telperion. On master f9487810, with the scratch sampler running natively, `finish` and shedding take 48.7% of ordinary seed 1's 47.7 ms, and `distance_to_profile` takes 38.7% of it. On Telperion seed 1, shedding takes 25.6% of 400 ms and `distance_to_profile` 19.7%. [checked]

## Architecture & Data Models
<!-- scope: technical -->

- **Today, checked 2026-09-27.** `shed` (`pipeline/branching.rs:273`) keeps a node when `radius_at(y) - r <= shell` or `distance_to_profile(&profile, r, y) <= shell`. The second test computes the exact minimum over all 128 outline segments for every node deeper than the shell, then compares it once. The only production caller is `finish` (`branching.rs:400`); `audit.rs:54` and the suite also call it. [checked]
- **An index already exists.** `specimen/crown.rs` builds a bounding-box tree over the same `Envelope::profile()` polyline once per envelope and prunes it conservatively (`Crown::nearest`, rounding margin `1e-12`). It serves crown exposure, and `shed` does not reach it. [checked]
- **The GPU cull already stops early.** `place.wgsl` `in_shell` returns at the first segment within `shell`. [checked]
- **The change.** Shedding asks whether any outline segment lies within `shell` and stops at the first one, walking the one outline index in near-first order. There is one index of the outline, reached by both crown exposure and shedding. Where it lives (in `envelope.rs` beside `profile()`, or passed from the specimen into `shed`) is the worker's call, recorded in the PR. No second index is built. [inferred]
- **Identity.** The keep decision is a comparison against `shell`, so a pruned search with a rounding margin keeps every decision and the tree's bytes. This comes free with the fastest form, so it is required. [inferred]

## Edge Cases & Constraints

- The `exceptional` branch of `distance_to_profile` (subnormal squared distances fall back to `hypot`) keeps its meaning in the pruned search. [checked]
- An envelope with no span (`profile()` still returns 129 points at one height) is handled as today. [unknown: not exercised by a shipped preset]
- fn-126's candidate A (a per-leaf bound in the CPU cull, `foliage.rs:298`) can reuse the same index; this spec does not change the cull. [inferred]

## Acceptance Criteria

- **R1:** Shedding uses the one outline index and stops at the first segment within `shell`. No second outline index exists in the crate. Errors: none beyond today's `shed` validation. [inferred]
- **R2:** Every catalogue and in-work preset at seeds 1 and 7 grows a tree whose `growth_profile` hash equals the base's. The existing crown index tests (`specimen/crown.rs` tests) pass. The pipeline build does not exercise crown exposure, and preset hashes miss the edge cases, so a differential test also holds the new keep decision to `distance_to_profile(..) <= shell`: at distances equal to `shell` and one ulp either side, for a subnormal squared distance, and for a zero-span outline. (plan review, 2026-09-27) [inferred]
- **R3:** `examples/growth_profile.rs` medians (six samples, first dropped) on base and candidate, same machine, one session: ordinary seed 1 and 7 growth at least 30% lower; Telperion seeds 1 and 7 recorded; no preset at either seed more than 2% slower. A miss stops with `NEEDS_HUMAN` and the numbers. [inferred]

## Boundaries

- The cull is fn-125's and fn-126's. [inferred]
- The shell table is fn-173's; this spec reads the outline as `profile()` gives it today. [inferred]

## Decision Context

- Separate from B because it is independent of the shell table, small, and lands first. [user]
- Evidence: `.flow/evidence/fn-173-growth-reads-a-prepared-crown-shell/PROFILE.md`, the 2026-09-27 growth profile all four specs cite. [checked]

## Superseded pending fn-190-one-growth-law-species-are-points-in-a (owner, 2026-10-03)

The new growth law replaces the growth-side work this spec speeds up (outline queries and shedding by outline distance). Not ready; to be closed or re-scoped once fn-190-one-growth-law-species-are-points-in-a's R2 design is recorded. [user]
