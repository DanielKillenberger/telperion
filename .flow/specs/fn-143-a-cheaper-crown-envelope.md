# A cheaper crown envelope

## Conversation Evidence

> assistant: "The oak's single biggest cost is two `pow` calls in the crown-envelope test, about 22 of its 65 ms. That test runs constantly during growth, and it's the same function the CPU cull hammers (fn-126). [...] a cheaper crown-envelope formula. It would change output bytes, so it needs visual evidence under the 2026-09-20 policy, but it's the only path to half time."
> user (choosing "Ship + envelope spec"): accepted.

## Goal & Context

<!-- Goal & Context: 20% [user], 80% [inferred] from fn-124's profile -->

`Envelope::radius_at` (`envelope.rs`) evaluates the crown's radius at a height as `max_radius * (1 - p^shoulder)^(1/shoulder)`, two `powf_fixed` calls per test. fn-124's profile (`.flow/evidence/fn-124-growth-in-half-the-time/REPORT.md`) puts `pow` inside `radius_at` at about 22 of the oak's 62 to 65 ms browser skeleton after fn-124's byte-identical fixes; the spruce, at shoulder 1, pays less. The same function runs per vertex in `foliage::cull` and in the field and station code. fn-124's R2 (half the browser skeleton time) was restated to its measured gain because no byte-identical change reaches it; this spec is the remaining path. [inferred]

## Architecture & Data Models

- Replace the per-test `pow` pair with a cheaper evaluation of the same profile curve. Candidates to screen, cheapest first: a per-envelope table of the profile sampled finely enough in `t` with linear or cubic interpolation; a specialised form for common shoulder values (1 is linear and needs no `pow`); a rational or polynomial approximation of `(1 - p^s)^(1/s)` with a stated error bound. [inferred]
- The change is allowed to alter output bytes under the 2026-09-20 policy, with measured gain, no perceptible visual regression and correctness intact (CLAUDE.md, "Generator evolution"). [user]
- One implementation serves growth, the cull and every other caller; no species or preset branch. [CLAUDE.md]

## Acceptance Criteria

- **R1:** The chosen evaluation's maximum radius error against the current formula is stated and tested over every catalogue and IN_WORK preset's envelope, across the full height range. [inferred]
- **R2:** Browser skeleton medians of five for the oak and spruce at seeds 1 and 7, and native `generation_stages` medians for every preset, are recorded on base and candidate; the oak's browser skeleton reaches half of fn-124's base (34.9 and 38.3 ms) or the report names the next limiting stage. [inferred]
- **R3:** The owner compares whole-tree stills of every catalogue preset at seeds 1 and 7, base against candidate, and records no perceptible regression; botanical counts and geometry checks the test suite holds still pass, and changed pinned hashes are rebaselined with that evidence. [user]
- **R4:** The cull's time for the birch and spruce is recorded on base and candidate. [inferred]

## Boundaries

- The envelope's shape parameters and presets are unchanged; only its evaluation changes. [inferred]
- No full-forest capture; stills are single trees. [CLAUDE.md]

## Decision Context

- Captured 2026-09-24 when fn-124 missed its half-time gate. fn-91's prepared envelope bounds (task .12, rejected) are a different, byte-identical approach worth about 12 to 17 ms on the oak and nothing on the spruce; this spec supersedes the need to revive them unless its screen fails. [inferred]
- Related: fn-126 (CPU cull), whose per-vertex `radius_at` is the same cost. [inferred]
