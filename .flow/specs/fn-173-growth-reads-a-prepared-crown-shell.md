# Growth reads a prepared crown shell

## Conversation Evidence

> user (2026-09-27): "we have found many optimizations but skeleton generation seems to still take a lot of time. Can we investigate how we can speed that up?"
> user (2026-09-27): "you also have to check the specs that follow from 125 as they rewrite the generation pipeline"
> user (2026-09-27): "i have said many times as we build the engine we don't need things byte identical. If we get massive improvements for different generators that aren't identical that's fine. If it being identical helps verifying at no downside then it's good req to have."
> user (2026-09-27, on splitting the growth work into focused specs): "yes and make fn-125 depend on B if that's the most efficient way"

## Goal & Context
<!-- Goal & Context: 30% [user], 70% [checked] from the 2026-09-27 growth profile -->

Every growth step asks the crown's shell for its radius, and each answer is computed from scratch: `Envelope::radius_at` evaluates `quadrant` as a log, an expm1 and an exp (#108), and a lobed shell multiplies that by `noise::seeded`. On master f9487810 (native, scratch sampler), `radius_at` takes 25.1% of oak seed 1's 51.3 ms growth, `radius_toward` 32.7% of beech seed 1's 103.2 ms, and `radius_at` 53.3% of birch seed 7's 268.4 ms, most of it inside the curtain search fn-174 covers. [checked]

An envelope's radius depends only on height, and for a lobed shell on height and bearing. This spec prepares each envelope once as a table and makes every reader of that envelope read the table. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **Readers today, checked 2026-09-27.** `local::planner::rejected` (admission), `Envelope::contains` (scaffold edges, attractor sampling), `Envelope::profile()` (shedding and crown exposure), the curtain's `lower_surface` (`local/pendant.rs:260`), and the GPU cull's `radius_at` and `quadrant` in `telperion-render/src/generation/place.wgsl:24-40`, which is the core's formula in WGSL. The GPU cull reads the smooth radius only, with no lobes. [checked]
- **Which presets it touches.** Beech (`irregularity = 0.18`) and birch (`0.35`) are lobed. The oak takes the default shoulder of 2.2 and no lobes. The spruce's shoulder is 1, which `quadrant` already answers as a straight line, so the spruce gains little. [checked]
- **Envelope views, checked 2026-09-27.** The direct build reads two envelopes: the outer one, for admission and edge containment, and `inner_envelope(envelope, reach)` (`branching.rs:232`), for attractor sampling and scaffold planning (`specimen.rs:110`, `scaffold/frontier.rs:85, 145`). The hidden growth path builds a new envelope per slice, with height scaled by the slice fraction (`specimen/timeline.rs:212`) and a planning envelope with crown base scaled (`:289`). For a lobed envelope, height enters the lobe coordinates through the wavelength. [checked]
- **The shell.** A table prepared once per envelope value and seed, cached by equality as `Crown::prepare` already caches its outline index. The direct build prepares two, the outer and the inner. The growth path prepares one per distinct slice envelope, and its preparation cost is measured. Each view is checked against its own exact envelope. The smooth radius is a table over height. A lobed shell's radius is a table over height and bearing, because its noise is sampled on the shell's surface at `(cos θ · around, y / wavelength, sin θ · around)` (`envelope.rs` `lobed`). Readers interpolate. [inferred]
- **The bound (host, 2026-09-27).** The table's radius differs from the exact `radius_toward` by at most 1e-3 of the envelope's widest radius (`max_radius() * (1 + irregularity)`), at every height and bearing. Preparation proves the bound per cell from a conservative interpolation-error estimate: for each cell, the interpolation's worst-case error from a bound on the tabled function's derivative over that cell, in whatever variable the table is taken in. It returns a named error for any cell whose estimate exceeds the bound. Sampled points cannot establish a bound everywhere: a half-cell check passes `sqrt(4t(1-t))` at 200,000 linear cells while a quarter-point in the first cell exceeds 1e-3 (plan review, 2026-09-27). A check grid offset from the table grid, including both crown ends and the bearing seam, is kept as additional verification of the estimate. The radius has an unbounded slope at the crown's base and top, so the sampling may be denser there or taken in a variable where the curve is smooth; the worker picks by measurement. [inferred]
- **The curtain's starting height.** `lower_surface` starts its climb at the smooth lower surface for the widest lobed radius, one exact `quadrant` call per search (`local/pendant.rs:269`). That one evaluation stays exact and is taken at the query radius less the bound, clamped at zero and rounded down, so a start taken against the tabled shell cannot lie above its first crossing. The bound is radial, so lowering the height by it is not conservative: at radial 0.99 on the envelope above, a height lowered by 1e-3 starts at 0.42847 while a permitted outward error moves the crossing to 0.42604 (plan review, 2026-09-27). It is the only exact `quadrant` call a growth reader keeps. [inferred]
- **One shell, two executors.** The GPU cull reads the same table, uploaded with the cull configuration, in place of its WGSL copy of `quadrant`. That is why fn-125 depends on this spec: fn-125's plan layout carries the envelope and cull configuration (its R1), so it carries the table from the start, and its byte-identity base (R3) and speed base (R7) are taken after this change instead of being reopened by it. [inferred]
- **Identity is not required.** The tree changes within the stated bound, under AGENTS.md "Generator evolution". [user]

