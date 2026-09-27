# The bias field's curl in one noise pass

## Conversation Evidence

> user (2026-09-27): "we have found many optimizations but skeleton generation seems to still take a lot of time. Can we investigate how we can speed that up?"
> user (2026-09-27): "i have said many times as we build the engine we don't need things byte identical. If we get massive improvements for different generators that aren't identical that's fine. If it being identical helps verifying at no downside then it's good req to have."
> user (2026-09-27, on splitting the growth work into focused specs): "yes and make fn-125 depend on B if that's the most efficient way"

## Goal & Context
<!-- Goal & Context: 30% [user], 70% [checked] from the 2026-09-27 growth profile -->

Telperion grows in 391 ms at seed 1 (`growth_profile` median, master f9487810). Under the scratch sampler, `GrowthBias::apply` takes 47.0% of that time and `Noise::curl` 42.4%, with `Noise::at` itself at 41.2%. The curl runs only when the supernatural bias is enabled, which Telperion and Laurelin set. This spec makes the curl cheap. [checked]

## Architecture & Data Models
<!-- scope: technical -->

- **Today, checked 2026-09-27.** `Noise::curl` (`noise.rs:72`) takes six central differences at `±0.001`, and each difference evaluates the two-octave `fbm` twice. That is 24 Perlin evaluations per heading. `GrowthBias::apply` (`pipeline/bias.rs:200`) calls it when `writhe_amplitude / writhe_wavelength > 0`. [checked]
- **Candidates, chosen by measurement.**
  - Gradient noise that returns its analytic derivative. Three fields at two octaves need 6 evaluations.
  - A curl field sampled once per tree on a grid over the crown and interpolated per heading, if its preparation costs less than it saves. [inferred]
- **Identity is not required.** The writhe changes within the owner's visual verdict. [user]

## Edge Cases & Constraints

- A preset with the supernatural bias disabled runs no curl and is unchanged. [checked]
- `Noise` keeps its permutation table (`noise.rs:91-95`); the envelope's `noise::seeded` is fn-173's. [checked]
- No full-forest capture. [AGENTS.md]

## Acceptance Criteria

- **R1:** Telperion growth at seed 1 is at most 70% of the base's `growth_profile` median. Telperion seed 7 and Laurelin at seeds 1 and 7 are recorded. No preset at either seed is more than 2% slower. A miss stops with `NEEDS_HUMAN` and the profile. [inferred]
- **R2:** A test holds the new curl to the finite-difference curl within a stated bound on sampled points. [inferred]
- **R3:** The owner's visual verdict on Telperion and Laurelin at seeds 1 and 7 is recorded, with at most four stills each. [user]

## Boundaries

- The writhe's parameters and meaning are unchanged. [inferred]

## Decision Context

- Independent of fn-172, fn-173 and fn-174, and can run beside them. [user]
- Evidence: `.flow/evidence/fn-173-growth-reads-a-prepared-crown-shell/PROFILE.md`. [checked]
