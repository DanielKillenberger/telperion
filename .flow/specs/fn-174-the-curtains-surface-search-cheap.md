# The curtain's surface search, cheap

## Conversation Evidence

> user (2026-09-27): "we have found many optimizations but skeleton generation seems to still take a lot of time. Can we investigate how we can speed that up?"
> user (2026-09-27): "i have said many times as we build the engine we don't need things byte identical. If we get massive improvements for different generators that aren't identical that's fine. If it being identical helps verifying at no downside then it's good req to have."
> user (2026-09-27, on splitting the growth work into focused specs): "yes and make fn-125 depend on B if that's the most efficient way"

## Goal & Context
<!-- Goal & Context: 30% [user], 70% [checked] from the 2026-09-27 growth profile -->

The silver birch grows in 136 ms at seed 1 and 267 ms at seed 7 (`growth_profile` medians, master f9487810), with fewer nodes than the oak's 51 ms. On seed 7, the curtain's admission (`Curtain::admits` → `in_band` → `lower_surface`) takes 78.6 to 81.3% of growth under the scratch sampler, about 210 ms. This spec makes that search cheap. [checked]

## Architecture & Data Models
<!-- scope: technical -->

- **Today, checked 2026-09-27.** A hanging shoot's candidate outside the shell is admitted when it lies in the curtain's band below the shell (`local/pendant.rs:111`, `in_band` `:226`). `lower_surface` (`:260`) finds the shell's lower surface over the candidate's column. It climbs up to `SEARCH = 32` steps and then halves `HALVINGS = 32` times, and each probe is a `radius_toward` with lobe noise. Its only production caller is `in_band`, which returns a bool. The suite calls `in_curtain_band` as a check. [checked]
- **Candidates, chosen by measurement.**
  - The lower surface as a prepared table over bearing and radial distance, from fn-173's shell, so a query becomes a lookup.
  - A search that stops once the band decision is known. `walk` is linear and `limit` is where the band's foot meets `p.y`, so the answer is often fixed before the bracket closes. [unknown: whether that holds for every branch of `in_band` has not been checked in code]
  - Halvings reduced to the precision the band test needs. [inferred]
- **Identity is not required.** Output may change within the curtain checks and the owner's visual verdict. [user]

## Edge Cases & Constraints

- A column that never meets the shell below `limit` stays `None`. [checked]
- The date palm (`hang = 2.75`) and the spruce (`hang = 1.0`) also hang shoots; both are measured. [checked]
- No full-forest capture. [AGENTS.md]

## Acceptance Criteria

- **R1:** On fn-173's merged code, birch growth at seeds 1 and 7 is at most half the base's `growth_profile` median. Every other preset is recorded, and none is more than 2% slower. A miss stops with `NEEDS_HUMAN` and the profile. [inferred]
- **R2:** The suite's curtain checks (`suite/drop.rs`, `suite/outline.rs`, `suite/growth.rs`) pass for every preset. [checked]
- **R3:** The owner's visual verdict on the birch at seeds 1 and 7 is recorded, with at most four stills each. [user]

## Boundaries

- The shell table is fn-173's. [inferred]
- The curtain's botany (hang, drop, floor) is unchanged. [inferred]

## Decision Context

- Depends on fn-173 so its gain is measured on top of the cheaper shell. [user]
- Evidence: `.flow/evidence/fn-173-growth-reads-a-prepared-crown-shell/PROFILE.md`. [checked]