## Edge Cases & Constraints

- Table memory is recorded per preset. A lobed table's size depends on the lobe wavelength against the crown's size. [unknown: resolution not yet measured]
- Points outside the crown's height span answer zero, as `radius_at` does today. [checked]
- The Wasm artifacts stay within CI's size budget. [inferred]
- No full-forest capture. [AGENTS.md]

## Acceptance Criteria

- **R1:** A prepared shell per envelope view serves every reader listed above, including the GPU cull. No reader evaluates `quadrant` or the lobe noise per query, except `lower_surface`'s one starting height per search. No second copy of the formula remains in WGSL. Errors: an envelope the table cannot represent within the bound is a named error with a red/green test. [inferred]
- **R2:** A test checks each prepared view (outer, inner, and a growth-path slice and planning envelope) against its exact envelope within the bound, over every catalogue and in-work preset at seeds 1 and 7. It covers off-grid heights and bearings, the bearing seam, both crown ends and a zero-span envelope. A red/green test shows an envelope whose lobes the table cannot resolve refused by name. A test shows `lower_surface`'s starting height lies at or below the tabled shell's first crossing, including shallow-slope columns near the widest radius. A test shows the per-cell estimate refuses a table that meets the bound at every sampled point but exceeds it between them. [inferred]
- **R3:** `growth_profile` medians on base and candidate: oak growth at least 15% lower and beech at least 20% lower at seeds 1 and 7; every other preset recorded; no preset more than 2% slower. A miss stops with `NEEDS_HUMAN` and the profile. [inferred]
- **R4:** The suite's containment and curtain checks pass for every preset. The owner's visual verdict is recorded for the oak, beech and birch at seeds 1 and 7, on stills of at most four images per preset. [user]
- **R5:** The GPU and CPU culls agree on surviving leaf counts for the oak and spruce at seeds 1 and 7, and the harness's browser completed-frame medians for both are no slower within fn-125's 5% tolerance. [inferred]

## Boundaries

- The curtain search algorithm is fn-174's. This spec only makes its radius queries cheap. [inferred]
- Shedding's search is fn-172's. [inferred]

## Decision Context

- fn-91 task .12 rejected "prepared envelope admission bounds" on 1 to 4% browser gains and an unexplained 41,824 KiB RSS increase (`.flow/evidence/fn-91-.../envelope/REPORT.md`). That candidate kept the exact formula behind bounds. This spec replaces the formula for every reader, and R3 and R5 measure what fn-91 found missing. [checked]
- fn-125 depends on this spec (owner, 2026-09-27). [user]
- Evidence: `PROFILE.md` in this spec's evidence directory. [checked]
